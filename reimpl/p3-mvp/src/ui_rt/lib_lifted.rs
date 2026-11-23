//! Generado por `reimpl/tools/ui_rt_lib.py` con `tools/lift.py` (no editar a mano): CRT/MFC del EXE que necesita el
//! enlazador de interfaz. Rutinas de biblioteca, sin prueba de paridad propia.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static LIB: [(u32, super::core_table::CoreFn); 108] = [
    (0x639710, f_00639710),
    (0x63978e, f_0063978e),
    (0x639da8, f_00639da8),
    (0x639e10, f_00639e10),
    (0x639e9c, f_00639e9c),
    (0x639f06, f_00639f06),
    (0x63a6a0, f_0063a6a0),
    (0x63ab8a, f_0063ab8a),
    (0x63ac5b, f_0063ac5b),
    (0x63b1f5, f_0063b1f5),
    (0x63b48c, f_0063b48c),
    (0x63b637, f_0063b637),
    (0x63b6a9, f_0063b6a9),
    (0x63b6fa, f_0063b6fa),
    (0x63b717, f_0063b717),
    (0x63b806, f_0063b806),
    (0x63b895, f_0063b895),
    (0x63b94f, f_0063b94f),
    (0x63b9e6, f_0063b9e6),
    (0x63be9d, f_0063be9d),
    (0x63bece, f_0063bece),
    (0x63cab7, f_0063cab7),
    (0x63ccb5, f_0063ccb5),
    (0x63cd28, f_0063cd28),
    (0x63cd31, f_0063cd31),
    (0x63e280, f_0063e280),
    (0x63e289, f_0063e289),
    (0x63eda0, f_0063eda0),
    (0x63edb3, f_0063edb3),
    (0x63f410, f_0063f410),
    (0x641bab, f_00641bab),
    (0x64211c, f_0064211c),
    (0x642234, f_00642234),
    (0x642975, f_00642975),
    (0x6429aa, f_006429aa),
    (0x6429db, f_006429db),
    (0x642a13, f_00642a13),
    (0x642a20, f_00642a20),
    (0x643488, f_00643488),
    (0x6435a6, f_006435a6),
    (0x643945, f_00643945),
    (0x645da9, f_00645da9),
    (0x645e0e, f_00645e0e),
    (0x645e81, f_00645e81),
    (0x645ee6, f_00645ee6),
    (0x646071, f_00646071),
    (0x6460b5, f_006460b5),
    (0x6460de, f_006460de),
    (0x646b18, f_00646b18),
    (0x646b5a, f_00646b5a),
    (0x646bb9, f_00646bb9),
    (0x64b52e, f_0064b52e),
    (0x64b5ca, f_0064b5ca),
    (0x64cd70, f_0064cd70),
    (0x64ce30, f_0064ce30),
    (0x64d180, f_0064d180),
    (0x64d360, f_0064d360),
    (0x64d390, f_0064d390),
    (0x64d410, f_0064d410),
    (0x64e1e0, f_0064e1e0),
    (0x64e2f0, f_0064e2f0),
    (0x64e603, f_0064e603),
    (0x64e642, f_0064e642),
    (0x64e68b, f_0064e68b),
    (0x64e6e3, f_0064e6e3),
    (0x64e6fc, f_0064e6fc),
    (0x64e749, f_0064e749),
    (0x64e780, f_0064e780),
    (0x64e7a6, f_0064e7a6),
    (0x64e7c9, f_0064e7c9),
    (0x64e83a, f_0064e83a),
    (0x64e85c, f_0064e85c),
    (0x64e979, f_0064e979),
    (0x64e9a5, f_0064e9a5),
    (0x64ea47, f_0064ea47),
    (0x64eae9, f_0064eae9),
    (0x64eb86, f_0064eb86),
    (0x64ebf1, f_0064ebf1),
    (0x64ec25, f_0064ec25),
    (0x64ece7, f_0064ece7),
    (0x64ed07, f_0064ed07),
    (0x64ee91, f_0064ee91),
    (0x64efc8, f_0064efc8),
    (0x64f0c0, f_0064f0c0),
    (0x64f18a, f_0064f18a),
    (0x64f1bb, f_0064f1bb),
    (0x64f1de, f_0064f1de),
    (0x64f1fc, f_0064f1fc),
    (0x64f22a, f_0064f22a),
    (0x64f27d, f_0064f27d),
    (0x64f313, f_0064f313),
    (0x64f340, f_0064f340),
    (0x64f3f8, f_0064f3f8),
    (0x64f584, f_0064f584),
    (0x64f60a, f_0064f60a),
    (0x64f6ce, f_0064f6ce),
    (0x64f6dc, f_0064f6dc),
    (0x64f709, f_0064f709),
    (0x650184, f_00650184),
    (0x65058b, f_0065058b),
    (0x652db8, f_00652db8),
    (0x6534f6, f_006534f6),
    (0x653517, f_00653517),
    (0x653688, f_00653688),
    (0x65388d, f_0065388d),
    (0x653b0b, f_00653b0b),
    (0x653c1d, f_00653c1d),
    (0x6592c2, f_006592c2),
];
pub fn f_00639710(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x639710;
    loop {
        match pc {
            0x639710 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x639716);
            { let args = []; c.r[0] = host.call(m, 0x63e280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,0x6f1a94u32,32); c.push(m, v); }
            c.push(m, 0x639721);
            { let args = []; c.r[0] = host.call(m, 0x63e410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6f1a94u32,32));
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,rm(m,0x6f1a90u32,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x639776; continue; }
            pc = 0x639739; continue;
            }
            0x639739 => {
            c.push(m,c.get(2,32));
            c.push(m, 0x63973f);
            { let args = []; c.r[0] = host.call(m, 0x63e410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0x10u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let v = rm(m,0x6f1a94u32,32); c.push(m, v); }
            c.push(m, 0x63974e);
            { let args = []; c.r[0] = host.call(m, 0x63a29f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x639759; continue; }
            pc = 0x639755; continue;
            }
            0x639755 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x639785; continue;
            }
            0x639759 => {
            c.set(1,32,rm(m,0x6f1a90u32,32));
            { let a = c.get(1,32); let r = c.sub(a,rm(m,0x6f1a94u32,32),32); c.set(1,32,r); }
            { let a = 0x6f1a94u32; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = 0x6f1a90u32; wm(m,a,32,c.get(1,32)); }
            pc = 0x639776; continue;
            }
            0x639776 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,0x6f1a90u32,32); let r = c.add(a,0x4u32,32); { let a = 0x6f1a90u32; wm(m,a,32,r); } }
            c.set(6,32,c.get(0,32));
            pc = 0x639785; continue;
            }
            0x639785 => {
            c.push(m, 0x63978a);
            { let args = []; c.r[0] = host.call(m, 0x63e289, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063978e(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63978e;
    loop {
        match pc {
            0x63978e => {
            { let v = rm(m,c.r[4].wrapping_add(0x4),32); c.push(m, v); }
            c.push(m, 0x639797);
            { let args = []; c.r[0] = host.call(m, 0x639710, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00639da8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x639da8;
    loop {
        match pc {
            0x639da8 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x67d200u32);
            c.push(m,0x63ff40u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe4),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,r); } }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[5].wrapping_add(0x10),32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x639ddf; continue;
            }
            0x639ddf => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            if c.cond(8) { pc = 0x639def; continue; }
            pc = 0x639de4; continue;
            }
            0x639de4 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); let r = c.sub(a,c.get(6,32),32); { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,r); } }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let target = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, 0x639ded); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x639ddf; continue;
            }
            0x639def => {
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,0x1u32); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m, 0x639dff);
            { let args = []; c.r[0] = host.call(m, 0x639e10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff0),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
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

pub fn f_00639e10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x639e10;
    loop {
        match pc {
            0x639e10 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe4),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x639e27; continue; }
            pc = 0x639e16; continue;
            }
            0x639e16 => {
            { let v = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x639e27);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639e28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x639e27; continue;
            }
            0x639e27 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00639e9c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x639e9c;
    loop {
        match pc {
            0x639e9c => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x67d220u32);
            c.push(m,0x63ff40u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            pc = 0x639eca; continue;
            }
            0x639eca => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x10),32),32); }
            if c.cond(13) { pc = 0x639ee5; continue; }
            pc = 0x639ed2; continue;
            }
            0x639ed2 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,c.get(6,32));
            { let target = rm(m,c.r[5].wrapping_add(0x14),32); c.push(m, 0x639eda); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0xc),32),32); c.set(6,32,r); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe4),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,r); } }
            pc = 0x639eca; continue;
            }
            0x639ee5 => {
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,0x1u32); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.push(m, 0x639ef5);
            { let args = []; c.r[0] = host.call(m, 0x639f06, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff0),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00639f06(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x639f06;
    loop {
        match pc {
            0x639f06 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe0),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x639f1d; continue; }
            pc = 0x639f0c; continue;
            }
            0x639f0c => {
            { let v = rm(m,c.r[5].wrapping_add(0x18),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xffffffe4),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x639f1d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639e28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x639f1d; continue;
            }
            0x639f1d => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063a6a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63a6a0;
    loop {
        match pc {
            0x63a6a0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.or(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            if c.cond(5) { pc = 0x63a6b9; continue; }
            pc = 0x63a6b0; continue;
            }
            0x63a6b0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x63a6b9 => {
            c.push(m,c.get(3,32));
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let b = rm(m,c.r[4].wrapping_add(0x14),32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let b = c.get(1,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063ab8a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63ab8a;
    loop {
        match pc {
            0x63ab8a => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,0x42u32); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x63abb7);
            { let args = []; c.r[0] = host.call(m, 0x642234, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe4),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,r); } }
            c.set(6,32,c.get(0,32));
            if c.cond(8) { pc = 0x63abc9; continue; }
            pc = 0x63abc1; continue;
            }
            0x63abc1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = rm(m,c.r[0],8); let r = c.and(a,0x0u32,8); { let a = c.r[0]; wm(m,a,8,r); } }
            pc = 0x63abd6; continue;
            }
            0x63abc9 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x63abd4);
            { let args = []; c.r[0] = host.call(m, 0x64211c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63abd6; continue;
            }
            0x63abd6 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063ac5b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63ac5b;
    loop {
        match pc {
            0x63ac5b => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = rm(m,0x6f1abcu32,32); c.sub(a,0x0u32,32); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            if c.cond(5) { pc = 0x63ac7c; continue; }
            pc = 0x63ac6a; continue;
            }
            0x63ac6a => {
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x63ac75);
            { let args = []; c.r[0] = host.call(m, 0x639cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63ad05; continue;
            }
            0x63ac7c => {
            c.push(m,0x19u32);
            c.push(m, 0x63ac83);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63ac8a; continue;
            }
            0x63ac8a => {
            c.set(1,16,rm(m,c.r[7],8));
            c.set(0,32,c.get(1,8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = rm(m,c.r[0].wrapping_add(0x6f1bc1),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x63acb4; continue; }
            pc = 0x63ac9e; continue;
            }
            0x63ac9e => {
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x63acaa; continue; }
            pc = 0x63aca4; continue;
            }
            0x63aca4 => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x63acb4; continue;
            }
            0x63acaa => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(6,8,c.get(1,8));
            c.set(2,8,c.get(0,8));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x63acb4; continue;
            }
            0x63acb4 => {
            c.set(3,16,rm(m,c.r[6],8));
            c.set(0,32,c.get(3,8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x6f1bc1),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x63acd8; continue; }
            pc = 0x63acc5; continue;
            }
            0x63acc5 => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x63accf; continue; }
            pc = 0x63accb; continue;
            }
            0x63accb => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x63acd8; continue;
            }
            0x63accf => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(5,8,c.get(3,8));
            c.set(1,8,c.get(0,8));
            c.set(3,32,c.get(1,32));
            pc = 0x63acd8; continue;
            }
            0x63acd8 => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),16); c.sub(a,c.get(3,16),16); }
            if c.cond(5) { pc = 0x63ace7; continue; }
            pc = 0x63acde; continue;
            }
            0x63acde => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x63acfb; continue; }
            pc = 0x63ace5; continue;
            }
            0x63ace5 => {
            pc = 0x63ac8a; continue;
            }
            0x63ace7 => {
            c.push(m,0x19u32);
            c.push(m, 0x63acee);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[5].wrapping_add(0xc),16),16); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            pc = 0x63ad05; continue;
            }
            0x63acfb => {
            c.push(m,0x19u32);
            c.push(m, 0x63ad02);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63ad05; continue;
            }
            0x63ad05 => {
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

pub fn f_0063b1f5(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63b1f5;
    loop {
        match pc {
            0x63b1f5 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,0xffffu32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m,0x133fu32);
            c.push(m, 0x63b20c);
            { let args = []; c.r[0] = host.call(m, 0x641ce3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xe),32));
            { let a = c.get(0,16); let r = c.and(a,0x7ff0u32,16); c.set(0,16,r); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,16); c.sub(a,0x7ff0u32,16); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x63b289; continue; }
            pc = 0x63b21d; continue;
            }
            0x63b21d => {
            c.fld(c.ld(m,c.r[5].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.fstp64(m,c.r[4]);
            c.push(m, 0x63b22a);
            { let args = []; c.r[0] = host.call(m, 0x641bab, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x63b27b; continue; }
            pc = 0x63b22f; continue;
            }
            0x63b22f => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x63b26b; continue; }
            pc = 0x63b232; continue;
            }
            0x63b232 => {
            c.fld(c.ld(m,c.r[5].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x63b25a; continue; }
            pc = 0x63b239; continue;
            }
            0x63b239 => {
            { let v = c.ld(m,0x67d260u32,8); c.fop('+',v); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.fstp64(m,c.r[4]);
            c.fld(c.ld(m,c.r[5].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.fstp64(m,c.r[4]);
            c.push(m,0x15u32);
            c.push(m,0x8u32);
            c.push(m, 0x63b255);
            { let args = []; c.r[0] = host.call(m, 0x64150c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            pc = 0x63b2a6; continue;
            }
            0x63b25a => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.fstp64(m,c.r[4]);
            c.push(m,0x15u32);
            c.push(m, 0x63b266);
            { let args = []; c.r[0] = host.call(m, 0x6414b8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x63b2a6; continue;
            }
            0x63b26b => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x63b272);
            { let args = []; c.r[0] = host.call(m, 0x641ce3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fld(c.ld(m,c.r[5].wrapping_add(0x8),8));
            { let v = c.pop(m); c.set(1,32,v); }
            c.fchs();
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63b2a6; continue;
            }
            0x63b27b => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x63b282);
            { let args = []; c.r[0] = host.call(m, 0x641ce3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fld(c.ld(m,c.r[5].wrapping_add(0x8),8));
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63b2a6; continue;
            }
            0x63b289 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0x7fffffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x63b2a1);
            { let args = []; c.r[0] = host.call(m, 0x641ce3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.fld(c.ld(m,c.r[5].wrapping_add(0xfffffff8),8));
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63b2a6; continue;
            }
            0x63b2a6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063b48c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63b48c;
    loop {
        match pc {
            0x63b48c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0],8));
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x6f1bc1),8));
            { let a = c.get(1,8); let r = c.and(a,0x4u32,8); c.set(1,8,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x63b4a2; continue; }
            pc = 0x63b4a1; continue;
            }
            0x63b4a1 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x63b4a2; continue;
            }
            0x63b4a2 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063b637(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63b637;
    loop {
        match pc {
            0x63b637 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = rm(m,0x6f1abcu32,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x63b654; continue; }
            pc = 0x63b645; continue;
            }
            0x63b645 => {
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x63b650);
            { let args = []; c.r[0] = host.call(m, 0x63c920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63b6a6; continue;
            }
            0x63b654 => {
            c.push(m,c.get(7,32));
            c.push(m,0x19u32);
            c.push(m, 0x63b65c);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            pc = 0x63b660; continue;
            }
            0x63b660 => {
            c.set(2,8,rm(m,c.r[1],8));
            c.set(0,32,c.get(2,8));
            c.set(7,32,c.get(0,8));
            { let a = rm(m,c.r[7].wrapping_add(0x6f1bc1),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x63b68f; continue; }
            pc = 0x63b671; continue;
            }
            0x63b671 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x1),8));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x63b68b; continue; }
            pc = 0x63b679; continue;
            }
            0x63b679 => {
            c.set(7,32,c.get(2,8));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x63b696; continue; }
            pc = 0x63b686; continue;
            }
            0x63b686 => {
            c.set(6,32,c.r[1].wrapping_add(0xffffffff));
            pc = 0x63b696; continue;
            }
            0x63b68b => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            pc = 0x63b692; continue;
            }
            0x63b68f => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,c.get(0,32),32); }
            pc = 0x63b692; continue;
            }
            0x63b692 => {
            if c.cond(5) { pc = 0x63b696; continue; }
            pc = 0x63b694; continue;
            }
            0x63b694 => {
            c.set(6,32,c.get(1,32));
            pc = 0x63b696; continue;
            }
            0x63b696 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x63b660; continue; }
            pc = 0x63b69b; continue;
            }
            0x63b69b => {
            c.push(m,0x19u32);
            c.push(m, 0x63b6a2);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x63b6a6; continue;
            }
            0x63b6a6 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063b6a9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63b6a9;
    loop {
        match pc {
            0x63b6a9 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,0x42u32); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,0x7fffffffu32); }
            c.push(m,c.get(0,32));
            c.push(m, 0x63b6d6);
            { let args = []; c.r[0] = host.call(m, 0x642234, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe4),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,r); } }
            c.set(6,32,c.get(0,32));
            if c.cond(8) { pc = 0x63b6e8; continue; }
            pc = 0x63b6e0; continue;
            }
            0x63b6e0 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = rm(m,c.r[0],8); let r = c.and(a,0x0u32,8); { let a = c.r[0]; wm(m,a,8,r); } }
            pc = 0x63b6f5; continue;
            }
            0x63b6e8 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x63b6f3);
            { let args = []; c.r[0] = host.call(m, 0x64211c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63b6f5; continue;
            }
            0x63b6f5 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063b6fa(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63b6fa;
    loop {
        match pc {
            0x63b6fa => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = rm(m,c.r[1],16); c.sub(a,0x0u32,16); }
            c.set(0,32,c.r[1].wrapping_add(0x2));
            if c.cond(4) { pc = 0x63b711; continue; }
            pc = 0x63b707; continue;
            }
            0x63b707 => {
            c.set(2,16,rm(m,c.r[0],16));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,16); c.and(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x63b707; continue; }
            pc = 0x63b711; continue;
            }
            0x63b711 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063b717(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63b717;
    loop {
        match pc {
            0x63b717 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,c.get(3,32),32); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x63b7ff; continue; }
            pc = 0x63b728; continue;
            }
            0x63b728 => {
            { let a = rm(m,0x6f1abcu32,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x63b746; continue; }
            pc = 0x63b730; continue;
            }
            0x63b730 => {
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x63b73e);
            { let args = []; c.r[0] = host.call(m, 0x642b30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x63b801; continue;
            }
            0x63b746 => {
            c.push(m,0x19u32);
            c.push(m, 0x63b74d);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63b757; continue;
            }
            0x63b757 => {
            c.set(1,16,rm(m,c.r[7],8));
            c.set(0,32,c.get(1,8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = rm(m,c.r[0].wrapping_add(0x6f1bc1),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x63b798; continue; }
            pc = 0x63b76b; continue;
            }
            0x63b76b => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x63b783; continue; }
            pc = 0x63b770; continue;
            }
            0x63b770 => {
            c.set(0,32,rm(m,c.r[6],8));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = rm(m,c.r[0].wrapping_add(0x6f1bc1),8); c.and(a,0x4u32,8); }
            if c.cond(5) { pc = 0x63b7f7; continue; }
            pc = 0x63b77f; continue;
            }
            0x63b77f => {
            c.set(3,32,c.get(0,32));
            pc = 0x63b7c5; continue;
            }
            0x63b783 => {
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x63b78e; continue; }
            pc = 0x63b789; continue;
            }
            0x63b789 => {
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            pc = 0x63b798; continue;
            }
            0x63b78e => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(6,8,c.get(1,8));
            c.set(2,8,c.get(0,8));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x63b798; continue;
            }
            0x63b798 => {
            c.set(3,16,rm(m,c.r[6],8));
            c.set(0,32,c.get(3,8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x6f1bc1),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x63b7c5; continue; }
            pc = 0x63b7a9; continue;
            }
            0x63b7a9 => {
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x63b7b8; continue; }
            pc = 0x63b7af; continue;
            }
            0x63b7af => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x63b7bc; continue; }
            pc = 0x63b7b8; continue;
            }
            0x63b7b8 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x63b7c5; continue;
            }
            0x63b7bc => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(5,8,c.get(3,8));
            c.set(1,8,c.get(0,8));
            c.set(3,32,c.get(1,32));
            pc = 0x63b7c5; continue;
            }
            0x63b7c5 => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),16); c.sub(a,c.get(3,16),16); }
            if c.cond(5) { pc = 0x63b7e3; continue; }
            pc = 0x63b7cb; continue;
            }
            0x63b7cb => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x63b7f7; continue; }
            pc = 0x63b7d2; continue;
            }
            0x63b7d2 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,r); } }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63b7f7; continue; }
            pc = 0x63b7dc; continue;
            }
            0x63b7dc => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x63b757; continue;
            }
            0x63b7e3 => {
            c.push(m,0x19u32);
            c.push(m, 0x63b7ea);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[5].wrapping_add(0xc),16),16); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            pc = 0x63b801; continue;
            }
            0x63b7f7 => {
            c.push(m,0x19u32);
            c.push(m, 0x63b7fe);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63b7ff; continue;
            }
            0x63b7ff => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63b801; continue;
            }
            0x63b801 => {
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

pub fn f_0063b806(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63b806;
    loop {
        match pc {
            0x63b806 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.sub(a,0xffu32,32); }
            if c.cond(6) { pc = 0x63b873; continue; }
            pc = 0x63b815; continue;
            }
            0x63b815 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),16); let r = c.and(a,0x0u32,16); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,16,r); } }
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0xfffffffe));
            c.stos(m,16,false);
            c.set(0,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xb); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = rm(m,0x6f1abcu32,32); c.sub(a,0x0u32,32); }
            { let a = c.r[5].wrapping_add(0xa); wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(5) { pc = 0x63b83b; continue; }
            pc = 0x63b837; continue;
            }
            0x63b837 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63b83b => {
            c.push(m,0x1u32);
            c.set(0,32,c.r[5].wrapping_add(0xfffffffc));
            { let v = rm(m,0x6f1cc4u32,32); c.push(m, v); }
            { let v = rm(m,0x6f1aa0u32,32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xa));
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m, 0x63b85a);
            { let args = []; c.r[0] = host.call(m, 0x642b68, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63b837; continue; }
            pc = 0x63b861; continue;
            }
            0x63b861 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffe),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x63b837; continue; }
            pc = 0x63b868; continue;
            }
            0x63b868 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x63b837; continue; }
            pc = 0x63b86e; continue;
            }
            0x63b86e => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63b873 => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x63b888; continue; }
            pc = 0x63b87c; continue;
            }
            0x63b87c => {
            c.push(m,0x4u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x63b884);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63b888 => {
            c.set(0,32,rm(m,0x6c8760u32,32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,0x4u32,32); c.set(0,32,r); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063b895(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63b895;
    loop {
        match pc {
            0x63b895 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[0],8));
            { let a = rm(m,c.r[0].wrapping_add(0x6f1bc1),8); c.and(a,0x4u32,8); }
            { let v = c.pop(m); c.set(0,32,v); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063b94f(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63b94f;
    loop {
        match pc {
            0x63b94f => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = rm(m,0x6f1abcu32,32); c.sub(a,0x0u32,32); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            if c.cond(5) { pc = 0x63b96c; continue; }
            pc = 0x63b95d; continue;
            }
            0x63b95d => {
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x63b968);
            { let args = []; c.r[0] = host.call(m, 0x63ba90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63b9e2; continue;
            }
            0x63b96c => {
            c.push(m,0x19u32);
            c.push(m, 0x63b973);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63b977; continue;
            }
            0x63b977 => {
            c.set(3,16,rm(m,c.r[6],8));
            { let a = c.get(3,16); c.and(a,c.get(3,16),16); }
            if c.cond(4) { pc = 0x63b9ca; continue; }
            pc = 0x63b980; continue;
            }
            0x63b980 => {
            c.set(0,32,c.get(3,8));
            { let a = rm(m,c.r[0].wrapping_add(0x6f1bc1),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x63b9a6; continue; }
            pc = 0x63b98c; continue;
            }
            0x63b98c => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x63b9b1; continue; }
            pc = 0x63b994; continue;
            }
            0x63b994 => {
            c.set(1,32,c.get(3,16));
            c.set(0,32,c.get(0,8));
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.or(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x63b9bd; continue; }
            pc = 0x63b9a4; continue;
            }
            0x63b9a4 => {
            pc = 0x63b9ae; continue;
            }
            0x63b9a6 => {
            c.set(0,32,c.get(3,16));
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63b9ca; continue; }
            pc = 0x63b9ae; continue;
            }
            0x63b9ae => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x63b977; continue;
            }
            0x63b9b1 => {
            c.push(m,0x19u32);
            c.push(m, 0x63b9b8);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63b9e2; continue;
            }
            0x63b9bd => {
            c.push(m,0x19u32);
            c.push(m, 0x63b9c4);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,c.r[6].wrapping_add(0xffffffff));
            pc = 0x63b9e2; continue;
            }
            0x63b9ca => {
            c.push(m,0x19u32);
            c.push(m, 0x63b9d1);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,c.get(3,16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.set(0,32,!a & 0xffffffffu32); }
            { let a = c.get(0,32); let r = c.and(a,c.get(6,32),32); c.set(0,32,r); }
            pc = 0x63b9e2; continue;
            }
            0x63b9e2 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063b9e6(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63b9e6;
    loop {
        match pc {
            0x63b9e6 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.sub(a,0xffu32,32); }
            if c.cond(6) { pc = 0x63ba53; continue; }
            pc = 0x63b9f5; continue;
            }
            0x63b9f5 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),16); let r = c.and(a,0x0u32,16); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,16,r); } }
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[5].wrapping_add(0xfffffffe));
            c.stos(m,16,false);
            c.set(0,32,c.get(1,32));
            { let a = c.r[5].wrapping_add(0xb); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = rm(m,0x6f1abcu32,32); c.sub(a,0x0u32,32); }
            { let a = c.r[5].wrapping_add(0xa); wm(m,a,8,c.get(0,8)); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(5) { pc = 0x63ba1b; continue; }
            pc = 0x63ba17; continue;
            }
            0x63ba17 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63ba1b => {
            c.push(m,0x1u32);
            c.set(0,32,c.r[5].wrapping_add(0xfffffffc));
            { let v = rm(m,0x6f1cc4u32,32); c.push(m, v); }
            { let v = rm(m,0x6f1aa0u32,32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xa));
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m, 0x63ba3a);
            { let args = []; c.r[0] = host.call(m, 0x642b68, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63ba17; continue; }
            pc = 0x63ba41; continue;
            }
            0x63ba41 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffe),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x63ba17; continue; }
            pc = 0x63ba48; continue;
            }
            0x63ba48 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); c.and(a,0x8u32,8); }
            if c.cond(4) { pc = 0x63ba17; continue; }
            pc = 0x63ba4e; continue;
            }
            0x63ba4e => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(0,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63ba53 => {
            { let a = rm(m,0x6c896cu32,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x63ba68; continue; }
            pc = 0x63ba5c; continue;
            }
            0x63ba5c => {
            c.push(m,0x8u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x63ba64);
            { let args = []; c.r[0] = host.call(m, 0x63f410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63ba68 => {
            c.set(0,32,rm(m,0x6c8760u32,32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)),8));
            { let a = c.get(0,32); let r = c.and(a,0x8u32,32); c.set(0,32,r); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063be9d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63be9d;
    loop {
        match pc {
            0x63be9d => {
            c.push(m,c.get(6,32));
            c.push(m, 0x63bea3);
            { let args = []; c.r[0] = host.call(m, 0x643ab5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x63beab; continue; }
            pc = 0x63bea9; continue;
            }
            0x63bea9 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63beab => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            c.push(m, 0x63bebe);
            { let args = []; c.r[0] = host.call(m, 0x643945, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x63bec6);
            { let args = []; c.r[0] = host.call(m, 0x6435a6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063bece(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63bece;
    loop {
        match pc {
            0x63bece => {
            c.push(m,0x40u32);
            { let v = rm(m,c.r[4].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0xc),32); c.push(m, v); }
            c.push(m, 0x63bedd);
            { let args = []; c.r[0] = host.call(m, 0x63be9d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063cab7(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63cab7;
    loop {
        match pc {
            0x63cab7 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x19u32);
            c.push(m, 0x63cac3);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(6,32,c.get(7,32));
            { let a = rm(m,c.r[7],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x63cb27; continue; }
            pc = 0x63cace; continue;
            }
            0x63cace => {
            c.set(1,32,rm(m,c.r[6],8));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6f1bc1),8));
            { let a = c.get(0,8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x63cb12; continue; }
            pc = 0x63cadb; continue;
            }
            0x63cadb => {
            c.push(m,0x1u32);
            c.set(0,32,c.r[5].wrapping_add(0x8));
            { let v = rm(m,0x6f1aa0u32,32); c.push(m, v); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,0x2u32);
            c.push(m,c.get(6,32));
            c.push(m,0x100u32);
            { let v = rm(m,0x6f1cc4u32,32); c.push(m, v); }
            c.push(m, 0x63cafc);
            { let args = []; c.r[0] = host.call(m, 0x643b7d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63cb35; continue; }
            pc = 0x63cb03; continue;
            }
            0x63cb03 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            { let a = c.r[6]; wm(m,a,8,c.get(1,8)); }
            if c.cond(14) { pc = 0x63cb21; continue; }
            pc = 0x63cb0d; continue;
            }
            0x63cb0d => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[6]; wm(m,a,8,c.get(5,8)); }
            pc = 0x63cb21; continue;
            }
            0x63cb12 => {
            { let a = c.get(0,8); let r = c.and(a,0x10u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x10u32,8); }
            if c.cond(5) { pc = 0x63cb1f; continue; }
            pc = 0x63cb18; continue;
            }
            0x63cb18 => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x6f1ac0),8));
            pc = 0x63cb1f; continue;
            }
            0x63cb1f => {
            { let a = c.r[6]; wm(m,a,8,c.get(1,8)); }
            pc = 0x63cb21; continue;
            }
            0x63cb21 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = rm(m,c.r[6],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x63cace; continue; }
            pc = 0x63cb27; continue;
            }
            0x63cb27 => {
            c.push(m,0x19u32);
            c.push(m, 0x63cb2e);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,c.get(7,32));
            pc = 0x63cb31; continue;
            }
            0x63cb31 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63cb35 => {
            c.push(m,0x19u32);
            c.push(m, 0x63cb3c);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x63cb31; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063ccb5(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63ccb5;
    loop {
        match pc {
            0x63ccb5 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x63ccbb);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(0,32,0x6c8420u32);
            pc = 0x63ccc8; continue;
            }
            0x63ccc8 => {
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[0],32),32); }
            if c.cond(4) { pc = 0x63ccee; continue; }
            pc = 0x63cccc; continue;
            }
            0x63cccc => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,0x6c8588u32,32); }
            if c.cond(2) { pc = 0x63ccc8; continue; }
            pc = 0x63ccd7; continue;
            }
            0x63ccd7 => {
            { let a = c.get(1,32); c.sub(a,0x13u32,32); }
            if c.cond(2) { pc = 0x63ccfe; continue; }
            pc = 0x63ccdc; continue;
            }
            0x63ccdc => {
            { let a = c.get(1,32); c.sub(a,0x24u32,32); }
            if c.cond(7) { pc = 0x63ccfe; continue; }
            pc = 0x63cce1; continue;
            }
            0x63cce1 => {
            c.push(m, 0x63cce6);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0xdu32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63ccee => {
            c.push(m, 0x63ccf3);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_mul(8).wrapping_add(0x6c8424),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63ccfe => {
            { let a = c.get(1,32); c.sub(a,0xbcu32,32); }
            if c.cond(2) { pc = 0x63cd1b; continue; }
            pc = 0x63cd06; continue;
            }
            0x63cd06 => {
            { let a = c.get(1,32); c.sub(a,0xcau32,32); }
            if c.cond(7) { pc = 0x63cd1b; continue; }
            pc = 0x63cd0e; continue;
            }
            0x63cd0e => {
            c.push(m, 0x63cd13);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x8u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63cd1b => {
            c.push(m, 0x63cd20);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x16u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063cd28(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63cd28;
    loop {
        match pc {
            0x63cd28 => {
            c.push(m, 0x63cd2d);
            { let args = []; c.r[0] = host.call(m, 0x63edb3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063cd31(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63cd31;
    loop {
        match pc {
            0x63cd31 => {
            c.push(m, 0x63cd36);
            { let args = []; c.r[0] = host.call(m, 0x63edb3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063e280(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63e280;
    loop {
        match pc {
            0x63e280 => {
            c.push(m,0xdu32);
            c.push(m, 0x63e287);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063e289(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63e289;
    loop {
        match pc {
            0x63e289 => {
            c.push(m,0xdu32);
            c.push(m, 0x63e290);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063eda0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63eda0;
    loop {
        match pc {
            0x63eda0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x50); wm(m,a,32,0x6cafc0u32); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063edb3(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63edb3;
    loop {
        match pc {
            0x63edb3 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x63edbb); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,0x6c8740u32,32); c.push(m, v); }
            c.set(7,32,c.get(0,32));
            { let target = rm(m,0x66a1d4u32,32); c.push(m, 0x63edc9); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x63ee0e; continue; }
            pc = 0x63edcf; continue;
            }
            0x63edcf => {
            c.push(m,0x74u32);
            c.push(m,0x1u32);
            c.push(m, 0x63edd8);
            { let args = []; c.r[0] = host.call(m, 0x645243, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x63ee06; continue; }
            pc = 0x63ede0; continue;
            }
            0x63ede0 => {
            c.push(m,c.get(6,32));
            { let v = rm(m,0x6c8740u32,32); c.push(m, v); }
            { let target = rm(m,0x66a1ccu32,32); c.push(m, 0x63eded); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x63ee06; continue; }
            pc = 0x63edf1; continue;
            }
            0x63edf1 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x63edf7);
            { let args = []; c.r[0] = host.call(m, 0x63eda0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let target = rm(m,0x66a11cu32,32); c.push(m, 0x63edfe); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x63ee0e; continue;
            }
            0x63ee06 => {
            c.push(m,0x10u32);
            c.push(m, 0x63ee0d);
            { let args = []; c.r[0] = host.call(m, 0x63ca4f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x63ee0e; continue;
            }
            0x63ee0e => {
            c.push(m,c.get(7,32));
            { let target = rm(m,0x66a12cu32,32); c.push(m, 0x63ee15); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0063f410(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x63f410;
    loop {
        match pc {
            0x63f410 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,c.r[0].wrapping_add(0x1));
            { let a = c.get(1,32); c.sub(a,0x100u32,32); }
            if c.cond(7) { pc = 0x63f42e; continue; }
            pc = 0x63f422; continue;
            }
            0x63f422 => {
            c.set(1,32,rm(m,0x6c8760u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)),16));
            pc = 0x63f480; continue;
            }
            0x63f42e => {
            c.set(1,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6c8760u32,32));
            { let a = c.get(1,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(2,32,c.get(1,8));
            { let a = rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x1),8); c.and(a,0x80u32,8); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(4) { pc = 0x63f453; continue; }
            pc = 0x63f445; continue;
            }
            0x63f445 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffe),8); let r = c.and(a,0x0u32,8); { let a = c.r[5].wrapping_add(0xfffffffe); wm(m,a,8,r); } }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[5].wrapping_add(0xfffffffd); wm(m,a,8,c.get(0,8)); }
            c.push(m,0x2u32);
            pc = 0x63f45c; continue;
            }
            0x63f453 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); let r = c.and(a,0x0u32,8); { let a = c.r[5].wrapping_add(0xfffffffd); wm(m,a,8,r); } }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,c.get(0,8)); }
            c.push(m,0x1u32);
            pc = 0x63f45c; continue;
            }
            0x63f45c => {
            { let v = c.pop(m); c.set(0,32,v); }
            c.set(1,32,c.r[5].wrapping_add(0xa));
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffffc));
            c.push(m,c.get(0,32));
            c.push(m,0x1u32);
            c.push(m, 0x63f473);
            { let args = []; c.r[0] = host.call(m, 0x642b68, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x63f47c; continue; }
            pc = 0x63f47a; continue;
            }
            0x63f47a => {
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x63f47c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xa),16));
            pc = 0x63f480; continue;
            }
            0x63f480 => {
            { let a = c.get(0,32); let r = c.and(a,rm(m,c.r[5].wrapping_add(0xc),32),32); c.set(0,32,r); }
            c.r[4] = c.r[5]; c.r[5] = c.pop(m);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00641bab(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x641bab;
    loop {
        match pc {
            0x641bab => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,0x7ff00000u32,32); }
            if c.cond(5) { pc = 0x641bc2; continue; }
            pc = 0x641bb9; continue;
            }
            0x641bb9 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x641bd4; continue; }
            pc = 0x641bbe; continue;
            }
            0x641bbe => {
            c.push(m,0x1u32);
            pc = 0x641bfe; continue;
            }
            0x641bc2 => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,0xfff00000u32,32); }
            if c.cond(5) { pc = 0x641bd4; continue; }
            pc = 0x641bcb; continue;
            }
            0x641bcb => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x641bd4; continue; }
            pc = 0x641bd0; continue;
            }
            0x641bd0 => {
            c.push(m,0x2u32);
            pc = 0x641bfe; continue;
            }
            0x641bd4 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xe),32));
            c.set(0,32,0x7ff8u32);
            { let a = c.get(1,32); let r = c.and(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,16); c.sub(a,c.get(0,16),16); }
            if c.cond(5) { pc = 0x641be7; continue; }
            pc = 0x641be3; continue;
            }
            0x641be3 => {
            c.push(m,0x3u32);
            pc = 0x641bfe; continue;
            }
            0x641be7 => {
            { let a = c.get(1,16); c.sub(a,0x7ff0u32,16); }
            if c.cond(5) { pc = 0x641c01; continue; }
            pc = 0x641bee; continue;
            }
            0x641bee => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.and(a,0x7ffffu32,32); }
            if c.cond(5) { pc = 0x641bfc; continue; }
            pc = 0x641bf7; continue;
            }
            0x641bf7 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x641c01; continue; }
            pc = 0x641bfc; continue;
            }
            0x641bfc => {
            c.push(m,0x4u32);
            pc = 0x641bfe; continue;
            }
            0x641bfe => {
            { let v = c.pop(m); c.set(0,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x641c01 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064211c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64211c;
    loop {
        match pc {
            0x64211c => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,8); c.and(a,0x82u32,8); }
            if c.cond(4) { pc = 0x642228; continue; }
            pc = 0x642132; continue;
            }
            0x642132 => {
            { let a = c.get(0,8); c.and(a,0x40u32,8); }
            if c.cond(5) { pc = 0x642228; continue; }
            pc = 0x64213a; continue;
            }
            0x64213a => {
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x642154; continue; }
            pc = 0x64213e; continue;
            }
            0x64213e => {
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(4) { pc = 0x642228; continue; }
            pc = 0x64214a; continue;
            }
            0x64214a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,8); let r = c.and(a,0xfeu32,8); c.set(0,8,r); }
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x642154; continue;
            }
            0x642154 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = c.get(0,8); let r = c.and(a,0xefu32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.or(a,0x2u32,8); c.set(0,8,r); }
            { let a = c.get(0,16); c.and(a,0x10cu32,16); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x64218e; continue; }
            pc = 0x64216c; continue;
            }
            0x64216c => {
            { let a = c.get(6,32); c.sub(a,0x6cabf0u32,32); }
            if c.cond(4) { pc = 0x64217c; continue; }
            pc = 0x642174; continue;
            }
            0x642174 => {
            { let a = c.get(6,32); c.sub(a,0x6cac10u32,32); }
            if c.cond(5) { pc = 0x642187; continue; }
            pc = 0x64217c; continue;
            }
            0x64217c => {
            c.push(m,c.get(3,32));
            c.push(m, 0x642182);
            { let args = []; c.r[0] = host.call(m, 0x6460b5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x64218e; continue; }
            pc = 0x642187; continue;
            }
            0x642187 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64218d);
            { let args = []; c.r[0] = host.call(m, 0x646071, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x64218e; continue;
            }
            0x64218e => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),16); c.and(a,0x108u32,16); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x6421fe; continue; }
            pc = 0x642197; continue;
            }
            0x642197 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[6],32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[0].wrapping_add(0x1));
            { let a = c.r[6]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x6421be; continue; }
            pc = 0x6421ae; continue;
            }
            0x6421ae => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x6421b6);
            { let args = []; c.r[0] = host.call(m, 0x645e81, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x6421f4; continue;
            }
            0x6421be => {
            { let a = c.get(3,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x6421dc; continue; }
            pc = 0x6421c3; continue;
            }
            0x6421c3 => {
            c.set(1,32,c.get(3,32));
            c.set(0,32,c.get(3,32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            pc = 0x6421e1; continue;
            }
            0x6421dc => {
            c.set(0,32,0x6cb048u32);
            pc = 0x6421e1; continue;
            }
            0x6421e1 => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x6421f4; continue; }
            pc = 0x6421e7; continue;
            }
            0x6421e7 => {
            c.push(m,0x2u32);
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x6421f1);
            { let args = []; c.r[0] = host.call(m, 0x645da9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x6421f4; continue;
            }
            0x6421f4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x8),8));
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x642212; continue;
            }
            0x6421fe => {
            c.push(m,0x1u32);
            c.set(0,32,c.r[5].wrapping_add(0x8));
            { let v = c.pop(m); c.set(7,32,v); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x64220c);
            { let args = []; c.r[0] = host.call(m, 0x645e81, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x642212; continue;
            }
            0x642212 => {
            { let a = rm(m,c.r[5].wrapping_add(0xc),32); c.sub(a,c.get(7,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(4) { pc = 0x64221e; continue; }
            pc = 0x642218; continue;
            }
            0x642218 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.or(a,0x20u32,32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x64222d; continue;
            }
            0x64221e => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            pc = 0x642230; continue;
            }
            0x642228 => {
            { let a = c.get(0,8); let r = c.or(a,0x20u32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x64222d; continue;
            }
            0x64222d => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x642230; continue;
            }
            0x642230 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00642234(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x642234;
    loop {
        match pc {
            0x642234 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x248u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x64294d; continue; }
            pc = 0x642259; continue;
            }
            0x642259 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff0),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x642268; continue;
            }
            0x642260 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff0),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffd0),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x642268; continue;
            }
            0x642268 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffec),32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x64294d; continue; }
            pc = 0x642271; continue;
            }
            0x642271 => {
            { let a = c.get(3,8); c.sub(a,0x20u32,8); }
            if c.cond(12) { pc = 0x642289; continue; }
            pc = 0x642276; continue;
            }
            0x642276 => {
            { let a = c.get(3,8); c.sub(a,0x78u32,8); }
            if c.cond(15) { pc = 0x642289; continue; }
            pc = 0x64227b; continue;
            }
            0x64227b => {
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x67d5f4),8));
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            pc = 0x64228b; continue;
            }
            0x642289 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64228b; continue;
            }
            0x64228b => {
            c.set(0,32,sx(rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x67d614),8),8)&0xffffffffu32);
            { let a = c.get(0,32); let n = 0x4u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x7u32,32); }
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(0,32)); }
            if c.cond(7) { pc = 0x64293c; continue; }
            pc = 0x6422a2; continue;
            }
            0x6422a2 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x642955)); continue;
            }
            0x6422a9 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff0),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,r); } }
            { let a = c.r[5].wrapping_add(0xffffffcc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(2,32)); }
            pc = 0x64293c; continue;
            }
            0x6422c4 => {
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.sub(a,0x20u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x642307; continue; }
            pc = 0x6422cc; continue;
            }
            0x6422cc => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6422fe; continue; }
            pc = 0x6422d1; continue;
            }
            0x6422d1 => {
            { let a = c.get(0,32); let r = c.sub(a,0x8u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6422f5; continue; }
            pc = 0x6422d6; continue;
            }
            0x6422d6 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6422ec; continue; }
            pc = 0x6422da; continue;
            }
            0x6422da => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x64293c; continue; }
            pc = 0x6422e3; continue;
            }
            0x6422e3 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0x8u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x64293c; continue;
            }
            0x6422ec => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0x4u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x64293c; continue;
            }
            0x6422f5 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0x1u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x64293c; continue;
            }
            0x6422fe => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); let r = c.or(a,0x80u32,8); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,r); } }
            pc = 0x64293c; continue;
            }
            0x642307 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0x2u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x64293c; continue;
            }
            0x642310 => {
            { let a = c.get(3,8); c.sub(a,0x2au32,8); }
            if c.cond(5) { pc = 0x642338; continue; }
            pc = 0x642315; continue;
            }
            0x642315 => {
            c.set(0,32,c.r[5].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x64231e);
            { let args = []; c.r[0] = host.call(m, 0x642a13, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x64293c; continue; }
            pc = 0x64232a; continue;
            }
            0x64232a => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0x4u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            pc = 0x642330; continue;
            }
            0x642330 => {
            { let a = c.r[5].wrapping_add(0xffffffe0); wm(m,a,32,c.get(0,32)); }
            pc = 0x64293c; continue;
            }
            0x642338 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.set(1,32,sx(c.get(3,8),8)&0xffffffffu32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0xffffffd0));
            pc = 0x642330; continue;
            }
            0x642347 => {
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(2,32)); }
            pc = 0x64293c; continue;
            }
            0x64234f => {
            { let a = c.get(3,8); c.sub(a,0x2au32,8); }
            if c.cond(5) { pc = 0x642372; continue; }
            pc = 0x642354; continue;
            }
            0x642354 => {
            c.set(0,32,c.r[5].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x64235d);
            { let args = []; c.r[0] = host.call(m, 0x642a13, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x64293c; continue; }
            pc = 0x642369; continue;
            }
            0x642369 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff0),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,r); } }
            pc = 0x64293c; continue;
            }
            0x642372 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,sx(c.get(3,8),8)&0xffffffffu32);
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0xffffffd0));
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(0,32)); }
            pc = 0x64293c; continue;
            }
            0x642384 => {
            { let a = c.get(3,8); c.sub(a,0x49u32,8); }
            if c.cond(4) { pc = 0x6423b7; continue; }
            pc = 0x642389; continue;
            }
            0x642389 => {
            { let a = c.get(3,8); c.sub(a,0x68u32,8); }
            if c.cond(4) { pc = 0x6423ae; continue; }
            pc = 0x64238e; continue;
            }
            0x64238e => {
            { let a = c.get(3,8); c.sub(a,0x6cu32,8); }
            if c.cond(4) { pc = 0x6423a5; continue; }
            pc = 0x642393; continue;
            }
            0x642393 => {
            { let a = c.get(3,8); c.sub(a,0x77u32,8); }
            if c.cond(5) { pc = 0x64293c; continue; }
            pc = 0x64239c; continue;
            }
            0x64239c => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); let r = c.or(a,0x8u32,8); { let a = c.r[5].wrapping_add(0xfffffffd); wm(m,a,8,r); } }
            pc = 0x64293c; continue;
            }
            0x6423a5 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0x10u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x64293c; continue;
            }
            0x6423ae => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0x20u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x64293c; continue;
            }
            0x6423b7 => {
            { let a = rm(m,c.r[7],8); c.sub(a,0x36u32,8); }
            if c.cond(5) { pc = 0x6423d0; continue; }
            pc = 0x6423bc; continue;
            }
            0x6423bc => {
            { let a = rm(m,c.r[7].wrapping_add(0x1),8); c.sub(a,0x34u32,8); }
            if c.cond(5) { pc = 0x6423d0; continue; }
            pc = 0x6423c2; continue;
            }
            0x6423c2 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); let r = c.or(a,0x80u32,8); { let a = c.r[5].wrapping_add(0xfffffffd); wm(m,a,8,r); } }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            pc = 0x64293c; continue;
            }
            0x6423d0 => {
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(2,32)); }
            pc = 0x6423d3; continue;
            }
            0x6423d3 => {
            c.set(1,32,rm(m,0x6c8760u32,32));
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.get(3,8));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x1),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x6423ff; continue; }
            pc = 0x6423e6; continue;
            }
            0x6423e6 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x6423f6);
            { let args = []; c.r[0] = host.call(m, 0x642975, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            pc = 0x6423ff; continue;
            }
            0x6423ff => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x64240f);
            { let args = []; c.r[0] = host.call(m, 0x642975, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x64293c; continue;
            }
            0x642417 => {
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,0x67u32,32); }
            if c.cond(15) { pc = 0x64263f; continue; }
            pc = 0x642423; continue;
            }
            0x642423 => {
            { let a = c.get(0,32); c.sub(a,0x65u32,32); }
            if c.cond(13) { pc = 0x6424c2; continue; }
            pc = 0x64242c; continue;
            }
            0x64242c => {
            { let a = c.get(0,32); c.sub(a,0x58u32,32); }
            if c.cond(15) { pc = 0x642520; continue; }
            pc = 0x642435; continue;
            }
            0x642435 => {
            if c.cond(4) { pc = 0x6426b3; continue; }
            pc = 0x64243b; continue;
            }
            0x64243b => {
            { let a = c.get(0,32); let r = c.sub(a,0x43u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6424e3; continue; }
            pc = 0x642444; continue;
            }
            0x642444 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6424b8; continue; }
            pc = 0x642448; continue;
            }
            0x642448 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6424b8; continue; }
            pc = 0x64244c; continue;
            }
            0x64244c => {
            { let a = c.get(0,32); let r = c.sub(a,0xcu32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x64283e; continue; }
            pc = 0x642455; continue;
            }
            0x642455 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),16); c.and(a,0x830u32,16); }
            if c.cond(5) { pc = 0x642461; continue; }
            pc = 0x64245d; continue;
            }
            0x64245d => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); let r = c.or(a,0x8u32,8); { let a = c.r[5].wrapping_add(0xfffffffd); wm(m,a,8,r); } }
            pc = 0x642461; continue;
            }
            0x642461 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xfffffff0),32));
            { let a = c.get(6,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x64246e; continue; }
            pc = 0x642469; continue;
            }
            0x642469 => {
            c.set(6,32,0x7fffffffu32);
            pc = 0x64246e; continue;
            }
            0x64246e => {
            c.set(0,32,c.r[5].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x642477);
            { let args = []; c.r[0] = host.call(m, 0x642a13, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),16); c.and(a,0x810u32,16); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,c.get(0,32));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x642687; continue; }
            pc = 0x642489; continue;
            }
            0x642489 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x642496; continue; }
            pc = 0x64248d; continue;
            }
            0x64248d => {
            c.set(1,32,rm(m,0x6cabc4u32,32));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            pc = 0x642496; continue;
            }
            0x642496 => {
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,0x1u32); }
            c.set(0,32,c.get(1,32));
            pc = 0x64249f; continue;
            }
            0x64249f => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x64267e; continue; }
            pc = 0x6424aa; continue;
            }
            0x6424aa => {
            { let a = rm(m,c.r[0],16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x64267e; continue; }
            pc = 0x6424b4; continue;
            }
            0x6424b4 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x64249f; continue;
            }
            0x6424b8 => {
            { let a = c.r[5].wrapping_add(0xffffffcc); wm(m,a,32,0x1u32); }
            { let a = c.get(3,8); let r = c.add(a,0x20u32,8); c.set(3,8,r); }
            pc = 0x6424c2; continue;
            }
            0x6424c2 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0x40u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(7,32,c.r[5].wrapping_add(0xfffffdb8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(7,32)); }
            if c.cond(13) { pc = 0x6425a6; continue; }
            pc = 0x6424d7; continue;
            }
            0x6424d7 => {
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,0x6u32); }
            pc = 0x6425b4; continue;
            }
            0x6424e3 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),16); c.and(a,0x830u32,16); }
            if c.cond(5) { pc = 0x6424ef; continue; }
            pc = 0x6424eb; continue;
            }
            0x6424eb => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); let r = c.or(a,0x8u32,8); { let a = c.r[5].wrapping_add(0xfffffffd); wm(m,a,8,r); } }
            pc = 0x6424ef; continue;
            }
            0x6424ef => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),16); c.and(a,0x810u32,16); }
            c.set(0,32,c.r[5].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            if c.cond(4) { pc = 0x642536; continue; }
            pc = 0x6424fb; continue;
            }
            0x6424fb => {
            c.push(m, 0x642500);
            { let args = []; c.r[0] = host.call(m, 0x642a20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffdb8));
            c.push(m,c.get(0,32));
            c.push(m, 0x64250d);
            { let args = []; c.r[0] = host.call(m, 0x6460de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x642549; continue; }
            pc = 0x642517; continue;
            }
            0x642517 => {
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,0x1u32); }
            pc = 0x642549; continue;
            }
            0x642520 => {
            { let a = c.get(0,32); let r = c.sub(a,0x5au32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x642557; continue; }
            pc = 0x642525; continue;
            }
            0x642525 => {
            { let a = c.get(0,32); let r = c.sub(a,0x9u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6424ef; continue; }
            pc = 0x64252a; continue;
            }
            0x64252a => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x642719; continue; }
            pc = 0x642531; continue;
            }
            0x642531 => {
            pc = 0x64283e; continue;
            }
            0x642536 => {
            c.push(m, 0x64253b);
            { let args = []; c.r[0] = host.call(m, 0x642a13, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[5].wrapping_add(0xfffffdb8); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,0x1u32); }
            pc = 0x642549; continue;
            }
            0x642549 => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffdb8));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            pc = 0x64283e; continue;
            }
            0x642557 => {
            c.set(0,32,c.r[5].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x642560);
            { let args = []; c.r[0] = host.call(m, 0x642a13, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x642598; continue; }
            pc = 0x642565; continue;
            }
            0x642565 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x642598; continue; }
            pc = 0x64256c; continue;
            }
            0x64256c => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); c.and(a,0x8u32,8); }
            if c.cond(4) { pc = 0x642589; continue; }
            pc = 0x642572; continue;
            }
            0x642572 => {
            c.set(0,32,sx(rm(m,c.r[0],16),16)&0xffffffffu32);
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,0x1u32); }
            pc = 0x64283e; continue;
            }
            0x642589 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffdc),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,r); } }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,sx(rm(m,c.r[0],16),16)&0xffffffffu32);
            pc = 0x64283b; continue;
            }
            0x642598 => {
            c.set(0,32,rm(m,0x6cabc0u32,32));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(0,32));
            pc = 0x642634; continue;
            }
            0x6425a6 => {
            if c.cond(5) { pc = 0x6425b4; continue; }
            pc = 0x6425a8; continue;
            }
            0x6425a8 => {
            { let a = c.get(3,8); c.sub(a,0x67u32,8); }
            if c.cond(5) { pc = 0x6425b4; continue; }
            pc = 0x6425ad; continue;
            }
            0x6425ad => {
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,0x1u32); }
            pc = 0x6425b4; continue;
            }
            0x6425b4 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let v = rm(m,c.r[5].wrapping_add(0xffffffcc),32); c.push(m, v); }
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff0),32); c.push(m, v); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.r[5].wrapping_add(0xffffffb8); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            { let a = c.r[5].wrapping_add(0xffffffbc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffdb8));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffb8));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x6c8748u32,32); c.push(m, 0x6425e4); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0x80u32,32); c.set(6,32,r); }
            if c.cond(4) { pc = 0x642606; continue; }
            pc = 0x6425f2; continue;
            }
            0x6425f2 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff0),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x642606; continue; }
            pc = 0x6425f8; continue;
            }
            0x6425f8 => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffdb8));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x6c8754u32,32); c.push(m, 0x642605); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x642606; continue;
            }
            0x642606 => {
            { let a = c.get(3,8); c.sub(a,0x67u32,8); }
            if c.cond(5) { pc = 0x64261d; continue; }
            pc = 0x64260b; continue;
            }
            0x64260b => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x64261d; continue; }
            pc = 0x64260f; continue;
            }
            0x64260f => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffdb8));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x6c874cu32,32); c.push(m, 0x64261c); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x64261d; continue;
            }
            0x64261d => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffdb8),8); c.sub(a,0x2du32,8); }
            if c.cond(5) { pc = 0x642633; continue; }
            pc = 0x642626; continue;
            }
            0x642626 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); let r = c.or(a,0x1u32,8); { let a = c.r[5].wrapping_add(0xfffffffd); wm(m,a,8,r); } }
            c.set(7,32,c.r[5].wrapping_add(0xfffffdb9));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(7,32)); }
            pc = 0x642633; continue;
            }
            0x642633 => {
            c.push(m,c.get(7,32));
            pc = 0x642634; continue;
            }
            0x642634 => {
            c.push(m, 0x642639);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x64283b; continue;
            }
            0x64263f => {
            { let a = c.get(0,32); let r = c.sub(a,0x69u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x642719; continue; }
            pc = 0x642648; continue;
            }
            0x642648 => {
            { let a = c.get(0,32); let r = c.sub(a,0x5u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6426ef; continue; }
            pc = 0x642651; continue;
            }
            0x642651 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6426dc; continue; }
            pc = 0x642658; continue;
            }
            0x642658 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6426ac; continue; }
            pc = 0x64265b; continue;
            }
            0x64265b => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x642461; continue; }
            pc = 0x642664; continue;
            }
            0x642664 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x64271d; continue; }
            pc = 0x64266c; continue;
            }
            0x64266c => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x64283e; continue; }
            pc = 0x642675; continue;
            }
            0x642675 => {
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,0x27u32); }
            pc = 0x6426ba; continue;
            }
            0x64267e => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            pc = 0x64283b; continue;
            }
            0x642687 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x642694; continue; }
            pc = 0x64268b; continue;
            }
            0x64268b => {
            c.set(1,32,rm(m,0x6cabc0u32,32));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(1,32)); }
            pc = 0x642694; continue;
            }
            0x642694 => {
            c.set(0,32,c.get(1,32));
            pc = 0x642696; continue;
            }
            0x642696 => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x6426a5; continue; }
            pc = 0x64269d; continue;
            }
            0x64269d => {
            { let a = rm(m,c.r[0],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x6426a5; continue; }
            pc = 0x6426a2; continue;
            }
            0x6426a2 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x642696; continue;
            }
            0x6426a5 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            pc = 0x64283b; continue;
            }
            0x6426ac => {
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,0x8u32); }
            pc = 0x6426b3; continue;
            }
            0x6426b3 => {
            { let a = c.r[5].wrapping_add(0xffffffd4); wm(m,a,32,0x7u32); }
            pc = 0x6426ba; continue;
            }
            0x6426ba => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); c.and(a,0x80u32,8); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,0x10u32); }
            if c.cond(4) { pc = 0x642724; continue; }
            pc = 0x6426c7; continue;
            }
            0x6426c7 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0xffffffd4),8));
            { let a = c.r[5].wrapping_add(0xffffffea); wm(m,a,8,0x30u32); }
            { let a = c.get(0,8); let r = c.add(a,0x51u32,8); c.set(0,8,r); }
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,0x2u32); }
            { let a = c.r[5].wrapping_add(0xffffffeb); wm(m,a,8,c.get(0,8)); }
            pc = 0x642724; continue;
            }
            0x6426dc => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); c.and(a,0x80u32,8); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,0x8u32); }
            if c.cond(4) { pc = 0x642724; continue; }
            pc = 0x6426e9; continue;
            }
            0x6426e9 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); let r = c.or(a,0x2u32,8); { let a = c.r[5].wrapping_add(0xfffffffd); wm(m,a,8,r); } }
            pc = 0x642724; continue;
            }
            0x6426ef => {
            c.set(0,32,c.r[5].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x6426f8);
            { let args = []; c.r[0] = host.call(m, 0x642a13, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); c.and(a,0x20u32,8); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x642708; continue; }
            pc = 0x6426ff; continue;
            }
            0x6426ff => {
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xffffffec),16));
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            pc = 0x64270d; continue;
            }
            0x642708 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x64270d; continue;
            }
            0x64270d => {
            { let a = c.r[5].wrapping_add(0xffffffd8); wm(m,a,32,0x1u32); }
            pc = 0x64293c; continue;
            }
            0x642719 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.or(a,0x40u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x64271d; continue;
            }
            0x64271d => {
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,0xau32); }
            pc = 0x642724; continue;
            }
            0x642724 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x642736; continue; }
            pc = 0x64272a; continue;
            }
            0x64272a => {
            c.set(0,32,c.r[5].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x642733);
            { let args = []; c.r[0] = host.call(m, 0x643488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x642777; continue;
            }
            0x642736 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x64275d; continue; }
            pc = 0x64273c; continue;
            }
            0x64273c => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); c.and(a,0x40u32,8); }
            c.set(0,32,c.r[5].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            if c.cond(4) { pc = 0x642752; continue; }
            pc = 0x642746; continue;
            }
            0x642746 => {
            c.push(m, 0x64274b);
            { let args = []; c.r[0] = host.call(m, 0x642a13, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,sx(c.get(0,16),16)&0xffffffffu32);
            pc = 0x64274f; continue;
            }
            0x64274f => {
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            pc = 0x642777; continue;
            }
            0x642752 => {
            c.push(m, 0x642757);
            { let args = []; c.r[0] = host.call(m, 0x642a13, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(0,32,c.get(0,16));
            pc = 0x64274f; continue;
            }
            0x64275d => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); c.and(a,0x40u32,8); }
            c.set(0,32,c.r[5].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            if c.cond(4) { pc = 0x64276f; continue; }
            pc = 0x642767; continue;
            }
            0x642767 => {
            c.push(m, 0x64276c);
            { let args = []; c.r[0] = host.call(m, 0x642a13, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x64274f; continue;
            }
            0x64276f => {
            c.push(m, 0x642774);
            { let args = []; c.r[0] = host.call(m, 0x642a13, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x642777; continue;
            }
            0x642777 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x642798; continue; }
            pc = 0x64277d; continue;
            }
            0x64277d => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(15) { pc = 0x642798; continue; }
            pc = 0x642781; continue;
            }
            0x642781 => {
            if c.cond(12) { pc = 0x642787; continue; }
            pc = 0x642783; continue;
            }
            0x642783 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x642798; continue; }
            pc = 0x642787; continue;
            }
            0x642787 => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.adc(a,0x0u32,32); c.set(2,32,r); }
            c.set(6,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.neg(a,32); c.set(2,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); let r = c.or(a,0x1u32,8); { let a = c.r[5].wrapping_add(0xfffffffd); wm(m,a,8,r); } }
            c.set(7,32,c.get(2,32));
            pc = 0x64279c; continue;
            }
            0x642798 => {
            c.set(6,32,c.get(0,32));
            c.set(7,32,c.get(2,32));
            pc = 0x64279c; continue;
            }
            0x64279c => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); c.and(a,0x80u32,8); }
            if c.cond(5) { pc = 0x6427a5; continue; }
            pc = 0x6427a2; continue;
            }
            0x6427a2 => {
            { let a = c.get(7,32); let r = c.and(a,0x0u32,32); c.set(7,32,r); }
            pc = 0x6427a5; continue;
            }
            0x6427a5 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff0),32); c.sub(a,0x0u32,32); }
            if c.cond(13) { pc = 0x6427b4; continue; }
            pc = 0x6427ab; continue;
            }
            0x6427ab => {
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,0x1u32); }
            pc = 0x6427b8; continue;
            }
            0x6427b4 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.and(a,0xfffffff7u32,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            pc = 0x6427b8; continue;
            }
            0x6427b8 => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.or(a,c.get(7,32),32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x6427c2; continue; }
            pc = 0x6427be; continue;
            }
            0x6427be => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffe4),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,r); } }
            pc = 0x6427c2; continue;
            }
            0x6427c2 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffb7));
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            pc = 0x6427c8; continue;
            }
            0x6427c8 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff0),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff0),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,r); } }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x6427d8; continue; }
            pc = 0x6427d2; continue;
            }
            0x6427d2 => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.or(a,c.get(7,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x642813; continue; }
            pc = 0x6427d8; continue;
            }
            0x6427d8 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            { let a = c.r[5].wrapping_add(0xffffffc0); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x6427eb);
            { let (a, b) = (u64::from(m.r32(c.r[4].wrapping_add(4))) | u64::from(m.r32(c.r[4].wrapping_add(8))) << 32, u64::from(m.r32(c.r[4].wrapping_add(12))) | u64::from(m.r32(c.r[4].wrapping_add(16))) << 32); let q: u64 = a.checked_rem(b).unwrap_or(0); c.r[0] = q as u32; c.r[2] = (q >> 32) as u32; }
            c.r[4] = c.r[4].wrapping_add(20);
            { let v = rm(m,c.r[5].wrapping_add(0xffffffc4),32); c.push(m, v); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.add(a,0x30u32,32); c.set(3,32,r); }
            { let v = rm(m,c.r[5].wrapping_add(0xffffffc0),32); c.push(m, v); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x6427fd);
            { let (a, b) = (u64::from(m.r32(c.r[4].wrapping_add(4))) | u64::from(m.r32(c.r[4].wrapping_add(8))) << 32, u64::from(m.r32(c.r[4].wrapping_add(12))) | u64::from(m.r32(c.r[4].wrapping_add(16))) << 32); let q: u64 = a.checked_div(b).unwrap_or(0); c.r[0] = q as u32; c.r[2] = (q >> 32) as u32; }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(3,32); c.sub(a,0x39u32,32); }
            c.set(6,32,c.get(0,32));
            c.set(7,32,c.get(2,32));
            if c.cond(14) { pc = 0x642809; continue; }
            pc = 0x642806; continue;
            }
            0x642806 => {
            { let a = c.get(3,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0xffffffd4),32),32); c.set(3,32,r); }
            pc = 0x642809; continue;
            }
            0x642809 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            pc = 0x6427c8; continue;
            }
            0x642813 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffb7));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffff8),32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffd),8); c.and(a,0x2u32,8); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x64283e; continue; }
            pc = 0x642825; continue;
            }
            0x642825 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = rm(m,c.r[1],8); c.sub(a,0x30u32,8); }
            if c.cond(5) { pc = 0x642831; continue; }
            pc = 0x64282d; continue;
            }
            0x64282d => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64283e; continue; }
            pc = 0x642831; continue;
            }
            0x642831 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.dec(a,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.r[1]; wm(m,a,8,0x30u32); }
            pc = 0x64283b; continue;
            }
            0x64283b => {
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,c.get(0,32)); }
            pc = 0x64283e; continue;
            }
            0x64283e => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffd8),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x64293c; continue; }
            pc = 0x642848; continue;
            }
            0x642848 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(3,8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x642876; continue; }
            pc = 0x642850; continue;
            }
            0x642850 => {
            { let a = c.get(7,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x64285b; continue; }
            pc = 0x642855; continue;
            }
            0x642855 => {
            { let a = c.r[5].wrapping_add(0xffffffea); wm(m,a,8,0x2du32); }
            pc = 0x64286f; continue;
            }
            0x64285b => {
            { let a = c.get(3,8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x642866; continue; }
            pc = 0x642860; continue;
            }
            0x642860 => {
            { let a = c.r[5].wrapping_add(0xffffffea); wm(m,a,8,0x2bu32); }
            pc = 0x64286f; continue;
            }
            0x642866 => {
            { let a = c.get(3,8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x642876; continue; }
            pc = 0x64286b; continue;
            }
            0x64286b => {
            { let a = c.r[5].wrapping_add(0xffffffea); wm(m,a,8,0x20u32); }
            pc = 0x64286f; continue;
            }
            0x64286f => {
            { let a = c.r[5].wrapping_add(0xffffffe4); wm(m,a,32,0x1u32); }
            pc = 0x642876; continue;
            }
            0x642876 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            { let a = c.get(6,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0xffffffe4),32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffff4),32),32); c.set(6,32,r); }
            { let a = c.get(3,8); c.and(a,0xcu32,8); }
            if c.cond(5) { pc = 0x642896; continue; }
            pc = 0x642884; continue;
            }
            0x642884 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m,c.get(6,32));
            c.push(m,0x20u32);
            c.push(m, 0x642893);
            { let args = []; c.r[0] = host.call(m, 0x6429aa, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x642896; continue;
            }
            0x642896 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffea));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xffffffe4),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x6428a9);
            { let args = []; c.r[0] = host.call(m, 0x6429db, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(3,8); c.and(a,0x8u32,8); }
            if c.cond(4) { pc = 0x6428c8; continue; }
            pc = 0x6428b1; continue;
            }
            0x6428b1 => {
            { let a = c.get(3,8); c.and(a,0x4u32,8); }
            if c.cond(5) { pc = 0x6428c8; continue; }
            pc = 0x6428b6; continue;
            }
            0x6428b6 => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m,c.get(6,32));
            c.push(m,0x30u32);
            c.push(m, 0x6428c5);
            { let args = []; c.r[0] = host.call(m, 0x6429aa, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x6428c8; continue;
            }
            0x6428c8 => {
            { let a = rm(m,c.r[5].wrapping_add(0xffffffdc),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x64290f; continue; }
            pc = 0x6428ce; continue;
            }
            0x6428ce => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff4),32); c.sub(a,0x0u32,32); }
            if c.cond(14) { pc = 0x64290f; continue; }
            pc = 0x6428d4; continue;
            }
            0x6428d4 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            c.set(7,32,c.r[0].wrapping_add(0xffffffff));
            pc = 0x6428dd; continue;
            }
            0x6428dd => {
            c.set(0,16,rm(m,c.r[3],16));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffc8));
            c.push(m,c.get(0,32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.push(m, 0x6428ec);
            { let args = []; c.r[0] = host.call(m, 0x6460de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(14) { pc = 0x642924; continue; }
            pc = 0x6428f2; continue;
            }
            0x6428f2 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(1,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xffffffc8));
            c.push(m,c.get(0,32));
            c.push(m, 0x642903);
            { let args = []; c.r[0] = host.call(m, 0x6429db, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x6428dd; continue; }
            pc = 0x64290d; continue;
            }
            0x64290d => {
            pc = 0x642924; continue;
            }
            0x64290f => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff4),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.push(m, v); }
            c.push(m, 0x642921);
            { let args = []; c.r[0] = host.call(m, 0x6429db, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x642924; continue;
            }
            0x642924 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),8); c.and(a,0x4u32,8); }
            if c.cond(4) { pc = 0x64293c; continue; }
            pc = 0x64292a; continue;
            }
            0x64292a => {
            c.set(0,32,c.r[5].wrapping_add(0xffffffec));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m,c.get(6,32));
            c.push(m,0x20u32);
            c.push(m, 0x642939);
            { let args = []; c.r[0] = host.call(m, 0x6429aa, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            pc = 0x64293c; continue;
            }
            0x64293c => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(3,8,rm(m,c.r[7],8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x642260; continue; }
            pc = 0x64294d; continue;
            }
            0x64294d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffec),32));
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

pub fn f_00642975(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x642975;
    loop {
        match pc {
            0x642975 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); let r = c.dec(a,32); { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,r); } }
            if c.cond(8) { pc = 0x64298e; continue; }
            pc = 0x642980; continue;
            }
            0x642980 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x8),8));
            { let a = c.r[2]; wm(m,a,8,c.get(0,8)); }
            { let a = rm(m,c.r[1],32); let r = c.inc(a,32); { let a = c.r[1]; wm(m,a,32,r); } }
            c.set(0,32,c.get(0,8));
            pc = 0x642999; continue;
            }
            0x64298e => {
            c.push(m,c.get(1,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x642997);
            { let args = []; c.r[0] = host.call(m, 0x64211c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x642999; continue;
            }
            0x642999 => {
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            if c.cond(5) { pc = 0x6429a6; continue; }
            pc = 0x6429a1; continue;
            }
            0x6429a1 => {
            { let a = rm(m,c.r[0],32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x6429a6 => {
            { let a = rm(m,c.r[0],32); let r = c.inc(a,32); { let a = c.r[0]; wm(m,a,32,r); } }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006429aa(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6429aa;
    loop {
        match pc {
            0x6429aa => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.get(7,32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x6429d8; continue; }
            pc = 0x6429b7; continue;
            }
            0x6429b7 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x6429bb; continue;
            }
            0x6429bb => {
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m, 0x6429c9);
            { let args = []; c.r[0] = host.call(m, 0x642975, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6],32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x6429d8; continue; }
            pc = 0x6429d1; continue;
            }
            0x6429d1 => {
            c.set(0,32,c.get(7,32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x6429bb; continue; }
            pc = 0x6429d8; continue;
            }
            0x6429d8 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006429db(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6429db;
    loop {
        match pc {
            0x6429db => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,c.get(3,32));
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x642a0f; continue; }
            pc = 0x6429e9; continue;
            }
            0x6429e9 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x6429f1; continue;
            }
            0x6429f1 => {
            c.set(0,32,sx(rm(m,c.r[6],8),8)&0xffffffffu32);
            c.push(m,c.get(7,32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let v = rm(m,c.r[4].wrapping_add(0x1c),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x642a00);
            { let args = []; c.r[0] = host.call(m, 0x642975, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[7],32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x642a0f; continue; }
            pc = 0x642a08; continue;
            }
            0x642a08 => {
            c.set(0,32,c.get(3,32));
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x6429f1; continue; }
            pc = 0x642a0f; continue;
            }
            0x642a0f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00642a13(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x642a13;
    loop {
        match pc {
            0x642a13 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = rm(m,c.r[0],32); let r = c.add(a,0x4u32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            c.set(0,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00642a20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x642a20;
    loop {
        match pc {
            0x642a20 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = rm(m,c.r[0],32); let r = c.add(a,0x4u32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            c.set(0,32,rm(m,c.r[0],32));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0xfffffffc),16));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00643488(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x643488;
    loop {
        match pc {
            0x643488 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = rm(m,c.r[0],32); let r = c.add(a,0x8u32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            c.set(1,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xfffffff8),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xfffffffc),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006435a6(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6435a6;
    loop {
        match pc {
            0x6435a6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,0x6cabd0u32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x6435ca; continue; }
            pc = 0x6435b3; continue;
            }
            0x6435b3 => {
            { let a = c.get(0,32); c.sub(a,0x6cae30u32,32); }
            if c.cond(7) { pc = 0x6435ca; continue; }
            pc = 0x6435ba; continue;
            }
            0x6435ba => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x1cu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x6435c8);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x6435ca => {
            { let a = c.get(0,32); let r = c.add(a,0x20u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1acu32,32); c.push(m, 0x6435d4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00643945(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x643945;
    loop {
        match pc {
            0x643945 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6f040cu32,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(3,32)); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,8); c.sub(a,0x61u32,8); }
            if c.cond(4) { pc = 0x64397e; continue; }
            pc = 0x643964; continue;
            }
            0x643964 => {
            { let a = c.get(0,8); c.sub(a,0x72u32,8); }
            if c.cond(4) { pc = 0x643977; continue; }
            pc = 0x643968; continue;
            }
            0x643968 => {
            { let a = c.get(0,8); c.sub(a,0x77u32,8); }
            if c.cond(5) { pc = 0x643a92; continue; }
            pc = 0x643970; continue;
            }
            0x643970 => {
            c.set(1,32,0x301u32);
            pc = 0x643983; continue;
            }
            0x643977 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.or(a,0x1u32,32); c.set(6,32,r); }
            pc = 0x643986; continue;
            }
            0x64397e => {
            c.set(1,32,0x109u32);
            pc = 0x643983; continue;
            }
            0x643983 => {
            { let a = c.get(6,32); let r = c.or(a,0x2u32,32); c.set(6,32,r); }
            pc = 0x643986; continue;
            }
            0x643986 => {
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(2,32,v); }
            pc = 0x643989; continue;
            }
            0x643989 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x643a78; continue; }
            pc = 0x643995; continue;
            }
            0x643995 => {
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x643a78; continue; }
            pc = 0x64399d; continue;
            }
            0x64399d => {
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            { let a = c.get(0,32); c.sub(a,0x54u32,32); }
            if c.cond(15) { pc = 0x643a17; continue; }
            pc = 0x6439a5; continue;
            }
            0x6439a5 => {
            if c.cond(4) { pc = 0x643a07; continue; }
            pc = 0x6439a7; continue;
            }
            0x6439a7 => {
            { let a = c.get(0,32); let r = c.sub(a,0x2bu32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6439f1; continue; }
            pc = 0x6439ac; continue;
            }
            0x6439ac => {
            { let a = c.get(0,32); let r = c.sub(a,0x19u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6439e7; continue; }
            pc = 0x6439b1; continue;
            }
            0x6439b1 => {
            { let a = c.get(0,32); let r = c.sub(a,0xeu32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6439d2; continue; }
            pc = 0x6439b6; continue;
            }
            0x6439b6 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x643a69; continue; }
            pc = 0x6439bd; continue;
            }
            0x6439bd => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x643a69; continue; }
            pc = 0x6439c6; continue;
            }
            0x6439c6 => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            { let a = c.get(1,32); let r = c.or(a,0x20u32,32); c.set(1,32,r); }
            pc = 0x643989; continue;
            }
            0x6439d2 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x643a69; continue; }
            pc = 0x6439db; continue;
            }
            0x6439db => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            { let a = c.get(1,32); let r = c.or(a,0x10u32,32); c.set(1,32,r); }
            pc = 0x643989; continue;
            }
            0x6439e7 => {
            { let a = c.get(1,8); c.and(a,0x40u32,8); }
            if c.cond(5) { pc = 0x643a69; continue; }
            pc = 0x6439ec; continue;
            }
            0x6439ec => {
            { let a = c.get(1,32); let r = c.or(a,0x40u32,32); c.set(1,32,r); }
            pc = 0x643989; continue;
            }
            0x6439f1 => {
            { let a = c.get(1,8); c.and(a,0x2u32,8); }
            if c.cond(5) { pc = 0x643a69; continue; }
            pc = 0x6439f6; continue;
            }
            0x6439f6 => {
            { let a = c.get(1,32); let r = c.and(a,0xfffffffeu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xfffffffcu32,32); c.set(6,32,r); }
            { let a = c.get(1,32); let r = c.or(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.or(a,0x80u32,32); c.set(6,32,r); }
            pc = 0x643989; continue;
            }
            0x643a07 => {
            c.set(0,32,0x1000u32);
            { let a = c.get(0,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x643a69; continue; }
            pc = 0x643a10; continue;
            }
            0x643a10 => {
            { let a = c.get(1,32); let r = c.or(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x643989; continue;
            }
            0x643a17 => {
            { let a = c.get(0,32); let r = c.sub(a,0x62u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x643a64; continue; }
            pc = 0x643a1c; continue;
            }
            0x643a1c => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x643a4d; continue; }
            pc = 0x643a1f; continue;
            }
            0x643a1f => {
            { let a = c.get(0,32); let r = c.sub(a,0xbu32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x643a36; continue; }
            pc = 0x643a24; continue;
            }
            0x643a24 => {
            { let a = c.get(0,32); let r = c.sub(a,0x6u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x643a69; continue; }
            pc = 0x643a29; continue;
            }
            0x643a29 => {
            { let a = c.get(5,8); c.and(a,0xc0u32,8); }
            if c.cond(5) { pc = 0x643a69; continue; }
            pc = 0x643a2e; continue;
            }
            0x643a2e => {
            { let a = c.get(5,8); let r = c.or(a,0x40u32,8); c.set(5,8,r); }
            pc = 0x643989; continue;
            }
            0x643a36 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x643a69; continue; }
            pc = 0x643a3b; continue;
            }
            0x643a3b => {
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,0x1u32); }
            { let a = c.get(6,32); let r = c.and(a,0xffffbfffu32,32); c.set(6,32,r); }
            pc = 0x643989; continue;
            }
            0x643a4d => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x643a69; continue; }
            pc = 0x643a52; continue;
            }
            0x643a52 => {
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,0x1u32); }
            { let a = c.get(6,32); let r = c.or(a,0x4000u32,32); c.set(6,32,r); }
            pc = 0x643989; continue;
            }
            0x643a64 => {
            { let a = c.get(5,8); c.and(a,0xc0u32,8); }
            if c.cond(4) { pc = 0x643a70; continue; }
            pc = 0x643a69; continue;
            }
            0x643a69 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x643989; continue;
            }
            0x643a70 => {
            { let a = c.get(5,8); let r = c.or(a,0x80u32,8); c.set(5,8,r); }
            pc = 0x643989; continue;
            }
            0x643a78 => {
            c.push(m,0x1a4u32);
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m,c.get(1,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x643a89);
            { let args = []; c.r[0] = host.call(m, 0x646eac, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x643a96; continue; }
            pc = 0x643a92; continue;
            }
            0x643a92 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x643ab0; continue;
            }
            0x643a96 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            { let a = rm(m,0x6f020cu32,32); let r = c.inc(a,32); { let a = 0x6f020cu32; wm(m,a,32,r); } }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x643ab0; continue;
            }
            0x643ab0 => {
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

pub fn f_00645da9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x645da9;
    loop {
        match pc {
            0x645da9 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6f0a40u32,32),32); }
            if c.cond(3) { pc = 0x645df6; continue; }
            pc = 0x645db6; continue;
            }
            0x645db6 => {
            c.set(1,32,c.get(6,32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x645df6; continue; }
            pc = 0x645dd1; continue;
            }
            0x645dd1 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x645dd8);
            { let args = []; c.r[0] = host.call(m, 0x646b5a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            c.push(m,c.get(6,32));
            c.push(m, 0x645de6);
            { let args = []; c.r[0] = host.call(m, 0x645e0e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x645dee);
            { let args = []; c.r[0] = host.call(m, 0x646bb9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x645df6 => {
            c.push(m, 0x645dfb);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x9u32); }
            c.push(m, 0x645e06);
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

pub fn f_00645e0e(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x645e0e;
    loop {
        match pc {
            0x645e0e => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x645e1a);
            { let args = []; c.r[0] = host.call(m, 0x646b18, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x645e2d; continue; }
            pc = 0x645e20; continue;
            }
            0x645e20 => {
            c.push(m, 0x645e25);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x9u32); }
            pc = 0x645e5a; continue;
            }
            0x645e2d => {
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            c.push(m,0x0u32);
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a148u32,32); c.push(m, 0x645e3e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x645e4d; continue; }
            pc = 0x645e45; continue;
            }
            0x645e45 => {
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x645e4b); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x645e4f; continue;
            }
            0x645e4d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x645e4f; continue;
            }
            0x645e4f => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x645e5f; continue; }
            pc = 0x645e53; continue;
            }
            0x645e53 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x645e59);
            { let args = []; c.r[0] = host.call(m, 0x63ccb5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x645e5a; continue;
            }
            0x645e5a => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x645e7e; continue;
            }
            0x645e5f => {
            c.set(1,32,c.get(6,32));
            { let a = c.get(6,32); let r = c.and(a,0x1fu32,32); c.set(6,32,r); }
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.get(6,32));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),8); let r = c.and(a,0xfdu32,8); { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,8,r); } }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4));
            c.set(0,32,c.get(7,32));
            pc = 0x645e7e; continue;
            }
            0x645e7e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00645e81(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x645e81;
    loop {
        match pc {
            0x645e81 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6f0a40u32,32),32); }
            if c.cond(3) { pc = 0x645ece; continue; }
            pc = 0x645e8e; continue;
            }
            0x645e8e => {
            c.set(1,32,c.get(6,32));
            c.set(0,32,c.get(6,32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x645ece; continue; }
            pc = 0x645ea9; continue;
            }
            0x645ea9 => {
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x645eb0);
            { let args = []; c.r[0] = host.call(m, 0x646b5a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            c.push(m,c.get(6,32));
            c.push(m, 0x645ebe);
            { let args = []; c.r[0] = host.call(m, 0x645ee6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x645ec6);
            { let args = []; c.r[0] = host.call(m, 0x646bb9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x645ece => {
            c.push(m, 0x645ed3);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x9u32); }
            c.push(m, 0x645ede);
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

pub fn f_00645ee6(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x645ee6;
    loop {
        match pc {
            0x645ee6 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x414u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x645f06; continue; }
            pc = 0x645eff; continue;
            }
            0x645eff => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64606c; continue;
            }
            0x645f06 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            c.set(3,32,c.r[0].wrapping_mul(4).wrapping_add(0x6f0940));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4),8); c.and(a,0x20u32,8); }
            if c.cond(4) { pc = 0x645f36; continue; }
            pc = 0x645f28; continue;
            }
            0x645f28 => {
            c.push(m,0x2u32);
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x645f33);
            { let args = []; c.r[0] = host.call(m, 0x645e0e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x645f36; continue;
            }
            0x645f36 => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x4),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x646005; continue; }
            pc = 0x645f44; continue;
            }
            0x645f44 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = rm(m,c.r[5].wrapping_add(0x10),32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            if c.cond(6) { pc = 0x646040; continue; }
            pc = 0x645f56; continue;
            }
            0x645f56 => {
            c.set(0,32,c.r[5].wrapping_add(0xfffffbec));
            pc = 0x645f5c; continue;
            }
            0x645f5c => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(1,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0xc),32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x10),32),32); }
            if c.cond(3) { pc = 0x645f90; continue; }
            pc = 0x645f67; continue;
            }
            0x645f67 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffffc),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,r); } }
            c.set(1,8,rm(m,c.r[1],8));
            { let a = c.get(1,8); c.sub(a,0xau32,8); }
            if c.cond(5) { pc = 0x645f7b; continue; }
            pc = 0x645f74; continue;
            }
            0x645f74 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff0),32); let r = c.inc(a,32); { let a = c.r[5].wrapping_add(0xfffffff0); wm(m,a,32,r); } }
            { let a = c.r[0]; wm(m,a,8,0xdu32); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            pc = 0x645f7b; continue;
            }
            0x645f7b => {
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.get(0,32));
            c.set(2,32,c.r[5].wrapping_add(0xfffffbec));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x400u32,32); }
            if c.cond(12) { pc = 0x645f5c; continue; }
            pc = 0x645f90; continue;
            }
            0x645f90 => {
            c.set(7,32,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffbec));
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[5].wrapping_add(0xfffffbec));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3],32));
            { let v = rm(m,c.r[0].wrapping_add(c.r[6]),32); c.push(m, v); }
            { let target = rm(m,0x66a144u32,32); c.push(m, 0x645fb3); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x645ffa; continue; }
            pc = 0x645fb7; continue;
            }
            0x645fb7 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(12) { pc = 0x645fcc; continue; }
            pc = 0x645fc1; continue;
            }
            0x645fc1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffffc),32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0xc),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x10),32),32); }
            if c.cond(2) { pc = 0x645f56; continue; }
            pc = 0x645fcc; continue;
            }
            0x645fcc => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x645fce; continue;
            }
            0x645fce => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x646069; continue; }
            pc = 0x645fd9; continue;
            }
            0x645fd9 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x646040; continue; }
            pc = 0x645fde; continue;
            }
            0x645fde => {
            c.push(m,0x5u32);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x646032; continue; }
            pc = 0x645fe6; continue;
            }
            0x645fe6 => {
            c.push(m, 0x645feb);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x9u32); }
            c.push(m, 0x645ff6);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            pc = 0x64603b; continue;
            }
            0x645ffa => {
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x646000); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x645fcc; continue;
            }
            0x646005 => {
            c.set(1,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            { let v = rm(m,c.r[5].wrapping_add(0x10),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[0],32); c.push(m, v); }
            { let target = rm(m,0x66a144u32,32); c.push(m, 0x646018); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x646027; continue; }
            pc = 0x64601c; continue;
            }
            0x64601c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xfffffff4),32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,c.get(0,32)); }
            pc = 0x645fce; continue;
            }
            0x646027 => {
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x64602d); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x645fce; continue;
            }
            0x646032 => {
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x64603a);
            { let args = []; c.r[0] = host.call(m, 0x63ccb5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x64603b; continue;
            }
            0x64603b => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x64606c; continue;
            }
            0x646040 => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x4),8); c.and(a,0x40u32,8); }
            if c.cond(4) { pc = 0x646055; continue; }
            pc = 0x646049; continue;
            }
            0x646049 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = rm(m,c.r[0],8); c.sub(a,0x1au32,8); }
            if c.cond(4) { pc = 0x645eff; continue; }
            pc = 0x646055; continue;
            }
            0x646055 => {
            c.push(m, 0x64605a);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x1cu32); }
            c.push(m, 0x646065);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            pc = 0x64603b; continue;
            }
            0x646069 => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[5].wrapping_add(0xfffffff0),32),32); c.set(0,32,r); }
            pc = 0x64606c; continue;
            }
            0x64606c => {
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

pub fn f_00646071(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x646071;
    loop {
        match pc {
            0x646071 => {
            { let a = rm(m,0x6f020cu32,32); let r = c.inc(a,32); { let a = 0x6f020cu32; wm(m,a,32,r); } }
            c.push(m,0x1000u32);
            c.push(m, 0x646081);
            { let args = []; c.r[0] = host.call(m, 0x63cb41, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x64609a; continue; }
            pc = 0x64608d; continue;
            }
            0x64608d => {
            { let a = rm(m,c.r[1].wrapping_add(0xc),32); let r = c.or(a,0x8u32,32); { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = c.r[1].wrapping_add(0x18); wm(m,a,32,0x1000u32); }
            pc = 0x6460ab; continue;
            }
            0x64609a => {
            { let a = rm(m,c.r[1].wrapping_add(0xc),32); let r = c.or(a,0x4u32,32); { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,r); } }
            c.set(0,32,c.r[1].wrapping_add(0x14));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(0x18); wm(m,a,32,0x2u32); }
            pc = 0x6460ab; continue;
            }
            0x6460ab => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); let r = c.and(a,0x0u32,32); { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,r); } }
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006460b5(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6460b5;
    loop {
        match pc {
            0x6460b5 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,0x6f0a40u32,32),32); }
            if c.cond(2) { pc = 0x6460c4; continue; }
            pc = 0x6460c1; continue;
            }
            0x6460c1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x6460c4 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),8));
            { let a = c.get(0,32); let r = c.and(a,0x40u32,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006460de(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6460de;
    loop {
        match pc {
            0x6460de => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,0x6f0a48u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a0dcu32,32); c.push(m, 0x6460f0); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,0x66a0e0u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = rm(m,0x6f0a44u32,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x64610e; continue; }
            pc = 0x646100; continue;
            }
            0x646100 => {
            c.push(m,c.get(6,32));
            { let target = c.get(7,32); c.push(m, 0x646103); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x13u32);
            c.push(m, 0x64610a);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x64610e; continue;
            }
            0x64610e => {
            { let v = rm(m,c.r[5].wrapping_add(0xc),32); c.push(m, v); }
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x646119);
            { let args = []; c.r[0] = host.call(m, 0x646137, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x64612c; continue; }
            pc = 0x646122; continue;
            }
            0x646122 => {
            c.push(m,0x13u32);
            c.push(m, 0x646129);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x64612f; continue;
            }
            0x64612c => {
            c.push(m,c.get(6,32));
            { let target = c.get(7,32); c.push(m, 0x64612f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64612f; continue;
            }
            0x64612f => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
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

pub fn f_00646b18(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x646b18;
    loop {
        match pc {
            0x646b18 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,0x6f0a40u32,32),32); }
            if c.cond(3) { pc = 0x646b43; continue; }
            pc = 0x646b24; continue;
            }
            0x646b24 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),8); c.and(a,0x1u32,8); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            if c.cond(4) { pc = 0x646b43; continue; }
            pc = 0x646b40; continue;
            }
            0x646b40 => {
            c.set(0,32,rm(m,c.r[0],32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x646b43 => {
            c.push(m, 0x646b48);
            { let args = []; c.r[0] = host.call(m, 0x63cd28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0]; wm(m,a,32,0x9u32); }
            c.push(m, 0x646b53);
            { let args = []; c.r[0] = host.call(m, 0x63cd31, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[0],32); let r = c.and(a,0x0u32,32); { let a = c.r[0]; wm(m,a,32,r); } }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00646b5a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x646b5a;
    loop {
        match pc {
            0x646b5a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(3,32,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(7,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(7,32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x646ba8; continue; }
            pc = 0x646b85; continue;
            }
            0x646b85 => {
            c.push(m,0x11u32);
            c.push(m, 0x646b8c);
            { let args = []; c.r[0] = host.call(m, 0x641427, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); c.sub(a,0x0u32,32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x646ba0; continue; }
            pc = 0x646b93; continue;
            }
            0x646b93 => {
            c.set(0,32,c.r[6].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1a4u32,32); c.push(m, 0x646b9d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); let r = c.inc(a,32); { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,r); } }
            pc = 0x646ba0; continue;
            }
            0x646ba0 => {
            c.push(m,0x11u32);
            c.push(m, 0x646ba7);
            { let args = []; c.r[0] = host.call(m, 0x641488, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x646ba8; continue;
            }
            0x646ba8 => {
            c.set(0,32,rm(m,c.r[3],32));
            c.set(0,32,c.r[0].wrapping_add(c.r[7]).wrapping_add(0xc));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1b0u32,32); c.push(m, 0x646bb5); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00646bb9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x646bb9;
    loop {
        match pc {
            0x646bb9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6f0940),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xc));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a1acu32,32); c.push(m, 0x646bda); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064b52e(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64b52e;
    loop {
        match pc {
            0x64b52e => {
            c.set(0,32,rm(m,0x6f08b8u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x2u32);
            c.set(6,32,c.get(1,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x64b546; continue; }
            pc = 0x64b53f; continue;
            }
            0x64b53f => {
            c.push(m,0x6f08a0u32);
            pc = 0x64b5a0; continue;
            }
            0x64b546 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,32,0x6f08a0u32);
            if c.cond(5) { pc = 0x64b584; continue; }
            pc = 0x64b54f; continue;
            }
            0x64b54f => {
            c.push(m,0x1u32);
            c.push(m,0x6f08b8u32);
            { let target = rm(m,0x66a2ccu32,32); c.push(m, 0x64b55c); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64b57a; continue; }
            pc = 0x64b560; continue;
            }
            0x64b560 => {
            c.push(m,c.get(3,32));
            { let target = rm(m,0x66a1a4u32,32); c.push(m, 0x64b567); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x64b5acu32);
            c.push(m, 0x64b571);
            { let args = []; c.r[0] = host.call(m, 0x63978e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = 0x6f08b8u32; wm(m,a,32,c.get(7,32)); }
            pc = 0x64b59f; continue;
            }
            0x64b57a => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x64b584; continue; }
            pc = 0x64b57e; continue;
            }
            0x64b57e => {
            { let a = 0x6f08b8u32; wm(m,a,32,c.get(7,32)); }
            pc = 0x64b584; continue;
            }
            0x64b584 => {
            { let a = rm(m,0x6f08b8u32,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x64b597; continue; }
            pc = 0x64b58d; continue;
            }
            0x64b58d => {
            c.push(m,0x1u32);
            { let target = rm(m,0x66a2bcu32,32); c.push(m, 0x64b595); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64b584; continue;
            }
            0x64b597 => {
            { let a = rm(m,0x6f08b8u32,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x64b5a6; continue; }
            pc = 0x64b59f; continue;
            }
            0x64b59f => {
            c.push(m,c.get(3,32));
            pc = 0x64b5a0; continue;
            }
            0x64b5a0 => {
            { let target = rm(m,0x66a1b0u32,32); c.push(m, 0x64b5a6); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64b5a6; continue;
            }
            0x64b5a6 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064b5ca(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64b5ca;
    loop {
        match pc {
            0x64b5ca => {
            { let a = rm(m,0x6f08b8u32,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x64b5de; continue; }
            pc = 0x64b5d3; continue;
            }
            0x64b5d3 => {
            c.push(m,0x6f08a0u32);
            { let target = rm(m,0x66a1acu32,32); c.push(m, 0x64b5de); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64b5de; continue;
            }
            0x64b5de => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064cd70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64cd70;
    loop {
        match pc {
            0x64cd70 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(13) { pc = 0x64cd81; continue; }
            pc = 0x64cd7f; continue;
            }
            0x64cd7f => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x64cd81; continue;
            }
            0x64cd81 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64cd8d; continue; }
            pc = 0x64cd89; continue;
            }
            0x64cd89 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x64cd96; continue;
            }
            0x64cd8d => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a29cu32,32); c.push(m, 0x64cd94); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            pc = 0x64cd96; continue;
            }
            0x64cd96 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            c.set(6,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            if c.cond(14) { pc = 0x64ce1b; continue; }
            pc = 0x64cd9f; continue;
            }
            0x64cd9f => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x64cda6);
            { let args = []; c.r[0] = host.call(m, 0x64f1fc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); c.sub(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x64cdac; continue; }
            pc = 0x64cdaa; continue;
            }
            0x64cdaa => {
            c.set(5,32,c.get(6,32));
            pc = 0x64cdac; continue;
            }
            0x64cdac => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(6,32); let r = c.add(a,c.get(3,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[0].wrapping_add(0xfffffffc),32); c.sub(a,c.get(6,32),32); }
            if c.cond(13) { pc = 0x64cdeb; continue; }
            pc = 0x64cdb9; continue;
            }
            0x64cdb9 => {
            { let a = c.get(0,32); let r = c.add(a,0xfffffff4u32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x64cdc8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x64cdde);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x64cdeb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f1bb, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64cdeb; continue;
            }
            0x64cdeb => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[5]));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64ce03);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(3,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x64ce13);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.r[2].wrapping_add(0xfffffff8); wm(m,a,32,c.get(6,32)); }
            pc = 0x64ce1b; continue;
            }
            0x64ce1b => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
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

pub fn f_0064ce30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ce30;
    loop {
        match pc {
            0x64ce30 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x4),8));
            c.push(m,c.get(3,32));
            c.set(3,8,rm(m,c.r[4].wrapping_add(0xc),8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            c.set(6,32,c.get(1,32));
            if c.cond(4) { pc = 0x64ce6d; continue; }
            pc = 0x64ce43; continue;
            }
            0x64ce43 => {
            c.push(m, 0x64ce48);
            { let args = []; c.r[0] = host.call(m, 0x64f1fc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x64ce6d; continue; }
            pc = 0x64ce53; continue;
            }
            0x64ce53 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x64ce60; continue; }
            pc = 0x64ce5d; continue;
            }
            0x64ce5d => {
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x64ce60; continue;
            }
            0x64ce60 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x64ce66);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x64ce53; continue; }
            pc = 0x64ce6d; continue;
            }
            0x64ce6d => {
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064d180(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64d180;
    loop {
        match pc {
            0x64d180 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66112au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            { let a = c.r[4]; wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x64d1a8; continue; }
            pc = 0x64d1a6; continue;
            }
            0x64d1a6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64d1a8; continue;
            }
            0x64d1a8 => {
            c.set(2,32,rm(m,c.r[1],32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[2].wrapping_add(0xfffffff8),32),32); }
            if c.cond(12) { pc = 0x64d1d0; continue; }
            pc = 0x64d1b0; continue;
            }
            0x64d1b0 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64d1bc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x64d1d0 => {
            c.set(6,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            c.set(2,32,rm(m,c.r[2].wrapping_add(0xfffffff8),32));
            c.push(m,0x0u32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x1u32); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d1f5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x64f27d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64d205);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x0u32); }
            c.push(m, 0x64d21b);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064d360(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64d360;
    loop {
        match pc {
            0x64d360 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d376);
            { let args = []; c.r[0] = host.call(m, 0x63b637, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64d384; continue; }
            pc = 0x64d37d; continue;
            }
            0x64d37d => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x64d384 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064d390(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64d390;
    loop {
        match pc {
            0x64d390 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x64d3a3; continue; }
            pc = 0x64d39c; continue;
            }
            0x64d39c => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x64d3a3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d3ae);
            { let args = []; c.r[0] = host.call(m, 0x63b4a3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64d3bc; continue; }
            pc = 0x64d3b5; continue;
            }
            0x64d3b5 => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x64d3bc => {
            c.set(1,32,rm(m,c.r[6],32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064d410(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64d410;
    loop {
        match pc {
            0x64d410 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = 0x6d84d8u32; wm(m,a,32,0x6d84e0u32); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            c.push(m,c.get(7,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(5,32,c.r[4].wrapping_add(0x1c));
            if c.cond(4) { pc = 0x64d75f; continue; }
            pc = 0x64d432; continue;
            }
            0x64d432 => {
            c.set(0,8,rm(m,c.r[6],8));
            c.push(m,c.get(6,32));
            { let a = c.get(0,8); c.sub(a,0x25u32,8); }
            if c.cond(4) { pc = 0x64d46b; continue; }
            pc = 0x64d439; continue;
            }
            0x64d439 => {
            c.push(m, 0x64d43e);
            { let args = []; c.r[0] = host.call(m, 0x63b895, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64d74b; continue; }
            pc = 0x64d449; continue;
            }
            0x64d449 => {
            c.set(1,32,c.get(6,32));
            pc = 0x64d44b; continue;
            }
            0x64d44b => {
            c.set(7,32,rm(m,0x6d84d8u32,32));
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.r[7]; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,0x6d84d8u32,32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(2,32)); }
            if c.cond(5) { pc = 0x64d44b; continue; }
            pc = 0x64d466; continue;
            }
            0x64d466 => {
            pc = 0x64d74b; continue;
            }
            0x64d46b => {
            c.push(m, 0x64d470);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6],8); c.sub(a,0x25u32,8); }
            if c.cond(5) { pc = 0x64d4a0; continue; }
            pc = 0x64d47a; continue;
            }
            0x64d47a => {
            c.set(0,32,rm(m,0x6d84d8u32,32));
            { let a = c.r[0]; wm(m,a,8,0x25u32); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0]; wm(m,a,8,0x25u32); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x64d74b; continue;
            }
            0x64d4a0 => {
            c.set(2,32,rm(m,0x6d84d8u32,32));
            { let a = c.r[3].wrapping_mul(4).wrapping_add(0x6d8ce0); wm(m,a,32,c.get(5,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xffffffff),8));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d4bd);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.sub(a,0x24u32,8); }
            if c.cond(5) { pc = 0x64d52f; continue; }
            pc = 0x64d4cb; continue;
            }
            0x64d4cb => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d4d1);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x64d4d9);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            { let a = c.r[3].wrapping_add(0x6d9560); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(0,8); c.sub(a,0x24u32,8); }
            if c.cond(4) { pc = 0x64d522; continue; }
            pc = 0x64d4eb; continue;
            }
            0x64d4eb => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x64d51d; continue; }
            pc = 0x64d4ef; continue;
            }
            0x64d4ef => {
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x64d4f8);
            { let args = []; c.r[0] = host.call(m, 0x63b806, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64d51d; continue; }
            pc = 0x64d4ff; continue;
            }
            0x64d4ff => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d505);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,0x24u32,8); }
            if c.cond(5) { pc = 0x64d4eb; continue; }
            pc = 0x64d510; continue;
            }
            0x64d510 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d516);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            pc = 0x64d536; continue;
            }
            0x64d51d => {
            { let a = rm(m,c.r[6],8); c.sub(a,0x24u32,8); }
            if c.cond(5) { pc = 0x64d536; continue; }
            pc = 0x64d522; continue;
            }
            0x64d522 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d528);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            pc = 0x64d536; continue;
            }
            0x64d52f => {
            { let a = c.r[3].wrapping_add(0x6d9560); wm(m,a,8,c.get(3,8)); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            pc = 0x64d536; continue;
            }
            0x64d536 => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x64d590; continue; }
            pc = 0x64d53e; continue;
            }
            0x64d53e => {
            c.set(1,32,rm(m,0x6d84d8u32,32));
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d551);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.sub(a,0x23u32,8); }
            if c.cond(4) { pc = 0x64d57b; continue; }
            pc = 0x64d55f; continue;
            }
            0x64d55f => {
            { let a = c.get(0,8); c.sub(a,0x2au32,8); }
            if c.cond(5) { pc = 0x64d56b; continue; }
            pc = 0x64d563; continue;
            }
            0x64d563 => {
            c.set(7,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            pc = 0x64d57b; continue;
            }
            0x64d56b => {
            { let a = c.get(0,8); c.sub(a,0x2du32,8); }
            if c.cond(4) { pc = 0x64d57b; continue; }
            pc = 0x64d56f; continue;
            }
            0x64d56f => {
            { let a = c.get(0,8); c.sub(a,0x2bu32,8); }
            if c.cond(4) { pc = 0x64d57b; continue; }
            pc = 0x64d573; continue;
            }
            0x64d573 => {
            { let a = c.get(0,8); c.sub(a,0x30u32,8); }
            if c.cond(4) { pc = 0x64d57b; continue; }
            pc = 0x64d577; continue;
            }
            0x64d577 => {
            { let a = c.get(0,8); c.sub(a,0x20u32,8); }
            if c.cond(5) { pc = 0x64d58c; continue; }
            pc = 0x64d57b; continue;
            }
            0x64d57b => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d581);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x64d53e; continue; }
            pc = 0x64d58c; continue;
            }
            0x64d58c => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x64d5da; continue; }
            pc = 0x64d590; continue;
            }
            0x64d590 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d596);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x64d5da; continue; }
            pc = 0x64d59f; continue;
            }
            0x64d59f => {
            c.set(2,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(2,32));
            c.push(m, 0x64d5a8);
            { let args = []; c.r[0] = host.call(m, 0x63b806, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64d5da; continue; }
            pc = 0x64d5af; continue;
            }
            0x64d5af => {
            c.set(1,32,rm(m,0x6d84d8u32,32));
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d5c4);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x64d5cf);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x64d59f; continue; }
            pc = 0x64d5da; continue;
            }
            0x64d5da => {
            { let a = rm(m,c.r[6],8); c.sub(a,0x2eu32,8); }
            if c.cond(5) { pc = 0x64d680; continue; }
            pc = 0x64d5e3; continue;
            }
            0x64d5e3 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d5e9);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d5ff);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.sub(a,0x2au32,8); }
            c.push(m,c.get(6,32));
            if c.cond(5) { pc = 0x64d637; continue; }
            pc = 0x64d60e; continue;
            }
            0x64d60e => {
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            c.push(m, 0x64d616);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6d84d8u32,32));
            c.set(6,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d62d);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x64d680; continue;
            }
            0x64d637 => {
            c.push(m, 0x64d63c);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x64d680; continue; }
            pc = 0x64d645; continue;
            }
            0x64d645 => {
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x64d64e);
            { let args = []; c.r[0] = host.call(m, 0x63b806, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64d680; continue; }
            pc = 0x64d655; continue;
            }
            0x64d655 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d65b);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6d84d8u32,32));
            c.set(6,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d672);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x64d645; continue; }
            pc = 0x64d680; continue;
            }
            0x64d680 => {
            c.push(m,0x3u32);
            c.push(m,0x6badf4u32);
            c.push(m,c.get(6,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.push(m, 0x64d68f);
            { let args = []; c.r[0] = host.call(m, 0x63b717, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64d6a0; continue; }
            pc = 0x64d696; continue;
            }
            0x64d696 => {
            { let a = c.get(6,32); let r = c.add(a,0x3u32,32); c.set(6,32,r); }
            c.set(7,32,0x40000u32);
            pc = 0x64d710; continue;
            }
            0x64d6a0 => {
            c.set(0,32,sx(rm(m,c.r[6],8),8)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,0xffffffbau32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x26u32,32); }
            if c.cond(7) { pc = 0x64d710; continue; }
            pc = 0x64d6ab; continue;
            }
            0x64d6ab => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x64d898),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x64d888)); continue;
            }
            0x64d6ba => {
            c.push(m,c.get(6,32));
            c.set(7,32,0x10000u32);
            c.push(m, 0x64d6c5);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            pc = 0x64d6fd; continue;
            }
            0x64d6d2 => {
            c.push(m,c.get(6,32));
            c.set(7,32,0x20000u32);
            c.push(m, 0x64d6dd);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6d84d8u32,32));
            c.set(6,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            pc = 0x64d6fd; continue;
            }
            0x64d6eb => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d6f1);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6d84d8u32,32));
            c.set(6,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            pc = 0x64d6fd; continue;
            }
            0x64d6fd => {
            c.set(0,32,rm(m,0x6d84d8u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d708);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = 0x6d84d8u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x64d710; continue;
            }
            0x64d710 => {
            c.set(0,32,sx(rm(m,c.r[6],8),8)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.or(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x10063u32,32); }
            if c.cond(15) { pc = 0x64d7dd; continue; }
            pc = 0x64d720; continue;
            }
            0x64d720 => {
            if c.cond(4) { pc = 0x64d748; continue; }
            pc = 0x64d722; continue;
            }
            0x64d722 => {
            { let a = c.get(0,32); c.sub(a,0x73u32,32); }
            if c.cond(15) { pc = 0x64d7ba; continue; }
            pc = 0x64d72b; continue;
            }
            0x64d72b => {
            if c.cond(4) { pc = 0x64d7c6; continue; }
            pc = 0x64d731; continue;
            }
            0x64d731 => {
            { let a = c.get(0,32); c.sub(a,0x43u32,32); }
            if c.cond(4) { pc = 0x64d748; continue; }
            pc = 0x64d736; continue;
            }
            0x64d736 => {
            { let a = c.get(0,32); c.sub(a,0x53u32,32); }
            if c.cond(4) { pc = 0x64d80a; continue; }
            pc = 0x64d73f; continue;
            }
            0x64d73f => {
            { let a = c.get(0,32); c.sub(a,0x63u32,32); }
            if c.cond(5) { pc = 0x64d832; continue; }
            pc = 0x64d748; continue;
            }
            0x64d748 => {
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            pc = 0x64d74b; continue;
            }
            0x64d74b => {
            c.push(m,c.get(6,32));
            c.push(m, 0x64d751);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[6],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x64d432; continue; }
            pc = 0x64d75f; continue;
            }
            0x64d75f => {
            c.set(0,32,rm(m,0x6d84d8u32,32));
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            { let a = c.r[0]; wm(m,a,8,0x0u32); }
            { let a = c.r[3].wrapping_mul(4).wrapping_add(0x6d8ce0); wm(m,a,32,c.get(5,32)); }
            if c.cond(14) { pc = 0x64d864; continue; }
            pc = 0x64d777; continue;
            }
            0x64d777 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(6,32,0x6d8d60u32);
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x64d7ab; continue; }
            pc = 0x64d782; continue;
            }
            0x64d782 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x6d9560),8));
            c.set(0,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6d8ce0),32));
            c.set(1,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6d8ce4),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x64d7a6; continue; }
            pc = 0x64d79c; continue;
            }
            0x64d79c => {
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.r[6]; wm(m,a,8,c.get(2,8)); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x64d79c; continue; }
            pc = 0x64d7a6; continue;
            }
            0x64d7a6 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x64d782; continue; }
            pc = 0x64d7ab; continue;
            }
            0x64d7ab => {
            c.push(m,0x6d8d60u32);
            c.push(m,0x6d84e0u32);
            pc = 0x64d86e; continue;
            }
            0x64d7ba => {
            { let a = c.get(0,32); let r = c.sub(a,0x10043u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x64d748; continue; }
            pc = 0x64d7c1; continue;
            }
            0x64d7c1 => {
            { let a = c.get(0,32); let r = c.sub(a,0x10u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x64d832; continue; }
            pc = 0x64d7c6; continue;
            }
            0x64d7c6 => {
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64d74b; continue; }
            pc = 0x64d7d4; continue;
            }
            0x64d7d4 => {
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a29cu32,32); c.push(m, 0x64d7db); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64d821; continue;
            }
            0x64d7dd => {
            { let a = c.get(0,32); c.sub(a,0x20053u32,32); }
            if c.cond(15) { pc = 0x64d7fa; continue; }
            pc = 0x64d7e4; continue;
            }
            0x64d7e4 => {
            if c.cond(4) { pc = 0x64d80a; continue; }
            pc = 0x64d7e6; continue;
            }
            0x64d7e6 => {
            { let a = c.get(0,32); c.sub(a,0x10073u32,32); }
            if c.cond(4) { pc = 0x64d7c6; continue; }
            pc = 0x64d7ed; continue;
            }
            0x64d7ed => {
            { let a = c.get(0,32); c.sub(a,0x20043u32,32); }
            if c.cond(4) { pc = 0x64d748; continue; }
            pc = 0x64d7f8; continue;
            }
            0x64d7f8 => {
            pc = 0x64d832; continue;
            }
            0x64d7fa => {
            { let a = c.get(0,32); let r = c.sub(a,0x20063u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x64d748; continue; }
            pc = 0x64d805; continue;
            }
            0x64d805 => {
            { let a = c.get(0,32); let r = c.sub(a,0x10u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x64d832; continue; }
            pc = 0x64d80a; continue;
            }
            0x64d80a => {
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64d74b; continue; }
            pc = 0x64d818; continue;
            }
            0x64d818 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x64d81e);
            { let args = []; c.r[0] = host.call(m, 0x63b6fa, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x64d821; continue;
            }
            0x64d821 => {
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(12) { pc = 0x64d74b; continue; }
            pc = 0x64d82a; continue;
            }
            0x64d82a => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64d74b; continue; }
            pc = 0x64d832; continue;
            }
            0x64d832 => {
            c.set(0,32,sx(rm(m,c.r[6],8),8)&0xffffffffu32);
            { let a = c.get(0,32); let r = c.add(a,0xffffffb9u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x31u32,32); }
            if c.cond(7) { pc = 0x64d74b; continue; }
            pc = 0x64d841; continue;
            }
            0x64d841 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x64d8d0),8));
            pc = m.r32(c.r[2].wrapping_mul(4).wrapping_add(0x64d8c0)); continue;
            }
            0x64d850 => {
            { let a = c.get(7,32); c.and(a,0x40000u32,32); }
            if c.cond(4) { pc = 0x64d748; continue; }
            pc = 0x64d85c; continue;
            }
            0x64d85c => {
            { let a = c.get(5,32); let r = c.add(a,0x8u32,32); c.set(5,32,r); }
            pc = 0x64d74b; continue;
            }
            0x64d864 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x64d86e; continue;
            }
            0x64d86e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64d878);
            { let args = []; c.r[0] = host.call(m, 0x63b6a9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,0x1u32);
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

pub fn f_0064e1e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e1e0;
    loop {
        match pc {
            0x64e1e0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.get(1,32));
            c.push(m, 0x64e1ea);
            { let args = []; c.r[0] = host.call(m, 0x64f1fc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[3],32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x64e22c; continue; }
            pc = 0x64e1f4; continue;
            }
            0x64e1f4 => {
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x64e1fd);
            { let args = []; c.r[0] = host.call(m, 0x63b9e6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64e20c; continue; }
            pc = 0x64e204; continue;
            }
            0x64e204 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x64e20e; continue; }
            pc = 0x64e208; continue;
            }
            0x64e208 => {
            c.set(6,32,c.get(7,32));
            pc = 0x64e20e; continue;
            }
            0x64e20c => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x64e20e; continue;
            }
            0x64e20e => {
            c.push(m,c.get(7,32));
            c.push(m, 0x64e214);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x64e1f4; continue; }
            pc = 0x64e21f; continue;
            }
            0x64e21f => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x64e22c; continue; }
            pc = 0x64e223; continue;
            }
            0x64e223 => {
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.r[0].wrapping_add(0xfffffff8); wm(m,a,32,c.get(6,32)); }
            pc = 0x64e22c; continue;
            }
            0x64e22c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e2f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e2f0;
    loop {
        match pc {
            0x64e2f0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.get(1,32));
            c.push(m, 0x64e2f9);
            { let args = []; c.r[0] = host.call(m, 0x64f1fc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[3],32));
            c.set(0,32,sx(rm(m,c.r[7],8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x64e304);
            { let args = []; c.r[0] = host.call(m, 0x63b9e6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64e323; continue; }
            pc = 0x64e30b; continue;
            }
            0x64e30b => {
            c.push(m,c.get(7,32));
            c.push(m, 0x64e311);
            { let args = []; c.r[0] = host.call(m, 0x63b48c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            c.set(1,32,sx(rm(m,c.r[7],8),8)&0xffffffffu32);
            c.push(m,c.get(1,32));
            c.push(m, 0x64e31c);
            { let args = []; c.r[0] = host.call(m, 0x63b9e6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64e30b; continue; }
            pc = 0x64e323; continue;
            }
            0x64e323 => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64e345; continue; }
            pc = 0x64e329; continue;
            }
            0x64e329 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(2,32,c.r[6].wrapping_add(0x1));
            c.push(m,c.get(2,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64e33c);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[0].wrapping_add(0xfffffff8); wm(m,a,32,c.get(6,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x64e345; continue;
            }
            0x64e345 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e603(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e603;
    loop {
        match pc {
            0x64e603 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[0]; wm(m,a,32,0x67cf80u32); }
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e642(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e642;
    loop {
        match pc {
            0x64e642 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x64e65c);
            { let args = []; c.r[0] = host.call(m, 0x64ed07, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e68b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e68b;
    loop {
        match pc {
            0x64e68b => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = rm(m,c.r[6].wrapping_add(0x10),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x64e6c3; continue; }
            pc = 0x64e694; continue;
            }
            0x64e694 => {
            c.push(m,0xcu32);
            c.set(0,32,c.r[6].wrapping_add(0x14));
            { let v = rm(m,c.r[6].wrapping_add(0x18),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x64e6a2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64ece7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xfffffff4));
            if c.cond(12) { pc = 0x64e6c3; continue; }
            pc = 0x64e6b4; continue;
            }
            0x64e6b4 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x64e6b5; continue;
            }
            0x64e6b5 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.sub(a,0xcu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x64e6b5; continue; }
            pc = 0x64e6c3; continue;
            }
            0x64e6c3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.inc(a,32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = rm(m,c.r[0].wrapping_add(0x8),32); let r = c.and(a,0x0u32,32); { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,r); } }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e6e3(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e6e3;
    loop {
        match pc {
            0x64e6e3 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x10),32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = rm(m,c.r[1].wrapping_add(0xc),32); let r = c.dec(a,32); { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,r); } }
            { let a = c.r[1].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x64e6f9; continue; }
            pc = 0x64e6f4; continue;
            }
            0x64e6f4 => {
            c.push(m, 0x64e6f9);
            { let args = []; c.r[0] = host.call(m, 0x64e642, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x64e6f9; continue;
            }
            0x64e6f9 => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e6fc(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e6fc;
    loop {
        match pc {
            0x64e6fc => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x0u32);
            { let v = rm(m,c.r[6].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x64e709);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e68b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x64e71b; continue; }
            pc = 0x64e717; continue;
            }
            0x64e717 => {
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            pc = 0x64e71e; continue;
            }
            0x64e71b => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x64e71e; continue;
            }
            0x64e71e => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e749(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e749;
    loop {
        match pc {
            0x64e749 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x64e75a; continue; }
            pc = 0x64e753; continue;
            }
            0x64e753 => {
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x64e761; continue;
            }
            0x64e75a => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(6,32,rm(m,c.r[0],32));
            { let a = c.r[2]; wm(m,a,32,c.get(6,32)); }
            pc = 0x64e761; continue;
            }
            0x64e761 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x64e76e; continue; }
            pc = 0x64e766; continue;
            }
            0x64e766 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            pc = 0x64e776; continue;
            }
            0x64e76e => {
            c.set(2,32,rm(m,c.r[0],32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            pc = 0x64e776; continue;
            }
            0x64e776 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x64e77c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64e6e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e780(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e780;
    loop {
        match pc {
            0x64e780 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1].wrapping_add(0xc),32),32); }
            if c.cond(13) { pc = 0x64e7a1; continue; }
            pc = 0x64e789; continue;
            }
            0x64e789 => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x64e7a1; continue; }
            pc = 0x64e78d; continue;
            }
            0x64e78d => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x64e7a3; continue; }
            pc = 0x64e797; continue;
            }
            0x64e797 => {
            c.set(1,32,c.r[2].wrapping_add(0x1));
            pc = 0x64e79a; continue;
            }
            0x64e79a => {
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x64e79a; continue; }
            pc = 0x64e79f; continue;
            }
            0x64e79f => {
            pc = 0x64e7a3; continue;
            }
            0x64e7a1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64e7a3; continue;
            }
            0x64e7a3 => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e7a6(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e7a6;
    loop {
        match pc {
            0x64e7a6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64e7b3; continue; }
            pc = 0x64e7ae; continue;
            }
            0x64e7ae => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            pc = 0x64e7b5; continue;
            }
            0x64e7b3 => {
            c.set(0,32,rm(m,c.r[0],32));
            pc = 0x64e7b5; continue;
            }
            0x64e7b5 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64e7c4; continue; }
            pc = 0x64e7b9; continue;
            }
            0x64e7b9 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x64e7b3; continue; }
            pc = 0x64e7c2; continue;
            }
            0x64e7c2 => {
            pc = 0x64e7c6; continue;
            }
            0x64e7c4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64e7c6; continue;
            }
            0x64e7c6 => {
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e7c9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e7c9;
    loop {
        match pc {
            0x64e7c9 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,0x67d0f0u32); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e83a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e83a;
    loop {
        match pc {
            0x64e83a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x64e85b; continue; }
            pc = 0x64e845; continue;
            }
            0x64e845 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.r[0].wrapping_add(0x1));
            pc = 0x64e849; continue;
            }
            0x64e849 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m, 0x64e852);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),32); let r = c.add(a,0x4u32,32); { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            if c.cond(5) { pc = 0x64e849; continue; }
            pc = 0x64e85a; continue;
            }
            0x64e85a => {
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x64e85b; continue;
            }
            0x64e85b => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e85c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e85c;
    loop {
        match pc {
            0x64e85c => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            if c.cond(4) { pc = 0x64e86f; continue; }
            pc = 0x64e86c; continue;
            }
            0x64e86c => {
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x64e86f; continue;
            }
            0x64e86f => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x64e89c; continue; }
            pc = 0x64e878; continue;
            }
            0x64e878 => {
            { let v = rm(m,c.r[6].wrapping_add(0x8),32); c.push(m, v); }
            { let v = rm(m,c.r[6].wrapping_add(0x4),32); c.push(m, v); }
            c.push(m, 0x64e883);
            { let args = []; c.r[0] = host.call(m, 0x64e83a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[6].wrapping_add(0x4),32); c.push(m, v); }
            c.push(m, 0x64e88b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            pc = 0x64e972; continue;
            }
            0x64e89c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x64e8c6; continue; }
            pc = 0x64e8a3; continue;
            }
            0x64e8a3 => {
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x64e8ae);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x64e8b8);
            { let args = []; c.r[0] = host.call(m, 0x64e979, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            pc = 0x64e8be; continue;
            }
            0x64e8be => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            pc = 0x64e972; continue;
            }
            0x64e8c6 => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(15) { pc = 0x64e8f6; continue; }
            pc = 0x64e8cd; continue;
            }
            0x64e8cd => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(13) { pc = 0x64e8e4; continue; }
            pc = 0x64e8d4; continue;
            }
            0x64e8d4 => {
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64e8e2);
            { let args = []; c.r[0] = host.call(m, 0x64e979, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x64e8f2; continue;
            }
            0x64e8e4 => {
            if c.cond(14) { pc = 0x64e8be; continue; }
            pc = 0x64e8e6; continue;
            }
            0x64e8e6 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)));
            c.push(m,c.get(0,32));
            c.push(m, 0x64e8f2);
            { let args = []; c.r[0] = host.call(m, 0x64e83a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x64e8f2; continue;
            }
            0x64e8f2 => {
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            pc = 0x64e8be; continue;
            }
            0x64e8f6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64e920; continue; }
            pc = 0x64e8fd; continue;
            }
            0x64e8fd => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,0x8u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let v = c.pop(m); c.set(1,32,v); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(12) { pc = 0x64e91d; continue; }
            pc = 0x64e90b; continue;
            }
            0x64e90b => {
            c.set(1,32,0x400u32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x64e918; continue; }
            pc = 0x64e914; continue;
            }
            0x64e914 => {
            c.set(0,32,c.get(1,32));
            pc = 0x64e920; continue;
            }
            0x64e918 => {
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(13) { pc = 0x64e920; continue; }
            pc = 0x64e91d; continue;
            }
            0x64e91d => {
            c.push(m,0x4u32);
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x64e920; continue;
            }
            0x64e920 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x64e92c; continue; }
            pc = 0x64e929; continue;
            }
            0x64e929 => {
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            pc = 0x64e92c; continue;
            }
            0x64e92c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x64e938);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[6].wrapping_add(0x4),32); c.push(m, v); }
            c.push(m,c.get(3,32));
            c.push(m, 0x64e94a);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[3].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64e95b);
            { let args = []; c.r[0] = host.call(m, 0x64e979, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[6].wrapping_add(0x4),32); c.push(m, v); }
            c.push(m, 0x64e963);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x64e972; continue;
            }
            0x64e972 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e979(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e979;
    loop {
        match pc {
            0x64e979 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x64e9a4; continue; }
            pc = 0x64e984; continue;
            }
            0x64e984 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.r[0].wrapping_add(0x1));
            pc = 0x64e988; continue;
            }
            0x64e988 => {
            c.push(m,0x4u32);
            c.push(m,0x6c7cccu32);
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            c.push(m, 0x64e998);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); let r = c.add(a,0x4u32,32); { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,r); } }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            if c.cond(5) { pc = 0x64e988; continue; }
            pc = 0x64e9a3; continue;
            }
            0x64e9a3 => {
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x64e9a4; continue;
            }
            0x64e9a4 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064e9a5(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64e9a5;
    loop {
        match pc {
            0x64e9a5 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[7].wrapping_add(0x8),32),32); }
            if c.cond(12) { pc = 0x64e9bd; continue; }
            pc = 0x64e9b2; continue;
            }
            0x64e9b2 => {
            c.set(0,32,c.r[6].wrapping_add(0x1));
            c.push(m,0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x64e9bd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e85c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x64e9bd; continue;
            }
            0x64e9bd => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let v = rm(m,c.r[4].wrapping_add(0x10),32); c.push(m, v); }
            c.set(1,32,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)));
            c.push(m, 0x64e9cc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ea47(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ea47;
    loop {
        match pc {
            0x64ea47 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x64ea5b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e9d1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.get(7,32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64ea7a; continue; }
            pc = 0x64ea62; continue;
            }
            0x64ea62 => {
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x64ea66; continue;
            }
            0x64ea66 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m, 0x64ea77);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x64ea66; continue; }
            pc = 0x64ea7a; continue;
            }
            0x64ea7a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064eae9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64eae9;
    loop {
        match pc {
            0x64eae9 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[0]; wm(m,a,32,0x67d10cu32); }
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064eb86(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64eb86;
    loop {
        match pc {
            0x64eb86 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[6].wrapping_add(0x10),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x64ebbf; continue; }
            pc = 0x64eb90; continue;
            }
            0x64eb90 => {
            c.push(m,0xcu32);
            c.set(0,32,c.r[6].wrapping_add(0x14));
            { let v = rm(m,c.r[6].wrapping_add(0x18),32); c.push(m, v); }
            c.push(m,c.get(0,32));
            c.push(m, 0x64eb9e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64ece7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xfffffff4));
            if c.cond(12) { pc = 0x64ebbf; continue; }
            pc = 0x64ebb0; continue;
            }
            0x64ebb0 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            pc = 0x64ebb1; continue;
            }
            0x64ebb1 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.sub(a,0xcu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x64ebb1; continue; }
            pc = 0x64ebbf; continue;
            }
            0x64ebbf => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,0x4u32);
            c.push(m,0x6c7cccu32);
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.inc(a,32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            c.set(0,32,c.r[7].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m, 0x64ebe7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ebf1(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ebf1;
    loop {
        match pc {
            0x64ebf1 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x0u32);
            { let v = rm(m,c.r[6].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x64ebff);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64eb86, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = rm(m,c.r[4].wrapping_add(0xc),32); c.push(m, v); }
            c.set(7,32,c.get(0,32));
            c.set(1,32,c.r[7].wrapping_add(0x8));
            c.push(m, 0x64ec0d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64ec18; continue; }
            pc = 0x64ec14; continue;
            }
            0x64ec14 => {
            { let a = c.r[0]; wm(m,a,32,c.get(7,32)); }
            pc = 0x64ec1b; continue;
            }
            0x64ec18 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            pc = 0x64ec1b; continue;
            }
            0x64ec1b => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ec25(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ec25;
    loop {
        match pc {
            0x64ec25 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64ec33; continue; }
            pc = 0x64ec2e; continue;
            }
            0x64ec2e => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            pc = 0x64ec35; continue;
            }
            0x64ec33 => {
            c.set(6,32,rm(m,c.r[0],32));
            pc = 0x64ec35; continue;
            }
            0x64ec35 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x64ec54; continue; }
            pc = 0x64ec39; continue;
            }
            0x64ec39 => {
            { let v = rm(m,c.r[4].wrapping_add(0x8),32); c.push(m, v); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64ec46);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(4) { pc = 0x64ec50; continue; }
            pc = 0x64ec4c; continue;
            }
            0x64ec4c => {
            c.set(6,32,rm(m,c.r[6],32));
            pc = 0x64ec35; continue;
            }
            0x64ec50 => {
            c.set(0,32,c.get(6,32));
            pc = 0x64ec56; continue;
            }
            0x64ec54 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64ec56; continue;
            }
            0x64ec56 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ece7(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ece7;
    loop {
        match pc {
            0x64ece7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0xc),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x64ecf9);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ed07(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ed07;
    loop {
        match pc {
            0x64ed07 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x64ed1c; continue; }
            pc = 0x64ed0b; continue;
            }
            0x64ed0b => {
            c.push(m,c.get(6,32));
            pc = 0x64ed0c; continue;
            }
            0x64ed0c => {
            c.set(6,32,rm(m,c.r[1],32));
            c.push(m,c.get(1,32));
            c.push(m, 0x64ed14);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.set(1,32,c.get(6,32));
            if c.cond(5) { pc = 0x64ed0c; continue; }
            pc = 0x64ed1b; continue;
            }
            0x64ed1b => {
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x64ed1c; continue;
            }
            0x64ed1c => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064ee91(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64ee91;
    loop {
        match pc {
            0x64ee91 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(4) { pc = 0x64eebd; continue; }
            pc = 0x64ee9a; continue;
            }
            0x64ee9a => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let b = rm(m,c.r[1].wrapping_add(0x8),32); c.div32(b); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)),32));
            pc = 0x64eeab; continue;
            }
            0x64eeab => {
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x64eebd; continue; }
            pc = 0x64eeaf; continue;
            }
            0x64eeaf => {
            { let a = rm(m,c.r[2].wrapping_add(0x4),32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x64eeb8; continue; }
            pc = 0x64eeb4; continue;
            }
            0x64eeb4 => {
            c.set(2,32,rm(m,c.r[2],32));
            pc = 0x64eeab; continue;
            }
            0x64eeb8 => {
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            pc = 0x64eebf; continue;
            }
            0x64eebd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x64eebf; continue;
            }
            0x64eebf => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064efc8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64efc8;
    loop {
        match pc {
            0x64efc8 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[1],32));
            { let a = rm(m,c.r[0].wrapping_add(0xfffffff4),32); c.sub(a,0x0u32,32); }
            if c.cond(12) { pc = 0x64efe5; continue; }
            pc = 0x64efd7; continue;
            }
            0x64efd7 => {
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0xfffffff4u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a0dcu32,32); c.push(m, 0x64efe3); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64eff5; continue;
            }
            0x64efe5 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            { let v = rm(m,c.r[1],32); c.push(m, v); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64eff5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64eff5; continue;
            }
            0x64eff5 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f0c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f0c0;
    loop {
        match pc {
            0x64f0c0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x64f0d5; continue; }
            pc = 0x64f0cc; continue;
            }
            0x64f0cc => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            pc = 0x64f13d; continue;
            }
            0x64f0d5 => {
            c.push(m,c.get(7,32));
            c.push(m,0x40u32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x64f0ec; continue; }
            pc = 0x64f0dd; continue;
            }
            0x64f0dd => {
            c.set(1,32,0x6ee1a8u32);
            pc = 0x64f0e2; continue;
            }
            0x64f0e2 => {
            c.push(m, 0x64f0e7);
            { let args = []; c.r[0] = host.call(m, 0x6394fd, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            pc = 0x64f129; continue;
            }
            0x64f0ec => {
            c.set(7,32,0x80u32);
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x64f0fc; continue; }
            pc = 0x64f0f5; continue;
            }
            0x64f0f5 => {
            c.set(1,32,0x6ee180u32);
            pc = 0x64f0e2; continue;
            }
            0x64f0fc => {
            c.set(7,32,0x100u32);
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x64f10c; continue; }
            pc = 0x64f105; continue;
            }
            0x64f105 => {
            c.set(1,32,0x6ee158u32);
            pc = 0x64f0e2; continue;
            }
            0x64f10c => {
            c.set(7,32,0x200u32);
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x64f11c; continue; }
            pc = 0x64f115; continue;
            }
            0x64f115 => {
            c.set(1,32,0x6ee130u32);
            pc = 0x64f0e2; continue;
            }
            0x64f11c => {
            c.set(0,32,c.r[6].wrapping_add(0xd));
            c.push(m,c.get(0,32));
            c.push(m, 0x64f125);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            pc = 0x64f129; continue;
            }
            0x64f129 => {
            { let a = c.r[0]; wm(m,a,32,0x1u32); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0xc),8); let r = c.and(a,0x0u32,8); { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0xc); wm(m,a,8,r); } }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x64f13d; continue;
            }
            0x64f13d => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f18a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f18a;
    loop {
        match pc {
            0x64f18a => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[0].wrapping_add(0xfffffff4));
            { let a = c.get(1,32); c.sub(a,rm(m,0x6c7cd0u32,32),32); }
            if c.cond(4) { pc = 0x64f1b9; continue; }
            pc = 0x64f19a; continue;
            }
            0x64f19a => {
            { let a = c.get(0,32); let r = c.add(a,0xfffffff4u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a0e0u32,32); c.push(m, 0x64f1a4); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x64f1b2; continue; }
            pc = 0x64f1a8; continue;
            }
            0x64f1a8 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.sub(a,0xcu32,32); c.set(1,32,r); }
            c.push(m, 0x64f1b2);
            { let args = []; c.r[0] = host.call(m, 0x64f142, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x64f1b2; continue;
            }
            0x64f1b2 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            pc = 0x64f1b9; continue;
            }
            0x64f1b9 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f1bb(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f1bb;
    loop {
        match pc {
            0x64f1bb => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6c7cd0u32,32),32); }
            if c.cond(4) { pc = 0x64f1da; continue; }
            pc = 0x64f1c8; continue;
            }
            0x64f1c8 => {
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a0e0u32,32); c.push(m, 0x64f1cf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x64f1da; continue; }
            pc = 0x64f1d3; continue;
            }
            0x64f1d3 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64f1da);
            { let args = []; c.r[0] = host.call(m, 0x64f142, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x64f1da; continue;
            }
            0x64f1da => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f1de(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f1de;
    loop {
        match pc {
            0x64f1de => {
            c.set(0,32,rm(m,c.r[1],32));
            { let a = rm(m,c.r[0].wrapping_add(0xfffffff8),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x64f1fb; continue; }
            pc = 0x64f1e6; continue;
            }
            0x64f1e6 => {
            { let a = rm(m,c.r[0].wrapping_add(0xfffffff4),32); c.sub(a,0x0u32,32); }
            if c.cond(12) { pc = 0x64f1f1; continue; }
            pc = 0x64f1ec; continue;
            }
            0x64f1ec => {
            { let args = []; c.r[0] = host.call(m, 0x64f18a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x64f1f1 => {
            c.push(m,0x6ee1d0u32);
            c.push(m, 0x64f1fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64f1fb; continue;
            }
            0x64f1fb => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f1fc(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f1fc;
    loop {
        match pc {
            0x64f1fc => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(6,32,rm(m,c.r[7],32));
            { let a = rm(m,c.r[6].wrapping_add(0xfffffff4),32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x64f227; continue; }
            pc = 0x64f208; continue;
            }
            0x64f208 => {
            c.push(m, 0x64f20d);
            { let args = []; c.r[0] = host.call(m, 0x64f18a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[6].wrapping_add(0xfffffff8),32); c.push(m, v); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x64f217);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let v = rm(m,c.r[7],32); c.push(m, v); }
            c.push(m, 0x64f224);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x64f227; continue;
            }
            0x64f227 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f22a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f22a;
    loop {
        match pc {
            0x64f22a => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = rm(m,c.r[0].wrapping_add(0xfffffff4),32); c.sub(a,0x1u32,32); }
            if c.cond(15) { pc = 0x64f23f; continue; }
            pc = 0x64f23a; continue;
            }
            0x64f23a => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffffc),32),32); }
            if c.cond(14) { pc = 0x64f24e; continue; }
            pc = 0x64f23f; continue;
            }
            0x64f23f => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64f246);
            { let args = []; c.r[0] = host.call(m, 0x64f18a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64f24e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64f24e; continue;
            }
            0x64f24e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f27d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f27d;
    loop {
        match pc {
            0x64f27d => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x14),32));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x64f29d; continue; }
            pc = 0x64f290; continue;
            }
            0x64f290 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x64f2bb; continue;
            }
            0x64f29d => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64f2a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7],32));
            c.push(m,c.get(3,32));
            { let a = c.get(0,32); let r = c.add(a,rm(m,c.r[5].wrapping_add(0x10),32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[6],32); c.push(m, v); }
            c.push(m, 0x64f2b7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x64f2bb; continue;
            }
            0x64f2bb => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f313(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f313;
    loop {
        match pc {
            0x64f313 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x64f321);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f22a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            { let v = rm(m,c.r[6],32); c.push(m, v); }
            c.push(m, 0x64f32d);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[0].wrapping_add(0xfffffff8); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[6],32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[7]),8); let r = c.and(a,0x0u32,8); { let a = c.r[0].wrapping_add(c.r[7]); wm(m,a,8,r); } }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f340(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f340;
    loop {
        match pc {
            0x64f340 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[6],32));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64f389; continue; }
            pc = 0x64f350; continue;
            }
            0x64f350 => {
            { let a = rm(m,c.r[1].wrapping_add(0xfffffff4),32); c.sub(a,0x0u32,32); }
            if c.cond(13) { pc = 0x64f361; continue; }
            pc = 0x64f356; continue;
            }
            0x64f356 => {
            { let a = c.get(1,32); let r = c.add(a,0xfffffff4u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,rm(m,0x6c7cd0u32,32),32); }
            if c.cond(5) { pc = 0x64f367; continue; }
            pc = 0x64f361; continue;
            }
            0x64f361 => {
            { let a = rm(m,c.r[0].wrapping_add(0xfffffff4),32); c.sub(a,0x0u32,32); }
            if c.cond(13) { pc = 0x64f374; continue; }
            pc = 0x64f367; continue;
            }
            0x64f367 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let v = rm(m,c.r[0].wrapping_add(0xfffffff8),32); c.push(m, v); }
            c.push(m, 0x64f372);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64f313, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x64f389; continue;
            }
            0x64f374 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x64f37b);
            { let args = []; c.r[0] = host.call(m, 0x64f18a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[6]; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0xfffffff4u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a0dcu32,32); c.push(m, 0x64f389); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64f389; continue;
            }
            0x64f389 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f3f8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f3f8;
    loop {
        match pc {
            0x64f3f8 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,c.r[7].wrapping_add(c.r[3]));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x64f430; continue; }
            pc = 0x64f40c; continue;
            }
            0x64f40c => {
            c.push(m,c.get(0,32));
            c.push(m, 0x64f412);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            { let v = rm(m,c.r[6],32); c.push(m, v); }
            c.push(m, 0x64f41e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            c.push(m,c.get(3,32));
            { let v = rm(m,c.r[4].wrapping_add(0x2c),32); c.push(m, v); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x64f42d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x64f430; continue;
            }
            0x64f430 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f584(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f584;
    loop {
        match pc {
            0x64f584 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            c.set(6,32,c.get(1,32));
            if c.cond(4) { pc = 0x64f5dd; continue; }
            pc = 0x64f591; continue;
            }
            0x64f591 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = rm(m,c.r[0].wrapping_add(0xfffffff4),32); c.sub(a,0x1u32,32); }
            c.set(3,32,c.r[0].wrapping_add(0xfffffff4));
            if c.cond(15) { pc = 0x64f5c7; continue; }
            pc = 0x64f59c; continue;
            }
            0x64f59c => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[7]));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffffc),32),32); }
            if c.cond(15) { pc = 0x64f5c7; continue; }
            pc = 0x64f5a7; continue;
            }
            0x64f5a7 => {
            c.push(m,c.get(7,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let v = rm(m,c.r[4].wrapping_add(0x18),32); c.push(m, v); }
            c.push(m,c.get(1,32));
            c.push(m, 0x64f5b4);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0xfffffff8),32); let r = c.add(a,c.get(7,32),32); { let a = c.r[0].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]),8); let r = c.and(a,0x0u32,8); { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,8,r); } }
            pc = 0x64f5dd; continue;
            }
            0x64f5c7 => {
            { let v = rm(m,c.r[4].wrapping_add(0x14),32); c.push(m, v); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            { let v = rm(m,c.r[0].wrapping_add(0xfffffff8),32); c.push(m, v); }
            c.push(m, 0x64f5d7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x64f3f8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(3,32));
            c.push(m, 0x64f5dd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f1bb, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x64f5dd; continue;
            }
            0x64f5dd => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f60a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f60a;
    loop {
        match pc {
            0x64f60a => {
            c.set(0,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0x1u32);
            c.push(m, 0x64f619);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64f584, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f6ce(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f6ce;
    loop {
        match pc {
            0x64f6ce => {
            c.push(m,0x0u32);
            { let v = rm(m,c.r[4].wrapping_add(0x8),32); c.push(m, v); }
            c.push(m, 0x64f6d9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64f6dc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f6dc(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f6dc;
    loop {
        match pc {
            0x64f6dc => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffff8),32),32); }
            if c.cond(13) { pc = 0x64f6fe; continue; }
            pc = 0x64f6ea; continue;
            }
            0x64f6ea => {
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x64f6f8);
            { let args = []; c.r[0] = host.call(m, 0x63b94f, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let v = c.pop(m); c.set(1,32,v); }
            if c.cond(5) { pc = 0x64f703; continue; }
            pc = 0x64f6fe; continue;
            }
            0x64f6fe => {
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            pc = 0x64f705; continue;
            }
            0x64f703 => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[6],32),32); c.set(0,32,r); }
            pc = 0x64f705; continue;
            }
            0x64f705 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0064f709(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x64f709;
    loop {
        match pc {
            0x64f709 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x64f711);
            { let args = []; c.r[0] = host.call(m, 0x64f1fc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[6],32); c.push(m, v); }
            c.push(m, 0x64f718);
            { let args = []; c.r[0] = host.call(m, 0x63cab7, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00650184(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x650184;
    loop {
        match pc {
            0x650184 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x65018c);
            { let args = []; c.r[0] = host.call(m, 0x652db8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x20u32);
            c.set(0,32,c.r[6].wrapping_add(0x1c));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[6]; wm(m,a,32,0x67bdf0u32); }
            c.push(m, 0x65019f);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
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

pub fn f_0065058b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65058b;
    loop {
        match pc {
            0x65058b => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x1u32);
            c.push(m, 0x650594);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x650519, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let v = rm(m,c.r[4].wrapping_add(0xc),32); c.push(m, v); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x6505a1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x655365, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x6505ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x653517, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00652db8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x652db8;
    loop {
        match pc {
            0x652db8 => {
            c.set(0,32,c.get(1,32));
            c.push(m,0x1u32);
            { let v = c.pop(m); c.set(1,32,v); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_006534f6(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6534f6;
    loop {
        match pc {
            0x6534f6 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x38),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x65350d; continue; }
            pc = 0x6534fd; continue;
            }
            0x6534fd => {
            { let v = rm(m,c.r[1].wrapping_add(0x1c),32); c.push(m, v); }
            { let target = rm(m,0x66a3fcu32,32); c.push(m, 0x653506); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.push(m, 0x65350c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65058b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x65350d => {
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0xac),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653517(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653517;
    loop {
        match pc {
            0x653517 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x653550; continue; }
            pc = 0x65351e; continue;
            }
            0x65351e => {
            { let a = rm(m,c.r[6].wrapping_add(0x38),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x653550; continue; }
            pc = 0x653524; continue;
            }
            0x653524 => {
            { let v = rm(m,c.r[6].wrapping_add(0x1c),32); c.push(m, v); }
            { let target = rm(m,0x66a344u32,32); c.push(m, 0x65352d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,c.r[0].wrapping_add(0x4));
            c.push(m, 0x65353a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ee91, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x653550; continue; }
            pc = 0x65353e; continue;
            }
            0x65353e => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x34),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x653550; continue; }
            pc = 0x653545; continue;
            }
            0x653545 => {
            c.set(2,32,rm(m,c.r[0],32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(0,32));
            { let target = rm(m,c.r[2].wrapping_add(0x8c),32); c.push(m, 0x653550); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x653550; continue;
            }
            0x653550 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653688(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653688;
    loop {
        match pc {
            0x653688 => {
            c.set(0,32,c.get(1,32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,0x1u32); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0065388d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x65388d;
    loop {
        match pc {
            0x65388d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,0x10bu32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x653a5b; continue; }
            pc = 0x65389e; continue;
            }
            0x65389e => {
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x6538a4; continue;
            }
            0x6538a4 => {
            { let a = c.get(0,32); c.sub(a,0x3eu32,32); }
            if c.cond(7) { pc = 0x65394b; continue; }
            pc = 0x6538ad; continue;
            }
            0x6538ad => {
            if c.cond(4) { pc = 0x653947; continue; }
            pc = 0x6538b3; continue;
            }
            0x6538b3 => {
            { let a = c.get(0,32); c.sub(a,0x19u32,32); }
            if c.cond(7) { pc = 0x6538df; continue; }
            pc = 0x6538b8; continue;
            }
            0x6538b8 => {
            if c.cond(4) { pc = 0x65390c; continue; }
            pc = 0x6538ba; continue;
            }
            0x6538ba => {
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(7) { pc = 0x653a7c; continue; }
            pc = 0x6538c3; continue;
            }
            0x6538c3 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x653a83)); continue;
            }
            0x6538ca => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x65398f; continue;
            }
            0x6538d1 => {
            c.push(m,0x4u32);
            pc = 0x65398e; continue;
            }
            0x6538d8 => {
            c.push(m,0x2u32);
            pc = 0x65398e; continue;
            }
            0x6538df => {
            { let a = c.get(0,32); c.sub(a,0x27u32,32); }
            if c.cond(7) { pc = 0x653930; continue; }
            pc = 0x6538e4; continue;
            }
            0x6538e4 => {
            if c.cond(4) { pc = 0x6539c7; continue; }
            pc = 0x6538ea; continue;
            }
            0x6538ea => {
            { let a = c.get(0,32); c.sub(a,0x20u32,32); }
            if c.cond(7) { pc = 0x653914; continue; }
            pc = 0x6538ef; continue;
            }
            0x6538ef => {
            if c.cond(4) { pc = 0x653910; continue; }
            pc = 0x6538f1; continue;
            }
            0x6538f1 => {
            { let a = c.get(0,32); let r = c.sub(a,0x1au32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6539c3; continue; }
            pc = 0x6538fa; continue;
            }
            0x6538fa => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x65390c; continue; }
            pc = 0x6538fd; continue;
            }
            0x6538fd => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653992; continue; }
            pc = 0x653905; continue;
            }
            0x653905 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x653a7c; continue; }
            pc = 0x65390c; continue;
            }
            0x65390c => {
            c.push(m,0x9u32);
            pc = 0x65398e; continue;
            }
            0x653910 => {
            c.push(m,0xbu32);
            pc = 0x65398e; continue;
            }
            0x653914 => {
            { let a = c.get(0,32); let r = c.sub(a,0x21u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653a23; continue; }
            pc = 0x65391d; continue;
            }
            0x65391d => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x653920; continue;
            }
            0x653920 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6538d1; continue; }
            pc = 0x653924; continue;
            }
            0x653924 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x653a7c; continue; }
            pc = 0x65392c; continue;
            }
            0x65392c => {
            c.push(m,0xeu32);
            pc = 0x65398e; continue;
            }
            0x653930 => {
            { let a = c.get(0,32); let r = c.add(a,0xffffffccu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(7) { pc = 0x653a7c; continue; }
            pc = 0x65393c; continue;
            }
            0x65393c => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x653ae7)); continue;
            }
            0x653943 => {
            c.push(m,0x3u32);
            pc = 0x65398e; continue;
            }
            0x653947 => {
            c.push(m,0x8u32);
            pc = 0x65398e; continue;
            }
            0x65394b => {
            { let a = c.get(0,32); c.sub(a,0x6bu32,32); }
            if c.cond(7) { pc = 0x653996; continue; }
            pc = 0x653950; continue;
            }
            0x653950 => {
            if c.cond(4) { pc = 0x6538d8; continue; }
            pc = 0x653952; continue;
            }
            0x653952 => {
            { let a = c.get(0,32); c.sub(a,0x47u32,32); }
            if c.cond(7) { pc = 0x653973; continue; }
            pc = 0x653957; continue;
            }
            0x653957 => {
            if c.cond(4) { pc = 0x653992; continue; }
            pc = 0x653959; continue;
            }
            0x653959 => {
            { let a = c.get(0,32); let r = c.sub(a,0x40u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653992; continue; }
            pc = 0x65395e; continue;
            }
            0x65395e => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653992; continue; }
            pc = 0x653961; continue;
            }
            0x653961 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6539c3; continue; }
            pc = 0x653964; continue;
            }
            0x653964 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x653967; continue;
            }
            0x653967 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6538d1; continue; }
            pc = 0x65396e; continue;
            }
            0x65396e => {
            pc = 0x653a52; continue;
            }
            0x653973 => {
            { let a = c.get(0,32); let r = c.sub(a,0x50u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653992; continue; }
            pc = 0x653978; continue;
            }
            0x653978 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653992; continue; }
            pc = 0x65397c; continue;
            }
            0x65397c => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x653981; continue;
            }
            0x653981 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653992; continue; }
            pc = 0x653984; continue;
            }
            0x653984 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x653a7c; continue; }
            pc = 0x65398c; continue;
            }
            0x65398c => {
            c.push(m,0xau32);
            pc = 0x65398e; continue;
            }
            0x65398e => {
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x65398f; continue;
            }
            0x65398f => {
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x653992 => {
            c.push(m,0x5u32);
            pc = 0x65398e; continue;
            }
            0x653996 => {
            c.set(1,32,0x90u32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x6539f0; continue; }
            pc = 0x65399f; continue;
            }
            0x65399f => {
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x6539a1; continue;
            }
            0x6539a1 => {
            { let a = c.get(0,32); c.sub(a,0x75u32,32); }
            if c.cond(7) { pc = 0x6539cb; continue; }
            pc = 0x6539a6; continue;
            }
            0x6539a6 => {
            if c.cond(4) { pc = 0x65398c; continue; }
            pc = 0x6539a8; continue;
            }
            0x6539a8 => {
            { let a = c.get(0,32); let r = c.sub(a,0x6cu32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653a23; continue; }
            pc = 0x6539ad; continue;
            }
            0x6539ad => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x6539b2; continue;
            }
            0x6539b2 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6539c7; continue; }
            pc = 0x6539b5; continue;
            }
            0x6539b5 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6538d1; continue; }
            pc = 0x6539bc; continue;
            }
            0x6539bc => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x653a7c; continue; }
            pc = 0x6539c3; continue;
            }
            0x6539c3 => {
            c.push(m,0x6u32);
            pc = 0x65398e; continue;
            }
            0x6539c7 => {
            c.push(m,0xdu32);
            pc = 0x65398e; continue;
            }
            0x6539cb => {
            { let a = c.get(0,32); let r = c.sub(a,0x7bu32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x6539d4; continue;
            }
            0x6539d4 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x6539db; continue;
            }
            0x6539db => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x6539e2; continue;
            }
            0x6539e2 => {
            { let a = c.get(0,32); let r = c.sub(a,0x6u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x65390c; continue; }
            pc = 0x6539eb; continue;
            }
            0x6539eb => {
            pc = 0x653905; continue;
            }
            0x6539f0 => {
            c.set(1,32,0xb6u32);
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x653a31; continue; }
            pc = 0x6539f9; continue;
            }
            0x6539f9 => {
            if c.cond(4) { pc = 0x6539c3; continue; }
            pc = 0x6539fb; continue;
            }
            0x6539fb => {
            { let a = c.get(0,32); let r = c.sub(a,0x91u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653a2a; continue; }
            pc = 0x653a02; continue;
            }
            0x653a02 => {
            { let a = c.get(0,32); let r = c.sub(a,0x9u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x653a0b; continue;
            }
            0x653a0b => {
            { let a = c.get(0,32); let r = c.sub(a,0x7u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x653a14; continue;
            }
            0x653a14 => {
            { let a = c.get(0,32); let r = c.sub(a,0x6u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653a23; continue; }
            pc = 0x653a19; continue;
            }
            0x653a19 => {
            { let a = c.get(0,32); let r = c.sub(a,0x3u32,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x653a7c; continue; }
            pc = 0x653a1e; continue;
            }
            0x653a1e => {
            pc = 0x653992; continue;
            }
            0x653a23 => {
            c.push(m,0xcu32);
            pc = 0x65398e; continue;
            }
            0x653a2a => {
            c.push(m,0x7u32);
            pc = 0x65398e; continue;
            }
            0x653a31 => {
            { let a = c.get(0,32); let r = c.sub(a,0xb7u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653992; continue; }
            pc = 0x653a3c; continue;
            }
            0x653a3c => {
            { let a = c.get(0,32); let r = c.sub(a,0x8u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6539c3; continue; }
            pc = 0x653a41; continue;
            }
            0x653a41 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x6539c3; continue; }
            pc = 0x653a49; continue;
            }
            0x653a49 => {
            { let a = c.get(0,32); let r = c.sub(a,0xdu32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653943; continue; }
            pc = 0x653a52; continue;
            }
            0x653a52 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x653a7c; continue; }
            pc = 0x653a56; continue;
            }
            0x653a56 => {
            pc = 0x653943; continue;
            }
            0x653a5b => {
            { let a = c.get(0,32); let r = c.sub(a,0x3e3u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x65398c; continue; }
            pc = 0x653a66; continue;
            }
            0x653a66 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x65398c; continue; }
            pc = 0x653a6d; continue;
            }
            0x653a6d => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x65398c; continue; }
            pc = 0x653a74; continue;
            }
            0x653a74 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653992; continue; }
            pc = 0x653a7c; continue;
            }
            0x653a7c => {
            c.push(m,0x1u32);
            pc = 0x65398e; continue;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653b0b(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653b0b;
    loop {
        match pc {
            0x653b0b => {
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,r); } }
            { let a = rm(m,c.r[0].wrapping_add(0x8),32); let r = c.and(a,0x0u32,32); { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = c.r[0]; wm(m,a,32,0x67cdd4u32); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00653c1d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x653c1d;
    loop {
        match pc {
            0x653c1d => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(4,32));
            { let a = c.get(4,32); let r = c.sub(a,0x110u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); let r = c.and(a,0x0u32,32); { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,r); } }
            { let a = rm(m,c.r[6].wrapping_add(0x4),32); let r = c.or(a,0xffffffffu32,32); { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,r); } }
            c.set(3,32,c.r[6].wrapping_add(0xc));
            { let a = c.get(7,32); let r = c.and(a,0xffff7fffu32,32); c.set(7,32,r); }
            c.set(1,32,c.get(3,32));
            c.push(m, 0x653c46);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.set(0,32,c.r[5].wrapping_add(0xfffffef0));
            c.push(m,c.get(0,32));
            c.push(m, 0x653c55);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x653f67, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.r[5].wrapping_add(0xfffffef0));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x653c63);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(7,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653c82; continue; }
            pc = 0x653c6e; continue;
            }
            0x653c6e => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x653c7b; continue; }
            pc = 0x653c71; continue;
            }
            0x653c71 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x653c87; continue; }
            pc = 0x653c74; continue;
            }
            0x653c74 => {
            c.set(1,32,0xc0000000u32);
            pc = 0x653c87; continue;
            }
            0x653c7b => {
            c.set(1,32,0x40000000u32);
            pc = 0x653c87; continue;
            }
            0x653c82 => {
            c.set(1,32,0x80000000u32);
            pc = 0x653c87; continue;
            }
            0x653c87 => {
            c.set(0,32,c.get(7,32));
            c.push(m,0x1u32);
            { let a = c.get(0,32); let r = c.and(a,0x70u32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(4) { pc = 0x653cb2; continue; }
            pc = 0x653c91; continue;
            }
            0x653c91 => {
            { let a = c.get(0,32); c.sub(a,0x10u32,32); }
            if c.cond(4) { pc = 0x653cb2; continue; }
            pc = 0x653c96; continue;
            }
            0x653c96 => {
            { let a = c.get(0,32); c.sub(a,0x20u32,32); }
            if c.cond(4) { pc = 0x653cae; continue; }
            pc = 0x653c9b; continue;
            }
            0x653c9b => {
            { let a = c.get(0,32); c.sub(a,0x30u32,32); }
            if c.cond(4) { pc = 0x653caa; continue; }
            pc = 0x653ca0; continue;
            }
            0x653ca0 => {
            { let a = c.get(0,32); c.sub(a,0x40u32,32); }
            if c.cond(5) { pc = 0x653cb2; continue; }
            pc = 0x653ca5; continue;
            }
            0x653ca5 => {
            c.push(m,0x3u32);
            pc = 0x653ca7; continue;
            }
            0x653ca7 => {
            { let v = c.pop(m); c.set(0,32,v); }
            pc = 0x653cb4; continue;
            }
            0x653caa => {
            c.push(m,0x2u32);
            pc = 0x653ca7; continue;
            }
            0x653cae => {
            c.set(0,32,c.get(3,32));
            pc = 0x653cb4; continue;
            }
            0x653cb2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x653cb4; continue;
            }
            0x653cb4 => {
            { let a = rm(m,c.r[5].wrapping_add(0xfffffff8),32); let r = c.and(a,0x0u32,32); { let a = c.r[5].wrapping_add(0xfffffff8); wm(m,a,32,r); } }
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); c.set(2,32,!a & 0xffffffffu32); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.r[5].wrapping_add(0xfffffff4); wm(m,a,32,0xcu32); }
            { let a = c.get(7,32); c.and(a,0x1000u32,32); }
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x653ce4; continue; }
            pc = 0x653cd3; continue;
            }
            0x653cd3 => {
            { let a = c.get(7,32); let r = c.and(a,0x2000u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.neg(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sbb(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x653ce7; continue;
            }
            0x653ce4 => {
            c.push(m,0x3u32);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x653ce7; continue;
            }
            0x653ce7 => {
            c.push(m,0x0u32);
            c.push(m,0x80u32);
            c.set(2,32,c.r[5].wrapping_add(0xfffffff4));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            { let target = rm(m,0x66a13cu32,32); c.push(m, 0x653cfe); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(36);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x653d2b; continue; }
            pc = 0x653d03; continue;
            }
            0x653d03 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x653d27; continue; }
            pc = 0x653d0a; continue;
            }
            0x653d0a => {
            { let target = rm(m,0x66a0d4u32,32); c.push(m, 0x653d10); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x653d19);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x65388d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = rm(m,c.r[5].wrapping_add(0x8),32); c.push(m, v); }
            c.set(1,32,c.r[6].wrapping_add(0x10));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x653d27);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x653d27; continue;
            }
            0x653d27 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x653d33; continue;
            }
            0x653d2b => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,c.get(3,32));
            pc = 0x653d33; continue;
            }
            0x653d33 => {
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

pub fn f_006592c2(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6592c2;
    loop {
        match pc {
            0x6592c2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = rm(m,c.r[1],32); let r = c.add(a,c.get(2,32),32); { let a = c.r[1]; wm(m,a,32,r); } }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = rm(m,c.r[1].wrapping_add(0x4),32); let r = c.add(a,c.get(2,32),32); { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,r); } }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1].wrapping_add(0x8),32); let r = c.sub(a,c.get(2,32),32); { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,r); } }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = rm(m,c.r[1].wrapping_add(0xc),32); let r = c.sub(a,c.get(0,32),32); { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,r); } }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
