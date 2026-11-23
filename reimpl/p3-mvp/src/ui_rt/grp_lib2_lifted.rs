//! Generado por `reimpl/tools/ui_rt_grp.py lib2` con `tools/lift.py` (no editar a mano): grupo `lib2`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_LIB2: [(u32, super::core_table::CoreFn); 125] = [
    (0x548860, f_00548860),
    (0x54b670, f_0054b670),
    (0x63bdef, f_0063bdef),
    (0x63d118, f_0063d118),
    (0x63d791, f_0063d791),
    (0x63e161, f_0063e161),
    (0x642a2e, f_00642a2e),
    (0x643554, f_00643554),
    (0x643583, f_00643583),
    (0x6435d5, f_006435d5),
    (0x6435f8, f_006435f8),
    (0x643869, f_00643869),
    (0x644ab4, f_00644ab4),
    (0x644be7, f_00644be7),
    (0x644c81, f_00644c81),
    (0x644f36, f_00644f36),
    (0x6468fa, f_006468fa),
    (0x646a1d, f_00646a1d),
    (0x646cd3, f_00646cd3),
    (0x648ca9, f_00648ca9),
    (0x64e51a, f_0064e51a),
    (0x64e725, f_0064e725),
    (0x64eec4, f_0064eec4),
    (0x64ef14, f_0064ef14),
    (0x64feb9, f_0064feb9),
    (0x650a3d, f_00650a3d),
    (0x650a89, f_00650a89),
    (0x6510c6, f_006510c6),
    (0x65252d, f_0065252d),
    (0x6526f8, f_006526f8),
    (0x652748, f_00652748),
    (0x652d66, f_00652d66),
    (0x6532e6, f_006532e6),
    (0x65348d, f_0065348d),
    (0x653554, f_00653554),
    (0x653645, f_00653645),
    (0x65369e, f_0065369e),
    (0x65371b, f_0065371b),
    (0x654037, f_00654037),
    (0x65444b, f_0065444b),
    (0x654dad, f_00654dad),
    (0x655300, f_00655300),
    (0x6557a8, f_006557a8),
    (0x65585f, f_0065585f),
    (0x655896, f_00655896),
    (0x6558c7, f_006558c7),
    (0x655fb8, f_00655fb8),
    (0x6560a0, f_006560a0),
    (0x6566d0, f_006566d0),
    (0x6566e7, f_006566e7),
    (0x6567f0, f_006567f0),
    (0x656bbe, f_00656bbe),
    (0x65732c, f_0065732c),
    (0x657350, f_00657350),
    (0x659d49, f_00659d49),
    (0x65a1cf, f_0065a1cf),
    (0x65a268, f_0065a268),
    (0x65a3d7, f_0065a3d7),
    (0x65a9f9, f_0065a9f9),
    (0x65aa69, f_0065aa69),
    (0x42c020, f_0042c020),
    (0x4d7d50, f_004d7d50),
    (0x50dde0, f_0050dde0),
    (0x54bf80, f_0054bf80),
    (0x6389fa, f_006389fa),
    (0x63d88b, f_0063d88b),
    (0x643655, f_00643655),
    (0x6436d8, f_006436d8),
    (0x643760, f_00643760),
    (0x644adb, f_00644adb),
    (0x644f57, f_00644f57),
    (0x6458b3, f_006458b3),
    (0x646285, f_00646285),
    (0x646c6e, f_00646c6e),
    (0x647180, f_00647180),
    (0x649010, f_00649010),
    (0x64e4ce, f_0064e4ce),
    (0x64ed1d, f_0064ed1d),
    (0x64ed60, f_0064ed60),
    (0x64edf9, f_0064edf9),
    (0x64ee46, f_0064ee46),
    (0x64ee5f, f_0064ee5f),
    (0x6505d0, f_006505d0),
    (0x650666, f_00650666),
    (0x651c06, f_00651c06),
    (0x653310, f_00653310),
    (0x653573, f_00653573),
    (0x6557d8, f_006557d8),
    (0x656716, f_00656716),
    (0x65674b, f_0065674b),
    (0x656b90, f_00656b90),
    (0x656cdd, f_00656cdd),
    (0x656d36, f_00656d36),
    (0x65a144, f_0065a144),
    (0x65a157, f_0065a157),
    (0x65a19d, f_0065a19d),
    (0x65a966, f_0065a966),
    (0x63a19f, f_0063a19f),
    (0x63cf26, f_0063cf26),
    (0x63dc7a, f_0063dc7a),
    (0x63dca1, f_0063dca1),
    (0x63dd59, f_0063dd59),
    (0x644bb0, f_00644bb0),
    (0x645103, f_00645103),
    (0x645747, f_00645747),
    (0x646a99, f_00646a99),
    (0x64858c, f_0064858c),
    (0x648b06, f_00648b06),
    (0x64eda5, f_0064eda5),
    (0x654281, f_00654281),
    (0x656dd2, f_00656dd2),
    (0x63cf34, f_0063cf34),
    (0x6455ec, f_006455ec),
    (0x645678, f_00645678),
    (0x645693, f_00645693),
    (0x64569f, f_0064569f),
    (0x6456ba, f_006456ba),
    (0x6484d3, f_006484d3),
    (0x648531, f_00648531),
    (0x6488e6, f_006488e6),
    (0x64554d, f_0064554d),
    (0x645596, f_00645596),
    (0x6484b2, f_006484b2),
    (0x64855f, f_0064855f),
    (0x6505b2, f_006505b2),
];
pub fn f_00548860(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x548860;
    loop {
        match pc {
            0x548860 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x663925u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x44u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.r[1].wrapping_add(0x914); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x92c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0x924); wm(m,a,32,c.get(7,32)); }
            { let a = rm(m,0x6cbdd8u32,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x548c84; continue; }
            pc = 0x5488a4; continue;
            }
            0x5488a4 => {
            { let a = rm(m,0x6e51acu32,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x548c84; continue; }
            pc = 0x5488b0; continue;
            }
            0x5488b0 => {
            { let a = rm(m,c.r[4].wrapping_add(0x60),16); c.sub(a,c.get(3,16),16); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            if c.cond(5) { pc = 0x5488d0; continue; }
            pc = 0x5488b9; continue;
            }
            0x5488b9 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5488d0; continue; }
            pc = 0x5488cc; continue;
            }
            0x5488cc => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x5488d5; continue;
            }
            0x5488d0 => {
            c.set(5,32,0x1u32);
            pc = 0x5488d5; continue;
            }
            0x5488d5 => {
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5488df);
            { let args = []; c.r[0] = host.call(m, 0x50dc50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,0x5u32);
            pc = 0x5488e4; continue;
            }
            0x5488e4 => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x5488ef);
            { let args = []; c.r[0] = host.call(m, 0x42c020, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            if c.cond(5) { pc = 0x5488e4; continue; }
            pc = 0x5488f2; continue;
            }
            0x5488f2 => {
            c.set(2,32,rm(m,0x6dfc14u32,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x548903);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50dde0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(3,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x28cc),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x54891b);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x548929);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6de4a8u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x54898b; continue; }
            pc = 0x548941; continue;
            }
            0x548941 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x54894c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2c6),8));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,0x6cbdd8u32,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x64),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x28cc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2]); wm(m,a,8,0x1u32); }
            c.set(2,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2c8),16));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x548941; continue; }
            pc = 0x54898b; continue;
            }
            0x54898b => {
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x548af6; continue; }
            pc = 0x548993; continue;
            }
            0x548993 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x54899e);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x5489da; continue; }
            pc = 0x5489a2; continue;
            }
            0x5489a2 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5489b2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5489bf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x5489ce); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5489da); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5489da; continue;
            }
            0x5489da => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x5489ef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x44f490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x548a08);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x19),8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x39c),8));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe4),32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1a),8));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x548a50);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1a),8));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0ef8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6c2a34u32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x7c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x548a7a);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x84),32));
            c.set(1,32,rm(m,0x6cbb10u32,32));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x3a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x4a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x43); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x41); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x42); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,0x3f4ccccdu32); }
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x548acf); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x548ae3; continue; }
            pc = 0x548ad3; continue;
            }
            0x548ad3 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.push(m, 0x548ae3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x548ae3; continue;
            }
            0x548ae3 => {
            c.set(1,32,c.r[4].wrapping_add(0x64));
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x548af4);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x548b6a; continue;
            }
            0x548af6 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x548b01);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x548b3d; continue; }
            pc = 0x548b05; continue;
            }
            0x548b05 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(3,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x548b14); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(3,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x548b20); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x548b30); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x548b3d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x548b3d; continue;
            }
            0x548b3d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x548b53);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x58a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.push(m, 0x548b60);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x593910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,0x6ce6d8u32);
            c.push(m, 0x548b6a);
            { let args = []; c.r[0] = host.call(m, 0x495be0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x548b6a; continue;
            }
            0x548b6a => {
            c.set(2,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,0x1u32); }
            c.push(m, 0x548b87);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x19),8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x39c),8));
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xe4),32));
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x74),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x1a),8));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x548bcf);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1a),8));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x69ee48),32));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6c0ef8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6c2a34u32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x548bf9);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(1,32,rm(m,0x6cbb10u32,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x5a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x6a); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[4].wrapping_add(0x6c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x63); wm(m,a,8,0xffu32); }
            { let a = c.r[4].wrapping_add(0x61); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x62); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,0x3f4ccccdu32); }
            c.set(0,32,rm(m,c.r[1],32));
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0xc8),32); c.push(m, 0x548c4b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x548c61; continue; }
            pc = 0x548c51; continue;
            }
            0x548c51 => {
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb10u32,32));
            c.push(m, 0x548c61);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x41d9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x548c61; continue;
            }
            0x548c61 => {
            c.set(2,16,rm(m,0x6dfc14u32,16));
            c.set(1,32,0x6dd730u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x548c73);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d7d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x548c84);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x548c84; continue;
            }
            0x548c84 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x50u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0054b670(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x54b670;
    loop {
        match pc {
            0x54b670 => {
            { let a = c.get(4,32); let r = c.sub(a,0x434u32,32); c.set(4,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            c.set(6,32,c.r[4].wrapping_add(0x30));
            c.set(7,32,0x34u32);
            pc = 0x54b688; continue;
            }
            0x54b688 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x54b68f);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x14u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x54b688; continue; }
            pc = 0x54b695; continue;
            }
            0x54b695 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x54b69e);
            { let args = []; c.r[0] = host.call(m, 0x543da0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,0x1u32);
            c.set(6,32,0x24u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            c.push(m,c.get(3,32));
            pc = 0x54b6b1; continue;
            }
            0x54b6b1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x44),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x18),32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x54b8f1; continue; }
            pc = 0x54b6bf; continue;
            }
            0x54b6bf => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x3c),32));
            c.push(m,0x420u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a31cu32,32); c.push(m, 0x54b6d9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x54b8f1; continue; }
            pc = 0x54b6e4; continue;
            }
            0x54b6e4 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x44),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x18),32));
            c.set(7,32,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x18));
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x54b8d8; continue; }
            pc = 0x54b6fc; continue;
            }
            0x54b6fc => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x40),32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x54b763; continue; }
            pc = 0x54b706; continue;
            }
            0x54b706 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(1,32,0xeu32);
            { let a = rm(m,c.r[4].wrapping_add(0x32),16); c.sub(a,c.get(6,16),16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(6) { pc = 0x54b808; continue; }
            pc = 0x54b71c; continue;
            }
            0x54b71c => {
            c.set(0,32,c.r[4].wrapping_add(0x34));
            pc = 0x54b720; continue;
            }
            0x54b720 => {
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); c.sub(a,0xceu32,32); }
            if c.cond(5) { pc = 0x54b732; continue; }
            pc = 0x54b72a; continue;
            }
            0x54b72a => {
            { let a = c.r[0]; wm(m,a,32,0xcfu32); }
            pc = 0x54b748; continue;
            }
            0x54b732 => {
            { let a = c.get(2,32); c.sub(a,0xc3u32,32); }
            if c.cond(5) { pc = 0x54b748; continue; }
            pc = 0x54b73a; continue;
            }
            0x54b73a => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); c.sub(a,0x85u32,32); }
            if c.cond(4) { pc = 0x54b748; continue; }
            pc = 0x54b743; continue;
            }
            0x54b743 => {
            c.set(1,32,0x7530u32);
            pc = 0x54b748; continue;
            }
            0x54b748 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x32),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x14u32,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x54b720; continue; }
            pc = 0x54b75a; continue;
            }
            0x54b75a => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x54b808; continue;
            }
            0x54b763 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(0x32),16); c.sub(a,c.get(1,16),16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(6) { pc = 0x54b804; continue; }
            pc = 0x54b776; continue;
            }
            0x54b776 => {
            c.set(6,32,c.r[4].wrapping_add(0x34));
            pc = 0x54b77a; continue;
            }
            0x54b77a => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,0xceu32,32); }
            if c.cond(5) { pc = 0x54b7d6; continue; }
            pc = 0x54b783; continue;
            }
            0x54b783 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x54b7c5; continue; }
            pc = 0x54b78d; continue;
            }
            0x54b78d => {
            c.set(0,32,c.r[6].wrapping_add(0x6));
            c.set(7,32,0xeu32);
            pc = 0x54b795; continue;
            }
            0x54b795 => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[3].wrapping_add(0xfffffff8),32),32); }
            if c.cond(13) { pc = 0x54b7a4; continue; }
            pc = 0x54b79a; continue;
            }
            0x54b79a => {
            c.set(2,8,rm(m,c.r[3].wrapping_add(c.r[1]),8));
            { let a = c.get(2,8); c.sub(a,rm(m,c.r[0],8),8); }
            if c.cond(5) { pc = 0x54b7ac; continue; }
            pc = 0x54b7a1; continue;
            }
            0x54b7a1 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x54b7b1; continue;
            }
            0x54b7a4 => {
            { let a = rm(m,c.r[0],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x54b7ac; continue; }
            pc = 0x54b7a9; continue;
            }
            0x54b7a9 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x54b7b1; continue;
            }
            0x54b7ac => {
            c.set(1,32,0x4e20u32);
            pc = 0x54b7b1; continue;
            }
            0x54b7b1 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x54b795; continue; }
            pc = 0x54b7b9; continue;
            }
            0x54b7b9 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[6]; wm(m,a,32,0xcfu32); }
            pc = 0x54b7ef; continue;
            }
            0x54b7c5 => {
            c.set(1,32,0x4e20u32);
            { let a = c.r[6]; wm(m,a,32,0xcfu32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x54b7ef; continue;
            }
            0x54b7d6 => {
            { let a = c.get(0,32); c.sub(a,0xc3u32,32); }
            if c.cond(5) { pc = 0x54b7ef; continue; }
            pc = 0x54b7dd; continue;
            }
            0x54b7dd => {
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); c.sub(a,0x85u32,32); }
            if c.cond(4) { pc = 0x54b7ef; continue; }
            pc = 0x54b7e6; continue;
            }
            0x54b7e6 => {
            c.set(1,32,0x7530u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x54b7ef; continue;
            }
            0x54b7ef => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x32),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x14u32,32); c.set(6,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x54b77a; continue; }
            pc = 0x54b804; continue;
            }
            0x54b804 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x54b808; continue;
            }
            0x54b808 => {
            c.set(6,32,rm(m,c.r[3].wrapping_add(0xfffffff8),32));
            c.set(0,32,0x92492493u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x3u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1));
            c.set(2,32,c.r[0].wrapping_mul(8));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x54b87b; continue; }
            pc = 0x54b82f; continue;
            }
            0x54b82f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.sub(a,0x7530u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x34); wm(m,a,32,0xd0u32); }
            { let a = c.get(1,32); let r = c.and(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0xc7u32); }
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,0xffffu32); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,16,0x2u32); }
            { let a = c.r[7]; wm(m,a,32,0x3u32); }
            pc = 0x54b8d8; continue;
            }
            0x54b87b => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x54b88a; continue; }
            pc = 0x54b882; continue;
            }
            0x54b882 => {
            { let a = c.r[7]; wm(m,a,32,0x4u32); }
            pc = 0x54b8d8; continue;
            }
            0x54b88a => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x54b8d2; continue; }
            pc = 0x54b891; continue;
            }
            0x54b891 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x54b89d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54bf80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x54b8b3; continue; }
            pc = 0x54b8a2; continue;
            }
            0x54b8a2 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x44),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[1].wrapping_add(c.r[2]).wrapping_add(0x18); wm(m,a,32,0x6u32); }
            pc = 0x54b8d8; continue;
            }
            0x54b8b3 => {
            { let a = c.get(0,32); c.sub(a,0x320u32,32); }
            if c.cond(4) { pc = 0x54b8c1; continue; }
            pc = 0x54b8ba; continue;
            }
            0x54b8ba => {
            { let a = c.get(0,32); c.sub(a,0x3e7u32,32); }
            if c.cond(5) { pc = 0x54b8d8; continue; }
            pc = 0x54b8c1; continue;
            }
            0x54b8c1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x44),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[0].wrapping_add(c.r[1]).wrapping_add(0x18); wm(m,a,32,0x1u32); }
            pc = 0x54b8d8; continue;
            }
            0x54b8d2 => {
            { let a = c.r[7]; wm(m,a,32,0x1u32); }
            pc = 0x54b8d8; continue;
            }
            0x54b8d8 => {
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.r[5].wrapping_add(0x8b8));
            c.push(m,c.get(2,32));
            c.push(m, 0x54b8e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x543ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,0x1u32);
            pc = 0x54b8f1; continue;
            }
            0x54b8f1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let r = c.add(a,0x24u32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,0x120u32,32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            if c.cond(2) { pc = 0x54b6b1; continue; }
            pc = 0x54b90d; continue;
            }
            0x54b90d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x470),32));
            { let a = c.r[5].wrapping_add(0x46c); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(4) { pc = 0x54b932; continue; }
            pc = 0x54b91e; continue;
            }
            0x54b91e => {
            c.set(6,32,rm(m,0x66a2bcu32,32));
            pc = 0x54b924; continue;
            }
            0x54b924 => {
            c.push(m,0x1u32);
            { let target = c.get(6,32); c.push(m, 0x54b928); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x470),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x54b924; continue; }
            pc = 0x54b932; continue;
            }
            0x54b932 => {
            c.set(6,32,c.r[5].wrapping_add(0x48));
            c.set(1,32,c.r[5].wrapping_add(0x8b8));
            c.push(m,c.get(6,32));
            c.push(m, 0x54b941);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x543ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x54b948);
            { let args = []; c.r[0] = host.call(m, 0x544040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[5].wrapping_add(0x46c); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x434u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063bdef(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63bdef;
    loop {
        match pc {
            0x63bdef => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.and(a,0x83u32,8); }
            if c.cond(4) { pc = 0x63be32; continue; }
            pc = 0x63bdfe; continue;
            }
            0x63bdfe => {
            c.push(m,c.get(6,32));
            c.push(m, 0x63be04);
            { let args = []; c.r[0] = host.call(m, 0x643760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x63be0c);
            { let args = []; c.r[0] = host.call(m, 0x6436d8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[6].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m, 0x63be14);
            { let args = []; c.r[0] = host.call(m, 0x6435f8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x63be20; continue; }
            pc = 0x63be1b; continue;
            }
            0x63be1b => {
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            pc = 0x63be32; continue;
            }
            0x63be20 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63be32; continue; }
            pc = 0x63be27; continue;
            }
            0x63be27 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x63be2d);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x1c),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,r); } }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63be32; continue;
            }
            0x63be32 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063d118(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63d118;
    loop {
        match pc {
            0x63d118 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,rm(m,c.r[0],32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m, 0x63d128);
            { let args = []; c.r[0] = host.call(m, 0x63edb3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(7,32,c.get(0,32));
            if c.cond(13) { pc = 0x63d135; continue; }
            pc = 0x63d12e; continue;
            }
            0x63d12e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63d21e; continue;
            }
            0x63d135 => {
            { let a = rm(m,c.r[7].wrapping_add(0x40),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x63d14f; continue; }
            pc = 0x63d13b; continue;
            }
            0x63d13b => {
            c.push(m,0x24u32);
            c.push(m, 0x63d142);
            { let args = []; c.r[0] = host.call(m, 0x63cb41, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[7].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,32,0x6f0130u32);
            if c.cond(4) { pc = 0x63d152; continue; }
            pc = 0x63d14f; continue;
            }
            0x63d14f => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x40),32));
            pc = 0x63d152; continue;
            }
            0x63d152 => {
            c.set(0,32,c.get(6,32));
            c.set(7,32,0x7861f80u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.imul(a,0xf879e080u32,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(2,32,c.r[0].wrapping_mul(4).wrapping_add(0x46));
            c.set(0,32,0x1e13380u32);
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x63d191; continue; }
            pc = 0x63d176; continue;
            }
            0x63d176 => {
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x63d191; continue; }
            pc = 0x63d17d; continue;
            }
            0x63d17d => {
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,0x1e28500u32);
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x63d18e; continue; }
            pc = 0x63d189; continue;
            }
            0x63d189 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            pc = 0x63d191; continue;
            }
            0x63d18e => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x63d191; continue;
            }
            0x63d191 => {
            c.set(0,32,c.get(6,32));
            { let a = c.r[1].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(7,32,0x15180u32);
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(7,32,0x6cb1b4u32);
            { let a = c.r[1].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.imul(a,0xfffeae80u32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x63d1b7; continue; }
            pc = 0x63d1b2; continue;
            }
            0x63d1b2 => {
            c.set(7,32,0x6cb1e8u32);
            pc = 0x63d1b7; continue;
            }
            0x63d1b7 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x1c),32));
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            c.set(3,32,c.r[7].wrapping_add(0x4));
            pc = 0x63d1c0; continue;
            }
            0x63d1c0 => {
            { let a = rm(m,c.r[3],32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x63d1ca; continue; }
            pc = 0x63d1c4; continue;
            }
            0x63d1c4 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            pc = 0x63d1c0; continue;
            }
            0x63d1ca => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x1c),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.push(m,0x7u32);
            { let a = c.get(2,32); let r = c.sub(a,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(7,32,0x15180u32);
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[0],32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.push(m,0x3cu32);
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(0,32,c.get(6,32));
            c.set(7,32,0xe10u32);
            { let a = c.r[1].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.imul(a,0xfffff1f0u32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.imul(a,0x3cu32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x20),32); let r = c.and(a,0x0u32,32); { let a = c.r[1].wrapping_add(0x20); wm(m,a,32,r); } }
            { let a = c.r[1]; wm(m,a,32,c.get(6,32)); }
            c.set(0,32,c.get(1,32));
            pc = 0x63d21e; continue;
            }
            0x63d21e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063d791(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63d791;
    loop {
        match pc {
            0x63d791 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,0x6f0a48u32);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a0dcu32,32); c.push(m, 0x63d7ab); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,0x66a0e0u32,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = rm(m,0x6f0a44u32,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x63d7cf; continue; }
            pc = 0x63d7bb; continue;
            }
            0x63d7bb => {
            c.push(m,c.get(7,32));
            { let target = c.get(3,32); c.push(m, 0x63d7be); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x13u32);
            c.push(m, 0x63d7c5);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,0x1u32); }
            pc = 0x63d7d2; continue;
            }
            0x63d7cf => {
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(6,32)); }
            pc = 0x63d7d2; continue;
            }
            0x63d7d2 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x63d7de; continue; }
            pc = 0x63d7d9; continue;
            }
            0x63d7d9 => {
            c.set(0,32,rm(m,0x6c8680u32,32));
            pc = 0x63d7de; continue;
            }
            0x63d7de => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,c.get(6,32),32); }
            { let a = c.r[5].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x63d85d; continue; }
            pc = 0x63d7e6; continue;
            }
            0x63d7e6 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x10),32));
            pc = 0x63d7e9; continue;
            }
            0x63d7e9 => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x63d85d; continue; }
            pc = 0x63d7ef; continue;
            }
            0x63d7ef => {
            { let a = c.get(0,8); c.sub(a,0x25u32,8); }
            if c.cond(4) { pc = 0x63d825; continue; }
            pc = 0x63d7f3; continue;
            }
            0x63d7f3 => {
            c.set(2,32,rm(m,0x6c8760u32,32));
            c.set(1,32,c.get(0,8));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x1),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x63d815; continue; }
            pc = 0x63d803; continue;
            }
            0x63d803 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x1u32,32); }
            if c.cond(6) { pc = 0x63d815; continue; }
            pc = 0x63d809; continue;
            }
            0x63d809 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x63d815; continue;
            }
            0x63d815 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x63d857; continue;
            }
            0x63d825 => {
            { let a = rm(m,0x6f1a98u32,32); let r = c.and(a,0x0u32,32); { let a = 0x6f1a98u32; wm(m,a,32,r); } }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = rm(m,c.r[6],8); c.sub(a,0x23u32,8); }
            if c.cond(5) { pc = 0x63d83d; continue; }
            pc = 0x63d832; continue;
            }
            0x63d832 => {
            { let a = 0x6f1a98u32; wm(m,a,32,0x1u32); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x63d83d; continue;
            }
            0x63d83d => {
            { let v = rm(m,c.r[5].wrapping_add(0x18),32); c.push(m, v); }
            c.set(0,32,c.r[5].wrapping_add(0xfffffffc));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[6],8));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x63d853);
            { let args = []; c.r[0] = host.call(m, 0x63d88b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x63d857; continue;
            }
            0x63d857 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x0u32,32); }
            if c.cond(7) { pc = 0x63d7e9; continue; }
            pc = 0x63d85d; continue;
            }
            0x63d85d => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x63d86d; continue; }
            pc = 0x63d863; continue;
            }
            0x63d863 => {
            c.push(m,0x13u32);
            c.push(m, 0x63d86a);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63d870; continue;
            }
            0x63d86d => {
            c.push(m,c.get(7,32));
            { let target = c.get(3,32); c.push(m, 0x63d870); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x63d870; continue;
            }
            0x63d870 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x0u32,32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(6) { pc = 0x63d887; continue; }
            pc = 0x63d879; continue;
            }
            0x63d879 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[0],8); let r = c.and(a,0x0u32,8); { let a = c.r[0]; wm(m,a,8,r); } }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffffc),32),32); c.set(0,32,r); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63d887 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063e161(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63e161;
    loop {
        match pc {
            0x63e161 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x63e188; continue; }
            pc = 0x63e16a; continue;
            }
            0x63e16a => {
            c.push(m,c.get(6,32));
            c.push(m, 0x63e170);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x63e177);
            { let args = []; c.r[0] = host.call(m, 0x63cb41, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x63e188; continue; }
            pc = 0x63e17d; continue;
            }
            0x63e17d => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x63e184);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63e188 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00642a2e(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x642a2e;
    loop {
        match pc {
            0x642a2e => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.set(0,32,c.r[5].wrapping_add(0xfffffffc));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m,c.get(0,32));
            c.push(m, 0x642a4f);
            { let args = []; c.r[0] = host.call(m, 0x646285, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x642a66; continue; }
            pc = 0x642a59; continue;
            }
            0x642a59 => {
            c.set(7,32,0x200u32);
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(6,32)); }
            pc = 0x642a94; continue;
            }
            0x642a66 => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m,c.get(0,32));
            c.push(m, 0x642a73);
            { let args = []; c.r[0] = host.call(m, 0x6458b3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(3,8); c.and(a,0x2u32,8); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x642a7f; continue; }
            pc = 0x642a7a; continue;
            }
            0x642a7a => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x642a84; continue; }
            pc = 0x642a7f; continue;
            }
            0x642a7f => {
            c.set(7,32,0x80u32);
            pc = 0x642a84; continue;
            }
            0x642a84 => {
            { let a = c.get(3,8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x642a8e; continue; }
            pc = 0x642a89; continue;
            }
            0x642a89 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x642a94; continue; }
            pc = 0x642a8e; continue;
            }
            0x642a8e => {
            { let a = c.get(7,32); let r = c.or(a,0x100u32,32); c.set(7,32,r); }
            pc = 0x642a94; continue;
            }
            0x642a94 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0xc),32),32); c.set(1,32,r); }
            c.fld(c.ld(m,c.r[5].wrapping_add(0xfffffff4),8));
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.fstp64(m,c.r[0].wrapping_add(0x10));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00643554(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x643554;
    loop {
        match pc {
            0x643554 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,0x6cabd0u32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x643578; continue; }
            pc = 0x643561; continue;
            }
            0x643561 => {
            { let a = c.get(0,32); c.sub(a,0x6cae30u32,32); }
            if c.cond(7) { pc = 0x643578; continue; }
            pc = 0x643568; continue;
            }
            0x643568 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x1cu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x643576);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x643578 => {
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1b0u32,32); c.push(m, 0x643582); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00643583(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x643583;
    loop {
        match pc {
            0x643583 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x643597; continue; }
            pc = 0x64358c; continue;
            }
            0x64358c => {
            { let a = c.get(0,32); let r = c.add(a,0x1cu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x643595);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x643597 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1b0u32,32); c.push(m, 0x6435a5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006435d5(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6435d5;
    loop {
        match pc {
            0x6435d5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x6435e9; continue; }
            pc = 0x6435de; continue;
            }
            0x6435de => {
            { let a = c.get(0,32); let r = c.add(a,0x1cu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x6435e7);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x6435e9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1acu32,32); c.push(m, 0x6435f7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006435f8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6435f8;
    loop {
        match pc {
            0x6435f8 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6f0a40u32,32),32); }
            if c.cond(3) { pc = 0x64363d; continue; }
            pc = 0x643605; continue;
            }
            0x643605 => {
            c.set(1,32,c.get(6,32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x64363d; continue; }
            pc = 0x643620; continue;
            }
            0x643620 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x643627);
            { let args = []; c.r[0] = host.call(m, 0x646b5a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x64362d);
            { let args = []; c.r[0] = host.call(m, 0x643655, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x643635);
            { let args = []; c.r[0] = host.call(m, 0x646bb9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x64363d => {
            c.push(m, 0x643642);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x9u32); }
            c.push(m, 0x64364d);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0],32); let r = c.and(a,0x0u32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00643869(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x643869;
    loop {
        match pc {
            0x643869 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,8); c.and(a,0x83u32,8); }
            if c.cond(4) { pc = 0x643940; continue; }
            pc = 0x643879; continue;
            }
            0x643879 => {
            { let a = c.get(0,8); c.and(a,0x40u32,8); }
            if c.cond(5) { pc = 0x643940; continue; }
            pc = 0x643881; continue;
            }
            0x643881 => {
            { let a = c.get(0,8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x64388f; continue; }
            pc = 0x643885; continue;
            }
            0x643885 => {
            { let a = c.get(0,8); let r = c.or(a,0x20u32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x643940; continue;
            }
            0x64388f => {
            { let a = c.get(0,8); let r = c.or(a,0x1u32,8); c.set(0,8,r); }
            { let a = c.get(0,16); c.and(a,0x10cu32,16); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x6438a3; continue; }
            pc = 0x64389a; continue;
            }
            0x64389a => {
            c.push(m,c.get(6,32));
            c.push(m, 0x6438a0);
            { let args = []; c.r[0] = host.call(m, 0x646071, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x6438a8; continue;
            }
            0x6438a3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x6438a8; continue;
            }
            0x6438a8 => {
            { let v = rm(m,c.r[6].wrapping_add(0x18),32); c.push(m, v); }
            { let v = rm(m,c.r[6].wrapping_add(0x8),32); c.push(m, v); }
            { let v = rm(m,c.r[6].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m, 0x6438b6);
            { let args = []; c.r[0] = host.call(m, 0x646c6e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64392f; continue; }
            pc = 0x6438c0; continue;
            }
            0x6438c0 => {
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x64392f; continue; }
            pc = 0x6438c5; continue;
            }
            0x6438c5 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(2,8); c.and(a,0x82u32,8); }
            if c.cond(5) { pc = 0x643904; continue; }
            pc = 0x6438cd; continue;
            }
            0x6438cd => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x6438ed; continue; }
            pc = 0x6438d6; continue;
            }
            0x6438d6 => {
            c.set(7,32,c.get(1,32));
            { let a = c.get(7,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x1fu32,32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(7,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            pc = 0x6438f2; continue;
            }
            0x6438ed => {
            c.set(7,32,0x6cb048u32);
            pc = 0x6438f2; continue;
            }
            0x6438f2 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(1,8); let r = c.and(a,0x82u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x82u32,8); }
            if c.cond(5) { pc = 0x643904; continue; }
            pc = 0x6438fe; continue;
            }
            0x6438fe => {
            { let a = c.get(6,8); let r = c.or(a,0x20u32,8); c.set(6,8,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x643904; continue;
            }
            0x643904 => {
            { let a = rm(m,c.r[6].wrapping_add(0x18),32); c.sub(a,0x200u32,32); }
            if c.cond(5) { pc = 0x643921; continue; }
            pc = 0x64390d; continue;
            }
            0x64390d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,8); c.and(a,0x8u32,8); }
            if c.cond(4) { pc = 0x643921; continue; }
            pc = 0x643915; continue;
            }
            0x643915 => {
            { let a = c.get(5,8); c.and(a,0x4u32,8); }
            if c.cond(5) { pc = 0x643921; continue; }
            pc = 0x64391a; continue;
            }
            0x64391a => {
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,0x1000u32); }
            pc = 0x643921; continue;
            }
            0x643921 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1],8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x64392f => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x10u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.or(a,c.get(0,32),32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            pc = 0x643940; continue;
            }
            0x643940 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00644ab4(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x644ab4;
    loop {
        match pc {
            0x644ab4 => {
            c.push(m,c.get(6,32));
            c.push(m,0xcu32);
            c.push(m, 0x644abc);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,0x0u32);
            c.push(m, 0x644acb);
            { let args = []; c.r[0] = host.call(m, 0x644adb, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xcu32);
            c.set(6,32,c.get(0,32));
            c.push(m, 0x644ad4);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00644be7(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x644be7;
    loop {
        match pc {
            0x644be7 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = rm(m,0x6f1abcu32,32); c.sub(a,0x0u32,32); }
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x644c0b; continue; }
            pc = 0x644bfa; continue;
            }
            0x644bfa => {
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.push(m,c.get(7,32));
            c.push(m, 0x644c06);
            { let args = []; c.r[0] = host.call(m, 0x647180, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x644c7e; continue;
            }
            0x644c0b => {
            c.push(m,c.get(6,32));
            c.push(m,0x19u32);
            c.push(m, 0x644c13);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x644c58; continue; }
            pc = 0x644c1b; continue;
            }
            0x644c1b => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            pc = 0x644c1e; continue;
            }
            0x644c1e => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.set(6,32,c.get(0,8));
            { let a = rm(m,c.r[6].wrapping_add(0x6f1bc1),8); c.and(a,0x4u32,8); }
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            if c.cond(4) { pc = 0x644c42; continue; }
            pc = 0x644c2f; continue;
            }
            0x644c2f => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x644c4e; continue; }
            pc = 0x644c35; continue;
            }
            0x644c35 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x644c54; continue; }
            pc = 0x644c40; continue;
            }
            0x644c40 => {
            pc = 0x644c48; continue;
            }
            0x644c42 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x644c58; continue; }
            pc = 0x644c48; continue;
            }
            0x644c48 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x644c1e; continue; }
            pc = 0x644c4c; continue;
            }
            0x644c4c => {
            pc = 0x644c58; continue;
            }
            0x644c4e => {
            { let a = rm(m,c.r[7].wrapping_add(0xffffffff),8); let r = c.and(a,0x0u32,8); { let a = c.r[7].wrapping_add(0xffffffff); wm(m,a,8,r); } }
            pc = 0x644c58; continue;
            }
            0x644c54 => {
            { let a = rm(m,c.r[7].wrapping_add(0xfffffffe),8); let r = c.and(a,0x0u32,8); { let a = c.r[7].wrapping_add(0xfffffffe); wm(m,a,8,r); } }
            pc = 0x644c58; continue;
            }
            0x644c58 => {
            c.set(0,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(4) { pc = 0x644c73; continue; }
            pc = 0x644c60; continue;
            }
            0x644c60 => {
            c.set(1,32,c.r[2].wrapping_add(0x1));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            c.stos(m,32,true);
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.stos(m,8,true);
            pc = 0x644c73; continue;
            }
            0x644c73 => {
            c.push(m,0x19u32);
            c.push(m, 0x644c7a);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x644c7e; continue;
            }
            0x644c7e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00644c81(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x644c81;
    loop {
        match pc {
            0x644c81 => {
            { let a = rm(m,0x6f0408u32,32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x644cae; continue; }
            pc = 0x644c8a; continue;
            }
            0x644c8a => {
            c.push(m,0xbu32);
            c.push(m, 0x644c91);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,0x6f0408u32,32); c.sub(a,0x0u32,32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x644ca6; continue; }
            pc = 0x644c9b; continue;
            }
            0x644c9b => {
            c.push(m, 0x644ca0);
            { let args = []; c.r[0] = host.call(m, 0x644caf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,0x6f0408u32,32); let r = c.inc(a,32); { let a = 0x6f0408u32; wm(m,a,32,r); } }
            pc = 0x644ca6; continue;
            }
            0x644ca6 => {
            c.push(m,0xbu32);
            c.push(m, 0x644cad);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x644cae; continue;
            }
            0x644cae => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00644f36(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x644f36;
    loop {
        match pc {
            0x644f36 => {
            c.push(m,c.get(6,32));
            c.push(m,0xbu32);
            c.push(m, 0x644f3e);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[4].wrapping_add(0xc),32); c.push(m, v); }
            c.push(m, 0x644f47);
            { let args = []; c.r[0] = host.call(m, 0x644f57, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xbu32);
            c.set(6,32,c.get(0,32));
            c.push(m, 0x644f50);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006468fa(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6468fa;
    loop {
        match pc {
            0x6468fa => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x12u32);
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            c.push(m, 0x64690a);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            c.set(5,32,0x6f0940u32);
            pc = 0x64691a; continue;
            }
            0x64691a => {
            c.set(6,32,rm(m,c.r[5],32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x6469be; continue; }
            pc = 0x646925; continue;
            }
            0x646925 => {
            c.set(0,32,c.r[6].wrapping_add(0x480));
            pc = 0x64692b; continue;
            }
            0x64692b => {
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x6469a3; continue; }
            pc = 0x64692f; continue;
            }
            0x64692f => {
            { let a = rm(m,c.r[6].wrapping_add(0x4),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x646979; continue; }
            pc = 0x646935; continue;
            }
            0x646935 => {
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x64695e; continue; }
            pc = 0x64693b; continue;
            }
            0x64693b => {
            c.push(m,0x11u32);
            c.push(m, 0x646942);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x646956; continue; }
            pc = 0x646949; continue;
            }
            0x646949 => {
            c.set(0,32,c.r[6].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1a4u32,32); c.push(m, 0x646953); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); let r = c.inc(a,32); { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,r); } }
            pc = 0x646956; continue;
            }
            0x646956 => {
            c.push(m,0x11u32);
            c.push(m, 0x64695d);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x64695e; continue;
            }
            0x64695e => {
            c.set(3,32,c.r[6].wrapping_add(0xc));
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a1b0u32,32); c.push(m, 0x646968); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x4),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x646986; continue; }
            pc = 0x64696e; continue;
            }
            0x64696e => {
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a1acu32,32); c.push(m, 0x646975); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x646979; continue;
            }
            0x646979 => {
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(6,32); let r = c.add(a,0x24u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x480u32,32); c.set(0,32,r); }
            pc = 0x64692b; continue;
            }
            0x646986 => {
            { let a = rm(m,c.r[6],32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[6]; wm(m,a,32,r); } }
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[5],32),32); c.set(0,32,r); }
            c.push(m,0x24u32);
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.add(a,rm(m,c.r[4].wrapping_add(0x14),32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x646a0c; continue; }
            pc = 0x64699f; continue;
            }
            0x64699f => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x6469a3; continue;
            }
            0x6469a3 => {
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); let r = c.add(a,0x20u32,32); { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,r); } }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(5,32); c.sub(a,0x6f0a40u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            if c.cond(12) { pc = 0x64691a; continue; }
            pc = 0x6469bc; continue;
            }
            0x6469bc => {
            pc = 0x646a0c; continue;
            }
            0x6469be => {
            c.set(6,32,0x480u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x6469c9);
            { let args = []; c.r[0] = host.call(m, 0x63cb41, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x646a0c; continue; }
            pc = 0x6469ce; continue;
            }
            0x6469ce => {
            { let a = rm(m,0x6f0a40u32,32); let r = c.add(a,0x20u32,32); { let a = 0x6f0a40u32; wm(m,a,32,r); } }
            c.set(1,32,c.r[3].wrapping_mul(4).wrapping_add(0x6f0940));
            c.set(2,32,c.r[0].wrapping_add(0x480));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            pc = 0x6469e4; continue;
            }
            0x6469e4 => {
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x646a00; continue; }
            pc = 0x6469e8; continue;
            }
            0x6469e8 => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),8); let r = c.and(a,0x0u32,8); { let a = c.r[0].wrapping_add(0x4); wm(m,a,8,r); } }
            { let a = rm(m,c.r[0],32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            { let a = rm(m,c.r[0].wrapping_add(0x8),32); let r = c.and(a,0x0u32,32); { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = c.r[0].wrapping_add(0x5); wm(m,a,8,0xau32); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.add(a,0x24u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            pc = 0x6469e4; continue;
            }
            0x646a00 => {
            { let a = c.get(3,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            c.set(7,32,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x646a0b);
            { let args = []; c.r[0] = host.call(m, 0x646b5a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x646a0c; continue;
            }
            0x646a0c => {
            c.push(m,0x12u32);
            c.push(m, 0x646a13);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00646a1d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x646a1d;
    loop {
        match pc {
            0x646a1d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            { let a = c.get(1,32); c.sub(a,rm(m,0x6f0a40u32,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x646a80; continue; }
            pc = 0x646a2b; continue;
            }
            0x646a2b => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_mul(4).wrapping_add(0x6f0940));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x646a80; continue; }
            pc = 0x646a4a; continue;
            }
            0x646a4a => {
            { let a = rm(m,0x6c8414u32,32); c.sub(a,0x1u32,32); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            if c.cond(5) { pc = 0x646a76; continue; }
            pc = 0x646a58; continue;
            }
            0x646a58 => {
            { let a = c.get(1,32); let r = c.sub(a,0x0u32,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x646a6d; continue; }
            pc = 0x646a5d; continue;
            }
            0x646a5d => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x646a68; continue; }
            pc = 0x646a60; continue;
            }
            0x646a60 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x646a76; continue; }
            pc = 0x646a63; continue;
            }
            0x646a63 => {
            c.push(m,c.get(3,32));
            c.push(m,0xfffffff4u32);
            pc = 0x646a70; continue;
            }
            0x646a68 => {
            c.push(m,c.get(3,32));
            c.push(m,0xfffffff5u32);
            pc = 0x646a70; continue;
            }
            0x646a6d => {
            c.push(m,c.get(3,32));
            c.push(m,0xfffffff6u32);
            pc = 0x646a70; continue;
            }
            0x646a70 => {
            { let target = rm(m,0x66a284u32,32); c.push(m, 0x646a76); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x646a76; continue;
            }
            0x646a76 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x646a96; continue;
            }
            0x646a80 => {
            c.push(m, 0x646a85);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x9u32); }
            c.push(m, 0x646a90);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0],32); let r = c.and(a,0x0u32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x646a96; continue;
            }
            0x646a96 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00646cd3(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x646cd3;
    loop {
        match pc {
            0x646cd3 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,0x0u32,32); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(2,32,c.get(3,32));
            if c.cond(4) { pc = 0x646ea5; continue; }
            pc = 0x646cef; continue;
            }
            0x646cef => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(7,32,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x4),8));
            { let a = c.get(1,8); c.and(a,0x2u32,8); }
            if c.cond(5) { pc = 0x646ea5; continue; }
            pc = 0x646d1c; continue;
            }
            0x646d1c => {
            { let a = c.get(1,8); c.and(a,0x48u32,8); }
            if c.cond(4) { pc = 0x646d3e; continue; }
            pc = 0x646d21; continue;
            }
            0x646d21 => {
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x5),8));
            { let a = c.get(0,8); c.sub(a,0xau32,8); }
            if c.cond(4) { pc = 0x646d3e; continue; }
            pc = 0x646d28; continue;
            }
            0x646d28 => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            { let a = c.r[3]; wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[7],32));
            c.set(2,32,c.r[3].wrapping_add(0x1));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,0x1u32); }
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x5); wm(m,a,8,0xau32); }
            pc = 0x646d3e; continue;
            }
            0x646d3e => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,c.get(2,32));
            { let v = rm(m,c.r[0].wrapping_add(c.r[6]),32); c.push(m, v); }
            { let target = rm(m,0x66a140u32,32); c.push(m, 0x646d53); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x646d90; continue; }
            pc = 0x646d57; continue;
            }
            0x646d57 => {
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x646d5d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x5u32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x646d78; continue; }
            pc = 0x646d64; continue;
            }
            0x646d64 => {
            c.push(m, 0x646d69);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x9u32); }
            c.push(m, 0x646d74);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            pc = 0x646d88; continue;
            }
            0x646d78 => {
            { let a = c.get(0,32); c.sub(a,0x6du32,32); }
            if c.cond(4) { pc = 0x646ea5; continue; }
            pc = 0x646d81; continue;
            }
            0x646d81 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x646d87);
            { let args = []; c.r[0] = host.call(m, 0x63ccb5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x646d88; continue;
            }
            0x646d88 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x646ea7; continue;
            }
            0x646d90 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.add(a,c.get(2,32),32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            c.set(1,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4),8));
            { let a = c.get(0,8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x646ea0; continue; }
            pc = 0x646da8; continue;
            }
            0x646da8 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x646db5; continue; }
            pc = 0x646dac; continue;
            }
            0x646dac => {
            { let a = rm(m,c.r[3],8); c.sub(a,0xau32,8); }
            if c.cond(5) { pc = 0x646db5; continue; }
            pc = 0x646db1; continue;
            }
            0x646db1 => {
            { let a = c.get(0,8); let r = c.or(a,0x4u32,8); c.set(0,8,r); }
            pc = 0x646db7; continue;
            }
            0x646db5 => {
            { let a = c.get(0,8); let r = c.and(a,0xfbu32,8); c.set(0,8,r); }
            pc = 0x646db7; continue;
            }
            0x646db7 => {
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x646e9a; continue; }
            pc = 0x646dcf; continue;
            }
            0x646dcf => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(0,8,rm(m,c.r[0],8));
            { let a = c.get(0,8); c.sub(a,0x1au32,8); }
            if c.cond(4) { pc = 0x646e8a; continue; }
            pc = 0x646ddc; continue;
            }
            0x646ddc => {
            { let a = c.get(0,8); c.sub(a,0xdu32,8); }
            if c.cond(4) { pc = 0x646deb; continue; }
            pc = 0x646de0; continue;
            }
            0x646de0 => {
            { let a = c.r[3]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            pc = 0x646e7c; continue;
            }
            0x646deb => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x646e09; continue; }
            pc = 0x646df1; continue;
            }
            0x646df1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0],8); c.sub(a,0xau32,8); }
            if c.cond(5) { pc = 0x646e00; continue; }
            pc = 0x646dfa; continue;
            }
            0x646dfa => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.add(a,0x2u32,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            pc = 0x646e5e; continue;
            }
            0x646e00 => {
            { let a = c.r[3]; wm(m,a,8,0xdu32); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x646e7c; continue;
            }
            0x646e09 => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            c.set(0,32,c.r[5].wrapping_add(0xffffffff));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[7],32));
            { let v = rm(m,c.r[0].wrapping_add(c.r[6]),32); c.push(m, v); }
            { let target = rm(m,0x66a140u32,32); c.push(m, 0x646e23); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x646e31; continue; }
            pc = 0x646e27; continue;
            }
            0x646e27 => {
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x646e2d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x646e78; continue; }
            pc = 0x646e31; continue;
            }
            0x646e31 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff4),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x646e78; continue; }
            pc = 0x646e37; continue;
            }
            0x646e37 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4),8); c.and(a,0x48u32,8); }
            if c.cond(4) { pc = 0x646e53; continue; }
            pc = 0x646e40; continue;
            }
            0x646e40 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xffffffff),8));
            { let a = c.get(0,8); c.sub(a,0xau32,8); }
            if c.cond(4) { pc = 0x646e5e; continue; }
            pc = 0x646e47; continue;
            }
            0x646e47 => {
            { let a = c.r[3]; wm(m,a,8,0xdu32); }
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[1].wrapping_add(c.r[6]).wrapping_add(0x5); wm(m,a,8,c.get(0,8)); }
            pc = 0x646e7c; continue;
            }
            0x646e53 => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[5].wrapping_add(0xc),32),32); }
            if c.cond(5) { pc = 0x646e63; continue; }
            pc = 0x646e58; continue;
            }
            0x646e58 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffff),8); c.sub(a,0xau32,8); }
            if c.cond(5) { pc = 0x646e63; continue; }
            pc = 0x646e5e; continue;
            }
            0x646e5e => {
            { let a = c.r[3]; wm(m,a,8,0xau32); }
            pc = 0x646e7b; continue;
            }
            0x646e63 => {
            c.push(m,0x1u32);
            c.push(m,0xffffffffu32);
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x646e6f);
            { let args = []; c.r[0] = host.call(m, 0x645e0e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffffff),8); c.sub(a,0xau32,8); }
            if c.cond(4) { pc = 0x646e7c; continue; }
            pc = 0x646e78; continue;
            }
            0x646e78 => {
            { let a = c.r[3]; wm(m,a,8,0xdu32); }
            pc = 0x646e7b; continue;
            }
            0x646e7b => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            pc = 0x646e7c; continue;
            }
            0x646e7c => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x646dcf; continue; }
            pc = 0x646e88; continue;
            }
            0x646e88 => {
            pc = 0x646e9a; continue;
            }
            0x646e8a => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(6,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4));
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.and(a,0x40u32,8); }
            if c.cond(5) { pc = 0x646e9a; continue; }
            pc = 0x646e96; continue;
            }
            0x646e96 => {
            { let a = c.get(0,8); let r = c.or(a,0x2u32,8); c.set(0,8,r); }
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            pc = 0x646e9a; continue;
            }
            0x646e9a => {
            { let a = c.get(3,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0xc),32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(3,32)); }
            pc = 0x646ea0; continue;
            }
            0x646ea0 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            pc = 0x646ea7; continue;
            }
            0x646ea5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x646ea7; continue;
            }
            0x646ea7 => {
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

pub fn f_00648ca9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x648ca9;
    loop {
        match pc {
            0x648ca9 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(0,32,0x1004u32);
            c.push(m, 0x648cb6);
            { let sz = c.r[0]; c.r[0] = host.call(m, 0x63b920, c.r[1], &[]); c.r[4] = c.r[4].wrapping_sub(sz); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x648cc5);
            { let args = []; c.r[0] = host.call(m, 0x645e0e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x648dc8; continue; }
            pc = 0x648cd6; continue;
            }
            0x648cd6 => {
            c.push(m,0x2u32);
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x648ce1);
            { let args = []; c.r[0] = host.call(m, 0x645e0e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x648dc8; continue; }
            pc = 0x648cec; continue;
            }
            0x648cec => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x648d6b; continue; }
            pc = 0x648cf6; continue;
            }
            0x648cf6 => {
            c.set(3,32,0x1000u32);
            c.set(0,32,c.r[5].wrapping_add(0xffffeffc));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x648d09);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x8000u32);
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x648d16);
            { let args = []; c.r[0] = host.call(m, 0x649010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x648d1c; continue;
            }
            0x648d1c => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            c.set(0,32,c.get(3,32));
            if c.cond(13) { pc = 0x648d24; continue; }
            pc = 0x648d22; continue;
            }
            0x648d22 => {
            c.set(0,32,c.get(7,32));
            pc = 0x648d24; continue;
            }
            0x648d24 => {
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffeffc));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x648d34);
            { let args = []; c.r[0] = host.call(m, 0x645ee6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x648d44; continue; }
            pc = 0x648d3c; continue;
            }
            0x648d3c => {
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x648d5c; continue; }
            pc = 0x648d42; continue;
            }
            0x648d42 => {
            pc = 0x648d1c; continue;
            }
            0x648d44 => {
            c.push(m, 0x648d49);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0],32); c.sub(a,0x5u32,32); }
            if c.cond(5) { pc = 0x648d59; continue; }
            pc = 0x648d4e; continue;
            }
            0x648d4e => {
            c.push(m, 0x648d53);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0xdu32); }
            pc = 0x648d59; continue;
            }
            0x648d59 => {
            { let a = c.get(6,32); let r = c.or(a,0xffffffffu32,32); c.set(6,32,r); }
            pc = 0x648d5c; continue;
            }
            0x648d5c => {
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x648d67);
            { let args = []; c.r[0] = host.call(m, 0x649010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x648db3; continue;
            }
            0x648d6b => {
            if c.cond(13) { pc = 0x648db3; continue; }
            pc = 0x648d6d; continue;
            }
            0x648d6d => {
            c.push(m,0x0u32);
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x648d7a);
            { let args = []; c.r[0] = host.call(m, 0x645e0e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x648d82);
            { let args = []; c.r[0] = host.call(m, 0x646b18, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a15cu32,32); c.push(m, 0x648d8c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sbb(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.neg(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x648db3; continue; }
            pc = 0x648d99; continue;
            }
            0x648d99 => {
            c.push(m, 0x648d9e);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0xdu32); }
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x648daa); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x648db1);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            pc = 0x648db3; continue;
            }
            0x648db3 => {
            c.push(m,0x0u32);
            { let v = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x648dc0);
            { let args = []; c.r[0] = host.call(m, 0x645e0e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x648dca; continue;
            }
            0x648dc8 => {
            c.set(0,32,c.get(3,32));
            pc = 0x648dca; continue;
            }
            0x648dca => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e51a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e51a;
    loop {
        match pc {
            0x64e51a => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,16,rm(m,c.r[0],16));
            { let a = c.get(1,16); c.sub(a,0x76cu32,16); }
            if c.cond(3) { pc = 0x64e532; continue; }
            pc = 0x64e52d; continue;
            }
            0x64e52d => {
            { let a = rm(m,c.r[6],32); let r = c.and(a,0x0u32,32); { let a = c.r[6]; wm(m,a,32,r); } }
            pc = 0x64e55f; continue;
            }
            0x64e532 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),16));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xa),16));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),16));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x6),16));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x2),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.get(1,16));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[5].wrapping_add(0x8));
            c.push(m, 0x64e55a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x64e4ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x64e55f; continue;
            }
            0x64e55f => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e725(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e725;
    loop {
        match pc {
            0x64e725 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x64e73b; continue; }
            pc = 0x64e735; continue;
            }
            0x64e735 => {
            { let a = rm(m,c.r[2].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,r); } }
            pc = 0x64e73f; continue;
            }
            0x64e73b => {
            { let a = rm(m,c.r[1].wrapping_add(0x8),32); let r = c.and(a,0x0u32,32); { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,r); } }
            pc = 0x64e73f; continue;
            }
            0x64e73f => {
            c.push(m,c.get(0,32));
            c.push(m, 0x64e745);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64e6e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064eec4(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64eec4;
    loop {
        match pc {
            0x64eec4 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x64eed8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ee5f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64ef0b; continue; }
            pc = 0x64eedc; continue;
            }
            0x64eedc => {
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64eeed; continue; }
            pc = 0x64eee1; continue;
            }
            0x64eee1 => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            { let v = rm(m,c.r[6].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x64eeed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ed60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x64eeed; continue;
            }
            0x64eeed => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64eef4);
            { let args = []; c.r[0] = host.call(m, 0x64edf9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            pc = 0x64ef0b; continue;
            }
            0x64ef0b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ef14(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ef14;
    loop {
        match pc {
            0x64ef14 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x64ef4f; continue; }
            pc = 0x64ef1d; continue;
            }
            0x64ef1d => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let b = rm(m,c.r[1].wrapping_add(0x8),32); c.div32(b); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.set(2,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)));
            pc = 0x64ef31; continue;
            }
            0x64ef31 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64ef4f; continue; }
            pc = 0x64ef35; continue;
            }
            0x64ef35 => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x64ef40; continue; }
            pc = 0x64ef3a; continue;
            }
            0x64ef3a => {
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[0],32));
            pc = 0x64ef31; continue;
            }
            0x64ef40 => {
            c.set(6,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            { let a = c.r[2]; wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x64ef4a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ee46, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x64ef51; continue;
            }
            0x64ef4f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64ef51; continue;
            }
            0x64ef51 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064feb9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64feb9;
    loop {
        match pc {
            0x64feb9 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x64fec5);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x64fed0; continue; }
            pc = 0x64fecc; continue;
            }
            0x64fecc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64ff0b; continue;
            }
            0x64fed0 => {
            c.push(m, 0x64fed5);
            { let args = []; c.r[0] = host.call(m, 0x6389fa, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            c.set(6,32,0xe146u32);
            if c.cond(4) { pc = 0x64fef5; continue; }
            pc = 0x64fede; continue;
            }
            0x64fede => {
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,c.r[5].wrapping_add(0xfffffff8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xc),32); c.push(m, 0x64feec); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64fef5; continue; }
            pc = 0x64fef0; continue;
            }
            0x64fef0 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x64ff0b; continue;
            }
            0x64fef5 => {
            c.push(m, 0x64fefa);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.r[5].wrapping_add(0xfffffff8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(2,32,rm(m,c.r[0],32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xc),32); c.push(m, 0x64ff0b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x64ff0b; continue;
            }
            0x64ff0b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00650a3d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x650a3d;
    loop {
        match pc {
            0x650a3d => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x6593f0u32);
            c.set(1,32,0x6efb74u32);
            c.push(m, 0x650a4e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a5cf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = rm(m,c.r[6].wrapping_add(0x14),32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x650a84; continue; }
            pc = 0x650a59; continue;
            }
            0x650a59 => {
            { let a = rm(m,c.r[6].wrapping_add(0x2c),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x650a81; continue; }
            pc = 0x650a5f; continue;
            }
            0x650a5f => {
            { let target = rm(m,0x66a11cu32,32); c.push(m, 0x650a65); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m,0x650847u32);
            c.push(m,0x5u32);
            { let target = rm(m,0x66a3a0u32,32); c.push(m, 0x650a75); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x650a81; continue; }
            pc = 0x650a7c; continue;
            }
            0x650a7c => {
            c.push(m, 0x650a81);
            { let args = []; c.r[0] = host.call(m, 0x64e49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x650a81; continue;
            }
            0x650a81 => {
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            pc = 0x650a84; continue;
            }
            0x650a84 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00650a89(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x650a89;
    loop {
        match pc {
            0x650a89 => {
            c.push(m,c.get(6,32));
            c.push(m,0x6593f0u32);
            c.set(1,32,0x6efb74u32);
            c.push(m, 0x650a99);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a5cf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.push(m, 0x650aa0);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0].wrapping_add(0x14),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x650ab8; continue; }
            pc = 0x650aa6; continue;
            }
            0x650aa6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x650ab8; continue; }
            pc = 0x650aad; continue;
            }
            0x650aad => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a390u32,32); c.push(m, 0x650ab4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x2c),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,r); } }
            pc = 0x650ab8; continue;
            }
            0x650ab8 => {
            { let a = rm(m,c.r[6].wrapping_add(0x14),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x650ac6; continue; }
            pc = 0x650abe; continue;
            }
            0x650abe => {
            { let a = rm(m,c.r[6].wrapping_add(0x14),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x650ac6 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006510c6(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6510c6;
    loop {
        match pc {
            0x6510c6 => {
            c.set(0,32,0x6687fcu32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x6510d0; }
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[6].wrapping_add(0x24),32); c.push(m, v); }
            { let v = rm(m,c.r[6].wrapping_add(0x10),32); c.push(m, v); }
            { let target = rm(m,0x66a3ccu32,32); c.push(m, 0x6510ec); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x651143; continue; }
            pc = 0x6510f0; continue;
            }
            0x6510f0 => {
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a3c8u32,32); c.push(m, 0x6510f7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x651100; continue; }
            pc = 0x6510fc; continue;
            }
            0x6510fc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x651146; continue;
            }
            0x651100 => {
            c.push(m, 0x651105);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x14),8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x651143; continue; }
            pc = 0x65110c; continue;
            }
            0x65110c => {
            c.push(m,0x1u32);
            c.push(m, 0x651113);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a9f9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x65111b);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[6].wrapping_add(0x24),32); c.push(m, v); }
            c.set(6,32,rm(m,0x66a120u32,32));
            c.set(7,32,c.r[0].wrapping_add(0x34));
            c.push(m,c.get(7,32));
            { let target = c.get(6,32); c.push(m, 0x65112a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[5].wrapping_add(0xa));
            { let a = c.r[5].wrapping_add(0xa); wm(m,a,8,0xau32); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xb); wm(m,a,8,c.get(3,8)); }
            { let target = c.get(6,32); c.push(m, 0x651138); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m,0x1u32);
            c.push(m, 0x651143);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65aa69, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x651143; continue;
            }
            0x651143 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x651146; continue;
            }
            0x651146 => {
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

pub fn f_0065252d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65252d;
    loop {
        match pc {
            0x65252d => {
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = rm(m,c.r[0].wrapping_add(0x8),32); let r = c.and(a,0x0u32,32); { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006526f8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6526f8;
    loop {
        match pc {
            0x6526f8 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,c.get(7,32),32); }
            c.set(3,32,c.get(1,32));
            if c.cond(4) { pc = 0x65273a; continue; }
            pc = 0x652705; continue;
            }
            0x652705 => {
            c.push(m, 0x65270a);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.push(m,0xf0u32);
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a0ecu32,32); c.push(m, 0x65271d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x65273a; continue; }
            pc = 0x652721; continue;
            }
            0x652721 => {
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a0e8u32,32); c.push(m, 0x652729); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x652731; continue; }
            pc = 0x65272d; continue;
            }
            0x65272d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x652742; continue;
            }
            0x652731 => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a0f0u32,32); c.push(m, 0x652738); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            pc = 0x65273a; continue;
            }
            0x65273a => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x652742);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x652748, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x652742; continue;
            }
            0x652742 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00652748(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x652748;
    loop {
        match pc {
            0x652748 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            if c.cond(4) { pc = 0x6527fe; continue; }
            pc = 0x652764; continue;
            }
            0x652764 => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x66a40cu32,32));
            pc = 0x65276b; continue;
            }
            0x65276b => {
            c.set(1,16,rm(m,c.r[6],16));
            { let a = c.get(1,16); c.and(a,c.get(1,16),16); }
            if c.cond(4) { pc = 0x6527f7; continue; }
            pc = 0x652777; continue;
            }
            0x652777 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[6],16));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(0,16); c.sub(a,0x1234u32,16); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x6527aa; continue; }
            pc = 0x65278c; continue;
            }
            0x65278c => {
            { let a = c.get(0,16); c.sub(a,0x401u32,16); }
            if c.cond(5) { pc = 0x652799; continue; }
            pc = 0x652792; continue;
            }
            0x652792 => {
            c.set(0,32,0x180u32);
            pc = 0x6527d1; continue;
            }
            0x652799 => {
            { let a = c.get(0,16); c.sub(a,0x403u32,16); }
            if c.cond(5) { pc = 0x6527a4; continue; }
            pc = 0x65279f; continue;
            }
            0x65279f => {
            c.set(0,32,0x143u32);
            pc = 0x6527a4; continue;
            }
            0x6527a4 => {
            { let a = c.get(0,16); c.sub(a,0x401u32,16); }
            if c.cond(5) { pc = 0x6527c5; continue; }
            pc = 0x6527aa; continue;
            }
            0x6527aa => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffdc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,r); } }
            c.set(0,32,c.r[5].wrapping_add(0xffffffd8));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,0x1u32); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(6,32)); }
            c.push(m,0x401u32);
            pc = 0x6527d8; continue;
            }
            0x6527c5 => {
            { let a = c.get(0,16); c.sub(a,0x180u32,16); }
            if c.cond(4) { pc = 0x6527d1; continue; }
            pc = 0x6527cb; continue;
            }
            0x6527cb => {
            { let a = c.get(0,16); c.sub(a,0x143u32,16); }
            if c.cond(5) { pc = 0x6527ea; continue; }
            pc = 0x6527d1; continue;
            }
            0x6527d1 => {
            c.set(0,32,c.get(0,16));
            c.push(m,c.get(6,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            pc = 0x6527d8; continue;
            }
            0x6527d8 => {
            c.set(0,32,c.get(1,16));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[3].wrapping_add(0x1c),32); c.push(m, v); }
            { let target = c.get(7,32); c.push(m, 0x6527e1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x6527ea; continue; }
            pc = 0x6527e6; continue;
            }
            0x6527e6 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x6527ea; continue;
            }
            0x6527ea => {
            { let a = c.get(6,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x8),32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x65276b; continue; }
            pc = 0x6527f7; continue;
            }
            0x6527f7 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x0u32,32); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(4) { pc = 0x652811; continue; }
            pc = 0x6527fe; continue;
            }
            0x6527fe => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(0,32));
            c.push(m,0x364u32);
            { let v = rm(m,c.r[3].wrapping_add(0x1c),32); c.push(m, v); }
            c.push(m, 0x652811);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x651c06, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            pc = 0x652811; continue;
            }
            0x652811 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00652d66(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x652d66;
    loop {
        match pc {
            0x652d66 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let v = rm(m,c.r[4].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x652d72);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6505d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x652da5; continue; }
            pc = 0x652d76; continue;
            }
            0x652d76 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x50),32); c.push(m, 0x652d7d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x80),32); c.push(m, 0x652d87); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.push(m, 0x652d8e);
            { let args = []; c.r[0] = host.call(m, 0x650666, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m,0xfffffffcu32);
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            { let target = rm(m,0x66a514u32,32); c.push(m, 0x652d9b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x652da2; continue; }
            pc = 0x652da0; continue;
            }
            0x652da0 => {
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x652da2; continue;
            }
            0x652da2 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x652da5; continue;
            }
            0x652da5 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006532e6(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6532e6;
    loop {
        match pc {
            0x6532e6 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x34),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x653302; continue; }
            pc = 0x6532ed; continue;
            }
            0x6532ed => {
            { let v = rm(m,c.r[4].wrapping_add(0x4),32); c.push(m, v); }
            { let v = rm(m,c.r[1].wrapping_add(0x1c),32); c.push(m, v); }
            { let target = rm(m,0x66a340u32,32); c.push(m, 0x6532fa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(0,32));
            c.push(m, 0x653300);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65058b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x65330d; continue;
            }
            0x653302 => {
            c.set(2,32,rm(m,c.r[0],32));
            { let v = rm(m,c.r[4].wrapping_add(0x4),32); c.push(m, v); }
            c.set(1,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x70),32); c.push(m, 0x65330d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x65330d; continue;
            }
            0x65330d => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065348d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65348d;
    loop {
        match pc {
            0x65348d => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x38),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x6534a3; continue; }
            pc = 0x653494; continue;
            }
            0x653494 => {
            { let v = rm(m,c.r[4].wrapping_add(0x4),32); c.push(m, v); }
            { let v = rm(m,c.r[1].wrapping_add(0x1c),32); c.push(m, v); }
            { let target = rm(m,0x66a53cu32,32); c.push(m, 0x6534a1); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x6534b1; continue;
            }
            0x6534a3 => {
            c.set(2,32,rm(m,c.r[0],32));
            { let v = rm(m,c.r[4].wrapping_add(0x4),32); c.push(m, v); }
            c.set(1,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xa0),32); c.push(m, 0x6534b1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6534b1; continue;
            }
            0x6534b1 => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653554(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653554;
    loop {
        match pc {
            0x653554 => {
            { let a = rm(m,c.r[1].wrapping_add(0x38),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x653570; continue; }
            pc = 0x65355a; continue;
            }
            0x65355a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x34),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x653570; continue; }
            pc = 0x653565; continue;
            }
            0x653565 => {
            c.set(2,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8c),32); c.push(m, 0x653570); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x653570; continue;
            }
            0x653570 => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653645(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653645;
    loop {
        match pc {
            0x653645 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x653666; continue; }
            pc = 0x653650; continue;
            }
            0x653650 => {
            c.push(m,0x0u32);
            c.push(m, 0x653657);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x653573, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x653666; continue; }
            pc = 0x65365b; continue;
            }
            0x65365b => {
            { let v = rm(m,c.r[6].wrapping_add(0x4),32); c.push(m, v); }
            c.set(1,32,c.r[0].wrapping_add(0x4));
            c.push(m, 0x653666);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ef14, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x653666; continue;
            }
            0x653666 => {
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065369e(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65369e;
    loop {
        match pc {
            0x65369e => {
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x6536af; continue; }
            pc = 0x6536a4; continue;
            }
            0x6536a4 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x6536af; continue; }
            pc = 0x6536a8; continue;
            }
            0x6536a8 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x4),32); c.push(m, 0x6536af); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6536af; continue;
            }
            0x6536af => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065371b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65371b;
    loop {
        match pc {
            0x65371b => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x653723); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x6593f0u32);
            c.set(1,32,0x6efb74u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x653734);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a5cf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(0,32));
            { let target = rm(m,0x66a12cu32,32); c.push(m, 0x65373d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x10));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00654037(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x654037;
    loop {
        match pc {
            0x654037 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,0x104u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x654049);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f637, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x654055);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a2d0u32,32); c.push(m, 0x654064); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,c.r[7],8));
            c.set(0,32,c.get(7,32));
            pc = 0x654068; continue;
            }
            0x654068 => {
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x65408e; continue; }
            pc = 0x65406c; continue;
            }
            0x65406c => {
            { let a = c.get(1,8); c.sub(a,0x5cu32,8); }
            if c.cond(4) { pc = 0x654076; continue; }
            pc = 0x654071; continue;
            }
            0x654071 => {
            { let a = c.get(1,8); c.sub(a,0x2fu32,8); }
            if c.cond(5) { pc = 0x654083; continue; }
            pc = 0x654076; continue;
            }
            0x654076 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(1,8); c.sub(a,0x5cu32,8); }
            if c.cond(4) { pc = 0x65408e; continue; }
            pc = 0x65407e; continue;
            }
            0x65407e => {
            { let a = c.get(1,8); c.sub(a,0x2fu32,8); }
            if c.cond(4) { pc = 0x65408e; continue; }
            pc = 0x654083; continue;
            }
            0x654083 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x654089);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,8,rm(m,c.r[0],8));
            pc = 0x654068; continue;
            }
            0x65408e => {
            { let a = rm(m,c.r[0],8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x6540e0; continue; }
            pc = 0x654092; continue;
            }
            0x654092 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x654094; continue;
            }
            0x654094 => {
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x6540ad; continue; }
            pc = 0x65409a; continue;
            }
            0x65409a => {
            { let a = c.get(1,8); c.sub(a,0x5cu32,8); }
            if c.cond(4) { pc = 0x6540ad; continue; }
            pc = 0x65409f; continue;
            }
            0x65409f => {
            { let a = c.get(1,8); c.sub(a,0x2fu32,8); }
            if c.cond(4) { pc = 0x6540ad; continue; }
            pc = 0x6540a4; continue;
            }
            0x6540a4 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x6540aa);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x654094; continue;
            }
            0x6540ad => {
            { let a = rm(m,c.r[0],8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x6540b8; continue; }
            pc = 0x6540b1; continue;
            }
            0x6540b1 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x6540b7);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x6540b8; continue;
            }
            0x6540b8 => {
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x6540c8; continue; }
            pc = 0x6540be; continue;
            }
            0x6540be => {
            { let a = c.get(1,8); c.sub(a,0x5cu32,8); }
            if c.cond(4) { pc = 0x6540c8; continue; }
            pc = 0x6540c3; continue;
            }
            0x6540c3 => {
            { let a = c.get(1,8); c.sub(a,0x2fu32,8); }
            if c.cond(5) { pc = 0x6540b1; continue; }
            pc = 0x6540c8; continue;
            }
            0x6540c8 => {
            { let a = rm(m,c.r[0],8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x6540cf; continue; }
            pc = 0x6540cc; continue;
            }
            0x6540cc => {
            { let a = c.r[0].wrapping_add(0x1); wm(m,a,8,c.get(3,8)); }
            pc = 0x6540cf; continue;
            }
            0x6540cf => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,0xffffffffu32);
            c.push(m, 0x6540da);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f686, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x6540e0 => {
            c.set(1,8,rm(m,c.r[7],8));
            c.set(0,32,c.get(7,32));
            pc = 0x6540e4; continue;
            }
            0x6540e4 => {
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x6540c8; continue; }
            pc = 0x6540e8; continue;
            }
            0x6540e8 => {
            { let a = c.get(1,8); c.sub(a,0x5cu32,8); }
            if c.cond(4) { pc = 0x6540c8; continue; }
            pc = 0x6540ed; continue;
            }
            0x6540ed => {
            { let a = c.get(1,8); c.sub(a,0x2fu32,8); }
            if c.cond(4) { pc = 0x6540c8; continue; }
            pc = 0x6540f2; continue;
            }
            0x6540f2 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x6540f8);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,8,rm(m,c.r[0],8));
            pc = 0x6540e4; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065444b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65444b;
    loop {
        match pc {
            0x65444b => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0x1cu32);
            c.set(0,32,c.r[5].wrapping_add(0xffffffe4));
            c.push(m,c.get(3,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x654464);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m, 0x65446c);
            { let args = []; c.r[0] = host.call(m, 0x659754, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,0x66a194u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,0x1u32);
            c.push(m,c.get(3,32));
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(6,32)); }
            { let target = c.get(7,32); c.push(m, 0x65447f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,0x1u32);
            c.push(m,c.get(3,32));
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(0,32)); }
            { let target = c.get(7,32); c.push(m, 0x654489); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff4),32); c.sub(a,c.get(3,32),32); }
            c.set(7,32,rm(m,0x66a150u32,32));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x65451a; continue; }
            pc = 0x65449d; continue;
            }
            0x65449d => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x654515; continue; }
            pc = 0x6544a2; continue;
            }
            0x6544a2 => {
            c.set(1,32,c.r[6].wrapping_add(0x2c));
            { let a = c.get(0,8); let r = c.or(a,0x4u32,8); c.set(0,8,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe4));
            c.push(m,c.get(0,32));
            c.push(m,0x654153u32);
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m, 0x6544bd);
            { let args = []; c.r[0] = host.call(m, 0x63e028, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x654524; continue; }
            pc = 0x6544c7; continue;
            }
            0x6544c7 => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a188u32,32); c.push(m, 0x6544ce); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xffffffffu32);
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff4),32); c.push(m, v); }
            { let target = rm(m,0x66a2b8u32,32); c.push(m, 0x6544d9); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff4),32); c.push(m, v); }
            { let target = c.get(7,32); c.push(m, 0x6544de); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[5].wrapping_add(0x8),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x6544ed; continue; }
            pc = 0x6544e4; continue;
            }
            0x6544e4 => {
            { let v = rm(m,c.r[6].wrapping_add(0x28),32); c.push(m, v); }
            { let target = rm(m,0x66a190u32,32); c.push(m, 0x6544ed); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6544ed; continue;
            }
            0x6544ed => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x654507; continue; }
            pc = 0x6544f2; continue;
            }
            0x6544f2 => {
            c.push(m,0xffffffffu32);
            { let v = rm(m,c.r[6].wrapping_add(0x28),32); c.push(m, v); }
            { let target = rm(m,0x66a2b8u32,32); c.push(m, 0x6544fd); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = rm(m,c.r[6].wrapping_add(0x28),32); c.push(m, v); }
            { let target = c.get(7,32); c.push(m, 0x654502); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            pc = 0x65451f; continue;
            }
            0x654507 => {
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.push(m, v); }
            { let target = rm(m,0x66a184u32,32); c.push(m, 0x654510); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x654526; continue;
            }
            0x654515 => {
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff4),32); c.push(m, v); }
            { let target = c.get(7,32); c.push(m, 0x65451a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x65451a; continue;
            }
            0x65451a => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x654524; continue; }
            pc = 0x65451f; continue;
            }
            0x65451f => {
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.push(m, v); }
            { let target = c.get(7,32); c.push(m, 0x654524); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x654524; continue;
            }
            0x654524 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x654526; continue;
            }
            0x654526 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00654dad(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x654dad;
    loop {
        match pc {
            0x654dad => {
            c.set(0,32,0x668d5cu32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x654db7; }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[1].wrapping_add(0xc),32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(4,32)); }
            if c.cond(4) { pc = 0x654dd6; continue; }
            pc = 0x654dc7; continue;
            }
            0x654dc7 => {
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,c.r[1].wrapping_add(0xc),32); c.push(m, 0x654dd0); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            pc = 0x654dd6; continue;
            }
            0x654dd6 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00655300(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x655300;
    loop {
        match pc {
            0x655300 => {
            c.set(0,32,0x668d8au32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x65530a; }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,0xau32);
            c.set(1,32,c.r[6].wrapping_add(0x4));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x65531c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ed1d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(7,32,c.r[6].wrapping_add(0x20));
            c.push(m,0x4u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x65532c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ed1d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,0x7u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            { let a = c.r[6]; wm(m,a,32,0x67d158u32); }
            c.push(m, 0x655341);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ed60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = c.r[6].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006557a8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6557a8;
    loop {
        match pc {
            0x6557a8 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,0x67cb00u32); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065585f(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65585f;
    loop {
        match pc {
            0x65585f => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x65586f; continue; }
            pc = 0x65586b; continue;
            }
            0x65586b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x655891; continue;
            }
            0x65586f => {
            c.push(m,0x1u32);
            c.push(m, 0x655876);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6557d8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[0].wrapping_add(0x4));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x655882);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64eec4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let v = rm(m,c.r[6].wrapping_add(0x4),32); c.push(m, v); }
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xc),32); c.push(m, 0x65588e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x655891; continue;
            }
            0x655891 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00655896(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x655896;
    loop {
        match pc {
            0x655896 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x6558b7; continue; }
            pc = 0x6558a1; continue;
            }
            0x6558a1 => {
            c.push(m,0x0u32);
            c.push(m, 0x6558a8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6557d8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6558b7; continue; }
            pc = 0x6558ac; continue;
            }
            0x6558ac => {
            { let v = rm(m,c.r[6].wrapping_add(0x4),32); c.push(m, v); }
            c.set(1,32,c.r[0].wrapping_add(0x4));
            c.push(m, 0x6558b7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ef14, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6558b7; continue;
            }
            0x6558b7 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x14),32); c.push(m, 0x6558be); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006558c7(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6558c7;
    loop {
        match pc {
            0x6558c7 => {
            c.set(0,32,0x668a70u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x6558d1; }
            c.push(m,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[1]; wm(m,a,32,0x67cb00u32); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x6558f1; continue; }
            pc = 0x6558e5; continue;
            }
            0x6558e5 => {
            c.push(m, 0x6558ea);
            { let args = []; c.r[0] = host.call(m, 0x655896, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a0b4u32,32); c.push(m, 0x6558f1); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6558f1; continue;
            }
            0x6558f1 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00655fb8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x655fb8;
    loop {
        match pc {
            0x655fb8 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x100u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let target = rm(m,0x66a29cu32,32); c.push(m, 0x655fcb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.set(0,32,0x100u32);
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x655fff; continue; }
            pc = 0x655fd6; continue;
            }
            0x655fd6 => {
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffff00));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let target = rm(m,0x66a3b0u32,32); c.push(m, 0x655fe7); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x655fff; continue; }
            pc = 0x655feb; continue;
            }
            0x655feb => {
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.set(0,32,c.r[5].wrapping_add(0xffffff00));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a19cu32,32); c.push(m, 0x655ffb); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x65600b; continue; }
            pc = 0x655fff; continue;
            }
            0x655fff => {
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let target = rm(m,0x66a434u32,32); c.push(m, 0x65600b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x65600b; continue;
            }
            0x65600b => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006560a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6560a0;
    loop {
        match pc {
            0x6560a0 => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x6560d2; continue; }
            pc = 0x6560a9; continue;
            }
            0x6560a9 => {
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a1b4u32,32); c.push(m, 0x6560b0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x6560cb; continue; }
            pc = 0x6560bc; continue;
            }
            0x6560bc => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.r[0].wrapping_add(0x1));
            pc = 0x6560c0; continue;
            }
            0x6560c0 => {
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a0f8u32,32); c.push(m, 0x6560c7); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            if c.cond(5) { pc = 0x6560c0; continue; }
            pc = 0x6560ca; continue;
            }
            0x6560ca => {
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x6560cb; continue;
            }
            0x6560cb => {
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a0f4u32,32); c.push(m, 0x6560d2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6560d2; continue;
            }
            0x6560d2 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006566d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6566d0;
    loop {
        match pc {
            0x6566d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let v = rm(m,c.r[4].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x6566dc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6566e7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,0x1u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006566e7(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6566e7;
    loop {
        match pc {
            0x6566e7 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m, 0x6566fc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x653310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x656707; continue; }
            pc = 0x656702; continue;
            }
            0x656702 => {
            c.push(m, 0x656707);
            { let args = []; c.r[0] = host.call(m, 0x64e4b5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x656707; continue;
            }
            0x656707 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006567f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6567f0;
    loop {
        match pc {
            0x6567f0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.push(m, 0x656805);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6566d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            c.set(7,32,c.get(0,32));
            if c.cond(4) { pc = 0x656841; continue; }
            pc = 0x65680c; continue;
            }
            0x65680c => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,0x20u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a3b0u32,32); c.push(m, 0x656819); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,c.r[5].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x656829);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x65674b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x65685c; continue; }
            pc = 0x65682d; continue;
            }
            0x65682d => {
            c.push(m,0xffffffffu32);
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m, 0x656838);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x656cdd, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x65683f);
            { let args = []; c.r[0] = host.call(m, 0x656716, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x65685c; continue;
            }
            0x656841 => {
            c.set(0,32,c.r[5].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a43cu32,32); c.push(m, 0x656852); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x65685c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x655fb8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x65685c; continue;
            }
            0x65685c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656bbe(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656bbe;
    loop {
        match pc {
            0x656bbe => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x114u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(7,32,c.get(1,32));
            c.push(m,c.get(3,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x656bd7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x656b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[5].wrapping_add(0xfffffffc));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x656be1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x656d36, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x656c02; continue; }
            pc = 0x656bea; continue;
            }
            0x656bea => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m,0x376u32);
            { let v = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.push(m, v); }
            { let target = rm(m,0x66a4a0u32,32); c.push(m, 0x656bfa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x656c02; continue; }
            pc = 0x656bfe; continue;
            }
            0x656bfe => {
            c.set(6,32,c.get(0,32));
            pc = 0x656c0c; continue;
            }
            0x656c02 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x656c0c; continue; }
            pc = 0x656c06; continue;
            }
            0x656c06 => {
            c.set(6,32,c.r[7].wrapping_add(0x9c));
            pc = 0x656c0c; continue;
            }
            0x656c0c => {
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x656c26; continue; }
            pc = 0x656c13; continue;
            }
            0x656c13 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x656c26; continue; }
            pc = 0x656c1f; continue;
            }
            0x656c1f => {
            { let a = c.get(0,32); let r = c.add(a,0x30000u32,32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x656c26; continue;
            }
            0x656c26 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(3,8); c.and(a,0xf0u32,8); }
            if c.cond(5) { pc = 0x656c45; continue; }
            pc = 0x656c2e; continue;
            }
            0x656c2e => {
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(6) { pc = 0x656c42; continue; }
            pc = 0x656c38; continue;
            }
            0x656c38 => {
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(6) { pc = 0x656c45; continue; }
            pc = 0x656c3d; continue;
            }
            0x656c3d => {
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x656c45; continue; }
            pc = 0x656c42; continue;
            }
            0x656c42 => {
            { let a = c.get(3,32); let r = c.or(a,0x30u32,32); c.set(3,32,r); }
            pc = 0x656c45; continue;
            }
            0x656c45 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x656c4e; continue; }
            pc = 0x656c49; continue;
            }
            0x656c49 => {
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x78),32));
            pc = 0x656c68; continue;
            }
            0x656c4e => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffeec));
            c.push(m,0x104u32);
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.set(7,32,c.r[5].wrapping_add(0xfffffeec));
            { let target = rm(m,0x66a174u32,32); c.push(m, 0x656c68); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x656c68; continue;
            }
            0x656c68 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff4),32); c.push(m, v); }
            { let target = rm(m,0x66a414u32,32); c.push(m, 0x656c76); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(7,32,c.get(0,32));
            if c.cond(4) { pc = 0x656c81; continue; }
            pc = 0x656c7c; continue;
            }
            0x656c7c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x656c81; continue;
            }
            0x656c81 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x656c92; continue; }
            pc = 0x656c87; continue;
            }
            0x656c87 => {
            c.push(m,0x1u32);
            { let v = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.push(m, v); }
            { let target = rm(m,0x66a4dcu32,32); c.push(m, 0x656c92); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x656c92; continue;
            }
            0x656c92 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff0),32));
            c.push(m,0x1u32);
            c.push(m, 0x656c9c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x656b90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065732c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65732c;
    loop {
        match pc {
            0x65732c => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x657334);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0x1070u32,32); c.set(0,32,r); }
            c.push(m,0x6595b2u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x657345);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a5cf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[0].wrapping_add(0x8));
            c.push(m, 0x65734e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a144, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00657350(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x657350;
    loop {
        match pc {
            0x657350 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x657358);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0x1070u32,32); c.set(0,32,r); }
            c.push(m,0x6595b2u32);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x657369);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a5cf, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[0].wrapping_add(0x8));
            c.push(m, 0x657372);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a157, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00659d49(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x659d49;
    loop {
        match pc {
            0x659d49 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x5c));
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x28); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x64); wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,0x66a480u32,32); c.push(m, 0x659d6b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x58); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x54); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x24); wm(m,a,32,0x1u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065a1cf(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65a1cf;
    loop {
        match pc {
            0x65a1cf => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,0x4u32); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x66a1bcu32,32); c.push(m, 0x65a1f7); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x65a203; continue; }
            pc = 0x65a1fe; continue;
            }
            0x65a1fe => {
            c.push(m, 0x65a203);
            { let args = []; c.r[0] = host.call(m, 0x64e49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x65a203; continue;
            }
            0x65a203 => {
            c.set(0,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1a4u32,32); c.push(m, 0x65a20d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065a268(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65a268;
    loop {
        match pc {
            0x65a268 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x66a1b0u32,32); c.push(m, 0x65a27d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(13) { pc = 0x65a294; continue; }
            pc = 0x65a287; continue;
            }
            0x65a287 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x65a34d; continue; }
            pc = 0x65a294; continue;
            }
            0x65a294 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(5,32); c.sub(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x65a2b6; continue; }
            pc = 0x65a29b; continue;
            }
            0x65a29b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            pc = 0x65a2a1; continue;
            }
            0x65a2a1 => {
            { let a = rm(m,c.r[0],8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x65a2ae; continue; }
            pc = 0x65a2a6; continue;
            }
            0x65a2a6 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x65a2a1; continue; }
            pc = 0x65a2ae; continue;
            }
            0x65a2ae => {
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x65a34d; continue; }
            pc = 0x65a2b6; continue;
            }
            0x65a2b6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.add(a,0x20u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(3,32));
            if c.cond(5) { pc = 0x65a2d4; continue; }
            pc = 0x65a2c1; continue;
            }
            0x65a2c1 => {
            c.set(0,32,c.get(5,32));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x2002u32);
            { let target = rm(m,0x66a1a0u32,32); c.push(m, 0x65a2d2); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x65a2f6; continue;
            }
            0x65a2d4 => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1c0u32,32); c.push(m, 0x65a2db); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a0f8u32,32); c.push(m, 0x65a2e4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(5,32));
            c.push(m,0x2002u32);
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a1c8u32,32); c.push(m, 0x65a2f6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x65a2f6; continue;
            }
            0x65a2f6 => {
            c.set(3,32,rm(m,0x66a0fcu32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x65a31f; continue; }
            pc = 0x65a304; continue;
            }
            0x65a304 => {
            { let v = rm(m,c.r[6].wrapping_add(0x10),32); c.push(m, v); }
            { let target = rm(m,0x66a1c0u32,32); c.push(m, 0x65a30d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let target = c.get(3,32); c.push(m, 0x65a310); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            { let target = rm(m,0x66a1acu32,32); c.push(m, 0x65a31a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m, 0x65a31f);
            { let args = []; c.r[0] = host.call(m, 0x64e49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x65a31f; continue;
            }
            0x65a31f => {
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            { let target = c.get(3,32); c.push(m, 0x65a325); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,0x1fffffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x65a343);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x65a34d; continue;
            }
            0x65a34d => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc),32),32); }
            if c.cond(12) { pc = 0x65a358; continue; }
            pc = 0x65a352; continue;
            }
            0x65a352 => {
            c.set(0,32,c.r[7].wrapping_add(0x1));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x65a358; continue;
            }
            0x65a358 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)),32); let r = c.or(a,0x1u32,32); { let a = c.r[0].wrapping_add(c.r[7].wrapping_mul(8)); wm(m,a,32,r); } }
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            c.set(0,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(8)));
            c.set(0,32,c.r[7].wrapping_add(0x1));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let target = rm(m,0x66a1acu32,32); c.push(m, 0x65a372); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065a3d7(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65a3d7;
    loop {
        match pc {
            0x65a3d7 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[6],32); c.push(m, v); }
            { let target = rm(m,0x66a1d4u32,32); c.push(m, 0x65a3e8); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x65a408; continue; }
            pc = 0x65a3ee; continue;
            }
            0x65a3ee => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(3,32,c.r[7].wrapping_add(0x8));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[3],32),32); }
            if c.cond(12) { pc = 0x65a4ae; continue; }
            pc = 0x65a3fc; continue;
            }
            0x65a3fc => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x65a4ae; continue; }
            pc = 0x65a406; continue;
            }
            0x65a406 => {
            pc = 0x65a44a; continue;
            }
            0x65a408 => {
            c.push(m,0x10u32);
            c.push(m, 0x65a40f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a19d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x65a41d; continue; }
            pc = 0x65a413; continue;
            }
            0x65a413 => {
            { let a = c.r[0]; wm(m,a,32,0x67d150u32); }
            c.set(7,32,c.get(0,32));
            pc = 0x65a41f; continue;
            }
            0x65a41d => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x65a41f; continue;
            }
            0x65a41f => {
            { let a = rm(m,c.r[7].wrapping_add(0x8),32); let r = c.and(a,0x0u32,32); { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[7].wrapping_add(0xc),32); let r = c.and(a,0x0u32,32); { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,r); } }
            c.set(3,32,c.r[7].wrapping_add(0x8));
            c.set(0,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,0x66a1b0u32,32); c.push(m, 0x65a437); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x14));
            c.push(m, 0x65a440);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65a144, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.r[6].wrapping_add(0x1c));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1acu32,32); c.push(m, 0x65a44a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x65a44a; continue;
            }
            0x65a44a => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x65a462; continue; }
            pc = 0x65a451; continue;
            }
            0x65a451 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            { let target = rm(m,0x66a1b8u32,32); c.push(m, 0x65a460); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x65a472; continue;
            }
            0x65a462 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,0x2u32);
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1d0u32,32); c.push(m, 0x65a472); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x65a472; continue;
            }
            0x65a472 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x65a47e; continue; }
            pc = 0x65a479; continue;
            }
            0x65a479 => {
            c.push(m, 0x65a47e);
            { let args = []; c.r[0] = host.call(m, 0x64e49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x65a47e; continue;
            }
            0x65a47e => {
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.imul(a,0x3fffffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0xc),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.push(m,0x0u32);
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(0,32));
            c.push(m, 0x65a49d);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[6],32); c.push(m, v); }
            { let target = rm(m,0x66a1ccu32,32); c.push(m, 0x65a4ae); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x65a4ae; continue;
            }
            0x65a4ae => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)); wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065a9f9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65a9f9;
    loop {
        match pc {
            0x65a9f9 => {
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,0x6f00fcu32,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x65aa09; continue; }
            pc = 0x65aa04; continue;
            }
            0x65aa04 => {
            c.push(m, 0x65aa09);
            { let args = []; c.r[0] = host.call(m, 0x65a966, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x65aa09; continue;
            }
            0x65aa09 => {
            { let a = rm(m,0x6f00f8u32,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x65aa65; continue; }
            pc = 0x65aa11; continue;
            }
            0x65aa11 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = rm(m,c.r[7].wrapping_mul(4).wrapping_add(0x6eff00),32); c.sub(a,c.get(3,32),32); }
            c.set(3,32,rm(m,0x66a1b0u32,32));
            c.set(6,32,c.r[7].wrapping_mul(4).wrapping_add(0x6eff00));
            if c.cond(5) { pc = 0x65aa56; continue; }
            pc = 0x65aa2d; continue;
            }
            0x65aa2d => {
            c.push(m,c.get(5,32));
            c.set(5,32,0x6eff48u32);
            c.push(m,c.get(5,32));
            { let target = c.get(3,32); c.push(m, 0x65aa36); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x65aa4e; continue; }
            pc = 0x65aa3b; continue;
            }
            0x65aa3b => {
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_mul(8).wrapping_add(0x6eff60));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1a4u32,32); c.push(m, 0x65aa4c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6],32); let r = c.inc(a,32); { let a = c.r[6]; wm(m,a,32,r); } }
            pc = 0x65aa4e; continue;
            }
            0x65aa4e => {
            c.push(m,c.get(5,32));
            { let target = rm(m,0x66a1acu32,32); c.push(m, 0x65aa55); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x65aa56; continue;
            }
            0x65aa56 => {
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_mul(8).wrapping_add(0x6eff60));
            c.push(m,c.get(0,32));
            { let target = c.get(3,32); c.push(m, 0x65aa63); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x65aa65; continue;
            }
            0x65aa65 => {
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065aa69(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65aa69;
    loop {
        match pc {
            0x65aa69 => {
            { let a = rm(m,0x6f00f8u32,32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x65aa87; continue; }
            pc = 0x65aa72; continue;
            }
            0x65aa72 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_mul(8).wrapping_add(0x6eff60));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1acu32,32); c.push(m, 0x65aa87); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x65aa87; continue;
            }
            0x65aa87 => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0042c020(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42c020;
    loop {
        match pc {
            0x42c020 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); c.push(m, 0x42c02e); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7b4),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x42c074; continue; }
            pc = 0x42c038; continue;
            }
            0x42c038 => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            c.push(m,0x0u32);
            { let a = c.r[6].wrapping_add(0x7b4); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x494),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x494));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x42c061); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7b4),8));
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x7e8); wm(m,a,32,0x0u32); }
            pc = 0x42c074; continue;
            }
            0x42c074 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004d7d50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d7d50;
    loop {
        match pc {
            0x4d7d50 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x662a21u32);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6e51acu32,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x4d7ff8; continue; }
            pc = 0x4d7d7b; continue;
            }
            0x4d7d7b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4d7ff8; continue; }
            pc = 0x4d7d8c; continue;
            }
            0x4d7d8c => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4d7d9c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,16,rm(m,c.r[0].wrapping_add(0xa),16));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,0x6dd736u32,16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(0,32,c.r[6].wrapping_mul(4).wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d7dbc);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x4d7de3; continue; }
            pc = 0x4d7dcb; continue;
            }
            0x4d7dcb => {
            c.push(m,0x64f253u32);
            c.push(m,0x4103c0u32);
            c.set(3,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(6,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x4d7de3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x4d7de3; continue;
            }
            0x4d7de3 => {
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.set(0,16,rm(m,0x6dd736u32,16));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x1u32); }
            { let a = c.get(7,16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x4d7f97; continue; }
            pc = 0x4d7e04; continue;
            }
            0x4d7e04 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            c.set(2,16,c.get(0,16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(7) { pc = 0x4d7e1f; continue; }
            pc = 0x4d7e13; continue;
            }
            0x4d7e13 => {
            c.set(0,32,rm(m,0x6dd730u32,32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            pc = 0x4d7e21; continue;
            }
            0x4d7e1f => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4d7e21; continue;
            }
            0x4d7e21 => {
            { let a = rm(m,c.r[7].wrapping_add(0x1),8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x4d7f87; continue; }
            pc = 0x4d7e2b; continue;
            }
            0x4d7e2b => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x26),8));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4d7e44; continue; }
            pc = 0x4d7e36; continue;
            }
            0x4d7e36 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = rm(m,c.r[1].wrapping_add(0x6c0198),8); c.sub(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4d7e68; continue; }
            pc = 0x4d7e44; continue;
            }
            0x4d7e44 => {
            { let a = c.get(0,8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x4d7e56; continue; }
            pc = 0x4d7e48; continue;
            }
            0x4d7e48 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = rm(m,c.r[2].wrapping_add(0x6c0198),8); c.sub(a,0x2u32,8); }
            if c.cond(4) { pc = 0x4d7e68; continue; }
            pc = 0x4d7e56; continue;
            }
            0x4d7e56 => {
            { let a = c.get(0,8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x4d7e84; continue; }
            pc = 0x4d7e5a; continue;
            }
            0x4d7e5a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = rm(m,c.r[0].wrapping_add(0x6c0198),8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x4d7e84; continue; }
            pc = 0x4d7e68; continue;
            }
            0x4d7e68 => {
            c.set(1,32,rm(m,0x6cbdb8u32,32));
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x40u32);
            c.push(m,0xffffffffu32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,0x1770u32);
            c.push(m, 0x4d7e84);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28))]; c.r[0] = host.call(m, 0x443150, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(32);
            pc = 0x4d7e84; continue;
            }
            0x4d7e84 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x26),8));
            { let a = c.get(2,8); c.and(a,0x8u32,8); }
            if c.cond(4) { pc = 0x4d7ed9; continue; }
            pc = 0x4d7e8c; continue;
            }
            0x4d7e8c => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x6c0198),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4d7ed9; continue; }
            pc = 0x4d7ea0; continue;
            }
            0x4d7ea0 => {
            { let a = c.get(0,8); c.sub(a,0x3cu32,8); }
            if c.cond(2) { pc = 0x4d7ead; continue; }
            pc = 0x4d7ea4; continue;
            }
            0x4d7ea4 => {
            { let a = c.get(0,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x4d7ead; continue; }
            pc = 0x4d7ea8; continue;
            }
            0x4d7ea8 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            pc = 0x4d7eb4; continue;
            }
            0x4d7ead => {
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a52d0),32));
            pc = 0x4d7eb4; continue;
            }
            0x4d7eb4 => {
            c.set(2,32,rm(m,0x6a4570u32,32));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d7ec6);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[1].wrapping_mul(4)));
            pc = 0x4d7f60; continue;
            }
            0x4d7ed9 => {
            { let a = c.get(2,8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x4d7f14; continue; }
            pc = 0x4d7ede; continue;
            }
            0x4d7ede => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x6c0198),8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x4d7f14; continue; }
            pc = 0x4d7ef2; continue;
            }
            0x4d7ef2 => {
            { let a = c.get(0,8); c.sub(a,0x3cu32,8); }
            if c.cond(2) { pc = 0x4d7f05; continue; }
            pc = 0x4d7ef6; continue;
            }
            0x4d7ef6 => {
            { let a = c.get(0,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x4d7f05; continue; }
            pc = 0x4d7efa; continue;
            }
            0x4d7efa => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(1,32,rm(m,0x6a4574u32,32));
            pc = 0x4d7f47; continue;
            }
            0x4d7f05 => {
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a52d0),32));
            c.set(1,32,rm(m,0x6a4574u32,32));
            pc = 0x4d7f47; continue;
            }
            0x4d7f14 => {
            { let a = c.get(2,8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x4d7f6b; continue; }
            pc = 0x4d7f19; continue;
            }
            0x4d7f19 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x6c0198),8); c.sub(a,0x3u32,8); }
            if c.cond(5) { pc = 0x4d7f6b; continue; }
            pc = 0x4d7f2d; continue;
            }
            0x4d7f2d => {
            { let a = c.get(0,8); c.sub(a,0x3cu32,8); }
            if c.cond(2) { pc = 0x4d7f3a; continue; }
            pc = 0x4d7f31; continue;
            }
            0x4d7f31 => {
            { let a = c.get(0,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x4d7f3a; continue; }
            pc = 0x4d7f35; continue;
            }
            0x4d7f35 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            pc = 0x4d7f41; continue;
            }
            0x4d7f3a => {
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a52d0),32));
            pc = 0x4d7f41; continue;
            }
            0x4d7f41 => {
            c.set(1,32,rm(m,0x6a4578u32,32));
            pc = 0x4d7f47; continue;
            }
            0x4d7f47 => {
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4d7f53);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(5,32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            pc = 0x4d7f60; continue;
            }
            0x4d7f60 => {
            c.set(2,32,c.r[4].wrapping_add(0x28));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4d7f6b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4d7f6b; continue;
            }
            0x4d7f6b => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(2,8); let r = c.and(a,0xbfu32,8); c.set(2,8,r); }
            { let a = c.r[7].wrapping_add(0x1); wm(m,a,8,c.get(2,8)); }
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x6),16));
            c.set(0,16,rm(m,0x6dd736u32,16));
            { let a = c.get(7,16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x4d7e04; continue; }
            pc = 0x4d7f87; continue;
            }
            0x4d7f87 => {
            { let a = c.get(5,16); c.and(a,c.get(5,16),16); }
            if c.cond(4) { pc = 0x4d7f97; continue; }
            pc = 0x4d7f8c; continue;
            }
            0x4d7f8c => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x4d7f97);
            { let args = []; c.r[0] = host.call(m, 0x4237d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4d7f97; continue;
            }
            0x4d7f97 => {
            { let a = c.get(5,32); let r = c.and(a,0xffffu32,32); c.set(5,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            c.set(0,32,c.get(5,32));
            if c.cond(8) { pc = 0x4d7fc6; continue; }
            pc = 0x4d7fa2; continue;
            }
            0x4d7fa2 => {
            c.set(6,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[0].wrapping_add(0x1));
            pc = 0x4d7fa8; continue;
            }
            0x4d7fa8 => {
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(6,32));
            c.push(m, 0x4d7fb5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x4d7fc0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42bb20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.sub(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x4d7fa8; continue; }
            pc = 0x4d7fc6; continue;
            }
            0x4d7fc6 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4d7fe7; continue; }
            pc = 0x4d7fca; continue;
            }
            0x4d7fca => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xfffffffc),32));
            c.set(6,32,c.r[3].wrapping_add(0xfffffffc));
            c.push(m,0x64f253u32);
            c.push(m,c.get(0,32));
            c.push(m,0x4u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x4d7fde);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(6,32));
            c.push(m, 0x4d7fe4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4d7fe7; continue;
            }
            0x4d7fe7 => {
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4d7ff8);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4d7ff8; continue;
            }
            0x4d7ff8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0050dde0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x50dde0;
    loop {
        match pc {
            0x50dde0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x100); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x102); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x104); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x106); wm(m,a,16,c.get(0,16)); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x50ded5; continue; }
            pc = 0x50de17; continue;
            }
            0x50de17 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x50de22);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xe),16));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0xf4),16));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x50ded5; continue; }
            pc = 0x50de3b; continue;
            }
            0x50de3b => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            pc = 0x50de3d; continue;
            }
            0x50de3d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[6].wrapping_add(0xf6),16),16); }
            if c.cond(3) { pc = 0x50de98; continue; }
            pc = 0x50de55; continue;
            }
            0x50de55 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(5,16,rm(m,c.r[3].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x50dec5; continue; }
            pc = 0x50de6f; continue;
            }
            0x50de6f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x134),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x50dec5; continue; }
            pc = 0x50de7c; continue;
            }
            0x50de7c => {
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(14) { pc = 0x50de8f; continue; }
            pc = 0x50de81; continue;
            }
            0x50de81 => {
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            if c.cond(5) { pc = 0x50dec5; continue; }
            pc = 0x50de86; continue;
            }
            0x50de86 => {
            { let a = rm(m,c.r[6].wrapping_add(0x106),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x106); wm(m,a,16,r); } }
            pc = 0x50dec5; continue;
            }
            0x50de8f => {
            { let a = rm(m,c.r[6].wrapping_add(0x104),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x104); wm(m,a,16,r); } }
            pc = 0x50dec5; continue;
            }
            0x50de98 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x134),16));
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            if c.cond(7) { pc = 0x50dec5; continue; }
            pc = 0x50dea6; continue;
            }
            0x50dea6 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x50dee8),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x50dedc)); continue;
            }
            0x50deb5 => {
            { let a = rm(m,c.r[6].wrapping_add(0x100),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x100); wm(m,a,16,r); } }
            pc = 0x50dec5; continue;
            }
            0x50debe => {
            { let a = rm(m,c.r[6].wrapping_add(0x102),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x102); wm(m,a,16,r); } }
            pc = 0x50dec5; continue;
            }
            0x50dec5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x50de3d; continue; }
            pc = 0x50ded3; continue;
            }
            0x50ded3 => {
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x50ded5; continue;
            }
            0x50ded5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0054bf80(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x54bf80;
    loop {
        match pc {
            0x54bf80 => {
            { let a = c.get(4,32); let r = c.sub(a,0x430u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(6,32,c.r[4].wrapping_add(0x30));
            c.set(3,32,0x34u32);
            pc = 0x54bf9b; continue;
            }
            0x54bf9b => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x54bfa2);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x14u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x54bf9b; continue; }
            pc = 0x54bfa8; continue;
            }
            0x54bfa8 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x444),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x44),32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            c.set(3,32,rm(m,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x14),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[1]).wrapping_add(0x14));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x44),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[2]));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,0x0u32); }
            { let a = rm(m,c.r[0],32); c.sub(a,0x0u32,32); }
            if c.cond(6) { pc = 0x54c0cc; continue; }
            pc = 0x54bff0; continue;
            }
            0x54bff0 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,32,0xcbu32);
            pc = 0x54bff7; continue;
            }
            0x54bff7 => {
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[4].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(6,32,c.r[4].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x38));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x144),8));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x194),8));
            c.set(1,32,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x16c),8));
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xa4),32));
            { let a = c.r[4].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x40); wm(m,a,32,c.get(1,32)); }
            { let a = rm(m,c.r[4].wrapping_add(0x2e),16); let r = c.inc(a,16); { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,r); } }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            c.set(6,32,rm(m,c.r[0],32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x54bff7; continue; }
            pc = 0x54c0be; continue;
            }
            0x54c0be => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x54c0d0; continue;
            }
            0x54c0cc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            pc = 0x54c0d0; continue;
            }
            0x54c0d0 => {
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[2].wrapping_mul(4).wrapping_add(0x10));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x3c),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a2f0u32,32); c.push(m, 0x54c0fa); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x54c270; continue; }
            pc = 0x54c108; continue;
            }
            0x54c108 => {
            c.push(m,0x410u32);
            c.set(2,32,c.r[4].wrapping_add(0x34));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x3a); wm(m,a,16,0x0u32); }
            c.push(m, 0x54c120);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1u32); }
            pc = 0x54c12b; continue;
            }
            0x54c12b => {
            { let a = c.get(3,32); c.sub(a,0x28u32,32); }
            if c.cond(7) { pc = 0x54c1fb; continue; }
            pc = 0x54c134; continue;
            }
            0x54c134 => {
            { let a = rm(m,c.r[4].wrapping_add(0x2e),16); c.sub(a,0x34u32,16); }
            if c.cond(3) { pc = 0x54c1fb; continue; }
            pc = 0x54c140; continue;
            }
            0x54c140 => {
            { let a = c.get(3,32); c.sub(a,0x28u32,32); }
            if c.cond(5) { pc = 0x54c175; continue; }
            pc = 0x54c145; continue;
            }
            0x54c145 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x30); wm(m,a,32,0xccu32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            { let a = rm(m,c.r[4].wrapping_add(0x2e),16); let r = c.inc(a,16); { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,r); } }
            c.set(3,32,0x29u32);
            pc = 0x54c12b; continue;
            }
            0x54c175 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x1bc),32));
            c.set(5,32,c.r[2].wrapping_add(c.r[3].wrapping_mul(4)).wrapping_add(0x1bc));
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffff8),32),32); }
            if c.cond(2) { pc = 0x54c194; continue; }
            pc = 0x54c18b; continue;
            }
            0x54c18b => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            pc = 0x54c12b; continue;
            }
            0x54c194 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x30); wm(m,a,32,0xccu32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(c.r[1]).wrapping_add(0x34));
            c.set(6,32,c.r[4].wrapping_add(c.r[1]).wrapping_add(0x44));
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x54c1cb; continue; }
            pc = 0x54c1c6; continue;
            }
            0x54c1c6 => {
            { let a = c.r[0]; wm(m,a,8,0x0u32); }
            pc = 0x54c1ce; continue;
            }
            0x54c1cb => {
            { let a = c.r[0]; wm(m,a,8,0x7fu32); }
            pc = 0x54c1ce; continue;
            }
            0x54c1ce => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x54c1e9; continue; }
            pc = 0x54c1d3; continue;
            }
            0x54c1d3 => {
            c.set(1,32,rm(m,c.r[5],32));
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[1].wrapping_add(0xfffffff8),32),32); }
            if c.cond(3) { pc = 0x54c1e9; continue; }
            pc = 0x54c1db; continue;
            }
            0x54c1db => {
            c.set(2,32,c.get(7,32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,c.get(2,8)); }
            if c.cond(12) { pc = 0x54c1d3; continue; }
            pc = 0x54c1e9; continue;
            }
            0x54c1e9 => {
            { let a = rm(m,c.r[4].wrapping_add(0x2e),16); let r = c.inc(a,16); { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,r); } }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x54c12b; continue;
            }
            0x54c1fb => {
            { let a = rm(m,c.r[4].wrapping_add(0x2e),16); c.sub(a,0x0u32,16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x54c26b; continue; }
            pc = 0x54c20b; continue;
            }
            0x54c20b => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x44),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x14),32));
            c.set(0,32,c.r[6].wrapping_add(c.r[0]).wrapping_add(0x14));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x44),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2e),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(0,32,c.r[0].wrapping_mul(4).wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x3c),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let target = rm(m,0x66a2f0u32,32); c.push(m, 0x54c252); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x54c108; continue; }
            pc = 0x54c25e; continue;
            }
            0x54c25e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x430u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x54c26b => {
            c.set(0,32,0x1u32);
            pc = 0x54c270; continue;
            }
            0x54c270 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x430u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006389fa(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6389fa;
    loop {
        match pc {
            0x6389fa => {
            c.push(m, 0x6389ff);
            { let args = []; c.r[0] = host.call(m, 0x654281, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x638a0b; continue; }
            pc = 0x638a03; continue;
            }
            0x638a03 => {
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x74),32); c.push(m, 0x638a0a); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x638a0b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063d88b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63d88b;
    loop {
        match pc {
            0x63d88b => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0x8),8),8)&0xffffffffu32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,0x5au32,32); }
            c.push(m,c.get(7,32));
            if c.cond(15) { pc = 0x63da6d; continue; }
            pc = 0x63d89e; continue;
            }
            0x63d89e => {
            if c.cond(4) { pc = 0x63dba9; continue; }
            pc = 0x63d8a4; continue;
            }
            0x63d8a4 => {
            { let a = c.get(0,32); c.sub(a,0x4du32,32); }
            if c.cond(15) { pc = 0x63d975; continue; }
            pc = 0x63d8ad; continue;
            }
            0x63d8ad => {
            if c.cond(4) { pc = 0x63d95b; continue; }
            pc = 0x63d8b3; continue;
            }
            0x63d8b3 => {
            { let a = c.get(0,32); let r = c.sub(a,0x25u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63d947; continue; }
            pc = 0x63d8bc; continue;
            }
            0x63d8bc => {
            { let a = c.get(0,32); let r = c.sub(a,0x1cu32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63d92f; continue; }
            pc = 0x63d8c1; continue;
            }
            0x63d8c1 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63d917; continue; }
            pc = 0x63d8c4; continue;
            }
            0x63d8c4 => {
            { let a = c.get(0,32); let r = c.sub(a,0x6u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63d8fa; continue; }
            pc = 0x63d8c9; continue;
            }
            0x63d8c9 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x63dc75; continue; }
            pc = 0x63d8d0; continue;
            }
            0x63d8d0 => {
            c.set(0,32,rm(m,0x6f1a98u32,32));
            c.push(m,0xcu32);
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x63d8ec; continue; }
            pc = 0x63d8ea; continue;
            }
            0x63d8ea => {
            c.push(m,c.get(1,32));
            { let v = c.pop(m); c.set(2,32,v); }
            pc = 0x63d8ec; continue;
            }
            0x63d8ec => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,0x2u32);
            pc = 0x63d8f4; continue;
            }
            0x63d8f4 => {
            c.push(m,c.get(2,32));
            pc = 0x63d9c7; continue;
            }
            0x63d8fa => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.set(0,32,rm(m,0x6f1a98u32,32));
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,0x2u32);
            { let v = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, v); }
            pc = 0x63d9c7; continue;
            }
            0x63d917 => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x10),32));
            { let v = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x68),32); c.push(m, v); }
            pc = 0x63dc6d; continue;
            }
            0x63d92f => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let v = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1c),32); c.push(m, v); }
            pc = 0x63dc6d; continue;
            }
            0x63d947 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[1]; wm(m,a,8,0x25u32); }
            { let a = rm(m,c.r[0],32); let r = c.inc(a,32); { let a = c.r[0]; wm(m,a,32,r); } }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = rm(m,c.r[0],32); let r = c.dec(a,32); { let a = c.r[0]; wm(m,a,32,r); } }
            pc = 0x63dc75; continue;
            }
            0x63d95b => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.set(0,32,rm(m,0x6f1a98u32,32));
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,0x2u32);
            { let v = rm(m,c.r[0].wrapping_add(0x4),32); c.push(m, v); }
            pc = 0x63d9c7; continue;
            }
            0x63d975 => {
            { let a = c.get(0,32); let r = c.sub(a,0x53u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63da51; continue; }
            pc = 0x63d97e; continue;
            }
            0x63d97e => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63da11; continue; }
            pc = 0x63d986; continue;
            }
            0x63d986 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63d9f3; continue; }
            pc = 0x63d98a; continue;
            }
            0x63d98a => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63d9d4; continue; }
            pc = 0x63d98d; continue;
            }
            0x63d98d => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x63dc75; continue; }
            pc = 0x63d994; continue;
            }
            0x63d994 => {
            c.set(0,32,rm(m,0x6f1a98u32,32));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x14),32));
            c.push(m,0x4u32);
            c.set(0,32,c.get(1,32));
            c.push(m,0x64u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(6,32,v); }
            { let b = c.get(6,32); c.idiv32(b); }
            c.push(m,0x64u32);
            c.set(6,32,c.get(0,32));
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(6,32); let r = c.add(a,0x13u32,32); c.set(6,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(6,32); let r = c.imul(a,0x64u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            pc = 0x63d9c7; continue;
            }
            0x63d9c7 => {
            c.push(m, 0x63d9cc);
            { let args = []; c.r[0] = host.call(m, 0x63dca1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x63dc75; continue;
            }
            0x63d9d4 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = rm(m,0x6f1a98u32,32); let r = c.and(a,0x0u32,32); { let a = 0x6f1a98u32; wm(m,a,32,r); } }
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[0].wrapping_add(0xa8),32); c.push(m, v); }
            pc = 0x63dc11; continue;
            }
            0x63d9f3 => {
            c.set(0,32,rm(m,0x6f1a98u32,32));
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x63da0c; continue; }
            pc = 0x63da07; continue;
            }
            0x63da07 => {
            c.push(m,0x6u32);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x63da21; continue;
            }
            0x63da0c => {
            c.set(6,32,c.r[1].wrapping_add(0xffffffff));
            pc = 0x63da21; continue;
            }
            0x63da11 => {
            c.set(0,32,rm(m,0x6f1a98u32,32));
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x18),32));
            pc = 0x63da21; continue;
            }
            0x63da21 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x1c),32));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x63da2c; continue; }
            pc = 0x63da28; continue;
            }
            0x63da28 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x63da43; continue;
            }
            0x63da2c => {
            c.set(0,32,c.get(1,32));
            c.push(m,0x7u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(7,32,v); }
            { let b = c.get(7,32); c.idiv32(b); }
            c.push(m,0x7u32);
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(1,32,v); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x63da43; continue; }
            pc = 0x63da42; continue;
            }
            0x63da42 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x63da43; continue;
            }
            0x63da43 => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            pc = 0x63d9c7; continue;
            }
            0x63da51 => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.set(0,32,rm(m,0x6f1a98u32,32));
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,0x2u32);
            { let v = rm(m,c.r[0],32); c.push(m, v); }
            pc = 0x63d9c7; continue;
            }
            0x63da6d => {
            { let a = c.get(0,32); c.sub(a,0x6du32,32); }
            if c.cond(15) { pc = 0x63db8a; continue; }
            pc = 0x63da76; continue;
            }
            0x63da76 => {
            if c.cond(4) { pc = 0x63db6d; continue; }
            pc = 0x63da7c; continue;
            }
            0x63da7c => {
            { let a = c.get(0,32); let r = c.sub(a,0x61u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63db56; continue; }
            pc = 0x63da85; continue;
            }
            0x63da85 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63db3e; continue; }
            pc = 0x63da8c; continue;
            }
            0x63da8c => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63dad7; continue; }
            pc = 0x63da8f; continue;
            }
            0x63da8f => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63daba; continue; }
            pc = 0x63da92; continue;
            }
            0x63da92 => {
            { let a = c.get(0,32); let r = c.sub(a,0x6u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x63dc75; continue; }
            pc = 0x63da9b; continue;
            }
            0x63da9b => {
            c.set(0,32,rm(m,0x6f1a98u32,32));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x1c),32));
            c.push(m,0x3u32);
            pc = 0x63dab3; continue;
            }
            0x63dab3 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            pc = 0x63d9c7; continue;
            }
            0x63daba => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.set(0,32,rm(m,0x6f1a98u32,32));
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,0x2u32);
            { let v = rm(m,c.r[0].wrapping_add(0xc),32); c.push(m, v); }
            pc = 0x63d9c7; continue;
            }
            0x63dad7 => {
            { let a = rm(m,0x6f1a98u32,32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x63dafe; continue; }
            pc = 0x63dae0; continue;
            }
            0x63dae0 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x18),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = rm(m,0x6f1a98u32,32); let r = c.and(a,0x0u32,32); { let a = 0x6f1a98u32; wm(m,a,32,r); } }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[3].wrapping_add(0xa4),32); c.push(m, v); }
            pc = 0x63db13; continue;
            }
            0x63dafe => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x18),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[3].wrapping_add(0xa0),32); c.push(m, v); }
            pc = 0x63db13; continue;
            }
            0x63db13 => {
            c.push(m, 0x63db18);
            { let args = []; c.r[0] = host.call(m, 0x63dd59, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x63dc75; continue; }
            pc = 0x63db24; continue;
            }
            0x63db24 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let a = c.r[0]; wm(m,a,8,0x20u32); }
            { let a = rm(m,c.r[7],32); let r = c.inc(a,32); { let a = c.r[7]; wm(m,a,32,r); } }
            { let a = rm(m,c.r[6],32); let r = c.dec(a,32); { let a = c.r[6]; wm(m,a,32,r); } }
            { let v = rm(m,c.r[3].wrapping_add(0xa8),32); c.push(m, v); }
            pc = 0x63dc11; continue;
            }
            0x63db3e => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x10),32));
            { let v = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x38),32); c.push(m, v); }
            pc = 0x63dc6d; continue;
            }
            0x63db56 => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x18),32));
            { let v = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32); c.push(m, v); }
            pc = 0x63dc6d; continue;
            }
            0x63db6d => {
            c.set(0,32,rm(m,0x6f1a98u32,32));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x10),32));
            c.push(m,0x2u32);
            pc = 0x63dab3; continue;
            }
            0x63db8a => {
            { let a = c.get(0,32); let r = c.sub(a,0x70u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63dc4d; continue; }
            pc = 0x63db93; continue;
            }
            0x63db93 => {
            { let a = c.get(0,32); let r = c.sub(a,0x7u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63dc30; continue; }
            pc = 0x63db9c; continue;
            }
            0x63db9c => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63dbee; continue; }
            pc = 0x63db9f; continue;
            }
            0x63db9f => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63dbcb; continue; }
            pc = 0x63dba2; continue;
            }
            0x63dba2 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x63dc75; continue; }
            pc = 0x63dba9; continue;
            }
            0x63dba9 => {
            c.push(m, 0x63dbae);
            { let args = []; c.r[0] = host.call(m, 0x644c81, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x20),32); c.sub(a,c.get(0,32),32); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.set(0,8,u32::from(c.cond(5)));
            { let v = rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6cb18c),32); c.push(m, v); }
            pc = 0x63dc6d; continue;
            }
            0x63dbcb => {
            c.set(0,32,rm(m,0x6f1a98u32,32));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x14),32));
            c.push(m,0x2u32);
            c.push(m,0x64u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(1,32,v); }
            { let b = c.get(1,32); c.idiv32(b); }
            pc = 0x63d8f4; continue;
            }
            0x63dbee => {
            { let a = rm(m,0x6f1a98u32,32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x63dc1b; continue; }
            pc = 0x63dbf7; continue;
            }
            0x63dbf7 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = rm(m,0x6f1a98u32,32); let r = c.and(a,0x0u32,32); { let a = 0x6f1a98u32; wm(m,a,32,r); } }
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[0].wrapping_add(0xa4),32); c.push(m, v); }
            pc = 0x63dc11; continue;
            }
            0x63dc11 => {
            c.push(m, 0x63dc16);
            { let args = []; c.r[0] = host.call(m, 0x63dd59, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x63dc75; continue;
            }
            0x63dc1b => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[0].wrapping_add(0xa0),32); c.push(m, v); }
            pc = 0x63dc11; continue;
            }
            0x63dc30 => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.set(0,32,rm(m,0x6f1a98u32,32));
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,0x1u32);
            { let v = rm(m,c.r[0].wrapping_add(0x18),32); c.push(m, v); }
            pc = 0x63d9c7; continue;
            }
            0x63dc4d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let a = rm(m,c.r[0].wrapping_add(0x8),32); c.sub(a,0xbu32,32); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            if c.cond(15) { pc = 0x63dc67; continue; }
            pc = 0x63dc5f; continue;
            }
            0x63dc5f => {
            { let v = rm(m,c.r[0].wrapping_add(0x98),32); c.push(m, v); }
            pc = 0x63dc6d; continue;
            }
            0x63dc67 => {
            { let v = rm(m,c.r[0].wrapping_add(0x9c),32); c.push(m, v); }
            pc = 0x63dc6d; continue;
            }
            0x63dc6d => {
            c.push(m, 0x63dc72);
            { let args = []; c.r[0] = host.call(m, 0x63dc7a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x63dc75; continue;
            }
            0x63dc75 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00643655(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x643655;
    loop {
        match pc {
            0x643655 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x643661);
            { let args = []; c.r[0] = host.call(m, 0x646b18, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x6436a3; continue; }
            pc = 0x643667; continue;
            }
            0x643667 => {
            { let a = c.get(6,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x643671; continue; }
            pc = 0x64366c; continue;
            }
            0x64366c => {
            { let a = c.get(6,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x643687; continue; }
            pc = 0x643671; continue;
            }
            0x643671 => {
            c.push(m,0x2u32);
            c.push(m, 0x643678);
            { let args = []; c.r[0] = host.call(m, 0x646b18, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.set(7,32,c.get(0,32));
            c.push(m, 0x643681);
            { let args = []; c.r[0] = host.call(m, 0x646b18, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x6436a3; continue; }
            pc = 0x643687; continue;
            }
            0x643687 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64368d);
            { let args = []; c.r[0] = host.call(m, 0x646b18, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a150u32,32); c.push(m, 0x643695); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x6436a3; continue; }
            pc = 0x643699; continue;
            }
            0x643699 => {
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x64369f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            pc = 0x6436a5; continue;
            }
            0x6436a3 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x6436a5; continue;
            }
            0x6436a5 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x6436ab);
            { let args = []; c.r[0] = host.call(m, 0x646a99, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(6,32));
            { let a = c.get(6,32); let r = c.and(a,0x1fu32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(8)));
            { let a = rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4),8); let r = c.and(a,0x0u32,8); { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,8,r); } }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x6436d3; continue; }
            pc = 0x6436c7; continue;
            }
            0x6436c7 => {
            c.push(m,c.get(7,32));
            c.push(m, 0x6436cd);
            { let args = []; c.r[0] = host.call(m, 0x63ccb5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x6436d5; continue;
            }
            0x6436d3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6436d5; continue;
            }
            0x6436d5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006436d8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6436d8;
    loop {
        match pc {
            0x6436d8 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,8); c.and(a,0x83u32,8); }
            if c.cond(4) { pc = 0x643701; continue; }
            pc = 0x6436e4; continue;
            }
            0x6436e4 => {
            { let a = c.get(0,8); c.and(a,0x8u32,8); }
            if c.cond(4) { pc = 0x643701; continue; }
            pc = 0x6436e8; continue;
            }
            0x6436e8 => {
            { let v = rm(m,c.r[6].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x6436f0);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0xc),16); let r = c.and(a,0xfbf7u32,16); { let a = c.r[6].wrapping_add(0xc); wm(m,a,16,r); } }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x643701; continue;
            }
            0x643701 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00643760(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x643760;
    loop {
        match pc {
            0x643760 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            { let a = c.get(1,8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x6437ad; continue; }
            pc = 0x643776; continue;
            }
            0x643776 => {
            { let a = c.get(0,16); c.and(a,0x108u32,16); }
            if c.cond(4) { pc = 0x6437ad; continue; }
            pc = 0x64377c; continue;
            }
            0x64377c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[6],32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(14) { pc = 0x6437ad; continue; }
            pc = 0x643787; continue;
            }
            0x643787 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[6].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m, 0x643791);
            { let args = []; c.r[0] = host.call(m, 0x645e81, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x6437a6; continue; }
            pc = 0x643798; continue;
            }
            0x643798 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x6437ad; continue; }
            pc = 0x64379f; continue;
            }
            0x64379f => {
            { let a = c.get(0,8); let r = c.and(a,0xfdu32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x6437ad; continue;
            }
            0x6437a6 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.or(a,0x20u32,32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            pc = 0x6437ad; continue;
            }
            0x6437ad => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
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

pub fn f_00644adb(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x644adb;
    loop {
        match pc {
            0x644adb => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x108u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x644b40; continue; }
            pc = 0x644aec; continue;
            }
            0x644aec => {
            c.push(m,c.get(3,32));
            c.push(m, 0x644af2);
            { let args = []; c.r[0] = host.call(m, 0x644bb0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x644b12; continue; }
            pc = 0x644af7; continue;
            }
            0x644af7 => {
            c.push(m, 0x644afc);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0xfu32); }
            c.push(m, 0x644b07);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0xdu32); }
            pc = 0x644b0d; continue;
            }
            0x644b0d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x644b0f; continue;
            }
            0x644b0f => {
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x644b12 => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffffc));
            { let a = rm(m,c.r[5].wrapping_add(0xb),8); let r = c.and(a,0x0u32,8); { let a = c.r[5].wrapping_add(0xb); wm(m,a,8,r); } }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffef8));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0x8));
            { let a = c.get(3,8); let r = c.add(a,0x40u32,8); c.set(3,8,r); }
            c.push(m,0x104u32);
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[5].wrapping_add(0x9); wm(m,a,8,0x3au32); }
            { let a = c.r[5].wrapping_add(0xa); wm(m,a,8,0x2eu32); }
            { let target = rm(m,0x66a170u32,32); c.push(m, 0x644b3e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x644b52; continue;
            }
            0x644b40 => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffef8));
            c.push(m,c.get(0,32));
            c.push(m,0x104u32);
            { let target = rm(m,0x66a1f4u32,32); c.push(m, 0x644b52); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x644b52; continue;
            }
            0x644b52 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x644b0d; continue; }
            pc = 0x644b56; continue;
            }
            0x644b56 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x104u32,32); }
            if c.cond(7) { pc = 0x644b0d; continue; }
            pc = 0x644b5e; continue;
            }
            0x644b5e => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x644b87; continue; }
            pc = 0x644b65; continue;
            }
            0x644b65 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x10),32),32); }
            if c.cond(15) { pc = 0x644b6d; continue; }
            pc = 0x644b6a; continue;
            }
            0x644b6a => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            pc = 0x644b6d; continue;
            }
            0x644b6d => {
            c.push(m,c.get(0,32));
            c.push(m, 0x644b73);
            { let args = []; c.r[0] = host.call(m, 0x63cb41, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x644b9c; continue; }
            pc = 0x644b7a; continue;
            }
            0x644b7a => {
            c.push(m, 0x644b7f);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0xcu32); }
            pc = 0x644b0d; continue;
            }
            0x644b87 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x10),32),32); }
            if c.cond(14) { pc = 0x644b9c; continue; }
            pc = 0x644b8c; continue;
            }
            0x644b8c => {
            c.push(m, 0x644b91);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x22u32); }
            pc = 0x644b0d; continue;
            }
            0x644b9c => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffef8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x644ba9);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x644b0f; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00644f57(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x644f57;
    loop {
        match pc {
            0x644f57 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,0x6cb104u32,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x644f6b; continue; }
            pc = 0x644f64; continue;
            }
            0x644f64 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6450b7; continue;
            }
            0x644f6b => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.sub(a,rm(m,0x6cb198u32,32),32); }
            if c.cond(5) { pc = 0x644f89; continue; }
            pc = 0x644f7d; continue;
            }
            0x644f7d => {
            { let a = c.get(0,32); c.sub(a,rm(m,0x6cb1a8u32,32),32); }
            if c.cond(4) { pc = 0x64508b; continue; }
            pc = 0x644f89; continue;
            }
            0x644f89 => {
            { let a = rm(m,0x6f0350u32,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x645061; continue; }
            pc = 0x644f95; continue;
            }
            0x644f95 => {
            c.set(1,32,rm(m,0x6f03feu32,16));
            c.push(m,c.get(1,32));
            { let a = rm(m,0x6f03f0u32,16); c.sub(a,c.get(7,16),16); }
            c.set(1,32,rm(m,0x6f03fcu32,16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6f03fau32,16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6f03f8u32,16));
            c.push(m,c.get(1,32));
            if c.cond(5) { pc = 0x644fdb; continue; }
            pc = 0x644fbe; continue;
            }
            0x644fbe => {
            c.set(1,32,rm(m,0x6f03f4u32,16));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6f03f6u32,16));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6f03f2u32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            pc = 0x644fef; continue;
            }
            0x644fdb => {
            c.set(1,32,rm(m,0x6f03f6u32,16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,0x6f03f2u32,16));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            pc = 0x644fef; continue;
            }
            0x644fef => {
            c.push(m,c.get(3,32));
            c.push(m, 0x644ff5);
            { let args = []; c.r[0] = host.call(m, 0x645103, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6f03aau32,16));
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            { let a = rm(m,0x6f039cu32,16); c.sub(a,c.get(7,16),16); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6f03a8u32,16));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6f03a6u32,16));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6f03a4u32,16));
            c.push(m,c.get(0,32));
            if c.cond(5) { pc = 0x645049; continue; }
            pc = 0x645021; continue;
            }
            0x645021 => {
            c.set(0,32,rm(m,0x6f03a0u32,16));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6f03a2u32,16));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,0x6f039eu32,16));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[6].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m,c.get(3,32));
            pc = 0x64503e; continue;
            }
            0x64503e => {
            c.push(m,c.get(7,32));
            c.push(m, 0x645044);
            { let args = []; c.r[0] = host.call(m, 0x645103, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            pc = 0x64508b; continue;
            }
            0x645049 => {
            c.set(0,32,rm(m,0x6f03a2u32,16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,0x6f039eu32,16));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[6].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m,c.get(7,32));
            pc = 0x64503e; continue;
            }
            0x645061 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(7,32));
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x645073);
            { let args = []; c.r[0] = host.call(m, 0x645103, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(7,32));
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(7,32));
            c.push(m,0x5u32);
            c.push(m,0xau32);
            { let v = rm(m,c.r[6].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x645088);
            { let args = []; c.r[0] = host.call(m, 0x645103, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x58u32,32); c.set(4,32,r); }
            pc = 0x64508b; continue;
            }
            0x64508b => {
            c.set(2,32,rm(m,0x6cb19cu32,32));
            c.set(0,32,rm(m,0x6cb1acu32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x6450bb; continue; }
            pc = 0x64509d; continue;
            }
            0x64509d => {
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x644f64; continue; }
            pc = 0x6450a5; continue;
            }
            0x6450a5 => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x644f64; continue; }
            pc = 0x6450ad; continue;
            }
            0x6450ad => {
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x6450cf; continue; }
            pc = 0x6450b1; continue;
            }
            0x6450b1 => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x6450cf; continue; }
            pc = 0x6450b5; continue;
            }
            0x6450b5 => {
            c.set(0,32,c.get(3,32));
            pc = 0x6450b7; continue;
            }
            0x6450b7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x6450bb => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x6450b5; continue; }
            pc = 0x6450bf; continue;
            }
            0x6450bf => {
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(15) { pc = 0x6450b5; continue; }
            pc = 0x6450c3; continue;
            }
            0x6450c3 => {
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x6450cf; continue; }
            pc = 0x6450c7; continue;
            }
            0x6450c7 => {
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x644f64; continue; }
            pc = 0x6450cf; continue;
            }
            0x6450cf => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.imul(a,0x3cu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x4),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,0x3cu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[6],32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,0x3e8u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x6450f6; continue; }
            pc = 0x6450e7; continue;
            }
            0x6450e7 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,rm(m,0x6cb1a0u32,32),32); }
            c.set(1,8,u32::from(c.cond(13)));
            pc = 0x6450f2; continue;
            }
            0x6450f2 => {
            c.set(0,32,c.get(1,32));
            pc = 0x6450b7; continue;
            }
            0x6450f6 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,rm(m,0x6cb1b0u32,32),32); }
            c.set(1,8,u32::from(c.cond(12)));
            pc = 0x6450f2; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006458b3(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6458b3;
    loop {
        match pc {
            0x6458b3 => {
            c.push(m,0x6cb220u32);
            { let v = rm(m,c.r[4].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0xc),32); c.push(m, v); }
            c.push(m, 0x6458c5);
            { let args = []; c.r[0] = host.call(m, 0x645747, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00646285(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x646285;
    loop {
        match pc {
            0x646285 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x5cu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffa4));
            c.push(m,0x1u32);
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(2,32,v); }
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            pc = 0x6462c0; continue;
            }
            0x6462c0 => {
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(1,8); c.sub(a,0x20u32,8); }
            if c.cond(4) { pc = 0x6462d6; continue; }
            pc = 0x6462c7; continue;
            }
            0x6462c7 => {
            { let a = c.get(1,8); c.sub(a,0x9u32,8); }
            if c.cond(4) { pc = 0x6462d6; continue; }
            pc = 0x6462cc; continue;
            }
            0x6462cc => {
            { let a = c.get(1,8); c.sub(a,0xau32,8); }
            if c.cond(4) { pc = 0x6462d6; continue; }
            pc = 0x6462d1; continue;
            }
            0x6462d1 => {
            { let a = c.get(1,8); c.sub(a,0xdu32,8); }
            if c.cond(5) { pc = 0x6462d9; continue; }
            pc = 0x6462d6; continue;
            }
            0x6462d6 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x6462c0; continue;
            }
            0x6462d9 => {
            c.push(m,0x4u32);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x6462dc; continue;
            }
            0x6462dc => {
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,0xbu32,32); }
            if c.cond(7) { pc = 0x64655f; continue; }
            pc = 0x6462e8; continue;
            }
            0x6462e8 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x646726)); continue;
            }
            0x6462ef => {
            { let a = c.get(3,8); c.sub(a,0x31u32,8); }
            if c.cond(12) { pc = 0x646300; continue; }
            pc = 0x6462f4; continue;
            }
            0x6462f4 => {
            { let a = c.get(3,8); c.sub(a,0x39u32,8); }
            if c.cond(15) { pc = 0x646300; continue; }
            pc = 0x6462f9; continue;
            }
            0x6462f9 => {
            c.push(m,0x3u32);
            pc = 0x64651d; continue;
            }
            0x646300 => {
            { let a = c.get(3,8); c.sub(a,rm(m,0x6c8970u32,8),8); }
            if c.cond(5) { pc = 0x64630f; continue; }
            pc = 0x646308; continue;
            }
            0x646308 => {
            c.push(m,0x5u32);
            pc = 0x646555; continue;
            }
            0x64630f => {
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,0x2bu32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x646335; continue; }
            pc = 0x646317; continue;
            }
            0x646317 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x646329; continue; }
            pc = 0x64631b; continue;
            }
            0x64631b => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x6465f8; continue; }
            pc = 0x646324; continue;
            }
            0x646324 => {
            pc = 0x6463b8; continue;
            }
            0x646329 => {
            c.push(m,0x2u32);
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,0x8000u32); }
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x6462dc; continue;
            }
            0x646335 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffd8),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,r); } }
            c.push(m,0x2u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x6462dc; continue;
            }
            0x64633e => {
            { let a = c.get(3,8); c.sub(a,0x31u32,8); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x64634b; continue; }
            pc = 0x646346; continue;
            }
            0x646346 => {
            { let a = c.get(3,8); c.sub(a,0x39u32,8); }
            if c.cond(14) { pc = 0x6462f9; continue; }
            pc = 0x64634b; continue;
            }
            0x64634b => {
            { let a = c.get(3,8); c.sub(a,rm(m,0x6c8970u32,8),8); }
            if c.cond(4) { pc = 0x646413; continue; }
            pc = 0x646357; continue;
            }
            0x646357 => {
            { let a = c.get(3,8); c.sub(a,0x2bu32,8); }
            if c.cond(4) { pc = 0x64638d; continue; }
            pc = 0x64635c; continue;
            }
            0x64635c => {
            { let a = c.get(3,8); c.sub(a,0x2du32,8); }
            if c.cond(4) { pc = 0x64638d; continue; }
            pc = 0x646361; continue;
            }
            0x646361 => {
            { let a = c.get(3,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x6463b8; continue; }
            pc = 0x646366; continue;
            }
            0x646366 => {
            { let a = c.get(3,8); c.sub(a,0x43u32,8); }
            if c.cond(14) { pc = 0x6465f8; continue; }
            pc = 0x64636f; continue;
            }
            0x64636f => {
            { let a = c.get(3,8); c.sub(a,0x45u32,8); }
            if c.cond(14) { pc = 0x646386; continue; }
            pc = 0x646374; continue;
            }
            0x646374 => {
            { let a = c.get(3,8); c.sub(a,0x63u32,8); }
            if c.cond(14) { pc = 0x6465f8; continue; }
            pc = 0x64637d; continue;
            }
            0x64637d => {
            { let a = c.get(3,8); c.sub(a,0x65u32,8); }
            if c.cond(15) { pc = 0x6465f8; continue; }
            pc = 0x646386; continue;
            }
            0x646386 => {
            c.push(m,0x6u32);
            pc = 0x646555; continue;
            }
            0x64638d => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            c.push(m,0xbu32);
            pc = 0x646555; continue;
            }
            0x646395 => {
            { let a = c.get(3,8); c.sub(a,0x31u32,8); }
            if c.cond(12) { pc = 0x6463a3; continue; }
            pc = 0x64639a; continue;
            }
            0x64639a => {
            { let a = c.get(3,8); c.sub(a,0x39u32,8); }
            if c.cond(14) { pc = 0x6462f9; continue; }
            pc = 0x6463a3; continue;
            }
            0x6463a3 => {
            { let a = c.get(3,8); c.sub(a,rm(m,0x6c8970u32,8),8); }
            if c.cond(4) { pc = 0x646308; continue; }
            pc = 0x6463af; continue;
            }
            0x6463af => {
            { let a = c.get(3,8); c.sub(a,0x30u32,8); }
            if c.cond(5) { pc = 0x64656d; continue; }
            pc = 0x6463b8; continue;
            }
            0x6463b8 => {
            c.set(0,32,c.get(2,32));
            pc = 0x6462dc; continue;
            }
            0x6463bf => {
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(2,32)); }
            pc = 0x6463c2; continue;
            }
            0x6463c2 => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x6463db; continue; }
            pc = 0x6463ca; continue;
            }
            0x6463ca => {
            c.set(0,32,c.get(3,8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6463d4);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(2,32,v); }
            pc = 0x6463e9; continue;
            }
            0x6463db => {
            c.set(1,32,rm(m,0x6c8760u32,32));
            c.set(0,32,c.get(3,8));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,c.get(6,32),32); c.set(0,32,r); }
            pc = 0x6463e9; continue;
            }
            0x6463e9 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64640b; continue; }
            pc = 0x6463ed; continue;
            }
            0x6463ed => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x19u32,32); }
            if c.cond(3) { pc = 0x646403; continue; }
            pc = 0x6463f3; continue;
            }
            0x6463f3 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(3,8); let r = c.sub(a,0x30u32,8); c.set(3,8,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff4),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,r); } }
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            pc = 0x646406; continue;
            }
            0x646403 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            pc = 0x646406; continue;
            }
            0x646406 => {
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x6463c2; continue;
            }
            0x64640b => {
            { let a = c.get(3,8); c.sub(a,rm(m,0x6c8970u32,8),8); }
            if c.cond(5) { pc = 0x64647a; continue; }
            pc = 0x646413; continue;
            }
            0x646413 => {
            c.set(0,32,c.get(6,32));
            pc = 0x6462dc; continue;
            }
            0x64641a => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x0u32,32); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x646433; continue; }
            pc = 0x646426; continue;
            }
            0x646426 => {
            { let a = c.get(3,8); c.sub(a,0x30u32,8); }
            if c.cond(5) { pc = 0x646433; continue; }
            pc = 0x64642b; continue;
            }
            0x64642b => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x646426; continue;
            }
            0x646433 => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x64644c; continue; }
            pc = 0x64643b; continue;
            }
            0x64643b => {
            c.set(0,32,c.get(3,8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x646445);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(2,32,v); }
            pc = 0x64645a; continue;
            }
            0x64644c => {
            c.set(1,32,rm(m,0x6c8760u32,32));
            c.set(0,32,c.get(3,8));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,c.get(6,32),32); c.set(0,32,r); }
            pc = 0x64645a; continue;
            }
            0x64645a => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64647a; continue; }
            pc = 0x64645e; continue;
            }
            0x64645e => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x19u32,32); }
            if c.cond(3) { pc = 0x646475; continue; }
            pc = 0x646464; continue;
            }
            0x646464 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(3,8); let r = c.sub(a,0x30u32,8); c.set(3,8,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff4),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            pc = 0x646475; continue;
            }
            0x646475 => {
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x646433; continue;
            }
            0x64647a => {
            { let a = c.get(3,8); c.sub(a,0x2bu32,8); }
            if c.cond(4) { pc = 0x64638d; continue; }
            pc = 0x646483; continue;
            }
            0x646483 => {
            { let a = c.get(3,8); c.sub(a,0x2du32,8); }
            if c.cond(4) { pc = 0x64638d; continue; }
            pc = 0x64648c; continue;
            }
            0x64648c => {
            pc = 0x646366; continue;
            }
            0x646491 => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x6464ad; continue; }
            pc = 0x64649c; continue;
            }
            0x64649c => {
            c.set(0,32,c.get(3,8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x6464a6);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(2,32,v); }
            pc = 0x6464bb; continue;
            }
            0x6464ad => {
            c.set(1,32,rm(m,0x6c8760u32,32));
            c.set(0,32,c.get(3,8));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,c.get(6,32),32); c.set(0,32,r); }
            pc = 0x6464bb; continue;
            }
            0x6464bb => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64656d; continue; }
            pc = 0x6464c3; continue;
            }
            0x6464c3 => {
            c.set(0,32,c.get(6,32));
            pc = 0x64651e; continue;
            }
            0x6464c7 => {
            c.set(1,32,c.r[7].wrapping_add(0xfffffffe));
            { let a = c.get(3,8); c.sub(a,0x31u32,8); }
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x6464d7; continue; }
            pc = 0x6464d2; continue;
            }
            0x6464d2 => {
            { let a = c.get(3,8); c.sub(a,0x39u32,8); }
            if c.cond(14) { pc = 0x64651b; continue; }
            pc = 0x6464d7; continue;
            }
            0x6464d7 => {
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,0x2bu32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x646553; continue; }
            pc = 0x6464df; continue;
            }
            0x6464df => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x646547; continue; }
            pc = 0x6464e3; continue;
            }
            0x6464e3 => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x6465fb; continue; }
            pc = 0x6464ec; continue;
            }
            0x6464ec => {
            c.push(m,0x8u32);
            pc = 0x646555; continue;
            }
            0x6464f0 => {
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(2,32)); }
            pc = 0x6464f3; continue;
            }
            0x6464f3 => {
            { let a = c.get(3,8); c.sub(a,0x30u32,8); }
            if c.cond(5) { pc = 0x6464fd; continue; }
            pc = 0x6464f8; continue;
            }
            0x6464f8 => {
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x6464f3; continue;
            }
            0x6464fd => {
            { let a = c.get(3,8); c.sub(a,0x31u32,8); }
            if c.cond(12) { pc = 0x6465f8; continue; }
            pc = 0x646506; continue;
            }
            0x646506 => {
            { let a = c.get(3,8); c.sub(a,0x39u32,8); }
            if c.cond(15) { pc = 0x6465f8; continue; }
            pc = 0x64650f; continue;
            }
            0x64650f => {
            pc = 0x64651b; continue;
            }
            0x646511 => {
            { let a = c.get(3,8); c.sub(a,0x31u32,8); }
            if c.cond(12) { pc = 0x646524; continue; }
            pc = 0x646516; continue;
            }
            0x646516 => {
            { let a = c.get(3,8); c.sub(a,0x39u32,8); }
            if c.cond(15) { pc = 0x646524; continue; }
            pc = 0x64651b; continue;
            }
            0x64651b => {
            c.push(m,0x9u32);
            pc = 0x64651d; continue;
            }
            0x64651d => {
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x64651e; continue;
            }
            0x64651e => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            pc = 0x6462dc; continue;
            }
            0x646524 => {
            { let a = c.get(3,8); c.sub(a,0x30u32,8); }
            if c.cond(5) { pc = 0x64656d; continue; }
            pc = 0x646529; continue;
            }
            0x646529 => {
            pc = 0x6464ec; continue;
            }
            0x64652b => {
            { let a = rm(m,c.r[5].wrapping_add(0x20),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x64655b; continue; }
            pc = 0x646531; continue;
            }
            0x646531 => {
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            c.set(1,32,c.r[7].wrapping_add(0xffffffff));
            { let a = c.get(0,32); let r = c.sub(a,0x2bu32,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x646553; continue; }
            pc = 0x64653f; continue;
            }
            0x64653f => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x6465fb; continue; }
            pc = 0x646547; continue;
            }
            0x646547 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe8),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,r); } }
            c.push(m,0x7u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x6462dc; continue;
            }
            0x646553 => {
            c.push(m,0x7u32);
            pc = 0x646555; continue;
            }
            0x646555 => {
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x6462dc; continue;
            }
            0x64655b => {
            c.push(m,0xau32);
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x64655f; continue;
            }
            0x64655f => {
            { let a = c.get(0,32); c.sub(a,0xau32,32); }
            if c.cond(4) { pc = 0x6465fd; continue; }
            pc = 0x646568; continue;
            }
            0x646568 => {
            pc = 0x6462dc; continue;
            }
            0x64656d => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x10),32));
            pc = 0x6465fd; continue;
            }
            0x646575 => {
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,0x1u32); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x64657e; continue;
            }
            0x64657e => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x646596; continue; }
            pc = 0x646587; continue;
            }
            0x646587 => {
            c.set(0,32,c.get(3,8));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x646592);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x6465a5; continue;
            }
            0x646596 => {
            c.set(1,32,rm(m,0x6c8760u32,32));
            c.set(0,32,c.get(3,8));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,0x4u32,32); c.set(0,32,r); }
            pc = 0x6465a5; continue;
            }
            0x6465a5 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6465c5; continue; }
            pc = 0x6465a9; continue;
            }
            0x6465a9 => {
            c.set(1,32,sx(c.get(3,8),8)&0xffffffffu32);
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(6,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0xffffffd0));
            { let a = c.get(6,32); c.sub(a,0x1450u32,32); }
            if c.cond(15) { pc = 0x6465c0; continue; }
            pc = 0x6465bb; continue;
            }
            0x6465bb => {
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x64657e; continue;
            }
            0x6465c0 => {
            c.set(6,32,0x1451u32);
            pc = 0x6465c5; continue;
            }
            0x6465c5 => {
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(6,32)); }
            pc = 0x6465c8; continue;
            }
            0x6465c8 => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x6465e0; continue; }
            pc = 0x6465d1; continue;
            }
            0x6465d1 => {
            c.set(0,32,c.get(3,8));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x6465dc);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x6465ef; continue;
            }
            0x6465e0 => {
            c.set(1,32,rm(m,0x6c8760u32,32));
            c.set(0,32,c.get(3,8));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,0x4u32,32); c.set(0,32,r); }
            pc = 0x6465ef; continue;
            }
            0x6465ef => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6465f8; continue; }
            pc = 0x6465f3; continue;
            }
            0x6465f3 => {
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x6465c8; continue;
            }
            0x6465f8 => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            pc = 0x6465fd; continue;
            }
            0x6465fb => {
            c.set(7,32,c.get(1,32));
            pc = 0x6465fd; continue;
            }
            0x6465fd => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff0),32); c.sub(a,0x0u32,32); }
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x6466e5; continue; }
            pc = 0x64660c; continue;
            }
            0x64660c => {
            c.push(m,0x18u32);
            { let v = c.pop(m); c.set(0,32,v); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x646629; continue; }
            pc = 0x646614; continue;
            }
            0x646614 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffbb),8); c.sub(a,0x5u32,8); }
            if c.cond(12) { pc = 0x64661d; continue; }
            pc = 0x64661a; continue;
            }
            0x64661a => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffbb),8); let r = c.inc(a,8); { let a = c.r[5].wrapping_add(0xffffffbb); wm(m,a,8,r); } }
            pc = 0x64661d; continue;
            }
            0x64661d => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            pc = 0x64662c; continue;
            }
            0x646629 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            pc = 0x64662c; continue;
            }
            0x64662c => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,0x0u32,32); }
            if c.cond(6) { pc = 0x6466db; continue; }
            pc = 0x646636; continue;
            }
            0x646636 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x646644; continue; }
            pc = 0x64663c; continue;
            }
            0x64663c => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            pc = 0x646636; continue;
            }
            0x646644 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffc0));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffa4));
            { let v = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x646654);
            { let args = []; c.r[0] = host.call(m, 0x64858c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe8),32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x646663; continue; }
            pc = 0x646661; continue;
            }
            0x646661 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x646663; continue;
            }
            0x646663 => {
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0xfffffff8),32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe0),32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x64666e; continue; }
            pc = 0x64666b; continue;
            }
            0x64666b => {
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x18),32),32); c.set(0,32,r); }
            pc = 0x64666e; continue;
            }
            0x64666e => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffdc),32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x646676; continue; }
            pc = 0x646673; continue;
            }
            0x646673 => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0x1c),32),32); c.set(0,32,r); }
            pc = 0x646676; continue;
            }
            0x646676 => {
            { let a = c.get(0,32); c.sub(a,0x1450u32,32); }
            if c.cond(14) { pc = 0x6466ad; continue; }
            pc = 0x64667d; continue;
            }
            0x64667d => {
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,0x1u32); }
            pc = 0x646684; continue;
            }
            0x646684 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x10),32));
            pc = 0x646690; continue;
            }
            0x646690 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffd4),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x6466f6; continue; }
            pc = 0x646696; continue;
            }
            0x646696 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,0x7fffu32);
            c.set(6,32,0x80000000u32);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,0x2u32); }
            pc = 0x64670b; continue;
            }
            0x6466ad => {
            { let a = c.get(0,32); c.sub(a,0xffffebb0u32,32); }
            if c.cond(13) { pc = 0x6466bd; continue; }
            pc = 0x6466b4; continue;
            }
            0x6466b4 => {
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,0x1u32); }
            pc = 0x646684; continue;
            }
            0x6466bd => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffc0));
            c.push(m,c.get(0,32));
            c.push(m, 0x6466ca);
            { let args = []; c.r[0] = host.call(m, 0x648b06, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffc0),32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xffffffc2),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffc6),32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffca),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x646690; continue;
            }
            0x6466db => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x646690; continue;
            }
            0x6466e5 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,0x4u32); }
            pc = 0x64670b; continue;
            }
            0x6466f6 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffd0),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x64670b; continue; }
            pc = 0x6466fc; continue;
            }
            0x6466fc => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,0x1u32); }
            pc = 0x64670b; continue;
            }
            0x64670b => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.or(a,rm(m,c.r[5].wrapping_add(0xffffffd8),32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[1].wrapping_add(0x6); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[1].wrapping_add(0x2); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[1].wrapping_add(0xa); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00646c6e(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x646c6e;
    loop {
        match pc {
            0x646c6e => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6f0a40u32,32),32); }
            if c.cond(3) { pc = 0x646cbb; continue; }
            pc = 0x646c7b; continue;
            }
            0x646c7b => {
            c.set(1,32,c.get(6,32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x646cbb; continue; }
            pc = 0x646c96; continue;
            }
            0x646c96 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x646c9d);
            { let args = []; c.r[0] = host.call(m, 0x646b5a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            c.push(m,c.get(6,32));
            c.push(m, 0x646cab);
            { let args = []; c.r[0] = host.call(m, 0x646cd3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x646cb3);
            { let args = []; c.r[0] = host.call(m, 0x646bb9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x646cbb => {
            c.push(m, 0x646cc0);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x9u32); }
            c.push(m, 0x646ccb);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0],32); let r = c.and(a,0x0u32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00647180(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x647180;
    loop {
        match pc {
            0x647180 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x647203; continue; }
            pc = 0x647189; continue;
            }
            0x647189 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); c.and(a,0x3u32,32); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            if c.cond(5) { pc = 0x6471a4; continue; }
            pc = 0x64719d; continue;
            }
            0x64719d => {
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x647211; continue; }
            pc = 0x6471a2; continue;
            }
            0x6471a2 => {
            pc = 0x6471c5; continue;
            }
            0x6471a4 => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x6471d2; continue; }
            pc = 0x6471ad; continue;
            }
            0x6471ad => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x6471da; continue; }
            pc = 0x6471b1; continue;
            }
            0x6471b1 => {
            { let a = c.get(6,32); c.and(a,0x3u32,32); }
            if c.cond(5) { pc = 0x6471a4; continue; }
            pc = 0x6471b9; continue;
            }
            0x6471b9 => {
            c.set(3,32,c.get(1,32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x647211; continue; }
            pc = 0x6471c0; continue;
            }
            0x6471c0 => {
            { let a = c.get(3,32); let r = c.and(a,0x3u32,32); c.set(3,32,r); }
            if c.cond(4) { pc = 0x6471d2; continue; }
            pc = 0x6471c5; continue;
            }
            0x6471c5 => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x6471fe; continue; }
            pc = 0x6471cf; continue;
            }
            0x6471cf => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x6471c5; continue; }
            pc = 0x6471d2; continue;
            }
            0x6471d2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x6471da => {
            { let a = c.get(7,32); c.and(a,0x3u32,32); }
            if c.cond(4) { pc = 0x6471f4; continue; }
            pc = 0x6471e2; continue;
            }
            0x6471e2 => {
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x647276; continue; }
            pc = 0x6471ec; continue;
            }
            0x6471ec => {
            { let a = c.get(7,32); c.and(a,0x3u32,32); }
            if c.cond(5) { pc = 0x6471e2; continue; }
            pc = 0x6471f4; continue;
            }
            0x6471f4 => {
            c.set(3,32,c.get(1,32));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x647267; continue; }
            pc = 0x6471fb; continue;
            }
            0x6471fb => {
            { let a = c.r[7]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x6471fe; continue;
            }
            0x6471fe => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x6471fb; continue; }
            pc = 0x647201; continue;
            }
            0x647201 => {
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x647203; continue;
            }
            0x647203 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x647209 => {
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x6471c0; continue; }
            pc = 0x647211; continue;
            }
            0x647211 => {
            c.set(2,32,0x7efefeffu32);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,0x81010100u32,32); }
            if c.cond(4) { pc = 0x647209; continue; }
            pc = 0x64722b; continue;
            }
            0x64722b => {
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x64725b; continue; }
            pc = 0x64722f; continue;
            }
            0x64722f => {
            { let a = c.get(6,8); c.and(a,c.get(6,8),8); }
            if c.cond(4) { pc = 0x647251; continue; }
            pc = 0x647233; continue;
            }
            0x647233 => {
            { let a = c.get(2,32); c.and(a,0xff0000u32,32); }
            if c.cond(4) { pc = 0x647247; continue; }
            pc = 0x64723b; continue;
            }
            0x64723b => {
            { let a = c.get(2,32); c.and(a,0xff000000u32,32); }
            if c.cond(5) { pc = 0x647209; continue; }
            pc = 0x647243; continue;
            }
            0x647243 => {
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            pc = 0x64725f; continue;
            }
            0x647247 => {
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            pc = 0x64725f; continue;
            }
            0x647251 => {
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            pc = 0x64725f; continue;
            }
            0x64725b => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            pc = 0x64725f; continue;
            }
            0x64725f => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x647271; continue; }
            pc = 0x647267; continue;
            }
            0x647267 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x647269; continue;
            }
            0x647269 => {
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x647269; continue; }
            pc = 0x647271; continue;
            }
            0x647271 => {
            { let a = c.get(3,32); let r = c.and(a,0x3u32,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x6471fb; continue; }
            pc = 0x647276; continue;
            }
            0x647276 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00649010(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x649010;
    loop {
        match pc {
            0x649010 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(6,32,0x8000u32);
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            c.set(1,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),8));
            c.set(0,8,c.get(1,8));
            { let a = c.get(0,32); let r = c.and(a,0x80u32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(0xc),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x649046; continue; }
            pc = 0x649041; continue;
            }
            0x649041 => {
            { let a = c.get(1,8); let r = c.and(a,0x7fu32,8); c.set(1,8,r); }
            pc = 0x649053; continue;
            }
            0x649046 => {
            { let a = rm(m,c.r[4].wrapping_add(0xc),32); c.sub(a,0x4000u32,32); }
            if c.cond(5) { pc = 0x649061; continue; }
            pc = 0x649050; continue;
            }
            0x649050 => {
            { let a = c.get(1,8); let r = c.or(a,0x80u32,8); c.set(1,8,r); }
            pc = 0x649053; continue;
            }
            0x649053 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            { let a = c.get(0,16); let r = c.and(a,0xc000u32,16); c.set(0,16,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x649061 => {
            c.push(m, 0x649066);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x16u32); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e4ce(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e4ce;
    loop {
        match pc {
            0x64e4ce => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x1c),32));
            c.push(m,c.get(6,32));
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(6,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.add(a,0xfffff894u32,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x20),32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0xffffffdc));
            c.push(m,c.get(0,32));
            c.push(m, 0x64e510);
            { let args = []; c.r[0] = host.call(m, 0x63cf26, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(32); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ed1d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ed1d;
    loop {
        match pc {
            0x64ed1d => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[0]; wm(m,a,32,0x67d18cu32); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,0x11u32); }
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ed60(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ed60;
    loop {
        match pc {
            0x64ed60 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64ed76; continue; }
            pc = 0x64ed6b; continue;
            }
            0x64ed6b => {
            c.push(m,c.get(0,32));
            c.push(m, 0x64ed71);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x64ed76; continue;
            }
            0x64ed76 => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,0x0u32,32); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc),32));
            if c.cond(4) { pc = 0x64ed9d; continue; }
            pc = 0x64ed81; continue;
            }
            0x64ed81 => {
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(3,32));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            c.push(m,c.get(7,32));
            c.push(m, 0x64ed8d);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x64ed99);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x64ed9d; continue;
            }
            0x64ed9d => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064edf9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64edf9;
    loop {
        match pc {
            0x64edf9 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = rm(m,c.r[6].wrapping_add(0x10),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x64ee31; continue; }
            pc = 0x64ee02; continue;
            }
            0x64ee02 => {
            c.push(m,0xcu32);
            c.set(0,32,c.r[6].wrapping_add(0x14));
            { let v = rm(m,c.r[6].wrapping_add(0x18),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x64ee10);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64ece7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xfffffff4));
            if c.cond(12) { pc = 0x64ee31; continue; }
            pc = 0x64ee22; continue;
            }
            0x64ee22 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x64ee23; continue;
            }
            0x64ee23 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.sub(a,0xcu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x64ee23; continue; }
            pc = 0x64ee31; continue;
            }
            0x64ee31 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.inc(a,32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,r); } }
            { let a = rm(m,c.r[0].wrapping_add(0x8),32); let r = c.and(a,0x0u32,32); { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ee46(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ee46;
    loop {
        match pc {
            0x64ee46 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x10),32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = rm(m,c.r[1].wrapping_add(0xc),32); let r = c.dec(a,32); { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = c.r[1].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x64ee5c; continue; }
            pc = 0x64ee57; continue;
            }
            0x64ee57 => {
            c.push(m, 0x64ee5c);
            { let args = []; c.r[0] = host.call(m, 0x64eda5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x64ee5c; continue;
            }
            0x64ee5c => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ee5f(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ee5f;
    loop {
        match pc {
            0x64ee5f => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let b = rm(m,c.r[1].wrapping_add(0x8),32); c.div32(b); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x64ee8b; continue; }
            pc = 0x64ee7b; continue;
            }
            0x64ee7b => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)),32));
            pc = 0x64ee7e; continue;
            }
            0x64ee7e => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64ee8b; continue; }
            pc = 0x64ee82; continue;
            }
            0x64ee82 => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x64ee8d; continue; }
            pc = 0x64ee87; continue;
            }
            0x64ee87 => {
            c.set(0,32,rm(m,c.r[0],32));
            pc = 0x64ee7e; continue;
            }
            0x64ee8b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64ee8d; continue;
            }
            0x64ee8d => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006505d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6505d0;
    loop {
        match pc {
            0x6505d0 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x6505e0; continue; }
            pc = 0x6505dc; continue;
            }
            0x6505dc => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x650604; continue;
            }
            0x6505e0 => {
            c.push(m,c.get(7,32));
            c.push(m,0x1u32);
            c.push(m, 0x6505e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x650519, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[7].wrapping_add(0x4));
            c.push(m, 0x6505f6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64eec4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            c.push(m, 0x650600);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x653517, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x650604; continue;
            }
            0x650604 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00650666(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x650666;
    loop {
        match pc {
            0x650666 => {
            c.set(0,32,0x650637u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00651c06(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x651c06;
    loop {
        match pc {
            0x651c06 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.set(7,32,rm(m,0x66a3dcu32,32));
            { let target = c.get(7,32); c.push(m, 0x651c16); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x651c16; continue;
            }
            0x651c16 => {
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x651c7d; continue; }
            pc = 0x651c1c; continue;
            }
            0x651c1c => {
            { let a = rm(m,c.r[5].wrapping_add(0x1c),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x651c40; continue; }
            pc = 0x651c22; continue;
            }
            0x651c22 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x651c28);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6505b2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x651c50; continue; }
            pc = 0x651c2c; continue;
            }
            0x651c2c => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[0].wrapping_add(0x1c),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x651c3e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x6503ca, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x651c50; continue;
            }
            0x651c40 => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a4a0u32,32); c.push(m, 0x651c50); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x651c50; continue;
            }
            0x651c50 => {
            { let a = rm(m,c.r[5].wrapping_add(0x18),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x651c72; continue; }
            pc = 0x651c56; continue;
            }
            0x651c56 => {
            c.push(m,c.get(6,32));
            { let target = c.get(7,32); c.push(m, 0x651c59); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x651c72; continue; }
            pc = 0x651c5d; continue;
            }
            0x651c5d => {
            { let v = rm(m,c.r[5].wrapping_add(0x1c),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x18),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            c.push(m,c.get(6,32));
            c.push(m, 0x651c72);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x651c06, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            pc = 0x651c72; continue;
            }
            0x651c72 => {
            c.push(m,0x2u32);
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a36cu32,32); c.push(m, 0x651c7b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x651c16; continue;
            }
            0x651c7d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653310(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653310;
    loop {
        match pc {
            0x653310 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x34),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x65332c; continue; }
            pc = 0x653317; continue;
            }
            0x653317 => {
            { let v = rm(m,c.r[4].wrapping_add(0x4),32); c.push(m, v); }
            { let v = rm(m,c.r[1].wrapping_add(0x1c),32); c.push(m, v); }
            { let target = rm(m,0x66a340u32,32); c.push(m, 0x653324); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            pc = 0x65333b; continue;
            }
            0x65332c => {
            { let v = rm(m,c.r[4].wrapping_add(0x8),32); c.push(m, v); }
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,c.get(0,32));
            { let v = rm(m,c.r[4].wrapping_add(0x8),32); c.push(m, v); }
            { let target = rm(m,c.r[2].wrapping_add(0x6c),32); c.push(m, 0x65333b); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x65333b; continue;
            }
            0x65333b => {
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653573(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653573;
    loop {
        match pc {
            0x653573 => {
            c.set(0,32,0x6689eeu32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x65357d; }
            c.push(m,c.get(6,32));
            c.push(m, 0x653583);
            { let args = []; c.r[0] = host.call(m, 0x6599a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = rm(m,c.r[6].wrapping_add(0x18),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x6535d3; continue; }
            pc = 0x65358b; continue;
            }
            0x65358b => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x6535d3; continue; }
            pc = 0x653591; continue;
            }
            0x653591 => {
            c.push(m,c.get(3,32));
            c.push(m,0x6560d6u32);
            c.push(m, 0x65359c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f7a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x48u32);
            c.set(3,32,c.get(0,32));
            c.push(m, 0x6535a5);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,c.get(0,32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x6535c3; continue; }
            pc = 0x6535b3; continue;
            }
            0x6535b3 => {
            c.push(m,0x1u32);
            c.push(m,0x4u32);
            c.push(m,0x67c958u32);
            c.push(m, 0x6535c1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655300, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x6535c5; continue;
            }
            0x6535c3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6535c5; continue;
            }
            0x6535c5 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x6535d2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f7a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x6535d3; continue;
            }
            0x6535d3 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006557d8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6557d8;
    loop {
        match pc {
            0x6557d8 => {
            c.set(0,32,0x668a5eu32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x6557e2; }
            c.push(m,c.get(6,32));
            c.push(m, 0x6557e8);
            { let args = []; c.r[0] = host.call(m, 0x6599a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = rm(m,c.r[6].wrapping_add(0x1c),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x655838; continue; }
            pc = 0x6557f0; continue;
            }
            0x6557f0 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x655838; continue; }
            pc = 0x6557f6; continue;
            }
            0x6557f6 => {
            c.push(m,c.get(3,32));
            c.push(m,0x6560d6u32);
            c.push(m, 0x655801);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f7a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x48u32);
            c.set(3,32,c.get(0,32));
            c.push(m, 0x65580a);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,c.get(0,32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x655828; continue; }
            pc = 0x655818; continue;
            }
            0x655818 => {
            c.push(m,0x2u32);
            c.push(m,0x4u32);
            c.push(m,0x67ca18u32);
            c.push(m, 0x655826);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x655300, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x65582a; continue;
            }
            0x655828 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x65582a; continue;
            }
            0x65582a => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m,c.get(3,32));
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x655837);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f7a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x655838; continue;
            }
            0x655838 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656716(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656716;
    loop {
        match pc {
            0x656716 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = rm(m,c.r[6],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x656744; continue; }
            pc = 0x65671e; continue;
            }
            0x65671e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x656744; continue; }
            pc = 0x656725; continue;
            }
            0x656725 => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a3fcu32,32); c.push(m, 0x65672c); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x656744; continue; }
            pc = 0x656732; continue;
            }
            0x656732 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x0u32);
            c.push(m,0xb1u32);
            { let v = rm(m,c.r[6].wrapping_add(0x8),32); c.push(m, v); }
            { let target = rm(m,0x66a4a0u32,32); c.push(m, 0x656744); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x656744; continue;
            }
            0x656744 => {
            c.push(m, 0x656749);
            { let args = []; c.r[0] = host.call(m, 0x655ea2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065674b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65674b;
    loop {
        match pc {
            0x65674b => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.sub(a,0x6cu32,8); }
            if c.cond(4) { pc = 0x656765; continue; }
            pc = 0x65675d; continue;
            }
            0x65675d => {
            { let a = c.get(1,8); c.sub(a,0x73u32,8); }
            if c.cond(5) { pc = 0x656766; continue; }
            pc = 0x656762; continue;
            }
            0x656762 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x656765; continue;
            }
            0x656765 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x656766; continue;
            }
            0x656766 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x656769; continue;
            }
            0x656769 => {
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.get(2,8); c.sub(a,0x20u32,8); }
            if c.cond(4) { pc = 0x656775; continue; }
            pc = 0x656770; continue;
            }
            0x656770 => {
            { let a = c.get(2,8); c.sub(a,0x9u32,8); }
            if c.cond(5) { pc = 0x65677b; continue; }
            pc = 0x656775; continue;
            }
            0x656775 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x656769; continue;
            }
            0x65677b => {
            { let a = rm(m,c.r[0],8); c.sub(a,0x64u32,8); }
            c.set(3,8,rm(m,c.r[1],8));
            if c.cond(5) { pc = 0x656790; continue; }
            pc = 0x656782; continue;
            }
            0x656782 => {
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,0xau32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x65678e);
            { let args = []; c.r[0] = host.call(m, 0x639f83, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x6567a1; continue;
            }
            0x656790 => {
            { let a = c.get(3,8); c.sub(a,0x2du32,8); }
            if c.cond(4) { pc = 0x6567d2; continue; }
            pc = 0x656795; continue;
            }
            0x656795 => {
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,0xau32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x6567a1);
            { let args = []; c.r[0] = host.call(m, 0x63a19f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x6567a1; continue;
            }
            0x6567a1 => {
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x6567ad; continue; }
            pc = 0x6567a8; continue;
            }
            0x6567a8 => {
            { let a = c.get(3,8); c.sub(a,0x30u32,8); }
            if c.cond(5) { pc = 0x6567d2; continue; }
            pc = 0x6567ad; continue;
            }
            0x6567ad => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x6567b0; continue;
            }
            0x6567b0 => {
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.get(2,8); c.sub(a,0x20u32,8); }
            if c.cond(4) { pc = 0x6567bc; continue; }
            pc = 0x6567b7; continue;
            }
            0x6567b7 => {
            { let a = c.get(2,8); c.sub(a,0x9u32,8); }
            if c.cond(5) { pc = 0x6567c2; continue; }
            pc = 0x6567bc; continue;
            }
            0x6567bc => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x6567b0; continue;
            }
            0x6567c2 => {
            { let a = rm(m,c.r[1],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x6567d2; continue; }
            pc = 0x6567c7; continue;
            }
            0x6567c7 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x6567e0; continue; }
            pc = 0x6567cb; continue;
            }
            0x6567cb => {
            c.set(1,32,sx(c.get(0,16),16)&0xffffffffu32);
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6567d6; continue; }
            pc = 0x6567d2; continue;
            }
            0x6567d2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x6567ea; continue;
            }
            0x6567d6 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[1],32));
            { let a = c.r[1]; wm(m,a,16,c.get(0,16)); }
            pc = 0x6567e7; continue;
            }
            0x6567e0 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[1],32));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            pc = 0x6567e7; continue;
            }
            0x6567e7 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x6567ea; continue;
            }
            0x6567ea => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656b90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656b90;
    loop {
        match pc {
            0x656b90 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x656b96);
            { let args = []; c.r[0] = host.call(m, 0x6389fa, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x656bba; continue; }
            pc = 0x656b9c; continue;
            }
            0x656b9c => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0xb0),32); c.push(m, 0x656ba6); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x656bba; continue; }
            pc = 0x656baa; continue;
            }
            0x656baa => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x68),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x656bba; continue; }
            pc = 0x656bb1; continue;
            }
            0x656bb1 => {
            c.set(0,32,rm(m,c.r[1],32));
            { let v = rm(m,c.r[4].wrapping_add(0x8),32); c.push(m, v); }
            { let target = rm(m,c.r[0].wrapping_add(0x64),32); c.push(m, 0x656bba); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x656bba; continue;
            }
            0x656bba => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656cdd(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656cdd;
    loop {
        match pc {
            0x656cdd => {
            c.set(0,32,0x668d08u32);
            { let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = 0x656ce7; }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(0,32)); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.r[5].wrapping_add(0xfffffff0));
            c.push(m, 0x656cff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x654c58, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x656d0a; continue; }
            pc = 0x656d07; continue;
            }
            0x656d07 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x656d0a; continue;
            }
            0x656d0a => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff0),32); c.push(m, v); }
            c.push(m, 0x656d17);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x656ca5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,32,c.r[5].wrapping_add(0xfffffff0));
            c.set(6,32,c.get(0,32));
            c.push(m, 0x656d25);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656d36(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656d36;
    loop {
        match pc {
            0x656d36 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,0x66a344u32,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(6,32,c.get(5,32));
            if c.cond(5) { pc = 0x656daf; continue; }
            pc = 0x656d4a; continue;
            }
            0x656d4a => {
            c.push(m, 0x656d4f);
            { let args = []; c.r[0] = host.call(m, 0x656dd2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x656da8; continue; }
            pc = 0x656d53; continue;
            }
            0x656d53 => {
            c.push(m, 0x656d58);
            { let args = []; c.r[0] = host.call(m, 0x6389fa, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x656da8; continue; }
            pc = 0x656d5c; continue;
            }
            0x656d5c => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x656d5e; continue;
            }
            0x656d5e => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(7,32,c.get(6,32));
            c.set(0,32,c.get(6,32));
            if c.cond(4) { pc = 0x656d6f; continue; }
            pc = 0x656d66; continue;
            }
            0x656d66 => {
            c.push(m,c.get(0,32));
            c.set(7,32,c.get(0,32));
            { let target = c.get(3,32); c.push(m, 0x656d6b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x656d66; continue; }
            pc = 0x656d6f; continue;
            }
            0x656d6f => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x656d80; continue; }
            pc = 0x656d73; continue;
            }
            0x656d73 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x656d80; continue; }
            pc = 0x656d77; continue;
            }
            0x656d77 => {
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a378u32,32); c.push(m, 0x656d7e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            pc = 0x656d80; continue;
            }
            0x656d80 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x656dc9; continue; }
            pc = 0x656d88; continue;
            }
            0x656d88 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x656dc6; continue; }
            pc = 0x656d8c; continue;
            }
            0x656d8c => {
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a33cu32,32); c.push(m, 0x656d93); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x656dc6; continue; }
            pc = 0x656d97; continue;
            }
            0x656d97 => {
            { let a = c.get(7,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x656dc6; continue; }
            pc = 0x656d9b; continue;
            }
            0x656d9b => {
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            { let a = c.r[3]; wm(m,a,32,c.get(7,32)); }
            { let target = rm(m,0x66a4dcu32,32); c.push(m, 0x656da6); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x656dc9; continue;
            }
            0x656da8 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x1c),32));
            pc = 0x656dab; continue;
            }
            0x656dab => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x656d5e; continue; }
            pc = 0x656daf; continue;
            }
            0x656daf => {
            c.push(m,0xfffffff0u32);
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a510u32,32); c.push(m, 0x656db8); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,0x40000000u32,32); }
            if c.cond(4) { pc = 0x656d5e; continue; }
            pc = 0x656dbf; continue;
            }
            0x656dbf => {
            c.push(m,c.get(6,32));
            { let target = c.get(3,32); c.push(m, 0x656dc2); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            pc = 0x656dab; continue;
            }
            0x656dc6 => {
            { let a = rm(m,c.r[3],32); let r = c.and(a,0x0u32,32); { let a = c.r[3]; wm(m,a,32,r); } }
            pc = 0x656dc9; continue;
            }
            0x656dc9 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065a144(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65a144;
    loop {
        match pc {
            0x65a144 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1],32));
            { let a = c.r[2].wrapping_add(c.r[0]); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065a157(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65a157;
    loop {
        match pc {
            0x65a157 => {
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x65a161; continue; }
            pc = 0x65a15d; continue;
            }
            0x65a15d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x65a19a; continue;
            }
            0x65a161 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x65a176; continue; }
            pc = 0x65a16c; continue;
            }
            0x65a16c => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[6]),32));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            pc = 0x65a196; continue;
            }
            0x65a176 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x65a199; continue; }
            pc = 0x65a17a; continue;
            }
            0x65a17a => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            pc = 0x65a17e; continue;
            }
            0x65a17e => {
            c.set(7,32,rm(m,c.r[1].wrapping_add(c.r[2]),32));
            { let a = c.get(7,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x65a18b; continue; }
            pc = 0x65a185; continue;
            }
            0x65a185 => {
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x65a17e; continue; }
            pc = 0x65a18b; continue;
            }
            0x65a18b => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(4) { pc = 0x65a199; continue; }
            pc = 0x65a190; continue;
            }
            0x65a190 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[6]),32));
            { let a = c.r[1].wrapping_add(c.r[2]); wm(m,a,32,c.get(0,32)); }
            pc = 0x65a196; continue;
            }
            0x65a196 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x65a199; continue;
            }
            0x65a199 => {
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x65a19a; continue;
            }
            0x65a19a => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065a19d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65a19d;
    loop {
        match pc {
            0x65a19d => {
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[4].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m,0x40u32);
            { let target = rm(m,0x66a1b8u32,32); c.push(m, 0x65a1aa); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x65a1b5; continue; }
            pc = 0x65a1b0; continue;
            }
            0x65a1b0 => {
            c.push(m, 0x65a1b5);
            { let args = []; c.r[0] = host.call(m, 0x64e49c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x65a1b5; continue;
            }
            0x65a1b5 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065a966(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65a966;
    loop {
        match pc {
            0x65a966 => {
            { let a = rm(m,0x6f00fcu32,32); c.sub(a,0x0u32,32); }
            c.push(m,c.get(6,32));
            if c.cond(5) { pc = 0x65a9a4; continue; }
            pc = 0x65a970; continue;
            }
            0x65a970 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x6f00fcu32; wm(m,a,32,c.get(6,32)); }
            { let target = rm(m,0x66a124u32,32); c.push(m, 0x65a97f); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(3) { pc = 0x65a992; continue; }
            pc = 0x65a983; continue;
            }
            0x65a983 => {
            { let a = c.get(0,32); c.and(a,0x80000000u32,32); }
            if c.cond(4) { pc = 0x65a992; continue; }
            pc = 0x65a98a; continue;
            }
            0x65a98a => {
            { let a = 0x6f00f8u32; wm(m,a,32,c.get(6,32)); }
            pc = 0x65a9a4; continue;
            }
            0x65a992 => {
            { let a = rm(m,0x6f00f8u32,32); let r = c.and(a,0x0u32,32); { let a = 0x6f00f8u32; wm(m,a,32,r); } }
            c.push(m,0x6eff48u32);
            { let target = rm(m,0x66a1a4u32,32); c.push(m, 0x65a9a4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x65a9a4; continue;
            }
            0x65a9a4 => {
            c.set(0,32,rm(m,0x6f00fcu32,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063a19f(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63a19f;
    loop {
        match pc {
            0x63a19f => {
            c.push(m,0x1u32);
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m, 0x63a1b2);
            { let args = []; c.r[0] = host.call(m, 0x639f9a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063cf26(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63cf26;
    loop {
        match pc {
            0x63cf26 => {
            c.push(m,0x1u32);
            { let v = rm(m,c.r[4].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x63cf31);
            { let args = []; c.r[0] = host.call(m, 0x63cf34, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063dc7a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63dc7a;
    loop {
        match pc {
            0x63dc7a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[1],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x63dc9e; continue; }
            pc = 0x63dc85; continue;
            }
            0x63dc85 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            pc = 0x63dc8d; continue;
            }
            0x63dc8d => {
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x63dc9e; continue; }
            pc = 0x63dc93; continue;
            }
            0x63dc93 => {
            c.set(7,32,rm(m,c.r[0],32));
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            { let a = rm(m,c.r[0],32); let r = c.inc(a,32); { let a = c.r[0]; wm(m,a,32,r); } }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = rm(m,c.r[1],32); let r = c.dec(a,32); { let a = c.r[1]; wm(m,a,32,r); } }
            if c.cond(5) { pc = 0x63dc8d; continue; }
            pc = 0x63dc9e; continue;
            }
            0x63dc9e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063dca1(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63dca1;
    loop {
        match pc {
            0x63dca1 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = rm(m,0x6f1a9cu32,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x63dcc6; continue; }
            pc = 0x63dcb3; continue;
            }
            0x63dcb3 => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x63dcc1);
            { let args = []; c.r[0] = host.call(m, 0x63dd10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x63dd0d; continue;
            }
            0x63dcc6 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[7],32),32); }
            if c.cond(3) { pc = 0x63dd0b; continue; }
            pc = 0x63dcd0; continue;
            }
            0x63dcd0 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(0,32,c.r[1].wrapping_add(0x1));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63dd01; continue; }
            pc = 0x63dcdc; continue;
            }
            0x63dcdc => {
            c.push(m,c.get(3,32));
            pc = 0x63dcdd; continue;
            }
            0x63dcdd => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,0xau32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(3,32,v); }
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(2,8); let r = c.add(a,0x30u32,8); c.set(2,8,r); }
            { let a = c.r[0].wrapping_add(c.r[1]); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0xffffffffu32,32); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x63dcdd; continue; }
            pc = 0x63dd00; continue;
            }
            0x63dd00 => {
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x63dd01; continue;
            }
            0x63dd01 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = rm(m,c.r[6],32); let r = c.add(a,c.get(0,32),32); { let a = c.r[6]; wm(m,a,32,r); } }
            { let a = rm(m,c.r[7],32); let r = c.sub(a,c.get(0,32),32); { let a = c.r[7]; wm(m,a,32,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x63dd0d; continue;
            }
            0x63dd0b => {
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            pc = 0x63dd0d; continue;
            }
            0x63dd0d => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063dd59(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63dd59;
    loop {
        match pc {
            0x63dd59 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[3],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x63de0c; continue; }
            pc = 0x63dd6c; continue;
            }
            0x63dd6c => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x10),32));
            pc = 0x63dd72; continue;
            }
            0x63dd72 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[7],32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x63de0c; continue; }
            pc = 0x63dd7c; continue;
            }
            0x63dd7c => {
            { let a = 0x6f1a9cu32; wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[3],8));
            { let a = c.r[5].wrapping_add(0xb); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[3].wrapping_add(0x1));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x63dd90; continue;
            }
            0x63dd90 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0xb),8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            c.set(0,8,rm(m,c.r[0],8));
            { let a = c.get(0,8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x63dd90; continue; }
            pc = 0x63dda0; continue;
            }
            0x63dda0 => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,sx(c.get(2,8),8)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,0x64u32,32); }
            if c.cond(15) { pc = 0x63df1e; continue; }
            pc = 0x63ddb2; continue;
            }
            0x63ddb2 => {
            if c.cond(4) { pc = 0x63dee9; continue; }
            pc = 0x63ddb8; continue;
            }
            0x63ddb8 => {
            { let a = c.get(0,32); c.sub(a,0x27u32,32); }
            if c.cond(4) { pc = 0x63de93; continue; }
            pc = 0x63ddc1; continue;
            }
            0x63ddc1 => {
            { let a = c.get(0,32); c.sub(a,0x41u32,32); }
            if c.cond(4) { pc = 0x63de5d; continue; }
            pc = 0x63ddca; continue;
            }
            0x63ddca => {
            { let a = c.get(0,32); c.sub(a,0x48u32,32); }
            if c.cond(4) { pc = 0x63de42; continue; }
            pc = 0x63ddcf; continue;
            }
            0x63ddcf => {
            { let a = c.get(0,32); c.sub(a,0x4du32,32); }
            if c.cond(4) { pc = 0x63de11; continue; }
            pc = 0x63ddd4; continue;
            }
            0x63ddd4 => {
            { let a = c.get(0,32); c.sub(a,0x61u32,32); }
            if c.cond(4) { pc = 0x63de5d; continue; }
            pc = 0x63dddd; continue;
            }
            0x63dddd => {
            c.set(1,32,rm(m,0x6c8760u32,32));
            c.set(0,32,c.get(2,8));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x1),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x63ddf8; continue; }
            pc = 0x63dded; continue;
            }
            0x63dded => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            { let a = rm(m,c.r[6],32); let r = c.inc(a,32); { let a = c.r[6]; wm(m,a,32,r); } }
            { let a = rm(m,c.r[7],32); let r = c.dec(a,32); { let a = c.r[7]; wm(m,a,32,r); } }
            pc = 0x63ddf8; continue;
            }
            0x63ddf8 => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,8,rm(m,c.r[3],8));
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            { let a = rm(m,c.r[6],32); let r = c.inc(a,32); { let a = c.r[6]; wm(m,a,32,r); } }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = rm(m,c.r[7],32); let r = c.dec(a,32); { let a = c.r[7]; wm(m,a,32,r); } }
            pc = 0x63de03; continue;
            }
            0x63de03 => {
            { let a = rm(m,c.r[3],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x63dd72; continue; }
            pc = 0x63de0c; continue;
            }
            0x63de0c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63de11 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63de2f; continue; }
            pc = 0x63de14; continue;
            }
            0x63de14 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63de39; continue; }
            pc = 0x63de17; continue;
            }
            0x63de17 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63de26; continue; }
            pc = 0x63de1a; continue;
            }
            0x63de1a => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x63dddd; continue; }
            pc = 0x63de1d; continue;
            }
            0x63de1d => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x42u32); }
            pc = 0x63e013; continue;
            }
            0x63de26 => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x62u32); }
            pc = 0x63e013; continue;
            }
            0x63de2f => {
            { let a = 0x6f1a9cu32; wm(m,a,32,0x1u32); }
            pc = 0x63de39; continue;
            }
            0x63de39 => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x6du32); }
            pc = 0x63e013; continue;
            }
            0x63de42 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63de4a; continue; }
            pc = 0x63de45; continue;
            }
            0x63de45 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63de54; continue; }
            pc = 0x63de48; continue;
            }
            0x63de48 => {
            pc = 0x63dddd; continue;
            }
            0x63de4a => {
            { let a = 0x6f1a9cu32; wm(m,a,32,0x1u32); }
            pc = 0x63de54; continue;
            }
            0x63de54 => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x48u32); }
            pc = 0x63e013; continue;
            }
            0x63de5d => {
            c.push(m,0x67d3e0u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x63de68);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x63de73; continue; }
            pc = 0x63de6e; continue;
            }
            0x63de6e => {
            { let a = c.get(3,32); let r = c.add(a,0x5u32,32); c.set(3,32,r); }
            pc = 0x63de87; continue;
            }
            0x63de73 => {
            c.push(m,0x67d3dcu32);
            c.push(m,c.get(3,32));
            c.push(m, 0x63de7e);
            { let args = []; c.r[0] = host.call(m, 0x643db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x63de8a; continue; }
            pc = 0x63de84; continue;
            }
            0x63de84 => {
            { let a = c.get(3,32); let r = c.add(a,0x3u32,32); c.set(3,32,r); }
            pc = 0x63de87; continue;
            }
            0x63de87 => {
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x63de8a; continue;
            }
            0x63de8a => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x70u32); }
            pc = 0x63e013; continue;
            }
            0x63de93 => {
            { let a = c.get(1,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x63dee2; continue; }
            pc = 0x63de98; continue;
            }
            0x63de98 => {
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            pc = 0x63de9a; continue;
            }
            0x63de9a => {
            c.set(0,8,rm(m,c.r[3],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x63de03; continue; }
            pc = 0x63dea4; continue;
            }
            0x63dea4 => {
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x63de03; continue; }
            pc = 0x63dead; continue;
            }
            0x63dead => {
            { let a = c.get(0,8); c.sub(a,0x27u32,8); }
            if c.cond(4) { pc = 0x63dedc; continue; }
            pc = 0x63deb1; continue;
            }
            0x63deb1 => {
            c.set(2,32,rm(m,0x6c8760u32,32));
            c.set(1,32,c.get(0,8));
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(2)).wrapping_add(0x1),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x63decf; continue; }
            pc = 0x63dec1; continue;
            }
            0x63dec1 => {
            { let a = rm(m,c.r[7],32); c.sub(a,0x1u32,32); }
            if c.cond(6) { pc = 0x63decf; continue; }
            pc = 0x63dec6; continue;
            }
            0x63dec6 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            { let a = rm(m,c.r[6],32); let r = c.inc(a,32); { let a = c.r[6]; wm(m,a,32,r); } }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = rm(m,c.r[7],32); let r = c.dec(a,32); { let a = c.r[7]; wm(m,a,32,r); } }
            pc = 0x63decf; continue;
            }
            0x63decf => {
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,8,rm(m,c.r[3],8));
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            { let a = rm(m,c.r[6],32); let r = c.inc(a,32); { let a = c.r[6]; wm(m,a,32,r); } }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = rm(m,c.r[7],32); let r = c.dec(a,32); { let a = c.r[7]; wm(m,a,32,r); } }
            pc = 0x63de9a; continue;
            }
            0x63dedc => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            pc = 0x63de03; continue;
            }
            0x63dee2 => {
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            pc = 0x63de03; continue;
            }
            0x63dee9 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63df0b; continue; }
            pc = 0x63deec; continue;
            }
            0x63deec => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63df15; continue; }
            pc = 0x63deef; continue;
            }
            0x63deef => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63df02; continue; }
            pc = 0x63def2; continue;
            }
            0x63def2 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x63dddd; continue; }
            pc = 0x63def9; continue;
            }
            0x63def9 => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x41u32); }
            pc = 0x63e013; continue;
            }
            0x63df02 => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x61u32); }
            pc = 0x63e013; continue;
            }
            0x63df0b => {
            { let a = 0x6f1a9cu32; wm(m,a,32,0x1u32); }
            pc = 0x63df15; continue;
            }
            0x63df15 => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x64u32); }
            pc = 0x63e013; continue;
            }
            0x63df1e => {
            { let a = c.get(0,32); let r = c.sub(a,0x68u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63dffa; continue; }
            pc = 0x63df27; continue;
            }
            0x63df27 => {
            { let a = c.get(0,32); let r = c.sub(a,0x5u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63dfdf; continue; }
            pc = 0x63df30; continue;
            }
            0x63df30 => {
            { let a = c.get(0,32); let r = c.sub(a,0x6u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63dfc4; continue; }
            pc = 0x63df39; continue;
            }
            0x63df39 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63df65; continue; }
            pc = 0x63df3c; continue;
            }
            0x63df3c => {
            { let a = c.get(0,32); let r = c.sub(a,0x5u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x63dddd; continue; }
            pc = 0x63df45; continue;
            }
            0x63df45 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63df5c; continue; }
            pc = 0x63df4b; continue;
            }
            0x63df4b => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x63dddd; continue; }
            pc = 0x63df53; continue;
            }
            0x63df53 => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x59u32); }
            pc = 0x63e013; continue;
            }
            0x63df5c => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x79u32); }
            pc = 0x63e013; continue;
            }
            0x63df65 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = rm(m,c.r[0].wrapping_add(0x8),32); c.sub(a,0xbu32,32); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            if c.cond(15) { pc = 0x63df79; continue; }
            pc = 0x63df71; continue;
            }
            0x63df71 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x98),32));
            pc = 0x63df7f; continue;
            }
            0x63df79 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x9c),32));
            pc = 0x63df7f; continue;
            }
            0x63df7f => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x63dfbc; continue; }
            pc = 0x63df83; continue;
            }
            0x63df83 => {
            { let a = rm(m,c.r[7],32); c.sub(a,0x0u32,32); }
            if c.cond(6) { pc = 0x63dfbc; continue; }
            pc = 0x63df88; continue;
            }
            0x63df88 => {
            c.set(1,8,rm(m,c.r[0],8));
            c.set(3,32,rm(m,0x6c8760u32,32));
            c.set(2,32,c.get(1,8));
            { let a = rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x1),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x63dfa8; continue; }
            pc = 0x63df9a; continue;
            }
            0x63df9a => {
            { let a = rm(m,c.r[7],32); c.sub(a,0x1u32,32); }
            if c.cond(6) { pc = 0x63dfa8; continue; }
            pc = 0x63df9f; continue;
            }
            0x63df9f => {
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            { let a = rm(m,c.r[6],32); let r = c.inc(a,32); { let a = c.r[6]; wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = rm(m,c.r[7],32); let r = c.dec(a,32); { let a = c.r[7]; wm(m,a,32,r); } }
            pc = 0x63dfa8; continue;
            }
            0x63dfa8 => {
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.r[1]; wm(m,a,8,c.get(2,8)); }
            { let a = rm(m,c.r[6],32); let r = c.inc(a,32); { let a = c.r[6]; wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = rm(m,c.r[7],32); let r = c.dec(a,32); { let a = c.r[7]; wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            if c.cond(15) { pc = 0x63df83; continue; }
            pc = 0x63dfbc; continue;
            }
            0x63dfbc => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x10),32));
            pc = 0x63de03; continue;
            }
            0x63dfc4 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63dfcf; continue; }
            pc = 0x63dfc7; continue;
            }
            0x63dfc7 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63dfd9; continue; }
            pc = 0x63dfca; continue;
            }
            0x63dfca => {
            pc = 0x63dddd; continue;
            }
            0x63dfcf => {
            { let a = 0x6f1a9cu32; wm(m,a,32,0x1u32); }
            pc = 0x63dfd9; continue;
            }
            0x63dfd9 => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x53u32); }
            pc = 0x63e013; continue;
            }
            0x63dfdf => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63dfea; continue; }
            pc = 0x63dfe2; continue;
            }
            0x63dfe2 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63dff4; continue; }
            pc = 0x63dfe5; continue;
            }
            0x63dfe5 => {
            pc = 0x63dddd; continue;
            }
            0x63dfea => {
            { let a = 0x6f1a9cu32; wm(m,a,32,0x1u32); }
            pc = 0x63dff4; continue;
            }
            0x63dff4 => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x4du32); }
            pc = 0x63e013; continue;
            }
            0x63dffa => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63e005; continue; }
            pc = 0x63dffd; continue;
            }
            0x63dffd => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x63e00f; continue; }
            pc = 0x63e000; continue;
            }
            0x63e000 => {
            pc = 0x63dddd; continue;
            }
            0x63e005 => {
            { let a = 0x6f1a9cu32; wm(m,a,32,0x1u32); }
            pc = 0x63e00f; continue;
            }
            0x63e00f => {
            { let a = c.r[5].wrapping_add(0x14); wm(m,a,8,0x49u32); }
            pc = 0x63e013; continue;
            }
            0x63e013 => {
            { let v = rm(m,c.r[5].wrapping_add(0x18),32); c.push(m, v); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m, 0x63e023);
            { let args = []; c.r[0] = host.call(m, 0x63d88b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            pc = 0x63dfbc; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00644bb0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x644bb0;
    loop {
        match pc {
            0x644bb0 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x644bde; continue; }
            pc = 0x644bba; continue;
            }
            0x644bba => {
            { let a = rm(m,c.r[5].wrapping_add(0xb),8); let r = c.and(a,0x0u32,8); { let a = c.r[5].wrapping_add(0xb); wm(m,a,8,r); } }
            { let a = c.get(0,8); let r = c.add(a,0x40u32,8); c.set(0,8,r); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            { let a = c.r[5].wrapping_add(0x9); wm(m,a,8,0x3au32); }
            { let a = c.r[5].wrapping_add(0xa); wm(m,a,8,0x5cu32); }
            { let target = rm(m,0x66a2a8u32,32); c.push(m, 0x644bd5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x644be3; continue; }
            pc = 0x644bd9; continue;
            }
            0x644bd9 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(4) { pc = 0x644be3; continue; }
            pc = 0x644bde; continue;
            }
            0x644bde => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x644be3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00645103(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x645103;
    loop {
        match pc {
            0x645103 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,0x1u32,32); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            if c.cond(5) { pc = 0x64519e; continue; }
            pc = 0x645115; continue;
            }
            0x645115 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.and(a,0x3u32,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            c.set(6,32,c.get(0,32));
            if c.cond(5) { pc = 0x64512e; continue; }
            pc = 0x645123; continue;
            }
            0x645123 => {
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x6cb1b0),32));
            pc = 0x645137; continue;
            }
            0x64512e => {
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x6cb1e4),32));
            pc = 0x645137; continue;
            }
            0x645137 => {
            c.set(2,32,c.get(3,32));
            c.set(1,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(2,32); let r = c.imul(a,0x16du32,32); c.set(2,32,r); }
            c.set(0,32,c.r[3].wrapping_add(0xffffffff));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(7,32,c.get(1,32));
            c.push(m,0x7u32);
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[7]).wrapping_add(0xffff9c25));
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x1c),32),32); }
            if c.cond(15) { pc = 0x645171; continue; }
            pc = 0x645163; continue;
            }
            0x645163 => {
            { let a = c.get(0,32); let r = c.imul(a,0x7u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x1c),32),32); c.set(0,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xfffffff9));
            pc = 0x64517b; continue;
            }
            0x645171 => {
            { let a = c.get(0,32); let r = c.imul(a,0x7u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x1c),32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x64517b; continue;
            }
            0x64517b => {
            { let a = rm(m,c.r[5].wrapping_add(0x18),32); c.sub(a,0x5u32,32); }
            if c.cond(5) { pc = 0x6451b9; continue; }
            pc = 0x645181; continue;
            }
            0x645181 => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x64518f; continue; }
            pc = 0x645187; continue;
            }
            0x645187 => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x6cb1b4),32));
            pc = 0x645195; continue;
            }
            0x64518f => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x6cb1e8),32));
            pc = 0x645195; continue;
            }
            0x645195 => {
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x6451b9; continue; }
            pc = 0x645199; continue;
            }
            0x645199 => {
            { let a = c.get(1,32); let r = c.sub(a,0x7u32,32); c.set(1,32,r); }
            pc = 0x6451b9; continue;
            }
            0x64519e => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = c.get(3,8); c.and(a,0x3u32,8); }
            if c.cond(5) { pc = 0x6451af; continue; }
            pc = 0x6451a6; continue;
            }
            0x6451a6 => {
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6cb1b0),32));
            pc = 0x6451b6; continue;
            }
            0x6451af => {
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6cb1e4),32));
            pc = 0x6451b6; continue;
            }
            0x6451b6 => {
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x20),32),32); c.set(1,32,r); }
            pc = 0x6451b9; continue;
            }
            0x6451b9 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x6451ea; continue; }
            pc = 0x6451bf; continue;
            }
            0x6451bf => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x24),32));
            { let a = 0x6cb19cu32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.imul(a,0x3cu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x28),32),32); c.set(0,32,r); }
            { let a = 0x6cb198u32; wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.imul(a,0x3cu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x2c),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,0x3e8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x30),32),32); c.set(0,32,r); }
            { let a = 0x6cb1a0u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x64523f; continue;
            }
            0x6451ea => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x24),32));
            { let a = 0x6cb1acu32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let r = c.imul(a,0x3cu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x28),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,0x3cu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,0x6cb108u32,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x2c),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,0x3e8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x30),32),32); c.set(0,32,r); }
            { let a = 0x6cb1b0u32; wm(m,a,32,c.get(0,32)); }
            if c.cond(9) { pc = 0x645222; continue; }
            pc = 0x645215; continue;
            }
            0x645215 => {
            { let a = c.get(0,32); let r = c.add(a,0x5265c00u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = 0x6cb1b0u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x645233; continue;
            }
            0x645222 => {
            c.set(2,32,0x5265c00u32);
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x645239; continue; }
            pc = 0x64522b; continue;
            }
            0x64522b => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = 0x6cb1b0u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x645233; continue;
            }
            0x645233 => {
            { let a = 0x6cb1acu32; wm(m,a,32,c.get(1,32)); }
            pc = 0x645239; continue;
            }
            0x645239 => {
            { let a = 0x6cb1a8u32; wm(m,a,32,c.get(3,32)); }
            pc = 0x64523f; continue;
            }
            0x64523f => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00645747(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x645747;
    loop {
        match pc {
            0x645747 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xa),16));
            c.set(3,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.and(a,0x8000u32,32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x6),32));
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x2),32));
            c.set(0,32,rm(m,c.r[0],16));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.and(a,0x7fffu32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,0x3fffu32,32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,0xffffc001u32,32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x6457b4; continue; }
            pc = 0x64578e; continue;
            }
            0x64578e => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x645799);
            { let args = []; c.r[0] = host.call(m, 0x64569f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x645873; continue; }
            pc = 0x6457a2; continue;
            }
            0x6457a2 => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            c.push(m, 0x6457ab);
            { let args = []; c.r[0] = host.call(m, 0x645693, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x6457ac; continue;
            }
            0x6457ac => {
            c.push(m,0x2u32);
            pc = 0x6457ae; continue;
            }
            0x6457ae => {
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x645875; continue;
            }
            0x6457b4 => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m,c.get(0,32));
            c.push(m, 0x6457c1);
            { let args = []; c.r[0] = host.call(m, 0x645678, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[7].wrapping_add(0x8),32); c.push(m, v); }
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            c.push(m, 0x6457cd);
            { let args = []; c.r[0] = host.call(m, 0x6455ec, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6457d5; continue; }
            pc = 0x6457d4; continue;
            }
            0x6457d4 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            pc = 0x6457d5; continue;
            }
            0x6457d5 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[7].wrapping_add(0x8),32),32); c.set(1,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x6457ed; continue; }
            pc = 0x6457e1; continue;
            }
            0x6457e1 => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            c.push(m, 0x6457ea);
            { let args = []; c.r[0] = host.call(m, 0x645693, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x645829; continue;
            }
            0x6457ed => {
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x645830; continue; }
            pc = 0x6457f1; continue;
            }
            0x6457f1 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(6,32,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            c.push(m, 0x645802);
            { let args = []; c.r[0] = host.call(m, 0x645678, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64580c);
            { let args = []; c.r[0] = host.call(m, 0x6456ba, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[7].wrapping_add(0x8),32); c.push(m, v); }
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            c.push(m, 0x645818);
            { let args = []; c.r[0] = host.call(m, 0x6455ec, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            c.push(m, 0x645826);
            { let args = []; c.r[0] = host.call(m, 0x6456ba, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            pc = 0x645829; continue;
            }
            0x645829 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x6457ac; continue;
            }
            0x645830 => {
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[7],32),32); }
            if c.cond(12) { pc = 0x64585c; continue; }
            pc = 0x645834; continue;
            }
            0x645834 => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            c.push(m, 0x64583d);
            { let args = []; c.r[0] = host.call(m, 0x645693, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[7].wrapping_add(0xc),32); c.push(m, v); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff7),8); let r = c.or(a,0x80u32,8); { let a = c.r[5].wrapping_add(0xfffffff7); wm(m,a,8,r); } }
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            c.push(m, 0x64584d);
            { let args = []; c.r[0] = host.call(m, 0x6456ba, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,rm(m,c.r[7],32),32); c.set(6,32,r); }
            c.push(m,0x1u32);
            pc = 0x6457ae; continue;
            }
            0x64585c => {
            { let v = rm(m,c.r[7].wrapping_add(0xc),32); c.push(m, v); }
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x14),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff7),8); let r = c.and(a,0x7fu32,8); { let a = c.r[5].wrapping_add(0xfffffff7); wm(m,a,8,r); } }
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(0,32));
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            c.push(m, 0x645871);
            { let args = []; c.r[0] = host.call(m, 0x6456ba, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x645873; continue;
            }
            0x645873 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x645875; continue;
            }
            0x645875 => {
            c.push(m,0x1fu32);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[7].wrapping_add(0xc),32),32); c.set(1,32,r); }
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.neg(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x80000000u32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.or(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.or(a,rm(m,c.r[5].wrapping_add(0xfffffff4),32),32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,0x40u32,32); }
            if c.cond(5) { pc = 0x6458a4; continue; }
            pc = 0x645897; continue;
            }
            0x645897 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            pc = 0x6458ae; continue;
            }
            0x6458a4 => {
            { let a = c.get(7,32); c.sub(a,0x20u32,32); }
            if c.cond(5) { pc = 0x6458ae; continue; }
            pc = 0x6458a9; continue;
            }
            0x6458a9 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.r[1]; wm(m,a,32,c.get(6,32)); }
            pc = 0x6458ae; continue;
            }
            0x6458ae => {
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

pub fn f_00646a99(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x646a99;
    loop {
        match pc {
            0x646a99 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            { let a = c.get(1,32); c.sub(a,rm(m,0x6f0a40u32,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x646aff; continue; }
            pc = 0x646aa7; continue;
            }
            0x646aa7 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_mul(4).wrapping_add(0x6f0940));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x4),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x646aff; continue; }
            pc = 0x646ac8; continue;
            }
            0x646ac8 => {
            { let a = rm(m,c.r[0],32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x646aff; continue; }
            pc = 0x646acd; continue;
            }
            0x646acd => {
            { let a = rm(m,0x6c8414u32,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x646af5; continue; }
            pc = 0x646ad6; continue;
            }
            0x646ad6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x646aec; continue; }
            pc = 0x646adc; continue;
            }
            0x646adc => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x646ae7; continue; }
            pc = 0x646adf; continue;
            }
            0x646adf => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x646af5; continue; }
            pc = 0x646ae2; continue;
            }
            0x646ae2 => {
            c.push(m,c.get(0,32));
            c.push(m,0xfffffff4u32);
            pc = 0x646aef; continue;
            }
            0x646ae7 => {
            c.push(m,c.get(0,32));
            c.push(m,0xfffffff5u32);
            pc = 0x646aef; continue;
            }
            0x646aec => {
            c.push(m,c.get(0,32));
            c.push(m,0xfffffff6u32);
            pc = 0x646aef; continue;
            }
            0x646aef => {
            { let target = rm(m,0x66a284u32,32); c.push(m, 0x646af5); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x646af5; continue;
            }
            0x646af5 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x646b15; continue;
            }
            0x646aff => {
            c.push(m, 0x646b04);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x9u32); }
            c.push(m, 0x646b0f);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0],32); let r = c.and(a,0x0u32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x646b15; continue;
            }
            0x646b15 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064858c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64858c;
    loop {
        match pc {
            0x64858c => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.push(m,c.get(6,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x404eu32); }
            { let a = c.r[3]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[3].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            if c.cond(6) { pc = 0x648600; continue; }
            pc = 0x6485af; continue;
            }
            0x6485af => {
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x6485b3; continue;
            }
            0x6485b3 => {
            c.set(6,32,c.get(3,32));
            c.set(7,32,c.r[5].wrapping_add(0xfffffff0));
            c.movs(m,32,false);
            c.movs(m,32,false);
            c.push(m,c.get(3,32));
            c.movs(m,32,false);
            c.push(m, 0x6485c1);
            { let args = []; c.r[0] = host.call(m, 0x648531, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m, 0x6485c7);
            { let args = []; c.r[0] = host.call(m, 0x648531, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[5].wrapping_add(0xfffffff0));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x6485d1);
            { let args = []; c.r[0] = host.call(m, 0x6484d3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m, 0x6485d7);
            { let args = []; c.r[0] = host.call(m, 0x648531, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff4),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            c.set(0,32,sx(rm(m,c.r[0],8),8)&0xffffffffu32);
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0xfffffff0));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x6485f2);
            { let args = []; c.r[0] = host.call(m, 0x6484d3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            if c.cond(5) { pc = 0x6485b3; continue; }
            pc = 0x6485fd; continue;
            }
            0x6485fd => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x648600; continue;
            }
            0x648600 => {
            { let a = rm(m,c.r[3].wrapping_add(0x8),32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x64862d; continue; }
            pc = 0x648605; continue;
            }
            0x648605 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[3],32));
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); let n = 0x10u32; let r = c.shr(a,n,32); c.set(6,32,r); }
            { let a = c.get(1,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.or(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.add(a,0xfff0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            pc = 0x648600; continue;
            }
            0x64862d => {
            c.set(6,32,0x8000u32);
            pc = 0x648632; continue;
            }
            0x648632 => {
            { let a = rm(m,c.r[3].wrapping_add(0x8),32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x648647; continue; }
            pc = 0x648637; continue;
            }
            0x648637 => {
            c.push(m,c.get(3,32));
            c.push(m, 0x64863d);
            { let args = []; c.r[0] = host.call(m, 0x648531, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.add(a,0xffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x648632; continue;
            }
            0x648647 => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xfffffffc),16));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[3].wrapping_add(0xa); wm(m,a,16,c.get(0,16)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00648b06(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x648b06;
    loop {
        match pc {
            0x648b06 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,0x6cb400u32);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,0x60u32,32); c.set(3,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x648b7f; continue; }
            pc = 0x648b1c; continue;
            }
            0x648b1c => {
            if c.cond(13) { pc = 0x648b2e; continue; }
            pc = 0x648b1e; continue;
            }
            0x648b1e => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(3,32,0x6cb560u32);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); let r = c.sub(a,0x60u32,32); c.set(3,32,r); }
            pc = 0x648b2e; continue;
            }
            0x648b2e => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x648b39; continue; }
            pc = 0x648b33; continue;
            }
            0x648b33 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            pc = 0x648b39; continue;
            }
            0x648b39 => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x648b7f; continue; }
            pc = 0x648b3e; continue;
            }
            0x648b3e => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            pc = 0x648b40; continue;
            }
            0x648b40 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(3,32); let r = c.add(a,0x54u32,32); c.set(3,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); let n = 0x3u32; let r = c.sar(a,n,32); { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.and(a,0x7u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x648b78; continue; }
            pc = 0x648b51; continue;
            }
            0x648b51 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)),16); c.sub(a,0x8000u32,16); }
            c.set(6,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            if c.cond(2) { pc = 0x648b6b; continue; }
            pc = 0x648b5f; continue;
            }
            0x648b5f => {
            c.set(7,32,c.r[5].wrapping_add(0xfffffff4));
            c.movs(m,32,false);
            c.movs(m,32,false);
            c.movs(m,32,false);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff6),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffff6); wm(m,a,32,r); } }
            c.set(6,32,c.r[5].wrapping_add(0xfffffff4));
            pc = 0x648b6b; continue;
            }
            0x648b6b => {
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x648b74);
            { let args = []; c.r[0] = host.call(m, 0x6488e6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x648b78; continue;
            }
            0x648b78 => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x648b40; continue; }
            pc = 0x648b7d; continue;
            }
            0x648b7d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x648b7f; continue;
            }
            0x648b7f => {
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064eda5(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64eda5;
    loop {
        match pc {
            0x64eda5 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64edba; continue; }
            pc = 0x64edaf; continue;
            }
            0x64edaf => {
            c.push(m,c.get(0,32));
            c.push(m, 0x64edb5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x64edba; continue;
            }
            0x64edba => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[6].wrapping_add(0x10),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,r); } }
            c.push(m, 0x64edca);
            { let args = []; c.r[0] = host.call(m, 0x64ed07, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x14),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00654281(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x654281;
    loop {
        match pc {
            0x654281 => {
            c.push(m, 0x654286);
            { let args = []; c.r[0] = host.call(m, 0x6599a7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x654295; continue; }
            pc = 0x65428d; continue;
            }
            0x65428d => {
            c.push(m, 0x654292);
            { let args = []; c.r[0] = host.call(m, 0x659981, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            pc = 0x654295; continue;
            }
            0x654295 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00656dd2(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x656dd2;
    loop {
        match pc {
            0x656dd2 => {
            c.push(m, 0x656dd7);
            { let args = []; c.r[0] = host.call(m, 0x659754, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc4),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063cf34(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63cf34;
    loop {
        match pc {
            0x63cf34 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x14),32));
            { let a = c.get(1,32); c.sub(a,0x45u32,32); }
            if c.cond(12) { pc = 0x63d111; continue; }
            pc = 0x63cf48; continue;
            }
            0x63cf48 => {
            { let a = c.get(1,32); c.sub(a,0x8bu32,32); }
            if c.cond(15) { pc = 0x63d111; continue; }
            pc = 0x63cf54; continue;
            }
            0x63cf54 => {
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x63cf60; continue; }
            pc = 0x63cf5b; continue;
            }
            0x63cf5b => {
            { let a = c.get(6,32); c.sub(a,0xbu32,32); }
            if c.cond(14) { pc = 0x63cf96; continue; }
            pc = 0x63cf60; continue;
            }
            0x63cf60 => {
            c.push(m,c.get(3,32));
            c.set(0,32,c.get(6,32));
            c.push(m,0xcu32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(3,32,v); }
            { let b = c.get(3,32); c.idiv32(b); }
            c.push(m,c.get(3,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.get(6,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(13) { pc = 0x63cf81; continue; }
            pc = 0x63cf7a; continue;
            }
            0x63cf7a => {
            { let a = c.get(2,32); let r = c.add(a,0xcu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            pc = 0x63cf81; continue;
            }
            0x63cf81 => {
            { let a = c.get(1,32); c.sub(a,0x45u32,32); }
            if c.cond(12) { pc = 0x63d111; continue; }
            pc = 0x63cf8a; continue;
            }
            0x63cf8a => {
            { let a = c.get(1,32); c.sub(a,0x8bu32,32); }
            if c.cond(15) { pc = 0x63d111; continue; }
            pc = 0x63cf96; continue;
            }
            0x63cf96 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(1,8); c.and(a,0x3u32,8); }
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6cb1e8),32));
            if c.cond(5) { pc = 0x63cfab; continue; }
            pc = 0x63cfa5; continue;
            }
            0x63cfa5 => {
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x63cfab; continue; }
            pc = 0x63cfaa; continue;
            }
            0x63cfaa => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x63cfab; continue;
            }
            0x63cfab => {
            c.set(2,32,c.r[1].wrapping_add(0xffffffff));
            { let a = c.get(1,32); let r = c.imul(a,0x16du32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0xffff9c21));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(6,32,c.r[0].wrapping_add(c.r[1]));
            if c.cond(12) { pc = 0x63cfd8; continue; }
            pc = 0x63cfca; continue;
            }
            0x63cfca => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x63cfe4; continue; }
            pc = 0x63cfce; continue;
            }
            0x63cfce => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x63d111; continue; }
            pc = 0x63cfd6; continue;
            }
            0x63cfd6 => {
            pc = 0x63cfe4; continue;
            }
            0x63cfd8 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x63cfe4; continue; }
            pc = 0x63cfdc; continue;
            }
            0x63cfdc => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x63d111; continue; }
            pc = 0x63cfe4; continue;
            }
            0x63cfe4 => {
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x63cfff; continue; }
            pc = 0x63cfee; continue;
            }
            0x63cfee => {
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            c.set(2,8,u32::from(c.cond(5)));
            c.set(0,32,c.get(2,32));
            pc = 0x63d001; continue;
            }
            0x63cfff => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63d001; continue;
            }
            0x63d001 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x63d111; continue; }
            pc = 0x63d009; continue;
            }
            0x63d009 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(6,32,c.r[0].wrapping_add(c.r[1]));
            if c.cond(12) { pc = 0x63d021; continue; }
            pc = 0x63d013; continue;
            }
            0x63d013 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x63d02d; continue; }
            pc = 0x63d017; continue;
            }
            0x63d017 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x63d111; continue; }
            pc = 0x63d01f; continue;
            }
            0x63d01f => {
            pc = 0x63d02d; continue;
            }
            0x63d021 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x63d02d; continue; }
            pc = 0x63d025; continue;
            }
            0x63d025 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x63d111; continue; }
            pc = 0x63d02d; continue;
            }
            0x63d02d => {
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.imul(a,0x3cu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x63d047; continue; }
            pc = 0x63d036; continue;
            }
            0x63d036 => {
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3cu32,32); }
            c.set(2,8,u32::from(c.cond(5)));
            c.set(0,32,c.get(2,32));
            pc = 0x63d049; continue;
            }
            0x63d047 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63d049; continue;
            }
            0x63d049 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x63d111; continue; }
            pc = 0x63d051; continue;
            }
            0x63d051 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(6,32,c.r[0].wrapping_add(c.r[1]));
            if c.cond(12) { pc = 0x63d069; continue; }
            pc = 0x63d05b; continue;
            }
            0x63d05b => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x63d075; continue; }
            pc = 0x63d05f; continue;
            }
            0x63d05f => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(12) { pc = 0x63d111; continue; }
            pc = 0x63d067; continue;
            }
            0x63d067 => {
            pc = 0x63d075; continue;
            }
            0x63d069 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x63d075; continue; }
            pc = 0x63d06d; continue;
            }
            0x63d06d => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x63d111; continue; }
            pc = 0x63d075; continue;
            }
            0x63d075 => {
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.imul(a,0x3cu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x63d08f; continue; }
            pc = 0x63d07e; continue;
            }
            0x63d07e => {
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3cu32,32); }
            c.set(2,8,u32::from(c.cond(5)));
            c.set(0,32,c.get(2,32));
            pc = 0x63d091; continue;
            }
            0x63d08f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63d091; continue;
            }
            0x63d091 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x63d111; continue; }
            pc = 0x63d095; continue;
            }
            0x63d095 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            if c.cond(12) { pc = 0x63d0ab; continue; }
            pc = 0x63d0a1; continue;
            }
            0x63d0a1 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x63d0b3; continue; }
            pc = 0x63d0a5; continue;
            }
            0x63d0a5 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x63d111; continue; }
            pc = 0x63d0a9; continue;
            }
            0x63d0a9 => {
            pc = 0x63d0b3; continue;
            }
            0x63d0ab => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x63d0b3; continue; }
            pc = 0x63d0af; continue;
            }
            0x63d0af => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x63d111; continue; }
            pc = 0x63d0b3; continue;
            }
            0x63d0b3 => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x63d0f7; continue; }
            pc = 0x63d0b9; continue;
            }
            0x63d0b9 => {
            c.push(m, 0x63d0be);
            { let args = []; c.r[0] = host.call(m, 0x644c81, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cb100u32,32));
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m, 0x63d0cf);
            { let args = []; c.r[0] = host.call(m, 0x63d222, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x63d111; continue; }
            pc = 0x63d0d4; continue;
            }
            0x63d0d4 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x20),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x63d0e3; continue; }
            pc = 0x63d0db; continue;
            }
            0x63d0db => {
            if c.cond(13) { pc = 0x63d105; continue; }
            pc = 0x63d0dd; continue;
            }
            0x63d0dd => {
            { let a = rm(m,c.r[0].wrapping_add(0x20),32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x63d105; continue; }
            pc = 0x63d0e3; continue;
            }
            0x63d0e3 => {
            c.set(0,32,rm(m,0x6cb108u32,32));
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m, 0x63d0f4);
            { let args = []; c.r[0] = host.call(m, 0x63d222, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63d105; continue;
            }
            0x63d0f7 => {
            c.set(0,32,c.r[5].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m, 0x63d100);
            { let args = []; c.r[0] = host.call(m, 0x63d118, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x63d111; continue; }
            pc = 0x63d105; continue;
            }
            0x63d105 => {
            c.push(m,0x9u32);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(1,32,v); }
            c.rep_movsd(m);
            pc = 0x63d114; continue;
            }
            0x63d111 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x63d114; continue;
            }
            0x63d114 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006455ec(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6455ec;
    loop {
        match pc {
            0x6455ec => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[0].wrapping_add(0xffffffff));
            c.push(m,0x20u32);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(3,32,c.r[7].wrapping_add(0x1));
            c.push(m,0x20u32);
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.push(m,0x1fu32);
            c.set(1,32,c.get(0,32));
            c.set(0,32,c.get(3,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.push(m,0x1u32);
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            { let v = c.pop(m); c.set(2,32,v); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = rm(m,c.r[0],32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x645650; continue; }
            pc = 0x64562f; continue;
            }
            0x64562f => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x645639);
            { let args = []; c.r[0] = host.call(m, 0x64554d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x64564d; continue; }
            pc = 0x64563f; continue;
            }
            0x64563f => {
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x645648);
            { let args = []; c.r[0] = host.call(m, 0x645596, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x64564d; continue;
            }
            0x64564d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            pc = 0x645650; continue;
            }
            0x645650 => {
            { let a = c.get(2,32); let r = c.or(a,0xffffffffu32,32); c.set(2,32,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,0x3u32);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = rm(m,c.r[0],32); let r = c.and(a,c.get(2,32),32); { let a = c.r[0]; wm(m,a,32,r); } }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x645670; continue; }
            pc = 0x645664; continue;
            }
            0x645664 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.stos(m,32,true);
            pc = 0x645670; continue;
            }
            0x645670 => {
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

pub fn f_00645678(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x645678;
    loop {
        match pc {
            0x645678 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.push(m,0x3u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(2,32,v); }
            pc = 0x645686; continue;
            }
            0x645686 => {
            c.set(6,32,rm(m,c.r[0],32));
            { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x645686; continue; }
            pc = 0x645691; continue;
            }
            0x645691 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00645693(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x645693;
    loop {
        match pc {
            0x645693 => {
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.stos(m,32,false);
            c.stos(m,32,false);
            c.stos(m,32,false);
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064569f(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64569f;
    loop {
        match pc {
            0x64569f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x6456a5; continue;
            }
            0x6456a5 => {
            { let a = rm(m,c.r[0],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x6456b7; continue; }
            pc = 0x6456aa; continue;
            }
            0x6456aa => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x6456a5; continue; }
            pc = 0x6456b3; continue;
            }
            0x6456b3 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x6456b7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006456ba(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6456ba;
    loop {
        match pc {
            0x6456ba => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x20u32);
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(6,32); let r = c.or(a,0xffffffffu32,32); c.set(6,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,c.get(3,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x3u32); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,r); } }
            c.set(1,32,c.get(2,32));
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); c.set(6,32,!a & 0xffffffffu32); }
            pc = 0x6456f0; continue;
            }
            0x6456f0 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,rm(m,c.r[5].wrapping_add(0xc),32),32); c.set(0,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            c.set(1,32,c.get(3,32));
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x6456f0; continue; }
            pc = 0x645714; continue;
            }
            0x645714 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.push(m,0x2u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(6,32,c.get(7,32));
            c.push(m,0x8u32);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            pc = 0x645722; continue;
            }
            0x645722 => {
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x645735; continue; }
            pc = 0x645726; continue;
            }
            0x645726 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2]),32));
            { let a = c.r[1].wrapping_add(c.r[2]); wm(m,a,32,c.get(0,32)); }
            pc = 0x64573c; continue;
            }
            0x645735 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]),32); let r = c.and(a,0x0u32,32); { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,32,r); } }
            pc = 0x64573c; continue;
            }
            0x64573c => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,0x4u32,32); c.set(1,32,r); }
            if c.cond(9) { pc = 0x645722; continue; }
            pc = 0x645742; continue;
            }
            0x645742 => {
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

pub fn f_006484d3(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6484d3;
    loop {
        match pc {
            0x6484d3 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[7],32); c.push(m, v); }
            { let v = rm(m,c.r[6],32); c.push(m, v); }
            c.push(m, 0x6484e7);
            { let args = []; c.r[0] = host.call(m, 0x6484b2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x648505; continue; }
            pc = 0x6484ee; continue;
            }
            0x6484ee => {
            c.set(0,32,c.r[6].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            { let v = rm(m,c.r[0],32); c.push(m, v); }
            c.push(m, 0x6484fb);
            { let args = []; c.r[0] = host.call(m, 0x6484b2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x648505; continue; }
            pc = 0x648502; continue;
            }
            0x648502 => {
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); let r = c.inc(a,32); { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,r); } }
            pc = 0x648505; continue;
            }
            0x648505 => {
            c.set(0,32,c.r[6].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[7].wrapping_add(0x4),32); c.push(m, v); }
            { let v = rm(m,c.r[0],32); c.push(m, v); }
            c.push(m, 0x648513);
            { let args = []; c.r[0] = host.call(m, 0x6484b2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64851d; continue; }
            pc = 0x64851a; continue;
            }
            0x64851a => {
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); let r = c.inc(a,32); { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,r); } }
            pc = 0x64851d; continue;
            }
            0x64851d => {
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[7].wrapping_add(0x8),32); c.push(m, v); }
            { let v = rm(m,c.r[0],32); c.push(m, v); }
            c.push(m, 0x64852b);
            { let args = []; c.r[0] = host.call(m, 0x6484b2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00648531(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x648531;
    loop {
        match pc {
            0x648531 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,rm(m,c.r[0],32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(6,32); let r = c.add(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            c.set(6,32,c.r[7].wrapping_add(c.r[7]));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.or(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(2,32,c.get(7,32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(2,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,c.get(2,32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006488e6(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6488e6;
    loop {
        match pc {
            0x6488e6 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,16,rm(m,c.r[3].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(7,32,c.get(1,32));
            c.set(2,32,0x7fffu32);
            { let a = c.get(7,32); let r = c.xor(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0x8000u32,32); c.set(7,32,r); }
            { let a = c.get(0,16); c.sub(a,0x7fffu32,16); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0]));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            if c.cond(3) { pc = 0x648ae6; continue; }
            pc = 0x64892e; continue;
            }
            0x64892e => {
            { let a = c.get(1,16); c.sub(a,0x7fffu32,16); }
            if c.cond(3) { pc = 0x648ae6; continue; }
            pc = 0x648939; continue;
            }
            0x648939 => {
            { let a = c.get(2,16); c.sub(a,0xbffdu32,16); }
            if c.cond(7) { pc = 0x648ae6; continue; }
            pc = 0x648944; continue;
            }
            0x648944 => {
            { let a = c.get(2,16); c.sub(a,0x3fbfu32,16); }
            if c.cond(7) { pc = 0x64894f; continue; }
            pc = 0x64894b; continue;
            }
            0x64894b => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x648989; continue;
            }
            0x64894f => {
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            c.set(2,32,0x7fffffffu32);
            if c.cond(5) { pc = 0x648971; continue; }
            pc = 0x648959; continue;
            }
            0x648959 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x648971; continue; }
            pc = 0x648961; continue;
            }
            0x648961 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x648973; continue; }
            pc = 0x648968; continue;
            }
            0x648968 => {
            { let a = rm(m,c.r[6],32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x648973; continue; }
            pc = 0x64896c; continue;
            }
            0x64896c => {
            pc = 0x648ae0; continue;
            }
            0x648971 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x648973; continue;
            }
            0x648973 => {
            { let a = c.get(1,16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x648996; continue; }
            pc = 0x648978; continue;
            }
            0x648978 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[3].wrapping_add(0x8),32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x648996; continue; }
            pc = 0x648980; continue;
            }
            0x648980 => {
            { let a = rm(m,c.r[3].wrapping_add(0x4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x648996; continue; }
            pc = 0x648985; continue;
            }
            0x648985 => {
            { let a = rm(m,c.r[3],32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x648996; continue; }
            pc = 0x648989; continue;
            }
            0x648989 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x648b01; continue;
            }
            0x648996 => {
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,0x5u32); }
            pc = 0x6489a6; continue;
            }
            0x6489a6 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff0),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x6489fa; continue; }
            pc = 0x6489b1; continue;
            }
            0x6489b1 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[3].wrapping_add(0x8));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            pc = 0x6489c2; continue;
            }
            0x6489c2 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(0,32,rm(m,c.r[0],16));
            c.set(1,32,rm(m,c.r[1],16));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(1,32); let r = c.add(a,0xfffffffcu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[1],32); c.push(m, v); }
            c.push(m, 0x6489e0);
            { let args = []; c.r[0] = host.call(m, 0x6484b2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6489ed; continue; }
            pc = 0x6489e7; continue;
            }
            0x6489e7 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = rm(m,c.r[0],16); let r = c.inc(a,16); { let a = c.r[0]; wm(m,a,16,r); } }
            pc = 0x6489ed; continue;
            }
            0x6489ed => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.add(a,0x2u32,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff4),32); let r = c.sub(a,0x2u32,32); { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe8),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,r); } }
            if c.cond(5) { pc = 0x6489c2; continue; }
            pc = 0x6489fa; continue;
            }
            0x6489fa => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.add(a,0x2u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff0),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,0x0u32,32); }
            if c.cond(15) { pc = 0x6489a6; continue; }
            pc = 0x648a0a; continue;
            }
            0x648a0a => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.add(a,0xc002u32,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(14) { pc = 0x648a3d; continue; }
            pc = 0x648a18; continue;
            }
            0x648a18 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe7),8); c.and(a,0x80u32,8); }
            if c.cond(5) { pc = 0x648a36; continue; }
            pc = 0x648a1e; continue;
            }
            0x648a1e => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffdc));
            c.push(m,c.get(0,32));
            c.push(m, 0x648a27);
            { let args = []; c.r[0] = host.call(m, 0x648531, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.add(a,0xffffu32,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = rm(m,c.r[5].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(15) { pc = 0x648a18; continue; }
            pc = 0x648a36; continue;
            }
            0x648a36 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(15) { pc = 0x648a76; continue; }
            pc = 0x648a3d; continue;
            }
            0x648a3d => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.add(a,0xffffu32,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(13) { pc = 0x648a76; continue; }
            pc = 0x648a4b; continue;
            }
            0x648a4b => {
            c.set(0,32,sx(rm(m,c.r[5].wrapping_add(0x8),16),16)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            c.set(3,32,c.get(0,32));
            pc = 0x648a56; continue;
            }
            0x648a56 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffdc),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x648a5f; continue; }
            pc = 0x648a5c; continue;
            }
            0x648a5c => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffec),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,r); } }
            pc = 0x648a5f; continue;
            }
            0x648a5f => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffdc));
            c.push(m,c.get(0,32));
            c.push(m, 0x648a68);
            { let args = []; c.r[0] = host.call(m, 0x64855f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x648a56; continue; }
            pc = 0x648a6c; continue;
            }
            0x648a6c => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffec),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x648a76; continue; }
            pc = 0x648a72; continue;
            }
            0x648a72 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffdc),8); let r = c.or(a,0x1u32,8); { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,8,r); } }
            pc = 0x648a76; continue;
            }
            0x648a76 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffdc),16); c.sub(a,0x8000u32,16); }
            if c.cond(7) { pc = 0x648a8d; continue; }
            pc = 0x648a7e; continue;
            }
            0x648a7e => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffdc),32));
            { let a = c.get(0,32); let r = c.and(a,0x1ffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18000u32,32); }
            if c.cond(5) { pc = 0x648ac2; continue; }
            pc = 0x648a8d; continue;
            }
            0x648a8d => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffde),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x648abf; continue; }
            pc = 0x648a93; continue;
            }
            0x648a93 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffde),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe2),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x648aba; continue; }
            pc = 0x648a9d; continue;
            }
            0x648a9d => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe2),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe6),16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x648ab4; continue; }
            pc = 0x648aa9; continue;
            }
            0x648aa9 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = c.r[5].wrapping_add(0xffffffe6); wm(m,a,16,0x8000u32); }
            pc = 0x648ac2; continue;
            }
            0x648ab4 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe6),16); let r = c.inc(a,16); { let a = c.r[5].wrapping_add(0xffffffe6); wm(m,a,16,r); } }
            pc = 0x648ac2; continue;
            }
            0x648aba => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe2),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xffffffe2); wm(m,a,32,r); } }
            pc = 0x648ac2; continue;
            }
            0x648abf => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffde),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xffffffde); wm(m,a,32,r); } }
            pc = 0x648ac2; continue;
            }
            0x648ac2 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,16); c.sub(a,0x7fffu32,16); }
            if c.cond(3) { pc = 0x648ae6; continue; }
            pc = 0x648acb; continue;
            }
            0x648acb => {
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xffffffde),16));
            { let a = c.get(0,32); let r = c.or(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.r[6].wrapping_add(0x2); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,32,c.get(1,32)); }
            pc = 0x648ae0; continue;
            }
            0x648ae0 => {
            { let a = c.r[6].wrapping_add(0xa); wm(m,a,16,c.get(0,16)); }
            pc = 0x648b01; continue;
            }
            0x648ae6 => {
            { let a = c.get(7,16); let r = c.neg(a,16); c.set(7,16,r); }
            { let a = c.get(7,32); let r = c.sbb(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            { let a = c.get(7,32); let r = c.and(a,0x80000000u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x7fff8000u32,32); c.set(7,32,r); }
            { let a = rm(m,c.r[6],32); let r = c.and(a,0x0u32,32); { let a = c.r[6]; wm(m,a,32,r); } }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            pc = 0x648b01; continue;
            }
            0x648b01 => {
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

pub fn f_0064554d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64554d;
    loop {
        match pc {
            0x64554d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            c.push(m,0x20u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(1,32,v); }
            { let b = c.get(1,32); c.idiv32(b); }
            c.push(m,0x1fu32);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.or(a,0xffffffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.set(2,32,!a & 0xffffffffu32); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)),32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x645592; continue; }
            pc = 0x645576; continue;
            }
            0x645576 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x3u32,32); }
            if c.cond(13) { pc = 0x64558d; continue; }
            pc = 0x64557c; continue;
            }
            0x64557c => {
            c.set(0,32,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)));
            pc = 0x64557f; continue;
            }
            0x64557f => {
            { let a = rm(m,c.r[0],32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x645592; continue; }
            pc = 0x645584; continue;
            }
            0x645584 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x64557f; continue; }
            pc = 0x64558d; continue;
            }
            0x64558d => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x645592 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00645596(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x645596;
    loop {
        match pc {
            0x645596 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x20u32);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(1,32,v); }
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(7,32,c.r[3].wrapping_add(c.r[6].wrapping_mul(4)));
            c.push(m,c.get(7,32));
            c.push(m,0x1fu32);
            { let v = c.pop(m); c.set(1,32,v); }
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[7],32); c.push(m, v); }
            c.push(m, 0x6455c6);
            { let args = []; c.r[0] = host.call(m, 0x6484b2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            if c.cond(8) { pc = 0x6455e8; continue; }
            pc = 0x6455cc; continue;
            }
            0x6455cc => {
            c.set(7,32,c.r[3].wrapping_add(c.r[6].wrapping_mul(4)));
            pc = 0x6455cf; continue;
            }
            0x6455cf => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x6455e8; continue; }
            pc = 0x6455d3; continue;
            }
            0x6455d3 => {
            c.push(m,c.get(7,32));
            c.push(m,0x1u32);
            { let v = rm(m,c.r[7],32); c.push(m, v); }
            c.push(m, 0x6455dd);
            { let args = []; c.r[0] = host.call(m, 0x6484b2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x6455cf; continue; }
            pc = 0x6455e8; continue;
            }
            0x6455e8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006484b2(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6484b2;
    loop {
        match pc {
            0x6484b2 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[6]));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x6484c8; continue; }
            pc = 0x6484c4; continue;
            }
            0x6484c4 => {
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x6484cb; continue; }
            pc = 0x6484c8; continue;
            }
            0x6484c8 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x6484cb; continue;
            }
            0x6484cb => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064855f(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64855f;
    loop {
        match pc {
            0x64855f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(6,32,c.get(2,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(6,32); let n = 0x1fu32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(7,32); let n = 0x1fu32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.or(a,c.get(7,32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006505b2(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6505b2;
    loop {
        match pc {
            0x6505b2 => {
            c.push(m,0x0u32);
            c.push(m, 0x6505b9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x650519, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x6505cd; continue; }
            pc = 0x6505c1; continue;
            }
            0x6505c1 => {
            { let v = rm(m,c.r[4].wrapping_add(0x4),32); c.push(m, v); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m, 0x6505cd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ee91, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x6505cd; continue;
            }
            0x6505cd => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
