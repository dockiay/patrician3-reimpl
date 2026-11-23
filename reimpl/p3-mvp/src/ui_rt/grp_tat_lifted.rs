//! Generado por `reimpl/tools/ui_rt_grp.py tat` con `tools/lift.py` (no editar a mano): grupo `tat`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_TAT: [(u32, super::core_table::CoreFn); 12] = [
    (0x5fd510, f_005fd510),
    (0x5fe7e0, f_005fe7e0),
    (0x621770, f_00621770),
    (0x6247b0, f_006247b0),
    (0x624950, f_00624950),
    (0x625bf0, f_00625bf0),
    (0x6258a0, f_006258a0),
    (0x625b00, f_00625b00),
    (0x612d20, f_00612d20),
    (0x57b640, f_0057b640),
    (0x57a4a0, f_0057a4a0),
    (0x5687b0, f_005687b0),
];
pub fn f_005fd510(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5fd510;
    loop {
        match pc {
            0x5fd510 => {
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x5fd522);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2f1a),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5fd67b; continue; }
            pc = 0x5fd530; continue;
            }
            0x5fd530 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5fd5fc; continue; }
            pc = 0x5fd545; continue;
            }
            0x5fd545 => {
            c.set(3,32,c.r[6].wrapping_add(0x1edc));
            c.set(0,32,c.r[6].wrapping_add(0x1ed8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x5fd55f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b4b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5fd56b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x58e760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            c.set(5,32,c.r[6].wrapping_add(0x1eb2));
            c.set(7,32,c.r[6].wrapping_add(0x1e9e));
            pc = 0x5fd57f; continue;
            }
            0x5fd57f => {
            c.set(0,32,c.r[6].wrapping_add(0x1ec4));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5fd5e7; continue; }
            pc = 0x5fd589; continue;
            }
            0x5fd589 => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xfffffffc),8));
            c.set(1,8,rm(m,c.r[5],8));
            c.set(2,8,rm(m,c.r[3],8));
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x1ed8),16),16); c.set(0,16,r); }
            { let a = c.get(1,8); let r = c.add(a,c.get(2,8),8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x1));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[7]; wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[5],8));
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[3],16),16); c.set(0,16,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffde));
            c.set(1,16,c.get(2,16));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,c.r[0].wrapping_add(0x22));
            { let a = c.get(0,32); let r = c.add(a,0xffffffdeu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[1].wrapping_add(0x22));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.add(a,0xffffffdeu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,0x8u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5fd5e3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x619450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,16,c.get(0,16)); }
            pc = 0x5fd5e7; continue;
            }
            0x5fd5e7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x5fd57f; continue; }
            pc = 0x5fd5fa; continue;
            }
            0x5fd5fa => {
            pc = 0x5fd66d; continue;
            }
            0x5fd5fc => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(5,32,c.r[6].wrapping_add(0x1eb2));
            c.set(7,32,c.r[6].wrapping_add(0x1e9e));
            pc = 0x5fd60a; continue;
            }
            0x5fd60a => {
            c.set(0,32,c.r[6].wrapping_add(0x1ec4));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5fd662; continue; }
            pc = 0x5fd614; continue;
            }
            0x5fd614 => {
            c.set(2,8,rm(m,c.r[5],8));
            { let a = c.get(0,16); let r = c.xor(a,c.get(0,16),16); c.set(0,16,r); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xfffffffc),8));
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            c.set(0,8,c.get(2,8));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x1));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[7]; wm(m,a,16,c.get(1,16)); }
            c.set(0,16,rm(m,c.r[5],8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffde));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,c.r[0].wrapping_add(0x22));
            { let a = c.get(0,32); let r = c.add(a,0xffffffdeu32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[1].wrapping_add(0x22));
            c.push(m,c.get(2,32));
            { let a = c.get(1,32); let r = c.add(a,0xffffffdeu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,0x8u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5fd65e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x619450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,16,c.get(0,16)); }
            pc = 0x5fd662; continue;
            }
            0x5fd662 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(3,16); c.sub(a,0x4u32,16); }
            if c.cond(2) { pc = 0x5fd60a; continue; }
            pc = 0x5fd66d; continue;
            }
            0x5fd66d => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5fd674);
            { let args = []; c.r[0] = host.call(m, 0x5ffc10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x2f1a); wm(m,a,8,0x0u32); }
            pc = 0x5fd67b; continue;
            }
            0x5fd67b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5c),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fd7d4; continue; }
            pc = 0x5fd68a; continue;
            }
            0x5fd68a => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12e9),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5fd6cd; continue; }
            pc = 0x5fd694; continue;
            }
            0x5fd694 => {
            c.set(0,8,rm(m,0x6e59cdu32,8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fd7d4; continue; }
            pc = 0x5fd6a1; continue;
            }
            0x5fd6a1 => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(0,32,c.r[6].wrapping_add(0x12ec));
            c.push(m,c.get(0,32));
            c.push(m, 0x5fd6b3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x12e9); wm(m,a,8,0x0u32); }
            { let a = 0x6e59cdu32; wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x12ea); wm(m,a,8,0x1u32); }
            pc = 0x5fd7d4; continue;
            }
            0x5fd6cd => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x558),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0xffu32);
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(6) { pc = 0x5fd7d4; continue; }
            pc = 0x5fd6e5; continue;
            }
            0x5fd6e5 => {
            c.set(7,32,c.r[6].wrapping_add(0x35a));
            pc = 0x5fd6eb; continue;
            }
            0x5fd6eb => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[7],16));
            { let a = rm(m,0x6dfc14u32,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x5fd6fa; continue; }
            pc = 0x5fd6f8; continue;
            }
            0x5fd6f8 => {
            c.set(1,32,c.get(0,32));
            pc = 0x5fd6fa; continue;
            }
            0x5fd6fa => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x5fd6eb; continue; }
            pc = 0x5fd703; continue;
            }
            0x5fd703 => {
            { let a = c.get(1,16); c.sub(a,0xffu32,16); }
            if c.cond(4) { pc = 0x5fd7d4; continue; }
            pc = 0x5fd70e; continue;
            }
            0x5fd70e => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x15c),16); c.sub(a,c.get(7,16),16); }
            if c.cond(6) { pc = 0x5fd7a8; continue; }
            pc = 0x5fd724; continue;
            }
            0x5fd724 => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xc8),32); c.push(m, 0x5fd732); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fd7d4; continue; }
            pc = 0x5fd73a; continue;
            }
            0x5fd73a => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(2,32,rm(m,0x6cbb10u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x96u32); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x662),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,0x66bdc0u32,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x5a4),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5fd781; continue; }
            pc = 0x5fd76f; continue;
            }
            0x5fd76f => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x662),8));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(7,32)); }
            { let a = 0x6e59ccu32; wm(m,a,8,c.get(1,8)); }
            pc = 0x5fd789; continue;
            }
            0x5fd781 => {
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x1u32); }
            pc = 0x5fd789; continue;
            }
            0x5fd789 => {
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5fd798);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x6e59cdu32; wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x12ea); wm(m,a,8,0x0u32); }
            pc = 0x5fd7d4; continue;
            }
            0x5fd7a8 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2f18),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(4) { pc = 0x5fd7d4; continue; }
            pc = 0x5fd7b4; continue;
            }
            0x5fd7b4 => {
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x3c),16),16); }
            if c.cond(7) { pc = 0x5fd7cb; continue; }
            pc = 0x5fd7bb; continue;
            }
            0x5fd7bb => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x2f18); wm(m,a,16,c.get(7,16)); }
            c.push(m, 0x5fd7c9);
            { let args = []; c.r[0] = host.call(m, 0x6100d9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5fd7d4; continue;
            }
            0x5fd7cb => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x2f18); wm(m,a,16,c.get(0,16)); }
            pc = 0x5fd7d4; continue;
            }
            0x5fd7d4 => {
            c.set(5,16,rm(m,c.r[4].wrapping_add(0x3c),16));
            { let a = c.get(5,16); c.and(a,c.get(5,16),16); }
            if c.cond(4) { pc = 0x5fe06b; continue; }
            pc = 0x5fd7e2; continue;
            }
            0x5fd7e2 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12ea),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5fd809; continue; }
            pc = 0x5fd7ec; continue;
            }
            0x5fd7ec => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x5fd7fa); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fd809; continue; }
            pc = 0x5fd7fe; continue;
            }
            0x5fd7fe => {
            { let a = 0x6e59cdu32; wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x12ea); wm(m,a,8,c.get(0,8)); }
            pc = 0x5fd809; continue;
            }
            0x5fd809 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fd9e7; continue; }
            pc = 0x5fd814; continue;
            }
            0x5fd814 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12e9),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5fd90b; continue; }
            pc = 0x5fd822; continue;
            }
            0x5fd822 => {
            c.set(0,8,rm(m,0x6e59cdu32,8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fd852; continue; }
            pc = 0x5fd82b; continue;
            }
            0x5fd82b => {
            c.set(1,32,c.r[6].wrapping_add(0x12ec));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.push(m, 0x5fd83d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x12e9); wm(m,a,8,0x0u32); }
            { let a = 0x6e59cdu32; wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x12ea); wm(m,a,8,0x1u32); }
            pc = 0x5fd852; continue;
            }
            0x5fd852 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x558),8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(1,8,0x1u32);
            if c.cond(6) { pc = 0x5fd8da; continue; }
            pc = 0x5fd860; continue;
            }
            0x5fd860 => {
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x5d),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x5fd8c4; continue; }
            pc = 0x5fd86f; continue;
            }
            0x5fd86f => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x35a),16));
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5fd8c2; continue; }
            pc = 0x5fd882; continue;
            }
            0x5fd882 => {
            c.set(2,16,rm(m,0x6e59ccu32,8));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x662),16));
            { let a = c.get(2,16); c.sub(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x5fd8c2; continue; }
            pc = 0x5fd896; continue;
            }
            0x5fd896 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x96u32); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0x1u32); }
            c.push(m, 0x5fd8c2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5fd8c2; continue;
            }
            0x5fd8c2 => {
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            pc = 0x5fd8c4; continue;
            }
            0x5fd8c4 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x558),8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x5fd860; continue; }
            pc = 0x5fd8d2; continue;
            }
            0x5fd8d2 => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5fd9de; continue; }
            pc = 0x5fd8da; continue;
            }
            0x5fd8da => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x130f),8));
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0xac8); wm(m,a,8,0x1u32); }
            c.set(1,8,c.get(3,8));
            c.set(5,16,c.get(3,16));
            { let a = c.get(1,8); let r = c.and(a,0xfu32,8); c.set(1,8,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(5,16); let n = 0x4u32; let r = c.shr(a,n,16); c.set(5,16,r); }
            { let a = c.r[6].wrapping_add(0x130f); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,16,c.get(5,16)); }
            pc = 0x5fda52; continue;
            }
            0x5fd90b => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x558),8));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(6) { pc = 0x5fd852; continue; }
            pc = 0x5fd91e; continue;
            }
            0x5fd91e => {
            c.set(1,32,c.r[6].wrapping_add(0x15c));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            pc = 0x5fd929; continue;
            }
            0x5fd929 => {
            { let a = rm(m,c.r[1],16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x5fd931; continue; }
            pc = 0x5fd92f; continue;
            }
            0x5fd92f => {
            c.set(2,8,0x1u32);
            pc = 0x5fd931; continue;
            }
            0x5fd931 => {
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x5fd929; continue; }
            pc = 0x5fd937; continue;
            }
            0x5fd937 => {
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x5fd852; continue; }
            pc = 0x5fd93f; continue;
            }
            0x5fd93f => {
            { let a = c.get(5,16); c.sub(a,0x5u32,16); }
            if c.cond(6) { pc = 0x5fd94e; continue; }
            pc = 0x5fd945; continue;
            }
            0x5fd945 => {
            c.set(3,32,0x5u32);
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            pc = 0x5fd94e; continue;
            }
            0x5fd94e => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(5,32,0xfffeu32);
            pc = 0x5fd95b; continue;
            }
            0x5fd957 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            pc = 0x5fd95b; continue;
            }
            0x5fd95b => {
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x35a),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5fd9c6; continue; }
            pc = 0x5fd976; continue;
            }
            0x5fd976 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x5d),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5fd9c6; continue; }
            pc = 0x5fd97e; continue;
            }
            0x5fd97e => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x15c),16));
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            if c.cond(6) { pc = 0x5fd9c6; continue; }
            pc = 0x5fd98b; continue;
            }
            0x5fd98b => {
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x3c),16),16); }
            if c.cond(7) { pc = 0x5fd9bc; continue; }
            pc = 0x5fd992; continue;
            }
            0x5fd992 => {
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x15c); wm(m,a,16,c.get(5,16)); }
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x5fd9a8); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5fd9c6; continue; }
            pc = 0x5fd9ac; continue;
            }
            0x5fd9ac => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x118),32); c.push(m, 0x5fd9ba); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5fd9c6; continue;
            }
            0x5fd9bc => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x15c); wm(m,a,16,c.get(1,16)); }
            pc = 0x5fd9c6; continue;
            }
            0x5fd9c6 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x558),8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x5fd957; continue; }
            pc = 0x5fd9d4; continue;
            }
            0x5fd9d4 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x5fd9de => {
            { let a = c.r[6].wrapping_add(0xac8); wm(m,a,8,0x0u32); }
            pc = 0x5fda52; continue;
            }
            0x5fd9e7 => {
            c.set(0,8,rm(m,0x6e59cdu32,8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fda21; continue; }
            pc = 0x5fd9f0; continue;
            }
            0x5fd9f0 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12e9),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5fda21; continue; }
            pc = 0x5fd9fa; continue;
            }
            0x5fd9fa => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.set(2,32,c.r[6].wrapping_add(0x12ec));
            c.push(m,c.get(2,32));
            c.push(m, 0x5fda0c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x12e9); wm(m,a,8,0x0u32); }
            { let a = 0x6e59cdu32; wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x12ea); wm(m,a,8,0x1u32); }
            pc = 0x5fda21; continue;
            }
            0x5fda21 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x15c),16));
            { let a = c.get(0,16); c.sub(a,c.get(5,16),16); }
            if c.cond(7) { pc = 0x5fda49; continue; }
            pc = 0x5fda2d; continue;
            }
            0x5fda2d => {
            { let a = c.r[6].wrapping_add(0x15c); wm(m,a,16,0x0u32); }
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x4),32); c.push(m, 0x5fda3f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x5fda49 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x15c); wm(m,a,16,c.get(0,16)); }
            pc = 0x5fda52; continue;
            }
            0x5fda52 => {
            { let a = c.get(5,16); c.and(a,c.get(5,16),16); }
            if c.cond(4) { pc = 0x5fe06b; continue; }
            pc = 0x5fda5b; continue;
            }
            0x5fda5b => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            if c.cond(6) { pc = 0x5fe06b; continue; }
            pc = 0x5fda69; continue;
            }
            0x5fda69 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            { let a = c.r[6].wrapping_add(0xa90); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xa94); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xa98); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0xa9a); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0xaa0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xaa4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xab4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xab8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xabc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xac0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            pc = 0x5fdab7; continue;
            }
            0x5fdab7 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x5fdac1; continue; }
            pc = 0x5fdabb; continue;
            }
            0x5fdabb => {
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x10),16));
            pc = 0x5fdac8; continue;
            }
            0x5fdac1 => {
            c.set(5,16,rm(m,c.r[6].wrapping_add(0x65c),16));
            pc = 0x5fdac8; continue;
            }
            0x5fdac8 => {
            { let a = c.get(5,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x5fdbee; continue; }
            pc = 0x5fdad3; continue;
            }
            0x5fdad3 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            { let a = rm(m,c.r[1].wrapping_add(0x12a),8); c.and(a,0x6u32,8); }
            if c.cond(5) { pc = 0x5fdbdc; continue; }
            pc = 0x5fdaec; continue;
            }
            0x5fdaec => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.push(m, 0x5fdafb);
            { let args = []; c.r[0] = host.call(m, 0x62125b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xa90),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xa90); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x40),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0xa98),16),16); }
            if c.cond(6) { pc = 0x5fdb37; continue; }
            pc = 0x5fdb2f; continue;
            }
            0x5fdb2f => {
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0xa98); wm(m,a,16,c.get(0,16)); }
            pc = 0x5fdb37; continue;
            }
            0x5fdb37 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x154),16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x5fdb5d; continue; }
            pc = 0x5fdb59; continue;
            }
            0x5fdb59 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5fdb5f; continue;
            }
            0x5fdb5d => {
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            pc = 0x5fdb5f; continue;
            }
            0x5fdb5f => {
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0xa9c),16),16); }
            if c.cond(6) { pc = 0x5fdb71; continue; }
            pc = 0x5fdb69; continue;
            }
            0x5fdb69 => {
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0xa9c); wm(m,a,16,c.get(0,16)); }
            pc = 0x5fdb71; continue;
            }
            0x5fdb71 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xaa0),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x120),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x5fdb9e; continue; }
            pc = 0x5fdb97; continue;
            }
            0x5fdb97 => {
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xaa0); wm(m,a,32,c.get(0,32)); }
            pc = 0x5fdb9e; continue;
            }
            0x5fdb9e => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(c.r[7]).wrapping_add(0x12),8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            { let a = c.r[4].wrapping_add(c.r[7]).wrapping_add(0x12); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xab4),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xab4); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xabc),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(4)).wrapping_add(0xabc); wm(m,a,32,c.get(1,32)); }
            pc = 0x5fdbdc; continue;
            }
            0x5fdbdc => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(5,16,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(2)),16));
            { let a = c.get(5,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x5fdad3; continue; }
            pc = 0x5fdbee; continue;
            }
            0x5fdbee => {
            { let a = c.get(3,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x5fdcb4; continue; }
            pc = 0x5fdbf7; continue;
            }
            0x5fdbf7 => {
            c.set(2,32,c.r[6].wrapping_add(0x1ec4));
            c.set(1,32,c.r[6].wrapping_add(0x1e99));
            c.set(5,32,c.r[6].wrapping_add(0x1ea6));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x4u32); }
            pc = 0x5fdc15; continue;
            }
            0x5fdc15 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(7,32,rm(m,c.r[0],32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5fdc96; continue; }
            pc = 0x5fdc1f; continue;
            }
            0x5fdc1f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[1],8));
            c.set(2,8,rm(m,0x672cccu32,8));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,0x672cc4u32,8));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xa94),32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0xau32; let r = c.shr(a,n,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(7,32),32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0xa94); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xaa4),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x5fdc60; continue; }
            pc = 0x5fdc5a; continue;
            }
            0x5fdc5a => {
            { let a = c.r[6].wrapping_add(0xaa4); wm(m,a,32,c.get(0,32)); }
            pc = 0x5fdc60; continue;
            }
            0x5fdc60 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xab8),32));
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xfffffff8),16));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xac0),32));
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0xab8); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[5],16));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xac0); wm(m,a,32,c.get(0,32)); }
            pc = 0x5fdc96; continue;
            }
            0x5fdc96 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.add(a,0x2u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5fdc15; continue; }
            pc = 0x5fdcb4; continue;
            }
            0x5fdcb4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(c.r[1]).wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5fdcf4; continue; }
            pc = 0x5fdcc6; continue;
            }
            0x5fdcc6 => {
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xab4),32));
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xab4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xabc),32));
            { let b = c.get(7,32); c.div32(b); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xabc); wm(m,a,32,c.get(0,32)); }
            pc = 0x5fdd0a; continue;
            }
            0x5fdcf4 => {
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xab4); wm(m,a,32,0x880u32); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xabc); wm(m,a,32,0x781u32); }
            pc = 0x5fdd0a; continue;
            }
            0x5fdd0a => {
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            { let a = c.get(3,8); c.sub(a,0x2u32,8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            if c.cond(2) { pc = 0x5fdab7; continue; }
            pc = 0x5fdd19; continue;
            }
            0x5fdd19 => {
            c.set(3,8,rm(m,c.r[6].wrapping_add(0xa8c),8));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            c.set(0,8,c.get(3,8));
            { let a = c.r[6].wrapping_add(0xa8c); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3e8u32,32); }
            if c.cond(12) { pc = 0x5fdd42; continue; }
            pc = 0x5fdd3b; continue;
            }
            0x5fdd3b => {
            { let a = c.r[6].wrapping_add(0xa8c); wm(m,a,8,0x0u32); }
            pc = 0x5fdd42; continue;
            }
            0x5fdd42 => {
            { let a = rm(m,c.r[6].wrapping_add(0x5c),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x5fdd62; continue; }
            pc = 0x5fdd48; continue;
            }
            0x5fdd48 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x15c),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x5fe020; continue; }
            pc = 0x5fdd58; continue;
            }
            0x5fdd58 => {
            { let a = c.get(0,16); c.sub(a,0xf000u32,16); }
            if c.cond(7) { pc = 0x5fe020; continue; }
            pc = 0x5fdd62; continue;
            }
            0x5fdd62 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5fdd69);
            { let args = []; c.r[0] = host.call(m, 0x6113c9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x1u32); }
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            c.set(3,8,0x1u32);
            if c.cond(4) { pc = 0x5fddae; continue; }
            pc = 0x5fdd7a; continue;
            }
            0x5fdd7a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = rm(m,c.r[1].wrapping_add(0x12a),8); c.and(a,0x6u32,8); }
            if c.cond(5) { pc = 0x5fdd96; continue; }
            pc = 0x5fdd91; continue;
            }
            0x5fdd91 => {
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,0x0u32); }
            pc = 0x5fdd96; continue;
            }
            0x5fdd96 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x670),8));
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(0,32));
            { let target = rm(m,c.r[2],32); c.push(m, 0x5fdda1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(2)),16));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x5fdd7a; continue; }
            pc = 0x5fddae; continue;
            }
            0x5fddae => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x65c),16));
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x5fddec; continue; }
            pc = 0x5fddbc; continue;
            }
            0x5fddbc => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = rm(m,c.r[1].wrapping_add(0x12a),8); c.and(a,0x6u32,8); }
            if c.cond(5) { pc = 0x5fddd3; continue; }
            pc = 0x5fddd1; continue;
            }
            0x5fddd1 => {
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            pc = 0x5fddd3; continue;
            }
            0x5fddd3 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x670),8));
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,c.get(2,32));
            { let target = rm(m,c.r[0],32); c.push(m, 0x5fddde); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)),16));
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x5fddbc; continue; }
            pc = 0x5fddec; continue;
            }
            0x5fddec => {
            c.set(1,32,c.r[6].wrapping_add(0x1ec4));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x5fddf8; continue;
            }
            0x5fddf8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = rm(m,c.r[2],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x5fdf3f; continue; }
            pc = 0x5fde05; continue;
            }
            0x5fde05 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x5fdf35; continue; }
            pc = 0x5fde0d; continue;
            }
            0x5fde0d => {
            { let a = rm(m,c.r[6].wrapping_add(0x2f18),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x5fdf35; continue; }
            pc = 0x5fde1b; continue;
            }
            0x5fde1b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5e),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fdf35; continue; }
            pc = 0x5fde26; continue;
            }
            0x5fde26 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5fdf35; continue; }
            pc = 0x5fde43; continue;
            }
            0x5fde43 => {
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x35a),16),16); }
            if c.cond(4) { pc = 0x5fde55; continue; }
            pc = 0x5fde4c; continue;
            }
            0x5fde4c => {
            { let a = c.r[6].wrapping_add(0x2f18); wm(m,a,16,0xfau32); }
            pc = 0x5fde55; continue;
            }
            0x5fde55 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xac4),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fdf35; continue; }
            pc = 0x5fde63; continue;
            }
            0x5fde63 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xac6),16));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = rm(m,c.r[1].wrapping_add(0x13d),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x5fdf35; continue; }
            pc = 0x5fde7f; continue;
            }
            0x5fde7f => {
            { let a = c.r[6].wrapping_add(0xac4); wm(m,a,8,0x1u32); }
            c.set(2,32,rm(m,0x6a1da4u32,32));
            c.set(7,32,c.r[6].wrapping_add(0x1310));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5fde99);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[6].wrapping_add(0x12ec));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,0xffu32); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,0x3fb00000u32); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            { let a = c.r[0].wrapping_add(0x16); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0xf); wm(m,a,8,0xffu32); }
            { let a = c.r[0].wrapping_add(0xd); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0].wrapping_add(0xe); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x12f2); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0x12f4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x12f0); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x12f8); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x12fc); wm(m,a,32,0x3f3504f4u32); }
            { let a = c.r[6].wrapping_add(0x1300); wm(m,a,8,0x1u32); }
            c.set(1,8,rm(m,0x6e59cdu32,8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5fdf14; continue; }
            pc = 0x5fdf0b; continue;
            }
            0x5fdf0b => {
            { let a = c.r[6].wrapping_add(0x12e9); wm(m,a,8,0x1u32); }
            pc = 0x5fdf35; continue;
            }
            0x5fdf14 => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5fdf20);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x12e9); wm(m,a,8,0x0u32); }
            { let a = 0x6e59cdu32; wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x12ea); wm(m,a,8,0x1u32); }
            pc = 0x5fdf35; continue;
            }
            0x5fdf35 => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m, 0x5fdf3f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5fe080, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5fdf3f; continue;
            }
            0x5fdf3f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(5,16); c.sub(a,0x4u32,16); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            if c.cond(2) { pc = 0x5fddf8; continue; }
            pc = 0x5fdf55; continue;
            }
            0x5fdf55 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0xacc),16); c.sub(a,c.get(7,16),16); }
            if c.cond(6) { pc = 0x5fdf83; continue; }
            pc = 0x5fdf60; continue;
            }
            0x5fdf60 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x668),32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5fdf79; continue; }
            pc = 0x5fdf74; continue;
            }
            0x5fdf74 => {
            c.push(m, 0x5fdf79);
            { let args = []; c.r[0] = host.call(m, 0x603130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5fdf79; continue;
            }
            0x5fdf79 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xacc),16),16); }
            if c.cond(2) { pc = 0x5fdf60; continue; }
            pc = 0x5fdf83; continue;
            }
            0x5fdf83 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fdf9f; continue; }
            pc = 0x5fdf8a; continue;
            }
            0x5fdf8a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x11),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fdf96; continue; }
            pc = 0x5fdf92; continue;
            }
            0x5fdf92 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x5fdfab; continue; }
            pc = 0x5fdf96; continue;
            }
            0x5fdf96 => {
            { let a = c.r[6].wrapping_add(0x15c); wm(m,a,16,0xfffeu32); }
            pc = 0x5fdf9f; continue;
            }
            0x5fdf9f => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x11),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5fdfaf; continue; }
            pc = 0x5fdfa7; continue;
            }
            0x5fdfa7 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x5fdfaf; continue; }
            pc = 0x5fdfab; continue;
            }
            0x5fdfab => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x5fdfb1; continue;
            }
            0x5fdfaf => {
            c.set(0,8,0x1u32);
            pc = 0x5fdfb1; continue;
            }
            0x5fdfb1 => {
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x5c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x5fdfbb);
            { let args = []; c.r[0] = host.call(m, 0x6197f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12e0),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5fe002; continue; }
            pc = 0x5fdfc5; continue;
            }
            0x5fdfc5 => {
            { let a = rm(m,c.r[6].wrapping_add(0xad0),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x5fe002; continue; }
            pc = 0x5fdfcf; continue;
            }
            0x5fdfcf => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x12dc),32));
            c.push(m,0x1u32);
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,0x2e8ba2e9u32);
            c.set(1,32,c.r[2].wrapping_add(c.r[2]).wrapping_add(0xb));
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x12d8),32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5fe002);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x608550, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x5fe002; continue;
            }
            0x5fe002 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x3c),16),16); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x5fda69; continue; }
            pc = 0x5fe016; continue;
            }
            0x5fe016 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x5fe020 => {
            { let a = rm(m,c.r[6].wrapping_add(0x658),16); c.sub(a,0x0u32,16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            if c.cond(6) { pc = 0x5fe05f; continue; }
            pc = 0x5fe02f; continue;
            }
            0x5fe02f => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x5fe05f; continue; }
            pc = 0x5fe039; continue;
            }
            0x5fe039 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            pc = 0x5fe03f; continue;
            }
            0x5fe03f => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = rm(m,c.r[7].wrapping_add(0x12a),8); c.and(a,0x6u32,8); }
            if c.cond(5) { pc = 0x5fe055; continue; }
            pc = 0x5fe050; continue;
            }
            0x5fe050 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            pc = 0x5fe055; continue;
            }
            0x5fe055 => {
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x5fe03f; continue; }
            pc = 0x5fe05f; continue;
            }
            0x5fe05f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[2].wrapping_add(0x1c),32); c.push(m, 0x5fe06b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5fe06b; continue;
            }
            0x5fe06b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005fe7e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5fe7e0;
    loop {
        match pc {
            0x5fe7e0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6681f4u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x244u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x15c),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x5fe817; continue; }
            pc = 0x5fe80d; continue;
            }
            0x5fe80d => {
            { let a = c.get(0,16); c.sub(a,0xf000u32,16); }
            if c.cond(2) { pc = 0x5ff89c; continue; }
            pc = 0x5fe817; continue;
            }
            0x5fe817 => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5fe84a; continue; }
            pc = 0x5fe829; continue;
            }
            0x5fe829 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),32));
            { let a = c.get(5,8); let r = c.and(a,0xf7u32,8); c.set(5,8,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x5fe84a; continue;
            }
            0x5fe84a => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x264),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x558),8));
            { let a = c.r[6].wrapping_add(0x130e); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x65e),16));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x5fe94b; continue; }
            pc = 0x5fe877; continue;
            }
            0x5fe877 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16));
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x5fe94b; continue; }
            pc = 0x5fe8a2; continue;
            }
            0x5fe8a2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(2,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x35a));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            pc = 0x5fe8b7; continue;
            }
            0x5fe8b7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            if c.cond(6) { pc = 0x5fe8fb; continue; }
            pc = 0x5fe8c6; continue;
            }
            0x5fe8c6 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x5fe8ce; continue;
            }
            0x5fe8ce => {
            c.set(3,16,rm(m,c.r[0].wrapping_add(c.r[5]),8));
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x35a),16),16); }
            if c.cond(5) { pc = 0x5fe8e2; continue; }
            pc = 0x5fe8dd; continue;
            }
            0x5fe8dd => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x5fe8e2; continue;
            }
            0x5fe8e2 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x5fe8ce; continue; }
            pc = 0x5fe8f3; continue;
            }
            0x5fe8f3 => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x5fe930; continue; }
            pc = 0x5fe8fb; continue;
            }
            0x5fe8fb => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[5]),8));
            { let a = c.r[2]; wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x5d); wm(m,a,8,0x1u32); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x559); wm(m,a,8,0x0u32); }
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(2,32)); }
            pc = 0x5fe930; continue;
            }
            0x5fe930 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[5]).wrapping_add(0x6),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x5fe8b7; continue; }
            pc = 0x5fe94b; continue;
            }
            0x5fe94b => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x5fe954);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            c.set(5,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x25c); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(3) { pc = 0x5fea85; continue; }
            pc = 0x5fe97a; continue;
            }
            0x5fe97a => {
            { let a = rm(m,c.r[6].wrapping_add(0x65a),16); c.sub(a,0x0u32,16); }
            if c.cond(6) { pc = 0x5fea85; continue; }
            pc = 0x5fe988; continue;
            }
            0x5fe988 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            { let a = c.get(1,16); c.sub(a,0xffu32,16); }
            if c.cond(4) { pc = 0x5fea85; continue; }
            pc = 0x5fe99a; continue;
            }
            0x5fe99a => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x264),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x5fed86; continue; }
            pc = 0x5fe9a9; continue;
            }
            0x5fe9a9 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x65c),16));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x5fea94; continue; }
            pc = 0x5fe9ba; continue;
            }
            0x5fe9ba => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4),32));
            pc = 0x5fe9bd; continue;
            }
            0x5fe9bd => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = rm(m,c.r[3].wrapping_add(0x12a),8); c.and(a,0x2u32,8); }
            if c.cond(5) { pc = 0x5fe9e0; continue; }
            pc = 0x5fe9ce; continue;
            }
            0x5fe9ce => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,16,rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x5fe9bd; continue; }
            pc = 0x5fe9db; continue;
            }
            0x5fe9db => {
            pc = 0x5fea94; continue;
            }
            0x5fe9e0 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x3bu32); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x21); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5fe9fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,16,c.get(0,16)); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5fea18);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x1c),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,16,c.get(2,16)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5fea36);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x1d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x5fea59);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5fea76);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5fea7d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f8ca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,0x6de4b0u32,32));
            pc = 0x5fea94; continue;
            }
            0x5fea85 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x264),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5fed86; continue; }
            pc = 0x5fea94; continue;
            }
            0x5fea94 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(3) { pc = 0x5fec0e; continue; }
            pc = 0x5feaa4; continue;
            }
            0x5feaa4 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x65e),16));
            c.set(2,32,c.r[4].wrapping_add(0x52));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(2,32,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6df270u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m, 0x5feace);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x633db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5feadd; continue; }
            pc = 0x5fead2; continue;
            }
            0x5fead2 => {
            c.push(m,0xfu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5feadb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x611649, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5feae6; continue;
            }
            0x5feadd => {
            { let a = c.r[6].wrapping_add(0x1e94); wm(m,a,16,0x0u32); }
            pc = 0x5feae6; continue;
            }
            0x5feae6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x65e),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            c.set(1,8,c.get(3,8));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x5feb4c; continue; }
            pc = 0x5feb0f; continue;
            }
            0x5feb0f => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x5feb24);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4b8),32));
            { let a = c.get(0,32); let r = c.add(a,0x4b8u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5feb42);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            pc = 0x5feb4c; continue;
            }
            0x5feb4c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            { let a = c.get(0,16); c.sub(a,0xffu32,16); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x3au32); }
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x27); wm(m,a,8,c.get(3,8)); }
            if c.cond(5) { pc = 0x5feb86; continue; }
            pc = 0x5feb74; continue;
            }
            0x5feb74 => {
            c.set(2,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x26); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(2,32));
            pc = 0x5feea3; continue;
            }
            0x5feb86 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5feb96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            c.push(m,c.get(2,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x39c),8));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x5febba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x1c),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5febd5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5febf0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5febf7);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x24),8));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            { let a = c.get(0,8); let n = 0x4u32; let r = c.shl(a,n,8); c.set(0,8,r); }
            { let a = c.get(1,8); let r = c.or(a,c.get(0,8),8); c.set(1,8,r); }
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(1,8)); }
            pc = 0x5feea3; continue;
            }
            0x5fec0e => {
            c.set(1,32,c.r[0].wrapping_mul(4));
            c.set(2,32,0x6ddba4u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x16),16));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5fec2f);
            { let args = []; c.r[0] = host.call(m, 0x60d259, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x65e),16));
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x5fecc4; continue; }
            pc = 0x5fec6f; continue;
            }
            0x5fec6f => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x5fec84);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x4b8),32));
            { let a = c.get(0,32); let r = c.add(a,0x4b8u32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x5feca2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5fecb7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(0,8)); }
            pc = 0x5fecce; continue;
            }
            0x5fecc4 => {
            c.set(2,8,rm(m,0x6de52eu32,8));
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(2,8)); }
            pc = 0x5fecce; continue;
            }
            0x5fecce => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x35a),16));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x3au32); }
            c.push(m, 0x5feceb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x35a),16));
            c.push(m,c.get(2,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x39c),8));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x5fed0f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x1c),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x35a),16));
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5fed2a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x35a),16));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5fed45);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5fed4c);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1e90),8));
            c.set(1,8,c.get(0,8));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x24),8));
            { let a = c.get(1,8); let n = 0x4u32; let r = c.shl(a,n,8); c.set(1,8,r); }
            { let a = c.get(2,32); let n = 0x18u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,8); let r = c.or(a,c.get(1,8),8); c.set(0,8,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1e91),8));
            { let a = c.get(7,32); let r = c.or(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,8); let r = c.or(a,0x80u32,8); c.set(0,8,r); }
            c.set(2,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x27); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(2,32));
            pc = 0x5feea3; continue;
            }
            0x5fed86 => {
            { let a = rm(m,c.r[6].wrapping_add(0xac4),8); c.sub(a,0x3u32,8); }
            if c.cond(4) { pc = 0x5feeaf; continue; }
            pc = 0x5fed93; continue;
            }
            0x5fed93 => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x39u32); }
            if c.cond(3) { pc = 0x5fee49; continue; }
            pc = 0x5feda0; continue;
            }
            0x5feda0 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,16); c.sub(a,0xffu32,16); }
            if c.cond(5) { pc = 0x5fedc8; continue; }
            pc = 0x5fedb1; continue;
            }
            0x5fedb1 => {
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x23); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,0x0u32); }
            c.push(m,c.get(0,32));
            pc = 0x5feea3; continue;
            }
            0x5fedc8 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5fedd8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5feddf);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5fedf7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            c.push(m,c.get(2,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x39c),8));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x5fee1b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x1c),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x35c),16));
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5fee36);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1d),8));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x23); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,0x0u32); }
            c.push(m,c.get(0,32));
            pc = 0x5feea3; continue;
            }
            0x5fee49 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x35a),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5fee5d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x19),8));
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5fee6d);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x19),8));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x39c),8));
            { let a = c.r[4].wrapping_add(0x21); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1c),8));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1e90),8));
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1d),8));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x23); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,0x1u32); }
            c.push(m,c.get(1,32));
            pc = 0x5feea3; continue;
            }
            0x5feea3 => {
            c.set(1,32,0x6dd730u32);
            c.push(m,0xfffffffeu32);
            c.push(m, 0x5feeaf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5feeaf; continue;
            }
            0x5feeaf => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(2,32); let r = c.add(a,0x8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5feec1);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5feed7);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(5,32,c.get(0,32));
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x5feef9; continue; }
            pc = 0x5feeeb; continue;
            }
            0x5feeeb => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x65c),16));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x5feefe; continue;
            }
            0x5feef9 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x5feefe; continue;
            }
            0x5feefe => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,0x0u32); }
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x5fef1b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(2,32); let r = c.add(a,0x8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5fef31);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1]; wm(m,a,8,c.get(3,8)); }
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(2,32); let r = c.add(a,0x8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x5fef4f);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x14),16));
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5fef65);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1]; wm(m,a,8,c.get(3,8)); }
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            if c.cond(4) { pc = 0x5ff1e0; continue; }
            pc = 0x5fef7f; continue;
            }
            0x5fef7f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x154); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x160));
            c.set(1,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5fefb4);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12a),8));
            { let a = c.get(1,8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x5ff005; continue; }
            pc = 0x5fefc8; continue;
            }
            0x5fefc8 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            c.set(2,32,c.r[0].wrapping_add(0x160));
            c.set(0,32,rm(m,0x6a2270u32,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a174c),32));
            c.set(1,32,c.r[4].wrapping_add(0x58));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5feffd);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x5ff157; continue;
            }
            0x5ff005 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            { let a = c.get(2,8); c.sub(a,rm(m,c.r[0].wrapping_add(0x142),8),8); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            if c.cond(4) { pc = 0x5ff0b6; continue; }
            pc = 0x5ff01a; continue;
            }
            0x5ff01a => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[1].wrapping_add(0x14),32); c.idiv32(b); }
            c.set(2,32,c.r[1].wrapping_add(0x160));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6a2274u32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a174c),32));
            c.set(0,32,c.r[4].wrapping_add(0x5c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ff05e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[1].wrapping_add(0x14),32); c.idiv32(b); }
            c.set(2,32,c.r[1].wrapping_add(0x160));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6a226cu32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a174c),32));
            c.set(0,32,c.r[4].wrapping_add(0x170));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ff0ae);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x28u32,32); c.set(4,32,r); }
            pc = 0x5ff157; continue;
            }
            0x5ff0b6 => {
            { let a = c.get(1,8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x5ff0fc; continue; }
            pc = 0x5ff0bb; continue;
            }
            0x5ff0bb => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[1].wrapping_add(0x14),32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_add(0x160));
            c.set(1,32,rm(m,0x6a2278u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a174c),32));
            c.set(2,32,c.r[4].wrapping_add(0x5c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x5ff13b; continue;
            }
            0x5ff0fc => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[1].wrapping_add(0x14),32); c.idiv32(b); }
            c.set(2,32,c.r[1].wrapping_add(0x160));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xe),8));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,0x6a226cu32,32));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a174c),32));
            c.set(0,32,c.r[4].wrapping_add(0x5c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x5ff13b; continue;
            }
            0x5ff13b => {
            c.push(m, 0x5ff140);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x142),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5ff157; continue; }
            pc = 0x5ff153; continue;
            }
            0x5ff153 => {
            { let a = rm(m,c.r[4].wrapping_add(0x2c),8); let r = c.inc(a,8); { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,r); } }
            pc = 0x5ff157; continue;
            }
            0x5ff157 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x142),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5ff187; continue; }
            pc = 0x5ff167; continue;
            }
            0x5ff167 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(2,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.get(3,8); let r = c.inc(a,8); c.set(3,8,r); }
            c.push(m, 0x5ff178);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,c.r[4].wrapping_add(0x15c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x5ff1a3; continue;
            }
            0x5ff187 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ff196);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,c.r[4].wrapping_add(0x15c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x5ff1a3; continue;
            }
            0x5ff1a3 => {
            c.push(m, 0x5ff1a8);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(7,16,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(2)),16));
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x5fef7f; continue; }
            pc = 0x5ff1bd; continue;
            }
            0x5ff1bd => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5ff1d1; continue; }
            pc = 0x5ff1c5; continue;
            }
            0x5ff1c5 => {
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x65c),16));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            pc = 0x5ff1d1; continue;
            }
            0x5ff1d1 => {
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x5fef7f; continue; }
            pc = 0x5ff1dc; continue;
            }
            0x5ff1dc => {
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,8,c.get(3,8)); }
            pc = 0x5ff1e0; continue;
            }
            0x5ff1e0 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x264),8));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5ff29d; continue; }
            pc = 0x5ff1f5; continue;
            }
            0x5ff1f5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5ff258; continue; }
            pc = 0x5ff201; continue;
            }
            0x5ff201 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xac4),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            { let a = c.get(1,8); c.sub(a,0x3u32,8); }
            c.set(1,32,rm(m,0x6a228cu32,32));
            c.push(m,c.get(0,32));
            if c.cond(4) { pc = 0x5ff225; continue; }
            pc = 0x5ff21f; continue;
            }
            0x5ff21f => {
            c.set(1,32,rm(m,0x6a2288u32,32));
            pc = 0x5ff225; continue;
            }
            0x5ff225 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5ff22c);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6a2284u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5ff250);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5ff3ca; continue;
            }
            0x5ff258 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1e91),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x5ff270);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1e90),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5ff284);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6aa1a4u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5ff291);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[1]; wm(m,a,8,0x0u32); }
            pc = 0x5ff3c7; continue;
            }
            0x5ff29d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5ff337; continue; }
            pc = 0x5ff2ad; continue;
            }
            0x5ff2ad => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            if c.cond(14) { pc = 0x5ff303; continue; }
            pc = 0x5ff2cf; continue;
            }
            0x5ff2cf => {
            c.set(0,32,rm(m,0x6a2290u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5ff2db);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6a227cu32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ff2fb);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x5ff3ca; continue;
            }
            0x5ff303 => {
            c.set(0,32,rm(m,0x6a2294u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5ff30f);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6a2280u32,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5ff32f);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x5ff3ce; continue;
            }
            0x5ff337 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x5ff386; continue; }
            pc = 0x5ff351; continue;
            }
            0x5ff351 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1e91),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x5ff369);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1e90),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x5ff37d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6aa1a8u32,32));
            c.push(m,c.get(0,32));
            pc = 0x5ff3ba; continue;
            }
            0x5ff386 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1e91),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x5ff39e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1e90),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5ff3b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6aa1acu32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x5ff3ba; continue;
            }
            0x5ff3ba => {
            c.push(m,c.get(5,32));
            c.push(m, 0x5ff3c0);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[2]; wm(m,a,8,0x0u32); }
            pc = 0x5ff3c7; continue;
            }
            0x5ff3c7 => {
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x5ff3ca; continue;
            }
            0x5ff3ca => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x5ff3ce; continue;
            }
            0x5ff3ce => {
            c.set(0,32,rm(m,0x6a2260u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5ff3da);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5ff3e5);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[3],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x5ff408; continue; }
            pc = 0x5ff3f1; continue;
            }
            0x5ff3f1 => {
            c.set(2,32,rm(m,0x6a2264u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5ff3fe);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5ff405);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x5ff408; continue;
            }
            0x5ff408 => {
            c.set(0,32,rm(m,0x6a2268u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5ff414);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5ff41f);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6a2260u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5ff42c);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5ff433);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[3],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x5ff456; continue; }
            pc = 0x5ff43f; continue;
            }
            0x5ff43f => {
            c.set(2,32,rm(m,0x6a2264u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5ff44c);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5ff453);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x5ff456; continue;
            }
            0x5ff456 => {
            c.set(0,32,rm(m,0x6a2268u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5ff462);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5ff46d);
            { let args = []; c.r[0] = host.call(m, 0x63ade0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x5ff491; continue; }
            pc = 0x5ff47e; continue;
            }
            0x5ff47e => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[7]),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x5ff47e; continue; }
            pc = 0x5ff48d; continue;
            }
            0x5ff48d => {
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            pc = 0x5ff491; continue;
            }
            0x5ff491 => {
            c.set(1,8,rm(m,c.r[5],8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x5ff4b1; continue; }
            pc = 0x5ff49e; continue;
            }
            0x5ff49e => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[5]),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x5ff49e; continue; }
            pc = 0x5ff4ad; continue;
            }
            0x5ff4ad => {
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            pc = 0x5ff4b1; continue;
            }
            0x5ff4b1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(5,32)); }
            if c.cond(6) { pc = 0x5ff829; continue; }
            pc = 0x5ff4c3; continue;
            }
            0x5ff4c3 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5ff4c5; continue;
            }
            0x5ff4c5 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x35a),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x5ff810; continue; }
            pc = 0x5ff4da; continue;
            }
            0x5ff4da => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x5d),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5ff575; continue; }
            pc = 0x5ff4e6; continue;
            }
            0x5ff4e6 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            c.set(2,32,rm(m,0x6de4b0u32,32));
            c.set(5,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x29u32); }
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x5ff509; continue; }
            pc = 0x5ff503; continue;
            }
            0x5ff503 => {
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(1,8)); }
            pc = 0x5ff520; continue;
            }
            0x5ff509 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x5ff519);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x19),8));
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(2,8)); }
            pc = 0x5ff520; continue;
            }
            0x5ff520 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x559),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5ff535; continue; }
            pc = 0x5ff52b; continue;
            }
            0x5ff52b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            pc = 0x5ff53d; continue;
            }
            0x5ff535 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            pc = 0x5ff53d; continue;
            }
            0x5ff53d => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x35a),16));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x5ff564);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            pc = 0x5ff810; continue;
            }
            0x5ff575 => {
            { let a = c.get(5,16); c.sub(a,0x2u32,16); }
            if c.cond(3) { pc = 0x5ff810; continue; }
            pc = 0x5ff57f; continue;
            }
            0x5ff57f => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x35a),16));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x264),8));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x35c),16),16); }
            if c.cond(5) { pc = 0x5ff5a3; continue; }
            pc = 0x5ff596; continue;
            }
            0x5ff596 => {
            { let a = c.get(1,8); c.sub(a,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x559),8),8); }
            if c.cond(5) { pc = 0x5ff810; continue; }
            pc = 0x5ff5a3; continue;
            }
            0x5ff5a3 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5ff810; continue; }
            pc = 0x5ff5b6; continue;
            }
            0x5ff5b6 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xac4),8));
            { let a = c.get(0,8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x5ff810; continue; }
            pc = 0x5ff5c4; continue;
            }
            0x5ff5c4 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x559),8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x5ff6d3; continue; }
            pc = 0x5ff5d8; continue;
            }
            0x5ff5d8 => {
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x5ff770; continue; }
            pc = 0x5ff5e1; continue;
            }
            0x5ff5e1 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x5ff8b8)); continue;
            }
            0x5ff5e8 => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5ff630; continue; }
            pc = 0x5ff5ec; continue;
            }
            0x5ff5ec => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,c.r[6].wrapping_add(0x1310));
            if c.cond(3) { pc = 0x5ff61c; continue; }
            pc = 0x5ff608; continue;
            }
            0x5ff608 => {
            c.set(0,32,rm(m,0x6a1da8u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ff614);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5ff770; continue;
            }
            0x5ff61c => {
            c.set(0,32,rm(m,0x6aa198u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ff628);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5ff770; continue;
            }
            0x5ff630 => {
            c.set(2,32,rm(m,0x6a1d9cu32,32));
            c.set(0,32,c.r[6].wrapping_add(0x1310));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x5ff768; continue;
            }
            0x5ff643 => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5ff677; continue; }
            pc = 0x5ff647; continue;
            }
            0x5ff647 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            c.set(1,32,c.r[6].wrapping_add(0x1310));
            if c.cond(3) { pc = 0x5ff61c; continue; }
            pc = 0x5ff663; continue;
            }
            0x5ff663 => {
            c.set(0,32,rm(m,0x6a1da8u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ff66f);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5ff770; continue;
            }
            0x5ff677 => {
            c.set(2,32,rm(m,0x6a1d98u32,32));
            c.set(0,32,c.r[6].wrapping_add(0x1310));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x5ff768; continue;
            }
            0x5ff68a => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5ff829; continue; }
            pc = 0x5ff692; continue;
            }
            0x5ff692 => {
            { let a = rm(m,c.r[6].wrapping_add(0x1e94),16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x5ff6bb; continue; }
            pc = 0x5ff69b; continue;
            }
            0x5ff69b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,0x6a1dacu32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[6].wrapping_add(0x1310));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ff6b3);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x5ff770; continue;
            }
            0x5ff6bb => {
            c.set(1,32,rm(m,0x6aa1e4u32,32));
            c.set(2,32,c.r[6].wrapping_add(0x1310));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5ff6ce);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ff76d; continue;
            }
            0x5ff6d3 => {
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(7) { pc = 0x5ff770; continue; }
            pc = 0x5ff6dc; continue;
            }
            0x5ff6dc => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x5ff8c8)); continue;
            }
            0x5ff6e3 => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5ff6ee; continue; }
            pc = 0x5ff6e7; continue;
            }
            0x5ff6e7 => {
            c.set(0,32,rm(m,0x6a1da4u32,32));
            pc = 0x5ff760; continue;
            }
            0x5ff6ee => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5ff706; continue; }
            pc = 0x5ff6ff; continue;
            }
            0x5ff6ff => {
            c.set(0,32,rm(m,0x6a1db4u32,32));
            pc = 0x5ff760; continue;
            }
            0x5ff706 => {
            c.set(2,32,rm(m,0x6aa1a0u32,32));
            c.set(0,32,c.r[6].wrapping_add(0x1310));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x5ff768; continue;
            }
            0x5ff716 => {
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5ff72a; continue; }
            pc = 0x5ff71a; continue;
            }
            0x5ff71a => {
            c.set(1,32,rm(m,0x6a1da0u32,32));
            c.set(2,32,c.r[6].wrapping_add(0x1310));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x5ff768; continue;
            }
            0x5ff72a => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1e98),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5ff74b; continue; }
            pc = 0x5ff73b; continue;
            }
            0x5ff73b => {
            c.set(2,32,rm(m,0x6a1db0u32,32));
            c.set(0,32,c.r[6].wrapping_add(0x1310));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x5ff768; continue;
            }
            0x5ff74b => {
            c.set(1,32,rm(m,0x6aa19cu32,32));
            c.set(2,32,c.r[6].wrapping_add(0x1310));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x5ff768; continue;
            }
            0x5ff75b => {
            c.set(0,32,rm(m,0x6a1db8u32,32));
            pc = 0x5ff760; continue;
            }
            0x5ff760 => {
            c.set(1,32,c.r[6].wrapping_add(0x1310));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            pc = 0x5ff768; continue;
            }
            0x5ff768 => {
            c.push(m, 0x5ff76d);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ff76d; continue;
            }
            0x5ff76d => {
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x5ff770; continue;
            }
            0x5ff770 => {
            c.set(0,32,c.r[6].wrapping_add(0x12ec));
            c.set(2,32,c.r[6].wrapping_add(0x1310));
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,0xffu32); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,0x3fb00000u32); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            { let a = c.r[0].wrapping_add(0x16); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0xf); wm(m,a,8,0xffu32); }
            { let a = c.r[0].wrapping_add(0xd); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0xe); wm(m,a,8,0x0u32); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x12f2); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x12f4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x12f0); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x12f8); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x12fc); wm(m,a,32,0x3f3504f4u32); }
            { let a = c.r[6].wrapping_add(0x1300); wm(m,a,8,0x1u32); }
            c.set(1,8,rm(m,0x6e59cdu32,8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5ff7ef; continue; }
            pc = 0x5ff7e6; continue;
            }
            0x5ff7e6 => {
            { let a = c.r[6].wrapping_add(0x12e9); wm(m,a,8,0x1u32); }
            pc = 0x5ff810; continue;
            }
            0x5ff7ef => {
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ff7fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x12e9); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x12ea); wm(m,a,8,0x1u32); }
            { let a = 0x6e59cdu32; wm(m,a,8,0x1u32); }
            pc = 0x5ff810; continue;
            }
            0x5ff810 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.set(7,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5ff4c5; continue; }
            pc = 0x5ff829; continue;
            }
            0x5ff829 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xac8),8));
            { let a = c.r[6].wrapping_add(0x5c); wm(m,a,8,0x1u32); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[6].wrapping_add(0x15c); wm(m,a,16,0xfau32); }
            if c.cond(4) { pc = 0x5ff849; continue; }
            pc = 0x5ff840; continue;
            }
            0x5ff840 => {
            { let a = c.r[6].wrapping_add(0x15c); wm(m,a,16,0x1u32); }
            pc = 0x5ff849; continue;
            }
            0x5ff849 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ff853);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ff85d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5ff867);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5ff871);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5ff87b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5ff885);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x25c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5ff89c);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x5ff89c; continue;
            }
            0x5ff89c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x254),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x250u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00621770(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x621770;
    loop {
        match pc {
            0x621770 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0xc0u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.r[5].wrapping_add(0xffffff6c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe4));
            c.push(m, 0x621789);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x11),8));
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe3); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6217fc; continue; }
            pc = 0x6217b2; continue;
            }
            0x6217b2 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe3),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x18u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x6217fc; continue; }
            pc = 0x6217c2; continue;
            }
            0x6217c2 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe3),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x10u32,32); }
            if c.cond(13) { pc = 0x6217e7; continue; }
            pc = 0x6217d0; continue;
            }
            0x6217d0 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x11),8));
            { let a = c.get(1,8); let r = c.sub(a,0x1u32,8); c.set(1,8,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x11); wm(m,a,8,c.get(1,8)); }
            pc = 0x6217fc; continue;
            }
            0x6217e7 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x11),8));
            { let a = c.get(1,8); let r = c.add(a,0x1u32,8); c.set(1,8,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x11); wm(m,a,8,c.get(1,8)); }
            pc = 0x6217fc; continue;
            }
            0x6217fc => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12a),8));
            { let a = c.get(1,32); let r = c.and(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x621918; continue; }
            pc = 0x621815; continue;
            }
            0x621815 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = rm(m,c.r[2].wrapping_add(0x1c),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x621918; continue; }
            pc = 0x621825; continue;
            }
            0x621825 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = rm(m,c.r[0].wrapping_add(0x15c),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x6218c3; continue; }
            pc = 0x621838; continue;
            }
            0x621838 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0xac8),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62188b; continue; }
            pc = 0x621850; continue;
            }
            0x621850 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = rm(m,c.r[1].wrapping_add(0x1c),32); c.sub(a,0x1790u32,32); }
            if c.cond(15) { pc = 0x621871; continue; }
            pc = 0x62185f; continue;
            }
            0x62185f => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            pc = 0x6247a8; continue;
            }
            0x621871 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.sub(a,0x1790u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            pc = 0x6218c3; continue;
            }
            0x62188b => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = rm(m,c.r[0].wrapping_add(0x1c),32); c.sub(a,0x179u32,32); }
            if c.cond(15) { pc = 0x6218ac; continue; }
            pc = 0x62189a; continue;
            }
            0x62189a => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            pc = 0x6247a8; continue;
            }
            0x6218ac => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.sub(a,0x179u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            pc = 0x6218c3; continue;
            }
            0x6218c3 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x20); wm(m,a,16,0xffffu32); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.r[5].wrapping_add(0xffffffd7); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x26); wm(m,a,8,0x0u32); }
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x671),8));
            c.push(m,c.get(2,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x8),8));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x62190c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x6264c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0xffffffd7),8));
            { let a = c.r[1].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            pc = 0x621918; continue;
            }
            0x621918 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12a),8));
            { let a = c.get(1,32); let r = c.and(a,0x6u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x621932; continue; }
            pc = 0x62192d; continue;
            }
            0x62192d => {
            pc = 0x6247a8; continue;
            }
            0x621932 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x149),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62195f; continue; }
            pc = 0x621944; continue;
            }
            0x621944 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x149),8));
            { let a = c.get(2,8); let r = c.sub(a,0x1u32,8); c.set(2,8,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x149); wm(m,a,8,c.get(2,8)); }
            pc = 0x62195f; continue;
            }
            0x62195f => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(2,32); c.sub(a,0xau32,32); }
            if c.cond(12) { pc = 0x6219a4; continue; }
            pc = 0x62196e; continue;
            }
            0x62196e => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,0xau32,32); }
            if c.cond(12) { pc = 0x6219a4; continue; }
            pc = 0x62197d; continue;
            }
            0x62197d => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,0x10f6u32,32); }
            if c.cond(15) { pc = 0x6219a4; continue; }
            pc = 0x62198e; continue;
            }
            0x62198e => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(2,32); c.sub(a,0xef8u32,32); }
            if c.cond(14) { pc = 0x621aeb; continue; }
            pc = 0x6219a4; continue;
            }
            0x6219a4 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            { let a = c.get(1,32); c.sub(a,0xffu32,32); }
            if c.cond(5) { pc = 0x6219e2; continue; }
            pc = 0x6219ba; continue;
            }
            0x6219ba => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1e98),8));
            c.set(6,32,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x6219da);
            { let args = []; c.r[0] = host.call(m, 0x4f3680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x621ae6; continue; }
            pc = 0x6219e2; continue;
            }
            0x6219e2 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,8); let r = c.or(a,0x2u32,8); c.set(0,8,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x12a); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.r[0].wrapping_add(0xac4); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            { let a = c.r[2].wrapping_add(0xac6); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = rm(m,c.r[2].wrapping_add(0x134),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x621a68; continue; }
            pc = 0x621a40; continue;
            }
            0x621a40 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x134),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); let r = c.add(a,0x3f8u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x3f8),32));
            { let target = rm(m,c.r[0].wrapping_add(0x4),32); c.push(m, 0x621a68); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x621a68; continue;
            }
            0x621a68 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x134); wm(m,a,32,0x0u32); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x138),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xa74),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.push(m, 0x621aaf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x619560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xa78),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.push(m, 0x621ae6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x619560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x621ae6; continue;
            }
            0x621ae6 => {
            pc = 0x6247a8; continue;
            }
            0x621aeb => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x130),8));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x621b18; continue; }
            pc = 0x621afd; continue;
            }
            0x621afd => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x130),8));
            { let a = c.get(1,8); let r = c.sub(a,0x1u32,8); c.set(1,8,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x130); wm(m,a,8,c.get(1,8)); }
            pc = 0x621b18; continue;
            }
            0x621b18 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x131),8));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x621b44; continue; }
            pc = 0x621b2a; continue;
            }
            0x621b2a => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x131),8));
            { let a = c.get(0,8); let r = c.sub(a,0x1u32,8); c.set(0,8,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x131); wm(m,a,8,c.get(0,8)); }
            pc = 0x621b44; continue;
            }
            0x621b44 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x161),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x621b71; continue; }
            pc = 0x621b56; continue;
            }
            0x621b56 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x161),8));
            { let a = c.get(2,8); let r = c.sub(a,0x1u32,8); c.set(2,8,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x161); wm(m,a,8,c.get(2,8)); }
            pc = 0x621b71; continue;
            }
            0x621b71 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0xac8),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x621c4a; continue; }
            pc = 0x621b8d; continue;
            }
            0x621b8d => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x130),8));
            { let a = c.get(2,32); c.sub(a,0x10u32,32); }
            if c.cond(13) { pc = 0x621baf; continue; }
            pc = 0x621ba0; continue;
            }
            0x621ba0 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x130); wm(m,a,8,0x0u32); }
            pc = 0x621bcc; continue;
            }
            0x621baf => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x130),8));
            { let a = c.get(2,32); let r = c.sub(a,0xfu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x130); wm(m,a,8,c.get(2,8)); }
            pc = 0x621bcc; continue;
            }
            0x621bcc => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x131),8));
            { let a = c.get(2,32); c.sub(a,0x10u32,32); }
            if c.cond(13) { pc = 0x621bee; continue; }
            pc = 0x621bdf; continue;
            }
            0x621bdf => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x131); wm(m,a,8,0x0u32); }
            pc = 0x621c0b; continue;
            }
            0x621bee => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x131),8));
            { let a = c.get(2,32); let r = c.sub(a,0xfu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x131); wm(m,a,8,c.get(2,8)); }
            pc = 0x621c0b; continue;
            }
            0x621c0b => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x161),8));
            { let a = c.get(2,32); c.sub(a,0x10u32,32); }
            if c.cond(13) { pc = 0x621c2d; continue; }
            pc = 0x621c1e; continue;
            }
            0x621c1e => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x161); wm(m,a,8,0x0u32); }
            pc = 0x621c4a; continue;
            }
            0x621c2d => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x161),8));
            { let a = c.get(2,32); let r = c.sub(a,0xfu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x161); wm(m,a,8,c.get(2,8)); }
            pc = 0x621c4a; continue;
            }
            0x621c4a => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x12a),8));
            { let a = c.get(2,32); let r = c.and(a,0x8u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x622125; continue; }
            pc = 0x621c63; continue;
            }
            0x621c63 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x128),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0x6u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x621cc6; continue; }
            pc = 0x621c93; continue;
            }
            0x621c93 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x128); wm(m,a,16,0xffffu32); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0xf7u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x12a); wm(m,a,8,c.get(0,8)); }
            pc = 0x622125; continue;
            }
            0x621cc6 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x621cf3; continue; }
            pc = 0x621ce8; continue;
            }
            0x621ce8 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.push(m, 0x621cf3);
            { let args = []; c.r[0] = host.call(m, 0x61f1bf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x621cf3; continue;
            }
            0x621cf3 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x128),16));
            { let a = c.get(1,32); c.sub(a,0xffffu32,32); }
            if c.cond(4) { pc = 0x621ea3; continue; }
            pc = 0x621d0e; continue;
            }
            0x621d0e => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x26),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x621d51; continue; }
            pc = 0x621d1d; continue;
            }
            0x621d1d => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x149),8));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x621d51; continue; }
            pc = 0x621d2f; continue;
            }
            0x621d2f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.get(1,8); let r = c.sub(a,0x1u32,8); c.set(1,8,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x26); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x149); wm(m,a,8,0x4bu32); }
            pc = 0x621d51; continue;
            }
            0x621d51 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x128),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd2); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x11),8));
            c.set(3,32,c.get(1,32));
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffffd2),16),16)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffd4),16),16)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x621de0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6029b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffd6); wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd6),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x621e05; continue; }
            pc = 0x621df7; continue;
            }
            0x621df7 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffd6),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x48u32,32); }
            if c.cond(12) { pc = 0x621e25; continue; }
            pc = 0x621e05; continue;
            }
            0x621e05 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffd6),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(14) { pc = 0x621e3e; continue; }
            pc = 0x621e14; continue;
            }
            0x621e14 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd6),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0xb8u32,32); }
            if c.cond(13) { pc = 0x621e3e; continue; }
            pc = 0x621e25; continue;
            }
            0x621e25 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x11),8));
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x11); wm(m,a,8,c.get(0,8)); }
            pc = 0x621e83; continue;
            }
            0x621e3e => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffd6),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x48u32,32); }
            if c.cond(14) { pc = 0x621e5b; continue; }
            pc = 0x621e4c; continue;
            }
            0x621e4c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffd6),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(12) { pc = 0x621e6c; continue; }
            pc = 0x621e5b; continue;
            }
            0x621e5b => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd6),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0xb8u32,32); }
            if c.cond(14) { pc = 0x621e83; continue; }
            pc = 0x621e6c; continue;
            }
            0x621e6c => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x11),8));
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x11); wm(m,a,8,c.get(0,8)); }
            pc = 0x621e83; continue;
            }
            0x621e83 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            pc = 0x621ea3; continue;
            }
            0x621ea3 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,16,0xffffu32); }
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x671),8));
            c.push(m,c.get(1,32));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x8),8));
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x621ed6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x6264c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12a),8));
            { let a = c.get(1,32); let r = c.and(a,0x6u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x621fb6; continue; }
            pc = 0x621eef; continue;
            }
            0x621eef => {
            c.push(m,0xffffff9cu32);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            { let a = c.get(0,32); let r = c.imul(a,0xffffff9cu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0xffffff9cu32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x138),16));
            { let a = c.get(2,32); let r = c.imul(a,0xffffff9cu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xa74),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.push(m, 0x621f50);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0xffffff9cu32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x138),16));
            { let a = c.get(2,32); let r = c.imul(a,0xffffff9cu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0xffffff9cu32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            { let a = c.get(1,32); let r = c.imul(a,0xffffff9cu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x138),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xa78),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.push(m, 0x621fb1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x6220e7; continue;
            }
            0x621fb6 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x621fca);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xe),8));
            c.set(0,16,rm(m,c.r[2].wrapping_mul(2).wrapping_add(0x67b0e4),16));
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffd0),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffd0),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffd0),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffd0),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xa74),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.push(m, 0x622056);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[5].wrapping_add(0xffffffd0),16),16)&0xffffffffu32);
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x168));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffffd0),16),16)&0xffffffffu32);
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x168));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffd0),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x168u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffd0),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x168u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xa78),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.push(m, 0x6220e7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x6220e7; continue;
            }
            0x6220e7 => {
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,16,0x0u32); }
            pc = 0x6220fb; continue;
            }
            0x6220ef => {
            c.set(2,16,rm(m,c.r[5].wrapping_add(0xffffffde),16));
            { let a = c.get(2,16); let r = c.add(a,0x1u32,16); c.set(2,16,r); }
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,16,c.get(2,16)); }
            pc = 0x6220fb; continue;
            }
            0x6220fb => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffde),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x100u32,32); }
            if c.cond(13) { pc = 0x622120; continue; }
            pc = 0x62210a; continue;
            }
            0x62210a => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffde),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(c.r[1]).wrapping_add(0x27); wm(m,a,8,0x0u32); }
            pc = 0x6220ef; continue;
            }
            0x622120 => {
            pc = 0x6247a8; continue;
            }
            0x622125 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x13d),8));
            c.set(1,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x559),8));
            { let a = c.r[5].wrapping_add(0xfffffffb); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x622706; continue; }
            pc = 0x622162; continue;
            }
            0x622162 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x5c),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x622706; continue; }
            pc = 0x62217b; continue;
            }
            0x62217b => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x5d),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x622706; continue; }
            pc = 0x6221a3; continue;
            }
            0x6221a3 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            { let a = c.get(2,32); c.sub(a,0xffffu32,32); }
            if c.cond(5) { pc = 0x622706; continue; }
            pc = 0x6221be; continue;
            }
            0x6221be => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x6223ff; continue; }
            pc = 0x6221d0; continue;
            }
            0x6221d0 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x11),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x6221fa; continue; }
            pc = 0x6221e9; continue;
            }
            0x6221e9 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,16,0xffffu32); }
            pc = 0x6223ff; continue;
            }
            0x6221fa => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(2,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x622260; continue; }
            pc = 0x62220c; continue;
            }
            0x62220c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x622234; continue; }
            pc = 0x62221b; continue;
            }
            0x62221b => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0x24),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x67b0f0),32));
            { let a = c.r[5].wrapping_add(0xffffff68); wm(m,a,32,c.get(1,32)); }
            pc = 0x622252; continue;
            }
            0x622234 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0x24),16),16)&0xffffffffu32);
            c.set(1,32,0x80u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x67b0f0),32));
            { let a = c.r[5].wrapping_add(0xffffff68); wm(m,a,32,c.get(2,32)); }
            pc = 0x622252; continue;
            }
            0x622252 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff68),32));
            { let a = c.r[5].wrapping_add(0xffffff64); wm(m,a,32,c.get(0,32)); }
            pc = 0x6222c0; continue;
            }
            0x622260 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,0x80u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x62228e; continue; }
            pc = 0x622275; continue;
            }
            0x622275 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x24),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x67aef0),32));
            { let a = c.r[5].wrapping_add(0xffffff60); wm(m,a,32,c.get(2,32)); }
            pc = 0x6222b2; continue;
            }
            0x62228e => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,0x80u32,32); c.set(1,32,r); }
            c.set(2,32,0x80u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x67b0f0),32));
            { let a = c.r[5].wrapping_add(0xffffff60); wm(m,a,32,c.get(0,32)); }
            pc = 0x6222b2; continue;
            }
            0x6222b2 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff60),32));
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffff64); wm(m,a,32,c.get(1,32)); }
            pc = 0x6222c0; continue;
            }
            0x6222c0 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff64),32));
            { let a = c.get(1,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x20); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,0x40u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x622368; continue; }
            pc = 0x6222fa; continue;
            }
            0x6222fa => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x622333; continue; }
            pc = 0x622311; continue;
            }
            0x622311 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,0x40u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x67b0f0),32));
            { let a = c.r[5].wrapping_add(0xffffff5c); wm(m,a,32,c.get(0,32)); }
            pc = 0x62235a; continue;
            }
            0x622333 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,0x40u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,0x80u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x67b0f0),32));
            { let a = c.r[5].wrapping_add(0xffffff5c); wm(m,a,32,c.get(1,32)); }
            pc = 0x62235a; continue;
            }
            0x62235a => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff5c),32));
            { let a = c.r[5].wrapping_add(0xffffff58); wm(m,a,32,c.get(2,32)); }
            pc = 0x6223e0; continue;
            }
            0x622368 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,0x40u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x80u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x40u32,32); }
            if c.cond(15) { pc = 0x6223a7; continue; }
            pc = 0x622386; continue;
            }
            0x622386 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x67aef0),32));
            { let a = c.r[5].wrapping_add(0xffffff54); wm(m,a,32,c.get(1,32)); }
            pc = 0x6223d2; continue;
            }
            0x6223a7 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0x24),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x80u32,32); c.set(0,32,r); }
            c.set(1,32,0x80u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x67b0f0),32));
            { let a = c.r[5].wrapping_add(0xffffff54); wm(m,a,32,c.get(2,32)); }
            pc = 0x6223d2; continue;
            }
            0x6223d2 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff54),32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffff58); wm(m,a,32,c.get(0,32)); }
            pc = 0x6223e0; continue;
            }
            0x6223e0 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff58),32));
            { let a = c.get(0,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x22); wm(m,a,16,c.get(2,16)); }
            pc = 0x6223ff; continue;
            }
            0x6223ff => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x149),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x622490; continue; }
            pc = 0x622411; continue;
            }
            0x622411 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x148),8));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x622451; continue; }
            pc = 0x62242e; continue;
            }
            0x62242e => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x26),8));
            { let a = c.get(0,8); let r = c.sub(a,0x1u32,8); c.set(0,8,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x26); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x149); wm(m,a,8,0x4bu32); }
            pc = 0x622490; continue;
            }
            0x622451 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x148),8));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x26),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x622490; continue; }
            pc = 0x62246e; continue;
            }
            0x62246e => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x26),8));
            { let a = c.get(2,8); let r = c.add(a,0x1u32,8); c.set(2,8,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x149); wm(m,a,8,0x4bu32); }
            pc = 0x622490; continue;
            }
            0x622490 => {
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x671),8));
            c.push(m,c.get(1,32));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x8),8));
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x6224b7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x6264c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12a),8));
            { let a = c.get(1,32); let r = c.and(a,0x6u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x622597; continue; }
            pc = 0x6224d0; continue;
            }
            0x6224d0 => {
            c.push(m,0xffffff9cu32);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            { let a = c.get(0,32); let r = c.imul(a,0xffffff9cu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0xffffff9cu32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x138),16));
            { let a = c.get(2,32); let r = c.imul(a,0xffffff9cu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xa74),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.push(m, 0x622531);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0xffffff9cu32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x138),16));
            { let a = c.get(2,32); let r = c.imul(a,0xffffff9cu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0xffffff9cu32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            { let a = c.get(1,32); let r = c.imul(a,0xffffff9cu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x138),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xa78),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.push(m, 0x622592);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x6226c8; continue;
            }
            0x622597 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x6225ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xe),8));
            c.set(0,16,rm(m,c.r[2].wrapping_mul(2).wrapping_add(0x67b0e4),16));
            { let a = c.r[5].wrapping_add(0xffffffce); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffce),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffce),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffce),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffce),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xa74),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.push(m, 0x622637);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[5].wrapping_add(0xffffffce),16),16)&0xffffffffu32);
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x168));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffffce),16),16)&0xffffffffu32);
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x168));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffce),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x168u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffce),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x168u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xa78),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.push(m, 0x6226c8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x6226c8; continue;
            }
            0x6226c8 => {
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,16,0x0u32); }
            pc = 0x6226dc; continue;
            }
            0x6226d0 => {
            c.set(2,16,rm(m,c.r[5].wrapping_add(0xffffffde),16));
            { let a = c.get(2,16); let r = c.add(a,0x1u32,16); c.set(2,16,r); }
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,16,c.get(2,16)); }
            pc = 0x6226dc; continue;
            }
            0x6226dc => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffde),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x100u32,32); }
            if c.cond(13) { pc = 0x622701; continue; }
            pc = 0x6226eb; continue;
            }
            0x6226eb => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffde),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(c.r[1]).wrapping_add(0x27); wm(m,a,8,0x0u32); }
            pc = 0x6226d0; continue;
            }
            0x622701 => {
            pc = 0x6247a8; continue;
            }
            0x622706 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x161),8));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x622c32; continue; }
            pc = 0x62271c; continue;
            }
            0x62271c => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x5d),8));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x622c32; continue; }
            pc = 0x622744; continue;
            }
            0x622744 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffffb),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x622b09; continue; }
            pc = 0x622755; continue;
            }
            0x622755 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffffb),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,0x1u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xaa0),32));
            { let a = c.get(1,32); let r = c.imul(a,0x8cu32,32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffc8); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffc8),32));
            { let a = c.get(6,32); let r = c.imul(a,0x3u32,32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x62279b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x6227c8; continue; }
            pc = 0x6227a4; continue;
            }
            0x6227a4 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x6227b8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x3u32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[5].wrapping_add(0xffffffc8); wm(m,a,32,c.get(0,32)); }
            pc = 0x6227c8; continue;
            }
            0x6227c8 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffc8),32));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x6227e2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),32),32); }
            if c.cond(3) { pc = 0x622804; continue; }
            pc = 0x6227e7; continue;
            }
            0x6227e7 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x6227fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x14),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffc8); wm(m,a,32,c.get(0,32)); }
            pc = 0x622804; continue;
            }
            0x622804 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x622818);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x144),32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x18),32),32); }
            if c.cond(5) { pc = 0x622866; continue; }
            pc = 0x622829; continue;
            }
            0x622829 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xac9),8));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x622866; continue; }
            pc = 0x622841; continue;
            }
            0x622841 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12a),8));
            { let a = c.get(1,32); let r = c.and(a,0xfeu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x12a); wm(m,a,8,c.get(1,8)); }
            pc = 0x622b04; continue;
            }
            0x622866 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0xab0),8));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xa90),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[2].wrapping_add(0xa94),32),32); }
            if c.cond(3) { pc = 0x62291e; continue; }
            pc = 0x6228a3; continue;
            }
            0x6228a3 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xa9c),16));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xab0),8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xa9a),16));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x62291e; continue; }
            pc = 0x6228ea; continue;
            }
            0x6228ea => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,8); let r = c.or(a,0x1u32,8); c.set(0,8,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x12a); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.r[0].wrapping_add(0xac9); wm(m,a,8,0x1u32); }
            pc = 0x622b04; continue;
            }
            0x62291e => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xab0),8));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0xa90),32));
            { let a = c.get(1,32); let r = c.imul(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xa94),32),32); }
            if c.cond(3) { pc = 0x622994; continue; }
            pc = 0x62295f; continue;
            }
            0x62295f => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x12a),8));
            { let a = c.get(2,32); let r = c.or(a,0x1u32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x12a); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.r[2].wrapping_add(0xac9); wm(m,a,8,0x1u32); }
            pc = 0x622b04; continue;
            }
            0x622994 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xa9c),16));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xab0),8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,0x3u32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xa9a),16));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x622a10; continue; }
            pc = 0x6229dc; continue;
            }
            0x6229dc => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,8); let r = c.or(a,0x1u32,8); c.set(0,8,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x12a); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.r[0].wrapping_add(0xac9); wm(m,a,8,0x1u32); }
            pc = 0x622b04; continue;
            }
            0x622a10 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x622a24);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(0xffffffc8),32),32); }
            if c.cond(3) { pc = 0x622a61; continue; }
            pc = 0x622a2c; continue;
            }
            0x622a2c => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x12a),8));
            { let a = c.get(2,32); let r = c.or(a,0x1u32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x12a); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.r[2].wrapping_add(0xac9); wm(m,a,8,0x1u32); }
            pc = 0x622b04; continue;
            }
            0x622a61 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x622a75);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x120),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x622ae5; continue; }
            pc = 0x622a7e; continue;
            }
            0x622a7e => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x622a92);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x40),16));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0xa9a),16));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x622ae5; continue; }
            pc = 0x622ab3; continue;
            }
            0x622ab3 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12a),8));
            { let a = c.get(1,32); let r = c.or(a,0x1u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x12a); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            { let a = c.r[1].wrapping_add(0xac9); wm(m,a,8,0x1u32); }
            pc = 0x622b04; continue;
            }
            0x622ae5 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0xfeu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x12a); wm(m,a,8,c.get(0,8)); }
            pc = 0x622b04; continue;
            }
            0x622b04 => {
            pc = 0x622c32; continue;
            }
            0x622b09 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.push(m, 0x622b14);
            { let args = []; c.r[0] = host.call(m, 0x62125b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0xa94),32),32); }
            if c.cond(3) { pc = 0x622c12; continue; }
            pc = 0x622b2c; continue;
            }
            0x622b2c => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.push(m, 0x622b37);
            { let args = []; c.r[0] = host.call(m, 0x62125b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0xa90),32),32); }
            if c.cond(3) { pc = 0x622c12; continue; }
            pc = 0x622b4f; continue;
            }
            0x622b4f => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x559),8));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xab4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x622b97);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0xc8u32,32); }
            if c.cond(15) { pc = 0x622bf3; continue; }
            pc = 0x622ba1; continue;
            }
            0x622ba1 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[3]).wrapping_add(0x559),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xabc),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x622be9);
            { let args = []; c.r[0] = host.call(m, 0x639f78, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0xc8u32,32); }
            if c.cond(14) { pc = 0x622c12; continue; }
            pc = 0x622bf3; continue;
            }
            0x622bf3 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12a),8));
            { let a = c.get(1,32); let r = c.or(a,0x1u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x12a); wm(m,a,8,c.get(1,8)); }
            pc = 0x622c32; continue;
            }
            0x622c12 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12a),8));
            { let a = c.get(1,32); let r = c.and(a,0xfeu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x12a); wm(m,a,8,c.get(1,8)); }
            pc = 0x622c32; continue;
            }
            0x622c32 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffb),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x622ccb; continue; }
            pc = 0x622c43; continue;
            }
            0x622c43 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x12a),8));
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x622ccb; continue; }
            pc = 0x622c58; continue;
            }
            0x622c58 => {
            { let a = c.r[5].wrapping_add(0xffffffc7); wm(m,a,8,0x0u32); }
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,16,0x0u32); }
            pc = 0x622c70; continue;
            }
            0x622c64 => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xffffffde),16));
            { let a = c.get(0,16); let r = c.add(a,0x1u32,16); c.set(0,16,r); }
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,16,c.get(0,16)); }
            pc = 0x622c70; continue;
            }
            0x622c70 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffde),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x622c9f; continue; }
            pc = 0x622c7e; continue;
            }
            0x622c7e => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = rm(m,c.r[0].wrapping_add(0x1ec4),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x622c9d; continue; }
            pc = 0x622c93; continue;
            }
            0x622c93 => {
            { let a = c.r[5].wrapping_add(0xffffffc7); wm(m,a,8,0x1u32); }
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,16,0x3u32); }
            pc = 0x622c9d; continue;
            }
            0x622c9d => {
            pc = 0x622c64; continue;
            }
            0x622c9f => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffc7),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x622ccb; continue; }
            pc = 0x622cac; continue;
            }
            0x622cac => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0xfeu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x12a); wm(m,a,8,c.get(0,8)); }
            pc = 0x622ccb; continue;
            }
            0x622ccb => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62306f; continue; }
            pc = 0x622ce4; continue;
            }
            0x622ce4 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.push(m, 0x622cef);
            { let args = []; c.r[0] = host.call(m, 0x625bf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffb),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x622e22; continue; }
            pc = 0x622d00; continue;
            }
            0x622d00 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,0x190u32,32); }
            if c.cond(12) { pc = 0x622d46; continue; }
            pc = 0x622d11; continue;
            }
            0x622d11 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(2,32); c.sub(a,0x2070u32,32); }
            if c.cond(15) { pc = 0x622d46; continue; }
            pc = 0x622d23; continue;
            }
            0x622d23 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,0x190u32,32); }
            if c.cond(12) { pc = 0x622d46; continue; }
            pc = 0x622d35; continue;
            }
            0x622d35 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,0x1c74u32,32); }
            if c.cond(14) { pc = 0x622d71; continue; }
            pc = 0x622d46; continue;
            }
            0x622d46 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,0x880u32,32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,0x781u32,32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,c.get(1,32)); }
            pc = 0x622e1d; continue;
            }
            0x622d71 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x13d),8));
            { let a = c.r[5].wrapping_add(0xffffff50); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[3]).wrapping_add(0x559),8));
            c.set(1,32,0x1u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff50),32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xab4),32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x13d),8));
            { let a = c.r[5].wrapping_add(0xffffff4c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x559),8));
            c.set(0,32,0x1u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff4c),32));
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xabc),32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,c.get(2,32)); }
            pc = 0x622e1d; continue;
            }
            0x622e1d => {
            pc = 0x622eac; continue;
            }
            0x622e22 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x559),8));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xab4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[3]).wrapping_add(0x559),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xabc),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,c.get(2,32)); }
            pc = 0x622eac; continue;
            }
            0x622eac => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffc0),32); c.sub(a,0x0u32,32); }
            if c.cond(13) { pc = 0x622edc; continue; }
            pc = 0x622eb2; continue;
            }
            0x622eb2 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffbc),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[5].wrapping_add(0xffffffc0),32); c.idiv32(b); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,32,c.get(0,32)); }
            pc = 0x622f41; continue;
            }
            0x622edc => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffc0),32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x622f16; continue; }
            pc = 0x622ee2; continue;
            }
            0x622ee2 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,0x2200u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffbc),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[5].wrapping_add(0xffffffc0),32); c.idiv32(b); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(1,32,0x2200u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,32,c.get(1,32)); }
            pc = 0x622f41; continue;
            }
            0x622f16 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffbc),32); c.sub(a,0x0u32,32); }
            if c.cond(13) { pc = 0x622f2d; continue; }
            pc = 0x622f1c; continue;
            }
            0x622f1c => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,c.get(0,32)); }
            pc = 0x622f41; continue;
            }
            0x622f2d => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,0x1e04u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,c.get(0,32)); }
            pc = 0x622f41; continue;
            }
            0x622f41 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffffbc),32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x622f7c; continue; }
            pc = 0x622f52; continue;
            }
            0x622f52 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffc0),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[5].wrapping_add(0xffffffbc),32); c.idiv32(b); }
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,c.get(0,32)); }
            pc = 0x622fc4; continue;
            }
            0x622f7c => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,0x1e04u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffffbc),32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x622fc4; continue; }
            pc = 0x622f92; continue;
            }
            0x622f92 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,0x1e04u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffc0),32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[5].wrapping_add(0xffffffbc),32); c.idiv32(b); }
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,0x1e04u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,c.get(1,32)); }
            pc = 0x622fc4; continue;
            }
            0x622fc4 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0xffffffc0),32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x20); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0xffffffbc),32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x22); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x26),8));
            { let a = c.r[5].wrapping_add(0xffffffbb); wm(m,a,8,c.get(0,8)); }
            c.push(m,0x1u32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x671),8));
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x623025);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x6264c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffbb),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x62306a; continue; }
            pc = 0x62303d; continue;
            }
            0x62303d => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x149),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62305d; continue; }
            pc = 0x62304f; continue;
            }
            0x62304f => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0xffffffbb),8));
            { let a = c.r[1].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            pc = 0x62306a; continue;
            }
            0x62305d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x149); wm(m,a,8,0x4bu32); }
            pc = 0x62306a; continue;
            }
            0x62306a => {
            pc = 0x624516; continue;
            }
            0x62306f => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            { let a = c.get(2,32); c.sub(a,0xffffu32,32); }
            if c.cond(5) { pc = 0x6232db; continue; }
            pc = 0x62308a; continue;
            }
            0x62308a => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,16,0xffffu32); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x5d),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6230d6; continue; }
            pc = 0x6230ba; continue;
            }
            0x6230ba => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x6230cb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.push(m, 0x6230d6);
            { let args = []; c.r[0] = host.call(m, 0x625bf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x6230d6; continue;
            }
            0x6230d6 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.r[5].wrapping_add(0xffffff97); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x5d),8));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x671),8));
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x623126);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x6264c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff97),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x62316b; continue; }
            pc = 0x62313e; continue;
            }
            0x62313e => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x149),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x62315e; continue; }
            pc = 0x623150; continue;
            }
            0x623150 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0xffffff97),8));
            { let a = c.r[1].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            pc = 0x62316b; continue;
            }
            0x62315e => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x149); wm(m,a,8,0x4bu32); }
            pc = 0x62316b; continue;
            }
            0x62316b => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x62317f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            c.set(2,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x67b0e4),16));
            { let a = c.r[5].wrapping_add(0xffffff98); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff98),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff98),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff98),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff98),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xa74),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.push(m, 0x62320b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffff98),16),16)&0xffffffffu32);
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x168));
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[5].wrapping_add(0xffffff98),16),16)&0xffffffffu32);
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x168));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff98),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x168u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff98),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x168u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xa78),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.push(m, 0x62329c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,16,0x0u32); }
            pc = 0x6232b0; continue;
            }
            0x6232a4 => {
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xffffffde),16));
            { let a = c.get(1,16); let r = c.add(a,0x1u32,16); c.set(1,16,r); }
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,16,c.get(1,16)); }
            pc = 0x6232b0; continue;
            }
            0x6232b0 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffde),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x100u32,32); }
            if c.cond(13) { pc = 0x6232d6; continue; }
            pc = 0x6232c1; continue;
            }
            0x6232c1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffde),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0x27); wm(m,a,8,0x0u32); }
            pc = 0x6232a4; continue;
            }
            0x6232d6 => {
            pc = 0x6247a8; continue;
            }
            0x6232db => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.push(m, 0x6232e6);
            { let args = []; c.r[0] = host.call(m, 0x625bf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0xffffff96); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x128),16));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x14),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x62345b; continue; }
            pc = 0x623313; continue;
            }
            0x623313 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.r[5].wrapping_add(0xffffff8c); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff8c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0x6u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x623352; continue; }
            pc = 0x623349; continue;
            }
            0x623349 => {
            { let a = c.r[5].wrapping_add(0xffffff96); wm(m,a,8,0x1u32); }
            pc = 0x623459; continue;
            }
            0x623352 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x5d),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x623459; continue; }
            pc = 0x62337a; continue;
            }
            0x62337a => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff8c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x559),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x559),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x6233ca; continue; }
            pc = 0x6233c1; continue;
            }
            0x6233c1 => {
            { let a = c.r[5].wrapping_add(0xffffff96); wm(m,a,8,0x1u32); }
            pc = 0x623459; continue;
            }
            0x6233ca => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff8c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x128),16));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x14),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x623459; continue; }
            pc = 0x6233ec; continue;
            }
            0x6233ec => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff8c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.r[5].wrapping_add(0xffffff90); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff8c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x623459; continue; }
            pc = 0x62341f; continue;
            }
            0x62341f => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff8c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x623430);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x40),16));
            c.set(6,32,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff90),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x623449);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x40),16));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x623459; continue; }
            pc = 0x623455; continue;
            }
            0x623455 => {
            { let a = c.r[5].wrapping_add(0xffffff96); wm(m,a,8,0x1u32); }
            pc = 0x623459; continue;
            }
            0x623459 => {
            pc = 0x62348b; continue;
            }
            0x62345b => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(1,32,0xfffeu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1ec4),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x62348b; continue; }
            pc = 0x623487; continue;
            }
            0x623487 => {
            { let a = c.r[5].wrapping_add(0xffffff96); wm(m,a,8,0x1u32); }
            pc = 0x62348b; continue;
            }
            0x62348b => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff96),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x62374e; continue; }
            pc = 0x62349c; continue;
            }
            0x62349c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,16,0xffffu32); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x5d),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6234df; continue; }
            pc = 0x6234cc; continue;
            }
            0x6234cc => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let target = rm(m,c.r[2].wrapping_add(0x4),32); c.push(m, 0x6234dd); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x623518; continue;
            }
            0x6234df => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x128); wm(m,a,16,0xffffu32); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x26),8));
            { let a = c.r[1].wrapping_add(0x148); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x11),8));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x24); wm(m,a,16,c.get(2,16)); }
            pc = 0x623518; continue;
            }
            0x623518 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x26),8));
            { let a = c.r[5].wrapping_add(0xffffff89); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x13d),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x5d),8));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x623566; continue; }
            pc = 0x623548; continue;
            }
            0x623548 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x5c),8));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x623566; continue; }
            pc = 0x62355d; continue;
            }
            0x62355d => {
            { let a = c.r[5].wrapping_add(0xffffff4b); wm(m,a,8,0x0u32); }
            pc = 0x62356d; continue;
            }
            0x623566 => {
            { let a = c.r[5].wrapping_add(0xffffff4b); wm(m,a,8,0x1u32); }
            pc = 0x62356d; continue;
            }
            0x62356d => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xffffff4b),8));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x671),8));
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x623599);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x6264c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff89),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x6235de; continue; }
            pc = 0x6235b1; continue;
            }
            0x6235b1 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x149),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6235d1; continue; }
            pc = 0x6235c3; continue;
            }
            0x6235c3 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0xffffff89),8));
            { let a = c.r[1].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            pc = 0x6235de; continue;
            }
            0x6235d1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x149); wm(m,a,8,0x4bu32); }
            pc = 0x6235de; continue;
            }
            0x6235de => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x6235f2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            c.set(2,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x67b0e4),16));
            { let a = c.r[5].wrapping_add(0xffffff8a); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff8a),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff8a),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff8a),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff8a),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xa74),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.push(m, 0x62367e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffff8a),16),16)&0xffffffffu32);
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x168));
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[5].wrapping_add(0xffffff8a),16),16)&0xffffffffu32);
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x168));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff8a),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x168u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff8a),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x168u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xa78),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.push(m, 0x62370f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,16,0x0u32); }
            pc = 0x623723; continue;
            }
            0x623717 => {
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xffffffde),16));
            { let a = c.get(1,16); let r = c.add(a,0x1u32,16); c.set(1,16,r); }
            { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,16,c.get(1,16)); }
            pc = 0x623723; continue;
            }
            0x623723 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffde),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x100u32,32); }
            if c.cond(13) { pc = 0x623749; continue; }
            pc = 0x623734; continue;
            }
            0x623734 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffde),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0x27); wm(m,a,8,0x0u32); }
            pc = 0x623717; continue;
            }
            0x623749 => {
            pc = 0x6247a8; continue;
            }
            0x62374e => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x128),16));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x14),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x6237d3; continue; }
            pc = 0x623773; continue;
            }
            0x623773 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.r[2].wrapping_add(0x20); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x128),16));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.r[1].wrapping_add(0x22); wm(m,a,16,c.get(2,16)); }
            pc = 0x62383b; continue;
            }
            0x6237d3 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x128),16));
            c.set(2,32,0xfffeu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x1e9e),16));
            { let a = c.r[0].wrapping_add(0x20); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(1,32,0xfffeu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x1ea6),16));
            { let a = c.r[2].wrapping_add(0x22); wm(m,a,16,c.get(0,16)); }
            pc = 0x62383b; continue;
            }
            0x62383b => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0x20),16),16)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffa0),32));
            { let a = c.r[5].wrapping_add(0xffffff9c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffb0); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffb0),32));
            { let a = c.r[5].wrapping_add(0xffffffa8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x128),16));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x14),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x6239ea; continue; }
            pc = 0x6238a2; continue;
            }
            0x6238a2 => {
            { let a = c.r[5].wrapping_add(0xffffff88); wm(m,a,8,0x0u32); }
            pc = 0x6238b1; continue;
            }
            0x6238a8 => {
            c.set(2,8,rm(m,c.r[5].wrapping_add(0xffffff88),8));
            { let a = c.get(2,8); let r = c.add(a,0x1u32,8); c.set(2,8,r); }
            { let a = c.r[5].wrapping_add(0xffffff88); wm(m,a,8,c.get(2,8)); }
            pc = 0x6238b1; continue;
            }
            0x6238b1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff88),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x6239ea; continue; }
            pc = 0x6238c2; continue;
            }
            0x6238c2 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffa0),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0xffffffa0),32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffb0),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0xffffffb0),32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffa4); wm(m,a,32,c.get(1,32)); }
            c.fild32(m,c.r[5].wrapping_add(0xffffffa4));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.fstp64(m,c.r[4]);
            c.push(m, 0x6238e3);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x6238eb);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x530u32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[5].wrapping_add(0xffffffb4); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xac8),8));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x62397f; continue; }
            pc = 0x623911; continue;
            }
            0x623911 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x14),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0xffffffb4),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff9c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x128),16));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0xffffffb4),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffa8),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffb0); wm(m,a,32,c.get(2,32)); }
            pc = 0x6239e5; continue;
            }
            0x62397f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x128),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0xffffffb4),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff9c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffa0); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0xffffffb4),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffa8),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffb0); wm(m,a,32,c.get(0,32)); }
            pc = 0x6239e5; continue;
            }
            0x6239e5 => {
            pc = 0x6238a8; continue;
            }
            0x6239ea => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffa0),32));
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0xffffffa0),32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffb0),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0xffffffb0),32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffa4); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x128),16));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x14),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x623a68; continue; }
            pc = 0x623a22; continue;
            }
            0x623a22 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[2].wrapping_add(0x128),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x13d),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x559),8));
            { let a = c.r[5].wrapping_add(0xffffff44); wm(m,a,32,c.get(2,32)); }
            pc = 0x623a72; continue;
            }
            0x623a68 => {
            { let a = c.r[5].wrapping_add(0xffffff44); wm(m,a,32,0x1u32); }
            pc = 0x623a72; continue;
            }
            0x623a72 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xffffff44),8));
            { let a = c.r[5].wrapping_add(0xffffffaf); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            { let a = c.get(2,32); c.sub(a,0xffffu32,32); }
            if c.cond(4) { pc = 0x623e14; continue; }
            pc = 0x623a96; continue;
            }
            0x623a96 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffaf),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffb),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x623e14; continue; }
            pc = 0x623aaf; continue;
            }
            0x623aaf => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,0x190u32,32); }
            if c.cond(13) { pc = 0x623b85; continue; }
            pc = 0x623ac4; continue;
            }
            0x623ac4 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x14),16));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x623b27; continue; }
            pc = 0x623ae9; continue;
            }
            0x623ae9 => {
            c.push(m,0x0u32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x128),16));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.push(m, 0x623b18);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x61f02f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x623b27; continue; }
            pc = 0x623b21; continue;
            }
            0x623b21 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffa0),32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x623b85; continue; }
            pc = 0x623b27; continue;
            }
            0x623b27 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(0,32,0x190u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffff84); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffff84),32); c.sub(a,0xffu32,32); }
            if c.cond(14) { pc = 0x623b4b; continue; }
            pc = 0x623b44; continue;
            }
            0x623b44 => {
            { let a = c.r[5].wrapping_add(0xffffff84); wm(m,a,32,0xffu32); }
            pc = 0x623b4b; continue;
            }
            0x623b4b => {
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,0x0u32); }
            pc = 0x623b5a; continue;
            }
            0x623b51 => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0xffffffe2),8));
            { let a = c.get(1,8); let r = c.add(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,c.get(1,8)); }
            pc = 0x623b5a; continue;
            }
            0x623b5a => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x623b85; continue; }
            pc = 0x623b6b; continue;
            }
            0x623b6b => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0xffffff84),8));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xa7); wm(m,a,8,c.get(2,8)); }
            pc = 0x623b51; continue;
            }
            0x623b85 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,0x190u32,32); }
            if c.cond(13) { pc = 0x623c62; continue; }
            pc = 0x623b9b; continue;
            }
            0x623b9b => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x128),16));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x14),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x623bfe; continue; }
            pc = 0x623bc0; continue;
            }
            0x623bc0 => {
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.push(m, 0x623bef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x61f02f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x623bfe; continue; }
            pc = 0x623bf8; continue;
            }
            0x623bf8 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffb0),32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x623c62; continue; }
            pc = 0x623bfe; continue;
            }
            0x623bfe => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(1,32,0x190u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffff80); wm(m,a,32,c.get(1,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffff80),32); c.sub(a,0xffu32,32); }
            if c.cond(14) { pc = 0x623c22; continue; }
            pc = 0x623c1b; continue;
            }
            0x623c1b => {
            { let a = c.r[5].wrapping_add(0xffffff80); wm(m,a,32,0xffu32); }
            pc = 0x623c22; continue;
            }
            0x623c22 => {
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,0x0u32); }
            pc = 0x623c31; continue;
            }
            0x623c28 => {
            c.set(2,8,rm(m,c.r[5].wrapping_add(0xffffffe2),8));
            { let a = c.get(2,8); let r = c.add(a,0x1u32,8); c.set(2,8,r); }
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,c.get(2,8)); }
            pc = 0x623c31; continue;
            }
            0x623c31 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x623c62; continue; }
            pc = 0x623c40; continue;
            }
            0x623c40 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xc0u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(c.r[1]).wrapping_add(0x27); wm(m,a,8,0xffu32); }
            pc = 0x623c28; continue;
            }
            0x623c62 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,0xf70u32,32); }
            if c.cond(14) { pc = 0x623d3b; continue; }
            pc = 0x623c78; continue;
            }
            0x623c78 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x128),16));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x14),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x623cdb; continue; }
            pc = 0x623c9d; continue;
            }
            0x623c9d => {
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.push(m, 0x623ccc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x61f02f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x623cdb; continue; }
            pc = 0x623cd5; continue;
            }
            0x623cd5 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffa0),32); c.sub(a,0x0u32,32); }
            if c.cond(13) { pc = 0x623d3b; continue; }
            pc = 0x623cdb; continue;
            }
            0x623cdb => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,0x2070u32,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffff7c); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffff7c),32); c.sub(a,0xffu32,32); }
            if c.cond(14) { pc = 0x623d06; continue; }
            pc = 0x623cfc; continue;
            }
            0x623cfc => {
            { let a = c.r[5].wrapping_add(0xffffff7c); wm(m,a,32,0xffu32); }
            pc = 0x623d06; continue;
            }
            0x623d06 => {
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,0x0u32); }
            pc = 0x623d15; continue;
            }
            0x623d0c => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0xffffffe2),8));
            { let a = c.get(1,8); let r = c.add(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,c.get(1,8)); }
            pc = 0x623d15; continue;
            }
            0x623d15 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x623d3b; continue; }
            pc = 0x623d26; continue;
            }
            0x623d26 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0x27); wm(m,a,8,0xffu32); }
            pc = 0x623d0c; continue;
            }
            0x623d3b => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,0xd72u32,32); }
            if c.cond(14) { pc = 0x623e14; continue; }
            pc = 0x623d50; continue;
            }
            0x623d50 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x128),16));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x14),16));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x623db3; continue; }
            pc = 0x623d75; continue;
            }
            0x623d75 => {
            c.push(m,0x0u32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x128),16));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.push(m, 0x623da4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x61f02f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x623db3; continue; }
            pc = 0x623dad; continue;
            }
            0x623dad => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffb0),32); c.sub(a,0x0u32,32); }
            if c.cond(13) { pc = 0x623e14; continue; }
            pc = 0x623db3; continue;
            }
            0x623db3 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,0x1c74u32,32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffff78); wm(m,a,32,c.get(2,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffff78),32); c.sub(a,0xffu32,32); }
            if c.cond(14) { pc = 0x623ddf; continue; }
            pc = 0x623dd5; continue;
            }
            0x623dd5 => {
            { let a = c.r[5].wrapping_add(0xffffff78); wm(m,a,32,0xffu32); }
            pc = 0x623ddf; continue;
            }
            0x623ddf => {
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,0x0u32); }
            pc = 0x623ded; continue;
            }
            0x623de5 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xffffffe2),8));
            { let a = c.get(0,8); let r = c.add(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,c.get(0,8)); }
            pc = 0x623ded; continue;
            }
            0x623ded => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x80u32,32); }
            if c.cond(13) { pc = 0x623e14; continue; }
            pc = 0x623dfe; continue;
            }
            0x623dfe => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x67); wm(m,a,8,0xffu32); }
            pc = 0x623de5; continue;
            }
            0x623e14 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x13c),8));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x623e5b; continue; }
            pc = 0x623e26; continue;
            }
            0x623e26 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x128),16));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x14),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x623e5b; continue; }
            pc = 0x623e4b; continue;
            }
            0x623e4b => {
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,0x15f90u32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1fa40u32); }
            pc = 0x623e69; continue;
            }
            0x623e5b => {
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,0x31a88u32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x38400u32); }
            pc = 0x623e69; continue;
            }
            0x623e69 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffa4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(0xffffffd8),32),32); }
            if c.cond(3) { pc = 0x623ef9; continue; }
            pc = 0x623e75; continue;
            }
            0x623e75 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffb),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffaf),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x623edc; continue; }
            pc = 0x623e8b; continue;
            }
            0x623e8b => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x149),8));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x623ece; continue; }
            pc = 0x623e9d; continue;
            }
            0x623e9d => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x26),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x623ece; continue; }
            pc = 0x623eac; continue;
            }
            0x623eac => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x26),8));
            { let a = c.get(2,8); let r = c.sub(a,0x1u32,8); c.set(2,8,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x149); wm(m,a,8,0x4bu32); }
            pc = 0x623ece; continue;
            }
            0x623ece => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x20); wm(m,a,16,0xffffu32); }
            pc = 0x623ef9; continue;
            }
            0x623edc => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x12a),8));
            { let a = c.get(1,32); let r = c.or(a,0x10u32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x12a); wm(m,a,8,c.get(1,8)); }
            pc = 0x623ef9; continue;
            }
            0x623ef9 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x128),16));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x14),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x623f55; continue; }
            pc = 0x623f1e; continue;
            }
            0x623f1e => {
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x128),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.push(m,c.get(2,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.push(m, 0x623f4d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x61f02f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[5].wrapping_add(0xffffff43); wm(m,a,8,c.get(0,8)); }
            pc = 0x623f5c; continue;
            }
            0x623f55 => {
            { let a = c.r[5].wrapping_add(0xffffff43); wm(m,a,8,0x0u32); }
            pc = 0x623f5c; continue;
            }
            0x623f5c => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xffffff43),8));
            { let a = c.r[5].wrapping_add(0xffffff9b); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffa4),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffffc),32),32); }
            if c.cond(3) { pc = 0x6243f5; continue; }
            pc = 0x623f71; continue;
            }
            0x623f71 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0x10u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6243f5; continue; }
            pc = 0x623f8a; continue;
            }
            0x623f8a => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff9b),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x6243f5; continue; }
            pc = 0x623f9b; continue;
            }
            0x623f9b => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x20); wm(m,a,16,0xffffu32); }
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffffa0),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,sx(rm(m,c.r[5].wrapping_add(0xffffffb0),16),16)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x623fc1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6029b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[5].wrapping_add(0xffffff77); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x11),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(14) { pc = 0x624007; continue; }
            pc = 0x623fef; continue;
            }
            0x623fef => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x80u32,32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffff77); wm(m,a,8,c.get(2,8)); }
            pc = 0x624007; continue;
            }
            0x624007 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x27),8));
            { let a = c.get(2,32); c.sub(a,0x80u32,32); }
            if c.cond(14) { pc = 0x62407a; continue; }
            pc = 0x624026; continue;
            }
            0x624026 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x80u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x27),8));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x27),8));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x62407a; continue; }
            pc = 0x624062; continue;
            }
            0x624062 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x80u32,32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffff77); wm(m,a,8,c.get(2,8)); }
            pc = 0x62407a; continue;
            }
            0x62407a => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x27),8));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x62428d; continue; }
            pc = 0x624099; continue;
            }
            0x624099 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x11),8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x80u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6240db; continue; }
            pc = 0x6240c5; continue;
            }
            0x6240c5 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x80u32,32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,c.get(1,8)); }
            pc = 0x6240e9; continue;
            }
            0x6240db => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,c.get(2,8)); }
            pc = 0x6240e9; continue;
            }
            0x6240e9 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x11),8));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xffffff76); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,0x1u32); }
            { let a = c.r[5].wrapping_add(0xffffffe1); wm(m,a,8,0xffu32); }
            pc = 0x62410d; continue;
            }
            0x62410d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x27),8));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x624281; continue; }
            pc = 0x62413c; continue;
            }
            0x62413c => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x27),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x624281; continue; }
            pc = 0x62416c; continue;
            }
            0x62416c => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0xffffffe2),8));
            { let a = c.get(1,8); let r = c.add(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x27),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe1),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x6241ef; continue; }
            pc = 0x6241aa; continue;
            }
            0x6241aa => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x27),8));
            { let a = c.r[5].wrapping_add(0xffffffe1); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffff76); wm(m,a,8,c.get(0,8)); }
            pc = 0x6241ef; continue;
            }
            0x6241ef => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x27),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe1),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x624269; continue; }
            pc = 0x624224; continue;
            }
            0x624224 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x27),8));
            { let a = c.r[5].wrapping_add(0xffffffe1); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffff76); wm(m,a,8,c.get(0,8)); }
            pc = 0x624269; continue;
            }
            0x624269 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe2),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x80u32,32); }
            if c.cond(5) { pc = 0x62427c; continue; }
            pc = 0x62427a; continue;
            }
            0x62427a => {
            pc = 0x624281; continue;
            }
            0x62427c => {
            pc = 0x62410d; continue;
            }
            0x624281 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xffffff76),8));
            { let a = c.r[5].wrapping_add(0xffffff77); wm(m,a,8,c.get(0,8)); }
            pc = 0x62428d; continue;
            }
            0x62428d => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x11),8));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x1u32,32); }
            if c.cond(15) { pc = 0x6242c8; continue; }
            pc = 0x6242ab; continue;
            }
            0x6242ab => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x11),8));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff77),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x6242dd; continue; }
            pc = 0x6242c8; continue;
            }
            0x6242c8 => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0xffffff77),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x6242dd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6271d7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6242dd; continue;
            }
            0x6242dd => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x149),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x6243c9; continue; }
            pc = 0x6242f3; continue;
            }
            0x6242f3 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x11),8));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x27),8));
            { let a = c.get(1,32); c.sub(a,0x80u32,32); }
            if c.cond(14) { pc = 0x624397; continue; }
            pc = 0x624316; continue;
            }
            0x624316 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x11),8));
            { let a = c.get(0,32); let r = c.add(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x27),8));
            { let a = c.get(2,32); c.sub(a,0x80u32,32); }
            if c.cond(14) { pc = 0x624397; continue; }
            pc = 0x62433d; continue;
            }
            0x62433d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x11),8));
            { let a = c.get(1,32); let r = c.sub(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x27),8));
            { let a = c.get(0,32); c.sub(a,0x80u32,32); }
            if c.cond(14) { pc = 0x624397; continue; }
            pc = 0x624364; continue;
            }
            0x624364 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x26),8));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x624395; continue; }
            pc = 0x624373; continue;
            }
            0x624373 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.get(1,8); let r = c.sub(a,0x1u32,8); c.set(1,8,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x26); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x149); wm(m,a,8,0x4bu32); }
            pc = 0x624395; continue;
            }
            0x624395 => {
            pc = 0x6243c9; continue;
            }
            0x624397 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x26),8));
            { let a = c.get(2,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x6243c9; continue; }
            pc = 0x6243a7; continue;
            }
            0x6243a7 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.get(1,8); let r = c.add(a,0x1u32,8); c.set(1,8,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[2].wrapping_add(0x26); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x149); wm(m,a,8,0x4bu32); }
            pc = 0x6243c9; continue;
            }
            0x6243c9 => {
            c.push(m,0x0u32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x671),8));
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x6243f0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x6264c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x624516; continue;
            }
            0x6243f5 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0xefu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x12a); wm(m,a,8,c.get(0,8)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x149),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x624476; continue; }
            pc = 0x624426; continue;
            }
            0x624426 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x11),8));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x27),8));
            { let a = c.get(1,32); c.sub(a,0x80u32,32); }
            if c.cond(14) { pc = 0x624476; continue; }
            pc = 0x624445; continue;
            }
            0x624445 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x26),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x624476; continue; }
            pc = 0x624454; continue;
            }
            0x624454 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x26),8));
            { let a = c.get(2,8); let r = c.sub(a,0x1u32,8); c.set(2,8,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[0].wrapping_add(0x26); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x149); wm(m,a,8,0x4bu32); }
            pc = 0x624476; continue;
            }
            0x624476 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x26),8));
            { let a = c.r[5].wrapping_add(0xffffff75); wm(m,a,8,c.get(0,8)); }
            c.push(m,0x1u32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x671),8));
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x6244ac);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x6264c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff75),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x26),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x624516; continue; }
            pc = 0x6244c7; continue;
            }
            0x6244c7 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x149),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x6244f8; continue; }
            pc = 0x6244d9; continue;
            }
            0x6244d9 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x11),8));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x27),8));
            { let a = c.get(1,32); c.sub(a,0x80u32,32); }
            if c.cond(14) { pc = 0x624509; continue; }
            pc = 0x6244f8; continue;
            }
            0x6244f8 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xffffff75),8));
            { let a = c.r[2].wrapping_add(0x26); wm(m,a,8,c.get(0,8)); }
            pc = 0x624516; continue;
            }
            0x624509 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.r[1].wrapping_add(0x149); wm(m,a,8,0x4bu32); }
            pc = 0x624516; continue;
            }
            0x624516 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x12a),8));
            { let a = c.get(0,32); let r = c.and(a,0x6u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6245f6; continue; }
            pc = 0x62452f; continue;
            }
            0x62452f => {
            c.push(m,0xffffff9cu32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x138),16));
            { let a = c.get(2,32); let r = c.imul(a,0xffffff9cu32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,0xffffff9cu32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            { let a = c.get(1,32); let r = c.imul(a,0xffffff9cu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x138),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xa74),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)),16));
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.push(m, 0x624590);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.push(m,0xffffff9cu32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            { let a = c.get(1,32); let r = c.imul(a,0xffffff9cu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0xffffff9cu32);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            { let a = c.get(0,32); let r = c.imul(a,0xffffff9cu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x138),16));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xa78),32));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.push(m,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.push(m, 0x6245f1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x624742; continue;
            }
            0x6245f6 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x62460a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46cc00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xe),8));
            c.set(2,16,rm(m,c.r[1].wrapping_mul(2).wrapping_add(0x67b0e4),16));
            { let a = c.r[5].wrapping_add(0xffffff72); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff72),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff72),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff72),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff72),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xa74),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.push(m, 0x6246a5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0xffffff72),16),16)&0xffffffffu32);
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x168));
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(1,32,sx(rm(m,c.r[5].wrapping_add(0xffffff72),16),16)&0xffffffffu32);
            c.set(2,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x168));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff72),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x168u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xffffff72),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x168u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x138),16));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xa78),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            c.push(m,c.get(2,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.push(m, 0x624742);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6197a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x624742; continue;
            }
            0x624742 => {
            c.push(m,0x100u32);
            c.push(m,0x0u32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.add(a,0x27u32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x624758);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x128),16));
            { let a = c.get(0,32); c.sub(a,0xffffu32,32); }
            if c.cond(5) { pc = 0x624795; continue; }
            pc = 0x624771; continue;
            }
            0x624771 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x13d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x5d),8));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6247a8; continue; }
            pc = 0x624795; continue;
            }
            0x624795 => {
            c.push(m,0x0u32);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffff6c),32));
            { let target = rm(m,c.r[2].wrapping_add(0x8),32); c.push(m, 0x6247a8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6247a8; continue;
            }
            0x6247a8 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006247b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6247b0;
    loop {
        match pc {
            0x6247b0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x161),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(7) { pc = 0x62493c; continue; }
            pc = 0x6247c6; continue;
            }
            0x6247c6 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x13d),8));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x559),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x6247ed; continue; }
            pc = 0x6247df; continue;
            }
            0x6247df => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x65c),16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,16,c.get(2,16)); }
            pc = 0x6247f6; continue;
            }
            0x6247ed => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x10),16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,16,c.get(1,16)); }
            pc = 0x6247f6; continue;
            }
            0x6247f6 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            { let a = c.get(6,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x62483b; continue; }
            pc = 0x624809; continue;
            }
            0x624809 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = rm(m,c.r[1].wrapping_add(0x12a),8); c.and(a,0x6u32,8); }
            if c.cond(5) { pc = 0x624827; continue; }
            pc = 0x62481e; continue;
            }
            0x62481e => {
            c.push(m, 0x624823);
            { let args = []; c.r[0] = host.call(m, 0x61efdf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0xc),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x624827; continue;
            }
            0x624827 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(6,16,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(2)),16));
            { let a = c.get(6,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x624809; continue; }
            pc = 0x62483b; continue;
            }
            0x62483b => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x13d),8));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x559),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x624878; continue; }
            pc = 0x624855; continue;
            }
            0x624855 => {
            c.set(1,32,c.r[3].wrapping_add(0x1ec4));
            c.set(7,32,0x4u32);
            pc = 0x624860; continue;
            }
            0x624860 => {
            c.set(6,32,rm(m,c.r[1],32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x624872; continue; }
            pc = 0x624866; continue;
            }
            0x624866 => {
            c.set(0,32,0xc80000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,r); } }
            pc = 0x624872; continue;
            }
            0x624872 => {
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x624860; continue; }
            pc = 0x624878; continue;
            }
            0x624878 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x624892; continue; }
            pc = 0x624880; continue;
            }
            0x624880 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x624887);
            { let args = []; c.r[0] = host.call(m, 0x60d259, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = rm(m,c.r[4].wrapping_add(0x10),32); c.div32(b); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            pc = 0x624892; continue;
            }
            0x624892 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x6248eb; continue; }
            pc = 0x62489e; continue;
            }
            0x62489e => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            c.set(6,32,c.get(7,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = rm(m,c.r[1].wrapping_add(0x12a),8); c.and(a,0x6u32,8); }
            if c.cond(5) { pc = 0x6248ce; continue; }
            pc = 0x6248bb; continue;
            }
            0x6248bb => {
            c.push(m, 0x6248c0);
            { let args = []; c.r[0] = host.call(m, 0x61efdf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x6248e4; continue; }
            pc = 0x6248ce; continue;
            }
            0x6248ce => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xc),32));
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(2)),16));
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x62489e; continue; }
            pc = 0x6248e2; continue;
            }
            0x6248e2 => {
            pc = 0x6248eb; continue;
            }
            0x6248e4 => {
            { let a = c.get(7,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x624935; continue; }
            pc = 0x6248eb; continue;
            }
            0x6248eb => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x13d),8));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x559),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x624935; continue; }
            pc = 0x624904; continue;
            }
            0x624904 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x624906; continue;
            }
            0x624906 => {
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(6,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1ec4),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(15) { pc = 0x624920; continue; }
            pc = 0x624919; continue;
            }
            0x624919 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            if c.cond(2) { pc = 0x624906; continue; }
            pc = 0x624920; continue;
            }
            0x624920 => {
            c.set(1,32,0xfffeu32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[5].wrapping_add(0x128); wm(m,a,16,c.get(1,16)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x624935 => {
            { let a = c.r[5].wrapping_add(0x128); wm(m,a,16,c.get(7,16)); }
            pc = 0x62493c; continue;
            }
            0x62493c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00624950(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x624950;
    loop {
        match pc {
            0x624950 => {
            { let a = c.get(4,32); let r = c.sub(a,0x48u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x120),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x625892; continue; }
            pc = 0x624979; continue;
            }
            0x624979 => {
            { let a = rm(m,c.r[1].wrapping_add(0x12a),8); c.and(a,0x2eu32,8); }
            if c.cond(5) { pc = 0x625892; continue; }
            pc = 0x624986; continue;
            }
            0x624986 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x130),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x62499e; continue; }
            pc = 0x624990; continue;
            }
            0x624990 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x131),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x625892; continue; }
            pc = 0x62499e; continue;
            }
            0x62499e => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x13d),8));
            c.set(1,32,0x7fffffffu32);
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x559),8));
            c.set(2,32,0xffffu32);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,16,c.get(2,16)); }
            if c.cond(5) { pc = 0x6249dd; continue; }
            pc = 0x6249d7; continue;
            }
            0x6249d7 => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x10),16));
            pc = 0x6249e4; continue;
            }
            0x6249dd => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x65c),16));
            pc = 0x6249e4; continue;
            }
            0x6249e4 => {
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x624c62; continue; }
            pc = 0x6249f1; continue;
            }
            0x6249f1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[2].wrapping_add(0x138),16),16); }
            if c.cond(4) { pc = 0x624c34; continue; }
            pc = 0x624a07; continue;
            }
            0x624a07 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            { let a = rm(m,c.r[0].wrapping_add(0x12a),8); c.and(a,0x6u32,8); }
            if c.cond(5) { pc = 0x624c34; continue; }
            pc = 0x624a27; continue;
            }
            0x624a27 => {
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(6,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x4u32); }
            c.set(6,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(3,32,c.get(1,32));
            c.set(5,32,c.get(6,32));
            pc = 0x624a47; continue;
            }
            0x624a47 => {
            c.set(0,32,c.get(6,32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x2c));
            c.fstp64(m,c.r[4].wrapping_add(0x48));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x624a6e);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x624a76);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x3159721fu32);
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xac8),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x624ab4; continue; }
            pc = 0x624aa0; continue;
            }
            0x624aa0 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0]),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(6,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            pc = 0x624ac0; continue;
            }
            0x624ab4 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x18),32));
            pc = 0x624ac0; continue;
            }
            0x624ac0 => {
            { let a = c.get(1,32); let r = c.imul(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.imul(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(6,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(5,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x624a47; continue; }
            pc = 0x624adf; continue;
            }
            0x624adf => {
            c.set(3,32,sx(c.get(6,16),16)&0xffffffffu32);
            c.set(0,32,sx(c.get(1,16),16)&0xffffffffu32);
            pc = 0x624ae5; continue;
            }
            0x624ae5 => {
            { let a = c.get(0,32); c.sub(a,0x7fffu32,32); }
            if c.cond(15) { pc = 0x624af3; continue; }
            pc = 0x624aec; continue;
            }
            0x624aec => {
            { let a = c.get(0,32); c.sub(a,0xffff8001u32,32); }
            if c.cond(13) { pc = 0x624af9; continue; }
            pc = 0x624af3; continue;
            }
            0x624af3 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            pc = 0x624ae5; continue;
            }
            0x624af9 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x624b0d; continue; }
            pc = 0x624afd; continue;
            }
            0x624afd => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(14)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,0xc0u32,8); c.set(0,8,r); }
            pc = 0x624b9a; continue;
            }
            0x624b0d => {
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x624b5b; continue; }
            pc = 0x624b19; continue;
            }
            0x624b19 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x624b47; continue; }
            pc = 0x624b22; continue;
            }
            0x624b22 => {
            c.set(0,32,0x40000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(2,32,0x80u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x67b1f8),8));
            c.set(2,32,0x40u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            pc = 0x624b8c; continue;
            }
            0x624b47 => {
            { let a = c.get(0,32); let r = c.add(a,0x200u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x67b1f8),8));
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            pc = 0x624b8c; continue;
            }
            0x624b5b => {
            { let a = c.get(7,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x624b7d; continue; }
            pc = 0x624b63; continue;
            }
            0x624b63 => {
            c.set(0,32,0x40000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(2,8,0x40u32);
            { let a = c.get(0,32); let r = c.add(a,0x80u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,8); let r = c.sub(a,rm(m,c.r[0].wrapping_add(0x67b1f8),8),8); c.set(2,8,r); }
            pc = 0x624b8c; continue;
            }
            0x624b7d => {
            { let a = c.get(7,32); let r = c.add(a,0x200u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(7,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x67b1f8),8));
            pc = 0x624b8c; continue;
            }
            0x624b8c => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x624b96; continue; }
            pc = 0x624b90; continue;
            }
            0x624b90 => {
            c.set(0,8,0x80u32);
            { let a = c.get(0,8); let r = c.sub(a,c.get(2,8),8); c.set(0,8,r); }
            pc = 0x624b9a; continue;
            }
            0x624b96 => {
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            c.set(0,8,c.get(2,8));
            pc = 0x624b9a; continue;
            }
            0x624b9a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let r = c.imul(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,8); let r = c.sub(a,rm(m,c.r[2].wrapping_add(0x11),8),8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x559u32,32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(6,32)); }
            if c.cond(13) { pc = 0x624bc2; continue; }
            pc = 0x624bbb; continue;
            }
            0x624bbb => {
            c.set(0,32,0x37u32);
            pc = 0x624beb; continue;
            }
            0x624bc2 => {
            c.fild32(m,c.r[4].wrapping_add(0x20));
            c.fstp64(m,c.r[4].wrapping_add(0x48));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x624bd9);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x624be1);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x7d0u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            pc = 0x624beb; continue;
            }
            0x624beb => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0]));
            { let a = c.get(2,32); c.sub(a,0x37u32,32); }
            if c.cond(14) { pc = 0x624c15; continue; }
            pc = 0x624bfd; continue;
            }
            0x624bfd => {
            c.set(7,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x49u32,32); }
            if c.cond(13) { pc = 0x624c15; continue; }
            pc = 0x624c06; continue;
            }
            0x624c06 => {
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); }
            if c.cond(13) { pc = 0x624c34; continue; }
            pc = 0x624c0c; continue;
            }
            0x624c0c => {
            { let a = c.get(6,32); let r = c.add(a,0xffffffb0u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(6,32)); }
            pc = 0x624c34; continue;
            }
            0x624c15 => {
            { let a = c.get(2,32); c.sub(a,0xb7u32,32); }
            if c.cond(14) { pc = 0x624c34; continue; }
            pc = 0x624c1d; continue;
            }
            0x624c1d => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0xc9u32,32); }
            if c.cond(13) { pc = 0x624c34; continue; }
            pc = 0x624c27; continue;
            }
            0x624c27 => {
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x38),32),32); }
            if c.cond(13) { pc = 0x624c34; continue; }
            pc = 0x624c2d; continue;
            }
            0x624c2d => {
            { let a = c.get(6,32); let r = c.add(a,0xffffffb0u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(6,32)); }
            pc = 0x624c34; continue;
            }
            0x624c34 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x12c),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.get(1,16); c.sub(a,0xffffu32,16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x6249f1; continue; }
            pc = 0x624c5e; continue;
            }
            0x624c5e => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x13),8));
            pc = 0x624c62; continue;
            }
            0x624c62 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x624c79; continue; }
            pc = 0x624c6a; continue;
            }
            0x624c6a => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x65c),16));
            pc = 0x624c87; continue;
            }
            0x624c79 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x10),16));
            pc = 0x624c87; continue;
            }
            0x624c87 => {
            { let a = c.get(1,16); c.sub(a,0xffffu32,16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x624f8f; continue; }
            pc = 0x624c96; continue;
            }
            0x624c96 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            { let a = rm(m,c.r[2].wrapping_add(0x12a),8); c.and(a,0x6u32,8); }
            if c.cond(5) { pc = 0x624f63; continue; }
            pc = 0x624cb7; continue;
            }
            0x624cb7 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x5c),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x624cf7; continue; }
            pc = 0x624cbf; continue;
            }
            0x624cbf => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x128),16),16); }
            if c.cond(4) { pc = 0x624ce5; continue; }
            pc = 0x624cd1; continue;
            }
            0x624cd1 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x13d),8));
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x5d),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x624f63; continue; }
            pc = 0x624ce5; continue;
            }
            0x624ce5 => {
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x624cef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x61f02f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x624f63; continue; }
            pc = 0x624cf7; continue;
            }
            0x624cf7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x4u32); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x12c),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,sx(rm(m,c.r[1].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(7,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(6,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,sx(rm(m,c.r[1].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(3,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(6,32)); }
            pc = 0x624d2d; continue;
            }
            0x624d2d => {
            c.set(5,32,c.get(6,32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(5,32); let r = c.imul(a,c.get(6,32),32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x20));
            c.fstp64(m,c.r[4].wrapping_add(0x50));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x624d54);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x624d5c);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x3159721fu32);
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xac8),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x18),32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            if c.cond(4) { pc = 0x624da0; continue; }
            pc = 0x624d9a; continue;
            }
            0x624d9a => {
            { let a = c.get(7,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(6,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            pc = 0x624da0; continue;
            }
            0x624da0 => {
            { let a = c.get(6,32); let r = c.imul(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.imul(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(6,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(7,32); let n = 0x10u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x624d2d; continue; }
            pc = 0x624dbf; continue;
            }
            0x624dbf => {
            c.set(0,32,sx(c.get(7,16),16)&0xffffffffu32);
            c.set(1,32,c.get(0,32));
            pc = 0x624dc4; continue;
            }
            0x624dc4 => {
            { let a = c.get(1,32); c.sub(a,0x7fffu32,32); }
            if c.cond(15) { pc = 0x624dd4; continue; }
            pc = 0x624dcc; continue;
            }
            0x624dcc => {
            { let a = c.get(1,32); c.sub(a,0xffff8001u32,32); }
            if c.cond(13) { pc = 0x624dd8; continue; }
            pc = 0x624dd4; continue;
            }
            0x624dd4 => {
            { let a = c.get(1,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            pc = 0x624dc4; continue;
            }
            0x624dd8 => {
            c.set(3,32,sx(c.get(6,16),16)&0xffffffffu32);
            pc = 0x624ddb; continue;
            }
            0x624ddb => {
            { let a = c.get(0,32); c.sub(a,0x7fffu32,32); }
            if c.cond(15) { pc = 0x624de9; continue; }
            pc = 0x624de2; continue;
            }
            0x624de2 => {
            { let a = c.get(0,32); c.sub(a,0xffff8001u32,32); }
            if c.cond(13) { pc = 0x624def; continue; }
            pc = 0x624de9; continue;
            }
            0x624de9 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            pc = 0x624ddb; continue;
            }
            0x624def => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x624e03; continue; }
            pc = 0x624df3; continue;
            }
            0x624df3 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(14)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,0xc0u32,8); c.set(0,8,r); }
            pc = 0x624e90; continue;
            }
            0x624e03 => {
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x624e51; continue; }
            pc = 0x624e0f; continue;
            }
            0x624e0f => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x624e3d; continue; }
            pc = 0x624e18; continue;
            }
            0x624e18 => {
            c.set(0,32,0x40000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,0x80u32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x67b1f8),8));
            c.set(1,32,0x40u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            pc = 0x624e82; continue;
            }
            0x624e3d => {
            { let a = c.get(0,32); let r = c.add(a,0x200u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x67b1f8),8));
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            pc = 0x624e82; continue;
            }
            0x624e51 => {
            { let a = c.get(1,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x624e73; continue; }
            pc = 0x624e59; continue;
            }
            0x624e59 => {
            c.set(0,32,0x40000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,8,0x40u32);
            { let a = c.get(0,32); let r = c.add(a,0x80u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,8); let r = c.sub(a,rm(m,c.r[0].wrapping_add(0x67b1f8),8),8); c.set(1,8,r); }
            pc = 0x624e82; continue;
            }
            0x624e73 => {
            { let a = c.get(1,32); let r = c.add(a,0x200u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x67b1f8),8));
            pc = 0x624e82; continue;
            }
            0x624e82 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x624e8c; continue; }
            pc = 0x624e86; continue;
            }
            0x624e86 => {
            c.set(0,8,0x80u32);
            { let a = c.get(0,8); let r = c.sub(a,c.get(1,8),8); c.set(0,8,r); }
            pc = 0x624e90; continue;
            }
            0x624e8c => {
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            c.set(0,8,c.get(1,8));
            pc = 0x624e90; continue;
            }
            0x624e90 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x11),8));
            { let a = c.get(0,8); let r = c.sub(a,c.get(2,8),8); c.set(0,8,r); }
            { let a = c.get(5,32); c.sub(a,0x64u32,32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(0,8)); }
            if c.cond(13) { pc = 0x624ea9; continue; }
            pc = 0x624ea2; continue;
            }
            0x624ea2 => {
            c.set(0,32,0x37u32);
            pc = 0x624ed2; continue;
            }
            0x624ea9 => {
            c.fild32(m,c.r[4].wrapping_add(0x20));
            c.fstp64(m,c.r[4].wrapping_add(0x50));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x624ec0);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x624ec8);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,0x1f4u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            pc = 0x624ed2; continue;
            }
            0x624ed2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0]));
            { let a = c.get(2,32); c.sub(a,0x37u32,32); }
            if c.cond(14) { pc = 0x624f20; continue; }
            pc = 0x624ee4; continue;
            }
            0x624ee4 => {
            c.set(3,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x49u32,32); }
            if c.cond(13) { pc = 0x624f20; continue; }
            pc = 0x624eed; continue;
            }
            0x624eed => {
            c.set(0,32,c.get(6,32));
            c.set(2,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x624f63; continue; }
            pc = 0x624f01; continue;
            }
            0x624f01 => {
            { let a = c.get(0,32); c.sub(a,0x38400u32,32); }
            if c.cond(15) { pc = 0x624f63; continue; }
            pc = 0x624f08; continue;
            }
            0x624f08 => {
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(6,32)); }
            pc = 0x624f63; continue;
            }
            0x624f20 => {
            { let a = c.get(2,32); c.sub(a,0xb7u32,32); }
            if c.cond(14) { pc = 0x624f63; continue; }
            pc = 0x624f28; continue;
            }
            0x624f28 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0xc9u32,32); }
            if c.cond(13) { pc = 0x624f63; continue; }
            pc = 0x624f32; continue;
            }
            0x624f32 => {
            c.set(0,32,c.get(6,32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x624f63; continue; }
            pc = 0x624f46; continue;
            }
            0x624f46 => {
            { let a = c.get(0,32); c.sub(a,0x38400u32,32); }
            if c.cond(15) { pc = 0x624f63; continue; }
            pc = 0x624f4d; continue;
            }
            0x624f4d => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(6,32)); }
            pc = 0x624f63; continue;
            }
            0x624f63 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x12c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(1,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)),16));
            { let a = c.get(1,16); c.sub(a,0xffffu32,16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x624c96; continue; }
            pc = 0x624f87; continue;
            }
            0x624f87 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x13),8));
            pc = 0x624f8f; continue;
            }
            0x624f8f => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x62518c; continue; }
            pc = 0x624f97; continue;
            }
            0x624f97 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[0].wrapping_add(0x1ea6));
            { let a = c.get(0,32); let r = c.add(a,0x1ec4u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x624fba; continue;
            }
            0x624fba => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = rm(m,c.r[2],32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x62515a; continue; }
            pc = 0x624fc7; continue;
            }
            0x624fc7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0xfffeu32);
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x128),16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x625001; continue; }
            pc = 0x624fdf; continue;
            }
            0x624fdf => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x5c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x625001; continue; }
            pc = 0x624fe7; continue;
            }
            0x624fe7 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x13d),8));
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x5d),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x62515a; continue; }
            pc = 0x625001; continue;
            }
            0x625001 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(1,32,sx(rm(m,c.r[5].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[5].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(6,16,rm(m,c.r[0].wrapping_add(0xfffffff8),16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0],16));
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(3,32,sx(c.get(7,16),16)&0xffffffffu32);
            c.set(0,32,sx(c.get(6,16),16)&0xffffffffu32);
            pc = 0x625022; continue;
            }
            0x625022 => {
            { let a = c.get(0,32); c.sub(a,0x7fffu32,32); }
            if c.cond(15) { pc = 0x625030; continue; }
            pc = 0x625029; continue;
            }
            0x625029 => {
            { let a = c.get(0,32); c.sub(a,0xffff8001u32,32); }
            if c.cond(13) { pc = 0x625036; continue; }
            pc = 0x625030; continue;
            }
            0x625030 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(3,32,r); }
            pc = 0x625022; continue;
            }
            0x625036 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x62504a; continue; }
            pc = 0x62503a; continue;
            }
            0x62503a => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(14)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,0xc0u32,8); c.set(0,8,r); }
            pc = 0x6250d7; continue;
            }
            0x62504a => {
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x625098; continue; }
            pc = 0x625056; continue;
            }
            0x625056 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x625084; continue; }
            pc = 0x62505f; continue;
            }
            0x62505f => {
            c.set(0,32,0x40000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,0x80u32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x67b1f8),8));
            c.set(1,32,0x40u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            pc = 0x6250c9; continue;
            }
            0x625084 => {
            { let a = c.get(0,32); let r = c.add(a,0x200u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x67b1f8),8));
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            pc = 0x6250c9; continue;
            }
            0x625098 => {
            { let a = c.get(1,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x6250ba; continue; }
            pc = 0x6250a0; continue;
            }
            0x6250a0 => {
            c.set(0,32,0x40000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,8,0x40u32);
            { let a = c.get(0,32); let r = c.add(a,0x80u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,8); let r = c.sub(a,rm(m,c.r[0].wrapping_add(0x67b1f8),8),8); c.set(1,8,r); }
            pc = 0x6250c9; continue;
            }
            0x6250ba => {
            { let a = c.get(1,32); let r = c.add(a,0x200u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x67b1f8),8));
            pc = 0x6250c9; continue;
            }
            0x6250c9 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x6250d3; continue; }
            pc = 0x6250cd; continue;
            }
            0x6250cd => {
            c.set(0,8,0x80u32);
            { let a = c.get(0,8); let r = c.sub(a,c.get(1,8),8); c.set(0,8,r); }
            pc = 0x6250d7; continue;
            }
            0x6250d3 => {
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            c.set(0,8,c.get(1,8));
            pc = 0x6250d7; continue;
            }
            0x6250d7 => {
            { let a = c.get(0,8); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0x11),8),8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x37u32,8); }
            if c.cond(6) { pc = 0x62511b; continue; }
            pc = 0x6250de; continue;
            }
            0x6250de => {
            { let a = c.get(0,8); c.sub(a,0x49u32,8); }
            if c.cond(3) { pc = 0x62511b; continue; }
            pc = 0x6250e2; continue;
            }
            0x6250e2 => {
            c.set(0,32,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x62515a; continue; }
            pc = 0x6250f6; continue;
            }
            0x6250f6 => {
            { let a = c.get(0,32); c.sub(a,0x38400u32,32); }
            if c.cond(15) { pc = 0x62515a; continue; }
            pc = 0x6250fd; continue;
            }
            0x6250fd => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0xfffeu32);
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,16,c.get(0,16)); }
            pc = 0x62515a; continue;
            }
            0x62511b => {
            { let a = c.get(0,8); c.sub(a,0xb7u32,8); }
            if c.cond(6) { pc = 0x62515a; continue; }
            pc = 0x62511f; continue;
            }
            0x62511f => {
            { let a = c.get(0,8); c.sub(a,0xc9u32,8); }
            if c.cond(3) { pc = 0x62515a; continue; }
            pc = 0x625123; continue;
            }
            0x625123 => {
            c.set(0,32,c.get(7,32));
            c.set(2,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x62515a; continue; }
            pc = 0x625137; continue;
            }
            0x625137 => {
            { let a = c.get(0,32); c.sub(a,0x38400u32,32); }
            if c.cond(15) { pc = 0x62515a; continue; }
            pc = 0x62513e; continue;
            }
            0x62513e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0xfffeu32);
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,16,c.get(0,16)); }
            pc = 0x62515a; continue;
            }
            0x62515a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x624fba; continue; }
            pc = 0x62518c; continue;
            }
            0x62518c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xac8),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x62542e; continue; }
            pc = 0x6251a0; continue;
            }
            0x6251a0 => {
            c.set(6,16,rm(m,c.r[4].wrapping_add(0x30),16));
            c.set(3,8,0x20u32);
            { let a = c.get(6,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x6252e0; continue; }
            pc = 0x6251b2; continue;
            }
            0x6251b2 => {
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x130),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x6252e0; continue; }
            pc = 0x6251c0; continue;
            }
            0x6251c0 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x13d),8));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x559),8));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xaca));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xaca),8));
            { let a = c.get(0,8); c.sub(a,0x7fu32,8); }
            if c.cond(3) { pc = 0x6251e7; continue; }
            pc = 0x6251e3; continue;
            }
            0x6251e3 => {
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            pc = 0x6251e7; continue;
            }
            0x6251e7 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            { let a = c.get(6,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),16),16); }
            if c.cond(3) { pc = 0x62526c; continue; }
            pc = 0x6251f3; continue;
            }
            0x6251f3 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[6]),32));
            { let a = rm(m,c.r[2].wrapping_add(0x12a),8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x625227; continue; }
            pc = 0x62520e; continue;
            }
            0x62520e => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x13d),8));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x559),8));
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0xaca); wm(m,a,8,0xffu32); }
            pc = 0x625227; continue;
            }
            0x625227 => {
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.push(m,0x3u32);
            c.push(m,0xffffffffu32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.fstp64(m,c.r[4].wrapping_add(0x60));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x625246);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62524e);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x625258);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x620f0b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6]),32));
            c.push(m, 0x62526a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x61f96f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x6252ae; continue;
            }
            0x62526c => {
            c.fild32(m,c.r[4].wrapping_add(0x38));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.fstp64(m,c.r[4].wrapping_add(0x58));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x625287);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x62528f);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x625299);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x620f0b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x30),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            c.push(m,c.get(0,32));
            c.set(0,8,0xfeu32);
            { let a = c.get(0,8); let r = c.sub(a,c.get(2,8),8); c.set(0,8,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x6252ae);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x60130b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x6252ae; continue;
            }
            0x6252ae => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x13d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x559),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xad2),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x6252d9; continue; }
            pc = 0x6252d2; continue;
            }
            0x6252d2 => {
            c.push(m,0xeu32);
            c.push(m, 0x6252d9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x611649, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6252d9; continue;
            }
            0x6252d9 => {
            { let a = c.r[5].wrapping_add(0x130); wm(m,a,8,0x96u32); }
            pc = 0x6252e0; continue;
            }
            0x6252e0 => {
            c.set(6,16,rm(m,c.r[4].wrapping_add(0x32),16));
            { let a = c.get(6,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x625892; continue; }
            pc = 0x6252f0; continue;
            }
            0x6252f0 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x131),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x625892; continue; }
            pc = 0x6252fe; continue;
            }
            0x6252fe => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x13d),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x559),8));
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xaca));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xaca),8));
            { let a = c.get(0,8); c.sub(a,0x7fu32,8); }
            if c.cond(3) { pc = 0x62532b; continue; }
            pc = 0x625327; continue;
            }
            0x625327 => {
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            pc = 0x62532b; continue;
            }
            0x62532b => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            { let a = c.get(6,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x14),16),16); }
            if c.cond(3) { pc = 0x6253b0; continue; }
            pc = 0x625337; continue;
            }
            0x625337 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x32),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[6]),32));
            { let a = rm(m,c.r[2].wrapping_add(0x12a),8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x62536b; continue; }
            pc = 0x625352; continue;
            }
            0x625352 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x13d),8));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x559),8));
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0xaca); wm(m,a,8,0xffu32); }
            pc = 0x62536b; continue;
            }
            0x62536b => {
            c.fild32(m,c.r[4].wrapping_add(0x3c));
            c.push(m,0x3u32);
            c.push(m,0xffffffffu32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.fstp64(m,c.r[4].wrapping_add(0x60));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x62538a);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x625392);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x62539c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x620f0b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6]),32));
            c.push(m, 0x6253ae);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x61f96f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x6253f2; continue;
            }
            0x6253b0 => {
            c.fild32(m,c.r[4].wrapping_add(0x3c));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.fstp64(m,c.r[4].wrapping_add(0x58));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x6253cb);
            c.crt_call(m, 0x63aae4);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.push(m, 0x6253d3);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x6253dd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x620f0b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x32),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            c.push(m,c.get(0,32));
            c.set(0,8,0xfeu32);
            { let a = c.get(0,8); let r = c.sub(a,c.get(2,8),8); c.set(0,8,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x6253f2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x60130b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x6253f2; continue;
            }
            0x6253f2 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x13d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x559),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xad2),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x62541d; continue; }
            pc = 0x625416; continue;
            }
            0x625416 => {
            c.push(m,0xeu32);
            c.push(m, 0x62541d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x611649, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x62541d; continue;
            }
            0x62541d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[5].wrapping_add(0x131); wm(m,a,8,0x96u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            0x62542e => {
            { let a = rm(m,c.r[4].wrapping_add(0x30),16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x62545e; continue; }
            pc = 0x625437; continue;
            }
            0x625437 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x130),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x62545e; continue; }
            pc = 0x625441; continue;
            }
            0x625441 => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xe),16));
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xa),16));
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x40),16),16); c.set(0,16,r); }
            { let a = c.get(1,16); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x48),16),16); c.set(1,16,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x62545e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x61e8ef, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x62545e; continue;
            }
            0x62545e => {
            { let a = rm(m,c.r[4].wrapping_add(0x32),16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x62548e; continue; }
            pc = 0x625467; continue;
            }
            0x625467 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x131),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x62548e; continue; }
            pc = 0x625471; continue;
            }
            0x625471 => {
            c.set(2,16,rm(m,c.r[5].wrapping_add(0xe),16));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xa),16));
            { let a = c.get(2,16); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x44),16),16); c.set(2,16,r); }
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x4c),16),16); c.set(0,16,r); }
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m, 0x62548e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x61e8ef, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x62548e; continue;
            }
            0x62548e => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x5c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x625892; continue; }
            pc = 0x62549a; continue;
            }
            0x62549a => {
            c.set(1,32,0xffffu32);
            { let a = rm(m,c.r[4].wrapping_add(0x30),16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x625892; continue; }
            pc = 0x6254aa; continue;
            }
            0x6254aa => {
            { let a = rm(m,c.r[4].wrapping_add(0x32),16); c.sub(a,c.get(1,16),16); }
            if c.cond(5) { pc = 0x625892; continue; }
            pc = 0x6254b5; continue;
            }
            0x6254b5 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            c.set(5,32,rm(m,c.r[5].wrapping_add(0x12c),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x6254d0; continue; }
            pc = 0x6254c7; continue;
            }
            0x6254c7 => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x65c),16));
            pc = 0x6254d4; continue;
            }
            0x6254d0 => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x10),16));
            pc = 0x6254d4; continue;
            }
            0x6254d4 => {
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x625652; continue; }
            pc = 0x6254e1; continue;
            }
            0x6254e1 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            pc = 0x6254f3; continue;
            }
            0x6254f3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = rm(m,c.r[0].wrapping_add(0x12a),8); c.and(a,0x6u32,8); }
            if c.cond(5) { pc = 0x625638; continue; }
            pc = 0x625514; continue;
            }
            0x625514 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,sx(rm(m,c.r[0].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(6,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(6,32,sx(rm(m,c.r[0].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(0,32,c.get(1,32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.imul(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(5,32,sx(c.get(6,16),16)&0xffffffffu32);
            c.set(0,32,sx(c.get(1,16),16)&0xffffffffu32);
            pc = 0x62553e; continue;
            }
            0x62553e => {
            { let a = c.get(0,32); c.sub(a,0x7fffu32,32); }
            if c.cond(15) { pc = 0x62554c; continue; }
            pc = 0x625545; continue;
            }
            0x625545 => {
            { let a = c.get(0,32); c.sub(a,0xffff8001u32,32); }
            if c.cond(13) { pc = 0x625552; continue; }
            pc = 0x62554c; continue;
            }
            0x62554c => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            pc = 0x62553e; continue;
            }
            0x625552 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x625566; continue; }
            pc = 0x625556; continue;
            }
            0x625556 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(14)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,0xc0u32,8); c.set(0,8,r); }
            pc = 0x6255f3; continue;
            }
            0x625566 => {
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x6255b4; continue; }
            pc = 0x625572; continue;
            }
            0x625572 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x6255a0; continue; }
            pc = 0x62557b; continue;
            }
            0x62557b => {
            c.set(0,32,0x40000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(2,32,0x80u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x67b1f8),8));
            c.set(2,32,0x40u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            pc = 0x6255e5; continue;
            }
            0x6255a0 => {
            { let a = c.get(0,32); let r = c.add(a,0x200u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x67b1f8),8));
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            pc = 0x6255e5; continue;
            }
            0x6255b4 => {
            { let a = c.get(3,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x6255d6; continue; }
            pc = 0x6255bc; continue;
            }
            0x6255bc => {
            c.set(0,32,0x40000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(2,8,0x40u32);
            { let a = c.get(0,32); let r = c.add(a,0x80u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,8); let r = c.sub(a,rm(m,c.r[0].wrapping_add(0x67b1f8),8),8); c.set(2,8,r); }
            pc = 0x6255e5; continue;
            }
            0x6255d6 => {
            { let a = c.get(3,32); let r = c.add(a,0x200u32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(3,32,r); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x67b1f8),8));
            pc = 0x6255e5; continue;
            }
            0x6255e5 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x6255ef; continue; }
            pc = 0x6255e9; continue;
            }
            0x6255e9 => {
            c.set(0,8,0x80u32);
            { let a = c.get(0,8); let r = c.sub(a,c.get(2,8),8); c.set(0,8,r); }
            pc = 0x6255f3; continue;
            }
            0x6255ef => {
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            c.set(0,8,c.get(2,8));
            pc = 0x6255f3; continue;
            }
            0x6255f3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,8); let r = c.sub(a,rm(m,c.r[2].wrapping_add(0x11),8),8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x80u32,8); }
            if c.cond(3) { pc = 0x62561c; continue; }
            pc = 0x6255fe; continue;
            }
            0x6255fe => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); }
            if c.cond(13) { pc = 0x625638; continue; }
            pc = 0x625604; continue;
            }
            0x625604 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(6,32)); }
            pc = 0x625638; continue;
            }
            0x62561c => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x38),32),32); }
            if c.cond(13) { pc = 0x625638; continue; }
            pc = 0x625622; continue;
            }
            0x625622 => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(6,32)); }
            pc = 0x625638; continue;
            }
            0x625638 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)),16));
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x6254f3; continue; }
            pc = 0x625652; continue;
            }
            0x625652 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x6257de; continue; }
            pc = 0x62565e; continue;
            }
            0x62565e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(2,32,c.r[0].wrapping_add(0x1ea6));
            { let a = c.get(0,32); let r = c.add(a,0x1ec4u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            pc = 0x62567d; continue;
            }
            0x62567d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = rm(m,c.r[0],32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x6257b5; continue; }
            pc = 0x62568a; continue;
            }
            0x62568a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(6,32,sx(rm(m,c.r[2].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xfffffff8),16));
            c.set(2,32,sx(rm(m,c.r[2].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[0],16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(5,32,sx(c.get(6,16),16)&0xffffffffu32);
            c.set(0,32,sx(c.get(1,16),16)&0xffffffffu32);
            pc = 0x6256bb; continue;
            }
            0x6256bb => {
            { let a = c.get(0,32); c.sub(a,0x7fffu32,32); }
            if c.cond(15) { pc = 0x6256c9; continue; }
            pc = 0x6256c2; continue;
            }
            0x6256c2 => {
            { let a = c.get(0,32); c.sub(a,0xffff8001u32,32); }
            if c.cond(13) { pc = 0x6256cf; continue; }
            pc = 0x6256c9; continue;
            }
            0x6256c9 => {
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(5,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(5,32,r); }
            pc = 0x6256bb; continue;
            }
            0x6256cf => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x6256e3; continue; }
            pc = 0x6256d3; continue;
            }
            0x6256d3 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(14)));
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.and(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.add(a,0xc0u32,8); c.set(0,8,r); }
            pc = 0x625770; continue;
            }
            0x6256e3 => {
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(5,32); c.idiv32(b); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x625731; continue; }
            pc = 0x6256ef; continue;
            }
            0x6256ef => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x62571d; continue; }
            pc = 0x6256f8; continue;
            }
            0x6256f8 => {
            c.set(0,32,0x40000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(2,32,0x80u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x67b1f8),8));
            c.set(2,32,0x40u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            pc = 0x625762; continue;
            }
            0x62571d => {
            { let a = c.get(0,32); let r = c.add(a,0x200u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x67b1f8),8));
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            pc = 0x625762; continue;
            }
            0x625731 => {
            { let a = c.get(3,32); c.sub(a,0x10000u32,32); }
            if c.cond(14) { pc = 0x625753; continue; }
            pc = 0x625739; continue;
            }
            0x625739 => {
            c.set(0,32,0x40000000u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(2,8,0x40u32);
            { let a = c.get(0,32); let r = c.add(a,0x80u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,8); let r = c.sub(a,rm(m,c.r[0].wrapping_add(0x67b1f8),8),8); c.set(2,8,r); }
            pc = 0x625762; continue;
            }
            0x625753 => {
            { let a = c.get(3,32); let r = c.add(a,0x200u32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let n = 0xau32; let r = c.sar(a,n,32); c.set(3,32,r); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x67b1f8),8));
            pc = 0x625762; continue;
            }
            0x625762 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(14) { pc = 0x62576c; continue; }
            pc = 0x625766; continue;
            }
            0x625766 => {
            c.set(0,8,0x80u32);
            { let a = c.get(0,8); let r = c.sub(a,c.get(2,8),8); c.set(0,8,r); }
            pc = 0x625770; continue;
            }
            0x62576c => {
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            c.set(0,8,c.get(2,8));
            pc = 0x625770; continue;
            }
            0x625770 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,8); let r = c.sub(a,rm(m,c.r[2].wrapping_add(0x11),8),8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x80u32,8); }
            if c.cond(3) { pc = 0x625799; continue; }
            pc = 0x62577b; continue;
            }
            0x62577b => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x3c),32),32); }
            if c.cond(13) { pc = 0x6257b5; continue; }
            pc = 0x625781; continue;
            }
            0x625781 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(6,32)); }
            pc = 0x6257b5; continue;
            }
            0x625799 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x38),32),32); }
            if c.cond(13) { pc = 0x6257b5; continue; }
            pc = 0x62579f; continue;
            }
            0x62579f => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(6,32)); }
            pc = 0x6257b5; continue;
            }
            0x6257b5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(0,16); c.sub(a,0x4u32,16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x62567d; continue; }
            pc = 0x6257de; continue;
            }
            0x6257de => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x130),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x6257f3; continue; }
            pc = 0x6257ec; continue;
            }
            0x6257ec => {
            { let a = c.get(2,16); let r = c.or(a,0xffffu32,16); c.set(2,16,r); }
            pc = 0x6257f8; continue;
            }
            0x6257f3 => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x30),16));
            pc = 0x6257f8; continue;
            }
            0x6257f8 => {
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x131),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x625808; continue; }
            pc = 0x625802; continue;
            }
            0x625802 => {
            { let a = c.get(0,16); let r = c.or(a,0xffffu32,16); c.set(0,16,r); }
            pc = 0x62580d; continue;
            }
            0x625808 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x32),16));
            pc = 0x62580d; continue;
            }
            0x62580d => {
            { let a = c.get(2,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x625831; continue; }
            pc = 0x625814; continue;
            }
            0x625814 => {
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x62583e; continue; }
            pc = 0x62581a; continue;
            }
            0x62581a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x62582c; continue; }
            pc = 0x625826; continue;
            }
            0x625826 => {
            { let a = c.get(0,16); let r = c.or(a,0xffffu32,16); c.set(0,16,r); }
            pc = 0x62583e; continue;
            }
            0x62582c => {
            { let a = c.get(2,16); let r = c.or(a,0xffffu32,16); c.set(2,16,r); }
            pc = 0x625831; continue;
            }
            0x625831 => {
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x625892; continue; }
            pc = 0x625837; continue;
            }
            0x625837 => {
            { let a = c.get(2,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x62586d; continue; }
            pc = 0x62583e; continue;
            }
            0x62583e => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x130),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x62586d; continue; }
            pc = 0x625848; continue;
            }
            0x625848 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xe),16));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x40),16),16); c.set(0,16,r); }
            { let a = c.get(2,16); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x48),16),16); c.set(2,16,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m, 0x625863);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x61e8ef, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            0x62586d => {
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x625892; continue; }
            pc = 0x625873; continue;
            }
            0x625873 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x625892; continue; }
            pc = 0x625877; continue;
            }
            0x625877 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xe),16));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.get(0,16); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x44),16),16); c.set(0,16,r); }
            { let a = c.get(2,16); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x4c),16),16); c.set(2,16,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m,0x1u32);
            c.push(m, 0x625892);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x61e8ef, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x625892; continue;
            }
            0x625892 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00625bf0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x625bf0;
    loop {
        match pc {
            0x625bf0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = rm(m,c.r[7].wrapping_add(0x12a),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x625c09; continue; }
            pc = 0x625c02; continue;
            }
            0x625c02 => {
            { let a = rm(m,c.r[7].wrapping_add(0x20),16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x625c14; continue; }
            pc = 0x625c09; continue;
            }
            0x625c09 => {
            { let a = rm(m,c.r[7].wrapping_add(0x1164),16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x625c6c; continue; }
            pc = 0x625c14; continue;
            }
            0x625c14 => {
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0x20),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x12c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x625c33);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5fff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x1164),16),16); }
            if c.cond(5) { pc = 0x625c4d; continue; }
            pc = 0x625c44; continue;
            }
            0x625c44 => {
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x1166),16),16); }
            if c.cond(4) { pc = 0x625c6c; continue; }
            pc = 0x625c4d; continue;
            }
            0x625c4d => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x20),16));
            { let a = c.r[7].wrapping_add(0x1166); wm(m,a,16,c.get(1,16)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x22),16));
            { let a = c.r[7].wrapping_add(0x1164); wm(m,a,16,c.get(0,16)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x625c6c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6258a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x625c6c; continue;
            }
            0x625c6c => {
            c.set(2,32,sx(rm(m,c.r[7].wrapping_add(0xe),16),16)&0xffffffffu32);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(0,32,sx(rm(m,c.r[7].wrapping_add(0xa),16),16)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x12c),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x625c8b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5fff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,16); c.sub(a,0x40u32,16); }
            if c.cond(3) { pc = 0x625cc0; continue; }
            pc = 0x625c99; continue;
            }
            0x625c99 => {
            { let a = c.get(1,16); c.sub(a,0x40u32,16); }
            if c.cond(3) { pc = 0x625cc0; continue; }
            pc = 0x625c9f; continue;
            }
            0x625c9f => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x164),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x625cc5; continue; }
            pc = 0x625cc0; continue;
            }
            0x625cc0 => {
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0xffu32); }
            pc = 0x625cc5; continue;
            }
            0x625cc5 => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x8u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x625cdd; continue;
            }
            0x625cdd => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.sub(a,0xe0u32,32); }
            if c.cond(7) { pc = 0x625d9b; continue; }
            pc = 0x625ced; continue;
            }
            0x625ced => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x6260b8),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x626094)); continue;
            }
            0x625cfc => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            pc = 0x625d90; continue;
            }
            0x625d0a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x625d90; continue;
            }
            0x625d16 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            pc = 0x625d9b; continue;
            }
            0x625d2c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x625d9b; continue;
            }
            0x625d43 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x625d9b; continue;
            }
            0x625d59 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x625d9b; continue;
            }
            0x625d70 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            pc = 0x625d9b; continue;
            }
            0x625d86 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            pc = 0x625d90; continue;
            }
            0x625d90 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            pc = 0x625d9b; continue;
            }
            0x625d9b => {
            { let a = c.get(0,16); c.sub(a,0x40u32,16); }
            if c.cond(3) { pc = 0x625e03; continue; }
            pc = 0x625da1; continue;
            }
            0x625da1 => {
            { let a = c.get(6,16); c.sub(a,0x40u32,16); }
            if c.cond(3) { pc = 0x625e03; continue; }
            pc = 0x625da7; continue;
            }
            0x625da7 => {
            c.set(2,32,c.get(0,32));
            c.set(3,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(5,32,c.get(7,32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(2,32),32); c.set(5,32,r); }
            c.set(3,8,rm(m,c.r[3].wrapping_add(c.r[5]).wrapping_add(0x164),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            if c.cond(5) { pc = 0x625e0a; continue; }
            pc = 0x625dcd; continue;
            }
            0x625dcd => {
            c.set(2,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x20u32); }
            pc = 0x625dd7; continue;
            }
            0x625dd7 => {
            c.set(5,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.r[7].wrapping_add(c.r[5]).wrapping_add(0x27); wm(m,a,8,0xc0u32); }
            c.set(5,32,c.get(2,32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(c.r[5]).wrapping_add(0x27); wm(m,a,8,0xc0u32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x625dd7; continue; }
            pc = 0x625dfe; continue;
            }
            0x625dfe => {
            pc = 0x625ed3; continue;
            }
            0x625e03 => {
            { let a = c.get(3,8); let r = c.or(a,0xffu32,8); c.set(3,8,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,c.get(3,8)); }
            pc = 0x625e0a; continue;
            }
            0x625e0a => {
            { let a = rm(m,c.r[7].wrapping_add(0x12a),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x625ed3; continue; }
            pc = 0x625e17; continue;
            }
            0x625e17 => {
            { let a = rm(m,c.r[7].wrapping_add(0x20),16); c.sub(a,0xffffu32,16); }
            if c.cond(4) { pc = 0x625ed3; continue; }
            pc = 0x625e22; continue;
            }
            0x625e22 => {
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[4].wrapping_add(0x1c),8),8); }
            if c.cond(2) { pc = 0x625e71; continue; }
            pc = 0x625e28; continue;
            }
            0x625e28 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x11u32); }
            c.set(2,32,c.get(1,32));
            pc = 0x625e36; continue;
            }
            0x625e36 => {
            c.set(5,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x27),8); c.sub(a,0x40u32,8); }
            c.set(5,32,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x27));
            if c.cond(3) { pc = 0x625e4d; continue; }
            pc = 0x625e49; continue;
            }
            0x625e49 => {
            { let a = c.r[5]; wm(m,a,8,0x40u32); }
            pc = 0x625e4d; continue;
            }
            0x625e4d => {
            c.set(5,32,c.get(2,32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x27),8); c.sub(a,0x40u32,8); }
            c.set(5,32,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x27));
            if c.cond(3) { pc = 0x625e64; continue; }
            pc = 0x625e60; continue;
            }
            0x625e60 => {
            { let a = c.r[5]; wm(m,a,8,0x40u32); }
            pc = 0x625e64; continue;
            }
            0x625e64 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x625e36; continue; }
            pc = 0x625e71; continue;
            }
            0x625e71 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x625ed3; continue; }
            pc = 0x625e8a; continue;
            }
            0x625e8a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x11u32); }
            c.set(2,32,c.get(1,32));
            pc = 0x625e98; continue;
            }
            0x625e98 => {
            c.set(5,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x27),8); c.sub(a,0x10u32,8); }
            c.set(5,32,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x27));
            if c.cond(3) { pc = 0x625eaf; continue; }
            pc = 0x625eab; continue;
            }
            0x625eab => {
            { let a = c.r[5]; wm(m,a,8,0x10u32); }
            pc = 0x625eaf; continue;
            }
            0x625eaf => {
            c.set(5,32,c.get(2,32));
            { let a = c.get(5,32); let r = c.and(a,0xffu32,32); c.set(5,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x27),8); c.sub(a,0x10u32,8); }
            c.set(5,32,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x27));
            if c.cond(3) { pc = 0x625ec6; continue; }
            pc = 0x625ec2; continue;
            }
            0x625ec2 => {
            { let a = c.r[5]; wm(m,a,8,0x10u32); }
            pc = 0x625ec6; continue;
            }
            0x625ec6 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x625e98; continue; }
            pc = 0x625ed3; continue;
            }
            0x625ed3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(2,32); let r = c.add(a,0x20u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x625cdd; continue; }
            pc = 0x625eed; continue;
            }
            0x625eed => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x10u32); }
            pc = 0x625f05; continue;
            }
            0x625f01 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x625f05; continue;
            }
            0x625f05 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); c.sub(a,0xf0u32,32); }
            if c.cond(7) { pc = 0x625f52; continue; }
            pc = 0x625f11; continue;
            }
            0x625f11 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6261e0),8));
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x62619c)); continue;
            }
            0x625f20 => {
            c.set(0,32,c.r[1].wrapping_add(0xfffffffe));
            pc = 0x625f23; continue;
            }
            0x625f23 => {
            { let a = c.get(5,16); c.sub(a,0x40u32,16); }
            if c.cond(3) { pc = 0x62608a; continue; }
            pc = 0x625f2d; continue;
            }
            0x625f2d => {
            { let a = c.get(0,16); c.sub(a,0x40u32,16); }
            if c.cond(3) { pc = 0x62608a; continue; }
            pc = 0x625f37; continue;
            }
            0x625f37 => {
            c.set(1,32,c.get(5,32));
            c.set(2,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x164),8));
            pc = 0x625f52; continue;
            }
            0x625f52 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x625fdc; continue; }
            pc = 0x625f5a; continue;
            }
            0x625f5a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,0x10u32);
            c.set(6,32,c.get(1,32));
            pc = 0x625f65; continue;
            }
            0x625f65 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x27),8); c.sub(a,0x40u32,8); }
            c.set(0,32,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x27));
            if c.cond(3) { pc = 0x625f7a; continue; }
            pc = 0x625f77; continue;
            }
            0x625f77 => {
            { let a = c.r[0]; wm(m,a,8,0x40u32); }
            pc = 0x625f7a; continue;
            }
            0x625f7a => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x27),8); c.sub(a,0x40u32,8); }
            c.set(0,32,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x27));
            if c.cond(3) { pc = 0x625f8f; continue; }
            pc = 0x625f8c; continue;
            }
            0x625f8c => {
            { let a = c.r[0]; wm(m,a,8,0x40u32); }
            pc = 0x625f8f; continue;
            }
            0x625f8f => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x625f65; continue; }
            pc = 0x625f94; continue;
            }
            0x625f94 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(6,32,0x30u32);
            c.set(0,32,c.r[1].wrapping_add(0xfffffff0));
            c.set(2,32,c.get(0,32));
            c.set(1,32,c.r[0].wrapping_add(0x20));
            pc = 0x625fa5; continue;
            }
            0x625fa5 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x27),8); c.sub(a,0x10u32,8); }
            c.set(0,32,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x27));
            if c.cond(3) { pc = 0x625fbe; continue; }
            pc = 0x625fb7; continue;
            }
            0x625fb7 => {
            { let a = c.r[0]; wm(m,a,8,0x10u32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x625fbe; continue;
            }
            0x625fbe => {
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x27),8); c.sub(a,0x10u32,8); }
            c.set(0,32,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x27));
            if c.cond(3) { pc = 0x625fd7; continue; }
            pc = 0x625fd0; continue;
            }
            0x625fd0 => {
            { let a = c.r[0]; wm(m,a,8,0x10u32); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x625fd7; continue;
            }
            0x625fd7 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            if c.cond(5) { pc = 0x625fa5; continue; }
            pc = 0x625fdc; continue;
            }
            0x625fdc => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x625f01; continue; }
            pc = 0x625ff6; continue;
            }
            0x625ff6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x625ffe => {
            { let a = c.get(1,32); let r = c.add(a,0xfffffffeu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[5].wrapping_add(0x1));
            pc = 0x62607e; continue;
            }
            0x626006 => {
            { let a = c.get(1,32); let r = c.add(a,0xfffffffeu32,32); c.set(1,32,r); }
            pc = 0x626009; continue;
            }
            0x626009 => {
            c.set(0,32,c.r[5].wrapping_add(0x2));
            pc = 0x62607e; continue;
            }
            0x62600e => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.set(0,32,c.r[5].wrapping_add(0x2));
            pc = 0x62607e; continue;
            }
            0x626014 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(0,32,c.r[5].wrapping_add(0x2));
            pc = 0x62607e; continue;
            }
            0x62601a => {
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[5].wrapping_add(0x2));
            pc = 0x62607e; continue;
            }
            0x626022 => {
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            c.set(0,32,c.r[5].wrapping_add(0x1));
            pc = 0x62607e; continue;
            }
            0x62602a => {
            c.set(0,32,c.r[1].wrapping_add(0x2));
            pc = 0x625f23; continue;
            }
            0x626032 => {
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            pc = 0x62607b; continue;
            }
            0x626037 => {
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            pc = 0x62603a; continue;
            }
            0x62603a => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffffe));
            pc = 0x62607e; continue;
            }
            0x62603f => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(0,32,c.r[5].wrapping_add(0xfffffffe));
            pc = 0x62607e; continue;
            }
            0x626045 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            pc = 0x62604b; continue;
            }
            0x626048 => {
            { let a = c.get(1,32); let r = c.add(a,0xfffffffeu32,32); c.set(1,32,r); }
            pc = 0x62604b; continue;
            }
            0x62604b => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffffe));
            { let a = c.get(0,16); c.sub(a,0x40u32,16); }
            if c.cond(3) { pc = 0x62608a; continue; }
            pc = 0x626054; continue;
            }
            0x626054 => {
            { let a = c.get(1,16); c.sub(a,0x40u32,16); }
            if c.cond(3) { pc = 0x62608a; continue; }
            pc = 0x62605a; continue;
            }
            0x62605a => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x164),8));
            pc = 0x625f52; continue;
            }
            0x626078 => {
            { let a = c.get(1,32); let r = c.add(a,0xfffffffeu32,32); c.set(1,32,r); }
            pc = 0x62607b; continue;
            }
            0x62607b => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffff));
            pc = 0x62607e; continue;
            }
            0x62607e => {
            { let a = c.get(0,16); c.sub(a,0x40u32,16); }
            if c.cond(3) { pc = 0x62608a; continue; }
            pc = 0x626084; continue;
            }
            0x626084 => {
            { let a = c.get(1,16); c.sub(a,0x40u32,16); }
            if c.cond(2) { pc = 0x62605a; continue; }
            pc = 0x62608a; continue;
            }
            0x62608a => {
            { let a = c.get(3,8); let r = c.or(a,0xffu32,8); c.set(3,8,r); }
            pc = 0x625fdc; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006258a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6258a0;
    loop {
        match pc {
            0x6258a0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x5cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x1u32);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(5,32,c.r[7].wrapping_add(0x164));
            pc = 0x625903; continue;
            }
            0x625903 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(5,32));
            pc = 0x625907; continue;
            }
            0x625907 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x12c),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[6]).wrapping_add(0x1ee0),8));
            { let a = c.get(2,8); let r = c.neg(a,8); c.set(2,8,r); }
            { let a = c.get(2,8); let r = c.sbb(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(0,32); let r = c.add(a,0x40u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            { let a = c.get(1,32); let r = c.add(a,0x40u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x1000u32,32); }
            if c.cond(12) { pc = 0x625907; continue; }
            pc = 0x62592f; continue;
            }
            0x62592f => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(6,32); c.sub(a,0x40u32,32); }
            if c.cond(12) { pc = 0x625903; continue; }
            pc = 0x625936; continue;
            }
            0x625936 => {
            c.push(m,0x8u32);
            c.push(m, 0x62593d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x74),32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x78),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(6,32,c.r[5].wrapping_add(0x2));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x12c),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x62597a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5fff40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x625983);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x625b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[5],16));
            c.set(0,16,rm(m,c.r[6],16));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.r[0].wrapping_add(c.r[7]).wrapping_add(0x164); wm(m,a,8,0x1u32); }
            pc = 0x62599f; continue;
            }
            0x62599d => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x62599f; continue;
            }
            0x62599f => {
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.r[4].wrapping_add(0x30));
            pc = 0x6259ac; continue;
            }
            0x6259ac => {
            c.set(3,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x4u32); }
            pc = 0x6259b6; continue;
            }
            0x6259b6 => {
            c.set(6,32,rm(m,c.r[3].wrapping_add(0xfffffffc),32));
            c.set(7,32,rm(m,c.r[3],32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[5],16));
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x2),16));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x625a4c; continue; }
            pc = 0x6259cf; continue;
            }
            0x6259cf => {
            { let a = c.get(6,32); c.sub(a,0x40u32,32); }
            if c.cond(13) { pc = 0x625a4c; continue; }
            pc = 0x6259d4; continue;
            }
            0x6259d4 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x625a4c; continue; }
            pc = 0x6259d8; continue;
            }
            0x6259d8 => {
            { let a = c.get(7,32); c.sub(a,0x40u32,32); }
            if c.cond(13) { pc = 0x625a4c; continue; }
            pc = 0x6259dd; continue;
            }
            0x6259dd => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x74),8));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x164),8));
            { let a = c.get(0,8); let r = c.add(a,c.get(2,8),8); c.set(0,8,r); }
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); let r = c.add(a,0x2u32,8); c.set(0,8,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x164));
            { let a = rm(m,c.r[1],8); c.sub(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x625a4c; continue; }
            pc = 0x625a07; continue;
            }
            0x625a07 => {
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x70),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x24),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x625a26; continue; }
            pc = 0x625a19; continue;
            }
            0x625a19 => {
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x625a3a; continue;
            }
            0x625a26 => {
            c.push(m,0x8u32);
            c.push(m, 0x625a2d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(c.r[2]).wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(2,32));
            pc = 0x625a3a; continue;
            }
            0x625a3a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x24),32));
            { let a = c.r[0]; wm(m,a,16,c.get(6,16)); }
            { let a = c.r[0].wrapping_add(0x2); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x625a4c; continue;
            }
            0x625a4c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.add(a,0x8u32,32); c.set(3,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x6259b6; continue; }
            pc = 0x625a5e; continue;
            }
            0x625a5e => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x74),8));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x70),32));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(1,8); c.sub(a,0x2u32,8); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(6,32)); }
            c.set(0,32,c.get(3,32));
            if c.cond(2) { pc = 0x6259ac; continue; }
            pc = 0x625a7e; continue;
            }
            0x625a7e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.r[5].wrapping_add(0x4));
            c.set(1,32,c.get(5,32));
            c.set(5,32,rm(m,c.r[0],32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x62599d; continue; }
            pc = 0x625a9b; continue;
            }
            0x625a9b => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x625a9d; continue;
            }
            0x625a9d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            if c.cond(5) { pc = 0x62599d; continue; }
            pc = 0x625ac5; continue;
            }
            0x625ac5 => {
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x3u32,8); }
            if c.cond(2) { pc = 0x625a9d; continue; }
            pc = 0x625acb; continue;
            }
            0x625acb => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x625ae7; continue; }
            pc = 0x625acf; continue;
            }
            0x625acf => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x625ae0);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x625acf; continue; }
            pc = 0x625ae7; continue;
            }
            0x625ae7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x5cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00625b00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x625b00;
    loop {
        match pc {
            0x625b00 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,16,rm(m,c.r[0],16));
            c.push(m,c.get(6,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            pc = 0x625b1e; continue;
            }
            0x625b1e => {
            c.set(0,32,c.r[1].wrapping_add(c.r[3]));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x625b51; continue; }
            pc = 0x625b25; continue;
            }
            0x625b25 => {
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(13) { pc = 0x625b51; continue; }
            pc = 0x625b2a; continue;
            }
            0x625b2a => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[5],16));
            c.set(7,32,c.r[2].wrapping_add(c.r[6]));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x625b51; continue; }
            pc = 0x625b37; continue;
            }
            0x625b37 => {
            { let a = c.get(7,32); c.sub(a,0x40u32,32); }
            if c.cond(13) { pc = 0x625b51; continue; }
            pc = 0x625b3c; continue;
            }
            0x625b3c => {
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x164),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x625b7a; continue; }
            pc = 0x625b51; continue;
            }
            0x625b51 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x625b60; continue; }
            pc = 0x625b55; continue;
            }
            0x625b55 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(15) { pc = 0x625b72; continue; }
            pc = 0x625b59; continue;
            }
            0x625b59 => {
            c.set(3,32,0x1u32);
            pc = 0x625b1e; continue;
            }
            0x625b60 => {
            if c.cond(14) { pc = 0x625b6e; continue; }
            pc = 0x625b62; continue;
            }
            0x625b62 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x625b6a; continue; }
            pc = 0x625b66; continue;
            }
            0x625b66 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x625b1e; continue;
            }
            0x625b6a => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x625b1e; continue;
            }
            0x625b6e => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x625b76; continue; }
            pc = 0x625b72; continue;
            }
            0x625b72 => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            pc = 0x625b1e; continue;
            }
            0x625b76 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            pc = 0x625b1e; continue;
            }
            0x625b7a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,16,rm(m,c.r[0],16));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[1]; wm(m,a,16,c.get(0,16)); }
            { let a = rm(m,c.r[5],16); let r = c.add(a,c.get(6,16),16); { let a = c.r[5]; wm(m,a,16,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00612d20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x612d20;
    loop {
        match pc {
            0x612d20 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x4),8));
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.push(m,c.get(3,32));
            if c.cond(5) { pc = 0x612d35; continue; }
            pc = 0x612d2c; continue;
            }
            0x612d2c => {
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x612d9f; continue; }
            pc = 0x612d35; continue;
            }
            0x612d35 => {
            c.set(3,8,rm(m,c.r[1].wrapping_add(0x404),8));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            if c.cond(6) { pc = 0x612d68; continue; }
            pc = 0x612d45; continue;
            }
            0x612d45 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x612d5e; continue; }
            pc = 0x612d55; continue;
            }
            0x612d55 => {
            { let a = rm(m,c.r[0].wrapping_add(0xac8),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x612d9f; continue; }
            pc = 0x612d5e; continue;
            }
            0x612d5e => {
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            if c.cond(2) { pc = 0x612d45; continue; }
            pc = 0x612d68; continue;
            }
            0x612d68 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x612d71);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0xc8u32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x612d9f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x612d9f; continue;
            }
            0x612d9f => {
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0057b640(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57b640;
    loop {
        match pc {
            0x57b640 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            if c.cond(4) { pc = 0x57b660; continue; }
            pc = 0x57b653; continue;
            }
            0x57b653 => {
            c.set(1,32,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x2d); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x35); wm(m,a,8,c.get(1,8)); }
            pc = 0x57b6a5; continue;
            }
            0x57b660 => {
            c.set(2,32,rm(m,0x6e51acu32,32));
            { let a = rm(m,c.r[2].wrapping_add(0xc594),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x57b682; continue; }
            pc = 0x57b66e; continue;
            }
            0x57b66e => {
            c.set(7,32,rm(m,0x6e2e80u32,32));
            c.push(m, 0x57b679);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(7,32); c.div32(b); }
            { let a = c.r[6].wrapping_add(0x35); wm(m,a,8,c.get(2,8)); }
            pc = 0x57b685; continue;
            }
            0x57b682 => {
            { let a = c.r[6].wrapping_add(0x35); wm(m,a,8,c.get(3,8)); }
            pc = 0x57b685; continue;
            }
            0x57b685 => {
            c.set(1,32,rm(m,0x6e2e88u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x35),8));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[1]));
            c.push(m, 0x57b69b);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = rm(m,c.r[7].wrapping_add(0x4),32); c.div32(b); }
            { let a = c.get(2,8); let r = c.add(a,rm(m,c.r[7],8),8); c.set(2,8,r); }
            { let a = c.r[6].wrapping_add(0x2d); wm(m,a,8,c.get(2,8)); }
            pc = 0x57b6a5; continue;
            }
            0x57b6a5 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,rm(m,0x6e2e88u32,32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x35),8));
            c.set(7,32,c.get(2,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(5,32,c.r[0].wrapping_add(c.r[1]));
            c.push(m, 0x57b6bf);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x8),8));
            { let b = rm(m,c.r[5].wrapping_add(0xc),32); c.div32(b); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(2,8); let r = c.add(a,c.get(1,8),8); c.set(2,8,r); }
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,0x6e2e80u32,32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x57b6dc; continue; }
            pc = 0x57b6d8; continue;
            }
            0x57b6d8 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x57b6e5; continue;
            }
            0x57b6dc => {
            c.set(2,32,rm(m,0x6e2e84u32,32));
            c.set(7,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(4)),32));
            pc = 0x57b6e5; continue;
            }
            0x57b6e5 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x57b6ef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x557730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x57b6fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x559cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x40),16));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x80u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(5,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2c),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xa));
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57b767);
            { let args = []; c.r[0] = host.call(m, 0x559260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x18),32));
            c.set(3,8,c.get(0,8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[5].wrapping_mul(4)),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xa));
            c.push(m,c.get(2,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = rm(m,c.r[0],16); let r = c.add(a,c.get(1,16),16); { let a = c.r[0]; wm(m,a,16,r); } }
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x57b796);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x57b7aa; continue; }
            pc = 0x57b79a; continue;
            }
            0x57b79a => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m, 0x57b7aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x57b7aa; continue;
            }
            0x57b7aa => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x57b7b5);
            { let args = []; c.r[0] = host.call(m, 0x559810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0057a4a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x57a4a0;
    loop {
        match pc {
            0x57a4a0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(0xe),16),16)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x57a4cf; continue; }
            pc = 0x57a4b8; continue;
            }
            0x57a4b8 => {
            c.set(2,32,sx(rm(m,c.r[6].wrapping_add(0x12),16),16)&0xffffffffu32);
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x57a4cf; continue; }
            pc = 0x57a4c0; continue;
            }
            0x57a4c0 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x14),8));
            { let a = c.get(0,8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x57a639; continue; }
            pc = 0x57a4cf; continue;
            }
            0x57a4cf => {
            c.set(2,32,rm(m,0x6e2e74u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[2]),8));
            c.set(2,32,rm(m,0x6e51acu32,32));
            { let a = rm(m,c.r[2].wrapping_add(0xc594),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x57a512; continue; }
            pc = 0x57a4ec; continue;
            }
            0x57a4ec => {
            { let a = c.get(3,8); c.sub(a,0x40u32,8); }
            if c.cond(2) { pc = 0x57a512; continue; }
            pc = 0x57a4f1; continue;
            }
            0x57a4f1 => {
            { let a = c.get(3,8); c.sub(a,0x43u32,8); }
            if c.cond(7) { pc = 0x57a512; continue; }
            pc = 0x57a4f6; continue;
            }
            0x57a4f6 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(5,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x57a512 => {
            c.set(1,32,rm(m,0x6e2e6cu32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x57a521; continue; }
            pc = 0x57a51d; continue;
            }
            0x57a51d => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x57a52a; continue;
            }
            0x57a521 => {
            c.set(1,32,rm(m,0x6e2e70u32,32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            pc = 0x57a52a; continue;
            }
            0x57a52a => {
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x57a53a);
            { let args = []; c.r[0] = host.call(m, 0x559130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,0x1u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x57a547);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x57af60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,c.get(3,8));
            { let a = c.get(0,8); let r = c.and(a,0x40u32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x57a561; continue; }
            pc = 0x57a551; continue;
            }
            0x57a551 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x57a55c);
            { let args = []; c.r[0] = host.call(m, 0x5596c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x57a584; continue;
            }
            0x57a561 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x57a571);
            { let args = []; c.r[0] = host.call(m, 0x559370, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m, 0x57a584);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x57a584; continue;
            }
            0x57a584 => {
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x57a58e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x57af60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(5,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.sub(a,0x24u32,8); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x57a5ca; continue; }
            pc = 0x57a5a3; continue;
            }
            0x57a5a3 => {
            { let a = c.get(3,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x57a5ca; continue; }
            pc = 0x57a5a8; continue;
            }
            0x57a5a8 => {
            { let a = c.get(3,8); c.sub(a,0x11u32,8); }
            if c.cond(4) { pc = 0x57a5ca; continue; }
            pc = 0x57a5ad; continue;
            }
            0x57a5ad => {
            c.set(2,32,rm(m,0x6e2e74u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2c),8));
            { let a = rm(m,c.r[1].wrapping_add(c.r[2]),8); c.sub(a,0xb0u32,8); }
            if c.cond(4) { pc = 0x57a5ca; continue; }
            pc = 0x57a5be; continue;
            }
            0x57a5be => {
            { let a = c.get(3,8); c.sub(a,0x81u32,8); }
            if c.cond(5) { pc = 0x57a5d2; continue; }
            pc = 0x57a5c3; continue;
            }
            0x57a5c3 => {
            { let a = c.get(0,32); let r = c.add(a,0xb0000u32,32); c.set(0,32,r); }
            pc = 0x57a5cf; continue;
            }
            0x57a5ca => {
            { let a = c.get(0,32); let r = c.add(a,0x300000u32,32); c.set(0,32,r); }
            pc = 0x57a5cf; continue;
            }
            0x57a5cf => {
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x57a5d2; continue;
            }
            0x57a5d2 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            c.push(m,0x1u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x57a5e3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x57af60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x28),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x57a5fb; continue; }
            pc = 0x57a5eb; continue;
            }
            0x57a5eb => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x57a5f6);
            { let args = []; c.r[0] = host.call(m, 0x559810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            pc = 0x57a61e; continue;
            }
            0x57a5fb => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x57a60b);
            { let args = []; c.r[0] = host.call(m, 0x559370, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x57a61e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x59b5b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x57a61e; continue;
            }
            0x57a61e => {
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x57a628);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x57af60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m, 0x57a635);
            { let args = []; c.r[0] = host.call(m, 0x559130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x57a639; continue;
            }
            0x57a639 => {
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

pub fn f_005687b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5687b0;
    loop {
        match pc {
            0x5687b0 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x6e17a0u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5687be);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x559cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x18),16));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(7,32,c.r[6].wrapping_add(0x8));
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,16,c.get(1,16)); }
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.push(m,c.get(7,32));
            c.push(m,0x6e17a0u32);
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            c.push(m, 0x5687f3);
            { let args = []; c.r[0] = host.call(m, 0x559810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6e17b8u32,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x20),16));
            c.push(m,c.get(7,32));
            c.push(m,0x6e17a0u32);
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,0x0u32);
            { let a = rm(m,c.r[0].wrapping_add(0xa),16); let r = c.sub(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(0xa); wm(m,a,16,r); } }
            c.push(m, 0x568810);
            { let args = []; c.r[0] = host.call(m, 0x559130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6e17b8u32,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x20),16));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = rm(m,c.r[0].wrapping_add(0xa),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[0].wrapping_add(0xa); wm(m,a,16,r); } }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
