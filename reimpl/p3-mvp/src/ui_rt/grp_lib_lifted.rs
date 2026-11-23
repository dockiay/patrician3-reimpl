//! Generado por `reimpl/tools/ui_rt_grp.py lib` con `tools/lift.py` (no editar a mano): grupo `lib`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_LIB: [(u32, super::core_table::CoreFn); 86] = [
    (0x6392b9, f_006392b9),
    (0x63958c, f_0063958c),
    (0x639c4f, f_00639c4f),
    (0x639f9a, f_00639f9a),
    (0x63b37b, f_0063b37b),
    (0x63bd1a, f_0063bd1a),
    (0x63bdbe, f_0063bdbe),
    (0x63be3b, f_0063be3b),
    (0x63bf96, f_0063bf96),
    (0x63cb53, f_0063cb53),
    (0x63cd3a, f_0063cd3a),
    (0x63cddf, f_0063cddf),
    (0x63d222, f_0063d222),
    (0x63d776, f_0063d776),
    (0x63e4f5, f_0063e4f5),
    (0x640232, f_00640232),
    (0x640586, f_00640586),
    (0x640a3b, f_00640a3b),
    (0x640f8d, f_00640f8d),
    (0x640fe4, f_00640fe4),
    (0x641029, f_00641029),
    (0x641355, f_00641355),
    (0x64149d, f_0064149d),
    (0x643ab5, f_00643ab5),
    (0x646137, f_00646137),
    (0x646eac, f_00646eac),
    (0x64ca70, f_0064ca70),
    (0x64d030, f_0064d030),
    (0x64d910, f_0064d910),
    (0x64e566, f_0064e566),
    (0x64e662, f_0064e662),
    (0x64e7fc, f_0064e7fc),
    (0x64e9d1, f_0064e9d1),
    (0x64eb28, f_0064eb28),
    (0x64f637, f_0064f637),
    (0x64f686, f_0064f686),
    (0x64f6ae, f_0064f6ae),
    (0x64f71b, f_0064f71b),
    (0x64f967, f_0064f967),
    (0x64fbd4, f_0064fbd4),
    (0x64fde9, f_0064fde9),
    (0x64ff0f, f_0064ff0f),
    (0x64ff74, f_0064ff74),
    (0x650519, f_00650519),
    (0x650acb, f_00650acb),
    (0x650bfd, f_00650bfd),
    (0x650dba, f_00650dba),
    (0x651167, f_00651167),
    (0x651b5b, f_00651b5b),
    (0x65248b, f_0065248b),
    (0x65366f, f_0065366f),
    (0x653705, f_00653705),
    (0x653743, f_00653743),
    (0x653d3a, f_00653d3a),
    (0x653f17, f_00653f17),
    (0x653f67, f_00653f67),
    (0x654296, f_00654296),
    (0x654c58, f_00654c58),
    (0x655365, f_00655365),
    (0x655cd4, f_00655cd4),
    (0x655d09, f_00655d09),
    (0x655d7b, f_00655d7b),
    (0x656860, f_00656860),
    (0x65688f, f_0065688f),
    (0x6568be, f_006568be),
    (0x656910, f_00656910),
    (0x656962, f_00656962),
    (0x6569f3, f_006569f3),
    (0x656ca5, f_00656ca5),
    (0x656fa3, f_00656fa3),
    (0x656fdb, f_00656fdb),
    (0x6570b4, f_006570b4),
    (0x6571ce, f_006571ce),
    (0x6572ac, f_006572ac),
    (0x659ddb, f_00659ddb),
    (0x659f25, f_00659f25),
    (0x65a5cf, f_0065a5cf),
    (0x65a691, f_0065a691),
    (0x652dd2, f_00652dd2),
    (0x6599a7, f_006599a7),
    (0x650609, f_00650609),
    (0x64f7a7, f_0064f7a7),
    (0x659cac, f_00659cac),
    (0x659d0c, f_00659d0c),
    (0x659754, f_00659754),
    (0x65ad40, f_0065ad40),
];
pub fn f_006392b9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6392b9;
    loop {
        match pc {
            0x6392b9 => {
            c.set(0,32,0x668bd3u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x6392c3; }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe8),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,r); } }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,c.r[5].wrapping_add(0xfffffff0));
            c.set(0,32,c.r[6].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m, 0x6392da);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff0),32));
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,0x5cu32,8); }
            if c.cond(4) { pc = 0x639300; continue; }
            pc = 0x6392ed; continue;
            }
            0x6392ed => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffffffff),8); c.sub(a,0x2fu32,8); }
            if c.cond(4) { pc = 0x639300; continue; }
            pc = 0x6392f4; continue;
            }
            0x6392f4 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x18),8));
            c.set(1,32,c.r[5].wrapping_add(0xfffffff0));
            c.push(m,c.get(0,32));
            c.push(m, 0x639300);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f60a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x639300; continue;
            }
            0x639300 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc),32); c.push(m, 0x63930b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[5].wrapping_add(0xfffffff0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            c.push(m, 0x639318);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f61f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x639323);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffff0));
            c.push(m,c.get(0,32));
            c.push(m, 0x63932f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(3,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); let r = c.and(a,0x0u32,8); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,r); } }
            c.set(1,32,c.r[5].wrapping_add(0xfffffff0));
            c.push(m, 0x63933e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063958c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63958c;
    loop {
        match pc {
            0x63958c => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x6395b3; continue; }
            pc = 0x639598; continue;
            }
            0x639598 => {
            c.push(m,c.get(3,32));
            c.set(3,32,c.r[6].wrapping_add(0x10));
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a1b0u32,32); c.push(m, 0x6395a3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,0x66a1acu32,32); c.push(m, 0x6395b2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x6395b3; continue;
            }
            0x6395b3 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00639c4f(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x639c4f;
    loop {
        match pc {
            0x639c4f => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x639c57; continue;
            }
            0x639c57 => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x639c6f; continue; }
            pc = 0x639c60; continue;
            }
            0x639c60 => {
            c.set(0,32,rm(m,c.r[7],8));
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x639c6b);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x639c7e; continue;
            }
            0x639c6f => {
            c.set(0,32,rm(m,c.r[7],8));
            c.set(1,32,rm(m,0x6c8760u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x639c7e; continue;
            }
            0x639c7e => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x639c85; continue; }
            pc = 0x639c82; continue;
            }
            0x639c82 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x639c57; continue;
            }
            0x639c85 => {
            c.set(6,32,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); c.sub(a,0x2du32,32); }
            c.set(5,32,c.get(6,32));
            if c.cond(4) { pc = 0x639c95; continue; }
            pc = 0x639c90; continue;
            }
            0x639c90 => {
            { let a = c.get(6,32); c.sub(a,0x2bu32,32); }
            if c.cond(5) { pc = 0x639c99; continue; }
            pc = 0x639c95; continue;
            }
            0x639c95 => {
            c.set(6,32,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x639c99; continue;
            }
            0x639c99 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x639c9b; continue;
            }
            0x639c9b => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x639cb0; continue; }
            pc = 0x639ca4; continue;
            }
            0x639ca4 => {
            c.push(m,0x4u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x639cac);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x639cbb; continue;
            }
            0x639cb0 => {
            c.set(0,32,rm(m,0x6c8760u32,32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,0x4u32,32); c.set(0,32,r); }
            pc = 0x639cbb; continue;
            }
            0x639cbb => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x639ccc; continue; }
            pc = 0x639cbf; continue;
            }
            0x639cbf => {
            c.set(0,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            c.set(3,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0xffffffd0));
            c.set(6,32,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x639c9b; continue;
            }
            0x639ccc => {
            { let a = c.get(5,32); c.sub(a,0x2du32,32); }
            c.set(0,32,c.get(3,32));
            if c.cond(5) { pc = 0x639cd5; continue; }
            pc = 0x639cd3; continue;
            }
            0x639cd3 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x639cd5; continue;
            }
            0x639cd5 => {
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

pub fn f_00639f9a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x639f9a;
    loop {
        match pc {
            0x639f9a => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(3,8,rm(m,c.r[7],8));
            c.set(6,32,c.r[7].wrapping_add(0x1));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(6,32)); }
            pc = 0x639fb2; continue;
            }
            0x639fb2 => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x639fca; continue; }
            pc = 0x639fbb; continue;
            }
            0x639fbb => {
            c.set(0,32,c.get(3,8));
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x639fc6);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x639fd9; continue;
            }
            0x639fca => {
            c.set(1,32,rm(m,0x6c8760u32,32));
            c.set(0,32,c.get(3,8));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x639fd9; continue;
            }
            0x639fd9 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x639fe2; continue; }
            pc = 0x639fdd; continue;
            }
            0x639fdd => {
            c.set(3,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x639fb2; continue;
            }
            0x639fe2 => {
            { let a = c.get(3,8); c.sub(a,0x2du32,8); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(6,32)); }
            if c.cond(5) { pc = 0x639ff0; continue; }
            pc = 0x639fea; continue;
            }
            0x639fea => {
            { let a = rm(m,c.r[5].wrapping_add(0x14),32); let r = c.or(a,0x2u32,32); { let a = c.r[5].wrapping_add(0x14); wm(m,a,32,r); } }
            pc = 0x639ff5; continue;
            }
            0x639ff0 => {
            { let a = c.get(3,8); c.sub(a,0x2bu32,8); }
            if c.cond(5) { pc = 0x639ffb; continue; }
            pc = 0x639ff5; continue;
            }
            0x639ff5 => {
            c.set(3,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(6,32)); }
            pc = 0x639ffb; continue;
            }
            0x639ffb => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x63a18f; continue; }
            pc = 0x63a006; continue;
            }
            0x63a006 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x63a18f; continue; }
            pc = 0x63a00f; continue;
            }
            0x63a00f => {
            { let a = c.get(0,32); c.sub(a,0x24u32,32); }
            if c.cond(15) { pc = 0x63a18f; continue; }
            pc = 0x63a018; continue;
            }
            0x63a018 => {
            c.push(m,0x10u32);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x63a043; continue; }
            pc = 0x63a01f; continue;
            }
            0x63a01f => {
            { let a = c.get(3,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x63a02d; continue; }
            pc = 0x63a024; continue;
            }
            0x63a024 => {
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,0xau32); }
            pc = 0x63a05f; continue;
            }
            0x63a02d => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,0x78u32,8); }
            if c.cond(4) { pc = 0x63a040; continue; }
            pc = 0x63a033; continue;
            }
            0x63a033 => {
            { let a = c.get(0,8); c.sub(a,0x58u32,8); }
            if c.cond(4) { pc = 0x63a040; continue; }
            pc = 0x63a037; continue;
            }
            0x63a037 => {
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,0x8u32); }
            pc = 0x63a05f; continue;
            }
            0x63a040 => {
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x63a043; continue;
            }
            0x63a043 => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x63a05f; continue; }
            pc = 0x63a048; continue;
            }
            0x63a048 => {
            { let a = c.get(3,8); c.sub(a,0x30u32,8); }
            if c.cond(5) { pc = 0x63a05f; continue; }
            pc = 0x63a04d; continue;
            }
            0x63a04d => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,0x78u32,8); }
            if c.cond(4) { pc = 0x63a057; continue; }
            pc = 0x63a053; continue;
            }
            0x63a053 => {
            { let a = c.get(0,8); c.sub(a,0x58u32,8); }
            if c.cond(5) { pc = 0x63a05f; continue; }
            pc = 0x63a057; continue;
            }
            0x63a057 => {
            c.set(3,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(6,32)); }
            pc = 0x63a05f; continue;
            }
            0x63a05f => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = rm(m,c.r[5].wrapping_add(0x10),32); c.div32(b); }
            c.set(7,32,0x103u32);
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(0,32)); }
            pc = 0x63a06f; continue;
            }
            0x63a06f => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,0x1u32,32); }
            c.set(6,32,c.get(3,8));
            if c.cond(14) { pc = 0x63a087; continue; }
            pc = 0x63a07b; continue;
            }
            0x63a07b => {
            c.push(m,0x4u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x63a083);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63a092; continue;
            }
            0x63a087 => {
            c.set(0,32,rm(m,0x6c8760u32,32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,0x4u32,32); c.set(0,32,r); }
            pc = 0x63a092; continue;
            }
            0x63a092 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63a09e; continue; }
            pc = 0x63a096; continue;
            }
            0x63a096 => {
            c.set(1,32,sx(c.get(3,8),8)&0xffffffffu32);
            { let a = c.get(1,32); let r = c.sub(a,0x30u32,32); c.set(1,32,r); }
            pc = 0x63a0d0; continue;
            }
            0x63a09e => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x63a0b2; continue; }
            pc = 0x63a0a7; continue;
            }
            0x63a0a7 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x63a0ae);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63a0bd; continue;
            }
            0x63a0b2 => {
            c.set(0,32,rm(m,0x6c8760u32,32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(2)),16));
            { let a = c.get(0,32); let r = c.and(a,c.get(7,32),32); c.set(0,32,r); }
            pc = 0x63a0bd; continue;
            }
            0x63a0bd => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63a10b; continue; }
            pc = 0x63a0c1; continue;
            }
            0x63a0c1 => {
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x63a0ca);
            { let args = []; c.r[0] = host.call(m, 0x63c7db, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,0x37u32,32); c.set(1,32,r); }
            pc = 0x63a0d0; continue;
            }
            0x63a0d0 => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x10),32),32); }
            if c.cond(3) { pc = 0x63a10b; continue; }
            pc = 0x63a0d5; continue;
            }
            0x63a0d5 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = rm(m,c.r[5].wrapping_add(0x14),32); let r = c.or(a,0x8u32,32); { let a = c.r[5].wrapping_add(0x14); wm(m,a,32,r); } }
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffff4),32),32); }
            if c.cond(2) { pc = 0x63a0f5; continue; }
            pc = 0x63a0e1; continue;
            }
            0x63a0e1 => {
            if c.cond(5) { pc = 0x63a0ef; continue; }
            pc = 0x63a0e3; continue;
            }
            0x63a0e3 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = rm(m,c.r[5].wrapping_add(0x10),32); c.div32(b); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x63a0f5; continue; }
            pc = 0x63a0ef; continue;
            }
            0x63a0ef => {
            { let a = rm(m,c.r[5].wrapping_add(0x14),32); let r = c.or(a,0x4u32,32); { let a = c.r[5].wrapping_add(0x14); wm(m,a,32,r); } }
            pc = 0x63a0fe; continue;
            }
            0x63a0f5 => {
            { let a = c.get(6,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0x10),32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(6,32)); }
            pc = 0x63a0fe; continue;
            }
            0x63a0fe => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(3,8,rm(m,c.r[0],8));
            pc = 0x63a06f; continue;
            }
            0x63a10b => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,8); c.and(a,0x8u32,8); }
            if c.cond(5) { pc = 0x63a128; continue; }
            pc = 0x63a118; continue;
            }
            0x63a118 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x63a122; continue; }
            pc = 0x63a11c; continue;
            }
            0x63a11c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            pc = 0x63a122; continue;
            }
            0x63a122 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            pc = 0x63a173; continue;
            }
            0x63a128 => {
            { let a = c.get(0,8); c.and(a,0x4u32,8); }
            c.set(6,32,0x7fffffffu32);
            if c.cond(5) { pc = 0x63a14c; continue; }
            pc = 0x63a131; continue;
            }
            0x63a131 => {
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x63a173; continue; }
            pc = 0x63a135; continue;
            }
            0x63a135 => {
            { let a = c.get(0,32); let r = c.and(a,0x2u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63a143; continue; }
            pc = 0x63a13a; continue;
            }
            0x63a13a => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.sub(a,0x80000000u32,32); }
            if c.cond(7) { pc = 0x63a14c; continue; }
            pc = 0x63a143; continue;
            }
            0x63a143 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x63a173; continue; }
            pc = 0x63a147; continue;
            }
            0x63a147 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.sub(a,c.get(6,32),32); }
            if c.cond(6) { pc = 0x63a173; continue; }
            pc = 0x63a14c; continue;
            }
            0x63a14c => {
            c.push(m, 0x63a151);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0x14),8); c.and(a,0x1u32,8); }
            { let a = c.r[0]; wm(m,a,32,0x22u32); }
            if c.cond(4) { pc = 0x63a163; continue; }
            pc = 0x63a15d; continue;
            }
            0x63a15d => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            pc = 0x63a173; continue;
            }
            0x63a163 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(0,8); let r = c.and(a,0x2u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.neg(a,8); c.set(0,8,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            pc = 0x63a173; continue;
            }
            0x63a173 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x63a17c; continue; }
            pc = 0x63a177; continue;
            }
            0x63a177 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            pc = 0x63a17c; continue;
            }
            0x63a17c => {
            { let a = rm(m,c.r[5].wrapping_add(0x14),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x63a18a; continue; }
            pc = 0x63a182; continue;
            }
            0x63a182 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            pc = 0x63a18a; continue;
            }
            0x63a18a => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            pc = 0x63a19a; continue;
            }
            0x63a18f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63a198; continue; }
            pc = 0x63a196; continue;
            }
            0x63a196 => {
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            pc = 0x63a198; continue;
            }
            0x63a198 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63a19a; continue;
            }
            0x63a19a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063b37b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63b37b;
    loop {
        match pc {
            0x63b37b => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x63b385; continue;
            }
            0x63b385 => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x63b39d; continue; }
            pc = 0x63b38e; continue;
            }
            0x63b38e => {
            c.set(0,32,rm(m,c.r[6],8));
            c.push(m,0x8u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x63b399);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63b3ac; continue;
            }
            0x63b39d => {
            c.set(0,32,rm(m,c.r[6],8));
            c.set(1,32,rm(m,0x6c8760u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x63b3ac; continue;
            }
            0x63b3ac => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63b3b3; continue; }
            pc = 0x63b3b0; continue;
            }
            0x63b3b0 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x63b385; continue;
            }
            0x63b3b3 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x63b3bd);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x63b3c9);
            { let args = []; c.r[0] = host.call(m, 0x642a2e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fld(c.ld(m,c.r[0].wrapping_add(0x10),8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063bd1a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63bd1a;
    loop {
        match pc {
            0x63bd1a => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.push(m, 0x63bd2b);
            { let args = []; c.r[0] = host.call(m, 0x63edb3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x8u32);
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x7u32);
            c.stos(m,32,true);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x63bd3b; continue;
            }
            0x63bd3b => {
            c.set(2,8,rm(m,c.r[6],8));
            c.set(3,8,0x1u32);
            c.set(1,32,c.get(2,8));
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.and(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,8); let n = c.get(1,8); let r = c.shl(a,n,8); c.set(3,8,r); }
            c.set(0,32,c.r[5].wrapping_add(c.r[0]).wrapping_add(0xffffffe0));
            { let a = rm(m,c.r[0],8); let r = c.or(a,c.get(3,8),8); { let a = c.r[0]; wm(m,a,8,r); } }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x63bd3b; continue; }
            pc = 0x63bd56; continue;
            }
            0x63bd56 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x63bd63; continue; }
            pc = 0x63bd5d; continue;
            }
            0x63bd5d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x18),32));
            pc = 0x63bd63; continue;
            }
            0x63bd63 => {
            c.set(0,8,rm(m,c.r[2],8));
            c.push(m,0x1u32);
            c.set(6,32,c.get(0,8));
            c.set(1,32,c.get(6,32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(1,32); let r = c.and(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(6,32); let n = 0x3u32; let r = c.shr(a,n,32); c.set(6,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(c.r[6]).wrapping_add(0xffffffe0),8));
            { let a = c.get(1,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x63bd83; continue; }
            pc = 0x63bd7c; continue;
            }
            0x63bd7c => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x63bd83; continue; }
            pc = 0x63bd80; continue;
            }
            0x63bd80 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x63bd63; continue;
            }
            0x63bd83 => {
            c.set(3,32,c.get(2,32));
            pc = 0x63bd85; continue;
            }
            0x63bd85 => {
            c.set(0,8,rm(m,c.r[2],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x63bda9; continue; }
            pc = 0x63bd8b; continue;
            }
            0x63bd8b => {
            c.set(6,32,c.get(0,8));
            c.set(1,32,c.get(6,32));
            c.push(m,0x1u32);
            { let a = c.get(1,32); let r = c.and(a,c.get(7,32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(0,32,v); }
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = 0x3u32; let r = c.shr(a,n,32); c.set(6,32,r); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(c.r[6]).wrapping_add(0xffffffe0),8));
            { let a = c.get(1,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x63bda5; continue; }
            pc = 0x63bda2; continue;
            }
            0x63bda2 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x63bd85; continue;
            }
            0x63bda5 => {
            { let a = rm(m,c.r[2],8); let r = c.and(a,0x0u32,8); { let a = c.r[2]; wm(m,a,8,r); } }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            pc = 0x63bda9; continue;
            }
            0x63bda9 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,c.get(3,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063bdbe(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63bdbe;
    loop {
        match pc {
            0x63bdbe => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x63bdd3; continue; }
            pc = 0x63bdcd; continue;
            }
            0x63bdcd => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x63bdea; continue;
            }
            0x63bdd3 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x63bdd9);
            { let args = []; c.r[0] = host.call(m, 0x643554, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x63bddf);
            { let args = []; c.r[0] = host.call(m, 0x63bdef, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x63bde7);
            { let args = []; c.r[0] = host.call(m, 0x6435a6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x63bdea; continue;
            }
            0x63bdea => {
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063be3b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63be3b;
    loop {
        match pc {
            0x63be3b => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,0x0u32,32); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(3,32));
            if c.cond(15) { pc = 0x63be4f; continue; }
            pc = 0x63be4b; continue;
            }
            0x63be4b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63be99; continue;
            }
            0x63be4f => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            c.push(m, 0x63be59);
            { let args = []; c.r[0] = host.call(m, 0x643554, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63be5a; continue;
            }
            0x63be5a => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,r); } }
            if c.cond(4) { pc = 0x63be8c; continue; }
            pc = 0x63be5f; continue;
            }
            0x63be5f => {
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.dec(a,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            if c.cond(8) { pc = 0x63be6e; continue; }
            pc = 0x63be64; continue;
            }
            0x63be64 => {
            c.set(1,32,rm(m,c.r[6],32));
            c.set(0,32,rm(m,c.r[1],8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            pc = 0x63be75; continue;
            }
            0x63be6e => {
            c.push(m,c.get(6,32));
            c.push(m, 0x63be74);
            { let args = []; c.r[0] = host.call(m, 0x643869, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63be75; continue;
            }
            0x63be75 => {
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x63be83; continue; }
            pc = 0x63be7a; continue;
            }
            0x63be7a => {
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,8); c.sub(a,0xau32,8); }
            if c.cond(4) { pc = 0x63be8c; continue; }
            pc = 0x63be81; continue;
            }
            0x63be81 => {
            pc = 0x63be5a; continue;
            }
            0x63be83 => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x63be8c; continue; }
            pc = 0x63be88; continue;
            }
            0x63be88 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x63be8f; continue;
            }
            0x63be8c => {
            { let a = rm(m,c.r[7],8); let r = c.and(a,0x0u32,8); { let a = c.r[7]; wm(m,a,8,r); } }
            pc = 0x63be8f; continue;
            }
            0x63be8f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x63be95);
            { let args = []; c.r[0] = host.call(m, 0x6435a6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x63be99; continue;
            }
            0x63be99 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063bf96(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63bf96;
    loop {
        match pc {
            0x63bf96 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            { let a = rm(m,0x6f0224u32,32); c.sub(a,0x0u32,32); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            if c.cond(5) { pc = 0x63bfc3; continue; }
            pc = 0x63bfa6; continue;
            }
            0x63bfa6 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,0x41u32,32); }
            if c.cond(12) { pc = 0x63c05c; continue; }
            pc = 0x63bfb2; continue;
            }
            0x63bfb2 => {
            { let a = c.get(0,32); c.sub(a,0x5au32,32); }
            if c.cond(15) { pc = 0x63c05c; continue; }
            pc = 0x63bfbb; continue;
            }
            0x63bfbb => {
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            pc = 0x63c05c; continue;
            }
            0x63bfc3 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(7,32,0x100u32);
            c.push(m,0x1u32);
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(13) { pc = 0x63bff7; continue; }
            pc = 0x63bfd2; continue;
            }
            0x63bfd2 => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x63bfe5; continue; }
            pc = 0x63bfda; continue;
            }
            0x63bfda => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x63bfe1);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63bfef; continue;
            }
            0x63bfe5 => {
            c.set(0,32,rm(m,0x6c8760u32,32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,c.get(6,32),32); c.set(0,32,r); }
            pc = 0x63bfef; continue;
            }
            0x63bfef => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x63bff7; continue; }
            pc = 0x63bff3; continue;
            }
            0x63bff3 => {
            c.set(0,32,c.get(3,32));
            pc = 0x63c05c; continue;
            }
            0x63bff7 => {
            c.set(2,32,rm(m,0x6c8760u32,32));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,8));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x1),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x63c01b; continue; }
            pc = 0x63c00c; continue;
            }
            0x63c00c => {
            { let a = rm(m,c.r[5].wrapping_add(0xa),8); let r = c.and(a,0x0u32,8); { let a = c.r[5].wrapping_add(0xa); wm(m,a,8,r); } }
            c.push(m,0x2u32);
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[5].wrapping_add(0x9); wm(m,a,8,c.get(3,8)); }
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x63c024; continue;
            }
            0x63c01b => {
            { let a = rm(m,c.r[5].wrapping_add(0x9),8); let r = c.and(a,0x0u32,8); { let a = c.r[5].wrapping_add(0x9); wm(m,a,8,r); } }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,c.get(6,32));
            pc = 0x63c024; continue;
            }
            0x63c024 => {
            c.push(m,c.get(6,32));
            c.push(m,0x0u32);
            c.set(1,32,c.r[5].wrapping_add(0xfffffffc));
            c.push(m,0x3u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let v = rm(m,0x6f0224u32,32); c.push(m, v); }
            c.push(m, 0x63c03e);
            { let args = []; c.r[0] = host.call(m, 0x643b7d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63bff3; continue; }
            pc = 0x63c045; continue;
            }
            0x63c045 => {
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x63c04f; continue; }
            pc = 0x63c049; continue;
            }
            0x63c049 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),8));
            pc = 0x63c05c; continue;
            }
            0x63c04f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffd),8));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),8));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            pc = 0x63c05c; continue;
            }
            0x63c05c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063cb53(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63cb53;
    loop {
        match pc {
            0x63cb53 => {
            { let a = rm(m,c.r[4].wrapping_add(0x4),32); c.sub(a,0xffffffe0u32,32); }
            if c.cond(7) { pc = 0x63cb7c; continue; }
            pc = 0x63cb5a; continue;
            }
            0x63cb5a => {
            { let v = rm(m,c.r[4].wrapping_add(0x4),32); c.push(m, v); }
            c.push(m, 0x63cb63);
            { let args = []; c.r[0] = host.call(m, 0x63cb7f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x63cb7e; continue; }
            pc = 0x63cb68; continue;
            }
            0x63cb68 => {
            { let a = rm(m,c.r[4].wrapping_add(0x8),32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63cb7e; continue; }
            pc = 0x63cb6e; continue;
            }
            0x63cb6e => {
            { let v = rm(m,c.r[4].wrapping_add(0x4),32); c.push(m, v); }
            c.push(m, 0x63cb77);
            { let args = []; c.r[0] = host.call(m, 0x64149d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x63cb5a; continue; }
            pc = 0x63cb7c; continue;
            }
            0x63cb7c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63cb7e; continue;
            }
            0x63cb7e => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063cd3a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63cd3a;
    loop {
        match pc {
            0x63cd3a => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x63cdce; continue; }
            pc = 0x63cd4a; continue;
            }
            0x63cd4a => {
            { let a = rm(m,c.r[3],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x63cdce; continue; }
            pc = 0x63cd4f; continue;
            }
            0x63cd4f => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x63cd7b; continue; }
            pc = 0x63cd56; continue;
            }
            0x63cd56 => {
            c.push(m,0x104u32);
            c.push(m, 0x63cd60);
            { let args = []; c.r[0] = host.call(m, 0x63cb41, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x63cd74; continue; }
            pc = 0x63cd67; continue;
            }
            0x63cd67 => {
            c.push(m, 0x63cd6c);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0xcu32); }
            pc = 0x63cdc6; continue;
            }
            0x63cd74 => {
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,0x104u32); }
            pc = 0x63cd7b; continue;
            }
            0x63cd7b => {
            c.set(0,32,c.r[5].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a170u32,32); c.push(m, 0x63cd8a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x10),32),32); }
            if c.cond(2) { pc = 0x63cda9; continue; }
            pc = 0x63cd8f; continue;
            }
            0x63cd8f => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x63cd9c; continue; }
            pc = 0x63cd95; continue;
            }
            0x63cd95 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x63cd9b);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63cd9c; continue;
            }
            0x63cd9c => {
            c.push(m, 0x63cda1);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x22u32); }
            pc = 0x63cdc6; continue;
            }
            0x63cda9 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x63cdca; continue; }
            pc = 0x63cdad; continue;
            }
            0x63cdad => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x63cdb9; continue; }
            pc = 0x63cdb2; continue;
            }
            0x63cdb2 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x63cdb8);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63cdb9; continue;
            }
            0x63cdb9 => {
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x63cdbf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x63cdc5);
            { let args = []; c.r[0] = host.call(m, 0x63ccb5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63cdc6; continue;
            }
            0x63cdc6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63cddb; continue;
            }
            0x63cdca => {
            c.set(0,32,c.get(7,32));
            pc = 0x63cddb; continue;
            }
            0x63cdce => {
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x63cdd9);
            { let args = []; c.r[0] = host.call(m, 0x644ab4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63cddb; continue;
            }
            0x63cddb => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063cddf(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63cddf;
    loop {
        match pc {
            0x63cddf => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x63cdf3);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(2) { pc = 0x63ce1a; continue; }
            pc = 0x63cdf9; continue;
            }
            0x63cdf9 => {
            { let a = rm(m,c.r[3].wrapping_add(0x1),8); c.sub(a,0x3au32,8); }
            if c.cond(5) { pc = 0x63ce1a; continue; }
            pc = 0x63cdff; continue;
            }
            0x63cdff => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x63ce16; continue; }
            pc = 0x63ce06; continue;
            }
            0x63ce06 => {
            c.push(m,0x2u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x63ce0f);
            { let args = []; c.r[0] = host.call(m, 0x644be7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x2),8); let r = c.and(a,0x0u32,8); { let a = c.r[6].wrapping_add(0x2); wm(m,a,8,r); } }
            pc = 0x63ce16; continue;
            }
            0x63ce16 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            pc = 0x63ce24; continue;
            }
            0x63ce1a => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63ce24; continue; }
            pc = 0x63ce21; continue;
            }
            0x63ce21 => {
            { let a = rm(m,c.r[0],8); let r = c.and(a,0x0u32,8); { let a = c.r[0]; wm(m,a,8,r); } }
            pc = 0x63ce24; continue;
            }
            0x63ce24 => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[3],8); c.sub(a,0x0u32,8); }
            c.set(0,32,c.get(3,32));
            c.set(6,32,0xffu32);
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x63ce9c; continue; }
            pc = 0x63ce37; continue;
            }
            0x63ce37 => {
            c.set(1,8,rm(m,c.r[0],8));
            c.set(2,32,c.get(1,8));
            { let a = rm(m,c.r[2].wrapping_add(0x6f1bc1),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x63ce48; continue; }
            pc = 0x63ce45; continue;
            }
            0x63ce45 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x63ce62; continue;
            }
            0x63ce48 => {
            { let a = c.get(1,8); c.sub(a,0x2fu32,8); }
            if c.cond(4) { pc = 0x63ce5c; continue; }
            pc = 0x63ce4d; continue;
            }
            0x63ce4d => {
            { let a = c.get(1,8); c.sub(a,0x5cu32,8); }
            if c.cond(4) { pc = 0x63ce5c; continue; }
            pc = 0x63ce52; continue;
            }
            0x63ce52 => {
            { let a = c.get(1,8); c.sub(a,0x2eu32,8); }
            if c.cond(5) { pc = 0x63ce62; continue; }
            pc = 0x63ce57; continue;
            }
            0x63ce57 => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            pc = 0x63ce62; continue;
            }
            0x63ce5c => {
            c.set(1,32,c.r[0].wrapping_add(0x1));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            pc = 0x63ce62; continue;
            }
            0x63ce62 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x63ce37; continue; }
            pc = 0x63ce68; continue;
            }
            0x63ce68 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x63ce9c; continue; }
            pc = 0x63ce72; continue;
            }
            0x63ce72 => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x63ce97; continue; }
            pc = 0x63ce78; continue;
            }
            0x63ce78 => {
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x63ce80; continue; }
            pc = 0x63ce7e; continue;
            }
            0x63ce7e => {
            c.set(7,32,c.get(6,32));
            pc = 0x63ce80; continue;
            }
            0x63ce80 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m, 0x63ce8a);
            { let args = []; c.r[0] = host.call(m, 0x644be7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[0]),8); let r = c.and(a,0x0u32,8); { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,8,r); } }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x63ce97; continue;
            }
            0x63ce97 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            pc = 0x63cea6; continue;
            }
            0x63ce9c => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x63cea6; continue; }
            pc = 0x63cea3; continue;
            }
            0x63cea3 => {
            { let a = rm(m,c.r[1],8); let r = c.and(a,0x0u32,8); { let a = c.r[1]; wm(m,a,8,r); } }
            pc = 0x63cea6; continue;
            }
            0x63cea6 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x63cef9; continue; }
            pc = 0x63cead; continue;
            }
            0x63cead => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x63cef9; continue; }
            pc = 0x63ceb1; continue;
            }
            0x63ceb1 => {
            { let a = rm(m,c.r[5].wrapping_add(0x14),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x63ced6; continue; }
            pc = 0x63ceb7; continue;
            }
            0x63ceb7 => {
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x63cebf; continue; }
            pc = 0x63cebd; continue;
            }
            0x63cebd => {
            c.set(7,32,c.get(6,32));
            pc = 0x63cebf; continue;
            }
            0x63cebf => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m, 0x63cec9);
            { let args = []; c.r[0] = host.call(m, 0x644be7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(c.r[0]),8); let r = c.and(a,0x0u32,8); { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,8,r); } }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x63ced6; continue;
            }
            0x63ced6 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x63cf21; continue; }
            pc = 0x63cedd; continue;
            }
            0x63cedd => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffffc),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x63cee6; continue; }
            pc = 0x63cee4; continue;
            }
            0x63cee4 => {
            c.set(6,32,c.get(0,32));
            pc = 0x63cee6; continue;
            }
            0x63cee6 => {
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.push(m, v); }
            c.push(m,c.get(7,32));
            c.push(m, 0x63cef0);
            { let args = []; c.r[0] = host.call(m, 0x644be7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[7]),8); let r = c.and(a,0x0u32,8); { let a = c.r[6].wrapping_add(c.r[7]); wm(m,a,8,r); } }
            pc = 0x63cf21; continue;
            }
            0x63cef9 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x63cf17; continue; }
            pc = 0x63cf00; continue;
            }
            0x63cf00 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x63cf08; continue; }
            pc = 0x63cf06; continue;
            }
            0x63cf06 => {
            c.set(6,32,c.get(0,32));
            pc = 0x63cf08; continue;
            }
            0x63cf08 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x63cf10);
            { let args = []; c.r[0] = host.call(m, 0x644be7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[7]),8); let r = c.and(a,0x0u32,8); { let a = c.r[6].wrapping_add(c.r[7]); wm(m,a,8,r); } }
            pc = 0x63cf17; continue;
            }
            0x63cf17 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63cf21; continue; }
            pc = 0x63cf1e; continue;
            }
            0x63cf1e => {
            { let a = rm(m,c.r[0],8); let r = c.and(a,0x0u32,8); { let a = c.r[0]; wm(m,a,8,r); } }
            pc = 0x63cf21; continue;
            }
            0x63cf21 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063d222(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63d222;
    loop {
        match pc {
            0x63d222 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            if c.cond(13) { pc = 0x63d235; continue; }
            pc = 0x63d22e; continue;
            }
            0x63d22e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63d347; continue;
            }
            0x63d235 => {
            c.push(m, 0x63d23a);
            { let args = []; c.r[0] = host.call(m, 0x644c81, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,0x3f480u32,32); }
            if c.cond(14) { pc = 0x63d29b; continue; }
            pc = 0x63d243; continue;
            }
            0x63d243 => {
            { let a = c.get(0,32); c.sub(a,0x7ffc0b7fu32,32); }
            if c.cond(13) { pc = 0x63d29b; continue; }
            pc = 0x63d24a; continue;
            }
            0x63d24a => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,0x6cb100u32,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m, 0x63d25c);
            { let args = []; c.r[0] = host.call(m, 0x63d118, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,0x6cb104u32,32); c.sub(a,0x0u32,32); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(6,32,c.get(0,32));
            if c.cond(4) { pc = 0x63d345; continue; }
            pc = 0x63d26c; continue;
            }
            0x63d26c => {
            c.push(m,c.get(6,32));
            c.push(m, 0x63d272);
            { let args = []; c.r[0] = host.call(m, 0x644f36, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x63d345; continue; }
            pc = 0x63d27b; continue;
            }
            0x63d27b => {
            c.set(0,32,rm(m,0x6cb108u32,32));
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.sub(a,c.get(0,32),32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m, 0x63d28c);
            { let args = []; c.r[0] = host.call(m, 0x63d118, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,0x1u32); }
            pc = 0x63d345; continue;
            }
            0x63d29b => {
            c.push(m,c.get(6,32));
            c.push(m, 0x63d2a1);
            { let args = []; c.r[0] = host.call(m, 0x63d118, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x63d2a9);
            { let args = []; c.r[0] = host.call(m, 0x644f36, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[6],32));
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x63d2b7; continue; }
            pc = 0x63d2b1; continue;
            }
            0x63d2b1 => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,0x6cb108u32,32),32); c.set(0,32,r); }
            pc = 0x63d2b7; continue;
            }
            0x63d2b7 => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,0x6cb100u32,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,0x3cu32);
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(7,32,c.get(1,32));
            { let b = c.get(7,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x63d2d6; continue; }
            pc = 0x63d2cf; continue;
            }
            0x63d2cf => {
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.sub(a,c.get(1,32),32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            pc = 0x63d2d6; continue;
            }
            0x63d2d6 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(7,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x4),32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x63d2f8; continue; }
            pc = 0x63d2ef; continue;
            }
            0x63d2ef => {
            { let a = c.get(2,32); let r = c.add(a,0x3cu32,32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.sub(a,c.get(1,32),32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            pc = 0x63d2f8; continue;
            }
            0x63d2f8 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,0x18u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x8),32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x63d319; continue; }
            pc = 0x63d311; continue;
            }
            0x63d311 => {
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.sub(a,c.get(1,32),32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            pc = 0x63d319; continue;
            }
            0x63d319 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,0x18u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(1,32,v); }
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x63d34a; continue; }
            pc = 0x63d32b; continue;
            }
            0x63d32b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.push(m,0x7u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x63d33f; continue;
            }
            0x63d33f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[6].wrapping_add(0x1c),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,r); } }
            pc = 0x63d345; continue;
            }
            0x63d345 => {
            c.set(0,32,c.get(6,32));
            pc = 0x63d347; continue;
            }
            0x63d347 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63d34a => {
            if c.cond(13) { pc = 0x63d345; continue; }
            pc = 0x63d34c; continue;
            }
            0x63d34c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.push(m,0x7u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x7));
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x63d33f; continue; }
            pc = 0x63d369; continue;
            }
            0x63d369 => {
            { let a = c.get(0,32); let r = c.add(a,0x1fu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x14),32); let r = c.dec(a,32); { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,r); } }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,0x16cu32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,0xbu32); }
            pc = 0x63d345; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063d776(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63d776;
    loop {
        match pc {
            0x63d776 => {
            c.push(m,0x0u32);
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m, 0x63d78d);
            { let args = []; c.r[0] = host.call(m, 0x63d791, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063e4f5(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63e4f5;
    loop {
        match pc {
            0x63e4f5 => {
            c.push(m,0x9u32);
            c.push(m, 0x63e4fc);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00640232(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x640232;
    loop {
        match pc {
            0x640232 => {
            c.set(0,32,rm(m,0x6f1a74u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6f1a78u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            pc = 0x640242; continue;
            }
            0x640242 => {
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x64025a; continue; }
            pc = 0x640246; continue;
            }
            0x640246 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[0].wrapping_add(0xc),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x100000u32,32); }
            if c.cond(2) { pc = 0x64025c; continue; }
            pc = 0x640255; continue;
            }
            0x640255 => {
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            pc = 0x640242; continue;
            }
            0x64025a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64025c; continue;
            }
            0x64025c => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00640586(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x640586;
    loop {
        match pc {
            0x640586 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6f1a74u32,32));
            c.set(2,32,rm(m,0x6f1a78u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.r[0].wrapping_add(0x17));
            { let a = c.get(1,32); let r = c.and(a,0xfffffff0u32,32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x20u32,32); }
            if c.cond(13) { pc = 0x6405c6; continue; }
            pc = 0x6405b8; continue;
            }
            0x6405b8 => {
            { let a = c.get(6,32); let r = c.or(a,0xffffffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(6,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(6,32)); }
            pc = 0x6405d6; continue;
            }
            0x6405c6 => {
            { let a = c.get(1,32); let r = c.add(a,0xffffffe0u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            pc = 0x6405d6; continue;
            }
            0x6405d6 => {
            c.set(0,32,rm(m,0x6f1a6cu32,32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            if c.cond(3) { pc = 0x6405fd; continue; }
            pc = 0x6405e4; continue;
            }
            0x6405e4 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.and(a,rm(m,c.r[5].wrapping_add(0xfffffff8),32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.and(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.or(a,c.get(7,32),32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x6405fd; continue; }
            pc = 0x6405f2; continue;
            }
            0x6405f2 => {
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffffc),32),32); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            if c.cond(2) { pc = 0x6405e4; continue; }
            pc = 0x6405fd; continue;
            }
            0x6405fd => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffffc),32),32); }
            if c.cond(5) { pc = 0x64067b; continue; }
            pc = 0x640602; continue;
            }
            0x640602 => {
            c.set(3,32,c.get(2,32));
            pc = 0x640604; continue;
            }
            0x640604 => {
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            if c.cond(3) { pc = 0x640620; continue; }
            pc = 0x64060b; continue;
            }
            0x64060b => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.and(a,rm(m,c.r[5].wrapping_add(0xfffffff8),32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.and(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.or(a,c.get(7,32),32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x64061e; continue; }
            pc = 0x640619; continue;
            }
            0x640619 => {
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            pc = 0x640604; continue;
            }
            0x64061e => {
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            pc = 0x640620; continue;
            }
            0x640620 => {
            if c.cond(5) { pc = 0x64067b; continue; }
            pc = 0x640622; continue;
            }
            0x640622 => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffffc),32),32); }
            if c.cond(3) { pc = 0x640638; continue; }
            pc = 0x640627; continue;
            }
            0x640627 => {
            { let a = rm(m,c.r[3].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x640635; continue; }
            pc = 0x64062d; continue;
            }
            0x64062d => {
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            pc = 0x640622; continue;
            }
            0x640635 => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffffc),32),32); }
            pc = 0x640638; continue;
            }
            0x640638 => {
            if c.cond(5) { pc = 0x640660; continue; }
            pc = 0x64063a; continue;
            }
            0x64063a => {
            c.set(3,32,c.get(2,32));
            pc = 0x64063c; continue;
            }
            0x64063c => {
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            if c.cond(3) { pc = 0x640650; continue; }
            pc = 0x640643; continue;
            }
            0x640643 => {
            { let a = rm(m,c.r[3].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x64064e; continue; }
            pc = 0x640649; continue;
            }
            0x640649 => {
            { let a = c.get(3,32); let r = c.add(a,0x14u32,32); c.set(3,32,r); }
            pc = 0x64063c; continue;
            }
            0x64064e => {
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            pc = 0x640650; continue;
            }
            0x640650 => {
            if c.cond(5) { pc = 0x640660; continue; }
            pc = 0x640652; continue;
            }
            0x640652 => {
            c.push(m, 0x640657);
            { let args = []; c.r[0] = host.call(m, 0x64088f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x640674; continue; }
            pc = 0x640660; continue;
            }
            0x640660 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x640666);
            { let args = []; c.r[0] = host.call(m, 0x640940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x10),32));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x10),32));
            { let a = rm(m,c.r[0],32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x64067b; continue; }
            pc = 0x640674; continue;
            }
            0x640674 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64088a; continue;
            }
            0x64067b => {
            { let a = 0x6f1a6cu32; wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x6406a2; continue; }
            pc = 0x64068e; continue;
            }
            0x64068e => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc4),32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x44),32));
            { let a = c.get(1,32); let r = c.and(a,rm(m,c.r[5].wrapping_add(0xfffffff8),32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.and(a,c.get(6,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.or(a,c.get(7,32),32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x6406d9; continue; }
            pc = 0x6406a2; continue;
            }
            0x6406a2 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc4),32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x44),32));
            { let a = c.get(2,32); let r = c.and(a,rm(m,c.r[5].wrapping_add(0xfffffff8),32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.and(a,rm(m,c.r[5].wrapping_add(0xfffffff4),32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.r[0].wrapping_add(0x44));
            { let a = c.get(2,32); let r = c.or(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            if c.cond(5) { pc = 0x6406d6; continue; }
            pc = 0x6406bf; continue;
            }
            0x6406bf => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x84),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(2,32); let r = c.and(a,rm(m,c.r[5].wrapping_add(0xfffffff8),32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let r = c.and(a,rm(m,c.r[1],32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.or(a,c.get(7,32),32); c.set(2,32,r); }
            if c.cond(4) { pc = 0x6406bf; continue; }
            pc = 0x6406d6; continue;
            }
            0x6406d6 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            pc = 0x6406d9; continue;
            }
            0x6406d9 => {
            c.set(1,32,c.get(2,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,0x204u32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x144));
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x44),32));
            { let a = c.get(1,32); let r = c.and(a,c.get(6,32),32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x640702; continue; }
            pc = 0x6406f5; continue;
            }
            0x6406f5 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc4),32));
            c.push(m,0x20u32);
            { let a = c.get(1,32); let r = c.and(a,rm(m,c.r[5].wrapping_add(0xfffffff8),32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x640702; continue;
            }
            0x640702 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x64070b; continue; }
            pc = 0x640706; continue;
            }
            0x640706 => {
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x640702; continue;
            }
            0x64070b => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[2],32));
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffff0),32),32); c.set(1,32,r); }
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(6,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x3fu32,32); }
            if c.cond(14) { pc = 0x640728; continue; }
            pc = 0x640725; continue;
            }
            0x640725 => {
            c.push(m,0x3fu32);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x640728; continue;
            }
            0x640728 => {
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x64083d; continue; }
            pc = 0x640730; continue;
            }
            0x640730 => {
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[2].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x640799; continue; }
            pc = 0x640738; continue;
            }
            0x640738 => {
            { let a = c.get(7,32); c.sub(a,0x20u32,32); }
            if c.cond(13) { pc = 0x640768; continue; }
            pc = 0x64073d; continue;
            }
            0x64073d => {
            c.set(3,32,0x80000000u32);
            c.set(1,32,c.get(7,32));
            { let a = c.get(3,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x4));
            { let a = c.get(3,32); c.set(3,32,!a & 0xffffffffu32); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(3,32); let r = c.and(a,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x44),32),32); c.set(3,32,r); }
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x44); wm(m,a,32,c.get(3,32)); }
            { let a = rm(m,c.r[7],8); let r = c.dec(a,8); { let a = c.r[7]; wm(m,a,8,r); } }
            if c.cond(5) { pc = 0x640796; continue; }
            pc = 0x64075e; continue;
            }
            0x64075e => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = rm(m,c.r[3],32); let r = c.and(a,c.get(1,32),32); { let a = c.r[3]; wm(m,a,32,r); } }
            pc = 0x640799; continue;
            }
            0x640768 => {
            c.set(1,32,c.r[7].wrapping_add(0xffffffe0));
            c.set(3,32,0x80000000u32);
            { let a = c.get(3,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[7]).wrapping_add(0x4));
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xc4));
            { let a = c.get(3,32); c.set(3,32,!a & 0xffffffffu32); }
            { let a = rm(m,c.r[1],32); let r = c.and(a,c.get(3,32),32); { let a = c.r[1]; wm(m,a,32,r); } }
            { let a = rm(m,c.r[7],8); let r = c.dec(a,8); { let a = c.r[7]; wm(m,a,8,r); } }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(3,32)); }
            if c.cond(5) { pc = 0x640796; continue; }
            pc = 0x64078b; continue;
            }
            0x64078b => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = rm(m,c.r[3].wrapping_add(0x4),32); let r = c.and(a,c.get(1,32),32); { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,r); } }
            pc = 0x640799; continue;
            }
            0x640796 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x640799; continue;
            }
            0x640799 => {
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.sub(a,0x0u32,32); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x640849; continue; }
            pc = 0x6407b5; continue;
            }
            0x6407b5 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(8)).wrapping_add(0x4),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[6].wrapping_mul(8)));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[2].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x64083a; continue; }
            pc = 0x6407d6; continue;
            }
            0x6407d6 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x4),8));
            { let a = c.get(6,32); c.sub(a,0x20u32,32); }
            { let a = c.r[5].wrapping_add(0xb); wm(m,a,8,c.get(1,8)); }
            if c.cond(13) { pc = 0x64080b; continue; }
            pc = 0x6407e2; continue;
            }
            0x6407e2 => {
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xb),8); c.sub(a,0x0u32,8); }
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            if c.cond(5) { pc = 0x6407f9; continue; }
            pc = 0x6407ee; continue;
            }
            0x6407ee => {
            c.set(7,32,0x80000000u32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(7,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(7,32,r); }
            { let a = rm(m,c.r[3],32); let r = c.or(a,c.get(7,32),32); { let a = c.r[3]; wm(m,a,32,r); } }
            pc = 0x6407f9; continue;
            }
            0x6407f9 => {
            c.set(7,32,0x80000000u32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(7,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x44),32); let r = c.or(a,c.get(7,32),32); { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x44); wm(m,a,32,r); } }
            pc = 0x64083a; continue;
            }
            0x64080b => {
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xb),8); c.sub(a,0x0u32,8); }
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            if c.cond(5) { pc = 0x640824; continue; }
            pc = 0x640817; continue;
            }
            0x640817 => {
            c.set(1,32,c.r[6].wrapping_add(0xffffffe0));
            c.set(7,32,0x80000000u32);
            { let a = c.get(7,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(7,32,r); }
            { let a = rm(m,c.r[3].wrapping_add(0x4),32); let r = c.or(a,c.get(7,32),32); { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,r); } }
            pc = 0x640824; continue;
            }
            0x640824 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xc4));
            c.set(1,32,c.r[6].wrapping_add(0xffffffe0));
            c.set(6,32,0x80000000u32);
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(6,32,r); }
            { let a = rm(m,c.r[7],32); let r = c.or(a,c.get(6,32),32); { let a = c.r[7]; wm(m,a,32,r); } }
            pc = 0x64083a; continue;
            }
            0x64083a => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            pc = 0x64083d; continue;
            }
            0x64083d => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x64084c; continue; }
            pc = 0x640841; continue;
            }
            0x640841 => {
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[1].wrapping_add(c.r[2]).wrapping_add(0xfffffffc); wm(m,a,32,c.get(1,32)); }
            pc = 0x64084c; continue;
            }
            0x640849 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            pc = 0x64084c; continue;
            }
            0x64084c => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xfffffff0),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x1));
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xfffffffc); wm(m,a,32,c.get(1,32)); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(7,32,c.r[1].wrapping_add(0x1));
            { let a = c.r[6]; wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x640882; continue; }
            pc = 0x640868; continue;
            }
            0x640868 => {
            { let a = c.get(3,32); c.sub(a,rm(m,0x6f1a70u32,32),32); }
            if c.cond(5) { pc = 0x640882; continue; }
            pc = 0x640870; continue;
            }
            0x640870 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(1,32); c.sub(a,rm(m,0x6f1a68u32,32),32); }
            if c.cond(5) { pc = 0x640882; continue; }
            pc = 0x64087b; continue;
            }
            0x64087b => {
            { let a = rm(m,0x6f1a70u32,32); let r = c.and(a,0x0u32,32); { let a = 0x6f1a70u32; wm(m,a,32,r); } }
            pc = 0x640882; continue;
            }
            0x640882 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[2].wrapping_add(0x4));
            pc = 0x64088a; continue;
            }
            0x64088a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00640a3b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x640a3b;
    loop {
        match pc {
            0x640a3b => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(2,32,c.get(7,32));
            c.set(6,32,c.r[0].wrapping_add(0x17));
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[1].wrapping_add(0xc),32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.and(a,0xfffffff0u32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let n = 0xfu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.imul(a,0x204u32,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x144));
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xfffffffc),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(3,32,rm(m,c.r[1].wrapping_add(c.r[7]).wrapping_add(0xfffffffc),32));
            c.set(7,32,c.r[1].wrapping_add(c.r[7]).wrapping_add(0xfffffffc));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            if c.cond(14) { pc = 0x640be9; continue; }
            pc = 0x640a8a; continue;
            }
            0x640a8a => {
            { let a = c.get(3,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x640be2; continue; }
            pc = 0x640a93; continue;
            }
            0x640a93 => {
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(15) { pc = 0x640be2; continue; }
            pc = 0x640a9d; continue;
            }
            0x640a9d => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x3fu32,32); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            if c.cond(6) { pc = 0x640ab2; continue; }
            pc = 0x640aac; continue;
            }
            0x640aac => {
            c.push(m,0x3fu32);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            pc = 0x640ab2; continue;
            }
            0x640ab2 => {
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[7].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x640b02; continue; }
            pc = 0x640aba; continue;
            }
            0x640aba => {
            { let a = c.get(1,32); c.sub(a,0x20u32,32); }
            if c.cond(3) { pc = 0x640ade; continue; }
            pc = 0x640abf; continue;
            }
            0x640abf => {
            c.set(3,32,0x80000000u32);
            { let a = c.get(3,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x4));
            { let a = c.get(3,32); c.set(3,32,!a & 0xffffffffu32); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x44),32); let r = c.and(a,c.get(3,32),32); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x44); wm(m,a,32,r); } }
            { let a = rm(m,c.r[1],8); let r = c.dec(a,8); { let a = c.r[1]; wm(m,a,8,r); } }
            if c.cond(5) { pc = 0x640b02; continue; }
            pc = 0x640ad7; continue;
            }
            0x640ad7 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1],32); let r = c.and(a,c.get(3,32),32); { let a = c.r[1]; wm(m,a,32,r); } }
            pc = 0x640b02; continue;
            }
            0x640ade => {
            { let a = c.get(1,32); let r = c.add(a,0xffffffe0u32,32); c.set(1,32,r); }
            c.set(3,32,0x80000000u32);
            { let a = c.get(3,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x4));
            { let a = c.get(3,32); c.set(3,32,!a & 0xffffffffu32); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc4),32); let r = c.and(a,c.get(3,32),32); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc4); wm(m,a,32,r); } }
            { let a = rm(m,c.r[1],8); let r = c.dec(a,8); { let a = c.r[1]; wm(m,a,8,r); } }
            if c.cond(5) { pc = 0x640b02; continue; }
            pc = 0x640afc; continue;
            }
            0x640afc => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); let r = c.and(a,c.get(3,32),32); { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,r); } }
            pc = 0x640b02; continue;
            }
            0x640b02 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.add(a,c.get(1,32),32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x640bd0; continue; }
            pc = 0x640b26; continue;
            }
            0x640b26 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[6]).wrapping_add(0xfffffffc));
            { let a = c.get(7,32); c.sub(a,0x3fu32,32); }
            if c.cond(6) { pc = 0x640b3c; continue; }
            pc = 0x640b39; continue;
            }
            0x640b39 => {
            c.push(m,0x3fu32);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x640b3c; continue;
            }
            0x640b3c => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(3,32,c.r[3].wrapping_add(c.r[7].wrapping_mul(8)));
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[3].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x640bbe; continue; }
            pc = 0x640b62; continue;
            }
            0x640b62 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x4),8));
            { let a = c.get(7,32); c.sub(a,0x20u32,32); }
            { let a = c.r[5].wrapping_add(0x13); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            if c.cond(3) { pc = 0x640b95; continue; }
            pc = 0x640b74; continue;
            }
            0x640b74 => {
            { let a = rm(m,c.r[5].wrapping_add(0x13),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x640b88; continue; }
            pc = 0x640b7a; continue;
            }
            0x640b7a => {
            c.set(3,32,0x80000000u32);
            c.set(1,32,c.get(7,32));
            { let a = c.get(3,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1],32); let r = c.or(a,c.get(3,32),32); { let a = c.r[1]; wm(m,a,32,r); } }
            pc = 0x640b88; continue;
            }
            0x640b88 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x44));
            c.set(2,32,0x80000000u32);
            c.set(1,32,c.get(7,32));
            pc = 0x640bba; continue;
            }
            0x640b95 => {
            { let a = rm(m,c.r[5].wrapping_add(0x13),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x640bab; continue; }
            pc = 0x640b9b; continue;
            }
            0x640b9b => {
            c.set(1,32,c.r[7].wrapping_add(0xffffffe0));
            c.set(3,32,0x80000000u32);
            { let a = c.get(3,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); let r = c.or(a,c.get(3,32),32); { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,r); } }
            pc = 0x640bab; continue;
            }
            0x640bab => {
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc4));
            c.set(1,32,c.r[7].wrapping_add(0xffffffe0));
            c.set(2,32,0x80000000u32);
            pc = 0x640bba; continue;
            }
            0x640bba => {
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = rm(m,c.r[0],32); let r = c.or(a,c.get(2,32),32); { let a = c.r[0]; wm(m,a,32,r); } }
            pc = 0x640bbe; continue;
            }
            0x640bbe => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.set(0,32,c.r[2].wrapping_add(c.r[6]).wrapping_add(0xfffffffc));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[1].wrapping_add(c.r[0]).wrapping_add(0xfffffffc); wm(m,a,32,c.get(1,32)); }
            pc = 0x640bd3; continue;
            }
            0x640bd0 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xc),32));
            pc = 0x640bd3; continue;
            }
            0x640bd3 => {
            c.set(0,32,c.r[6].wrapping_add(0x1));
            { let a = c.r[2].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            pc = 0x640d29; continue;
            }
            0x640be2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x640d2c; continue;
            }
            0x640be9 => {
            if c.cond(13) { pc = 0x640d29; continue; }
            pc = 0x640bef; continue;
            }
            0x640bef => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.sub(a,c.get(6,32),32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            c.set(1,32,c.r[6].wrapping_add(0x1));
            { let a = c.r[3].wrapping_add(0xfffffffc); wm(m,a,32,c.get(1,32)); }
            c.set(3,32,c.r[3].wrapping_add(c.r[6]).wrapping_add(0xfffffffc));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(6,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.r[3].wrapping_add(0xfffffffc); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(6,32); c.sub(a,0x3fu32,32); }
            if c.cond(6) { pc = 0x640c14; continue; }
            pc = 0x640c11; continue;
            }
            0x640c11 => {
            c.push(m,0x3fu32);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x640c14; continue;
            }
            0x640c14 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x640ca3; continue; }
            pc = 0x640c1e; continue;
            }
            0x640c1e => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x3fu32,32); }
            if c.cond(6) { pc = 0x640c2d; continue; }
            pc = 0x640c2a; continue;
            }
            0x640c2a => {
            c.push(m,0x3fu32);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x640c2d; continue;
            }
            0x640c2d => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[7].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x640c7c; continue; }
            pc = 0x640c35; continue;
            }
            0x640c35 => {
            { let a = c.get(6,32); c.sub(a,0x20u32,32); }
            if c.cond(3) { pc = 0x640c58; continue; }
            pc = 0x640c3a; continue;
            }
            0x640c3a => {
            c.set(3,32,0x80000000u32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(3,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(3,32,r); }
            c.set(6,32,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x4));
            { let a = c.get(3,32); c.set(3,32,!a & 0xffffffffu32); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x44),32); let r = c.and(a,c.get(3,32),32); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x44); wm(m,a,32,r); } }
            { let a = rm(m,c.r[6],8); let r = c.dec(a,8); { let a = c.r[6]; wm(m,a,8,r); } }
            if c.cond(5) { pc = 0x640c79; continue; }
            pc = 0x640c51; continue;
            }
            0x640c51 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1],32); let r = c.and(a,c.get(3,32),32); { let a = c.r[1]; wm(m,a,32,r); } }
            pc = 0x640c79; continue;
            }
            0x640c58 => {
            c.set(1,32,c.r[6].wrapping_add(0xffffffe0));
            c.set(3,32,0x80000000u32);
            { let a = c.get(3,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(3,32,r); }
            c.set(1,32,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x4));
            { let a = c.get(3,32); c.set(3,32,!a & 0xffffffffu32); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc4),32); let r = c.and(a,c.get(3,32),32); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc4); wm(m,a,32,r); } }
            { let a = rm(m,c.r[1],8); let r = c.dec(a,8); { let a = c.r[1]; wm(m,a,8,r); } }
            if c.cond(5) { pc = 0x640c79; continue; }
            pc = 0x640c73; continue;
            }
            0x640c73 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); let r = c.and(a,c.get(3,32),32); { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,r); } }
            pc = 0x640c79; continue;
            }
            0x640c79 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            pc = 0x640c7c; continue;
            }
            0x640c7c => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0xfffffffc),32),32); c.set(6,32,r); }
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(6,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x3fu32,32); }
            if c.cond(6) { pc = 0x640ca3; continue; }
            pc = 0x640ca0; continue;
            }
            0x640ca0 => {
            c.push(m,0x3fu32);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x640ca3; continue;
            }
            0x640ca3 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(8)).wrapping_add(0x4),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[6].wrapping_mul(8)));
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[3].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[3].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x640d20; continue; }
            pc = 0x640cc4; continue;
            }
            0x640cc4 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x4),8));
            { let a = c.get(6,32); c.sub(a,0x20u32,32); }
            { let a = c.r[5].wrapping_add(0xf); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let a = c.r[6].wrapping_add(c.r[0]).wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            if c.cond(3) { pc = 0x640cf7; continue; }
            pc = 0x640cd6; continue;
            }
            0x640cd6 => {
            { let a = rm(m,c.r[5].wrapping_add(0xf),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x640cea; continue; }
            pc = 0x640cdc; continue;
            }
            0x640cdc => {
            c.set(7,32,0x80000000u32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(7,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1],32); let r = c.or(a,c.get(7,32),32); { let a = c.r[1]; wm(m,a,32,r); } }
            pc = 0x640cea; continue;
            }
            0x640cea => {
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x44));
            c.set(2,32,0x80000000u32);
            c.set(1,32,c.get(6,32));
            pc = 0x640d1c; continue;
            }
            0x640cf7 => {
            { let a = rm(m,c.r[5].wrapping_add(0xf),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x640d0d; continue; }
            pc = 0x640cfd; continue;
            }
            0x640cfd => {
            c.set(1,32,c.r[6].wrapping_add(0xffffffe0));
            c.set(7,32,0x80000000u32);
            { let a = c.get(7,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); let r = c.or(a,c.get(7,32),32); { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,r); } }
            pc = 0x640d0d; continue;
            }
            0x640d0d => {
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xc4));
            c.set(1,32,c.r[6].wrapping_add(0xffffffe0));
            c.set(2,32,0x80000000u32);
            pc = 0x640d1c; continue;
            }
            0x640d1c => {
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = rm(m,c.r[0],32); let r = c.or(a,c.get(2,32),32); { let a = c.r[0]; wm(m,a,32,r); } }
            pc = 0x640d20; continue;
            }
            0x640d20 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(c.r[3]).wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            pc = 0x640d29; continue;
            }
            0x640d29 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x640d2c; continue;
            }
            0x640d2c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00640f8d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x640f8d;
    loop {
        match pc {
            0x640f8d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,0x6c8988u32);
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(2,32));
            pc = 0x640f99; continue;
            }
            0x640f99 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x10),32),32); }
            if c.cond(6) { pc = 0x640fa3; continue; }
            pc = 0x640f9e; continue;
            }
            0x640f9e => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x14),32),32); }
            if c.cond(2) { pc = 0x640fab; continue; }
            pc = 0x640fa3; continue;
            }
            0x640fa3 => {
            c.set(1,32,rm(m,c.r[1],32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x640fe0; continue; }
            pc = 0x640fa9; continue;
            }
            0x640fa9 => {
            pc = 0x640f99; continue;
            }
            0x640fab => {
            { let a = c.get(0,8); c.and(a,0xfu32,8); }
            if c.cond(5) { pc = 0x640fe0; continue; }
            pc = 0x640faf; continue;
            }
            0x640faf => {
            c.set(6,32,c.get(0,32));
            c.set(2,32,0x100u32);
            { let a = c.get(6,32); let r = c.and(a,0xfffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x640fe0; continue; }
            pc = 0x640fc0; continue;
            }
            0x640fc0 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,16); let r = c.and(a,0xf000u32,16); c.set(1,16,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x8));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x640fe0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00640fe4(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x640fe4;
    loop {
        match pc {
            0x640fe4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[0].wrapping_add(0x10),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0xcu32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x18));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[1],8));
            { let a = rm(m,c.r[0],32); let r = c.add(a,c.get(2,32),32); { let a = c.r[0]; wm(m,a,32,r); } }
            { let a = rm(m,c.r[1],8); let r = c.and(a,0x0u32,8); { let a = c.r[1]; wm(m,a,8,r); } }
            { let a = rm(m,c.r[0],32); c.sub(a,0xf0u32,32); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,0xf1u32); }
            if c.cond(5) { pc = 0x641028; continue; }
            pc = 0x641011; continue;
            }
            0x641011 => {
            { let a = rm(m,0x6f019cu32,32); let r = c.inc(a,32); { let a = 0x6f019cu32; wm(m,a,32,r); } }
            { let a = rm(m,0x6f019cu32,32); c.sub(a,0x20u32,32); }
            if c.cond(5) { pc = 0x641028; continue; }
            pc = 0x641020; continue;
            }
            0x641020 => {
            c.push(m,0x10u32);
            c.push(m, 0x641027);
            { let args = []; c.r[0] = host.call(m, 0x640ecb, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x641028; continue;
            }
            0x641028 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00641029(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x641029;
    loop {
        match pc {
            0x641029 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6ca9a8u32,32));
            c.push(m,c.get(7,32));
            pc = 0x641037; continue;
            }
            0x641037 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(2,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x6410e2; continue; }
            pc = 0x641043; continue;
            }
            0x641043 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,c.r[6].wrapping_add(0x2018));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,0x18u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0xcu32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            if c.cond(3) { pc = 0x64109c; continue; }
            pc = 0x641062; continue;
            }
            0x641062 => {
            c.set(1,32,rm(m,c.r[7],32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x641085; continue; }
            pc = 0x64106b; continue;
            }
            0x64106b => {
            { let a = rm(m,c.r[7].wrapping_add(0x4),32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x641085; continue; }
            pc = 0x641070; continue;
            }
            0x641070 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x641078);
            { let args = []; c.r[0] = host.call(m, 0x641231, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x6410f4; continue; }
            pc = 0x64107f; continue;
            }
            0x64107f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            pc = 0x641085; continue;
            }
            0x641085 => {
            { let a = c.get(7,32); let r = c.add(a,0x8u32,32); c.set(7,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x2018));
            { let a = c.get(0,32); let r = c.add(a,0x1000u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x641062; continue; }
            pc = 0x64109a; continue;
            }
            0x64109a => {
            pc = 0x64109f; continue;
            }
            0x64109c => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x64109f; continue;
            }
            0x64109f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(7,32,c.r[6].wrapping_add(0x18));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x6410e5; continue; }
            pc = 0x6410b2; continue;
            }
            0x6410b2 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x6410d1; continue; }
            pc = 0x6410b8; continue;
            }
            0x6410b8 => {
            { let a = rm(m,c.r[7].wrapping_add(0x4),32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x6410d1; continue; }
            pc = 0x6410bd; continue;
            }
            0x6410bd => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.push(m, v); }
            c.push(m, 0x6410c7);
            { let args = []; c.r[0] = host.call(m, 0x641231, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x6410f4; continue; }
            pc = 0x6410ce; continue;
            }
            0x6410ce => {
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            pc = 0x6410d1; continue;
            }
            0x6410d1 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.add(a,0x1000u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(7,32); let r = c.add(a,0x8u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffff8),32),32); }
            if c.cond(2) { pc = 0x6410b2; continue; }
            pc = 0x6410e0; continue;
            }
            0x6410e0 => {
            pc = 0x6410e5; continue;
            }
            0x6410e2 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x6410e5; continue;
            }
            0x6410e5 => {
            c.set(6,32,rm(m,c.r[6],32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6ca9a8u32,32),32); }
            if c.cond(4) { pc = 0x641104; continue; }
            pc = 0x6410ef; continue;
            }
            0x6410ef => {
            pc = 0x641037; continue;
            }
            0x6410f4 => {
            { let a = 0x6ca9a8u32; wm(m,a,32,c.get(6,32)); }
            { let a = rm(m,c.r[7],32); let r = c.sub(a,c.get(3,32),32); { let a = c.r[7]; wm(m,a,32,r); } }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            pc = 0x64122c; continue;
            }
            0x641104 => {
            c.set(0,32,0x6c8988u32);
            c.set(7,32,c.get(0,32));
            pc = 0x64110b; continue;
            }
            0x64110b => {
            { let a = rm(m,c.r[7].wrapping_add(0x10),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x641117; continue; }
            pc = 0x641111; continue;
            }
            0x641111 => {
            { let a = rm(m,c.r[7].wrapping_add(0xc),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x641123; continue; }
            pc = 0x641117; continue;
            }
            0x641117 => {
            c.set(7,32,rm(m,c.r[7],32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6411f8; continue; }
            pc = 0x641121; continue;
            }
            0x641121 => {
            pc = 0x64110b; continue;
            }
            0x641123 => {
            c.set(3,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(6,32,c.get(3,32));
            c.set(0,32,c.get(3,32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,0x18u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0xcu32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,rm(m,c.r[7].wrapping_add(0x10),32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[3],32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x641152; continue; }
            pc = 0x641141; continue;
            }
            0x641141 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x10u32,32); }
            if c.cond(13) { pc = 0x641152; continue; }
            pc = 0x641147; continue;
            }
            0x641147 => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[0],32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x641141; continue; }
            pc = 0x641152; continue;
            }
            0x641152 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.push(m,0x4u32);
            { let a = c.get(0,32); let n = 0xcu32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,0x1000u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x66a240u32,32); c.push(m, 0x64116a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x64122a; continue; }
            pc = 0x641172; continue;
            }
            0x641172 => {
            c.push(m,0x0u32);
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.push(m, v); }
            c.push(m,c.get(6,32));
            c.push(m, 0x64117d);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            c.set(1,32,c.get(3,32));
            if c.cond(14) { pc = 0x6411b9; continue; }
            pc = 0x641189; continue;
            }
            0x641189 => {
            c.set(0,32,c.r[6].wrapping_add(0x4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(2,32)); }
            pc = 0x64118f; continue;
            }
            0x64118f => {
            { let a = rm(m,c.r[0].wrapping_add(0xf4),8); let r = c.or(a,0xffu32,8); { let a = c.r[0].wrapping_add(0xf4); wm(m,a,8,r); } }
            c.set(2,32,c.r[0].wrapping_add(0x4));
            { let a = c.r[0].wrapping_add(0xfffffffc); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,0xf0u32);
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,0xf1u32); }
            { let a = c.get(0,32); let r = c.add(a,0x1000u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            if c.cond(5) { pc = 0x64118f; continue; }
            pc = 0x6411b9; continue;
            }
            0x6411b9 => {
            { let a = 0x6ca9a8u32; wm(m,a,32,c.get(7,32)); }
            c.set(0,32,c.r[7].wrapping_add(0x2018));
            pc = 0x6411c5; continue;
            }
            0x6411c5 => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x6411d5; continue; }
            pc = 0x6411c9; continue;
            }
            0x6411c9 => {
            { let a = rm(m,c.r[1],32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x6411d3; continue; }
            pc = 0x6411ce; continue;
            }
            0x6411ce => {
            { let a = c.get(1,32); let r = c.add(a,0x8u32,32); c.set(1,32,r); }
            pc = 0x6411c5; continue;
            }
            0x6411d3 => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            pc = 0x6411d5; continue;
            }
            0x6411d5 => {
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = rm(m,c.r[3],32); let r = c.sub(a,c.get(0,32),32); { let a = c.r[3]; wm(m,a,32,r); } }
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.sub(a,c.get(0,32),32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            c.set(1,32,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x8));
            c.set(0,32,c.r[6].wrapping_add(0x100));
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            pc = 0x64122c; continue;
            }
            0x6411f8 => {
            c.push(m, 0x6411fd);
            { let args = []; c.r[0] = host.call(m, 0x640d31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64122a; continue; }
            pc = 0x641201; continue;
            }
            0x641201 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x10),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,8,c.get(3,8)); }
            c.set(2,32,c.r[1].wrapping_add(c.r[3]).wrapping_add(0x8));
            { let a = 0x6ca9a8u32; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,0xf0u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,c.get(3,8));
            { let a = rm(m,c.r[0].wrapping_add(0x18),32); let r = c.sub(a,c.get(2,32),32); { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,r); } }
            c.set(0,32,c.r[1].wrapping_add(0x100));
            pc = 0x64122c; continue;
            }
            0x64122a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64122c; continue;
            }
            0x64122c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00641355(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x641355;
    loop {
        match pc {
            0x641355 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.set(1,32,rm(m,c.r[2],8));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[7].wrapping_add(0x10),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0xcu32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x14),32),32); }
            c.set(7,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x18));
            if c.cond(6) { pc = 0x64138f; continue; }
            pc = 0x64137d; continue;
            }
            0x64137d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[2]; wm(m,a,8,c.get(0,8)); }
            { let a = rm(m,c.r[7],32); let r = c.add(a,c.get(1,32),32); { let a = c.r[7]; wm(m,a,32,r); } }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,0xf1u32); }
            pc = 0x6413ef; continue;
            }
            0x64138f => {
            if c.cond(3) { pc = 0x6413f6; continue; }
            pc = 0x641391; continue;
            }
            0x641391 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(6,32,c.r[2].wrapping_add(c.r[0]));
            c.set(0,32,c.r[3].wrapping_add(0xf8));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x6413f6; continue; }
            pc = 0x6413a1; continue;
            }
            0x6413a1 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[2]));
            pc = 0x6413a4; continue;
            }
            0x6413a4 => {
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x6413b2; continue; }
            pc = 0x6413a8; continue;
            }
            0x6413a8 => {
            { let a = rm(m,c.r[0],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x6413b0; continue; }
            pc = 0x6413ad; continue;
            }
            0x6413ad => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x6413a4; continue;
            }
            0x6413b0 => {
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            pc = 0x6413b2; continue;
            }
            0x6413b2 => {
            if c.cond(5) { pc = 0x6413f6; continue; }
            pc = 0x6413b4; continue;
            }
            0x6413b4 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x14),8));
            { let a = c.r[2]; wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x6413ea; continue; }
            pc = 0x6413bf; continue;
            }
            0x6413bf => {
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x6413ea; continue; }
            pc = 0x6413c3; continue;
            }
            0x6413c3 => {
            c.set(0,32,c.r[3].wrapping_add(0xf8));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x6413e1; continue; }
            pc = 0x6413cd; continue;
            }
            0x6413cd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(6,32)); }
            { let a = rm(m,c.r[6],8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x6413dc; continue; }
            pc = 0x6413d5; continue;
            }
            0x6413d5 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(c.r[0]),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x6413d5; continue; }
            pc = 0x6413dc; continue;
            }
            0x6413dc => {
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x6413ea; continue;
            }
            0x6413e1 => {
            { let a = rm(m,c.r[3].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,r); } }
            c.set(0,32,c.r[3].wrapping_add(0x8));
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            pc = 0x6413ea; continue;
            }
            0x6413ea => {
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0x14),32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[7],32); let r = c.add(a,c.get(1,32),32); { let a = c.r[7]; wm(m,a,32,r); } }
            pc = 0x6413ef; continue;
            }
            0x6413ef => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            pc = 0x6413f6; continue;
            }
            0x6413f6 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064149d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64149d;
    loop {
        match pc {
            0x64149d => {
            c.set(0,32,rm(m,0x6f0200u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6414b5; continue; }
            pc = 0x6414a6; continue;
            }
            0x6414a6 => {
            { let v = rm(m,c.r[4].wrapping_add(0x4),32); c.push(m, v); }
            { let target = c.get(0,32); c.push(m, 0x6414ac); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x6414b5; continue; }
            pc = 0x6414b1; continue;
            }
            0x6414b1 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x6414b5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00643ab5(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x643ab5;
    loop {
        match pc {
            0x643ab5 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x2u32);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m, 0x643ac3);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = rm(m,0x6f1a60u32,32); c.sub(a,c.get(3,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(14) { pc = 0x643b6f; continue; }
            pc = 0x643ad2; continue;
            }
            0x643ad2 => {
            c.set(0,32,rm(m,0x6f0a4cu32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x643b15; continue; }
            pc = 0x643ade; continue;
            }
            0x643ade => {
            { let a = rm(m,c.r[0].wrapping_add(0xc),8); c.and(a,0x83u32,8); }
            if c.cond(5) { pc = 0x643b05; continue; }
            pc = 0x643ae4; continue;
            }
            0x643ae4 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x643aeb);
            { let args = []; c.r[0] = host.call(m, 0x643583, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,rm(m,0x6f0a4cu32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)),32));
            { let a = rm(m,c.r[0].wrapping_add(0xc),8); c.and(a,0x83u32,8); }
            if c.cond(4) { pc = 0x643b10; continue; }
            pc = 0x643afc; continue;
            }
            0x643afc => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x643b03);
            { let args = []; c.r[0] = host.call(m, 0x6435d5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x643b05; continue;
            }
            0x643b05 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,rm(m,0x6f1a60u32,32),32); }
            if c.cond(12) { pc = 0x643ad2; continue; }
            pc = 0x643b0e; continue;
            }
            0x643b0e => {
            pc = 0x643b6f; continue;
            }
            0x643b10 => {
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)),32));
            pc = 0x643b59; continue;
            }
            0x643b15 => {
            c.push(m,0x38u32);
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.push(m, 0x643b1f);
            { let args = []; c.r[0] = host.call(m, 0x63cb41, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,rm(m,0x6f0a4cu32,32));
            { let a = c.r[6].wrapping_add(c.r[1]); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6f0a4cu32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0]),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x643b6f; continue; }
            pc = 0x643b35; continue;
            }
            0x643b35 => {
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1a4u32,32); c.push(m, 0x643b3f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6f0a4cu32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[0]),32));
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1b0u32,32); c.push(m, 0x643b51); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6f0a4cu32,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(c.r[0]),32));
            pc = 0x643b59; continue;
            }
            0x643b59 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x643b6f; continue; }
            pc = 0x643b5d; continue;
            }
            0x643b5d => {
            { let a = rm(m,c.r[7].wrapping_add(0x10),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[7].wrapping_add(0x10); wm(m,a,32,r); } }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[7]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[7].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            pc = 0x643b6f; continue;
            }
            0x643b6f => {
            c.push(m,0x2u32);
            c.push(m, 0x643b76);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00646137(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x646137;
    loop {
        match pc {
            0x646137 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x646143; continue; }
            pc = 0x646141; continue;
            }
            0x646141 => {
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x646143 => {
            { let a = rm(m,0x6f0224u32,32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x64615e; continue; }
            pc = 0x64614c; continue;
            }
            0x64614c => {
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xc),16));
            { let a = c.get(1,16); c.sub(a,0xffu32,16); }
            if c.cond(7) { pc = 0x646190; continue; }
            pc = 0x646157; continue;
            }
            0x646157 => {
            c.push(m,0x1u32);
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            { let v = c.pop(m); c.set(0,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x64615e => {
            c.set(1,32,c.r[5].wrapping_add(0x8));
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            { let v = rm(m,0x6c896cu32,32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xc));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,0x220u32);
            { let v = rm(m,0x6f0234u32,32); c.push(m, v); }
            { let target = rm(m,0x66a2a0u32,32); c.push(m, 0x646186); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x646190; continue; }
            pc = 0x64618a; continue;
            }
            0x64618a => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x64619e; continue; }
            pc = 0x646190; continue;
            }
            0x646190 => {
            c.push(m, 0x646195);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x2au32); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x64619e; continue;
            }
            0x64619e => {
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00646eac(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x646eac;
    loop {
        match pc {
            0x646eac => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(1,8); c.and(a,0x80u32,8); }
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,0xcu32); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x646ed2; continue; }
            pc = 0x646ec9; continue;
            }
            0x646ec9 => {
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0xffffffff); wm(m,a,8,0x10u32); }
            pc = 0x646edd; continue;
            }
            0x646ed2 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffff),8); let r = c.and(a,0x0u32,8); { let a = c.r[5].wrapping_add(0xffffffff); wm(m,a,8,r); } }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,0x1u32); }
            pc = 0x646edd; continue;
            }
            0x646edd => {
            c.set(0,32,0x8000u32);
            { let a = c.get(0,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x646ef7; continue; }
            pc = 0x646ee6; continue;
            }
            0x646ee6 => {
            { let a = c.get(5,8); c.and(a,0x40u32,8); }
            if c.cond(5) { pc = 0x646ef3; continue; }
            pc = 0x646eeb; continue;
            }
            0x646eeb => {
            { let a = rm(m,0x6f0494u32,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x646ef7; continue; }
            pc = 0x646ef3; continue;
            }
            0x646ef3 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffff),8); let r = c.or(a,0x80u32,8); { let a = c.r[5].wrapping_add(0xffffffff); wm(m,a,8,r); } }
            pc = 0x646ef7; continue;
            }
            0x646ef7 => {
            c.push(m,0x3u32);
            c.set(0,32,c.get(1,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.and(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x646f1e; continue; }
            pc = 0x646f02; continue;
            }
            0x646f02 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x646f15; continue; }
            pc = 0x646f05; continue;
            }
            0x646f05 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x646fb0; continue; }
            pc = 0x646f0c; continue;
            }
            0x646f0c => {
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,0xc0000000u32); }
            pc = 0x646f25; continue;
            }
            0x646f15 => {
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,0x40000000u32); }
            pc = 0x646f25; continue;
            }
            0x646f1e => {
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,0x80000000u32); }
            pc = 0x646f25; continue;
            }
            0x646f25 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            if c.cond(4) { pc = 0x646f53; continue; }
            pc = 0x646f2d; continue;
            }
            0x646f2d => {
            { let a = c.get(0,32); c.sub(a,0x20u32,32); }
            if c.cond(4) { pc = 0x646f4a; continue; }
            pc = 0x646f32; continue;
            }
            0x646f32 => {
            { let a = c.get(0,32); c.sub(a,0x30u32,32); }
            if c.cond(4) { pc = 0x646f41; continue; }
            pc = 0x646f37; continue;
            }
            0x646f37 => {
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(5) { pc = 0x646fb0; continue; }
            pc = 0x646f3c; continue;
            }
            0x646f3c => {
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            pc = 0x646f56; continue;
            }
            0x646f41 => {
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,0x2u32); }
            pc = 0x646f56; continue;
            }
            0x646f4a => {
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,0x1u32); }
            pc = 0x646f56; continue;
            }
            0x646f53 => {
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(3,32)); }
            pc = 0x646f56; continue;
            }
            0x646f56 => {
            c.set(2,32,0x700u32);
            c.set(0,32,0x400u32);
            { let a = c.get(1,32); let r = c.and(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(7,32,0x100u32);
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x646f9c; continue; }
            pc = 0x646f6b; continue;
            }
            0x646f6b => {
            if c.cond(4) { pc = 0x646f97; continue; }
            pc = 0x646f6d; continue;
            }
            0x646f6d => {
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x646f97; continue; }
            pc = 0x646f71; continue;
            }
            0x646f71 => {
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x646f8e; continue; }
            pc = 0x646f75; continue;
            }
            0x646f75 => {
            { let a = c.get(1,32); c.sub(a,0x200u32,32); }
            if c.cond(4) { pc = 0x646fca; continue; }
            pc = 0x646f7d; continue;
            }
            0x646f7d => {
            { let a = c.get(1,32); c.sub(a,0x300u32,32); }
            if c.cond(5) { pc = 0x646fb0; continue; }
            pc = 0x646f85; continue;
            }
            0x646f85 => {
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,0x2u32); }
            pc = 0x646fda; continue;
            }
            0x646f8e => {
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,0x4u32); }
            pc = 0x646fda; continue;
            }
            0x646f97 => {
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(6,32)); }
            pc = 0x646fda; continue;
            }
            0x646f9c => {
            { let a = c.get(1,32); c.sub(a,0x500u32,32); }
            if c.cond(4) { pc = 0x646fd3; continue; }
            pc = 0x646fa4; continue;
            }
            0x646fa4 => {
            { let a = c.get(1,32); c.sub(a,0x600u32,32); }
            if c.cond(4) { pc = 0x646fca; continue; }
            pc = 0x646fac; continue;
            }
            0x646fac => {
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x646fd3; continue; }
            pc = 0x646fb0; continue;
            }
            0x646fb0 => {
            c.push(m, 0x646fb5);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x16u32); }
            c.push(m, 0x646fc0);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x647176; continue;
            }
            0x646fca => {
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,0x5u32); }
            pc = 0x646fda; continue;
            }
            0x646fd3 => {
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,0x1u32); }
            pc = 0x646fda; continue;
            }
            0x646fda => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(6,32,0x80u32);
            { let a = c.get(7,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x646ff9; continue; }
            pc = 0x646fe6; continue;
            }
            0x646fe6 => {
            c.set(1,32,rm(m,0x6f0158u32,32));
            { let a = c.get(1,32); c.set(1,32,!a & 0xffffffffu32); }
            { let a = c.get(1,32); let r = c.and(a,rm(m,c.r[5].wrapping_add(0x14),32),32); c.set(1,32,r); }
            { let a = c.get(1,8); c.and(a,0x80u32,8); }
            if c.cond(5) { pc = 0x646ff9; continue; }
            pc = 0x646ff6; continue;
            }
            0x646ff6 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x646ff9; continue;
            }
            0x646ff9 => {
            { let a = c.get(0,8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x647007; continue; }
            pc = 0x646ffd; continue;
            }
            0x646ffd => {
            { let a = c.get(6,32); let r = c.or(a,0x4000000u32,32); c.set(6,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff6),8); let r = c.or(a,0x1u32,8); { let a = c.r[5].wrapping_add(0xfffffff6); wm(m,a,8,r); } }
            pc = 0x647007; continue;
            }
            0x647007 => {
            { let a = c.get(4,8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x64700e; continue; }
            pc = 0x64700c; continue;
            }
            0x64700c => {
            { let a = c.get(6,32); let r = c.or(a,c.get(7,32),32); c.set(6,32,r); }
            pc = 0x64700e; continue;
            }
            0x64700e => {
            { let a = c.get(0,8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x64701a; continue; }
            pc = 0x647012; continue;
            }
            0x647012 => {
            { let a = c.get(6,32); let r = c.or(a,0x8000000u32,32); c.set(6,32,r); }
            pc = 0x647024; continue;
            }
            0x64701a => {
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x647024; continue; }
            pc = 0x64701e; continue;
            }
            0x64701e => {
            { let a = c.get(6,32); let r = c.or(a,0x10000000u32,32); c.set(6,32,r); }
            pc = 0x647024; continue;
            }
            0x647024 => {
            c.push(m, 0x647029);
            { let args = []; c.r[0] = host.call(m, 0x6468fa, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x64704c; continue; }
            pc = 0x647032; continue;
            }
            0x647032 => {
            c.push(m, 0x647037);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x18u32); }
            c.push(m, 0x647042);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0],32); let r = c.and(a,0x0u32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            c.set(0,32,c.get(7,32));
            pc = 0x647176; continue;
            }
            0x64704c => {
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.push(m, v); }
            c.set(0,32,c.r[5].wrapping_add(0xffffffe4));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff0),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff4),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let target = rm(m,0x66a13cu32,32); c.push(m, 0x647065); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x64707f; continue; }
            pc = 0x64706b; continue;
            }
            0x64706b => {
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x647071); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x647077);
            { let args = []; c.r[0] = host.call(m, 0x63ccb5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(6,32,c.get(7,32));
            pc = 0x64716d; continue;
            }
            0x64707f => {
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a274u32,32); c.push(m, 0x647086); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x647093; continue; }
            pc = 0x64708a; continue;
            }
            0x64708a => {
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a150u32,32); c.push(m, 0x647091); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64706b; continue;
            }
            0x647093 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x64709e; continue; }
            pc = 0x647098; continue;
            }
            0x647098 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffff),8); let r = c.or(a,0x40u32,8); { let a = c.r[5].wrapping_add(0xffffffff); wm(m,a,8,r); } }
            pc = 0x6470a7; continue;
            }
            0x64709e => {
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(5) { pc = 0x6470a7; continue; }
            pc = 0x6470a3; continue;
            }
            0x6470a3 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffff),8); let r = c.or(a,0x8u32,8); { let a = c.r[5].wrapping_add(0xffffffff); wm(m,a,8,r); } }
            pc = 0x6470a7; continue;
            }
            0x6470a7 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x6470ae);
            { let args = []; c.r[0] = host.call(m, 0x646a1d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0xffffffff),8));
            c.set(7,32,c.r[0].wrapping_mul(4).wrapping_add(0x6f0940));
            c.set(0,32,c.get(3,32));
            { let a = c.get(1,8); let r = c.or(a,0x1u32,8); c.set(1,8,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xb); wm(m,a,8,c.get(1,8)); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xb),8); let r = c.and(a,0x48u32,8); { let a = c.r[5].wrapping_add(0xb); wm(m,a,8,r); } }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            if c.cond(5) { pc = 0x647154; continue; }
            pc = 0x6470dc; continue;
            }
            0x6470dc => {
            { let a = c.get(1,8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x647154; continue; }
            pc = 0x6470e1; continue;
            }
            0x6470e1 => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x647154; continue; }
            pc = 0x6470e7; continue;
            }
            0x6470e7 => {
            c.push(m,0x2u32);
            c.push(m,0xffffffffu32);
            c.push(m,c.get(3,32));
            c.push(m, 0x6470f1);
            { let args = []; c.r[0] = host.call(m, 0x645e0e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x647115; continue; }
            pc = 0x6470fc; continue;
            }
            0x6470fc => {
            c.push(m, 0x647101);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0],32); c.sub(a,0x83u32,32); }
            if c.cond(4) { pc = 0x647154; continue; }
            pc = 0x647109; continue;
            }
            0x647109 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x64710f);
            { let args = []; c.r[0] = host.call(m, 0x6435f8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(6,32); let r = c.or(a,0xffffffffu32,32); c.set(6,32,r); }
            pc = 0x64716d; continue;
            }
            0x647115 => {
            { let a = rm(m,c.r[5].wrapping_add(0x13),8); let r = c.and(a,0x0u32,8); { let a = c.r[5].wrapping_add(0x13); wm(m,a,8,r); } }
            c.set(0,32,c.r[5].wrapping_add(0x13));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x647125);
            { let args = []; c.r[0] = host.call(m, 0x646cd3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x647142; continue; }
            pc = 0x64712c; continue;
            }
            0x64712c => {
            { let a = rm(m,c.r[5].wrapping_add(0x13),8); c.sub(a,0x1au32,8); }
            if c.cond(5) { pc = 0x647142; continue; }
            pc = 0x647132; continue;
            }
            0x647132 => {
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff0),32); c.push(m, v); }
            c.push(m,c.get(3,32));
            c.push(m, 0x64713b);
            { let args = []; c.r[0] = host.call(m, 0x648ca9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x647109; continue; }
            pc = 0x647142; continue;
            }
            0x647142 => {
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x64714c);
            { let args = []; c.r[0] = host.call(m, 0x645e0e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x647109; continue; }
            pc = 0x647154; continue;
            }
            0x647154 => {
            { let a = rm(m,c.r[5].wrapping_add(0xb),8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x64716b; continue; }
            pc = 0x64715a; continue;
            }
            0x64715a => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),8); c.and(a,0x8u32,8); }
            if c.cond(4) { pc = 0x64716b; continue; }
            pc = 0x647160; continue;
            }
            0x647160 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4),8); let r = c.or(a,0x20u32,8); { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4); wm(m,a,8,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4));
            pc = 0x64716b; continue;
            }
            0x64716b => {
            c.set(6,32,c.get(3,32));
            pc = 0x64716d; continue;
            }
            0x64716d => {
            c.push(m,c.get(3,32));
            c.push(m, 0x647173);
            { let args = []; c.r[0] = host.call(m, 0x646bb9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,c.get(6,32));
            pc = 0x647176; continue;
            }
            0x647176 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ca70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ca70;
    loop {
        match pc {
            0x64ca70 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x64ca9c; continue; }
            pc = 0x64ca84; continue;
            }
            0x64ca84 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x64ca8a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,sx(rm(m,c.r[4].wrapping_add(0xc),8),8)&0xffffffffu32);
            c.set(2,32,rm(m,c.r[6],32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x64ca99);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x64ca9c; continue;
            }
            0x64ca9c => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064d030(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64d030;
    loop {
        match pc {
            0x64d030 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.get(1,32));
            c.push(m, 0x64d03b);
            { let args = []; c.r[0] = host.call(m, 0x64f1fc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[3],32));
            c.set(7,32,c.get(6,32));
            c.set(5,32,rm(m,c.r[6].wrapping_add(0xfffffff8),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(6,32),32); c.set(5,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(3) { pc = 0x64d06e; continue; }
            pc = 0x64d048; continue;
            }
            0x64d048 => {
            c.set(0,8,rm(m,c.r[6],8));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x64d05f; continue; }
            pc = 0x64d052; continue;
            }
            0x64d052 => {
            c.push(m,c.get(7,32));
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x64d05a);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(7,32,c.get(0,32));
            pc = 0x64d05f; continue;
            }
            0x64d05f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d065);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(2) { pc = 0x64d048; continue; }
            pc = 0x64d06e; continue;
            }
            0x64d06e => {
            { let a = c.r[7]; wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,c.r[3],32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xfffffff8),32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[1].wrapping_add(0xfffffff8); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064d910(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64d910;
    loop {
        match pc {
            0x64d910 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = 0x6d84d8u32; wm(m,a,32,0x6d84e0u32); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x64de57; continue; }
            pc = 0x64d940; continue;
            }
            0x64d940 => {
            c.set(0,8,rm(m,c.r[6],8));
            c.push(m,c.get(6,32));
            { let a = c.get(0,8); c.sub(a,0x25u32,8); }
            if c.cond(4) { pc = 0x64d981; continue; }
            pc = 0x64d947; continue;
            }
            0x64d947 => {
            c.push(m, 0x64d94c);
            { let args = []; c.r[0] = host.call(m, 0x63b895, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x64de3d; continue; }
            pc = 0x64d95f; continue;
            }
            0x64d95f => {
            c.set(1,32,c.get(6,32));
            pc = 0x64d961; continue;
            }
            0x64d961 => {
            c.set(7,32,rm(m,0x6d84d8u32,32));
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,0x6d84d8u32,32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x64d961; continue; }
            pc = 0x64d97c; continue;
            }
            0x64d97c => {
            pc = 0x64de3d; continue;
            }
            0x64d981 => {
            c.push(m, 0x64d986);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,0x25u32,8); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            if c.cond(5) { pc = 0x64d9b7; continue; }
            pc = 0x64d996; continue;
            }
            0x64d996 => {
            { let a = c.r[0]; wm(m,a,8,0x25u32); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0]; wm(m,a,8,0x25u32); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x64de3d; continue;
            }
            0x64d9b7 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.r[3].wrapping_mul(4).wrapping_add(0x6d8ce0); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xffffffff),8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d9d1);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.sub(a,0x24u32,8); }
            if c.cond(5) { pc = 0x64da46; continue; }
            pc = 0x64d9df; continue;
            }
            0x64d9df => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d9e5);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x64d9ed);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[3].wrapping_add(0x6d9560); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(0,8); c.sub(a,0x24u32,8); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x64da39; continue; }
            pc = 0x64da02; continue;
            }
            0x64da02 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x64da34; continue; }
            pc = 0x64da06; continue;
            }
            0x64da06 => {
            c.set(1,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x64da0f);
            { let args = []; c.r[0] = host.call(m, 0x63b806, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64da34; continue; }
            pc = 0x64da16; continue;
            }
            0x64da16 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64da1c);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,0x24u32,8); }
            if c.cond(5) { pc = 0x64da02; continue; }
            pc = 0x64da27; continue;
            }
            0x64da27 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64da2d);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            pc = 0x64da50; continue;
            }
            0x64da34 => {
            { let a = rm(m,c.r[6],8); c.sub(a,0x24u32,8); }
            if c.cond(5) { pc = 0x64da50; continue; }
            pc = 0x64da39; continue;
            }
            0x64da39 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64da3f);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            pc = 0x64da50; continue;
            }
            0x64da46 => {
            { let a = c.r[3].wrapping_add(0x6d9560); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(3,32)); }
            pc = 0x64da50; continue;
            }
            0x64da50 => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x64dabe; continue; }
            pc = 0x64da5b; continue;
            }
            0x64da5b => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            pc = 0x64da5e; continue;
            }
            0x64da5e => {
            c.set(2,32,rm(m,0x6d84d8u32,32));
            { let a = c.r[2]; wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64da71);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.sub(a,0x23u32,8); }
            if c.cond(5) { pc = 0x64da85; continue; }
            pc = 0x64da7f; continue;
            }
            0x64da7f => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff4),32); let r = c.add(a,0x2u32,32); { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,r); } }
            pc = 0x64daa3; continue;
            }
            0x64da85 => {
            { let a = c.get(0,8); c.sub(a,0x2au32,8); }
            if c.cond(5) { pc = 0x64da93; continue; }
            pc = 0x64da89; continue;
            }
            0x64da89 => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            pc = 0x64daa3; continue;
            }
            0x64da93 => {
            { let a = c.get(0,8); c.sub(a,0x2du32,8); }
            if c.cond(4) { pc = 0x64daa3; continue; }
            pc = 0x64da97; continue;
            }
            0x64da97 => {
            { let a = c.get(0,8); c.sub(a,0x2bu32,8); }
            if c.cond(4) { pc = 0x64daa3; continue; }
            pc = 0x64da9b; continue;
            }
            0x64da9b => {
            { let a = c.get(0,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x64daa3; continue; }
            pc = 0x64da9f; continue;
            }
            0x64da9f => {
            { let a = c.get(0,8); c.sub(a,0x20u32,8); }
            if c.cond(5) { pc = 0x64dab4; continue; }
            pc = 0x64daa3; continue;
            }
            0x64daa3 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64daa9);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x64da5e; continue; }
            pc = 0x64dab4; continue;
            }
            0x64dab4 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64db0a; continue; }
            pc = 0x64dabe; continue;
            }
            0x64dabe => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64dac4);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x64db0a; continue; }
            pc = 0x64dad0; continue;
            }
            0x64dad0 => {
            c.set(1,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x64dad9);
            { let args = []; c.r[0] = host.call(m, 0x63b806, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64db0a; continue; }
            pc = 0x64dae0; continue;
            }
            0x64dae0 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64dae6);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64dafc);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x64dad0; continue; }
            pc = 0x64db0a; continue;
            }
            0x64db0a => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,0x0u32); }
            { let a = c.get(0,8); c.sub(a,0x2eu32,8); }
            if c.cond(5) { pc = 0x64dbc8; continue; }
            pc = 0x64db1b; continue;
            }
            0x64db1b => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64db21);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6d84d8u32,32));
            c.set(6,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64db38);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.sub(a,0x2au32,8); }
            c.push(m,c.get(6,32));
            if c.cond(5) { pc = 0x64db7c; continue; }
            pc = 0x64db47; continue;
            }
            0x64db47 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x64db5b);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6d84d8u32,32));
            c.set(6,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64db72);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x64dbc8; continue;
            }
            0x64db7c => {
            c.push(m, 0x64db81);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x64dbc8; continue; }
            pc = 0x64db8d; continue;
            }
            0x64db8d => {
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x64db96);
            { let args = []; c.r[0] = host.call(m, 0x63b806, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64dbc8; continue; }
            pc = 0x64db9d; continue;
            }
            0x64db9d => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64dba3);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6d84d8u32,32));
            c.set(6,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64dbba);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x64db8d; continue; }
            pc = 0x64dbc8; continue;
            }
            0x64dbc8 => {
            c.push(m,0x3u32);
            c.push(m,0x6badf8u32);
            c.push(m,c.get(6,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m, 0x64dbd7);
            { let args = []; c.r[0] = host.call(m, 0x63b717, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64dbe8; continue; }
            pc = 0x64dbde; continue;
            }
            0x64dbde => {
            { let a = c.get(6,32); let r = c.add(a,0x3u32,32); c.set(6,32,r); }
            c.set(3,32,0x40000u32);
            pc = 0x64dc58; continue;
            }
            0x64dbe8 => {
            c.set(0,32,sx(rm(m,c.r[6],8),8)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,0xffffffbau32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x26u32,32); }
            if c.cond(7) { pc = 0x64dc58; continue; }
            pc = 0x64dbf3; continue;
            }
            0x64dbf3 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x64df08),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x64def8)); continue;
            }
            0x64dc02 => {
            c.push(m,c.get(6,32));
            c.set(3,32,0x10000u32);
            c.push(m, 0x64dc0d);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            pc = 0x64dc45; continue;
            }
            0x64dc1a => {
            c.push(m,c.get(6,32));
            c.set(3,32,0x20000u32);
            c.push(m, 0x64dc25);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6d84d8u32,32));
            c.set(6,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            pc = 0x64dc45; continue;
            }
            0x64dc33 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64dc39);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6d84d8u32,32));
            c.set(6,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            pc = 0x64dc45; continue;
            }
            0x64dc45 => {
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64dc50);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x64dc58; continue;
            }
            0x64dc58 => {
            c.set(0,32,sx(rm(m,c.r[6],8),8)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.or(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10063u32,32); }
            if c.cond(15) { pc = 0x64dcd7; continue; }
            pc = 0x64dc64; continue;
            }
            0x64dc64 => {
            if c.cond(4) { pc = 0x64dd28; continue; }
            pc = 0x64dc6a; continue;
            }
            0x64dc6a => {
            { let a = c.get(0,32); c.sub(a,0x73u32,32); }
            if c.cond(15) { pc = 0x64dcc9; continue; }
            pc = 0x64dc6f; continue;
            }
            0x64dc6f => {
            if c.cond(4) { pc = 0x64dca6; continue; }
            pc = 0x64dc71; continue;
            }
            0x64dc71 => {
            { let a = c.get(0,32); c.sub(a,0x43u32,32); }
            if c.cond(4) { pc = 0x64dd28; continue; }
            pc = 0x64dc7a; continue;
            }
            0x64dc7a => {
            { let a = c.get(0,32); c.sub(a,0x53u32,32); }
            if c.cond(4) { pc = 0x64dcfa; continue; }
            pc = 0x64dc7f; continue;
            }
            0x64dc7f => {
            { let a = c.get(0,32); c.sub(a,0x63u32,32); }
            pc = 0x64dc82; continue;
            }
            0x64dc82 => {
            if c.cond(4) { pc = 0x64dd28; continue; }
            pc = 0x64dc88; continue;
            }
            0x64dc88 => {
            c.set(0,32,sx(rm(m,c.r[6],8),8)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,0xffffffb9u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x31u32,32); }
            if c.cond(7) { pc = 0x64de32; continue; }
            pc = 0x64dc97; continue;
            }
            0x64dc97 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x64df48),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x64df30)); continue;
            }
            0x64dca6 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64dcc0; continue; }
            pc = 0x64dcb6; continue;
            }
            0x64dcb6 => {
            c.set(7,32,0x6u32);
            pc = 0x64dd40; continue;
            }
            0x64dcc0 => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a29cu32,32); c.push(m, 0x64dcc7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64dd1a; continue;
            }
            0x64dcc9 => {
            { let a = c.get(0,32); let r = c.sub(a,0x10043u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x64dd28; continue; }
            pc = 0x64dcd0; continue;
            }
            0x64dcd0 => {
            { let a = c.get(0,32); let r = c.sub(a,0x10u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x64dca6; continue; }
            pc = 0x64dcd5; continue;
            }
            0x64dcd5 => {
            pc = 0x64dc88; continue;
            }
            0x64dcd7 => {
            { let a = c.get(0,32); c.sub(a,0x20053u32,32); }
            if c.cond(15) { pc = 0x64dcee; continue; }
            pc = 0x64dcde; continue;
            }
            0x64dcde => {
            if c.cond(4) { pc = 0x64dcfa; continue; }
            pc = 0x64dce0; continue;
            }
            0x64dce0 => {
            { let a = c.get(0,32); c.sub(a,0x10073u32,32); }
            if c.cond(4) { pc = 0x64dca6; continue; }
            pc = 0x64dce7; continue;
            }
            0x64dce7 => {
            { let a = c.get(0,32); c.sub(a,0x20043u32,32); }
            pc = 0x64dc82; continue;
            }
            0x64dcee => {
            { let a = c.get(0,32); let r = c.sub(a,0x20063u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x64dd28; continue; }
            pc = 0x64dcf5; continue;
            }
            0x64dcf5 => {
            { let a = c.get(0,32); let r = c.sub(a,0x10u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x64dc88; continue; }
            pc = 0x64dcfa; continue;
            }
            0x64dcfa => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64dd11; continue; }
            pc = 0x64dd0a; continue;
            }
            0x64dd0a => {
            c.set(7,32,0x6u32);
            pc = 0x64dd40; continue;
            }
            0x64dd11 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x64dd17);
            { let args = []; c.r[0] = host.call(m, 0x63b6fa, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x64dd1a; continue;
            }
            0x64dd1a => {
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            if c.cond(13) { pc = 0x64dd38; continue; }
            pc = 0x64dd21; continue;
            }
            0x64dd21 => {
            c.set(7,32,0x1u32);
            pc = 0x64dd40; continue;
            }
            0x64dd28 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(7,32,0x2u32);
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x64dd40; continue;
            }
            0x64dd38 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x64dc88; continue; }
            pc = 0x64dd40; continue;
            }
            0x64dd40 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64dd4d; continue; }
            pc = 0x64dd47; continue;
            }
            0x64dd47 => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x64dd4d; continue; }
            pc = 0x64dd4b; continue;
            }
            0x64dd4b => {
            c.set(7,32,c.get(0,32));
            pc = 0x64dd4d; continue;
            }
            0x64dd4d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x64de32; continue; }
            pc = 0x64dd58; continue;
            }
            0x64dd58 => {
            c.set(7,32,c.get(0,32));
            pc = 0x64de32; continue;
            }
            0x64dd5f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(3,32); c.and(a,0x40000u32,32); }
            if c.cond(4) { pc = 0x64dd6f; continue; }
            pc = 0x64dd6a; continue;
            }
            0x64dd6a => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x64dd72; continue;
            }
            0x64dd6f => {
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            pc = 0x64dd72; continue;
            }
            0x64dd72 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            c.set(7,32,0x20u32);
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x64de32; continue; }
            pc = 0x64dd8a; continue;
            }
            0x64dd8a => {
            c.set(7,32,c.get(0,32));
            pc = 0x64de32; continue;
            }
            0x64dd91 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(2,32); let r = c.add(a,0x8u32,32); c.set(2,32,r); }
            c.set(7,32,0x80u32);
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x64de32; continue; }
            pc = 0x64ddaf; continue;
            }
            0x64ddaf => {
            c.set(7,32,c.get(0,32));
            pc = 0x64de32; continue;
            }
            0x64ddb3 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1]).wrapping_add(0x27c));
            c.set(1,32,c.r[2].wrapping_add(c.r[2]));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x64ddc9; continue; }
            pc = 0x64ddc7; continue;
            }
            0x64ddc7 => {
            c.set(0,32,c.get(1,32));
            pc = 0x64ddc9; continue;
            }
            0x64ddc9 => {
            { let a = c.get(0,32); let r = c.add(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfcu32,8); c.set(0,8,r); }
            c.push(m, 0x64ddd3);
            { let sz = c.r[0]; c.r[0] = host.call(m, 0x63b920, c.r[1], &[]); c.r[4] = c.r[4].wrapping_sub(sz); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            c.set(7,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.add(a,0x6u32,32); c.set(2,32,r); }
            c.fld(c.ld(m,c.r[0].wrapping_add(0xfffffff8),8));
            c.fstp64(m,c.r[5].wrapping_add(0xffffffe0));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,0x6badfcu32);
            c.push(m,c.get(7,32));
            c.push(m, 0x64de02);
            { let args = []; c.r[0] = host.call(m, 0x63b8ab, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x64de08);
            { let args = []; c.r[0] = host.call(m, 0x63b6fa, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.set(7,32,c.get(0,32));
            pc = 0x64de32; continue;
            }
            0x64de0f => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            c.set(7,32,0x20u32);
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[2]));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x64de32; continue; }
            pc = 0x64de2a; continue;
            }
            0x64de2a => {
            c.set(7,32,c.get(0,32));
            pc = 0x64de32; continue;
            }
            0x64de2e => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); let r = c.add(a,0x4u32,32); { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x64de32; continue;
            }
            0x64de32 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xfffffff0),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(0,32)); }
            pc = 0x64de3d; continue;
            }
            0x64de3d => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64de43);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x64d940; continue; }
            pc = 0x64de51; continue;
            }
            0x64de51 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x64de57; continue;
            }
            0x64de57 => {
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            { let a = c.r[0]; wm(m,a,8,0x0u32); }
            { let a = c.r[3].wrapping_mul(4).wrapping_add(0x6d8ce0); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x64dec6; continue; }
            pc = 0x64de6e; continue;
            }
            0x64de6e => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(6,32,0x6d8d60u32);
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x64dea2; continue; }
            pc = 0x64de79; continue;
            }
            0x64de79 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x6d9560),8));
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6d8ce0),32));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6d8ce4),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x64de9d; continue; }
            pc = 0x64de93; continue;
            }
            0x64de93 => {
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.r[6]; wm(m,a,8,c.get(2,8)); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x64de93; continue; }
            pc = 0x64de9d; continue;
            }
            0x64de9d => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x64de79; continue; }
            pc = 0x64dea2; continue;
            }
            0x64dea2 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64deb0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f637, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6],32));
            c.push(m,0x6d8d60u32);
            c.push(m,0x6d84e0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x64dec2);
            { let args = []; c.r[0] = host.call(m, 0x63b6a9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            pc = 0x64dee0; continue;
            }
            0x64dec6 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x64ded1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f637, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe8),32));
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x64dede);
            { let args = []; c.r[0] = host.call(m, 0x63b6a9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            pc = 0x64dee0; continue;
            }
            0x64dee0 => {
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m, 0x64deea);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f686, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(4,32,c.r[5].wrapping_add(0xffffffd4));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(4,32,c.get(5,32));
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e566(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e566;
    loop {
        match pc {
            0x64e566 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[5].wrapping_add(0xfffffff8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(6,32,c.get(1,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let target = rm(m,0x66a180u32,32); c.push(m, 0x64e57c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64e592; continue; }
            pc = 0x64e580; continue;
            }
            0x64e580 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffff8));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a17cu32,32); c.push(m, 0x64e58e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64e597; continue; }
            pc = 0x64e592; continue;
            }
            0x64e592 => {
            { let a = rm(m,c.r[6],32); let r = c.and(a,0x0u32,32); { let a = c.r[6]; wm(m,a,32,r); } }
            pc = 0x64e5ab; continue;
            }
            0x64e597 => {
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.set(0,32,c.r[5].wrapping_add(0xffffffe8));
            c.set(1,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m, 0x64e5a6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e51a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x64e5ab; continue;
            }
            0x64e5ab => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e662(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e662;
    loop {
        match pc {
            0x64e662 => {
            c.set(0,32,0x668cb8u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x64e66c; }
            c.push(m,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[1]; wm(m,a,32,0x67cf80u32); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m, 0x64e67f);
            { let args = []; c.r[0] = host.call(m, 0x64e642, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e7fc(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e7fc;
    loop {
        match pc {
            0x64e7fc => {
            c.set(0,32,0x668d1cu32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x64e806; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67d0f0u32); }
            { let v = rm(m,c.r[6].wrapping_add(0x8),32); c.push(m, v); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let v = rm(m,c.r[6].wrapping_add(0x4),32); c.push(m, v); }
            c.push(m, 0x64e822);
            { let args = []; c.r[0] = host.call(m, 0x64e83a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[6].wrapping_add(0x4),32); c.push(m, v); }
            c.push(m, 0x64e82a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e9d1(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e9d1;
    loop {
        match pc {
            0x64e9d1 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x64e9f5; continue; }
            pc = 0x64e9e6; continue;
            }
            0x64e9e6 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,0xffffffffu32);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x64e9f3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e85c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x64ea41; continue;
            }
            0x64e9f5 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64ea05);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e85c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.get(7,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.imul(a,0x3fffffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0xfffffffc),32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64ea30);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0xc),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x64ea3d);
            { let args = []; c.r[0] = host.call(m, 0x64e979, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x64ea41; continue;
            }
            0x64ea41 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064eb28(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64eb28;
    loop {
        match pc {
            0x64eb28 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4),32));
            pc = 0x64eb32; continue;
            }
            0x64eb32 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x64eb42; continue; }
            pc = 0x64eb36; continue;
            }
            0x64eb36 => {
            c.set(1,32,c.r[7].wrapping_add(0x8));
            c.push(m, 0x64eb3e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[7],32));
            pc = 0x64eb32; continue;
            }
            0x64eb42 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x64eb56);
            { let args = []; c.r[0] = host.call(m, 0x64ed07, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f637(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f637;
    loop {
        match pc {
            0x64f637 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = rm(m,c.r[0].wrapping_add(0xfffffff4),32); c.sub(a,0x1u32,32); }
            c.set(3,32,c.r[0].wrapping_add(0xfffffff4));
            if c.cond(15) { pc = 0x64f64f; continue; }
            pc = 0x64f64a; continue;
            }
            0x64f64a => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffffc),32),32); }
            if c.cond(14) { pc = 0x64f67f; continue; }
            pc = 0x64f64f; continue;
            }
            0x64f64f => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x64f659; continue; }
            pc = 0x64f657; continue;
            }
            0x64f657 => {
            c.set(1,32,c.get(7,32));
            pc = 0x64f659; continue;
            }
            0x64f659 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64f661);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[7].wrapping_add(0x1));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[3].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[6],32); c.push(m, v); }
            c.push(m, 0x64f670);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            { let a = c.r[0].wrapping_add(0xfffffff8); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x64f67e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f1bb, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x64f67f; continue;
            }
            0x64f67f => {
            c.set(0,32,rm(m,c.r[6],32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f686(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f686;
    loop {
        match pc {
            0x64f686 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x64f68e);
            { let args = []; c.r[0] = host.call(m, 0x64f1fc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x64f69f; continue; }
            pc = 0x64f697; continue;
            }
            0x64f697 => {
            { let v = rm(m,c.r[6],32); c.push(m, v); }
            { let target = rm(m,0x66a29cu32,32); c.push(m, 0x64f69f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64f69f; continue;
            }
            0x64f69f => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.r[1].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]),8); let r = c.and(a,0x0u32,8); { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,8,r); } }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f6ae(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f6ae;
    loop {
        match pc {
            0x64f6ae => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x64f6bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f637, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[0].wrapping_add(0xfffffff8); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]),8); let r = c.and(a,0x0u32,8); { let a = c.r[0].wrapping_add(c.r[7]); wm(m,a,8,r); } }
            c.set(0,32,rm(m,c.r[6],32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f71b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f71b;
    loop {
        match pc {
            0x64f71b => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x64f723);
            { let args = []; c.r[0] = host.call(m, 0x64f1fc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0xc),8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,8,c.get(1,8)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f967(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f967;
    loop {
        match pc {
            0x64f967 => {
            c.set(0,32,0x668788u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x64f971; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67b8fcu32); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[6].wrapping_add(0x1c),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x64f98d; continue; }
            pc = 0x64f988; continue;
            }
            0x64f988 => {
            c.push(m, 0x64f98d);
            { let args = []; c.r[0] = host.call(m, 0x650dba, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x64f98d; continue;
            }
            0x64f98d => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64f998);
            { let args = []; c.r[0] = host.call(m, 0x650bfd, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064fbd4(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64fbd4;
    loop {
        match pc {
            0x64fbd4 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x64fbdd);
            { let args = []; c.r[0] = host.call(m, 0x650184, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.r[6].wrapping_add(0x3c));
            c.push(m,0x20u32);
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            { let a = c.r[6]; wm(m,a,32,0x67b8fcu32); }
            c.push(m, 0x64fbf0);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,c.get(0,16));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064fde9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64fde9;
    loop {
        match pc {
            0x64fde9 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = rm(m,c.r[6].wrapping_add(0x24),8); c.and(a,0x18u32,8); }
            if c.cond(4) { pc = 0x64fdfb; continue; }
            pc = 0x64fdf2; continue;
            }
            0x64fdf2 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let v = rm(m,c.r[4].wrapping_add(0x8),32); c.push(m, v); }
            { let target = rm(m,c.r[0].wrapping_add(0x74),32); c.push(m, 0x64fdfb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64fdfb; continue;
            }
            0x64fdfb => {
            { let v = rm(m,c.r[4].wrapping_add(0x8),32); c.push(m, v); }
            { let v = rm(m,c.r[6].wrapping_add(0x1c),32); c.push(m, v); }
            { let target = rm(m,0x66a358u32,32); c.push(m, 0x64fe08); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ff0f(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ff0f;
    loop {
        match pc {
            0x64ff0f => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64ff21; continue; }
            pc = 0x64ff19; continue;
            }
            0x64ff19 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x64ff1f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x652748, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64ff2b; continue;
            }
            0x64ff21 => {
            { let v = rm(m,c.r[6].wrapping_add(0x40),32); c.push(m, v); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64ff2b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6526f8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64ff2b; continue;
            }
            0x64ff2b => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64ff3c; continue; }
            pc = 0x64ff2f; continue;
            }
            0x64ff2f => {
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64ff38);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65248b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64ff49; continue; }
            pc = 0x64ff3c; continue;
            }
            0x64ff3c => {
            c.push(m,0xffffffffu32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64ff45);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64fde9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x64ff49 => {
            c.push(m,0xe146u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64ff55);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6532e6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x64ff6f; continue; }
            pc = 0x64ff5b; continue;
            }
            0x64ff5b => {
            c.push(m, 0x64ff60);
            { let args = []; c.r[0] = host.call(m, 0x64feb9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0x5u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x64ff6f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65348d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64ff6f; continue;
            }
            0x64ff6f => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ff74(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ff74;
    loop {
        match pc {
            0x64ff74 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x1u32);
            c.push(m, 0x64ff7e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65248b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64ff8b; continue; }
            pc = 0x64ff82; continue;
            }
            0x64ff82 => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64ff8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64fde9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64ff8b; continue;
            }
            0x64ff8b => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00650519(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x650519;
    loop {
        match pc {
            0x650519 => {
            c.set(0,32,0x6687d2u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x650523; }
            c.push(m,c.get(6,32));
            c.push(m, 0x650529);
            { let args = []; c.r[0] = host.call(m, 0x6599a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = rm(m,c.r[6].wrapping_add(0x14),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x650579; continue; }
            pc = 0x650531; continue;
            }
            0x650531 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x650579; continue; }
            pc = 0x650537; continue;
            }
            0x650537 => {
            c.push(m,c.get(3,32));
            c.push(m,0x6560d6u32);
            c.push(m, 0x650542);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f7a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x48u32);
            c.set(3,32,c.get(0,32));
            c.push(m, 0x65054b);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,c.get(0,32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x650569; continue; }
            pc = 0x650559; continue;
            }
            0x650559 => {
            c.push(m,0x1u32);
            c.push(m,0x1cu32);
            c.push(m,0x67b9f8u32);
            c.push(m, 0x650567);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655300, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x65056b; continue;
            }
            0x650569 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x65056b; continue;
            }
            0x65056b => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x650578);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f7a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x650579; continue;
            }
            0x650579 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00650acb(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x650acb;
    loop {
        match pc {
            0x650acb => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x1c),32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x20),32));
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x24),32));
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x28),32));
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x2c),32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x650b15);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x30),32));
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x5c),32); c.push(m, 0x650b2c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x650b3e; continue; }
            pc = 0x650b30; continue;
            }
            0x650b30 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xa4),32); c.push(m, 0x650b3a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x650b8c; continue;
            }
            0x650b3e => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x650b45);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x650a3d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = rm(m,c.r[5].wrapping_add(0xffffffd0),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xffffffd4),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xffffffd8),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xffffffdc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xffffffe0),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xffffffe4),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xffffffe8),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xffffffec),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff0),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff4),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.push(m, v); }
            { let target = rm(m,0x66a3a4u32,32); c.push(m, 0x650b6f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(52);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x650b76);
            { let args = []; c.r[0] = host.call(m, 0x650a89, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x650b84; continue; }
            pc = 0x650b7a; continue;
            }
            0x650b7a => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xa4),32); c.push(m, 0x650b84); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x650b84; continue;
            }
            0x650b84 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x650b8c; continue;
            }
            0x650b8c => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(48); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00650bfd(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x650bfd;
    loop {
        match pc {
            0x650bfd => {
            c.set(0,32,0x6687f0u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x650c07; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67bdf0u32); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[6].wrapping_add(0x1c),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x650c43; continue; }
            pc = 0x650c1e; continue;
            }
            0x650c1e => {
            { let a = c.get(6,32); c.sub(a,0x6efac8u32,32); }
            if c.cond(4) { pc = 0x650c43; continue; }
            pc = 0x650c26; continue;
            }
            0x650c26 => {
            { let a = c.get(6,32); c.sub(a,0x6efa88u32,32); }
            if c.cond(4) { pc = 0x650c43; continue; }
            pc = 0x650c2e; continue;
            }
            0x650c2e => {
            { let a = c.get(6,32); c.sub(a,0x6efa48u32,32); }
            if c.cond(4) { pc = 0x650c43; continue; }
            pc = 0x650c36; continue;
            }
            0x650c36 => {
            { let a = c.get(6,32); c.sub(a,0x6efa08u32,32); }
            if c.cond(4) { pc = 0x650c43; continue; }
            pc = 0x650c3e; continue;
            }
            0x650c3e => {
            c.push(m, 0x650c43);
            { let args = []; c.r[0] = host.call(m, 0x650dba, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x650c43; continue;
            }
            0x650c43 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x34),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x650c51; continue; }
            pc = 0x650c4a; continue;
            }
            0x650c4a => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x4),32); c.push(m, 0x650c51); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x650c51; continue;
            }
            0x650c51 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x38),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x650c61; continue; }
            pc = 0x650c58; continue;
            }
            0x650c58 => {
            { let a = rm(m,c.r[0].wrapping_add(0x24),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x650c61; continue; }
            pc = 0x650c5d; continue;
            }
            0x650c5d => {
            { let a = rm(m,c.r[0].wrapping_add(0x24),32); let r = c.and(a,0x0u32,32); { let a = c.r[0].wrapping_add(0x24); wm(m,a,32,r); } }
            pc = 0x650c61; continue;
            }
            0x650c61 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x650c6c);
            { let args = []; c.r[0] = host.call(m, 0x652dd2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00650dba(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x650dba;
    loop {
        match pc {
            0x650dba => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = rm(m,c.r[6].wrapping_add(0x1c),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x650dc7; continue; }
            pc = 0x650dc3; continue;
            }
            0x650dc3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x650dc7 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m, 0x650dd0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x650519, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = rm(m,c.r[6].wrapping_add(0x1c),32); c.push(m, v); }
            c.set(1,32,c.r[0].wrapping_add(0x4));
            c.push(m, 0x650ddb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ee91, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x38),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x650def; continue; }
            pc = 0x650de4; continue;
            }
            0x650de4 => {
            { let v = rm(m,c.r[6].wrapping_add(0x1c),32); c.push(m, v); }
            { let target = rm(m,0x66a348u32,32); c.push(m, 0x650ded); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x650df4; continue;
            }
            0x650def => {
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x50),32); c.push(m, 0x650df4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x650df4; continue;
            }
            0x650df4 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(3,32,c.get(0,32));
            if c.cond(5) { pc = 0x650e01; continue; }
            pc = 0x650dfa; continue;
            }
            0x650dfa => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x650e01);
            { let args = []; c.r[0] = host.call(m, 0x650609, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x650e01; continue;
            }
            0x650e01 => {
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00651167(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x651167;
    loop {
        match pc {
            0x651167 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x651175);
            { let args = []; c.r[0] = host.call(m, 0x659754, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.add(a,0x58u32,32); c.set(6,32,r); }
            c.push(m, 0x65117f);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x6511a8; continue; }
            pc = 0x651189; continue;
            }
            0x651189 => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x6511a8; continue; }
            pc = 0x65118e; continue;
            }
            0x65118e => {
            { let a = rm(m,c.r[5].wrapping_add(0x14),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x6511a8; continue; }
            pc = 0x651193; continue;
            }
            0x651193 => {
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m,c.get(7,32));
            c.push(m,0x67bec0u32);
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a3d0u32,32); c.push(m, 0x6511a3); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x6511c2; continue;
            }
            0x6511a8 => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,c.get(3,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m,c.get(7,32));
            c.push(m,0x67beacu32);
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a3d0u32,32); c.push(m, 0x6511bf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x6511c2; continue;
            }
            0x6511c2 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffd8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a3ccu32,32); c.push(m, 0x6511ce); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x651212; continue; }
            pc = 0x6511d2; continue;
            }
            0x6511d2 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x66a3a8u32,32));
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffd8));
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x651209);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6510c6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x651212; continue; }
            pc = 0x65120d; continue;
            }
            0x65120d => {
            c.push(m, 0x651212);
            { let args = []; c.r[0] = host.call(m, 0x655e89, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x651212; continue;
            }
            0x651212 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00651b5b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x651b5b;
    loop {
        match pc {
            0x651b5b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(6,32,c.get(1,32));
            if c.cond(5) { pc = 0x651b6e; continue; }
            pc = 0x651b66; continue;
            }
            0x651b66 => {
            c.push(m, 0x651b6b);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x10),32));
            pc = 0x651b6e; continue;
            }
            0x651b6e => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x651b76; continue; }
            pc = 0x651b72; continue;
            }
            0x651b72 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x651b79; continue;
            }
            0x651b76 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            pc = 0x651b79; continue;
            }
            0x651b79 => {
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a414u32,32); c.push(m, 0x651b89); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065248b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65248b;
    loop {
        match pc {
            0x65248b => {
            c.set(0,32,0x668854u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x652495; }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffd4));
            c.push(m,c.get(6,32));
            c.push(m, 0x6524ac);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x65252d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m, 0x6524b1);
            { let args = []; c.r[0] = host.call(m, 0x659754, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xb8),32));
            c.set(0,32,c.r[7].wrapping_add(0xb8));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffd4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x84),32); c.push(m, 0x6524e0); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(72);
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x652510, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065366f(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65366f;
    loop {
        match pc {
            0x65366f => {
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x653678; continue; }
            pc = 0x653675; continue;
            }
            0x653675 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x653678 => {
            c.push(m, 0x65367d);
            { let args = []; c.r[0] = host.call(m, 0x653645, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a450u32,32); c.push(m, 0x653684); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653705(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653705;
    loop {
        match pc {
            0x653705 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            c.push(m, 0x653711);
            { let args = []; c.r[0] = host.call(m, 0x65371b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653743(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653743;
    loop {
        match pc {
            0x653743 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x65374a);
            { let args = []; c.r[0] = host.call(m, 0x65371b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(6,32,rm(m,c.r[7],32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x65375a; continue; }
            pc = 0x653755; continue;
            }
            0x653755 => {
            c.push(m, 0x65375a);
            { let args = []; c.r[0] = host.call(m, 0x65369e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x65375a; continue;
            }
            0x65375a => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653d3a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653d3a;
    loop {
        match pc {
            0x653d3a => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x653d47; continue; }
            pc = 0x653d43; continue;
            }
            0x653d43 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x653d70; continue;
            }
            0x653d47 => {
            c.set(0,32,c.r[5].wrapping_add(0xc));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let v = rm(m,c.r[1].wrapping_add(0x4),32); c.push(m, v); }
            { let target = rm(m,0x66a140u32,32); c.push(m, 0x653d5c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x653d6d; continue; }
            pc = 0x653d60; continue;
            }
            0x653d60 => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x653d67); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x653d6d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x653761, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x653d6d; continue;
            }
            0x653d6d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            pc = 0x653d70; continue;
            }
            0x653d70 => {
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653f17(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653f17;
    loop {
        match pc {
            0x653f17 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[6],32));
            { let target = rm(m,c.r[0].wrapping_add(0x28),32); c.push(m, 0x653f25); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x2u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x28),32); c.push(m, 0x653f32); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x28),32); c.push(m, 0x653f3e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653f67(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653f67;
    loop {
        match pc {
            0x653f67 => {
            c.set(0,32,0x668b4cu32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x653f71; }
            { let a = c.get(4,32); let r = c.sub(a,0x14cu32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            c.set(6,32,0x104u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let target = rm(m,0x66a170u32,32); c.push(m, 0x653f90); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x653fa6; continue; }
            pc = 0x653f94; continue;
            }
            0x653f94 => {
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a2d0u32,32); c.push(m, 0x653f9f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x654027; continue;
            }
            0x653fa6 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0x8));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x653fbd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x654037, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(6,32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffff0));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let target = rm(m,0x66a16cu32,32); c.push(m, 0x653fd3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x654019; continue; }
            pc = 0x653fd7; continue;
            }
            0x653fd7 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff0),8); c.and(a,0x2u32,8); }
            if c.cond(5) { pc = 0x653fe4; continue; }
            pc = 0x653fdd; continue;
            }
            0x653fdd => {
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a47cu32,32); c.push(m, 0x653fe4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x653fe4; continue;
            }
            0x653fe4 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff0),8); c.and(a,0x4u32,8); }
            if c.cond(5) { pc = 0x654016; continue; }
            pc = 0x653fea; continue;
            }
            0x653fea => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffea8));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let target = rm(m,0x66a168u32,32); c.push(m, 0x653ffa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x654016; continue; }
            pc = 0x653fff; continue;
            }
            0x653fff => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a164u32,32); c.push(m, 0x654006); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[5].wrapping_add(0xfffffed4));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0xffffffec),32); c.push(m, v); }
            { let target = rm(m,0x66a104u32,32); c.push(m, 0x654016); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x654016; continue;
            }
            0x654016 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x654019; continue;
            }
            0x654019 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.r[5].wrapping_add(0x8));
            c.push(m, 0x654025);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(6,32));
            pc = 0x654027; continue;
            }
            0x654027 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00654296(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x654296;
    loop {
        match pc {
            0x654296 => {
            c.set(0,32,0x668c5du32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x6542a0; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,0x68u32);
            c.push(m, 0x6542a9);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(6,32)); }
            if c.cond(4) { pc = 0x6542c5; continue; }
            pc = 0x6542b8; continue;
            }
            0x6542b8 => {
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x6542c3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x659cac, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(6,32,c.get(0,32));
            pc = 0x6542c5; continue;
            }
            0x6542c5 => {
            { let v = rm(m,c.r[5].wrapping_add(0x1c),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(0,8); let r = c.or(a,0x4u32,8); c.set(0,8,r); }
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6542dc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x65444b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x6542eb; continue; }
            pc = 0x6542e0; continue;
            }
            0x6542e0 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x78),32); c.push(m, 0x6542e7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x654308; continue;
            }
            0x6542eb => {
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[6].wrapping_add(0x28),32); c.push(m, v); }
            { let target = rm(m,0x66a18cu32,32); c.push(m, 0x6542f7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[5].wrapping_add(0x18),8); c.and(a,0x4u32,8); }
            if c.cond(5) { pc = 0x654306; continue; }
            pc = 0x6542fd; continue;
            }
            0x6542fd => {
            { let v = rm(m,c.r[6].wrapping_add(0x28),32); c.push(m, v); }
            { let target = rm(m,0x66a188u32,32); c.push(m, 0x654306); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x654306; continue;
            }
            0x654306 => {
            c.set(0,32,c.get(6,32));
            pc = 0x654308; continue;
            }
            0x654308 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00654c58(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x654c58;
    loop {
        match pc {
            0x654c58 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x104u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,0x100u32);
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffefc));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x654c7b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x654cdc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(6) { pc = 0x654c9e; continue; }
            pc = 0x654c86; continue;
            }
            0x654c86 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffefc));
            c.push(m,c.get(0,32));
            c.push(m, 0x654c95);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            pc = 0x654cd6; continue;
            }
            0x654c9e => {
            c.push(m,c.get(3,32));
            c.set(7,32,c.get(6,32));
            pc = 0x654ca1; continue;
            }
            0x654ca1 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(6,32),32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[7].wrapping_add(0xffffffff));
            c.push(m,c.get(0,32));
            c.push(m, 0x654cb0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f637, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x654cb9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x654cdc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(3,32,c.get(0,32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(14) { pc = 0x654ca1; continue; }
            pc = 0x654cc4; continue;
            }
            0x654cc4 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            c.push(m,0xffffffffu32);
            c.push(m, 0x654cce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f686, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x654cd6; continue;
            }
            0x654cd6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00655365(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x655365;
    loop {
        match pc {
            0x655365 => {
            c.set(0,32,0x668d94u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x65536f; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            c.set(6,32,c.get(1,32));
            if c.cond(5) { pc = 0x655393; continue; }
            pc = 0x655380; continue;
            }
            0x655380 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x655382; continue;
            }
            0x655382 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x655393 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x4));
            c.push(m, 0x65539c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ee91, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x655382; continue; }
            pc = 0x6553a0; continue;
            }
            0x6553a0 => {
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m,c.get(7,32));
            c.push(m, 0x6553a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ee91, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6553c0; continue; }
            pc = 0x6553ad; continue;
            }
            0x6553ad => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x44),32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x655382; continue; }
            pc = 0x6553bb; continue;
            }
            0x6553bb => {
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            pc = 0x655382; continue;
            }
            0x6553c0 => {
            c.push(m,0x6560d6u32);
            c.push(m, 0x6553ca);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f7a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x3c),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x6553d9);
            { let args = []; c.r[0] = host.call(m, 0x654dad, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x6553e4; continue; }
            pc = 0x6553df; continue;
            }
            0x6553df => {
            c.push(m, 0x6553e4);
            { let args = []; c.r[0] = host.call(m, 0x64e49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x6553e4; continue;
            }
            0x6553e4 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x20));
            c.push(m, 0x6553ed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64eec4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x6553fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f7a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x40),32));
            { let a = c.r[0].wrapping_add(c.r[3]); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x44),32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x65540c; continue; }
            pc = 0x655409; continue;
            }
            0x655409 => {
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            pc = 0x65540c; continue;
            }
            0x65540c => {
            c.set(0,32,c.get(3,32));
            pc = 0x655382; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00655cd4(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x655cd4;
    loop {
        match pc {
            0x655cd4 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x655cf4; continue; }
            pc = 0x655cde; continue;
            }
            0x655cde => {
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x655cf4; continue; }
            pc = 0x655ce3; continue;
            }
            0x655ce3 => {
            c.push(m,0x0u32);
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a098u32,32); c.push(m, 0x655cf4); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x655cf4; continue;
            }
            0x655cf4 => {
            { let v = rm(m,c.r[4].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[6].wrapping_add(0x4),32); c.push(m, v); }
            { let target = rm(m,0x66a09cu32,32); c.push(m, 0x655d05); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00655d09(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x655d09;
    loop {
        match pc {
            0x655d09 => {
            c.set(0,32,0x668a84u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x655d13; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x655d1f);
            { let args = []; c.r[0] = host.call(m, 0x6557a8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.r[6]; wm(m,a,32,0x67cb78u32); }
            c.set(1,32,c.r[6].wrapping_add(0x14));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x66a468u32,32); c.push(m, 0x655d3d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x655d45);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65585f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x655d4e; continue; }
            pc = 0x655d49; continue;
            }
            0x655d49 => {
            c.push(m, 0x655d4e);
            { let args = []; c.r[0] = host.call(m, 0x655e89, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x655d4e; continue;
            }
            0x655d4e => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00655d7b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x655d7b;
    loop {
        match pc {
            0x655d7b => {
            c.set(0,32,0x668a98u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x655d85; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67cb78u32); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(0,32,c.r[6].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[6].wrapping_add(0x10),32); c.push(m, v); }
            { let target = rm(m,0x66a46cu32,32); c.push(m, 0x655da3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x655daa);
            { let args = []; c.r[0] = host.call(m, 0x655896, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x655db5);
            { let args = []; c.r[0] = host.call(m, 0x6558c7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656860(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656860;
    loop {
        match pc {
            0x656860 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[0],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x656870; continue; }
            pc = 0x65686b; continue;
            }
            0x65686b => {
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            pc = 0x656875; continue;
            }
            0x656870 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let v = rm(m,c.r[1],32); c.push(m, v); }
            pc = 0x656875; continue;
            }
            0x656875 => {
            c.push(m,0xf110u32);
            c.push(m,0x67c3a8u32);
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x656888);
            { let args = []; c.r[0] = host.call(m, 0x6567f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065688f(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65688f;
    loop {
        match pc {
            0x65688f => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[0],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x65689f; continue; }
            pc = 0x65689a; continue;
            }
            0x65689a => {
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            pc = 0x6568a4; continue;
            }
            0x65689f => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let v = rm(m,c.r[1],32); c.push(m, v); }
            pc = 0x6568a4; continue;
            }
            0x6568a4 => {
            c.push(m,0xf117u32);
            c.push(m,0x67c3acu32);
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x6568b7);
            { let args = []; c.r[0] = host.call(m, 0x6567f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006568be(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6568be;
    loop {
        match pc {
            0x6568be => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x6568d0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6566d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            c.set(6,32,c.get(0,32));
            if c.cond(4) { pc = 0x6568ff; continue; }
            pc = 0x6568d7; continue;
            }
            0x6568d7 => {
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a3b4u32,32); c.push(m, 0x6568de); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[0].wrapping_add(0x1));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6568eb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f6ae, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a3b0u32,32); c.push(m, 0x6568f3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.push(m,0xffffffffu32);
            c.push(m, 0x6568fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f686, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x65690a; continue;
            }
            0x6568ff => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let v = rm(m,c.r[0],32); c.push(m, v); }
            c.push(m,c.get(6,32));
            c.push(m, 0x65690a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655fb8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x65690a; continue;
            }
            0x65690a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656910(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656910;
    loop {
        match pc {
            0x656910 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m, 0x656921);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6566e7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[6],32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x65693d; continue; }
            pc = 0x656927; continue;
            }
            0x656927 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(7,32));
            c.push(m,0xf0u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4a0u32,32); c.push(m, 0x656935); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            pc = 0x65695d; continue;
            }
            0x65693d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x65694c; continue; }
            pc = 0x656947; continue;
            }
            0x656947 => {
            { let a = c.get(2,32); c.sub(a,0x2u32,32); }
            if c.cond(14) { pc = 0x65694e; continue; }
            pc = 0x65694c; continue;
            }
            0x65694c => {
            { let a = c.r[1]; wm(m,a,32,c.get(7,32)); }
            pc = 0x65694e; continue;
            }
            0x65694e => {
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[1],32); c.push(m, v); }
            c.push(m,0xf1u32);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a4a0u32,32); c.push(m, 0x65695d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x65695d; continue;
            }
            0x65695d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656962(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656962;
    loop {
        match pc {
            0x656962 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x656975);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6566e7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(6,32,c.get(0,32));
            { let a = rm(m,c.r[7],32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x656984; continue; }
            pc = 0x65697d; continue;
            }
            0x65697d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = rm(m,c.r[0],32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            pc = 0x656984; continue;
            }
            0x656984 => {
            c.set(7,32,rm(m,0x66a4a0u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x65698c; continue;
            }
            0x65698c => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(5,32));
            c.push(m,0x87u32);
            c.push(m,c.get(6,32));
            { let target = c.get(7,32); c.push(m, 0x656996); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x6569cd; continue; }
            pc = 0x65699a; continue;
            }
            0x65699a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(5,32));
            { let a = rm(m,c.r[0],32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x6569b8; continue; }
            pc = 0x6569a3; continue;
            }
            0x6569a3 => {
            c.push(m,c.get(5,32));
            c.push(m,0xf0u32);
            c.push(m,c.get(6,32));
            { let target = c.get(7,32); c.push(m, 0x6569ac); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6569cc; continue; }
            pc = 0x6569b0; continue;
            }
            0x6569b0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            pc = 0x6569cc; continue;
            }
            0x6569b8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[1],32),32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            c.push(m,0xf1u32);
            c.push(m,c.get(6,32));
            { let target = c.get(7,32); c.push(m, 0x6569cc); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x6569cc; continue;
            }
            0x6569cc => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            pc = 0x6569cd; continue;
            }
            0x6569cd => {
            c.push(m,0x2u32);
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a36cu32,32); c.push(m, 0x6569d6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x6569ec; continue; }
            pc = 0x6569dc; continue;
            }
            0x6569dc => {
            c.push(m,0xfffffff0u32);
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a510u32,32); c.push(m, 0x6569e5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,0x20000u32,32); }
            if c.cond(4) { pc = 0x65698c; continue; }
            pc = 0x6569ec; continue;
            }
            0x6569ec => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006569f3(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6569f3;
    loop {
        match pc {
            0x6569f3 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = rm(m,c.r[7].wrapping_add(0x1c),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x656a3a; continue; }
            pc = 0x6569ff; continue;
            }
            0x6569ff => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x656a0e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6566e7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x656a16);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x652d66, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x656a21; continue; }
            pc = 0x656a1a; continue;
            }
            0x656a1a => {
            c.push(m, 0x656a1f);
            { let args = []; c.r[0] = host.call(m, 0x64e4b5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x656a3a; continue;
            }
            0x656a21 => {
            { let v = rm(m,c.r[7].wrapping_add(0x1c),32); c.push(m, v); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let target = rm(m,0x66a344u32,32); c.push(m, 0x656a2d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x1c),32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x656a3a; continue; }
            pc = 0x656a32; continue;
            }
            0x656a32 => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x656a3a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x653554, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x656a3a; continue;
            }
            0x656a3a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656ca5(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656ca5;
    loop {
        match pc {
            0x656ca5 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m, 0x656cad);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x656cc9; continue; }
            pc = 0x656cb4; continue;
            }
            0x656cb4 => {
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let target = rm(m,c.r[2].wrapping_add(0x8c),32); c.push(m, 0x656cc7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x656cd9; continue;
            }
            0x656cc9 => {
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x656cd9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x656bbe, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x656cd9; continue;
            }
            0x656cd9 => {
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656fa3(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656fa3;
    loop {
        match pc {
            0x656fa3 => {
            c.set(0,32,0x6688ccu32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x656fad; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67c14cu32); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m, 0x656fc3);
            { let args = []; c.r[0] = host.call(m, 0x650dba, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x656fce);
            { let args = []; c.r[0] = host.call(m, 0x650bfd, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656fdb(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656fdb;
    loop {
        match pc {
            0x656fdb => {
            c.set(0,32,0x6688e0u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x656fe5; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67c208u32); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m, 0x656ffb);
            { let args = []; c.r[0] = host.call(m, 0x650dba, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x657006);
            { let args = []; c.r[0] = host.call(m, 0x650bfd, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006570b4(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6570b4;
    loop {
        match pc {
            0x6570b4 => {
            c.set(0,32,0x6688f4u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x6570be; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67c2dcu32); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m, 0x6570d4);
            { let args = []; c.r[0] = host.call(m, 0x650dba, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x6570df);
            { let args = []; c.r[0] = host.call(m, 0x650bfd, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006571ce(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6571ce;
    loop {
        match pc {
            0x6571ce => {
            c.set(0,32,0x668921u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x6571d8; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x6571e5);
            { let args = []; c.r[0] = host.call(m, 0x650184, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,0xau32);
            c.set(1,32,c.r[6].wrapping_add(0x6c));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x6571f4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64e603, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6].wrapping_add(0xac); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[6].wrapping_add(0x40),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[6].wrapping_add(0x40); wm(m,a,32,r); } }
            c.set(0,32,c.r[6].wrapping_add(0x58));
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            { let a = c.r[6]; wm(m,a,32,0x67c85cu32); }
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,32,0x1u32); }
            { let a = c.r[6].wrapping_add(0x9c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x44); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x8c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x90); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x68); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xa8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xb8); wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,0x66a4d4u32,32); c.push(m, 0x65725d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = rm(m,c.r[6].wrapping_add(0x88),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[6].wrapping_add(0x88); wm(m,a,32,r); } }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x54); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xb0); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x657280);
            { let args = []; c.r[0] = host.call(m, 0x65732c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006572ac(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6572ac;
    loop {
        match pc {
            0x6572ac => {
            c.set(0,32,0x66894du32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x6572b6; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67c85cu32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x2u32); }
            c.push(m, 0x6572cf);
            { let args = []; c.r[0] = host.call(m, 0x657350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m, 0x6572d4);
            { let args = []; c.r[0] = host.call(m, 0x659754, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0].wrapping_add(0xc4),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x6572e8; continue; }
            pc = 0x6572dc; continue;
            }
            0x6572dc => {
            c.push(m, 0x6572e1);
            { let args = []; c.r[0] = host.call(m, 0x659754, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0].wrapping_add(0xc4),32); let r = c.and(a,0x0u32,32); { let a = c.r[0].wrapping_add(0xc4); wm(m,a,32,r); } }
            pc = 0x6572e8; continue;
            }
            0x6572e8 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xa4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6572f9; continue; }
            pc = 0x6572f2; continue;
            }
            0x6572f2 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x6572f8);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x6572f9; continue;
            }
            0x6572f9 => {
            c.set(1,32,c.r[6].wrapping_add(0xac));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            c.push(m, 0x657308);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); let r = c.and(a,0x0u32,8); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,r); } }
            c.set(1,32,c.r[6].wrapping_add(0x6c));
            c.push(m, 0x657314);
            { let args = []; c.r[0] = host.call(m, 0x64e662, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x65731f);
            { let args = []; c.r[0] = host.call(m, 0x650bfd, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00659ddb(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x659ddb;
    loop {
        match pc {
            0x659ddb => {
            c.set(0,32,0x668ce0u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x659de5; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x659df3);
            { let args = []; c.r[0] = host.call(m, 0x659d0c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[6]; wm(m,a,32,0x67d040u32); }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x659e11; continue; }
            pc = 0x659e03; continue;
            }
            0x659e03 => {
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x659e0b);
            { let args = []; c.r[0] = host.call(m, 0x63e161, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[6].wrapping_add(0x78); wm(m,a,32,c.get(0,32)); }
            pc = 0x659e14; continue;
            }
            0x659e11 => {
            { let a = c.r[6].wrapping_add(0x78); wm(m,a,32,c.get(7,32)); }
            pc = 0x659e14; continue;
            }
            0x659e14 => {
            c.push(m, 0x659e19);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.push(m,0x6595b2u32);
            c.set(1,32,c.r[3].wrapping_add(0x1070));
            c.push(m, 0x659e2b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a5cf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let target = rm(m,0x66a198u32,32); c.push(m, 0x659e34); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x66a11cu32,32); c.push(m, 0x659e3d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6].wrapping_add(0x68); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x8c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x90); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x7c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x88); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xa8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x80); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xb2); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[6].wrapping_add(0xb0); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[6].wrapping_add(0x70); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xac); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xa0); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xa4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x94); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x98); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xb4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xbc); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x84); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xb8); wm(m,a,32,0x200u32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00659f25(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x659f25;
    loop {
        match pc {
            0x659f25 => {
            c.set(0,32,0x668cf4u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x659f2f; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67d040u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x80),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x659f53; continue; }
            pc = 0x659f4c; continue;
            }
            0x659f4c => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x4),32); c.push(m, 0x659f53); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x659f53; continue;
            }
            0x659f53 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xa8),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x659f64; continue; }
            pc = 0x659f5d; continue;
            }
            0x659f5d => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x14),32); c.push(m, 0x659f64); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x659f64; continue;
            }
            0x659f64 => {
            c.push(m, 0x659f69);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0].wrapping_add(0x14),8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x659f9c; continue; }
            pc = 0x659f6e; continue;
            }
            0x659f6e => {
            c.set(1,32,rm(m,0x6efe38u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x659f85; continue; }
            pc = 0x659f78; continue;
            }
            0x659f78 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x4),32); c.push(m, 0x659f7f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x6efe38u32; wm(m,a,32,c.get(3,32)); }
            pc = 0x659f85; continue;
            }
            0x659f85 => {
            c.set(1,32,rm(m,0x6efe34u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x659f9c; continue; }
            pc = 0x659f8f; continue;
            }
            0x659f8f => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x4),32); c.push(m, 0x659f96); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = 0x6efe34u32; wm(m,a,32,c.get(3,32)); }
            pc = 0x659f9c; continue;
            }
            0x659f9c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x94),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x659fac; continue; }
            pc = 0x659fa6; continue;
            }
            0x659fa6 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x659fac);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6560a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x659fac; continue;
            }
            0x659fac => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x98),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x659fbc; continue; }
            pc = 0x659fb6; continue;
            }
            0x659fb6 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x659fbc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6560a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x659fbc; continue;
            }
            0x659fbc => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xb0),16));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x66a108u32,32));
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x659fd2; continue; }
            pc = 0x659fcf; continue;
            }
            0x659fcf => {
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x659fd2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x659fd2; continue;
            }
            0x659fd2 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xb2),16));
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x659fe1; continue; }
            pc = 0x659fde; continue;
            }
            0x659fde => {
            c.push(m,c.get(0,32));
            { let target = c.get(7,32); c.push(m, 0x659fe1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x659fe1; continue;
            }
            0x659fe1 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xac),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x659ff3; continue; }
            pc = 0x659fec; continue;
            }
            0x659fec => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x4),32); c.push(m, 0x659ff3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x659ff3; continue;
            }
            0x659ff3 => {
            c.push(m, 0x659ff8);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x10),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x78),32),32); }
            if c.cond(5) { pc = 0x65a003; continue; }
            pc = 0x65a000; continue;
            }
            0x65a000 => {
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x65a003; continue;
            }
            0x65a003 => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x65a00b; continue; }
            pc = 0x65a008; continue;
            }
            0x65a008 => {
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            pc = 0x65a00b; continue;
            }
            0x65a00b => {
            { let v = rm(m,c.r[6].wrapping_add(0x78),32); c.push(m, v); }
            c.push(m, 0x65a013);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[6].wrapping_add(0x7c),32); c.push(m, v); }
            c.push(m, 0x65a01b);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[6].wrapping_add(0x88),32); c.push(m, v); }
            c.push(m, 0x65a026);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[6].wrapping_add(0x8c),32); c.push(m, v); }
            c.push(m, 0x65a031);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[6].wrapping_add(0x90),32); c.push(m, v); }
            c.push(m, 0x65a03c);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x65a04d);
            { let args = []; c.r[0] = host.call(m, 0x65ad40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065a5cf(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65a5cf;
    loop {
        match pc {
            0x65a5cf => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x65a605; continue; }
            pc = 0x65a5d8; continue;
            }
            0x65a5d8 => {
            c.set(1,32,rm(m,0x6efe50u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x65a5fe; continue; }
            pc = 0x65a5e2; continue;
            }
            0x65a5e2 => {
            c.set(1,32,0x6efe54u32);
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x65a5f6; continue; }
            pc = 0x65a5ed; continue;
            }
            0x65a5ed => {
            c.push(m, 0x65a5f2);
            { let args = []; c.r[0] = host.call(m, 0x65a1cf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            pc = 0x65a5f8; continue;
            }
            0x65a5f6 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x65a5f8; continue;
            }
            0x65a5f8 => {
            { let a = 0x6efe50u32; wm(m,a,32,c.get(1,32)); }
            pc = 0x65a5fe; continue;
            }
            0x65a5fe => {
            c.push(m, 0x65a603);
            { let args = []; c.r[0] = host.call(m, 0x65a268, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            pc = 0x65a605; continue;
            }
            0x65a605 => {
            c.set(0,32,rm(m,0x6efe50u32,32));
            c.set(6,32,rm(m,c.r[7],32));
            { let v = rm(m,c.r[0],32); c.push(m, v); }
            { let target = rm(m,0x66a1d4u32,32); c.push(m, 0x65a614); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x65a625; continue; }
            pc = 0x65a618; continue;
            }
            0x65a618 => {
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x8),32),32); }
            if c.cond(13) { pc = 0x65a625; continue; }
            pc = 0x65a61d; continue;
            }
            0x65a61d => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32));
            pc = 0x65a627; continue;
            }
            0x65a625 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x65a627; continue;
            }
            0x65a627 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x65a63f; continue; }
            pc = 0x65a62b; continue;
            }
            0x65a62b => {
            { let target = rm(m,c.r[4].wrapping_add(0xc),32); c.push(m, 0x65a62f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6efe50u32,32));
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[7],32); c.push(m, v); }
            c.push(m, 0x65a63f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x65a3d7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x65a63f; continue;
            }
            0x65a63f => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065a691(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65a691;
    loop {
        match pc {
            0x65a691 => {
            c.set(0,32,0x668d68u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x65a69b; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            { let a = rm(m,c.r[6],32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x65a6c9; continue; }
            pc = 0x65a6ab; continue;
            }
            0x65a6ab => {
            c.push(m,0x10u32);
            c.push(m, 0x65a6b2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a9f9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(7,32)); }
            { let a = rm(m,c.r[6],32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x65a6be; continue; }
            pc = 0x65a6b9; continue;
            }
            0x65a6b9 => {
            { let target = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, 0x65a6bc); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x65a6be; continue;
            }
            0x65a6be => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m,0x10u32);
            c.push(m, 0x65a6c9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65aa69, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x65a6c9; continue;
            }
            0x65a6c9 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00652dd2(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x652dd2;
    loop {
        match pc {
            0x652dd2 => {
            c.set(0,32,0x6688b8u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x652ddc; }
            c.push(m,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x10),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x652df1; continue; }
            pc = 0x652dee; continue;
            }
            0x652dee => {
            { let target = rm(m,c.r[0].wrapping_add(0x1c),32); c.push(m, 0x652df1); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x652df1; continue;
            }
            0x652df1 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006599a7(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6599a7;
    loop {
        match pc {
            0x6599a7 => {
            c.push(m, 0x6599ac);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0x1070u32,32); c.set(0,32,r); }
            c.push(m,0x6595b2u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x6599bd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a5cf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00650609(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x650609;
    loop {
        match pc {
            0x650609 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x65062e; continue; }
            pc = 0x650614; continue;
            }
            0x650614 => {
            c.push(m,0x0u32);
            c.push(m, 0x65061b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x650519, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x65062a; continue; }
            pc = 0x65061f; continue;
            }
            0x65061f => {
            { let v = rm(m,c.r[6].wrapping_add(0x1c),32); c.push(m, v); }
            c.set(1,32,c.r[0].wrapping_add(0x4));
            c.push(m, 0x65062a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ef14, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x65062a; continue;
            }
            0x65062a => {
            { let a = rm(m,c.r[6].wrapping_add(0x1c),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,r); } }
            pc = 0x65062e; continue;
            }
            0x65062e => {
            { let a = rm(m,c.r[6].wrapping_add(0x38),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x38); wm(m,a,32,r); } }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f7a7(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f7a7;
    loop {
        match pc {
            0x64f7a7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,32,rm(m,0x6c7ce4u32,32));
            { let a = 0x6c7ce4u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00659cac(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x659cac;
    loop {
        match pc {
            0x659cac => {
            c.set(0,32,0x668c7cu32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x659cb6; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x659cc2);
            { let args = []; c.r[0] = host.call(m, 0x652db8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6]; wm(m,a,32,0x67ced0u32); }
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x659cdf);
            { let args = []; c.r[0] = host.call(m, 0x659d49, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00659d0c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x659d0c;
    loop {
        match pc {
            0x659d0c => {
            c.set(0,32,0x668c90u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x659d16; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x659d22);
            { let args = []; c.r[0] = host.call(m, 0x652db8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67ced0u32); }
            { let a = c.r[6].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x659d3a);
            { let args = []; c.r[0] = host.call(m, 0x659d49, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00659754(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x659754;
    loop {
        match pc {
            0x659754 => {
            c.push(m,0x6593f0u32);
            c.set(1,32,0x6efb74u32);
            c.push(m, 0x659763);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a5cf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065ad40(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65ad40;
    loop {
        match pc {
            0x65ad40 => {
            c.set(0,32,0x668ca4u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x65ad4a; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67ced0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x28),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x65ad69; continue; }
            pc = 0x65ad62; continue;
            }
            0x65ad62 => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a150u32,32); c.push(m, 0x65ad69); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x65ad69; continue;
            }
            0x65ad69 => {
            c.push(m, 0x65ad6e);
            { let args = []; c.r[0] = host.call(m, 0x6599a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x65ad77; continue; }
            pc = 0x65ad73; continue;
            }
            0x65ad73 => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,r); } }
            pc = 0x65ad77; continue;
            }
            0x65ad77 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x65ad82);
            { let args = []; c.r[0] = host.call(m, 0x652dd2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
