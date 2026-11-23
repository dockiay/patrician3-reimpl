//! Generado por `reimpl/tools/ui_rt_grp.py ops` con `tools/lift.py` (no editar a mano): grupo `ops`.
#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::*;

pub static GRP_OPS: [(u32, super::core_table::CoreFn); 59] = [
    (0x4d6a50, f_004d6a50),
    (0x4d6af0, f_004d6af0),
    (0x4d74c0, f_004d74c0),
    (0x4dab00, f_004dab00),
    (0x509d10, f_00509d10),
    (0x51b890, f_0051b890),
    (0x522b20, f_00522b20),
    (0x522c70, f_00522c70),
    (0x5244f0, f_005244f0),
    (0x538000, f_00538000),
    (0x5386c0, f_005386c0),
    (0x538800, f_00538800),
    (0x538930, f_00538930),
    (0x5389c0, f_005389c0),
    (0x538c40, f_00538c40),
    (0x538de0, f_00538de0),
    (0x539270, f_00539270),
    (0x539990, f_00539990),
    (0x539cf0, f_00539cf0),
    (0x53ac50, f_0053ac50),
    (0x53ad10, f_0053ad10),
    (0x53cf50, f_0053cf50),
    (0x53d190, f_0053d190),
    (0x53da30, f_0053da30),
    (0x53da90, f_0053da90),
    (0x53dbf0, f_0053dbf0),
    (0x53ddf0, f_0053ddf0),
    (0x53df00, f_0053df00),
    (0x53e480, f_0053e480),
    (0x53e610, f_0053e610),
    (0x53e7c0, f_0053e7c0),
    (0x53e890, f_0053e890),
    (0x53e9c0, f_0053e9c0),
    (0x53eac0, f_0053eac0),
    (0x53ec30, f_0053ec30),
    (0x53ecb0, f_0053ecb0),
    (0x53eec0, f_0053eec0),
    (0x53fb00, f_0053fb00),
    (0x53fea0, f_0053fea0),
    (0x540240, f_00540240),
    (0x541840, f_00541840),
    (0x5419c0, f_005419c0),
    (0x541cc0, f_00541cc0),
    (0x542380, f_00542380),
    (0x5424d0, f_005424d0),
    (0x542980, f_00542980),
    (0x542f10, f_00542f10),
    (0x5436d0, f_005436d0),
    (0x543950, f_00543950),
    (0x543ae0, f_00543ae0),
    (0x543c70, f_00543c70),
    (0x5440a0, f_005440a0),
    (0x544140, f_00544140),
    (0x544160, f_00544160),
    (0x544300, f_00544300),
    (0x549190, f_00549190),
    (0x54a420, f_0054a420),
    (0x5338a0, f_005338a0),
    (0x5063c0, f_005063c0),
];
pub fn f_004d6a50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d6a50;
    loop {
        match pc {
            0x4d6a50 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4d6ade; continue; }
            pc = 0x4d6a62; continue;
            }
            0x4d6a62 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4d6ade; continue; }
            pc = 0x4d6a73; continue;
            }
            0x4d6a73 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4d6a7c; continue; }
            pc = 0x4d6a77; continue;
            }
            0x4d6a77 => {
            c.set(0,32,c.r[6].wrapping_add(0x8));
            pc = 0x4d6a8a; continue;
            }
            0x4d6a7c => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4d6a87);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,0xau32,32); c.set(0,32,r); }
            pc = 0x4d6a8a; continue;
            }
            0x4d6a8a => {
            c.set(1,16,rm(m,c.r[0],16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x4d6ade; continue; }
            pc = 0x4d6a96; continue;
            }
            0x4d6a96 => {
            { let a = c.get(1,16); c.sub(a,c.get(7,16),16); }
            if c.cond(4) { pc = 0x4d6ab7; continue; }
            pc = 0x4d6a9b; continue;
            }
            0x4d6a9b => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x6));
            c.set(1,16,rm(m,c.r[0],16));
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x4d6a96; continue; }
            pc = 0x4d6ab2; continue;
            }
            0x4d6ab2 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4d6ab7 => {
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,32,c.get(7,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x6),16));
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            c.set(0,32,rm(m,c.r[6],32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0x6); wm(m,a,16,c.get(1,16)); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,16,c.get(7,16)); }
            c.push(m, 0x4d6ade);
            { let args = []; c.r[0] = host.call(m, 0x51aab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4d6ade; continue;
            }
            0x4d6ade => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004d6af0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d6af0;
    loop {
        match pc {
            0x4d6af0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,0x6de4aau32,16));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x4d6b35; continue; }
            pc = 0x4d6b01; continue;
            }
            0x4d6b01 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4d6b0d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.add(a,0x38u32,32); c.set(0,32,r); }
            c.set(6,32,0x8u32);
            pc = 0x4d6b19; continue;
            }
            0x4d6b19 => {
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.r[0]; wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            if c.cond(5) { pc = 0x4d6b19; continue; }
            pc = 0x4d6b22; continue;
            }
            0x4d6b22 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4d6b2d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[0].wrapping_add(0x26); wm(m,a,8,c.get(1,8)); }
            pc = 0x4d6b35; continue;
            }
            0x4d6b35 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_004d74c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d74c0;
    loop {
        match pc {
            0x4d74c0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            c.push(m,c.get(7,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x6),16));
            if c.cond(3) { pc = 0x4d75bc; continue; }
            pc = 0x4d74e1; continue;
            }
            0x4d74e1 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4d74ec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,rm(m,c.r[0],32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(2,32),32); c.set(5,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(5,32,c.r[0].wrapping_add(0xa));
            c.set(1,32,c.get(5,32));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x4d75bc; continue; }
            pc = 0x4d750b; continue;
            }
            0x4d750b => {
            c.set(3,32,c.get(7,32));
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x4d75bc; continue; }
            pc = 0x4d7516; continue;
            }
            0x4d7516 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4d753b; continue; }
            pc = 0x4d7523; continue;
            }
            0x4d7523 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x6));
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x4d750b; continue; }
            pc = 0x4d7534; continue;
            }
            0x4d7534 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4d753b => {
            c.set(2,32,rm(m,c.r[6],32));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x4));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x4),8));
            { let a = c.get(2,8); c.sub(a,0x45u32,8); }
            if c.cond(4) { pc = 0x4d7554; continue; }
            pc = 0x4d754f; continue;
            }
            0x4d754f => {
            { let a = c.get(2,8); c.sub(a,0x46u32,8); }
            if c.cond(5) { pc = 0x4d75bc; continue; }
            pc = 0x4d7554; continue;
            }
            0x4d7554 => {
            { let a = c.r[7]; wm(m,a,8,0x46u32); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(7,16,rm(m,c.r[4].wrapping_add(0x20),16));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xa),16); let r = c.or(a,c.get(7,16),16); { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0xa); wm(m,a,16,r); } }
            c.set(3,8,rm(m,0x6de4a0u32,8));
            c.set(2,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0xa));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[0].wrapping_add(c.r[2]); wm(m,a,8,c.get(3,8)); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(3,8,rm(m,0x6de4a1u32,8));
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x1); wm(m,a,8,c.get(3,8)); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(7,16,rm(m,0x6de4a2u32,16));
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x2); wm(m,a,16,c.get(7,16)); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x1),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x1));
            { let a = c.get(3,8); let r = c.or(a,0x80u32,8); c.set(3,8,r); }
            { let a = c.r[2]; wm(m,a,8,c.get(3,8)); }
            c.set(2,32,rm(m,c.r[6],32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x6),16));
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,c.r[6],32));
            c.set(2,16,rm(m,c.r[5],16));
            { let a = c.r[0].wrapping_add(c.r[1]).wrapping_add(0x6); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[5]; wm(m,a,16,c.get(3,16)); }
            pc = 0x4d75bc; continue;
            }
            0x4d75bc => {
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

pub fn f_004dab00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4dab00;
    loop {
        match pc {
            0x4dab00 => {
            { let a = c.get(4,32); let r = c.sub(a,0x1cu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(0,32));
            c.set(5,32,c.get(1,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(6,32); let r = c.and(a,0xffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            if c.cond(3) { pc = 0x4daefe; continue; }
            pc = 0x4dab27; continue;
            }
            0x4dab27 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x776),16),16); }
            c.set(7,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(7,32)); }
            if c.cond(3) { pc = 0x4daefe; continue; }
            pc = 0x4dab4e; continue;
            }
            0x4dab4e => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4dab61);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4dafe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4daefe; continue; }
            pc = 0x4dab6c; continue;
            }
            0x4dab6c => {
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4dab7a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x2),8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4daefe; continue; }
            pc = 0x4dab93; continue;
            }
            0x4dab93 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4daba3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,0x1u32);
            c.set(1,32,c.get(6,32));
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x468),32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4daefe; continue; }
            pc = 0x4dabb8; continue;
            }
            0x4dabb8 => {
            c.set(6,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.or(a,0xffu32,32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8),16));
            { let a = c.get(6,32); let r = c.add(a,0x301u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0xd),8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[5].wrapping_add(0xc),16));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            if c.cond(5) { pc = 0x4dac63; continue; }
            pc = 0x4dabef; continue;
            }
            0x4dabef => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x4dac85; continue; }
            pc = 0x4dabf7; continue;
            }
            0x4dabf7 => {
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(2,32)); }
            pc = 0x4dabfe; continue;
            }
            0x4dabfe => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = rm(m,c.r[2],32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x4dac85; continue; }
            pc = 0x4dac0c; continue;
            }
            0x4dac0c => {
            { let a = rm(m,c.r[2].wrapping_add(0x6),16); c.sub(a,0x36u32,16); }
            if c.cond(5) { pc = 0x4dac53; continue; }
            pc = 0x4dac13; continue;
            }
            0x4dac13 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x15),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4dac53; continue; }
            pc = 0x4dac1e; continue;
            }
            0x4dac1e => {
            { let a = rm(m,c.r[2].wrapping_add(0x14),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4dac53; continue; }
            pc = 0x4dac24; continue;
            }
            0x4dac24 => {
            c.set(7,16,rm(m,c.r[7].wrapping_add(0x776),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(0x8));
            pc = 0x4dac30; continue;
            }
            0x4dac30 => {
            c.set(5,16,rm(m,c.r[1],16));
            { let a = c.get(5,32); let r = c.and(a,0x7fffu32,32); c.set(5,32,r); }
            { let a = c.get(5,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x4dad70; continue; }
            pc = 0x4dac42; continue;
            }
            0x4dac42 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x4dac30; continue; }
            pc = 0x4dac4b; continue;
            }
            0x4dac4b => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            pc = 0x4dac53; continue;
            }
            0x4dac53 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x4dabfe; continue; }
            pc = 0x4dac61; continue;
            }
            0x4dac61 => {
            pc = 0x4dac85; continue;
            }
            0x4dac63 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(13) { pc = 0x4dac85; continue; }
            pc = 0x4dac67; continue;
            }
            0x4dac67 => {
            c.set(2,32,rm(m,c.r[5],32));
            pc = 0x4dac6a; continue;
            }
            0x4dac6a => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(7,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.get(7,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x4dac85; continue; }
            pc = 0x4dac77; continue;
            }
            0x4dac77 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x4dac6a; continue; }
            pc = 0x4dac85; continue;
            }
            0x4dac85 => {
            c.set(2,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(7,32,c.r[2].wrapping_add(0x3200));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            if c.cond(13) { pc = 0x4dad04; continue; }
            pc = 0x4dac99; continue;
            }
            0x4dac99 => {
            c.set(5,32,rm(m,c.r[5],32));
            pc = 0x4dac9c; continue;
            }
            0x4dac9c => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,32,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(8)),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(7) { pc = 0x4dad00; continue; }
            pc = 0x4dacab; continue;
            }
            0x4dacab => {
            { let a = rm(m,c.r[2].wrapping_add(0x6),16); c.sub(a,0x36u32,16); }
            if c.cond(5) { pc = 0x4dacf2; continue; }
            pc = 0x4dacb2; continue;
            }
            0x4dacb2 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x30),8));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x15),8));
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4dacf2; continue; }
            pc = 0x4dacbd; continue;
            }
            0x4dacbd => {
            { let a = rm(m,c.r[2].wrapping_add(0x14),8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x4dacf2; continue; }
            pc = 0x4dacc3; continue;
            }
            0x4dacc3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(6,16,rm(m,c.r[1].wrapping_add(0x776),16));
            c.set(1,32,c.r[2].wrapping_add(0x8));
            pc = 0x4dacd3; continue;
            }
            0x4dacd3 => {
            c.set(3,16,rm(m,c.r[1],16));
            { let a = c.get(3,32); let r = c.and(a,0x7fffu32,32); c.set(3,32,r); }
            { let a = c.get(3,16); c.sub(a,c.get(6,16),16); }
            if c.cond(3) { pc = 0x4dad70; continue; }
            pc = 0x4dace5; continue;
            }
            0x4dace5 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x2u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x4dacd3; continue; }
            pc = 0x4dacee; continue;
            }
            0x4dacee => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x4dacf2; continue;
            }
            0x4dacf2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(12) { pc = 0x4dac9c; continue; }
            pc = 0x4dad00; continue;
            }
            0x4dad00 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x4dad04; continue;
            }
            0x4dad04 => {
            c.push(m,0x190u32);
            c.push(m, 0x4dad0e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4daefe; continue; }
            pc = 0x4dad1b; continue;
            }
            0x4dad1b => {
            c.push(m,0x190u32);
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4dad28);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.set(2,16,rm(m,c.r[5].wrapping_add(0xc),16));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(13) { pc = 0x4dae2a; continue; }
            pc = 0x4dad43; continue;
            }
            0x4dad43 => {
            c.set(2,32,rm(m,c.r[5],32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(2,16); c.sub(a,0xau32,16); }
            if c.cond(4) { pc = 0x4dad60; continue; }
            pc = 0x4dad56; continue;
            }
            0x4dad56 => {
            { let a = c.get(2,16); c.sub(a,0x36u32,16); }
            if c.cond(5) { pc = 0x4dae09; continue; }
            pc = 0x4dad60; continue;
            }
            0x4dad60 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x14),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4dadba; continue; }
            pc = 0x4dad67; continue;
            }
            0x4dad67 => {
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); let r = c.add(a,0x2000u32,32); c.set(0,32,r); }
            pc = 0x4daddc; continue;
            }
            0x4dad70 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x8); wm(m,a,16,c.get(6,16)); }
            c.set(1,8,rm(m,c.r[7],8));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x7ac),32));
            { let a = c.get(3,32); let r = c.imul(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x80c),32));
            { let a = rm(m,c.r[3].wrapping_add(c.r[1]),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x4daefe; continue; }
            pc = 0x4dada5; continue;
            }
            0x4dada5 => {
            { let a = c.get(6,32); let r = c.or(a,0x8000u32,32); c.set(6,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x8); wm(m,a,16,c.get(6,16)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4dadba => {
            { let a = c.get(2,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4dadd0; continue; }
            pc = 0x4dadbf; continue;
            }
            0x4dadbf => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.set(0,32,rm(m,c.r[0],32));
            c.set(0,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x200));
            pc = 0x4daddc; continue;
            }
            0x4dadd0 => {
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(2,8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x4daddc; continue; }
            pc = 0x4dadd7; continue;
            }
            0x4dadd7 => {
            { let a = c.get(0,32); let r = c.add(a,0x200u32,32); c.set(0,32,r); }
            pc = 0x4daddc; continue;
            }
            0x4daddc => {
            { let a = c.get(0,32); let r = c.sub(a,rm(m,0x6de4b4u32,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x17cu32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x4dae09; continue; }
            pc = 0x4dadf0; continue;
            }
            0x4dadf0 => {
            c.set(1,32,0x6u32);
            pc = 0x4dadf5; continue;
            }
            0x4dadf5 => {
            { let a = c.r[6].wrapping_add(c.r[0]); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4dadf5; continue; }
            pc = 0x4dae05; continue;
            }
            0x4dae05 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4dae09; continue;
            }
            0x4dae09 => {
            c.set(2,32,rm(m,c.r[5],32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0xc),16));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(12) { pc = 0x4dad43; continue; }
            pc = 0x4dae2a; continue;
            }
            0x4dae2a => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,c.get(5,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4dae38);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x36u32); }
            { let a = c.r[0].wrapping_add(0x17); wm(m,a,8,0xffu32); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,c.get(1,16)); }
            c.set(2,8,rm(m,c.r[7],8));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x7ac),32));
            { let a = c.get(3,32); let r = c.imul(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x80c),32));
            { let a = rm(m,c.r[3].wrapping_add(c.r[2]),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x4dae7b; continue; }
            pc = 0x4dae72; continue;
            }
            0x4dae72 => {
            c.set(2,32,c.get(1,32));
            { let a = c.get(6,8); let r = c.or(a,0x80u32,8); c.set(6,8,r); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,c.get(2,16)); }
            pc = 0x4dae7b; continue;
            }
            0x4dae7b => {
            c.set(2,32,0xffffu32);
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[0].wrapping_add(0xa); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,16,c.get(2,16)); }
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x30),8));
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[0].wrapping_add(0x12); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,8,0x1u32); }
            { let a = c.r[0].wrapping_add(0x15); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[0].wrapping_add(0x16); wm(m,a,8,0x0u32); }
            c.set(2,8,rm(m,0x6de4b4u32,8));
            { let a = c.get(2,32); let r = c.and(a,0x3u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1]).wrapping_add(0xe));
            { let a = c.r[0].wrapping_add(0xe); wm(m,a,16,c.get(1,16)); }
            pc = 0x4daeb5; continue;
            }
            0x4daeb5 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(1,32); let r = c.add(a,0x5u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x190u32,32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(13) { pc = 0x4daee9; continue; }
            pc = 0x4daeca; continue;
            }
            0x4daeca => {
            c.set(2,32,0x6u32);
            pc = 0x4daecf; continue;
            }
            0x4daecf => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[1]),8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x4daee1; continue; }
            pc = 0x4daed5; continue;
            }
            0x4daed5 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(15) { pc = 0x4daecf; continue; }
            pc = 0x4daedf; continue;
            }
            0x4daedf => {
            pc = 0x4daee9; continue;
            }
            0x4daee1 => {
            { let a = rm(m,c.r[0].wrapping_add(0xe),16); let r = c.add(a,c.get(2,16),16); { let a = c.r[0].wrapping_add(0xe); wm(m,a,16,r); } }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            if c.cond(15) { pc = 0x4daeb5; continue; }
            pc = 0x4daee9; continue;
            }
            0x4daee9 => {
            c.set(2,16,rm(m,c.r[0].wrapping_add(0xe),16));
            c.push(m,c.get(6,32));
            { let a = c.get(2,16); let n = 0x8u32; let r = c.shl(a,n,16); c.set(2,16,r); }
            { let a = c.r[0].wrapping_add(0xe); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x4daefb);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4daefe; continue;
            }
            0x4daefe => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00509d10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x509d10;
    loop {
        match pc {
            0x509d10 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,16,rm(m,c.r[5].wrapping_add(0xf4),16));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            if c.cond(3) { pc = 0x509f6c; continue; }
            pc = 0x509d30; continue;
            }
            0x509d30 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x509f6c; continue; }
            pc = 0x509d3c; continue;
            }
            0x509d3c => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(2,16,rm(m,0x6dd896u32,16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(6,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x509f6c; continue; }
            pc = 0x509d67; continue;
            }
            0x509d67 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x14),16));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,8); let r = c.and(a,0x2u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x2u32,8); }
            if c.cond(5) { pc = 0x509d93; continue; }
            pc = 0x509d87; continue;
            }
            0x509d87 => {
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(1,8); c.sub(a,rm(m,c.r[6],8),8); }
            if c.cond(5) { pc = 0x509f6c; continue; }
            pc = 0x509d91; continue;
            }
            0x509d91 => {
            pc = 0x509dba; continue;
            }
            0x509d93 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xe),8));
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x673660),8));
            { let a = rm(m,c.r[6].wrapping_add(0x40),16); c.sub(a,c.get(1,16),16); }
            if c.cond(2) { pc = 0x509f6c; continue; }
            pc = 0x509dab; continue;
            }
            0x509dab => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x14),32),32); }
            if c.cond(12) { pc = 0x509f6c; continue; }
            pc = 0x509dba; continue;
            }
            0x509dba => {
            { let a = rm(m,c.r[6].wrapping_add(0x8),16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x509f6c; continue; }
            pc = 0x509dc4; continue;
            }
            0x509dc4 => {
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,8); let r = c.and(a,0x10u32,8); c.set(2,8,r); }
            { let a = c.get(2,8); c.sub(a,0x10u32,8); }
            if c.cond(4) { pc = 0x509f6c; continue; }
            pc = 0x509dd2; continue;
            }
            0x509dd2 => {
            { let a = c.get(0,8); let r = c.and(a,0x4u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x4u32,8); }
            if c.cond(5) { pc = 0x509de2; continue; }
            pc = 0x509dd8; continue;
            }
            0x509dd8 => {
            { let a = rm(m,c.r[6].wrapping_add(0xe),8); c.sub(a,0x2u32,8); }
            if c.cond(3) { pc = 0x509f6c; continue; }
            pc = 0x509de2; continue;
            }
            0x509de2 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x1c),16),16); }
            if c.cond(5) { pc = 0x509f6c; continue; }
            pc = 0x509df1; continue;
            }
            0x509df1 => {
            { let a = rm(m,c.r[6].wrapping_add(0xe),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x509e22; continue; }
            pc = 0x509df7; continue;
            }
            0x509df7 => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1f),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x509e22; continue; }
            pc = 0x509e06; continue;
            }
            0x509e06 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2c8),8); c.and(a,0x1u32,8); }
            if c.cond(5) { pc = 0x509f6c; continue; }
            pc = 0x509e22; continue;
            }
            0x509e22 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x509e49; continue; }
            pc = 0x509e2e; continue;
            }
            0x509e2e => {
            { let a = c.get(0,16); c.sub(a,0x1u32,16); }
            if c.cond(5) { pc = 0x509f6c; continue; }
            pc = 0x509e38; continue;
            }
            0x509e38 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x138),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(0x16),16),16); }
            if c.cond(7) { pc = 0x509f6c; continue; }
            pc = 0x509e49; continue;
            }
            0x509e49 => {
            { let a = rm(m,c.r[7].wrapping_add(0x12),16); c.sub(a,0x9u32,16); }
            if c.cond(5) { pc = 0x509e64; continue; }
            pc = 0x509e50; continue;
            }
            0x509e50 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x509e58);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5039f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x509f6c; continue; }
            pc = 0x509e60; continue;
            }
            0x509e60 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x509e64; continue;
            }
            0x509e64 => {
            c.set(2,16,rm(m,c.r[5].wrapping_add(0xe8),16));
            c.set(1,32,c.r[5].wrapping_add(0xe8));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x509ea4; continue; }
            pc = 0x509e7c; continue;
            }
            0x509e7c => {
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[5].wrapping_add(0xf4),16),16); }
            if c.cond(3) { pc = 0x509f6c; continue; }
            pc = 0x509e89; continue;
            }
            0x509e89 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(1,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x6));
            c.set(2,16,rm(m,c.r[1],16));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x509e7c; continue; }
            pc = 0x509ea4; continue;
            }
            0x509ea4 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[1]; wm(m,a,16,c.get(0,16)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xa),16));
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,c.get(3,16)); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x8),16));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x136),8));
            { let a = rm(m,c.r[6].wrapping_add(0x3c),8); let r = c.or(a,0x60u32,8); { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,r); } }
            { let a = c.get(2,8); let r = c.and(a,0xfcu32,8); c.set(2,8,r); }
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x509ede);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x516840, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dfc14u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x509ef7; continue; }
            pc = 0x509eeb; continue;
            }
            0x509eeb => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x509ef7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x44f940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x509ef7; continue;
            }
            0x509ef7 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x509eff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50d040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x509f06);
            { let args = []; c.r[0] = host.call(m, 0x501be0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x12),16));
            { let a = c.r[7].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(0,16); c.sub(a,0x9u32,16); }
            if c.cond(4) { pc = 0x509f1d; continue; }
            pc = 0x509f17; continue;
            }
            0x509f17 => {
            { let a = c.get(0,16); c.sub(a,0xau32,16); }
            if c.cond(5) { pc = 0x509f6c; continue; }
            pc = 0x509f1d; continue;
            }
            0x509f1d => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x2),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x509f2e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,16,0x0u32); }
            c.push(m, 0x509f48);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x509f4f);
            { let args = []; c.r[0] = host.call(m, 0x4f8d20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,0xffu32);
            pc = 0x509f53; continue;
            }
            0x509f53 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(c.r[0]).wrapping_add(0x8),8));
            c.set(3,8,rm(m,c.r[6],8));
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x509f66; continue; }
            pc = 0x509f5d; continue;
            }
            0x509f5d => {
            { let a = c.r[7].wrapping_add(c.r[0]).wrapping_add(0x8); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,0x8u32);
            pc = 0x509f66; continue;
            }
            0x509f66 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x509f53; continue; }
            pc = 0x509f6c; continue;
            }
            0x509f6c => {
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

pub fn f_0051b890(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51b890;
    loop {
        match pc {
            0x51b890 => {
            { let a = c.get(4,32); let r = c.sub(a,0x58u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x68),32));
            { let a = c.get(6,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x13u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xbu32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xcu32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x11u32); }
            if c.cond(3) { pc = 0x51b9ff; continue; }
            pc = 0x51b8ca; continue;
            }
            0x51b8ca => {
            c.set(0,32,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x51b8dc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x2c0),8));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(3,32,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x51b8f5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x51b9ff; continue; }
            pc = 0x51b8fd; continue;
            }
            0x51b8fd => {
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x66); wm(m,a,16,c.get(6,16)); }
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,16,c.get(5,16)); }
            c.push(m, 0x51b913);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x51b9ff; continue; }
            pc = 0x51b91b; continue;
            }
            0x51b91b => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(c.r[5]).wrapping_add(0x14),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(c.r[5]).wrapping_add(0x44),32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(0,32,c.get(1,32));
            if c.cond(4) { pc = 0x51b95d; continue; }
            pc = 0x51b92f; continue;
            }
            0x51b92f => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[5]).wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(c.r[5]).wrapping_add(0x44),32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x51b98f; continue;
            }
            0x51b95d => {
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[5]).wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(c.r[5]).wrapping_add(0x44),32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x51b98f; continue;
            }
            0x51b98f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(c.r[5]).wrapping_add(0x34),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x51b9ad; continue; }
            pc = 0x51b997; continue;
            }
            0x51b997 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2c0),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x51b9ad);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4f5450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x51b9ad; continue;
            }
            0x51b9ad => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x51b9bd; continue; }
            pc = 0x51b9b5; continue;
            }
            0x51b9b5 => {
            { let a = c.r[7].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,0x0u32); }
            pc = 0x51b9bd; continue;
            }
            0x51b9bd => {
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x10u32,32); }
            if c.cond(12) { pc = 0x51b91b; continue; }
            pc = 0x51b9c9; continue;
            }
            0x51b9c9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(1,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[3].wrapping_add(0x4c0),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            { let a = c.get(5,32); let r = c.add(a,c.get(1,32),32); c.set(5,32,r); }
            { let a = c.r[3].wrapping_add(0x4c0); wm(m,a,32,c.get(5,32)); }
            c.push(m,0x1u32);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x2c0),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x51b9ff);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x51b9ff; continue;
            }
            0x51b9ff => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x58u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00522b20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x522b20;
    loop {
        match pc {
            0x522b20 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x663244u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x522b46);
            { let args = []; c.r[0] = host.call(m, 0x503d40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x766),16); c.sub(a,0xffffu32,16); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(6,32)); }
            if c.cond(2) { pc = 0x522c43; continue; }
            pc = 0x522b5b; continue;
            }
            0x522b5b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(3,32,c.r[7].wrapping_add(0x7a4));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x522b6d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fcfe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x522c43; continue; }
            pc = 0x522b75; continue;
            }
            0x522b75 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x522b86);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fa1d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,c.get(6,32),32); }
            if c.cond(6) { pc = 0x522c2f; continue; }
            pc = 0x522b90; continue;
            }
            0x522b90 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x2c),8));
            { let a = c.get(0,8); let r = c.or(a,0xffu32,8); c.set(0,8,r); }
            { let a = c.get(0,8); let r = c.sub(a,c.get(1,8),8); c.set(0,8,r); }
            c.push(m,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,c.get(0,8)); }
            pc = 0x522b9d; continue;
            }
            0x522b9d => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x522ba4);
            { let args = []; c.r[0] = host.call(m, 0x51f100, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x75c),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[6].wrapping_mul(4)),8));
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x17),8));
            { let a = c.r[0].wrapping_add(0x1); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[0].wrapping_add(0x2); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x2),8));
            { let a = c.r[0].wrapping_add(0x3); wm(m,a,8,c.get(2,8)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x766),16));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,16,c.get(1,16)); }
            c.set(1,8,c.get(2,8));
            c.set(2,32,c.get(1,32));
            c.push(m,c.get(1,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            c.push(m,0x0u32);
            c.set(2,8,rm(m,c.r[2].wrapping_add(0x672bb4),8));
            c.push(m,c.get(1,32));
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,c.get(3,32));
            c.set(2,8,rm(m,c.r[2].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x3),8));
            { let a = c.r[0].wrapping_add(0x7); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            c.push(m,c.get(2,32));
            c.push(m, 0x522c1a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4fb9c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[7].wrapping_add(0x766); wm(m,a,16,c.get(5,16)); }
            if c.cond(2) { pc = 0x522b9d; continue; }
            pc = 0x522c2e; continue;
            }
            0x522c2e => {
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x522c2f; continue;
            }
            0x522c2f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x7b0); wm(m,a,8,0xffu32); }
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x7b1); wm(m,a,8,0xffu32); }
            pc = 0x522c43; continue;
            }
            0x522c43 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x522c54);
            { let args = []; c.r[0] = host.call(m, 0x503d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00522c70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x522c70;
    loop {
        match pc {
            0x522c70 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x663256u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x68u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.set(3,32,c.r[6].wrapping_add(0x766));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x522ca3);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x2c8),8));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x523803; continue; }
            pc = 0x522cb8; continue;
            }
            0x522cb8 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x88),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x522cff; continue; }
            pc = 0x522ccb; continue;
            }
            0x522ccb => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x2c0),8));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x522ce0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x523803; continue; }
            pc = 0x522cee; continue;
            }
            0x522cee => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x522cf9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x522d07; continue;
            }
            0x522cff => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            pc = 0x522d07; continue;
            }
            0x522d07 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x776),16));
            pc = 0x522d0e; continue;
            }
            0x522d0e => {
            c.set(0,16,rm(m,c.r[3],16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x523803; continue; }
            pc = 0x522d1a; continue;
            }
            0x522d1a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(2,8); c.sub(a,0x36u32,8); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            c.set(0,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x522d43; continue; }
            pc = 0x522d3e; continue;
            }
            0x522d3e => {
            c.set(3,32,c.r[7].wrapping_add(0x4));
            pc = 0x522d0e; continue;
            }
            0x522d43 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x88),16));
            c.set(0,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x70); wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x76); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x522d66);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x523803; continue; }
            pc = 0x522d6e; continue;
            }
            0x522d6e => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3],16));
            { let a = c.r[3]; wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,16,0xffffu32); }
            c.push(m, 0x522d88);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5254c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x522f01; continue; }
            pc = 0x522d90; continue;
            }
            0x522d90 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x522de7; continue; }
            pc = 0x522daa; continue;
            }
            0x522daa => {
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m,0x13u32);
            c.set(3,32,c.get(0,32));
            c.push(m, 0x522dba);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(3,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(3,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x88));
            c.fstp32(m,c.r[4].wrapping_add(0x88));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x88),4));
            { let v = c.ld(m,0x673a64u32,4); c.fop('*',v); }
            c.push(m, 0x522de5);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            pc = 0x522de7; continue;
            }
            0x522de7 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x522e3e; continue; }
            pc = 0x522dff; continue;
            }
            0x522dff => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(5,32,c.get(0,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,0xbu32);
            c.push(m, 0x522e11);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(5,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(5,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x88));
            c.fstp32(m,c.r[4].wrapping_add(0x88));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x88),4));
            { let v = c.ld(m,0x673a44u32,4); c.fop('*',v); }
            c.push(m, 0x522e3c);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            pc = 0x522e3e; continue;
            }
            0x522e3e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x522e92; continue; }
            pc = 0x522e53; continue;
            }
            0x522e53 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,0xcu32);
            c.push(m, 0x522e65);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(5,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(5,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x88));
            c.fstp32(m,c.r[4].wrapping_add(0x88));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x88),4));
            { let v = c.ld(m,0x673a48u32,4); c.fop('*',v); }
            c.push(m, 0x522e90);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            pc = 0x522e92; continue;
            }
            0x522e92 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x522ee9; continue; }
            pc = 0x522eaa; continue;
            }
            0x522eaa => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,0x11u32);
            c.set(5,32,c.get(0,32));
            c.push(m, 0x522ebc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(5,32); let r = c.sub(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(5,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x88));
            c.fstp32(m,c.r[4].wrapping_add(0x88));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x88),4));
            { let v = c.ld(m,0x673a5cu32,4); c.fop('*',v); }
            c.push(m, 0x522ee7);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            pc = 0x522ee9; continue;
            }
            0x522ee9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x49c),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x49c); wm(m,a,32,c.get(1,32)); }
            pc = 0x522f01; continue;
            }
            0x522f01 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x68),32));
            c.push(m,0x6u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x522f0f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x529f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x50),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            if c.cond(9) { pc = 0x522f36; continue; }
            pc = 0x522f2f; continue;
            }
            0x522f2f => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(5,32)); }
            pc = 0x522f38; continue;
            }
            0x522f36 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x522f38; continue;
            }
            0x522f38 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            if c.cond(9) { pc = 0x522f5b; continue; }
            pc = 0x522f58; continue;
            }
            0x522f58 => {
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            pc = 0x522f5b; continue;
            }
            0x522f5b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5c),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x34),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,32,c.get(1,32)); }
            if c.cond(9) { pc = 0x522f7b; continue; }
            pc = 0x522f78; continue;
            }
            0x522f78 => {
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,32,c.get(5,32)); }
            pc = 0x522f7b; continue;
            }
            0x522f7b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x60),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x48),32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            if c.cond(9) { pc = 0x522f9e; continue; }
            pc = 0x522f9b; continue;
            }
            0x522f9b => {
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(5,32)); }
            pc = 0x522f9e; continue;
            }
            0x522f9e => {
            { let a = rm(m,c.r[4].wrapping_add(0x8c),32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x522fb1; continue; }
            pc = 0x522fa7; continue;
            }
            0x522fa7 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[7].wrapping_add(0x7),8));
            pc = 0x523038; continue;
            }
            0x522fb1 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(3,32,c.r[6].wrapping_add(0x7a4));
            { let a = c.get(0,8); c.sub(a,0x38u32,8); }
            if c.cond(5) { pc = 0x522fe7; continue; }
            pc = 0x522fbe; continue;
            }
            0x522fbe => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(2,32); let r = c.sub(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x522fd4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fbd60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(2,8,rm(m,c.r[7],8));
            { let a = c.get(1,32); let r = c.sub(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,0x2u32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            pc = 0x52300e; continue;
            }
            0x522fe7 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[7],8));
            { let a = c.get(0,32); let r = c.sub(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x522ffd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fbd60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(2,32); let r = c.sub(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            pc = 0x52300e; continue;
            }
            0x52300e => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x523015);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fbd60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(0,8,rm(m,c.r[7],8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x52302d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4fd830, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[7].wrapping_add(0x7),8));
            { let a = c.get(3,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,8); let r = c.or(a,0xffu32,8); c.set(3,8,r); }
            pc = 0x523038; continue;
            }
            0x523038 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7],8));
            c.set(1,32,c.r[6].wrapping_add(0x7a4));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x523052);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4fbd60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x5230c4; continue; }
            pc = 0x52305c; continue;
            }
            0x52305c => {
            c.set(5,32,rm(m,c.r[1].wrapping_add(0xc324),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x5230ba; continue; }
            pc = 0x52306e; continue;
            }
            0x52306e => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.set(0,32,sx(rm(m,c.r[2].wrapping_add(0x672fc0),8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x523090);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x590680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x6),8));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.push(m,0x0u32);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5230b0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x5891c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            pc = 0x5230c8; continue;
            }
            0x5230ba => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            pc = 0x5230c8; continue;
            }
            0x5230c4 => {
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            pc = 0x5230c8; continue;
            }
            0x5230c8 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = rm(m,c.r[6].wrapping_add(0x766),16); c.sub(a,c.get(0,16),16); }
            if c.cond(2) { pc = 0x523803; continue; }
            pc = 0x5230dc; continue;
            }
            0x5230dc => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5230e9);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            c.set(0,32,c.r[6].wrapping_add(0x768));
            { let a = c.r[6].wrapping_add(0x786); wm(m,a,16,0x0u32); }
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[0],16));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x776),16),16); }
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x523252; continue; }
            pc = 0x523122; continue;
            }
            0x523122 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x32u32,8); }
            if c.cond(2) { pc = 0x523236; continue; }
            pc = 0x52313c; continue;
            }
            0x52313c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x52321d; continue; }
            pc = 0x523150; continue;
            }
            0x523150 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x7),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7],8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m, 0x523170);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4facd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,8,rm(m,c.r[7],8));
            { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0]); wm(m,a,8,c.get(2,8)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5231f6; continue; }
            pc = 0x523191; continue;
            }
            0x523191 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5231a9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x599040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x36u32,8); }
            if c.cond(4) { pc = 0x5231f6; continue; }
            pc = 0x5231b0; continue;
            }
            0x5231b0 => {
            { let a = c.get(0,8); c.sub(a,0x37u32,8); }
            if c.cond(4) { pc = 0x5231d4; continue; }
            pc = 0x5231b4; continue;
            }
            0x5231b4 => {
            { let a = c.get(0,8); c.sub(a,0x34u32,8); }
            if c.cond(4) { pc = 0x5231d4; continue; }
            pc = 0x5231b8; continue;
            }
            0x5231b8 => {
            { let a = c.get(0,8); c.sub(a,0x38u32,8); }
            if c.cond(4) { pc = 0x5231c0; continue; }
            pc = 0x5231bc; continue;
            }
            0x5231bc => {
            { let a = c.get(0,8); c.sub(a,0x35u32,8); }
            if c.cond(5) { pc = 0x5231f6; continue; }
            pc = 0x5231c0; continue;
            }
            0x5231c0 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(3,32); let r = c.sub(a,0x2u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5231d1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x599040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            pc = 0x5231e6; continue;
            }
            0x5231d4 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(3,32); let r = c.sub(a,0x2u32,32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5231e5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x599040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            pc = 0x5231e6; continue;
            }
            0x5231e6 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(3,32); let r = c.sub(a,0x2u32,32); c.set(3,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5231f6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x599040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5231f6; continue;
            }
            0x5231f6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x88),32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x760),16));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[6].wrapping_add(0x760); wm(m,a,16,c.get(0,16)); }
            pc = 0x523242; continue;
            }
            0x52321d => {
            { let a = c.get(0,8); c.sub(a,0x36u32,8); }
            if c.cond(4) { pc = 0x52322f; continue; }
            pc = 0x523221; continue;
            }
            0x523221 => {
            { let a = c.get(0,8); c.sub(a,0x34u32,8); }
            if c.cond(2) { pc = 0x52322f; continue; }
            pc = 0x523225; continue;
            }
            0x523225 => {
            { let a = rm(m,c.r[6].wrapping_add(0x786),16); let r = c.add(a,0x3u32,16); { let a = c.r[6].wrapping_add(0x786); wm(m,a,16,r); } }
            pc = 0x523236; continue;
            }
            0x52322f => {
            { let a = rm(m,c.r[6].wrapping_add(0x786),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x786); wm(m,a,16,r); } }
            pc = 0x523236; continue;
            }
            0x523236 => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(7,32)); }
            c.set(1,32,c.get(7,32));
            pc = 0x523242; continue;
            }
            0x523242 => {
            c.set(0,16,rm(m,c.r[1],16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x523122; continue; }
            pc = 0x523252; continue;
            }
            0x523252 => {
            { let a = rm(m,c.r[6].wrapping_add(0x786),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x523265; continue; }
            pc = 0x52325c; continue;
            }
            0x52325c => {
            { let a = c.r[6].wrapping_add(0x786); wm(m,a,16,0x1u32); }
            pc = 0x523265; continue;
            }
            0x523265 => {
            c.set(5,32,c.r[6].wrapping_add(0x762));
            c.set(3,32,c.get(5,32));
            c.set(0,16,rm(m,c.r[3],16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x776),16),16); }
            if c.cond(3) { pc = 0x523316; continue; }
            pc = 0x52327d; continue;
            }
            0x52327d => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2),8));
            c.set(7,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = rm(m,c.r[4].wrapping_add(0x8c),32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x523303; continue; }
            pc = 0x52329a; continue;
            }
            0x52329a => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x32u32,8); }
            if c.cond(2) { pc = 0x523303; continue; }
            pc = 0x5232a1; continue;
            }
            0x5232a1 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x7),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(2,32));
            c.set(0,8,rm(m,c.r[7],8));
            c.push(m,c.get(0,32));
            c.push(m, 0x5232c1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4facd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5232df; continue; }
            pc = 0x5232c9; continue;
            }
            0x5232c9 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x1),8));
            c.set(0,8,rm(m,c.r[7],8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5232df);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x599040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5232df; continue;
            }
            0x5232df => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3],16));
            { let a = c.r[3]; wm(m,a,16,c.get(1,16)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x760),16));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[6].wrapping_add(0x760); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0x3); wm(m,a,8,0xffu32); }
            pc = 0x523306; continue;
            }
            0x523303 => {
            c.set(3,32,c.r[7].wrapping_add(0x4));
            pc = 0x523306; continue;
            }
            0x523306 => {
            c.set(0,16,rm(m,c.r[3],16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x52327d; continue; }
            pc = 0x523316; continue;
            }
            0x523316 => {
            c.set(0,16,rm(m,c.r[5],16));
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x52335e; continue; }
            pc = 0x523326; continue;
            }
            0x523326 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            c.set(3,32,0x3u32);
            pc = 0x523331; continue;
            }
            0x523331 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(0,8); c.sub(a,0x32u32,8); }
            if c.cond(2) { pc = 0x523355; continue; }
            pc = 0x523341; continue;
            }
            0x523341 => {
            { let a = c.get(0,8); c.sub(a,0x36u32,8); }
            if c.cond(6) { pc = 0x52334e; continue; }
            pc = 0x523345; continue;
            }
            0x523345 => {
            { let a = rm(m,c.r[6].wrapping_add(0x786),16); let r = c.add(a,c.get(3,16),16); { let a = c.r[6].wrapping_add(0x786); wm(m,a,16,r); } }
            pc = 0x523355; continue;
            }
            0x52334e => {
            { let a = rm(m,c.r[6].wrapping_add(0x786),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x786); wm(m,a,16,r); } }
            pc = 0x523355; continue;
            }
            0x523355 => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,c.get(7,16),16); }
            if c.cond(2) { pc = 0x523331; continue; }
            pc = 0x52335e; continue;
            }
            0x52335e => {
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x76c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(3,32); let r = c.or(a,0x10u32,32); c.set(3,32,r); }
            { let a = c.r[6].wrapping_add(0x76c); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x523376);
            { let args = []; c.r[0] = host.call(m, 0x4fd680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,0x1u32);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x768),16));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            { let a = c.r[6].wrapping_add(0x788); wm(m,a,8,0x0u32); }
            { let a = c.r[6].wrapping_add(0x78c); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x78e); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[6].wrapping_add(0x786); wm(m,a,16,0x0u32); }
            c.push(m, 0x5233aa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x524b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x784),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5233f6; continue; }
            pc = 0x5233c3; continue;
            }
            0x5233c3 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5233ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2ce),16));
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x2ca),16));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5233e8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x524b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x5233c3; continue; }
            pc = 0x5233f6; continue;
            }
            0x5233f6 => {
            { let a = rm(m,c.r[6].wrapping_add(0x786),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x523407; continue; }
            pc = 0x523400; continue;
            }
            0x523400 => {
            { let a = c.r[6].wrapping_add(0x786); wm(m,a,16,c.get(3,16)); }
            pc = 0x523407; continue;
            }
            0x523407 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x6f1),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            { let a = c.r[4].wrapping_add(0x29); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,c.get(0,8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,0x2fu32); }
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x523456; continue; }
            pc = 0x52342a; continue;
            }
            0x52342a => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x52343a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x1c),8));
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,c.get(1,16)); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x1d),8));
            { let a = c.get(1,16); let r = c.add(a,c.get(0,16),16); c.set(1,16,r); }
            { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,c.get(1,16)); }
            pc = 0x52347a; continue;
            }
            0x523456 => {
            { let a = c.get(1,32); let r = c.or(a,0xffffffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0x3u32,32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x6d4),8));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[6]).wrapping_add(0x6d8),8));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,c.get(2,16)); }
            pc = 0x52347a; continue;
            }
            0x52347a => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x776),16));
            c.set(0,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(0,32)); }
            c.set(0,16,rm(m,c.r[0],16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x5235a6; continue; }
            pc = 0x523496; continue;
            }
            0x523496 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(3,8); c.sub(a,0x34u32,8); }
            if c.cond(4) { pc = 0x5234bf; continue; }
            pc = 0x5234aa; continue;
            }
            0x5234aa => {
            { let a = c.get(3,8); c.sub(a,0x35u32,8); }
            if c.cond(4) { pc = 0x5234bf; continue; }
            pc = 0x5234af; continue;
            }
            0x5234af => {
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(1,32)); }
            pc = 0x52358c; continue;
            }
            0x5234bf => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x768),16));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,0x0u32); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            if c.cond(3) { pc = 0x52354d; continue; }
            pc = 0x5234e1; continue;
            }
            0x5234e1 => {
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x3),8));
            c.set(5,32,c.r[1].wrapping_add(c.r[7].wrapping_mul(8)));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(2,8); c.sub(a,0x37u32,8); }
            c.set(7,16,rm(m,c.r[5].wrapping_add(0x4),16));
            if c.cond(2) { pc = 0x52352f; continue; }
            pc = 0x5234f3; continue;
            }
            0x5234f3 => {
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(3,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x3u32,32); }
            if c.cond(5) { pc = 0x52352f; continue; }
            pc = 0x52350a; continue;
            }
            0x52350a => {
            c.set(2,8,rm(m,c.r[5],8));
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)),8));
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x52352f; continue; }
            pc = 0x523514; continue;
            }
            0x523514 => {
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x1),8));
            c.set(3,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x1),8));
            { let a = c.get(2,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x52352f; continue; }
            pc = 0x52351f; continue;
            }
            0x52351f => {
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,0xfffu32); }
            c.set(7,32,0xffffu32);
            pc = 0x52352f; continue;
            }
            0x52352f => {
            { let a = c.get(7,32); c.sub(a,rm(m,c.r[4].wrapping_add(0x18),32),32); }
            if c.cond(2) { pc = 0x5234e1; continue; }
            pc = 0x523535; continue;
            }
            0x523535 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x52354d; continue; }
            pc = 0x523540; continue;
            }
            0x523540 => {
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x88); wm(m,a,32,c.get(0,32)); }
            pc = 0x52358c; continue;
            }
            0x52354d => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x88),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),16));
            { let a = c.r[7]; wm(m,a,16,c.get(2,16)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x760),16));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[6].wrapping_add(0x760); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(8)); wm(m,a,8,0xffu32); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x1); wm(m,a,8,0xffu32); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2); wm(m,a,8,0xffu32); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3); wm(m,a,8,0xffu32); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x6); wm(m,a,8,0x0u32); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x7); wm(m,a,8,0x0u32); }
            pc = 0x52358c; continue;
            }
            0x52358c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x88),32));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x776),16));
            c.set(0,16,rm(m,c.r[0],16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x523496; continue; }
            pc = 0x5235a6; continue;
            }
            0x5235a6 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x6f1),8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x5237a9; continue; }
            pc = 0x5235bb; continue;
            }
            0x5235bb => {
            c.set(0,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[6].wrapping_add(0x784),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5237a9; continue; }
            pc = 0x5235d6; continue;
            }
            0x5235d6 => {
            c.push(m,c.get(3,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5235e1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,16,rm(m,c.r[5].wrapping_add(0x2ca),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x776),16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2ce),16));
            { let a = c.r[4].wrapping_add(0x8c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x523643; continue; }
            pc = 0x52360f; continue;
            }
            0x52360f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            pc = 0x523615; continue;
            }
            0x523615 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x672b78),8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.set(3,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x523615; continue; }
            pc = 0x52363c; continue;
            }
            0x52363c => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            pc = 0x523643; continue;
            }
            0x523643 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2d0),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x523684; continue; }
            pc = 0x523650; continue;
            }
            0x523650 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            pc = 0x523656; continue;
            }
            0x523656 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x3),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x672b78),8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.set(3,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x4),16));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x523656; continue; }
            pc = 0x52367d; continue;
            }
            0x52367d => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8c),32));
            pc = 0x523684; continue;
            }
            0x523684 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x786),16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5236a1; continue; }
            pc = 0x52369d; continue;
            }
            0x52369d => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x5236a1; continue;
            }
            0x5236a1 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x2c4),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x5236b5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,c.get(2,16),16); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(0,8,u32::from(c.cond(15)));
            { let a = c.get(0,32); c.and(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x52370b; continue; }
            pc = 0x5236cf; continue;
            }
            0x5236cf => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x1c),8));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,16,c.get(0,16)); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x1d),8));
            { let a = c.get(0,16); let r = c.add(a,c.get(2,16),16); c.set(0,16,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x2c4),16));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x523707);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x52370b; continue;
            }
            0x52370b => {
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x523795; continue; }
            pc = 0x52371c; continue;
            }
            0x52371c => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,0x6dd73cu32);
            { let a = c.get(0,32); let r = c.add(a,0x1400u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x523731);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x1u32); }
            { let a = c.r[0].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0x15); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0x16); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0x17); wm(m,a,8,0x0u32); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,8,0xffu32); }
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x2c4),8));
            { let a = c.r[0].wrapping_add(0x11); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,0x0u32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4bc),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,0x2u32);
            { let a = c.r[7].wrapping_add(0x4bc); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x523795);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x529f90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x523795; continue;
            }
            0x523795 => {
            c.set(2,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x5235d6; continue; }
            pc = 0x5237a9; continue;
            }
            0x5237a9 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x523803; continue; }
            pc = 0x5237b1; continue;
            }
            0x5237b1 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            if c.cond(6) { pc = 0x5237dc; continue; }
            pc = 0x5237be; continue;
            }
            0x5237be => {
            { let a = c.get(7,32); let r = c.sub(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,0x0u32);
            c.set(0,8,rm(m,c.r[3].wrapping_add(c.r[7]).wrapping_add(0x1),8));
            c.set(1,8,rm(m,c.r[3].wrapping_add(c.r[7]),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5237d7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4fbff0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(7,32); c.sub(a,0x1u32,32); }
            if c.cond(7) { pc = 0x5237be; continue; }
            pc = 0x5237dc; continue;
            }
            0x5237dc => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x7a8),32));
            { let a = c.get(6,32); let r = c.add(a,0xfffffff4u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0xcu32,32); }
            if c.cond(14) { pc = 0x5237fa; continue; }
            pc = 0x5237ea; continue;
            }
            0x5237ea => {
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5237f2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fc130, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.sub(a,0x12u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0xcu32,32); }
            if c.cond(15) { pc = 0x5237ea; continue; }
            pc = 0x5237fa; continue;
            }
            0x5237fa => {
            c.push(m,c.get(3,32));
            c.push(m, 0x523800);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x523803; continue;
            }
            0x523803 => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x80); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x523817);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x78),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x74u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005244f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5244f0;
    loop {
        match pc {
            0x5244f0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x54u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,16,0x2u32); }
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            if c.cond(3) { pc = 0x524639; continue; }
            pc = 0x524522; continue;
            }
            0x524522 => {
            { let a = c.r[4].wrapping_add(0x62); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x52452c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x524770; continue; }
            pc = 0x524534; continue;
            }
            0x524534 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2c0),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x5e),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x52454d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x524770; continue; }
            pc = 0x524557; continue;
            }
            0x524557 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x5e),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5245b8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            c.set(7,32,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5245c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4f8a50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(0,32,rm(m,c.r[7],32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4c0),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(0x4c0); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x68),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x5245fb; continue; }
            pc = 0x5245ed; continue;
            }
            0x5245ed => {
            c.set(1,32,c.r[4].wrapping_add(0x68));
            c.push(m,c.get(1,32));
            c.push(m,0x13u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x5245fb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5245fb; continue;
            }
            0x5245fb => {
            { let a = rm(m,c.r[4].wrapping_add(0x10),32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x52460f; continue; }
            pc = 0x524601; continue;
            }
            0x524601 => {
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m,0xbu32);
            c.push(m, 0x52460f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x52460f; continue;
            }
            0x52460f => {
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x524623; continue; }
            pc = 0x524615; continue;
            }
            0x524615 => {
            c.set(0,32,c.r[4].wrapping_add(0x14));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m,0xcu32);
            c.push(m, 0x524623);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x524623; continue;
            }
            0x524623 => {
            { let a = rm(m,c.r[4].wrapping_add(0x18),32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x52464d; continue; }
            pc = 0x524629; continue;
            }
            0x524629 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,0x11u32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x524637);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x500a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x52464d; continue;
            }
            0x524639 => {
            { let a = c.r[4].wrapping_add(0x62); wm(m,a,16,0xffu32); }
            c.push(m, 0x524645);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x524770; continue; }
            pc = 0x52464d; continue;
            }
            0x52464d => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(7,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x68); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(7,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            if c.cond(14) { pc = 0x5246ab; continue; }
            pc = 0x52469e; continue;
            }
            0x52469e => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x50),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(5,32)); }
            if c.cond(9) { pc = 0x5246ab; continue; }
            pc = 0x5246a8; continue;
            }
            0x5246a8 => {
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            pc = 0x5246ab; continue;
            }
            0x5246ab => {
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x5246bc; continue; }
            pc = 0x5246af; continue;
            }
            0x5246af => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            if c.cond(9) { pc = 0x5246bc; continue; }
            pc = 0x5246b9; continue;
            }
            0x5246b9 => {
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            pc = 0x5246bc; continue;
            }
            0x5246bc => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x5246cd; continue; }
            pc = 0x5246c0; continue;
            }
            0x5246c0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x34),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,32,c.get(0,32)); }
            if c.cond(9) { pc = 0x5246cd; continue; }
            pc = 0x5246ca; continue;
            }
            0x5246ca => {
            { let a = c.r[6].wrapping_add(0x34); wm(m,a,32,c.get(3,32)); }
            pc = 0x5246cd; continue;
            }
            0x5246cd => {
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x5246de; continue; }
            pc = 0x5246d1; continue;
            }
            0x5246d1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x48),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            if c.cond(9) { pc = 0x5246de; continue; }
            pc = 0x5246db; continue;
            }
            0x5246db => {
            { let a = c.r[6].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            pc = 0x5246de; continue;
            }
            0x5246de => {
            c.set(0,32,rm(m,0x6e51acu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5246fe; continue; }
            pc = 0x5246e7; continue;
            }
            0x5246e7 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc324),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x2c0),8));
            c.set(7,32,0x1u32);
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x524702; continue; }
            pc = 0x5246fe; continue;
            }
            0x5246fe => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x524702; continue;
            }
            0x524702 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x524709);
            { let args = []; c.r[0] = host.call(m, 0x51f100, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x75c),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.r[1]; wm(m,a,8,0xffu32); }
            { let a = c.r[1].wrapping_add(0x1); wm(m,a,8,0xffu32); }
            { let a = c.r[1].wrapping_add(0x2); wm(m,a,8,0xffu32); }
            { let a = c.r[1].wrapping_add(0x3); wm(m,a,8,0x2u32); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,16,0xffffu32); }
            c.set(2,8,rm(m,0x672bb6u32,8));
            { let a = c.r[1].wrapping_add(0x6); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[1].wrapping_add(0x7); wm(m,a,8,0xffu32); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x524746);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5254c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[6].wrapping_add(0x7a4));
            c.push(m, 0x524752);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4fb580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x790),16); let r = c.add(a,c.get(0,16),16); { let a = c.r[6].wrapping_add(0x790); wm(m,a,16,r); } }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x790),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x792),16),16); }
            if c.cond(6) { pc = 0x524770; continue; }
            pc = 0x524769; continue;
            }
            0x524769 => {
            { let a = c.r[6].wrapping_add(0x792); wm(m,a,16,c.get(0,16)); }
            pc = 0x524770; continue;
            }
            0x524770 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x54u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00538000(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x538000;
    loop {
        match pc {
            0x538000 => {
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x5380fb; continue; }
            pc = 0x53801c; continue;
            }
            0x53801c => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x42),16));
            c.set(7,32,c.r[6].wrapping_add(0x42));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x5380fb; continue; }
            pc = 0x53803e; continue;
            }
            0x53803e => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x5380fb; continue; }
            pc = 0x53804c; continue;
            }
            0x53804c => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5380fb; continue; }
            pc = 0x53805f; continue;
            }
            0x53805f => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x538083; continue; }
            pc = 0x53806f; continue;
            }
            0x53806f => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53807f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x1f); wm(m,a,8,0x0u32); }
            pc = 0x538083; continue;
            }
            0x538083 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.push(m, 0x53809e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x526b00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0xfffff600u32);
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x40),16));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = rm(m,c.r[6].wrapping_add(0x3e),16); let r = c.add(a,c.get(0,16),16); { let a = c.r[6].wrapping_add(0x3e); wm(m,a,16,r); } }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x3f),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(13) { pc = 0x5380be; continue; }
            pc = 0x5380b8; continue;
            }
            0x5380b8 => {
            { let a = c.r[6].wrapping_add(0x3e); wm(m,a,16,0x0u32); }
            pc = 0x5380be; continue;
            }
            0x5380be => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = rm(m,c.r[6].wrapping_add(0x3c),16); let r = c.or(a,0x202u32,16); { let a = c.r[6].wrapping_add(0x3c); wm(m,a,16,r); } }
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x5380fb; continue; }
            pc = 0x5380d1; continue;
            }
            0x5380d1 => {
            c.set(6,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10),16));
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5380fb; continue; }
            pc = 0x5380f0; continue;
            }
            0x5380f0 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x5380fb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50a580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5380fb; continue;
            }
            0x5380fb => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005386c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5386c0;
    loop {
        match pc {
            0x5386c0 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(2,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x5387fc; continue; }
            pc = 0x5386db; continue;
            }
            0x5386db => {
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(2) { pc = 0x538729; continue; }
            pc = 0x5386f1; continue;
            }
            0x5386f1 => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x5387fc; continue; }
            pc = 0x5386ff; continue;
            }
            0x5386ff => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x53871b; continue; }
            pc = 0x538706; continue;
            }
            0x538706 => {
            { let a = rm(m,c.r[6].wrapping_add(0x3d),8); let r = c.or(a,0x18u32,8); { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,r); } }
            { let a = c.r[6].wrapping_add(0x15c); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53871b => {
            { let a = rm(m,c.r[6].wrapping_add(0x3d),8); let r = c.or(a,0x10u32,8); { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,r); } }
            { let a = c.r[6].wrapping_add(0x15c); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x538729 => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,0x1000u32);
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.or(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x53879f; continue; }
            pc = 0x538751; continue;
            }
            0x538751 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xa),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x5387fc; continue; }
            pc = 0x53876b; continue;
            }
            0x53876b => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x3c),16); let r = c.or(a,c.get(7,16),16); { let a = c.r[0].wrapping_add(0x3c); wm(m,a,16,r); } }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.r[0].wrapping_add(0x15c); wm(m,a,8,0x0u32); }
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,c.get(2,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x53876b; continue; }
            pc = 0x53879c; continue;
            }
            0x53879c => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53879f => {
            c.set(0,8,rm(m,c.r[2],8));
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x5387fc; continue; }
            pc = 0x5387a7; continue;
            }
            0x5387a7 => {
            { let a = rm(m,c.r[2].wrapping_add(0x12),16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x5387fc; continue; }
            pc = 0x5387ae; continue;
            }
            0x5387ae => {
            { let a = rm(m,c.r[2].wrapping_add(0x14),8); c.and(a,0x2u32,8); }
            if c.cond(4) { pc = 0x5387fc; continue; }
            pc = 0x5387b4; continue;
            }
            0x5387b4 => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xa),16));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x5387fc; continue; }
            pc = 0x5387ca; continue;
            }
            0x5387ca => {
            c.set(1,8,0x1u32);
            pc = 0x5387cc; continue;
            }
            0x5387cc => {
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = rm(m,c.r[0].wrapping_add(0x3c),16); let r = c.or(a,c.get(7,16),16); { let a = c.r[0].wrapping_add(0x3c); wm(m,a,16,r); } }
            { let a = c.r[0].wrapping_add(0x15c); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,c.get(2,32));
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x5387cc; continue; }
            pc = 0x5387fc; continue;
            }
            0x5387fc => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00538800(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x538800;
    loop {
        match pc {
            0x538800 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6dd894u32,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x5388c2; continue; }
            pc = 0x538819; continue;
            }
            0x538819 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x5388c2; continue; }
            pc = 0x538824; continue;
            }
            0x538824 => {
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x134),16));
            { let a = c.get(6,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x53884d; continue; }
            pc = 0x538847; continue;
            }
            0x538847 => {
            { let a = c.get(6,16); c.sub(a,0x10u32,16); }
            if c.cond(5) { pc = 0x5388c2; continue; }
            pc = 0x53884d; continue;
            }
            0x53884d => {
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x15c),8));
            c.set(6,16,rm(m,c.r[2].wrapping_add(0x134),16));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x538872; continue; }
            pc = 0x53885e; continue;
            }
            0x53885e => {
            { let a = c.get(6,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x538888; continue; }
            pc = 0x538864; continue;
            }
            0x538864 => {
            { let a = c.get(6,16); c.sub(a,0x10u32,16); }
            if c.cond(4) { pc = 0x538888; continue; }
            pc = 0x53886a; continue;
            }
            0x53886a => {
            { let a = c.get(6,16); c.sub(a,0x12u32,16); }
            if c.cond(5) { pc = 0x5388c2; continue; }
            pc = 0x538870; continue;
            }
            0x538870 => {
            pc = 0x538888; continue;
            }
            0x538872 => {
            { let a = c.get(6,16); c.sub(a,0x12u32,16); }
            if c.cond(4) { pc = 0x538888; continue; }
            pc = 0x538878; continue;
            }
            0x538878 => {
            { let a = c.get(6,16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x5388c2; continue; }
            pc = 0x53887e; continue;
            }
            0x53887e => {
            c.set(3,8,rm(m,c.r[2].wrapping_add(0x15c),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x5388c2; continue; }
            pc = 0x538888; continue;
            }
            0x538888 => {
            c.set(6,32,sx(rm(m,c.r[2].wrapping_add(0x1e),16),16)&0xffffffffu32);
            c.set(2,32,sx(rm(m,c.r[2].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.push(m,c.get(7,32));
            c.set(7,32,sx(rm(m,c.r[0].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(7,32,sx(rm(m,c.r[0].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            c.set(7,32,c.get(2,32));
            { let a = c.get(7,32); let r = c.imul(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,0x6dd89cu32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(15) { pc = 0x5388c2; continue; }
            pc = 0x5388ba; continue;
            }
            0x5388ba => {
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.r[0].wrapping_add(0x3a); wm(m,a,16,c.get(1,16)); }
            pc = 0x5388c2; continue;
            }
            0x5388c2 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00538930(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x538930;
    loop {
        match pc {
            0x538930 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x5389b7; continue; }
            pc = 0x538946; continue;
            }
            0x538946 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x5389b7; continue; }
            pc = 0x53894d; continue;
            }
            0x53894d => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x40),16));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x538975; continue; }
            pc = 0x538972; continue;
            }
            0x538972 => {
            { let a = c.r[3].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x538975; continue;
            }
            0x538975 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x53897c);
            { let args = []; c.r[0] = host.call(m, 0x5184f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[3].wrapping_add(0xc),32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x538984; continue; }
            pc = 0x538981; continue;
            }
            0x538981 => {
            { let a = c.r[3].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x538984; continue;
            }
            0x538984 => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xc),16));
            c.push(m,0x2u32);
            { let a = rm(m,c.r[6].wrapping_add(0x40),16); let r = c.sub(a,c.get(0,16),16); { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,r); } }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0xc),16));
            { let a = rm(m,c.r[7].wrapping_add(0x40),16); let r = c.add(a,c.get(1,16),16); { let a = c.r[7].wrapping_add(0x40); wm(m,a,16,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x53899d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x2u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5389a6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5389ad);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5389b7 => {
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005389c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5389c0;
    loop {
        match pc {
            0x5389c0 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x538a72; continue; }
            pc = 0x5389da; continue;
            }
            0x5389da => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x538a72; continue; }
            pc = 0x5389e5; continue;
            }
            0x5389e5 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x154),16));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x538a10; continue; }
            pc = 0x538a0d; continue;
            }
            0x538a0d => {
            { let a = c.r[3].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            pc = 0x538a10; continue;
            }
            0x538a10 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x538a17);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xc),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x538a39; continue; }
            pc = 0x538a25; continue;
            }
            0x538a25 => {
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            pc = 0x538a39; continue;
            }
            0x538a39 => {
            c.set(2,16,rm(m,c.r[3].wrapping_add(0xc),16));
            c.push(m,0x14u32);
            { let a = rm(m,c.r[6].wrapping_add(0x154),16); let r = c.sub(a,c.get(2,16),16); { let a = c.r[6].wrapping_add(0x154); wm(m,a,16,r); } }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xc),16));
            { let a = rm(m,c.r[7].wrapping_add(0x154),16); let r = c.add(a,c.get(0,16),16); { let a = c.r[7].wrapping_add(0x154); wm(m,a,16,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x538a58);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x14u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x538a61);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x538a68);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x538a72 => {
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00538c40(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x538c40;
    loop {
        match pc {
            0x538c40 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x66350cu32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6dd892u32,16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x538d8c; continue; }
            pc = 0x538c6e; continue;
            }
            0x538c6e => {
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x538d8c; continue; }
            pc = 0x538c85; continue;
            }
            0x538c85 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,0x6dd73cu32);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x538c96);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x27u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(0,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(7,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.r[7].wrapping_add(0xe); wm(m,a,8,0x1u32); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(6,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(0,8,rm(m,c.r[6],8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x538d8c; continue; }
            pc = 0x538cde; continue;
            }
            0x538cde => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x538cee);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            { let a = rm(m,c.r[3].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x538d8c; continue; }
            pc = 0x538cfb; continue;
            }
            0x538cfb => {
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x538d04);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x52u32); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x538d29; continue; }
            pc = 0x538d26; continue;
            }
            0x538d26 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x19),8));
            pc = 0x538d29; continue;
            }
            0x538d29 => {
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(0,8)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x15e),16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x1c),8));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1a); wm(m,a,16,c.get(0,16)); }
            c.push(m,c.get(2,32));
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x1d),8));
            { let a = c.get(0,16); let r = c.add(a,c.get(1,16),16); c.set(0,16,r); }
            c.set(1,32,0x6dd730u32);
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            c.push(m,c.get(0,32));
            c.push(m, 0x538d7b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x538d8c);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x538d8c; continue;
            }
            0x538d8c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00538de0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x538de0;
    loop {
        match pc {
            0x538de0 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            if c.cond(3) { pc = 0x5391fc; continue; }
            pc = 0x538e02; continue;
            }
            0x538e02 => {
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(6,32),32); c.set(3,32,r); }
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(0,8,rm(m,c.r[3],8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x5391fc; continue; }
            pc = 0x538e24; continue;
            }
            0x538e24 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x538e34);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x538e6c; continue; }
            pc = 0x538e40; continue;
            }
            0x538e40 => {
            c.push(m,0x80u32);
            c.push(m, 0x538e4a);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xec); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5391fc; continue; }
            pc = 0x538e5b; continue;
            }
            0x538e5b => {
            { let a = c.r[0]; wm(m,a,16,0x40u32); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = c.r[2].wrapping_add(0x2); wm(m,a,16,0x4u32); }
            pc = 0x538e6c; continue;
            }
            0x538e6c => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x538e84; continue; }
            pc = 0x538e73; continue;
            }
            0x538e73 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = c.r[0].wrapping_add(0x2); wm(m,a,16,0x4u32); }
            pc = 0x5391c0; continue;
            }
            0x538e84 => {
            { let a = c.get(2,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x539100; continue; }
            pc = 0x538e8d; continue;
            }
            0x538e8d => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0x11u32,16); }
            if c.cond(3) { pc = 0x538ecb; continue; }
            pc = 0x538e9a; continue;
            }
            0x538e9a => {
            { let a = c.get(0,16); c.sub(a,0xcu32,16); }
            if c.cond(2) { pc = 0x538ea6; continue; }
            pc = 0x538ea0; continue;
            }
            0x538ea0 => {
            { let a = c.get(0,16); c.sub(a,0x6u32,16); }
            if c.cond(6) { pc = 0x538ecb; continue; }
            pc = 0x538ea6; continue;
            }
            0x538ea6 => {
            { let a = c.get(0,16); c.sub(a,0xeu32,16); }
            if c.cond(4) { pc = 0x538ecb; continue; }
            pc = 0x538eac; continue;
            }
            0x538eac => {
            { let a = rm(m,c.r[3].wrapping_add(0x18),32); c.sub(a,0x1u32,32); }
            if c.cond(12) { pc = 0x538ecb; continue; }
            pc = 0x538eb2; continue;
            }
            0x538eb2 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x39),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x538eea; continue; }
            pc = 0x538ec0; continue;
            }
            0x538ec0 => {
            c.set(1,32,c.get(3,32));
            c.push(m, 0x538ec7);
            { let args = []; c.r[0] = host.call(m, 0x519ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x538eea; continue; }
            pc = 0x538ecb; continue;
            }
            0x538ecb => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5391fc; continue; }
            pc = 0x538ed9; continue;
            }
            0x538ed9 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x538ee2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x44f940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x538eea => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.get(1,16); c.sub(a,0x5u32,16); }
            if c.cond(6) { pc = 0x5391fc; continue; }
            pc = 0x538efe; continue;
            }
            0x538efe => {
            c.set(7,32,0x4u32);
            { let a = c.get(1,16); c.sub(a,c.get(7,16),16); }
            if c.cond(6) { pc = 0x538f83; continue; }
            pc = 0x538f08; continue;
            }
            0x538f08 => {
            c.set(5,32,0xau32);
            pc = 0x538f0d; continue;
            }
            0x538f0d => {
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[5]),16));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)),16));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x538f1c);
            { let args = []; c.r[0] = host.call(m, 0x517260, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x538f34; continue; }
            pc = 0x538f23; continue;
            }
            0x538f23 => {
            { let a = c.get(0,32); c.sub(a,rm(m,0x6de4b0u32,32),32); }
            if c.cond(3) { pc = 0x538f34; continue; }
            pc = 0x538f2b; continue;
            }
            0x538f2b => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x39),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x538f4c; continue; }
            pc = 0x538f34; continue;
            }
            0x538f34 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x538f0d; continue; }
            pc = 0x538f4a; continue;
            }
            0x538f4a => {
            pc = 0x538f7f; continue;
            }
            0x538f4c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)),16));
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[7].wrapping_mul(2)).wrapping_add(0x2),16));
            { let a = c.get(7,32); let r = c.add(a,0x2u32,32); c.set(7,32,r); }
            { let a = c.r[1].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = c.r[0].wrapping_add(0x2); wm(m,a,16,c.get(7,16)); }
            pc = 0x538f7f; continue;
            }
            0x538f7f => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x538f83; continue;
            }
            0x538f83 => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x538ff0; continue; }
            pc = 0x538f89; continue;
            }
            0x538f89 => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0x3u32,16); }
            if c.cond(4) { pc = 0x538fac; continue; }
            pc = 0x538f96; continue;
            }
            0x538f96 => {
            { let a = c.get(0,16); c.sub(a,0x2u32,16); }
            if c.cond(4) { pc = 0x538fac; continue; }
            pc = 0x538f9c; continue;
            }
            0x538f9c => {
            c.set(1,32,sx(rm(m,c.r[3].wrapping_add(0x1e),16),16)&0xffffffffu32);
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,sx(rm(m,c.r[3].wrapping_add(0x22),16),16)&0xffffffffu32);
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            pc = 0x538ff0; continue;
            }
            0x538fac => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x39),8));
            c.set(1,32,0x6de4b9u32);
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1],8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbb0),32));
            c.set(0,32,0x6de4b9u32);
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x39),8));
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            c.set(0,32,c.get(1,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbb4),32));
            { let a = c.r[5].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            pc = 0x538ff0; continue;
            }
            0x538ff0 => {
            c.set(1,32,c.get(5,32));
            c.push(m, 0x538ff7);
            { let args = []; c.r[0] = host.call(m, 0x537670, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x36),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x539020; continue; }
            pc = 0x538ffe; continue;
            }
            0x538ffe => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x24),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x539020; continue; }
            pc = 0x539005; continue;
            }
            0x539005 => {
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x539014; continue; }
            pc = 0x53900b; continue;
            }
            0x53900b => {
            c.push(m,c.get(0,32));
            c.push(m, 0x539011);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x539014; continue;
            }
            0x539014 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x24),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x53901d);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x539020; continue;
            }
            0x539020 => {
            c.set(5,32,c.r[3].wrapping_add(0x24));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[5]; wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.get(1,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x1e),16));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x22),16));
            { let a = c.r[2].wrapping_add(0x6); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            if c.cond(14) { pc = 0x5390ce; continue; }
            pc = 0x53906f; continue;
            }
            0x53906f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,0x2u32);
            c.set(7,32,0x4u32);
            pc = 0x53907d; continue;
            }
            0x53907d => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[1]),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0xfffffffc),16),16); }
            if c.cond(5) { pc = 0x5390c3; continue; }
            pc = 0x539088; continue;
            }
            0x539088 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0x2),16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7].wrapping_add(c.r[1]).wrapping_add(0xfffffffe),16),16); }
            if c.cond(5) { pc = 0x5390c3; continue; }
            pc = 0x539094; continue;
            }
            0x539094 => {
            { let a = c.get(6,32); c.sub(a,c.get(2,32),32); }
            c.set(0,32,c.get(6,32));
            if c.cond(13) { pc = 0x5390be; continue; }
            pc = 0x53909a; continue;
            }
            0x53909a => {
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xfffffffc); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2),16));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xfffffffe); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x53909a; continue; }
            pc = 0x5390be; continue;
            }
            0x5390be => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            pc = 0x5390c3; continue;
            }
            0x5390c3 => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0xffffffff));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x53907d; continue; }
            pc = 0x5390ce; continue;
            }
            0x5390ce => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbdc8u32,32));
            c.push(m, 0x5390df);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x445010, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[5],32));
            { let a = c.r[3].wrapping_add(0x36); wm(m,a,8,c.get(0,8)); }
            c.set(6,32,rm(m,0x6de4b4u32,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5390f3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5169c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[3].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x539100 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(7,32,c.r[2].wrapping_add(c.r[2]).wrapping_add(0x4));
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x2),16));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x539117; continue; }
            pc = 0x539114; continue;
            }
            0x539114 => {
            c.push(m,c.get(1,32));
            pc = 0x539165; continue;
            }
            0x539117 => {
            if c.cond(3) { pc = 0x53913c; continue; }
            pc = 0x539119; continue;
            }
            0x539119 => {
            c.set(0,16,rm(m,c.r[5].wrapping_add(0xc),16));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x10),16));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xa); wm(m,a,16,c.get(0,16)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53913c => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            c.set(2,16,rm(m,c.r[1],16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x5391c0; continue; }
            pc = 0x539148; continue;
            }
            0x539148 => {
            c.set(1,32,c.r[2].wrapping_mul(4));
            c.push(m,c.get(1,32));
            c.push(m, 0x539155);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x53917f; continue; }
            pc = 0x53915e; continue;
            }
            0x53915e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.push(m,c.get(2,32));
            pc = 0x539165; continue;
            }
            0x539165 => {
            c.push(m, 0x53916a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xec); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53917f => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x2),16));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x5391a2; continue; }
            pc = 0x53918f; continue;
            }
            0x53918f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(2)),16));
            { let a = c.r[7].wrapping_add(c.r[0].wrapping_mul(2)); wm(m,a,16,c.get(1,16)); }
            if c.cond(15) { pc = 0x53918f; continue; }
            pc = 0x5391a2; continue;
            }
            0x5391a2 => {
            c.set(2,16,rm(m,c.r[7],16));
            { let a = c.get(2,16); let r = c.add(a,c.get(2,16),16); c.set(2,16,r); }
            { let a = c.r[7]; wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5391b7);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0xec); wm(m,a,32,c.get(7,32)); }
            pc = 0x5391c0; continue;
            }
            0x5391c0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.set(2,16,rm(m,c.r[5].wrapping_add(0xc),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x2),16); let r = c.inc(a,16); { let a = c.r[0].wrapping_add(0x2); wm(m,a,16,r); } }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xec),32));
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x10),16));
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x2),16));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(2,16)); }
            c.set(6,32,rm(m,c.r[6].wrapping_add(0xec),32));
            { let a = rm(m,c.r[6].wrapping_add(0x2),16); let r = c.inc(a,16); { let a = c.r[6].wrapping_add(0x2); wm(m,a,16,r); } }
            pc = 0x5391fc; continue;
            }
            0x5391fc => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00539270(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x539270;
    loop {
        match pc {
            0x539270 => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5392fe; continue; }
            pc = 0x539282; continue;
            }
            0x539282 => {
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x5392fe; continue; }
            pc = 0x53929d; continue;
            }
            0x53929d => {
            c.set(6,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(6,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            c.set(1,32,0x6dd7a0u32);
            if c.cond(4) { pc = 0x5392c8; continue; }
            pc = 0x5392c0; continue;
            }
            0x5392c0 => {
            c.push(m,c.get(2,32));
            c.push(m, 0x5392c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x509f80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5392ce; continue;
            }
            0x5392c8 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5392ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x50a580, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5392ce; continue;
            }
            0x5392ce => {
            c.set(2,16,rm(m,0x6e627cu32,16));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x1c),16),16); }
            if c.cond(5) { pc = 0x5392fe; continue; }
            pc = 0x5392db; continue;
            }
            0x5392db => {
            c.push(m,0x1u32);
            c.push(m,0xffu32);
            c.set(1,32,0x6e6220u32);
            c.push(m, 0x5392ec);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6141d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x1c),8));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6e6220u32);
            c.push(m, 0x5392fe);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6141d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5392fe; continue;
            }
            0x5392fe => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00539990(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x539990;
    loop {
        match pc {
            0x539990 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x66351eu32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,16,rm(m,0x6dd896u32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x539b24; continue; }
            pc = 0x5399c1; continue;
            }
            0x5399c1 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1c),16));
            c.set(3,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x539b24; continue; }
            pc = 0x5399e3; continue;
            }
            0x5399e3 => {
            c.set(5,32,0x4u32);
            { let a = rm(m,c.r[3].wrapping_add(0x12),16); c.sub(a,c.get(5,16),16); }
            if c.cond(3) { pc = 0x539b24; continue; }
            pc = 0x5399f2; continue;
            }
            0x5399f2 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[3].wrapping_add(0x12); wm(m,a,16,c.get(5,16)); }
            c.push(m, 0x539a00);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,0x6dd7a8u32,32));
            c.set(6,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0xa),16));
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7],8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x539a3f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x1c),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(2,8)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x1c),16));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x26); wm(m,a,16,0x0u32); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,16,c.get(5,16)); }
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x1u32); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x539a78);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x52a950, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x539ad4; continue; }
            pc = 0x539a8f; continue;
            }
            0x539a8f => {
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.set(1,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(6,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = rm(m,c.r[1].wrapping_add(0x14),32); c.idiv32(b); }
            { let a = rm(m,c.r[4].wrapping_add(0x22),16); let r = c.add(a,c.get(0,16),16); { let a = c.r[4].wrapping_add(0x22); wm(m,a,16,r); } }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x18),32));
            c.set(6,16,rm(m,c.r[1].wrapping_add(0x6),16));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(2) { pc = 0x539a8f; continue; }
            pc = 0x539ad0; continue;
            }
            0x539ad0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x539ad4; continue;
            }
            0x539ad4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x22),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,c.get(5,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(7,32); c.idiv32(b); }
            { let a = c.r[4].wrapping_add(0x26); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x4ac),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(1,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x4ac); wm(m,a,32,c.get(7,32)); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x1c),16));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x539b12);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4f9120, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x539b23);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x539b24; continue;
            }
            0x539b24 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00539cf0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x539cf0;
    loop {
        match pc {
            0x539cf0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6dd896u32,16));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x539dae; continue; }
            pc = 0x539d06; continue;
            }
            0x539d06 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(7,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x539dae; continue; }
            pc = 0x539d17; continue;
            }
            0x539d17 => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x1f),8));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x539dae; continue; }
            pc = 0x539d2f; continue;
            }
            0x539d2f => {
            { let a = rm(m,c.r[2].wrapping_add(0x12),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x539dae; continue; }
            pc = 0x539d36; continue;
            }
            0x539d36 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x1c),16));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x539dae; continue; }
            pc = 0x539d40; continue;
            }
            0x539d40 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x8),8));
            c.set(6,32,0x1u32);
            { let a = c.r[2].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            c.set(1,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(1,16); let n = 0x8u32; let r = c.shl(a,n,16); c.set(1,16,r); }
            { let a = c.r[2].wrapping_add(0x16); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[2].wrapping_add(0x12); wm(m,a,16,c.get(6,16)); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x539dae; continue; }
            pc = 0x539d6f; continue;
            }
            0x539d6f => {
            c.set(7,32,0x20u32);
            c.push(m,c.get(3,32));
            pc = 0x539d75; continue;
            }
            0x539d75 => {
            c.set(3,16,rm(m,c.r[2].wrapping_add(0x16),16));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = rm(m,c.r[0].wrapping_add(0x3c),16); let r = c.or(a,c.get(7,16),16); { let a = c.r[0].wrapping_add(0x3c); wm(m,a,16,r); } }
            { let a = c.r[0].wrapping_add(0x138); wm(m,a,16,c.get(3,16)); }
            { let a = c.r[0].wrapping_add(0x134); wm(m,a,16,c.get(6,16)); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x539d75; continue; }
            pc = 0x539dad; continue;
            }
            0x539dad => {
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x539dae; continue;
            }
            0x539dae => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053ac50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53ac50;
    loop {
        match pc {
            0x53ac50 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53ad02; continue; }
            pc = 0x53ac64; continue;
            }
            0x53ac64 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x53ad02; continue; }
            pc = 0x53ac78; continue;
            }
            0x53ac78 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,0x6de508u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x6dc),8));
            c.set(2,32,0x4u32);
            pc = 0x53ac9e; continue;
            }
            0x53ac9e => {
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x53aca3; continue; }
            pc = 0x53aca2; continue;
            }
            0x53aca2 => {
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            pc = 0x53aca3; continue;
            }
            0x53aca3 => {
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x53ac9e; continue; }
            pc = 0x53aca6; continue;
            }
            0x53aca6 => {
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53acb1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x4bc),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[0].wrapping_add(0x4bc); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x53acda);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4f8e30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); c.sub(a,0x2u32,32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(13) { pc = 0x53ad02; continue; }
            pc = 0x53ace1; continue;
            }
            0x53ace1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x8),8));
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x6dc); wm(m,a,8,c.get(1,8)); }
            pc = 0x53ad02; continue;
            }
            0x53ad02 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053ad10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53ad10;
    loop {
        match pc {
            0x53ad10 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x53ad88; continue; }
            pc = 0x53ad1f; continue;
            }
            0x53ad1f => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x53ad88; continue; }
            pc = 0x53ad2f; continue;
            }
            0x53ad2f => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53ad3b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x53ad42);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f8e90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,0x4u32,32); }
            if c.cond(3) { pc = 0x53ad88; continue; }
            pc = 0x53ad4a; continue;
            }
            0x53ad4a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x6dc));
            c.set(0,8,rm(m,c.r[1],8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(7,32),32); }
            { let v = c.pop(m); c.set(7,32,v); }
            if c.cond(4) { pc = 0x53ad85; continue; }
            pc = 0x53ad78; continue;
            }
            0x53ad78 => {
            c.set(0,16,c.get(0,8));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x53ad88; continue; }
            pc = 0x53ad85; continue;
            }
            0x53ad85 => {
            { let a = c.r[1]; wm(m,a,8,0x1u32); }
            pc = 0x53ad88; continue;
            }
            0x53ad88 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053cf50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53cf50;
    loop {
        match pc {
            0x53cf50 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53d112; continue; }
            pc = 0x53cf6e; continue;
            }
            0x53cf6e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53d112; continue; }
            pc = 0x53cf82; continue;
            }
            0x53cf82 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53cfa8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x53d112; continue; }
            pc = 0x53cfb6; continue;
            }
            0x53cfb6 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x2c8),8));
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(2,8); c.and(a,0x10u32,8); }
            c.set(7,32,c.r[1].wrapping_add(0x2d0));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            if c.cond(4) { pc = 0x53cff0; continue; }
            pc = 0x53cfd7; continue;
            }
            0x53cfd7 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x7ac),32));
            { let a = c.get(2,32); let r = c.imul(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[3].wrapping_add(0x80c),32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[6]),8); c.sub(a,0x0u32,8); }
            if c.cond(9) { pc = 0x53d112; continue; }
            pc = 0x53cff0; continue;
            }
            0x53cff0 => {
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[3].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x53d01f; continue; }
            pc = 0x53cffc; continue;
            }
            0x53cffc => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x53d00d; continue; }
            pc = 0x53d000; continue;
            }
            0x53d000 => {
            c.set(7,32,c.r[1].wrapping_add(0x2ce));
            c.set(5,32,0x1u32);
            pc = 0x53cff0; continue;
            }
            0x53d00d => {
            { let a = c.get(5,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x53d050; continue; }
            pc = 0x53d012; continue;
            }
            0x53d012 => {
            c.set(7,32,c.r[3].wrapping_add(0x762));
            c.set(5,32,0x2u32);
            pc = 0x53cff0; continue;
            }
            0x53d01f => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x53d02c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(6,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x53d047; continue; }
            pc = 0x53d03a; continue;
            }
            0x53d03a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x53d05d; continue; }
            pc = 0x53d047; continue;
            }
            0x53d047 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(7,32,c.r[6].wrapping_add(0x4));
            pc = 0x53cff0; continue;
            }
            0x53d050 => {
            c.push(m, 0x53d055);
            { let args = []; c.r[0] = host.call(m, 0x4ff2d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53d05d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[7],16));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x4),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x53d07f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4dafe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = rm(m,c.r[4].wrapping_add(0x20),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x53d112; continue; }
            pc = 0x53d08a; continue;
            }
            0x53d08a => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.r[7]; wm(m,a,16,c.get(0,16)); }
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,16,0xffffu32); }
            c.set(1,32,rm(m,0x6e51acu32,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x53d0c1; continue; }
            pc = 0x53d0a5; continue;
            }
            0x53d0a5 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc324),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x53d0c1; continue; }
            pc = 0x53d0b2; continue;
            }
            0x53d0b2 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x53d0c1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x599040, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x53d0c1; continue;
            }
            0x53d0c1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x3),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[3].wrapping_add(0x7a4));
            c.push(m, 0x53d0e2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4facd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m, 0x53d0eb);
            { let args = []; c.r[0] = host.call(m, 0x4ff2d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[6].wrapping_add(0x3),8); c.sub(a,0x15u32,8); }
            if c.cond(3) { pc = 0x53d10a; continue; }
            pc = 0x53d0f1; continue;
            }
            0x53d0f1 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53d103);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            c.push(m, 0x53d10a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f8d70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x53d10a; continue;
            }
            0x53d10a => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x53d112);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f050, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x53d112; continue;
            }
            0x53d112 => {
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

pub fn f_0053d190(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53d190;
    loop {
        match pc {
            0x53d190 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53d2a6; continue; }
            pc = 0x53d1aa; continue;
            }
            0x53d1aa => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53d2a6; continue; }
            pc = 0x53d1be; continue;
            }
            0x53d1be => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x4),16));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53d1e4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x53d2a6; continue; }
            pc = 0x53d1f2; continue;
            }
            0x53d1f2 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x2c8),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x10),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(7,32,c.r[1].wrapping_add(0x2d0));
            { let a = c.get(2,8); c.and(a,0x10u32,8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x53d22a; continue; }
            pc = 0x53d213; continue;
            }
            0x53d213 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x7ac),32));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[6].wrapping_add(0x80c),32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(c.r[0]),8); c.and(a,0x80u32,8); }
            if c.cond(4) { pc = 0x53d2a6; continue; }
            pc = 0x53d22a; continue;
            }
            0x53d22a => {
            c.set(0,16,rm(m,c.r[7],16));
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[6].wrapping_add(0x776),16),16); }
            if c.cond(2) { pc = 0x53d259; continue; }
            pc = 0x53d236; continue;
            }
            0x53d236 => {
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x53d247; continue; }
            pc = 0x53d23a; continue;
            }
            0x53d23a => {
            c.set(7,32,c.r[1].wrapping_add(0x2ce));
            c.set(3,32,0x1u32);
            pc = 0x53d22a; continue;
            }
            0x53d247 => {
            { let a = c.get(3,32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x53d288; continue; }
            pc = 0x53d24c; continue;
            }
            0x53d24c => {
            c.set(7,32,c.r[6].wrapping_add(0x762));
            c.set(3,32,0x2u32);
            pc = 0x53d22a; continue;
            }
            0x53d259 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x53d266);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51f1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x53d27f; continue; }
            pc = 0x53d272; continue;
            }
            0x53d272 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x53d294; continue; }
            pc = 0x53d27f; continue;
            }
            0x53d27f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(7,32,c.r[0].wrapping_add(0x4));
            pc = 0x53d22a; continue;
            }
            0x53d288 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x4ff2d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53d294 => {
            c.set(0,16,rm(m,c.r[7],16));
            c.set(1,8,rm(m,c.r[5].wrapping_add(0x4),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x53d2a6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4dab00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x53d2a6; continue;
            }
            0x53d2a6 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053da30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53da30;
    loop {
        match pc {
            0x53da30 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66358du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,0x14u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x53da50);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x53da7e; continue; }
            pc = 0x53da63; continue;
            }
            0x53da63 => {
            c.set(1,32,rm(m,0x6de584u32,32));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x53da75);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5338a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x53da7e; continue; }
            pc = 0x53da79; continue;
            }
            0x53da79 => {
            { let a = 0x6de584u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x53da7e; continue;
            }
            0x53da7e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053da90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53da90;
    loop {
        match pc {
            0x53da90 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66359fu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x53dab4);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x53dbcd; continue; }
            pc = 0x53daca; continue;
            }
            0x53daca => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x53dadb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x15u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,16,c.get(2,16)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa),8));
            { let a = c.r[0].wrapping_add(0xe); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xb),8));
            { let a = c.r[0].wrapping_add(0xf); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = c.r[0].wrapping_add(0x11); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xe),8));
            { let a = c.r[0].wrapping_add(0x12); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xf),8));
            { let a = c.r[0].wrapping_add(0x13); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x10),8));
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = c.r[0].wrapping_add(0x15); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x53dbcd; continue; }
            pc = 0x53db34; continue;
            }
            0x53db34 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53db45);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6dd730u32);
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xa),16));
            c.push(m,c.get(2,32));
            c.push(m, 0x53db58);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d6fd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xe),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x33u32); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x19),8));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,c.r[4].wrapping_add(0x1a));
            c.set(2,32,c.r[4].wrapping_add(0xb));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[4].wrapping_add(0xe));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53db9f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5311d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(2,16,rm(m,c.r[4].wrapping_add(0xa),8));
            c.set(0,16,rm(m,c.r[4].wrapping_add(0xb),8));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0xffffffff));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x11),8));
            c.set(2,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x53dbcc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x53dbcd; continue;
            }
            0x53dbcd => {
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x53dbde);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053dbf0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53dbf0;
    loop {
        match pc {
            0x53dbf0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53dc75; continue; }
            pc = 0x53dc03; continue;
            }
            0x53dc03 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53dc0e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x53dc75; continue; }
            pc = 0x53dc17; continue;
            }
            0x53dc17 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53dc25);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x7u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,0x2u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53dc4c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x53dc63);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4bc),32));
            { let a = c.get(0,32); let r = c.add(a,0x4bcu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            pc = 0x53dc75; continue;
            }
            0x53dc75 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053ddf0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53ddf0;
    loop {
        match pc {
            0x53ddf0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6635b1u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x53deee; continue; }
            pc = 0x53de1d; continue;
            }
            0x53de1d => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x53deee; continue; }
            pc = 0x53de2d; continue;
            }
            0x53de2d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6dd892u32,16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x53deee; continue; }
            pc = 0x53de41; continue;
            }
            0x53de41 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xc),16));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53de57);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x53deee; continue; }
            pc = 0x53de61; continue;
            }
            0x53de61 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x2f2),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x53deee; continue; }
            pc = 0x53de71; continue;
            }
            0x53de71 => {
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x53de88);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x10),8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            { let a = c.r[7].wrapping_add(0xa); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(7,32));
            c.push(m, 0x53de9d);
            { let args = []; c.r[0] = host.call(m, 0x4fe160, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xc),8));
            c.set(1,32,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x7cu32); }
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x53deb0);
            { let args = []; c.r[0] = host.call(m, 0x500f10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,c.r[4].wrapping_add(0xc));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x53dedc);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x53deed);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x53deee; continue;
            }
            0x53deee => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053df00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53df00;
    loop {
        match pc {
            0x53df00 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53e474; continue; }
            pc = 0x53df1d; continue;
            }
            0x53df1d => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            if c.cond(5) { pc = 0x53dfff; continue; }
            pc = 0x53df3a; continue;
            }
            0x53df3a => {
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(2) { pc = 0x53df6f; continue; }
            pc = 0x53df43; continue;
            }
            0x53df43 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x136),8));
            { let a = rm(m,c.r[6].wrapping_add(0x3d),8); let r = c.or(a,0x8u32,8); { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,r); } }
            { let a = c.get(2,8); let r = c.and(a,0xfcu32,8); c.set(2,8,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0x1u32,16); }
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,c.get(2,8)); }
            if c.cond(7) { pc = 0x53e474; continue; }
            pc = 0x53df64; continue;
            }
            0x53df64 => {
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,8,c.get(2,8)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53df6f => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x12),16); c.sub(a,0x1u32,16); }
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            if c.cond(7) { pc = 0x53df91; continue; }
            pc = 0x53df8b; continue;
            }
            0x53df8b => {
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x1c),8));
            { let a = c.r[2].wrapping_add(0x1f); wm(m,a,8,c.get(0,8)); }
            pc = 0x53df91; continue;
            }
            0x53df91 => {
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x53e474; continue; }
            pc = 0x53dfa9; continue;
            }
            0x53dfa9 => {
            c.set(3,8,0xfcu32);
            c.set(6,32,0x800u32);
            pc = 0x53dfb0; continue;
            }
            0x53dfb0 => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.sub(a,rm(m,c.r[2],8),8); }
            if c.cond(5) { pc = 0x53dfd6; continue; }
            pc = 0x53dfc4; continue;
            }
            0x53dfc4 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x136),8));
            { let a = c.get(1,8); let r = c.and(a,c.get(3,8),8); c.set(1,8,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x3c),16); let r = c.or(a,c.get(6,16),16); { let a = c.r[0].wrapping_add(0x3c); wm(m,a,16,r); } }
            { let a = c.r[0].wrapping_add(0x136); wm(m,a,8,c.get(1,8)); }
            pc = 0x53dfd6; continue;
            }
            0x53dfd6 => {
            { let a = rm(m,c.r[0].wrapping_add(0x134),16); c.sub(a,0x1u32,16); }
            if c.cond(7) { pc = 0x53dfe6; continue; }
            pc = 0x53dfe0; continue;
            }
            0x53dfe0 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x39),8));
            { let a = c.r[0].wrapping_add(0x38); wm(m,a,8,c.get(1,8)); }
            pc = 0x53dfe6; continue;
            }
            0x53dfe6 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6),16));
            c.set(0,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x53dfb0; continue; }
            pc = 0x53dffa; continue;
            }
            0x53dffa => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53dfff => {
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x53e032; continue; }
            pc = 0x53e008; continue;
            }
            0x53e008 => {
            c.set(2,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            pc = 0x53e043; continue;
            }
            0x53e032 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x42),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x53e474; continue; }
            pc = 0x53e043; continue;
            }
            0x53e043 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x15c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53e474; continue; }
            pc = 0x53e051; continue;
            }
            0x53e051 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x132),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd72au32,16),16); }
            if c.cond(3) { pc = 0x53e461; continue; }
            pc = 0x53e065; continue;
            }
            0x53e065 => {
            c.set(1,16,rm(m,0x6dd892u32,16));
            { let a = rm(m,c.r[6].wrapping_add(0x42),16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x53e461; continue; }
            pc = 0x53e076; continue;
            }
            0x53e076 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,c.get(6,32));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x53e084);
            { let args = []; c.r[0] = host.call(m, 0x51a3c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.set(1,32,0x6dd728u32);
            c.push(m, 0x53e08f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d5ab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53e0d2; continue; }
            pc = 0x53e093; continue;
            }
            0x53e093 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6dd72cu32,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x132),16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),16));
            { let a = c.r[6].wrapping_add(0x132); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x53e474; continue; }
            pc = 0x53e0c3; continue;
            }
            0x53e0c3 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6dd728u32);
            c.push(m, 0x53e0ce);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d5ab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x53e093; continue; }
            pc = 0x53e0d2; continue;
            }
            0x53e0d2 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x53e0d9);
            { let args = []; c.r[0] = host.call(m, 0x51a450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(5) { pc = 0x53e135; continue; }
            pc = 0x53e0e6; continue;
            }
            0x53e0e6 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x3d),8); let r = c.or(a,0x8u32,8); { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,r); } }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x132),16));
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,0x1u32); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,0x6dd72cu32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,8,rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2),8));
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53e474; continue; }
            pc = 0x53e11f; continue;
            }
            0x53e11f => {
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x4),16));
            c.push(m,c.get(1,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x53e130);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5172a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53e135 => {
            { let a = c.get(0,16); c.sub(a,0x8u32,16); }
            if c.cond(5) { pc = 0x53e1ed; continue; }
            pc = 0x53e13f; continue;
            }
            0x53e13f => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            c.set(1,32,rm(m,0x6de508u32,32));
            { let b = rm(m,0x6de4b0u32,32); c.div32(b); }
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x994),16));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x53e189; continue; }
            pc = 0x53e16e; continue;
            }
            0x53e16e => {
            { let a = c.r[0].wrapping_add(c.r[1]).wrapping_add(0x990); wm(m,a,32,0x0u32); }
            c.set(1,32,rm(m,0x6de508u32,32));
            { let a = c.r[0].wrapping_add(c.r[1]).wrapping_add(0x994); wm(m,a,16,0xffffu32); }
            pc = 0x53e189; continue;
            }
            0x53e189 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x3c),16); let r = c.or(a,0x820u32,16); { let a = c.r[6].wrapping_add(0x3c); wm(m,a,16,r); } }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x132),16));
            { let a = c.r[6].wrapping_add(0x37); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6dd72cu32,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x2),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.get(0,8); c.sub(a,c.get(1,8),8); }
            { let a = c.r[6].wrapping_add(0x38); wm(m,a,8,c.get(0,8)); }
            if c.cond(5) { pc = 0x53e1df; continue; }
            pc = 0x53e1bf; continue;
            }
            0x53e1bf => {
            c.set(2,16,0x40u32);
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,16,0x3u32); }
            { let a = c.get(2,16); let r = c.sub(a,rm(m,c.r[6].wrapping_add(0x138),16),16); c.set(2,16,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,16,c.get(2,16)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53e1df => {
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,16,0x2u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53e1ed => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x53e1f4);
            { let args = []; c.r[0] = host.call(m, 0x519ba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x53e474; continue; }
            pc = 0x53e1fc; continue;
            }
            0x53e1fc => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x134),16));
            { let a = rm(m,c.r[6].wrapping_add(0x3d),8); let r = c.or(a,0x8u32,8); { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,r); } }
            { let a = c.get(0,16); c.sub(a,0x1u32,16); }
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,0x1u32); }
            if c.cond(7) { pc = 0x53e299; continue; }
            pc = 0x53e218; continue;
            }
            0x53e218 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x53e28b; continue; }
            pc = 0x53e225; continue;
            }
            0x53e225 => {
            c.set(1,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.r[1].wrapping_add(0x16); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53e474; continue; }
            pc = 0x53e259; continue;
            }
            0x53e259 => {
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.r[0].wrapping_add(0x138); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x53e259; continue; }
            pc = 0x53e286; continue;
            }
            0x53e286 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53e28b => {
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,16,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53e299 => {
            { let a = c.get(0,16); c.sub(a,0x6u32,16); }
            if c.cond(5) { pc = 0x53e474; continue; }
            pc = 0x53e2a3; continue;
            }
            0x53e2a3 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(3,16,rm(m,0x6dd896u32,16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x39),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.get(1,16); c.sub(a,c.get(3,16),16); }
            if c.cond(3) { pc = 0x53e392; continue; }
            pc = 0x53e2c9; continue;
            }
            0x53e2c9 => {
            c.set(7,32,rm(m,0x6dd7a8u32,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x820));
            c.set(0,16,rm(m,c.r[2],16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(4) { pc = 0x53e300; continue; }
            pc = 0x53e2de; continue;
            }
            0x53e2de => {
            { let a = c.get(0,16); c.sub(a,c.get(3,16),16); }
            if c.cond(3) { pc = 0x53e474; continue; }
            pc = 0x53e2e7; continue;
            }
            0x53e2e7 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            c.set(2,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x8));
            if c.cond(5) { pc = 0x53e2de; continue; }
            pc = 0x53e300; continue;
            }
            0x53e300 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x8),16));
            { let a = c.r[2]; wm(m,a,16,c.get(0,16)); }
            c.set(2,16,rm(m,0x6dd88cu32,16));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = 0x6dd88cu32; wm(m,a,16,c.get(0,16)); }
            { let a = c.r[1].wrapping_add(0x12); wm(m,a,16,c.get(2,16)); }
            c.set(6,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xa),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x53e37e; continue; }
            pc = 0x53e348; continue;
            }
            0x53e348 => {
            c.set(6,32,0x100u32);
            pc = 0x53e34d; continue;
            }
            0x53e34d => {
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = rm(m,c.r[0].wrapping_add(0x3c),16); let r = c.or(a,c.get(6,16),16); { let a = c.r[0].wrapping_add(0x3c); wm(m,a,16,r); } }
            { let a = c.r[0].wrapping_add(0x134); wm(m,a,16,c.get(2,16)); }
            c.set(0,32,c.get(7,32));
            c.set(7,32,rm(m,0x6dd894u32,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x53e34d; continue; }
            pc = 0x53e37e; continue;
            }
            0x53e37e => {
            c.set(1,16,rm(m,c.r[1].wrapping_add(0x10),16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6e6220u32);
            c.push(m, 0x53e38d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x613920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53e392 => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x81e));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x81e),16));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x53e3dc; continue; }
            pc = 0x53e3af; continue;
            }
            0x53e3af => {
            c.set(3,32,rm(m,0x6dd7a4u32,32));
            pc = 0x53e3b5; continue;
            }
            0x53e3b5 => {
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x53e474; continue; }
            pc = 0x53e3c2; continue;
            }
            0x53e3c2 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(c.r[3]).wrapping_add(0x6),16));
            c.set(1,32,c.r[1].wrapping_add(c.r[3]).wrapping_add(0x6));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x53e3b5; continue; }
            pc = 0x53e3dc; continue;
            }
            0x53e3dc => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.set(7,32,0xffffu32);
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,c.get(7,16)); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.and(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x53e43b; continue; }
            pc = 0x53e3fa; continue;
            }
            0x53e3fa => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x6),16); c.sub(a,c.get(7,16),16); }
            if c.cond(5) { pc = 0x53e43b; continue; }
            pc = 0x53e40d; continue;
            }
            0x53e40d => {
            { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x134); wm(m,a,16,0x0u32); }
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x3c),8); let r = c.or(a,0x20u32,8); { let a = c.r[0].wrapping_add(c.r[2]).wrapping_add(0x3c); wm(m,a,8,r); } }
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            c.set(2,16,rm(m,0x6dd888u32,16));
            { let a = c.r[0].wrapping_add(c.r[7]).wrapping_add(0x6); wm(m,a,16,c.get(2,16)); }
            { let a = 0x6dd888u32; wm(m,a,16,c.get(1,16)); }
            pc = 0x53e43b; continue;
            }
            0x53e43b => {
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,16,0x5u32); }
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,16,0x0u32); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x4),16));
            c.set(1,32,0x6e6220u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53e45c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x613920, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53e461 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x136),8));
            { let a = c.get(1,8); let r = c.and(a,0xfcu32,8); c.set(1,8,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x3d),8); let r = c.or(a,0x8u32,8); { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,r); } }
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,c.get(1,8)); }
            pc = 0x53e474; continue;
            }
            0x53e474 => {
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

pub fn f_0053e480(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53e480;
    loop {
        match pc {
            0x53e480 => {
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd72au32,16),16); }
            if c.cond(7) { pc = 0x53e604; continue; }
            pc = 0x53e499; continue;
            }
            0x53e499 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,0x18u32,16); }
            if c.cond(3) { pc = 0x53e604; continue; }
            pc = 0x53e4a7; continue;
            }
            0x53e4a7 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,0x6dd72cu32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[3].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x7c); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.r[3].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53e4e9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x53e517; continue; }
            pc = 0x53e4f0; continue;
            }
            0x53e4f0 => {
            if c.cond(13) { pc = 0x53e505; continue; }
            pc = 0x53e4f2; continue;
            }
            0x53e4f2 => {
            { let a = c.get(1,16); let r = c.neg(a,16); c.set(1,16,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x548); wm(m,a,16,c.get(1,16)); }
            pc = 0x53e517; continue;
            }
            0x53e505 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x10),16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54a); wm(m,a,16,c.get(2,16)); }
            pc = 0x53e517; continue;
            }
            0x53e517 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(1,16); c.sub(a,0x18u32,16); }
            if c.cond(3) { pc = 0x53e604; continue; }
            pc = 0x53e525; continue;
            }
            0x53e525 => {
            c.set(7,16,rm(m,c.r[3].wrapping_add(0x4),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,16); c.sub(a,c.get(2,16),16); }
            if c.cond(4) { pc = 0x53e546; continue; }
            pc = 0x53e535; continue;
            }
            0x53e535 => {
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(13) { pc = 0x53e56a; continue; }
            pc = 0x53e53a; continue;
            }
            0x53e53a => {
            c.set(7,16,rm(m,c.r[3].wrapping_add(c.r[0]).wrapping_add(0x5),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(7,16); c.sub(a,c.get(2,16),16); }
            if c.cond(5) { pc = 0x53e535; continue; }
            pc = 0x53e546; continue;
            }
            0x53e546 => {
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x53e57d; continue; }
            pc = 0x53e550; continue;
            }
            0x53e550 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(12) { pc = 0x53e597; continue; }
            pc = 0x53e555; continue;
            }
            0x53e555 => {
            c.set(1,8,rm(m,c.r[3].wrapping_add(c.r[0]).wrapping_add(0x4),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(c.r[0]).wrapping_add(0x5); wm(m,a,8,c.get(1,8)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x53e555; continue; }
            pc = 0x53e568; continue;
            }
            0x53e568 => {
            pc = 0x53e597; continue;
            }
            0x53e56a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x53e56c; continue;
            }
            0x53e56c => {
            { let a = c.r[3].wrapping_add(c.r[0]).wrapping_add(0x4); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x53e56c; continue; }
            pc = 0x53e576; continue;
            }
            0x53e576 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53e57d => {
            if c.cond(13) { pc = 0x53e597; continue; }
            pc = 0x53e57f; continue;
            }
            0x53e57f => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x53e597; continue; }
            pc = 0x53e584; continue;
            }
            0x53e584 => {
            c.set(1,8,rm(m,c.r[3].wrapping_add(c.r[0]).wrapping_add(0x4),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(c.r[0]).wrapping_add(0x3); wm(m,a,8,c.get(1,8)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xa),16));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x53e584; continue; }
            pc = 0x53e597; continue;
            }
            0x53e597 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x8),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xa),16));
            c.set(7,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[0].wrapping_add(c.r[3]).wrapping_add(0x4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6u32);
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            c.stos(m,32,true);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x53e5b4; continue;
            }
            0x53e5b4 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(c.r[1]).wrapping_add(0x4),8));
            { let a = c.get(0,8); c.sub(a,0x18u32,8); }
            if c.cond(3) { pc = 0x53e5c5; continue; }
            pc = 0x53e5bc; continue;
            }
            0x53e5bc => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0xc); wm(m,a,8,c.get(1,8)); }
            pc = 0x53e5c5; continue;
            }
            0x53e5c5 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x53e5b4; continue; }
            pc = 0x53e5cb; continue;
            }
            0x53e5cb => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x53e5cd; continue;
            }
            0x53e5cd => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(c.r[1]).wrapping_add(0x4),8));
            { let a = c.get(0,8); c.sub(a,0x18u32,8); }
            if c.cond(3) { pc = 0x53e5e4; continue; }
            pc = 0x53e5d5; continue;
            }
            0x53e5d5 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0xc),8));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x53e5fe; continue; }
            pc = 0x53e5e4; continue;
            }
            0x53e5e4 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,8,0xffu32);
            pc = 0x53e5e8; continue;
            }
            0x53e5e8 => {
            { let a = rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0xc),8); c.sub(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x53e5f6; continue; }
            pc = 0x53e5ee; continue;
            }
            0x53e5ee => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x53e5e8; continue; }
            pc = 0x53e5f4; continue;
            }
            0x53e5f4 => {
            pc = 0x53e5fe; continue;
            }
            0x53e5f6 => {
            { let a = c.r[3].wrapping_add(c.r[1]).wrapping_add(0x4); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0xc); wm(m,a,8,c.get(1,8)); }
            pc = 0x53e5fe; continue;
            }
            0x53e5fe => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x18u32,32); }
            if c.cond(12) { pc = 0x53e5cd; continue; }
            pc = 0x53e604; continue;
            }
            0x53e604 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053e610(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53e610;
    loop {
        match pc {
            0x53e610 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,16,rm(m,0x6dd72au32,16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x53e7b7; continue; }
            pc = 0x53e629; continue;
            }
            0x53e629 => {
            c.set(5,32,rm(m,0x6dd72cu32,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(0,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x53e69e; continue; }
            pc = 0x53e643; continue;
            }
            0x53e643 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x53e7b7; continue; }
            pc = 0x53e653; continue;
            }
            0x53e653 => {
            c.set(1,32,0x6dd728u32);
            c.push(m, 0x53e65d);
            { let args = []; c.r[0] = host.call(m, 0x4d4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(7,32,rm(m,0x6dd72cu32,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(3,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[3].wrapping_mul(2)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(7,16,rm(m,c.r[2],16));
            { let a = c.r[1]; wm(m,a,16,c.get(7,16)); }
            { let a = c.r[2]; wm(m,a,16,c.get(0,16)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[1].wrapping_add(0x2); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[1].wrapping_add(0x3); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53e69e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x53e704; continue; }
            pc = 0x53e6ab; continue;
            }
            0x53e6ab => {
            c.set(2,16,rm(m,c.r[7],16));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x53e6c0; continue; }
            pc = 0x53e6b9; continue;
            }
            0x53e6b9 => {
            c.set(2,32,0xffffu32);
            pc = 0x53e6e1; continue;
            }
            0x53e6c0 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3),8));
            { let a = c.get(1,8); let r = c.and(a,0x4u32,8); c.set(1,8,r); }
            { let a = c.get(1,8); c.sub(a,0x4u32,8); }
            if c.cond(5) { pc = 0x53e6e1; continue; }
            pc = 0x53e6cb; continue;
            }
            0x53e6cb => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,8,rm(m,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3),8));
            c.set(0,32,c.r[5].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3));
            { let a = c.get(1,8); let r = c.or(a,0x4u32,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x53e6e1; continue;
            }
            0x53e6e1 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x53e6f1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x509030, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,0x6dd728u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x53e6ff);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d4e80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53e704 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x2),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x53e7b7; continue; }
            pc = 0x53e711; continue;
            }
            0x53e711 => {
            c.set(3,8,rm(m,c.r[7].wrapping_add(0x3),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(3,8); let r = c.and(a,0x4u32,8); c.set(3,8,r); }
            { let a = c.r[7].wrapping_add(0x2); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[7].wrapping_add(0x3); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,0x6dd894u32,32));
            c.set(3,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x53e79c; continue; }
            pc = 0x53e731; continue;
            }
            0x53e731 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(2)));
            { let a = c.get(3,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(1,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x132),16));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x53e778; continue; }
            pc = 0x53e74f; continue;
            }
            0x53e74f => {
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x136),8));
            { let a = rm(m,c.r[3].wrapping_add(0x3c),8); let r = c.or(a,0x20u32,8); { let a = c.r[3].wrapping_add(0x3c); wm(m,a,8,r); } }
            { let a = c.get(2,8); let r = c.and(a,0xfcu32,8); c.set(2,8,r); }
            { let a = rm(m,c.r[3].wrapping_add(0x134),16); c.sub(a,0xcu32,16); }
            { let a = c.r[3].wrapping_add(0x136); wm(m,a,8,c.get(2,8)); }
            if c.cond(2) { pc = 0x53e778; continue; }
            pc = 0x53e76c; continue;
            }
            0x53e76c => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x10),16));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x53e778);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5171e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x53e778; continue;
            }
            0x53e778 => {
            c.set(0,16,rm(m,c.r[3],8));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x53e79c; continue; }
            pc = 0x53e785; continue;
            }
            0x53e785 => {
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x2),8));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53e798);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x53e7b7; continue; }
            pc = 0x53e79c; continue;
            }
            0x53e79c => {
            { let a = c.get(7,32); let r = c.add(a,0x7cu32,32); c.set(7,32,r); }
            c.set(0,32,0x18u32);
            pc = 0x53e7a4; continue;
            }
            0x53e7a4 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xffffffa0),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x53e7b1; continue; }
            pc = 0x53e7ab; continue;
            }
            0x53e7ab => {
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            pc = 0x53e7b1; continue;
            }
            0x53e7b1 => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x53e7a4; continue; }
            pc = 0x53e7b7; continue;
            }
            0x53e7b7 => {
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

pub fn f_0053e7c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53e7c0;
    loop {
        match pc {
            0x53e7c0 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(2,32));
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x53e881; continue; }
            pc = 0x53e7db; continue;
            }
            0x53e7db => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x53e820; continue; }
            pc = 0x53e7f6; continue;
            }
            0x53e7f6 => {
            c.set(6,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x10),16));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(7) { pc = 0x53e881; continue; }
            pc = 0x53e813; continue;
            }
            0x53e813 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(7,32),32); c.set(6,32,r); }
            pc = 0x53e820; continue;
            }
            0x53e820 => {
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x132),16));
            c.set(7,32,c.r[6].wrapping_add(0x132));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6dd72au32,16),16); }
            if c.cond(3) { pc = 0x53e881; continue; }
            pc = 0x53e836; continue;
            }
            0x53e836 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,0x1u32,32); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc),32));
            c.set(0,8,u32::from(c.cond(4)));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.set(1,32,0x6dd728u32);
            c.push(m, 0x53e855);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4d4f60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,16,rm(m,c.r[7],16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd72au32,16),16); }
            if c.cond(2) { pc = 0x53e874; continue; }
            pc = 0x53e861; continue;
            }
            0x53e861 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x136),8));
            { let a = c.get(1,8); let r = c.and(a,0xfcu32,8); c.set(1,8,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x3d),8); let r = c.or(a,0x8u32,8); { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,r); } }
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,c.get(1,8)); }
            pc = 0x53e874; continue;
            }
            0x53e874 => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x48fd90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53e881 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053e890(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53e890;
    loop {
        match pc {
            0x53e890 => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,c.get(2,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53e9b1; continue; }
            pc = 0x53e8ae; continue;
            }
            0x53e8ae => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(7,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x53e9b1; continue; }
            pc = 0x53e8bf; continue;
            }
            0x53e8bf => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); c.sub(a,0x1u32,32); }
            if c.cond(2) { pc = 0x53e9b1; continue; }
            pc = 0x53e8c9; continue;
            }
            0x53e8c9 => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x53e92e; continue; }
            pc = 0x53e8e0; continue;
            }
            0x53e8e0 => {
            c.set(7,32,rm(m,0x6dd7a8u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0],8));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10),16));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53e9b1; continue; }
            pc = 0x53e914; continue;
            }
            0x53e914 => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x53e9b1; continue; }
            pc = 0x53e92e; continue;
            }
            0x53e92e => {
            c.set(7,32,c.r[0].wrapping_add(0x132));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x132),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd72au32,16),16); }
            if c.cond(3) { pc = 0x53e95b; continue; }
            pc = 0x53e944; continue;
            }
            0x53e944 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd728u32);
            c.push(m, 0x53e956);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d4dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.r[7]; wm(m,a,16,0xffffu32); }
            pc = 0x53e95b; continue;
            }
            0x53e95b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(5,32,c.get(7,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x53e9ae; continue; }
            pc = 0x53e964; continue;
            }
            0x53e964 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(1,32,0x6dd728u32);
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x53e972);
            { let args = []; c.r[0] = host.call(m, 0x4d4c90, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            { let a = c.r[5]; wm(m,a,16,c.get(0,16)); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,16); c.sub(a,rm(m,c.r[7],16),16); }
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,0x6dd72cu32,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            if c.cond(5) { pc = 0x53e999; continue; }
            pc = 0x53e995; continue;
            }
            0x53e995 => {
            { let a = c.r[1].wrapping_add(0x3); wm(m,a,8,0x4u32); }
            pc = 0x53e999; continue;
            }
            0x53e999 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x8),8));
            c.set(5,32,c.get(1,32));
            { let a = c.r[1].wrapping_add(0x2); wm(m,a,8,c.get(0,8)); }
            c.set(2,16,rm(m,c.r[7],16));
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x53e964; continue; }
            pc = 0x53e9ae; continue;
            }
            0x53e9ae => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); let r = c.dec(a,32); { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,r); } }
            pc = 0x53e9b1; continue;
            }
            0x53e9b1 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053e9c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53e9c0;
    loop {
        match pc {
            0x53e9c0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,0x6dd72au32,16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x53ea40; continue; }
            pc = 0x53e9d0; continue;
            }
            0x53e9d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(2,32,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x53ea28; continue; }
            pc = 0x53e9df; continue;
            }
            0x53e9df => {
            c.set(7,32,rm(m,0x6dd72cu32,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(3,8,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x2),8));
            c.set(7,32,rm(m,0x6de4b0u32,32));
            c.set(2,32,c.get(3,32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x53ea24; continue; }
            pc = 0x53ea02; continue;
            }
            0x53ea02 => {
            c.set(7,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(7,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[7].wrapping_mul(8)).wrapping_add(0x76c),32));
            { let a = c.get(2,32); let r = c.and(a,0x40u32,32); c.set(2,32,r); }
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x53ea28; continue; }
            pc = 0x53ea1e; continue;
            }
            0x53ea1e => {
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.or(a,0x8u32,32); c.set(6,32,r); }
            pc = 0x53ea25; continue;
            }
            0x53ea24 => {
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            pc = 0x53ea25; continue;
            }
            0x53ea25 => {
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            pc = 0x53ea28; continue;
            }
            0x53ea28 => {
            c.set(1,8,rm(m,c.r[1].wrapping_add(0x8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)));
            c.set(2,32,rm(m,0x6dd72cu32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[2].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x3); wm(m,a,8,c.get(1,8)); }
            pc = 0x53ea40; continue;
            }
            0x53ea40 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053eac0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53eac0;
    loop {
        match pc {
            0x53eac0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6635c3u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x53eae2);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xb),8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x53ec06; continue; }
            pc = 0x53eb03; continue;
            }
            0x53eb03 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53eb13);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[6],16));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4bc),32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[6],16));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x4bc); wm(m,a,32,c.get(2,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xf),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x53eb90; continue; }
            pc = 0x53eb38; continue;
            }
            0x53eb38 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xc),8));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x19),8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53eb4e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x531b70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(6) { pc = 0x53eb63; continue; }
            pc = 0x53eb5e; continue;
            }
            0x53eb5e => {
            c.set(0,32,0x8u32);
            pc = 0x53eb63; continue;
            }
            0x53eb63 => {
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53eb79);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x17u32); }
            c.push(m,0x10u32);
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x53eb8b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x53ebcd; continue;
            }
            0x53eb90 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xe),8));
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.r[4].wrapping_add(0xe); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xb),8));
            c.set(0,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x9); wm(m,a,8,c.get(2,8)); }
            c.set(2,16,rm(m,c.r[6],16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x2bu32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x53ebcd);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x53ebcd; continue;
            }
            0x53ebcd => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x2),16));
            c.set(1,16,rm(m,0x6dd736u32,16));
            { let a = c.get(0,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x53ec06; continue; }
            pc = 0x53ebdd; continue;
            }
            0x53ebdd => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(2,16,c.get(1,16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(7) { pc = 0x53ebf8; continue; }
            pc = 0x53ebeb; continue;
            }
            0x53ebeb => {
            c.set(1,32,rm(m,0x6dd730u32,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            pc = 0x53ebfa; continue;
            }
            0x53ebf8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x53ebfa; continue;
            }
            0x53ebfa => {
            { let a = rm(m,c.r[0].wrapping_add(0x4),8); c.sub(a,0x21u32,8); }
            if c.cond(5) { pc = 0x53ec06; continue; }
            pc = 0x53ec00; continue;
            }
            0x53ec00 => {
            { let a = c.r[0].wrapping_add(0xe); wm(m,a,16,0x0u32); }
            pc = 0x53ec06; continue;
            }
            0x53ec06 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x53ec17);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053ec30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53ec30;
    loop {
        match pc {
            0x53ec30 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6dd748u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53eca4; continue; }
            pc = 0x53ec46; continue;
            }
            0x53ec46 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x53ec51);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x6),16); c.sub(a,0x1bu32,16); }
            if c.cond(5) { pc = 0x53ec6e; continue; }
            pc = 0x53ec58; continue;
            }
            0x53ec58 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x12),16));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x53ec6e; continue; }
            pc = 0x53ec65; continue;
            }
            0x53ec65 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            pc = 0x53ec6e; continue;
            }
            0x53ec6e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6dd736u32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(7) { pc = 0x53eca4; continue; }
            pc = 0x53ec7e; continue;
            }
            0x53ec7e => {
            c.set(2,32,rm(m,0x6dd730u32,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x53eca4; continue; }
            pc = 0x53ec8d; continue;
            }
            0x53ec8d => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x4),8));
            { let a = c.get(1,8); c.sub(a,0x3cu32,8); }
            if c.cond(2) { pc = 0x53eca4; continue; }
            pc = 0x53ec95; continue;
            }
            0x53ec95 => {
            { let a = c.get(1,8); c.sub(a,0x40u32,8); }
            if c.cond(7) { pc = 0x53eca4; continue; }
            pc = 0x53ec9a; continue;
            }
            0x53ec9a => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            pc = 0x53eca4; continue;
            }
            0x53eca4 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053ecb0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53ecb0;
    loop {
        match pc {
            0x53ecb0 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x53ecd0; continue; }
            pc = 0x53ecbc; continue;
            }
            0x53ecbc => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x53eccd);
            { let args = []; c.r[0] = host.call(m, 0x534600, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x53ecd0; continue;
            }
            0x53ecd0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,0x6de584u32);
            c.push(m, 0x53ece6);
            { let args = []; c.r[0] = host.call(m, 0x5340d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53ed63; continue; }
            pc = 0x53ecf9; continue;
            }
            0x53ecf9 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53ed04);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xa),16));
            c.set(2,16,rm(m,0x6dd736u32,16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x53ed63; continue; }
            pc = 0x53ed17; continue;
            }
            0x53ed17 => {
            c.set(7,32,rm(m,0x6dd730u32,32));
            pc = 0x53ed1d; continue;
            }
            0x53ed1d => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x4),8); c.sub(a,0x1eu32,8); }
            if c.cond(5) { pc = 0x53ed48; continue; }
            pc = 0x53ed2a; continue;
            }
            0x53ed2a => {
            c.set(5,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x53ed48; continue; }
            pc = 0x53ed32; continue;
            }
            0x53ed32 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xc),16));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x53ed48; continue; }
            pc = 0x53ed3d; continue;
            }
            0x53ed3d => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc),32),32); }
            if c.cond(4) { pc = 0x53ed54; continue; }
            pc = 0x53ed48; continue;
            }
            0x53ed48 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x53ed1d; continue; }
            pc = 0x53ed52; continue;
            }
            0x53ed52 => {
            pc = 0x53ed63; continue;
            }
            0x53ed54 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x53ed63);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6a50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x53ed63; continue;
            }
            0x53ed63 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53eddf; continue; }
            pc = 0x53ed73; continue;
            }
            0x53ed73 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53ed7e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0xa),16));
            c.set(2,16,rm(m,0x6dd736u32,16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x53eddf; continue; }
            pc = 0x53ed91; continue;
            }
            0x53ed91 => {
            c.set(7,32,rm(m,0x6dd730u32,32));
            pc = 0x53ed97; continue;
            }
            0x53ed97 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x4),8); c.sub(a,0x1fu32,8); }
            if c.cond(5) { pc = 0x53edc2; continue; }
            pc = 0x53eda4; continue;
            }
            0x53eda4 => {
            c.set(5,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x53edc2; continue; }
            pc = 0x53edac; continue;
            }
            0x53edac => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xc),16));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x53edc2; continue; }
            pc = 0x53edb7; continue;
            }
            0x53edb7 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0xe),16));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[6].wrapping_add(0xc),32),32); }
            if c.cond(4) { pc = 0x53edd0; continue; }
            pc = 0x53edc2; continue;
            }
            0x53edc2 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x53ed97; continue; }
            pc = 0x53edcc; continue;
            }
            0x53edcc => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53edd0 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x53eddf);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6a50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x53eddf; continue;
            }
            0x53eddf => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053eec0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53eec0;
    loop {
        match pc {
            0x53eec0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6635d5u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x30u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,0x6dd736u32,16));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(5,32,0xffffu32);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x53ef05; continue; }
            pc = 0x53eef1; continue;
            }
            0x53eef1 => {
            c.set(0,32,rm(m,0x6dd730u32,32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x4),8); c.sub(a,0x79u32,8); }
            if c.cond(5) { pc = 0x53ef05; continue; }
            pc = 0x53ef01; continue;
            }
            0x53ef01 => {
            { let a = c.r[1].wrapping_add(0xa); wm(m,a,16,c.get(5,16)); }
            pc = 0x53ef05; continue;
            }
            0x53ef05 => {
            c.set(1,32,rm(m,0x6dd748u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(1,32); let r = c.and(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x53f6f1; continue; }
            pc = 0x53ef18; continue;
            }
            0x53ef18 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x53ef23);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = rm(m,c.r[7].wrapping_add(0x6),16); c.sub(a,0x1u32,16); }
            if c.cond(5) { pc = 0x53f6f1; continue; }
            pc = 0x53ef30; continue;
            }
            0x53ef30 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x12),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53f6f1; continue; }
            pc = 0x53ef3b; continue;
            }
            0x53ef3b => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x11),8));
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(2,16); c.sub(a,c.get(1,16),16); }
            if c.cond(2) { pc = 0x53f6f1; continue; }
            pc = 0x53ef50; continue;
            }
            0x53ef50 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x10),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(0,8));
            { let a = c.get(2,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x53f6f1; continue; }
            pc = 0x53ef61; continue;
            }
            0x53ef61 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53ef72);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x53ef93; continue; }
            pc = 0x53ef7f; continue;
            }
            0x53ef7f => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x53ef8e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4f9280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x53f6ec; continue;
            }
            0x53ef93 => {
            c.set(1,32,c.r[4].wrapping_add(0x30));
            c.push(m, 0x53ef9c);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x14),8));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,0x800u32);
            { let a = c.r[4].wrapping_add(0x4c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x83u32); }
            { let a = c.r[4].wrapping_add(0x39); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x53efbc);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(5,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1a),8));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(6,32)); }
            if c.cond(4) { pc = 0x53f045; continue; }
            pc = 0x53efd6; continue;
            }
            0x53efd6 => {
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x1d),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x19),8));
            c.push(m,c.get(2,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[3]).wrapping_add(0x39c),8));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x53eff4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x1c),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53f005);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6a0628),32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x53f01f);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x1a),8));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0c54),32));
            { let a = c.get(2,32); let r = c.and(a,0x1u32,32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,32,0x6b1a00u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0],32));
            c.push(m,c.get(1,32));
            pc = 0x53f0b1; continue;
            }
            0x53f045 => {
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x19),8));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x1d),8));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[3]).wrapping_add(0x39c),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x53f063);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x1c),8));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m, 0x53f074);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x53f08d);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6a0c18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(2,32));
            c.set(2,32,0x6b1a00u32);
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1a),8));
            { let a = c.get(1,32); let r = c.and(a,0x1u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,c.r[2],32));
            c.push(m,c.get(0,32));
            pc = 0x53f0b1; continue;
            }
            0x53f0b1 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(3,32));
            c.push(m, 0x53f0bb);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[3],8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x53f0d5; continue; }
            pc = 0x53f0c4; continue;
            }
            0x53f0c4 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53f0c4; continue; }
            pc = 0x53f0d5; continue;
            }
            0x53f0d5 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x17),8));
            c.set(2,32,rm(m,c.r[1].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x53f0f2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512ae0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x16),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x53f103);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512a60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6b1bc0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x15),8));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a0628),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x53f11e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x53f139; continue; }
            pc = 0x53f128; continue;
            }
            0x53f128 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53f128; continue; }
            pc = 0x53f139; continue;
            }
            0x53f139 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(2,32));
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x53f14e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4f9280, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,0x100u32);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xc),8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,rm(m,c.r[7].wrapping_add(0x8),32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x8u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x3a98u32,32); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x53f2cf; continue; }
            pc = 0x53f16f; continue;
            }
            0x53f16f => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x14),8));
            c.push(m,0x1u32);
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x53f186);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x507d60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(6,32,c.get(0,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x53f193);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x509a50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); c.sub(a,0x4e20u32,32); }
            if c.cond(14) { pc = 0x53f1ce; continue; }
            pc = 0x53f1a3; continue;
            }
            0x53f1a3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x1u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x53f1bb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52a8d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,0x6b1bccu32,32));
            pc = 0x53f1f7; continue;
            }
            0x53f1ce => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,0x0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.push(m, 0x53f1e6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52a8d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.set(0,32,rm(m,0x6b1bc8u32,32));
            pc = 0x53f1f7; continue;
            }
            0x53f1f7 => {
            c.set(2,32,c.r[6].wrapping_add(0x160));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x53f206);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x53f221; continue; }
            pc = 0x53f210; continue;
            }
            0x53f210 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53f210; continue; }
            pc = 0x53f221; continue;
            }
            0x53f221 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x53f228);
            { let args = []; c.r[0] = host.call(m, 0x5181d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x53f22f);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x10),8));
            { let a = c.r[6]; wm(m,a,8,c.get(1,8)); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0xe),16));
            { let a = rm(m,c.r[6].wrapping_add(0x3d),8); let r = c.or(a,0x4u32,8); { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,r); } }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x20),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[3].wrapping_add(0xe); wm(m,a,16,c.get(0,16)); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x14),8));
            { let a = c.r[6].wrapping_add(0x13a); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(1,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0x1fu32,32); c.set(0,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xe),8));
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,16,0x3u32); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x673660),8));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x53f28a);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,8,rm(m,0x6de4b4u32,8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(0,32,c.r[1].wrapping_add(c.r[2]).wrapping_add(0x2ee));
            { let a = c.get(0,16); c.sub(a,0x700u32,16); }
            { let a = c.r[6].wrapping_add(0x3e); wm(m,a,16,c.get(0,16)); }
            if c.cond(14) { pc = 0x53f2b1; continue; }
            pc = 0x53f2ab; continue;
            }
            0x53f2ab => {
            { let a = c.r[6].wrapping_add(0x3e); wm(m,a,16,0x700u32); }
            pc = 0x53f2b1; continue;
            }
            0x53f2b1 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,0x4e20u32,32); }
            if c.cond(14) { pc = 0x53f2c5; continue; }
            pc = 0x53f2bb; continue;
            }
            0x53f2bb => {
            { let a = c.get(0,32); let r = c.add(a,0xffffb1e0u32,32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x53f304; continue;
            }
            0x53f2c5 => {
            { let a = c.get(0,32); let r = c.add(a,0xffffc568u32,32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x53f304; continue;
            }
            0x53f2cf => {
            c.set(2,32,rm(m,0x6b1bc4u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x53f2e9);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x53f304; continue; }
            pc = 0x53f2f3; continue;
            }
            0x53f2f3 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53f2f3; continue; }
            pc = 0x53f304; continue;
            }
            0x53f304 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x14),8));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x10),8));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x53f31c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x53f32c; continue; }
            pc = 0x53f324; continue;
            }
            0x53f324 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x53f666; continue; }
            pc = 0x53f32c; continue;
            }
            0x53f32c => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x15),8));
            c.set(2,32,rm(m,0x6de4a4u32,32));
            { let a = c.get(0,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0x3u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.set(3,32,0x3u32);
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x14),8));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); c.sub(a,0x2u32,32); }
            if c.cond(5) { pc = 0x53f361; continue; }
            pc = 0x53f359; continue;
            }
            0x53f359 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x673b38),8));
            pc = 0x53f368; continue;
            }
            0x53f361 => {
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x673b39),8));
            pc = 0x53f368; continue;
            }
            0x53f368 => {
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x673b38),8));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            c.set(1,32,c.r[0].wrapping_mul(4));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x53f3b4; continue; }
            pc = 0x53f396; continue;
            }
            0x53f396 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[1].wrapping_add(0x673a18),4));
            { let v = c.ld(m,0x66a91cu32,4); c.fop('*',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x10),4); c.fop('q',v); }
            pc = 0x53f3d0; continue;
            }
            0x53f3b4 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.fstp32(m,c.r[4].wrapping_add(0x10));
            c.fld(c.ld(m,c.r[1].wrapping_add(0x673a18),4));
            { let v = c.ld(m,0x66a918u32,4); c.fop('*',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x10),4); c.fop('q',v); }
            pc = 0x53f3d0; continue;
            }
            0x53f3d0 => {
            c.push(m, 0x53f3d5);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x53f421; continue; }
            pc = 0x53f3dd; continue;
            }
            0x53f3dd => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,8); let r = c.neg(a,8); c.set(3,8,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x6c0bb0),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,8); let r = c.sbb(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m,c.get(2,32));
            { let a = c.get(3,8); let r = c.and(a,0xf6u32,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.add(a,0x4cu32,8); c.set(3,8,r); }
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x6c36d4u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x53f406);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x53f421; continue; }
            pc = 0x53f410; continue;
            }
            0x53f410 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53f410; continue; }
            pc = 0x53f421; continue;
            }
            0x53f421 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,8); let r = c.neg(a,8); c.set(1,8,r); }
            { let a = c.get(1,32); let r = c.sbb(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xfffff8f8u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x7d0u32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(1,32,r); }
            c.set(3,32,c.get(1,32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x53f4b9; continue; }
            pc = 0x53f44c; continue;
            }
            0x53f44c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(14) { pc = 0x53f494; continue; }
            pc = 0x53f453; continue;
            }
            0x53f453 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53f476; continue; }
            pc = 0x53f457; continue;
            }
            0x53f457 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            pc = 0x53f490; continue;
            }
            0x53f476 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(1,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.set(3,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            pc = 0x53f490; continue;
            }
            0x53f490 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x53f494; continue;
            }
            0x53f494 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x53f4b1; continue; }
            pc = 0x53f4a0; continue;
            }
            0x53f4a0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x673a18),32));
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xb4); wm(m,a,32,c.get(2,32)); }
            pc = 0x53f4b1; continue;
            }
            0x53f4b1 => {
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(1,32)); }
            pc = 0x53f4e4; continue;
            }
            0x53f4b9 => {
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[0].wrapping_add(0x673a18),4); c.fop('*',v); }
            c.push(m, 0x53f4d4);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.push(m, 0x53f4e4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5007a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x53f4e4; continue;
            }
            0x53f4e4 => {
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[2].wrapping_add(0x673a18),4); c.fop('*',v); }
            c.push(m, 0x53f4ff);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.fild32(m,c.r[4].wrapping_add(0x18));
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x672c14),8));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x53f53c; continue; }
            pc = 0x53f526; continue;
            }
            0x53f526 => {
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[0].wrapping_add(0x673a18),4));
            { let v = c.ld(m,0x66a91cu32,4); c.fop('*',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fop('q',v); }
            pc = 0x53f550; continue;
            }
            0x53f53c => {
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[0].wrapping_add(0x673a18),4));
            { let v = c.ld(m,0x66a918u32,4); c.fop('*',v); }
            { let v = c.ld(m,c.r[4].wrapping_add(0x2c),4); c.fop('q',v); }
            pc = 0x53f550; continue;
            }
            0x53f550 => {
            c.push(m, 0x53f555);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x53f5a1; continue; }
            pc = 0x53f55d; continue;
            }
            0x53f55d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(3,8); let r = c.neg(a,8); c.set(3,8,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x6c0bb0),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,8); let r = c.sbb(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m,c.get(2,32));
            { let a = c.get(3,8); let r = c.and(a,0xf6u32,8); c.set(3,8,r); }
            { let a = c.get(3,8); let r = c.add(a,0x4cu32,8); c.set(3,8,r); }
            c.set(0,32,sx(c.get(3,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,0x6c36e0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x53f586);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x53f5a1; continue; }
            pc = 0x53f590; continue;
            }
            0x53f590 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53f590; continue; }
            pc = 0x53f5a1; continue;
            }
            0x53f5a1 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,8,rm(m,c.r[2].wrapping_add(0x672c14),8));
            { let a = c.get(3,8); let r = c.neg(a,8); c.set(3,8,r); }
            { let a = c.get(3,32); let r = c.sbb(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xfffff8f8u32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x7d0u32,32); c.set(3,32,r); }
            { let a = c.get(3,32); let r = c.imul(a,rm(m,c.r[4].wrapping_add(0x10),32),32); c.set(3,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x53f633; continue; }
            pc = 0x53f5c8; continue;
            }
            0x53f5c8 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x53f5cf);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(3,32); c.sub(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x53f620; continue; }
            pc = 0x53f5d9; continue;
            }
            0x53f5d9 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53f602; continue; }
            pc = 0x53f5e3; continue;
            }
            0x53f5e3 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            pc = 0x53f61c; continue;
            }
            0x53f602 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(3,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(3,32,c.r[3].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = c.get(3,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(3,32,r); }
            pc = 0x53f61c; continue;
            }
            0x53f61c => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            pc = 0x53f620; continue;
            }
            0x53f620 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x53f631);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x53f63f; continue;
            }
            0x53f633 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4),32); let r = c.add(a,c.get(3,32),32); { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,r); } }
            pc = 0x53f63f; continue;
            }
            0x53f63f => {
            c.fild32(m,c.r[4].wrapping_add(0x10));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.fstp32(m,c.r[4].wrapping_add(0x2c));
            c.fld(c.ld(m,c.r[4].wrapping_add(0x2c),4));
            { let v = c.ld(m,c.r[1].wrapping_add(0x673a18),4); c.fop('*',v); }
            c.push(m, 0x53f65a);
            c.crt_call(m, 0x639c28);
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x53f666; continue;
            }
            0x53f666 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x53f684; continue; }
            pc = 0x53f66d; continue;
            }
            0x53f66d => {
            c.set(1,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4b8),32));
            { let a = c.r[3]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[3].wrapping_add(0x4b8); wm(m,a,32,c.get(0,32)); }
            pc = 0x53f684; continue;
            }
            0x53f684 => {
            c.set(2,32,rm(m,0x6b1bd0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x14),8));
            c.set(1,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x53f69e);
            { let args = []; c.r[0] = host.call(m, 0x64d410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x53f6b9; continue; }
            pc = 0x53f6a8; continue;
            }
            0x53f6a8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(1,32)); }
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x53f6a8; continue; }
            pc = 0x53f6b9; continue;
            }
            0x53f6b9 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[5]; wm(m,a,8,0x0u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x10),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x53f6db);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x53f6ec);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x53f6ec; continue;
            }
            0x53f6ec => {
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,8,0xffu32); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x53f6f1; continue;
            }
            0x53f6f1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); c.fpu_exit(); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053fb00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53fb00;
    loop {
        match pc {
            0x53fb00 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,16,rm(m,c.r[6].wrapping_add(0x4),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x53fb3c; continue; }
            pc = 0x53fb11; continue;
            }
            0x53fb11 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(c.r[2]).wrapping_add(0x134),16));
            { let a = c.get(0,16); c.sub(a,0xfu32,16); }
            if c.cond(4) { pc = 0x53fb3c; continue; }
            pc = 0x53fb32; continue;
            }
            0x53fb32 => {
            { let a = c.get(0,16); c.sub(a,0x10u32,16); }
            if c.cond(5) { pc = 0x53fc64; continue; }
            pc = 0x53fb3c; continue;
            }
            0x53fb3c => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0xb),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x53fb4d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5063c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd894u32,16),16); }
            if c.cond(3) { pc = 0x53fbb2; continue; }
            pc = 0x53fb56; continue;
            }
            0x53fb56 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(7,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(7,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x42),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd892u32,16),16); }
            if c.cond(3) { pc = 0x53fb92; continue; }
            pc = 0x53fb76; continue;
            }
            0x53fb76 => {
            c.set(1,32,rm(m,0x6dd7a0u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x6),8));
            { let a = c.r[0].wrapping_add(0x2); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x7),8));
            { let a = c.r[0].wrapping_add(0x3); wm(m,a,8,c.get(2,8)); }
            pc = 0x53fb92; continue;
            }
            0x53fb92 => {
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x53fba9; continue; }
            pc = 0x53fba1; continue;
            }
            0x53fba1 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x53fba9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51a820, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x53fba9; continue;
            }
            0x53fba9 => {
            { let a = c.r[7].wrapping_add(0x136); wm(m,a,8,0x80u32); }
            pc = 0x53fbb4; continue;
            }
            0x53fbb2 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x53fbb4; continue;
            }
            0x53fbb4 => {
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6dd748u32,16),16); }
            if c.cond(3) { pc = 0x53fc64; continue; }
            pc = 0x53fbc5; continue;
            }
            0x53fbc5 => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53fbd5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x6),16); c.sub(a,0x1bu32,16); }
            if c.cond(5) { pc = 0x53fc64; continue; }
            pc = 0x53fbe0; continue;
            }
            0x53fbe0 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(1,8));
            { let a = c.get(2,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x12),16),16); }
            if c.cond(3) { pc = 0x53fc64; continue; }
            pc = 0x53fbee; continue;
            }
            0x53fbee => {
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x53fc5a; continue; }
            pc = 0x53fbf8; continue;
            }
            0x53fbf8 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x6),8));
            { let a = c.r[3].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa),8));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa),8));
            c.set(3,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x7),8));
            { let a = c.get(3,32); let r = c.add(a,c.get(2,32),32); c.set(3,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa),8));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xa),8));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x42),16));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,32,rm(m,c.r[0],32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x53fc5a => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xc),32));
            { let a = c.r[2].wrapping_add(c.r[1].wrapping_mul(4)); wm(m,a,32,0xfffffffeu32); }
            pc = 0x53fc64; continue;
            }
            0x53fc64 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0053fea0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x53fea0;
    loop {
        match pc {
            0x53fea0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6635e7u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.push(m,c.get(5,32));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x4),8));
            c.push(m,c.get(6,32));
            c.set(1,8,c.get(0,8));
            c.push(m,c.get(7,32));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x54000a; continue; }
            pc = 0x53fed3; continue;
            }
            0x53fed3 => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53fee3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(6,16,rm(m,c.r[7].wrapping_add(0x34),16));
            c.set(5,32,c.r[0].wrapping_add(0xfffee900));
            c.push(m, 0x53fefd);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x31),8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x53fff9; continue; }
            pc = 0x53ff17; continue;
            }
            0x53ff17 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x28),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x53fff9; continue; }
            pc = 0x53ff27; continue;
            }
            0x53ff27 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x2c),32));
            { let a = c.get(1,32); let r = c.add(a,0x11700u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x53fff9; continue; }
            pc = 0x53ff38; continue;
            }
            0x53ff38 => {
            c.set(1,16,rm(m,0x6dd892u32,16));
            { let a = c.get(6,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x53ff65; continue; }
            pc = 0x53ff44; continue;
            }
            0x53ff44 => {
            c.set(0,32,rm(m,0x6dd7a0u32,32));
            pc = 0x53ff49; continue;
            }
            0x53ff49 => {
            { let a = c.get(6,32); let r = c.and(a,0xffffu32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(2) { pc = 0x53fff9; continue; }
            pc = 0x53ff5d; continue;
            }
            0x53ff5d => {
            c.set(6,16,rm(m,c.r[6],16));
            { let a = c.get(6,16); c.sub(a,c.get(1,16),16); }
            if c.cond(2) { pc = 0x53ff49; continue; }
            pc = 0x53ff65; continue;
            }
            0x53ff65 => {
            c.push(m,0x1u32);
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x53ff71);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5097c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,0x6dd7a0u32,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x34),16));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.r[1]; wm(m,a,16,c.get(2,16)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x36),8));
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.r[7].wrapping_add(0x34); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[7].wrapping_add(0x36); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x5),8));
            { let a = c.r[1].wrapping_add(0x2); wm(m,a,8,c.get(0,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x1d),8));
            { let a = c.r[1].wrapping_add(0x3); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x6),8));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x7),8));
            { let a = c.r[1].wrapping_add(0x9); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x8),8));
            { let a = c.r[1].wrapping_add(0xa); wm(m,a,8,c.get(0,8)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x9),8));
            { let a = c.r[1].wrapping_add(0xb); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[1].wrapping_add(0xf); wm(m,a,8,0xffu32); }
            { let a = rm(m,c.r[7].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x53fff9; continue; }
            pc = 0x53ffce; continue;
            }
            0x53ffce => {
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x5),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x7au32); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x19),8));
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x4),8));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x53fff9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x53fff9; continue;
            }
            0x53fff9 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x54000a);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x54000a; continue;
            }
            0x54000a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00540240(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x540240;
    loop {
        match pc {
            0x540240 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6635f9u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x10),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x54047b; continue; }
            pc = 0x54026d; continue;
            }
            0x54026d => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m, 0x540276);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xc),16));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            { let a = c.get(0,16); c.and(a,c.get(0,16),16); }
            if c.cond(4) { pc = 0x5402c6; continue; }
            pc = 0x540287; continue;
            }
            0x540287 => {
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x10),8));
            c.set(1,16,rm(m,c.r[3].wrapping_add(0xa),16));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,16,c.get(0,16)); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x8),16));
            { let a = c.r[4].wrapping_add(0xe); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x9); wm(m,a,8,c.get(2,8)); }
            c.set(2,16,rm(m,c.r[3].wrapping_add(0xe),16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(0,32));
            c.push(m,0xfffffffeu32);
            c.set(1,32,0x6dd730u32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,0x38u32); }
            { let a = c.r[4].wrapping_add(0x1a); wm(m,a,16,c.get(2,16)); }
            c.push(m, 0x5402c4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x5402cd; continue;
            }
            0x5402c6 => {
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x10),8));
            { let a = c.r[4].wrapping_add(0x9); wm(m,a,8,c.get(1,8)); }
            pc = 0x5402cd; continue;
            }
            0x5402cd => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,0x37u32); }
            { let a = c.get(0,32); c.sub(a,0xfde8u32,32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x54030e; continue; }
            pc = 0x5402e0; continue;
            }
            0x5402e0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x10),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x2e0),32));
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x32u32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[3].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x54030e; continue;
            }
            0x54030e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x10),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x784),16));
            c.set(2,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x54046a; continue; }
            pc = 0x54033f; continue;
            }
            0x54033f => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            pc = 0x540342; continue;
            }
            0x540342 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x54034d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2c4),16));
            c.push(m,c.get(0,32));
            c.push(m, 0x540363);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,c.get(0,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x46c),32));
            c.set(0,32,c.r[1].wrapping_mul(8));
            c.set(6,32,c.r[1].wrapping_add(0x186a0));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(6,32); c.idiv32(b); }
            c.set(6,32,c.get(0,32));
            c.set(0,32,c.get(1,32));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x15u32);
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(0,32,0xd1b71759u32);
            c.set(2,32,c.r[6].wrapping_add(c.r[2]).wrapping_add(0xa));
            { let a = c.get(2,32); let r = c.imul(a,rm(m,c.r[3].wrapping_add(0x4),32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x1388u32,32); c.set(2,32,r); }
            { let b = c.get(2,32); let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0xdu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0xffdcu32,32); }
            if c.cond(6) { pc = 0x5403bc; continue; }
            pc = 0x5403b7; continue;
            }
            0x5403b7 => {
            c.set(6,32,0xffdcu32);
            pc = 0x5403bc; continue;
            }
            0x5403bc => {
            { let a = rm(m,c.r[5].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x540434; continue; }
            pc = 0x5403c3; continue;
            }
            0x5403c3 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x2c4),16));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x5403e0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,0x6dd73cu32);
            { let a = c.get(0,32); let r = c.add(a,0x1400u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5403f5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x1u32); }
            { let a = c.r[0].wrapping_add(0x12); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0x15); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0x16); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0x17); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[0].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,8,0xffu32); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2c4),8));
            { let a = c.r[0].wrapping_add(0x11); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,8,0x0u32); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x10),8));
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,8,c.get(1,8)); }
            pc = 0x54044a; continue;
            }
            0x540434 => {
            c.set(1,32,rm(m,c.r[5],32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4bc),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[5].wrapping_add(0x4bc); wm(m,a,32,c.get(0,32)); }
            pc = 0x54044a; continue;
            }
            0x54044a => {
            c.set(2,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x2ca),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x540342; continue; }
            pc = 0x540467; continue;
            }
            0x540467 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x54046a; continue;
            }
            0x54046a => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x54047b);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x54047b; continue;
            }
            0x54047b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00541840(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x541840;
    loop {
        match pc {
            0x541840 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66363bu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.set(6,32,c.get(1,32));
            if c.cond(4) { pc = 0x5419a5; continue; }
            pc = 0x541869; continue;
            }
            0x541869 => {
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x5),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.get(1,8); let r = c.and(a,0x7fu32,8); c.set(1,8,r); }
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1c4); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x6),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            c.set(2,32,rm(m,0x6cbb94u32,32));
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.r[1].wrapping_add(c.r[2]).wrapping_add(0x1cc); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x5),8));
            { let a = c.get(0,8); let r = c.and(a,0x80u32,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,0x80u32,8); }
            c.set(0,32,rm(m,0x6cbb94u32,32));
            c.set(1,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.r[2].wrapping_add(c.r[0]).wrapping_add(0x1fc); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6df31cu32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(1,32,rm(m,0x6cbb94u32,32));
            if c.cond(5) { pc = 0x5418f2; continue; }
            pc = 0x5418c5; continue;
            }
            0x5418c5 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1fc),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x5418f2; continue; }
            pc = 0x5418d0; continue;
            }
            0x5418d0 => {
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1f4),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x5418f2; continue; }
            pc = 0x5418db; continue;
            }
            0x5418db => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x10e0),32));
            { let a = c.get(1,32); let r = c.add(a,0x10e0u32,32); c.set(1,32,r); }
            c.push(m,0x1u32);
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x5418ec); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            pc = 0x5418f2; continue;
            }
            0x5418f2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.r[0].wrapping_add(c.r[1]).wrapping_add(0x1f4); wm(m,a,8,0x1u32); }
            c.set(2,32,rm(m,0x6cbb94u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(0,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x164),32));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x164));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x7),8));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x54195c; continue; }
            pc = 0x541924; continue;
            }
            0x541924 => {
            c.set(2,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x54192f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64d230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x164));
            c.push(m, 0x54194b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x54195c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x54195c; continue;
            }
            0x54195c => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x8),8));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[6].wrapping_add(0x8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x54198e; continue; }
            pc = 0x541967; continue;
            }
            0x541967 => {
            c.set(2,32,c.get(3,32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(2,32); c.sub(a,0xcu32,32); }
            if c.cond(13) { pc = 0x54198e; continue; }
            pc = 0x54196f; continue;
            }
            0x54196f => {
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x164));
            c.push(m, 0x541988);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f60a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[7],8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x541967; continue; }
            pc = 0x54198e; continue;
            }
            0x54198e => {
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m, 0x541999);
            { let args = []; c.r[0] = host.call(m, 0x43e3c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m, 0x5419a4);
            { let args = []; c.r[0] = host.call(m, 0x439db0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x5419a5; continue;
            }
            0x5419a5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005419c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5419c0;
    loop {
        match pc {
            0x5419c0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x663655u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(0,32,c.r[1].wrapping_mul(8));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,0x6cbb94u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x541a8b; continue; }
            pc = 0x541a0a; continue;
            }
            0x541a0a => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4440));
            c.push(m, 0x541a22);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64d230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            c.push(m, 0x541a31);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0xc));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            c.push(m, 0x541a3f);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x541a41; continue;
            }
            0x541a41 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[6]).wrapping_add(0x6),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x541a59; continue; }
            pc = 0x541a49; continue;
            }
            0x541a49 => {
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x541a53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f60a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0xeu32,32); }
            if c.cond(12) { pc = 0x541a41; continue; }
            pc = 0x541a59; continue;
            }
            0x541a59 => {
            c.set(0,32,rm(m,0x6cbb94u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x4440));
            c.push(m, 0x541a74);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); c.sub(a,0xeu32,32); }
            { let v = c.pop(m); c.set(6,32,v); }
            if c.cond(13) { pc = 0x541a8b; continue; }
            pc = 0x541a7a; continue;
            }
            0x541a7a => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m, 0x541a8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43e240, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x541a8b; continue;
            }
            0x541a8b => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x541a9c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00541cc0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x541cc0;
    loop {
        match pc {
            0x541cc0 => {
            c.set(0,32,rm(m,0x6df31cu32,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x541cd9; continue; }
            pc = 0x541ccd; continue;
            }
            0x541ccd => {
            c.set(1,32,rm(m,0x6cbb94u32,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x43e4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x541cd9 => {
            c.set(0,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(6,32));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x204); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m, 0x541cf2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43e1f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6cbb94u32,32));
            { let a = c.get(1,8); let r = c.xor(a,c.get(1,8),8); c.set(1,8,r); }
            { let a = c.r[2].wrapping_add(c.r[6]).wrapping_add(0x1fc); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6cbb94u32,32));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x1f4); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,rm(m,0x6cbb94u32,32));
            { let a = rm(m,c.r[0].wrapping_add(0x20c),8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x541d4d; continue; }
            pc = 0x541d1a; continue;
            }
            0x541d1a => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x1d4),32));
            { let a = c.get(1,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x541d3d; continue; }
            pc = 0x541d26; continue;
            }
            0x541d26 => {
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x541d31);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x1),8); let r = c.and(a,0x7fu32,8); { let a = c.r[0].wrapping_add(0x1); wm(m,a,8,r); } }
            c.set(0,32,rm(m,0x6cbb94u32,32));
            pc = 0x541d3d; continue;
            }
            0x541d3d => {
            { let a = c.r[0].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x1d4); wm(m,a,32,0xffffffffu32); }
            c.set(0,32,rm(m,0x6cbb94u32,32));
            pc = 0x541d4d; continue;
            }
            0x541d4d => {
            c.set(1,8,rm(m,0x6de4b0u32,8));
            { let a = c.r[0].wrapping_add(c.r[6]).wrapping_add(0x1c4); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6cbb94u32,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let args = []; c.r[0] = host.call(m, 0x43e3c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00542380(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x542380;
    loop {
        match pc {
            0x542380 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0xa7u32); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6df31cu32,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,0x6cbb94u32,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1c4),8));
            { let a = c.r[4].wrapping_add(0xd); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x4),8));
            { let a = c.r[4].wrapping_add(0xe); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x8),8));
            { let a = c.r[4].wrapping_add(0xf); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1cc),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x5423db);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            c.set(1,32,0x16du32);
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.r[4].wrapping_add(0x11); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x5423ec);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x10),8));
            { let a = c.r[4].wrapping_add(0x12); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            c.set(1,32,0x5u32);
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            c.set(1,32,0x6df2f0u32);
            { let a = 0x6de52cu32; wm(m,a,8,c.get(2,8)); }
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(2,32));
            c.push(m, 0x54241a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005424d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5424d0;
    loop {
        match pc {
            0x5424d0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66366au32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x14u32);
            c.set(6,32,c.get(1,32));
            c.push(m, 0x5424f1);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(7,32)); }
            if c.cond(4) { pc = 0x54250b; continue; }
            pc = 0x542502; continue;
            }
            0x542502 => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x542509);
            { let args = []; c.r[0] = host.call(m, 0x50fca0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            pc = 0x54250b; continue;
            }
            0x54250b => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x8),8));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x4),8));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0xc),16));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x542527);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x50fe00, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x9b4),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[7]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x9b4); wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00542980(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x542980;
    loop {
        match pc {
            0x542980 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(1,32));
            c.set(1,32,rm(m,0x6de4b0u32,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x6ddb74u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x4),16));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x542a28; continue; }
            pc = 0x5429ab; continue;
            }
            0x5429ab => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x784),16));
            { let a = c.get(0,16); c.sub(a,rm(m,0x6de4a8u32,16),16); }
            if c.cond(2) { pc = 0x542a28; continue; }
            pc = 0x5429ca; continue;
            }
            0x5429ca => {
            c.push(m,c.get(7,32));
            c.push(m, 0x5429d0);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x542a28; continue; }
            pc = 0x5429d9; continue;
            }
            0x5429d9 => {
            c.push(m,c.get(7,32));
            c.push(m,0x0u32);
            c.push(m,c.get(3,32));
            c.push(m, 0x5429e2);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(14) { pc = 0x542a08; continue; }
            pc = 0x5429e9; continue;
            }
            0x5429e9 => {
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x5429f5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1d),8));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x542a04; continue; }
            pc = 0x542a00; continue;
            }
            0x542a00 => {
            { let a = c.r[0].wrapping_add(c.r[3]); wm(m,a,8,0x1u32); }
            pc = 0x542a04; continue;
            }
            0x542a04 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(15) { pc = 0x5429e9; continue; }
            pc = 0x542a08; continue;
            }
            0x542a08 => {
            c.set(2,32,c.r[5].wrapping_add(0x10));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[5].wrapping_add(0x4),16));
            c.push(m,c.get(3,32));
            c.set(0,32,c.r[5].wrapping_add(0xc));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x542a1f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x542560, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(3,32));
            c.push(m, 0x542a25);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x542a28; continue;
            }
            0x542a28 => {
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

pub fn f_00542f10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x542f10;
    loop {
        match pc {
            0x542f10 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            if c.cond(7) { pc = 0x543192; continue; }
            pc = 0x542f42; continue;
            }
            0x542f42 => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x543478)); continue;
            }
            0x542f49 => {
            { let a = rm(m,0x6df31cu32,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x54346e; continue; }
            pc = 0x542f55; continue;
            }
            0x542f55 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,0x6dd894u32,16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x54346e; continue; }
            pc = 0x542f64; continue;
            }
            0x542f64 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(3,32,0xb9u32);
            pc = 0x542f6b; continue;
            }
            0x542f6b => {
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(7,16)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x34),16));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x18),32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x542faa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x180u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x542f6b; continue; }
            pc = 0x542fc1; continue;
            }
            0x542fc1 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x542fc9 => {
            { let a = rm(m,0x6df31cu32,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x54346e; continue; }
            pc = 0x542fd5; continue;
            }
            0x542fd5 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x54346e; continue; }
            pc = 0x542fe4; continue;
            }
            0x542fe4 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x542fe6; continue;
            }
            0x542fe6 => {
            c.set(0,32,rm(m,0x6de508u32,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xbau32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(3,8)); }
            c.set(5,32,0x18u32);
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2e4),16));
            { let a = c.get(2,16); let r = c.add(a,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2e0),16),16); c.set(2,16,r); }
            { let a = c.get(2,16); let r = c.add(a,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2dc),16),16); c.set(2,16,r); }
            { let a = c.get(2,16); let r = c.add(a,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2d8),16),16); c.set(2,16,r); }
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2c8),32));
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x2c8),8));
            c.set(0,32,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x310));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,8); let r = c.and(a,0x3fu32,8); c.set(1,8,r); }
            { let a = c.get(2,8); let r = c.and(a,0x3fu32,8); c.set(2,8,r); }
            { let a = c.get(1,8); let r = c.add(a,c.get(2,8),8); c.set(1,8,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            pc = 0x543057; continue;
            }
            0x543057 => {
            { let a = c.get(1,32); let r = c.add(a,rm(m,c.r[0].wrapping_add(0xfffffcf4),32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(2,32); let r = c.add(a,rm(m,c.r[0].wrapping_add(0xfffffffc),32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(7,32); let r = c.add(a,rm(m,c.r[0].wrapping_add(0xfffffd50),32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(7,32)); }
            if c.cond(5) { pc = 0x543057; continue; }
            pc = 0x543078; continue;
            }
            0x543078 => {
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x543087);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x9f8u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x542fe6; continue; }
            pc = 0x54309b; continue;
            }
            0x54309b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5430a3 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,0x6dd746u32,16));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6dd744u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6dd888u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,0x6dd88au32,16));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(0,16,rm(m,0x6dd882u32,16));
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6dd880u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6dd884u32,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6df2f0u32);
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6dd734u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.r[6]; wm(m,a,32,0xbcu32); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x543131);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x543139 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(5,16),16); }
            if c.cond(6) { pc = 0x54346e; continue; }
            pc = 0x543148; continue;
            }
            0x543148 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x543153);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6]; wm(m,a,32,0xbeu32); }
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x46c),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(6,32));
            c.set(0,32,sx(rm(m,c.r[0].wrapping_add(0x22),16),16)&0xffffffffu32);
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x54317c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x543148; continue; }
            pc = 0x54318a; continue;
            }
            0x54318a => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x543192 => {
            { let a = rm(m,c.r[6].wrapping_add(0x8),32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x5432c2; continue; }
            pc = 0x54319c; continue;
            }
            0x54319c => {
            { let a = c.r[6]; wm(m,a,32,0xbbu32); }
            c.set(2,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(7,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x543215; continue; }
            pc = 0x5431bb; continue;
            }
            0x5431bb => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x5431c7);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5431dc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x46c),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.add(a,c.get(1,32),32); c.set(2,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5431f5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x22),16));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(2,16); let r = c.add(a,0x64u32,16); c.set(2,16,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0x3fffffffu32,32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            if c.cond(7) { pc = 0x5431bb; continue; }
            pc = 0x543215; continue;
            }
            0x543215 => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x54324c; continue; }
            pc = 0x543224; continue;
            }
            0x543224 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x543226; continue;
            }
            0x543226 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(7,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x2d4),32));
            { let a = c.get(1,32); let r = c.add(a,0x9f8u32,32); c.set(1,32,r); }
            { let a = c.get(7,32); let r = c.add(a,c.get(2,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(7) { pc = 0x543226; continue; }
            pc = 0x54324c; continue;
            }
            0x54324c => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.set(0,16,rm(m,0x6dd746u32,16));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6dd744u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6dd888u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,16,rm(m,0x6dd88au32,16));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(2,16,rm(m,0x6dd882u32,16));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6dd880u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,0x6dd884u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            c.push(m, 0x5432c2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5432c2; continue;
            }
            0x5432c2 => {
            { let a = 0x6dfbf8u32; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6]; wm(m,a,32,0xd4u32); }
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x543304; continue; }
            pc = 0x5432d7; continue;
            }
            0x5432d7 => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(7,32,c.get(0,32));
            c.set(2,32,c.r[1].wrapping_add(0x4));
            pc = 0x5432e2; continue;
            }
            0x5432e2 => {
            c.set(0,32,c.get(2,32));
            c.set(1,32,0x18u32);
            pc = 0x5432e9; continue;
            }
            0x5432e9 => {
            c.set(5,32,rm(m,c.r[0],32));
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.and(a,0x3fffffu32,32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(5,32),32); c.set(3,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x5432e9; continue; }
            pc = 0x5432f9; continue;
            }
            0x5432f9 => {
            { let a = c.get(2,32); let r = c.add(a,0x9f8u32,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x5432e2; continue; }
            pc = 0x543302; continue;
            }
            0x543302 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x543304; continue;
            }
            0x543304 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(7,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x543320; continue;
            }
            0x543320 => {
            { let a = rm(m,c.r[1].wrapping_add(0x6dd7c0),16); c.sub(a,c.get(7,16),16); }
            if c.cond(3) { pc = 0x543342; continue; }
            pc = 0x543329; continue;
            }
            0x543329 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(5,16,rm(m,c.r[1].wrapping_add(0x6dd7c4),16));
            c.set(3,16,rm(m,c.r[1].wrapping_add(0x6dd7c6),16));
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            c.set(0,32,c.r[3].wrapping_add(c.r[5]));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x543342; continue;
            }
            0x543342 => {
            { let a = c.get(1,32); let r = c.add(a,0xcu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0xc0u32,32); }
            if c.cond(2) { pc = 0x543320; continue; }
            pc = 0x54334d; continue;
            }
            0x54334d => {
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let n = 0x10u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(7,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x5433b3; continue; }
            pc = 0x543367; continue;
            }
            0x543367 => {
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x543373);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6dd894u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xe),16));
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5433ad; continue; }
            pc = 0x543391; continue;
            }
            0x543391 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x20),32));
            { let a = c.get(3,32); let r = c.add(a,rm(m,c.r[0].wrapping_add(0x1c),32),32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x543391; continue; }
            pc = 0x5433ad; continue;
            }
            0x5433ad => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(7) { pc = 0x543367; continue; }
            pc = 0x5433b1; continue;
            }
            0x5433b1 => {
            pc = 0x5433b9; continue;
            }
            0x5433b3 => {
            c.set(2,32,rm(m,0x6dd7a4u32,32));
            pc = 0x5433b9; continue;
            }
            0x5433b9 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x5433bb; continue;
            }
            0x5433bb => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x6dd7ac),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5433f7; continue; }
            pc = 0x5433c5; continue;
            }
            0x5433c5 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xc),16));
            c.set(1,32,rm(m,0x6dd894u32,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5433f7; continue; }
            pc = 0x5433db; continue;
            }
            0x5433db => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,c.r[0].wrapping_add(0x20),32));
            { let a = c.get(3,32); let r = c.add(a,rm(m,c.r[0].wrapping_add(0x1c),32),32); c.set(3,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(3,32),32); c.set(5,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,16,rm(m,c.r[0].wrapping_add(0x4),16));
            c.set(0,32,c.get(3,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x5433db; continue; }
            pc = 0x5433f7; continue;
            }
            0x5433f7 => {
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0x14u32,32); }
            if c.cond(2) { pc = 0x5433bb; continue; }
            pc = 0x5433ff; continue;
            }
            0x5433ff => {
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,0x6de4aau32,16));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x543460; continue; }
            pc = 0x543411; continue;
            }
            0x543411 => {
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x54341d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,rm(m,0x6dd894u32,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xe),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x54345c; continue; }
            pc = 0x543435; continue;
            }
            0x543435 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(2,32,0x18u32);
            c.set(0,32,c.r[1].wrapping_add(0x54));
            pc = 0x54344a; continue;
            }
            0x54344a => {
            { let a = c.get(7,32); let r = c.add(a,rm(m,c.r[0],32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            if c.cond(5) { pc = 0x54344a; continue; }
            pc = 0x543452; continue;
            }
            0x543452 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x543435; continue; }
            pc = 0x54345c; continue;
            }
            0x54345c => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(7) { pc = 0x543411; continue; }
            pc = 0x543460; continue;
            }
            0x543460 => {
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.push(m, 0x54346e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x54346e; continue;
            }
            0x54346e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005436d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5436d0;
    loop {
        match pc {
            0x5436d0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,0x6dfc14u32,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            { let a = c.get(0,32); let r = c.and(a,0x7fu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x543741; continue; }
            pc = 0x5436e6; continue;
            }
            0x5436e6 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5436f1);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.r[7].wrapping_add(0x6));
            c.set(5,32,c.get(0,32));
            c.set(0,16,rm(m,0x6dd894u32,16));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(2,32,c.get(3,32));
            pc = 0x543700; continue;
            }
            0x543700 => {
            { let a = rm(m,c.r[2],16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x54370e; continue; }
            pc = 0x543705; continue;
            }
            0x543705 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,0x7u32,32); }
            if c.cond(2) { pc = 0x543700; continue; }
            pc = 0x54370e; continue;
            }
            0x54370e => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x4),8));
            c.push(m,0x0u32);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(9) { pc = 0x543732; continue; }
            pc = 0x543717; continue;
            }
            0x543717 => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.set(1,32,c.get(5,32));
            c.set(0,8,c.get(2,8));
            { let a = c.get(2,32); let r = c.and(a,0x7fu32,32); c.set(2,32,r); }
            { let a = c.get(0,8); let n = 0x7u32; let r = c.shr(a,n,8); c.set(0,8,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x54372d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4f5500, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x543732 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.push(m,c.get(3,32));
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x5),8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x543741);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4f56c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            pc = 0x543741; continue;
            }
            0x543741 => {
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

pub fn f_00543950(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x543950;
    loop {
        match pc {
            0x543950 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6636cbu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x543971);
            { let args = []; c.r[0] = host.call(m, 0x543a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x20u32);
            { let a = c.r[6]; wm(m,a,32,0x8u32); }
            c.push(m, 0x54397e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x54399b; continue; }
            pc = 0x543988; continue;
            }
            0x543988 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x54399b => {
            c.push(m,0x8u32);
            c.push(m, 0x5439a2);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5439c1; continue; }
            pc = 0x5439ac; continue;
            }
            0x5439ac => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x5439c1 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x5439c3; continue;
            }
            0x5439c3 => {
            c.push(m,0x1400u32);
            c.push(m, 0x5439cd);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x5439fa; continue; }
            pc = 0x5439e0; continue;
            }
            0x5439e0 => {
            c.set(7,32,c.get(0,32));
            c.set(5,32,0x100u32);
            pc = 0x5439e7; continue;
            }
            0x5439e7 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5439ee);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x5439e7; continue; }
            pc = 0x5439f4; continue;
            }
            0x5439f4 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5439fc; continue;
            }
            0x5439fa => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5439fc; continue;
            }
            0x5439fc => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[1].wrapping_add(c.r[3].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = rm(m,c.r[2].wrapping_add(c.r[3].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(4) { pc = 0x5439ac; continue; }
            pc = 0x543a13; continue;
            }
            0x543a13 => {
            { let a = c.get(3,32); c.sub(a,0x2u32,32); }
            if c.cond(3) { pc = 0x543a24; continue; }
            pc = 0x543a18; continue;
            }
            0x543a18 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(0,8,c.get(3,8));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.r[3].wrapping_add(c.r[1]); wm(m,a,8,c.get(0,8)); }
            pc = 0x543a2b; continue;
            }
            0x543a24 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[3].wrapping_add(c.r[2]); wm(m,a,8,0x0u32); }
            pc = 0x543a2b; continue;
            }
            0x543a2b => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x3u32,32); }
            if c.cond(2) { pc = 0x5439c3; continue; }
            pc = 0x543a31; continue;
            }
            0x543a31 => {
            c.set(1,32,rm(m,c.r[6],32));
            c.set(0,32,0x3u32);
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x543a4e; continue; }
            pc = 0x543a3c; continue;
            }
            0x543a3c => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0xfffffffc); wm(m,a,32,0x0u32); }
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x543a3c; continue; }
            pc = 0x543a4e; continue;
            }
            0x543a4e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00543ae0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x543ae0;
    loop {
        match pc {
            0x543ae0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6636e0u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = rm(m,c.r[3].wrapping_add(0xe),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x543c55; continue; }
            pc = 0x543b0b; continue;
            }
            0x543b0b => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x543b0f; continue;
            }
            0x543b0f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(7,16,rm(m,c.r[3].wrapping_add(0xe),16));
            c.set(0,32,0x100u32);
            { let a = c.get(7,32); let r = c.sub(a,c.get(5,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x543b27; continue; }
            pc = 0x543b25; continue;
            }
            0x543b25 => {
            c.set(7,32,c.get(0,32));
            pc = 0x543b27; continue;
            }
            0x543b27 => {
            c.set(0,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(1,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(2,32,c.r[3].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x10));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(1,32,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(2,32));
            c.push(m, 0x543b4f);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.add(a,c.get(7,32),32); c.set(5,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(0,32); c.sub(a,0x100u32,32); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(2) { pc = 0x543c4d; continue; }
            pc = 0x543b6d; continue;
            }
            0x543b6d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0]));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[2],8));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x543c0f; continue; }
            pc = 0x543b87; continue;
            }
            0x543b87 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(3,32,0x3u32);
            c.set(0,32,rm(m,c.r[2].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x543bad; continue; }
            pc = 0x543b96; continue;
            }
            0x543b96 => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x543ba4; continue; }
            pc = 0x543b9d; continue;
            }
            0x543b9d => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x543ba4);
            { let args = []; c.r[0] = host.call(m, 0x543d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x543ba4; continue;
            }
            0x543ba4 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)),32); c.sub(a,0x0u32,32); }
            if c.cond(5) { pc = 0x543b96; continue; }
            pc = 0x543bad; continue;
            }
            0x543bad => {
            c.push(m,0x1400u32);
            c.push(m, 0x543bb7);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x543be8; continue; }
            pc = 0x543bca; continue;
            }
            0x543bca => {
            c.set(7,32,c.get(0,32));
            c.set(5,32,0x100u32);
            pc = 0x543bd1; continue;
            }
            0x543bd1 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x543bd8);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.dec(a,32); c.set(5,32,r); }
            if c.cond(5) { pc = 0x543bd1; continue; }
            pc = 0x543bde; continue;
            }
            0x543bde => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x543bea; continue;
            }
            0x543be8 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x543bea; continue;
            }
            0x543bea => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[1].wrapping_add(c.r[3].wrapping_mul(4)); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(c.r[2]),8));
            { let a = c.r[0].wrapping_add(c.r[3]); wm(m,a,8,c.get(1,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.r[2].wrapping_add(c.r[0]); wm(m,a,8,c.get(3,8)); }
            pc = 0x543c38; continue;
            }
            0x543c0f => {
            { let a = c.get(7,32); c.sub(a,0x2u32,32); }
            if c.cond(6) { pc = 0x543c3a; continue; }
            pc = 0x543c14; continue;
            }
            0x543c14 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[7]),8));
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x543c3a; continue; }
            pc = 0x543c1d; continue;
            }
            0x543c1d => {
            { let a = c.r[2]; wm(m,a,8,c.get(3,8)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(c.r[7].wrapping_mul(4)),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x543c2b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[0].wrapping_add(c.r[7].wrapping_mul(4)); wm(m,a,32,0x0u32); }
            pc = 0x543c38; continue;
            }
            0x543c38 => {
            c.set(7,32,c.get(3,32));
            pc = 0x543c3a; continue;
            }
            0x543c3a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x543b0f; continue;
            }
            0x543c4d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[3].wrapping_add(0xe); wm(m,a,16,0x0u32); }
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x543c55; continue;
            }
            0x543c55 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00543c70(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x543c70;
    loop {
        match pc {
            0x543c70 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            pc = 0x543c7a; continue;
            }
            0x543c7a => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            if c.cond(5) { pc = 0x543c8c; continue; }
            pc = 0x543c87; continue;
            }
            0x543c87 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x10),32));
            pc = 0x543c91; continue;
            }
            0x543c8c => {
            c.set(7,32,0x100u32);
            pc = 0x543c91; continue;
            }
            0x543c91 => {
            { let a = c.get(7,32); let r = c.sub(a,c.get(0,32),32); c.set(7,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xe),16));
            c.set(1,32,0x34u32);
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x543ca6; continue; }
            pc = 0x543ca4; continue;
            }
            0x543ca4 => {
            c.set(7,32,c.get(1,32));
            pc = 0x543ca6; continue;
            }
            0x543ca6 => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,32,c.r[7].wrapping_add(c.r[7].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[3].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m, 0x543ccb);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[3].wrapping_add(0xe),16); let r = c.add(a,c.get(7,16),16); { let a = c.r[3].wrapping_add(0xe); wm(m,a,16,r); } }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,0x100u32,32); }
            if c.cond(2) { pc = 0x543cfa; continue; }
            pc = 0x543ce3; continue;
            }
            0x543ce3 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[2]),8));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x543c7a; continue;
            }
            0x543cfa => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005440a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5440a0;
    loop {
        match pc {
            0x5440a0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x420),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5440be; continue; }
            pc = 0x5440ad; continue;
            }
            0x5440ad => {
            c.push(m,0x1u32);
            c.push(m, 0x5440b4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5440a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[6].wrapping_add(0x420); wm(m,a,32,0x0u32); }
            pc = 0x5440be; continue;
            }
            0x5440be => {
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x5440ce; continue; }
            pc = 0x5440c5; continue;
            }
            0x5440c5 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x5440cb);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5440ce; continue;
            }
            0x5440ce => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00544140(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x544140;
    loop {
        match pc {
            0x544140 => {
            c.set(0,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            { let a = c.r[0].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00544160(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x544160;
    loop {
        match pc {
            0x544160 => {
            { let a = c.get(4,32); let r = c.sub(a,0x424u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            c.set(7,32,c.r[4].wrapping_add(0x20));
            c.set(3,32,0x34u32);
            pc = 0x544174; continue;
            }
            0x544174 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x54417b);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x544174; continue; }
            pc = 0x544181; continue;
            }
            0x544181 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5442e3; continue; }
            pc = 0x54418c; continue;
            }
            0x54418c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x438),16));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            c.set(3,32,0x1u32);
            if c.cond(5) { pc = 0x5441ea; continue; }
            pc = 0x5441ae; continue;
            }
            0x5441ae => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,0x6df30cu32,32));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,0xfffeu32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[1].wrapping_add(0x400));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5441d2; continue; }
            pc = 0x5441cb; continue;
            }
            0x5441cb => {
            c.set(0,32,rm(m,0x6df304u32,32));
            pc = 0x544211; continue;
            }
            0x5441d2 => {
            c.set(2,32,rm(m,0x6df304u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5441e8);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x544261; continue;
            }
            0x5441ea => {
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            if c.cond(5) { pc = 0x544244; continue; }
            pc = 0x5441ef; continue;
            }
            0x5441ef => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,0x6df310u32,32));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,0xffffu32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,c.r[1].wrapping_add(0x400));
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x544231; continue; }
            pc = 0x54420c; continue;
            }
            0x54420c => {
            c.set(0,32,rm(m,0x6df308u32,32));
            pc = 0x544211; continue;
            }
            0x544211 => {
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,0x400u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            c.push(m,c.get(1,32));
            c.push(m, 0x544223);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x400u32,32); c.set(7,32,r); }
            pc = 0x544266; continue;
            }
            0x544231 => {
            c.set(2,32,rm(m,0x6df308u32,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x28));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            pc = 0x544259; continue;
            }
            0x544244 => {
            c.set(1,32,rm(m,0x6cc3e8u32,32));
            c.push(m,0x4cu32);
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x26); wm(m,a,16,0xfffdu32); }
            c.push(m,c.get(2,32));
            pc = 0x544259; continue;
            }
            0x544259 => {
            c.push(m, 0x54425e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            pc = 0x544261; continue;
            }
            0x544261 => {
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x544266; continue;
            }
            0x544266 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x434),32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x420u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a2f0u32,32); c.push(m, 0x544284); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,0x320u32,32); }
            if c.cond(4) { pc = 0x5442dc; continue; }
            pc = 0x54428e; continue;
            }
            0x54428e => {
            { let a = c.get(0,32); c.sub(a,0x3e7u32,32); }
            if c.cond(4) { pc = 0x5442dc; continue; }
            pc = 0x544295; continue;
            }
            0x544295 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5442a7; continue; }
            pc = 0x544299; continue;
            }
            0x544299 => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x424u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x5442a7 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            if c.cond(5) { pc = 0x5442cb; continue; }
            pc = 0x5442b9; continue;
            }
            0x5442b9 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,0x4u32,32); }
            c.set(0,8,u32::from(c.cond(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0]).wrapping_add(0x3));
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            pc = 0x5442cb; continue;
            }
            0x5442cb => {
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x424u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x5442dc => {
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,0x3u32); }
            pc = 0x5442e3; continue;
            }
            0x5442e3 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x424u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00544300(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x544300;
    loop {
        match pc {
            0x544300 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x544312);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(3,32,0x4u32);
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x54440b; continue; }
            pc = 0x544328; continue;
            }
            0x544328 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xe),16));
            { let a = c.get(0,16); c.sub(a,0xfffeu32,16); }
            if c.cond(5) { pc = 0x54438a; continue; }
            pc = 0x544332; continue;
            }
            0x544332 => {
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6cbb94u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x544345; continue; }
            pc = 0x54433e; continue;
            }
            0x54433e => {
            { let a = c.r[0].wrapping_add(0x20c); wm(m,a,8,0x1u32); }
            pc = 0x544345; continue;
            }
            0x544345 => {
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = 0x6df30cu32; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,0x6df304u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x544362; continue; }
            pc = 0x544359; continue;
            }
            0x544359 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x54435f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x544362; continue;
            }
            0x544362 => {
            c.set(1,32,rm(m,0x6df30cu32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x54436e);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6df304u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x544376; continue;
            }
            0x544376 => {
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x54440b; continue; }
            pc = 0x54437e; continue;
            }
            0x54437e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x54438a => {
            { let a = c.get(0,16); c.sub(a,0xffffu32,16); }
            if c.cond(5) { pc = 0x5443c7; continue; }
            pc = 0x544390; continue;
            }
            0x544390 => {
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,0x5u32); }
            c.set(0,32,rm(m,0x6df308u32,32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = 0x6df310u32; wm(m,a,32,c.get(2,32)); }
            if c.cond(4) { pc = 0x5443b2; continue; }
            pc = 0x5443a9; continue;
            }
            0x5443a9 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x5443af);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x5443b2; continue;
            }
            0x5443b2 => {
            c.set(0,32,rm(m,0x6df310u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5443bd);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = 0x6df308u32; wm(m,a,32,c.get(0,32)); }
            pc = 0x544376; continue;
            }
            0x5443c7 => {
            { let a = c.get(0,16); c.sub(a,0xfffdu32,16); }
            if c.cond(5) { pc = 0x54437e; continue; }
            pc = 0x5443cd; continue;
            }
            0x5443cd => {
            c.set(0,32,rm(m,0x6cc3e8u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x5443e9; continue; }
            pc = 0x5443d6; continue;
            }
            0x5443d6 => {
            c.push(m,0x4cu32);
            c.push(m, 0x5443dd);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            { let a = 0x6cc3e8u32; wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x54437e; continue; }
            pc = 0x5443e9; continue;
            }
            0x5443e9 => {
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            c.push(m,0x4cu32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5443f5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,0x3u32); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x54440b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.r[1]; wm(m,a,32,0x1u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x400u32,32); }
            if c.cond(6) { pc = 0x544497; continue; }
            pc = 0x544424; continue;
            }
            0x544424 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            c.push(m,0x400u32);
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(5) { pc = 0x54443e; continue; }
            pc = 0x544434; continue;
            }
            0x544434 => {
            c.set(2,32,rm(m,0x6df304u32,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            pc = 0x544445; continue;
            }
            0x54443e => {
            c.set(0,32,rm(m,0x6df308u32,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            pc = 0x544445; continue;
            }
            0x544445 => {
            c.push(m,c.get(1,32));
            c.push(m, 0x54444b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x1c),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x400u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x54448b; continue; }
            pc = 0x544467; continue;
            }
            0x544467 => {
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,rm(m,0x6df31cu32,32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.push(m,c.get(2,32));
            c.set(1,32,0x6df2f0u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xc5u32); }
            c.push(m, 0x54448b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x54448b; continue;
            }
            0x54448b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x544497 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x18),32));
            { let a = c.get(2,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x544519; continue; }
            pc = 0x54449e; continue;
            }
            0x54449e => {
            c.set(2,32,rm(m,0x6df304u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x54437e; continue; }
            pc = 0x5444ac; continue;
            }
            0x5444ac => {
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x5444b9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5444c3);
            { let args = []; c.r[0] = host.call(m, 0x544590, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,8,c.get(0,8));
            c.set(0,32,rm(m,0x6df304u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5444d0);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6df304u32; wm(m,a,32,c.get(5,32)); }
            { let a = 0x6df30cu32; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,0x5u32); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,rm(m,0x6df31cu32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,0x6df2f0u32);
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xc5u32); }
            c.push(m, 0x54450d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x544519 => {
            { let a = c.get(2,32); c.sub(a,0x5u32,32); }
            if c.cond(5) { pc = 0x54437e; continue; }
            pc = 0x544522; continue;
            }
            0x544522 => {
            c.set(2,32,rm(m,0x6df308u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x54437e; continue; }
            pc = 0x544530; continue;
            }
            0x544530 => {
            { let a = c.get(7,32); let r = c.add(a,0x14u32,32); c.set(7,32,r); }
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x54453d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x544547);
            { let args = []; c.r[0] = host.call(m, 0x544730, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6df308u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x544552);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = 0x6df308u32; wm(m,a,32,c.get(5,32)); }
            { let a = 0x6df310u32; wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,0x3u32); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,0x6cbb94u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x54457b; continue; }
            pc = 0x544574; continue;
            }
            0x544574 => {
            { let a = c.r[0].wrapping_add(0x20d); wm(m,a,8,0x1u32); }
            pc = 0x54457b; continue;
            }
            0x54457b => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_00549190(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x549190;
    loop {
        match pc {
            0x549190 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x663937u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x5492a8; continue; }
            pc = 0x5491cd; continue;
            }
            0x5491cd => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5491d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x19),8));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xe4),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,16,0x8001u32); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(c.r[6]).wrapping_add(0x39c),8));
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dda00),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.push(m,c.get(0,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1a),8));
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.and(a,0x1u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,32,rm(m,0x6c35e4u32,32));
            c.set(0,32,rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6a0608),32));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x549232);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cbb40u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x54925a; continue; }
            pc = 0x54923e; continue;
            }
            0x54923e => {
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x54924f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x54925a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42b4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x54925a; continue;
            }
            0x54925a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x5492a8; continue; }
            pc = 0x549262; continue;
            }
            0x549262 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xe4),32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xe8),32));
            c.set(0,32,rm(m,0x6c35e8u32,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x549280);
            { let args = []; c.r[0] = host.call(m, 0x64df80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6cbb40u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5492a8; continue; }
            pc = 0x54928c; continue;
            }
            0x54928c => {
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[4].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x54929d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x5492a8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42b4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5492a8; continue;
            }
            0x5492a8 => {
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x5492b9);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_0054a420(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x54a420;
    loop {
        match pc {
            0x54a420 => {
            c.push(m,c.get(6,32));
            c.push(m,0x14u32);
            c.set(1,32,0x6dd73cu32);
            c.push(m, 0x54a42d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8e70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = rm(m,0x6de4aau32,16); c.sub(a,c.get(6,16),16); }
            if c.cond(6) { pc = 0x54a47a; continue; }
            pc = 0x54a438; continue;
            }
            0x54a438 => {
            c.push(m,c.get(6,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x54a443);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(5) { pc = 0x54a46d; continue; }
            pc = 0x54a44a; continue;
            }
            0x54a44a => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,0x6dd73cu32);
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x54a45c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x14u32); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,16,c.get(6,16)); }
            pc = 0x54a46d; continue;
            }
            0x54a46d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x54a438; continue; }
            pc = 0x54a47a; continue;
            }
            0x54a47a => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005338a0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5338a0;
    loop {
        match pc {
            0x5338a0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6634b8u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x5338c7);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,16,rm(m,0x6de4aau32,16));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0x0u32); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x8),8));
            { let a = c.get(1,16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x533cba; continue; }
            pc = 0x5338e7; continue;
            }
            0x5338e7 => {
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x9),8));
            { let a = c.get(2,16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x533cba; continue; }
            pc = 0x5338f5; continue;
            }
            0x5338f5 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x533947; continue; }
            pc = 0x5338fd; continue;
            }
            0x5338fd => {
            c.set(0,32,rm(m,c.r[3],32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,16,rm(m,c.r[3].wrapping_add(0x9),8));
            c.set(5,16,rm(m,c.r[3].wrapping_add(0x8),8));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x533924; continue; }
            pc = 0x533910; continue;
            }
            0x533910 => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(1,16); c.sub(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x533924; continue; }
            pc = 0x53391a; continue;
            }
            0x53391a => {
            c.set(1,16,rm(m,c.r[6].wrapping_add(0xd),8));
            { let a = c.get(1,16); c.sub(a,c.get(2,16),16); }
            if c.cond(4) { pc = 0x533936; continue; }
            pc = 0x533924; continue;
            }
            0x533924 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x533947; continue; }
            pc = 0x53392a; continue;
            }
            0x53392a => {
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x533932);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x533ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x533947; continue; }
            pc = 0x533936; continue;
            }
            0x533936 => {
            c.set(1,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[3]; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            if c.cond(3) { pc = 0x5338fd; continue; }
            pc = 0x533942; continue;
            }
            0x533942 => {
            pc = 0x533cba; continue;
            }
            0x533947 => {
            { let a = c.r[7]; wm(m,a,32,c.get(6,32)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.add(a,0x5u32,32); c.set(0,32,r); }
            c.set(1,32,0x6dd73cu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x53395c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x13u32); }
            c.set(2,32,rm(m,c.r[3],32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x8),8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x9),8));
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0x8),8));
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x9),8));
            { let a = c.r[7].wrapping_add(0xd); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0xa),8));
            { let a = c.r[7].wrapping_add(0xe); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0xb),8));
            { let a = c.r[7].wrapping_add(0xf); wm(m,a,8,c.get(2,8)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0xc),32));
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xb),8));
            { let a = c.get(0,32); let r = c.sub(a,0x0u32,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x533a91; continue; }
            pc = 0x5339ae; continue;
            }
            0x5339ae => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x5339f8; continue; }
            pc = 0x5339b1; continue;
            }
            0x5339b1 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            if c.cond(5) { pc = 0x533c62; continue; }
            pc = 0x5339b8; continue;
            }
            0x5339b8 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x5339c8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[0],32));
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x10),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x5339da; continue; }
            pc = 0x5339d1; continue;
            }
            0x5339d1 => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x533c62; continue;
            }
            0x5339da => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(15) { pc = 0x5339ea; continue; }
            pc = 0x5339de; continue;
            }
            0x5339de => {
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            pc = 0x533c62; continue;
            }
            0x5339ea => {
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            pc = 0x533c62; continue;
            }
            0x5339f8 => {
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(0,16); c.sub(a,c.get(2,16),16); }
            if c.cond(3) { pc = 0x533cba; continue; }
            pc = 0x533a0b; continue;
            }
            0x533a0b => {
            c.set(5,32,rm(m,0x6dd7a4u32,32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(6,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(5,32),32); c.set(6,32,r); }
            { let a = rm(m,c.r[6].wrapping_add(0x8),16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x533a32; continue; }
            pc = 0x533a27; continue;
            }
            0x533a27 => {
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,16,0xffffu32); }
            pc = 0x533cba; continue;
            }
            0x533a32 => {
            c.set(0,32,rm(m,0x6dd888u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x533a63; continue; }
            pc = 0x533a4a; continue;
            }
            0x533a4a => {
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x533a6e; continue; }
            pc = 0x533a4e; continue;
            }
            0x533a4e => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[5]).wrapping_add(0x6),16));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x533a4a; continue; }
            pc = 0x533a63; continue;
            }
            0x533a63 => {
            { let a = c.r[7].wrapping_add(0x10); wm(m,a,16,0xffffu32); }
            pc = 0x533cba; continue;
            }
            0x533a6e => {
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd7a0u32);
            c.push(m, 0x533a79);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x508fc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[6].wrapping_add(0x3c),8); let r = c.or(a,0x20u32,8); { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,r); } }
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,16,0xdu32); }
            { let a = c.r[6].wrapping_add(0x6); wm(m,a,16,0xffffu32); }
            pc = 0x533c62; continue;
            }
            0x533a91 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x12),16));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x533ab3; continue; }
            pc = 0x533aa7; continue;
            }
            0x533aa7 => {
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(5,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(5,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            pc = 0x533ac0; continue;
            }
            0x533ab3 => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(5,32,c.r[5].wrapping_add(c.r[5].wrapping_mul(4)));
            { let a = c.get(5,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            pc = 0x533ac0; continue;
            }
            0x533ac0 => {
            c.set(2,16,rm(m,c.r[7].wrapping_add(0xe),8));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xc),8));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x533ad8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x533b47; continue; }
            pc = 0x533ae0; continue;
            }
            0x533ae0 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(1,32,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            if c.cond(12) { pc = 0x533af8; continue; }
            pc = 0x533aee; continue;
            }
            0x533aee => {
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[0].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x533b47; continue;
            }
            0x533af8 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x672c14),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x533b1e; continue; }
            pc = 0x533b02; continue;
            }
            0x533b02 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x533b3b; continue;
            }
            0x533b1e => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            pc = 0x533b3b; continue;
            }
            0x533b3b => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6].wrapping_mul(4)).wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x533b47; continue;
            }
            0x533b47 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xc),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x533b57);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xe),16));
            if c.cond(14) { pc = 0x533c62; continue; }
            pc = 0x533b67; continue;
            }
            0x533b67 => {
            pc = 0x533b6d; continue;
            }
            0x533b69 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            pc = 0x533b6d; continue;
            }
            0x533b6d => {
            c.set(2,32,rm(m,0x6dd894u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x533c62; continue; }
            pc = 0x533b81; continue;
            }
            0x533b81 => {
            c.set(1,32,rm(m,0x6dd7a4u32,32));
            c.set(6,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(6,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x4),16));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x39),8));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xe),8));
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x533c5a; continue; }
            pc = 0x533ba7; continue;
            }
            0x533ba7 => {
            { let a = rm(m,c.r[6].wrapping_add(0x134),16); c.sub(a,0x4u32,16); }
            if c.cond(3) { pc = 0x533c5a; continue; }
            pc = 0x533bb5; continue;
            }
            0x533bb5 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(0,32,rm(m,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x54),32));
            c.set(1,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x54));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x533bd1; continue; }
            pc = 0x533bc7; continue;
            }
            0x533bc7 => {
            { let a = c.get(5,32); let r = c.sub(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.r[1]; wm(m,a,32,0x0u32); }
            pc = 0x533c4a; continue;
            }
            0x533bd1 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            c.set(2,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54),32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x533c19; continue; }
            pc = 0x533beb; continue;
            }
            0x533beb => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(0,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x533c48; continue;
            }
            0x533c19 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x10),16));
            c.set(0,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54));
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x54),32));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            pc = 0x533c48; continue;
            }
            0x533c48 => {
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            pc = 0x533c4a; continue;
            }
            0x533c4a => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x533c51);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x4u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x533c5a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x517770, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x533c5a; continue;
            }
            0x533c5a => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(15) { pc = 0x533b69; continue; }
            pc = 0x533c62; continue;
            }
            0x533c62 => {
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xa),8));
            c.set(1,32,rm(m,c.r[3],32));
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x8),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            c.set(1,16,rm(m,c.r[3].wrapping_add(0x9),8));
            c.set(2,8,c.get(0,8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(2,16)); }
            c.set(2,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(1,16)); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1eu32); }
            c.push(m, 0x533ca0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,8,rm(m,c.r[3].wrapping_add(0x9),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1fu32); }
            c.push(m, 0x533cba);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x533cba; continue;
            }
            0x533cba => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x533ccb);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x1cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn f_005063c0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5063c0;
    loop {
        match pc {
            0x5063c0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66300du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0xffffffffu32); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0xf4),16));
            { let a = c.get(1,16); c.sub(a,c.get(0,16),16); }
            if c.cond(3) { pc = 0x506417; continue; }
            pc = 0x5063f6; continue;
            }
            0x5063f6 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(0,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(0,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x1e),16));
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x22),16));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,16,c.get(0,16)); }
            pc = 0x506424; continue;
            }
            0x506417 => {
            c.set(6,32,0x3d2u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x317u32); }
            pc = 0x506424; continue;
            }
            0x506424 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(3,32,0x3u32);
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(6,32)); }
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            c.set(5,32,c.get(2,32));
            { let a = c.get(5,16); c.and(a,c.get(5,16),16); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(5,32)); }
            if c.cond(5) { pc = 0x506448; continue; }
            pc = 0x506444; continue;
            }
            0x506444 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x50648d; continue;
            }
            0x506448 => {
            { let a = c.get(5,16); c.sub(a,0x1u32,16); }
            if c.cond(4) { pc = 0x506488; continue; }
            pc = 0x50644e; continue;
            }
            0x50644e => {
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[7].wrapping_add(0xf4),16),16); }
            c.set(3,32,0x3u32);
            if c.cond(3) { pc = 0x50648d; continue; }
            pc = 0x50645c; continue;
            }
            0x50645c => {
            c.set(0,32,0x673568u32);
            pc = 0x506461; continue;
            }
            0x506461 => {
            { let a = c.get(6,16); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffffc),16),16); }
            if c.cond(12) { pc = 0x50647c; continue; }
            pc = 0x506467; continue;
            }
            0x506467 => {
            { let a = c.get(6,16); c.sub(a,rm(m,c.r[0],16),16); }
            if c.cond(15) { pc = 0x50647c; continue; }
            pc = 0x50646c; continue;
            }
            0x50646c => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffffe),16),16); }
            if c.cond(12) { pc = 0x50647c; continue; }
            pc = 0x506476; continue;
            }
            0x506476 => {
            { let a = c.get(1,16); c.sub(a,rm(m,c.r[0].wrapping_add(0x2),16),16); }
            if c.cond(14) { pc = 0x506488; continue; }
            pc = 0x50647c; continue;
            }
            0x50647c => {
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x673580u32,32); }
            if c.cond(12) { pc = 0x506461; continue; }
            pc = 0x506486; continue;
            }
            0x506486 => {
            pc = 0x50648d; continue;
            }
            0x506488 => {
            c.set(3,32,0x1u32);
            pc = 0x50648d; continue;
            }
            0x50648d => {
            c.push(m,0x3u32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x50649d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x507d60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5064ac);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x52a8d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(5,16); c.sub(a,0x2u32,16); }
            { let a = c.get(0,8); let r = c.sbb(a,c.get(0,8),8); c.set(0,8,r); }
            c.set(1,32,c.get(6,32));
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(2,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5064c3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51a750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6ddb78u32,32));
            c.set(2,16,rm(m,0x6dd88eu32,16));
            c.set(5,32,c.r[6].wrapping_add(0x160));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            pc = 0x5064de; continue;
            }
            0x5064de => {
            { let a = c.get(2,16); let r = c.add(a,rm(m,0x6dd890u32,16),16); c.set(2,16,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,0x133u32);
            c.set(0,16,c.get(2,16));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            { let a = c.get(2,16); c.sub(a,c.get(1,16),16); }
            if c.cond(3) { pc = 0x5064de; continue; }
            pc = 0x5064f7; continue;
            }
            0x5064f7 => {
            { let a = 0x6dd88eu32; wm(m,a,16,c.get(2,16)); }
            { let a = c.r[6].wrapping_add(0x15e); wm(m,a,16,c.get(2,16)); }
            c.set(1,16,rm(m,0x6dd88eu32,16));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6ddaa0u32);
            c.push(m, 0x506517);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x512b20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x506535; continue; }
            pc = 0x50651f; continue;
            }
            0x50651f => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x506523; continue;
            }
            0x506523 => {
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0x20u32,32); }
            if c.cond(13) { pc = 0x506535; continue; }
            pc = 0x506529; continue;
            }
            0x506529 => {
            { let a = c.r[5]; wm(m,a,8,c.get(1,8)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x506523; continue; }
            pc = 0x506535; continue;
            }
            0x506535 => {
            { let a = c.r[5]; wm(m,a,8,c.get(3,8)); }
            { let a = rm(m,c.r[6].wrapping_add(0x3c),8); let r = c.or(a,0x1u32,8); { let a = c.r[6].wrapping_add(0x3c); wm(m,a,8,r); } }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x506543);
            { let args = []; c.r[0] = host.call(m, 0x5185c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x40); wm(m,a,16,c.get(0,16)); }
            c.push(m, 0x506550);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5097c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,16,c.get(0,16));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x38),8));
            { let a = c.get(0,8); let r = c.add(a,0x2u32,8); c.set(0,8,r); }
            c.set(2,8,0x33u32);
            { let (lo,hi) = c.imul1(c.get(0,8),c.get(2,8),8); c.set(0,16,lo | (hi<<8)); }
            { let a = c.r[6].wrapping_add(0x42); wm(m,a,16,c.get(1,16)); }
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[1].wrapping_add(c.r[2]).wrapping_add(0xb); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x506585);
            { let args = []; c.r[0] = host.call(m, 0x515750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x50658f);
            { let args = []; c.r[0] = host.call(m, 0x5182b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x506591; continue;
            }
            0x506591 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[5].wrapping_mul(4)).wrapping_add(0xc),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5065b2; continue; }
            pc = 0x506599; continue;
            }
            0x506599 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5065a3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x515ea0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5065ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51a4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(5,32,0x5u32);
            pc = 0x506615; continue;
            }
            0x5065b2 => {
            { let a = c.get(5,32); c.sub(a,0x4u32,32); }
            if c.cond(5) { pc = 0x506615; continue; }
            pc = 0x5065b7; continue;
            }
            0x5065b7 => {
            c.push(m,0x18u32);
            c.push(m, 0x5065be);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x5065d8; continue; }
            pc = 0x5065cd; continue;
            }
            0x5065cd => {
            c.set(1,32,c.get(0,32));
            c.push(m, 0x5065d4);
            { let args = []; c.r[0] = host.call(m, 0x513720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(0,32));
            pc = 0x5065da; continue;
            }
            0x5065d8 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x5065da; continue;
            }
            0x5065da => {
            { let a = c.r[7].wrapping_add(0x1c); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(5,32,rm(m,0x6cc3e8u32,32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0xffffffffu32); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x13),8));
            c.set(5,32,0x28u32);
            c.push(m,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let b = c.get(5,32); c.div32(b); }
            c.push(m,c.get(2,32));
            c.push(m, 0x506610);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x513d80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(5,32,0x3u32);
            pc = 0x506615; continue;
            }
            0x506615 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(5,32); c.sub(a,0x5u32,32); }
            if c.cond(12) { pc = 0x506591; continue; }
            pc = 0x50661f; continue;
            }
            0x50661f => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0x18),16));
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x14),16));
            { let a = rm(m,c.r[6].wrapping_add(0x3d),8); let r = c.or(a,0x20u32,8); { let a = c.r[6].wrapping_add(0x3d); wm(m,a,8,r); } }
            { let a = c.r[6].wrapping_add(0x1e); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,0x7fffu32);
            { let a = c.r[6].wrapping_add(0x134); wm(m,a,16,0x12u32); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x20); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.r[6].wrapping_add(0x22); wm(m,a,16,c.get(1,16)); }
            { let a = c.r[6].wrapping_add(0x50); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[6].wrapping_add(0x52); wm(m,a,16,c.get(0,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,c.r[7].wrapping_add(0xec));
            c.push(m,c.get(2,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x50666a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5062f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(0,16,rm(m,c.r[5],16));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x16); wm(m,a,16,c.get(3,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[5],16));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x14),8); let r = c.or(a,0x40u32,8); { let a = c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x14); wm(m,a,8,r); } }
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x14));
            { let a = c.r[6].wrapping_add(0x138); wm(m,a,16,c.get(3,16)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0xea),16));
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5066b9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x50ce50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x136),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,8); let r = c.or(a,0xc0u32,8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[6].wrapping_add(0x136); wm(m,a,8,c.get(0,8)); }
            c.set(0,16,rm(m,c.r[4].wrapping_add(0xc),16));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}
