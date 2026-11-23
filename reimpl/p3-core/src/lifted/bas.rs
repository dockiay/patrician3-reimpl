//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn archive_file_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402740;
    loop {
        match pc {
            0x402740 => {
            c.set(0,32,c.get(1,32));
            { let a = c.r[0]; wm(m,a,32,0x66a7dcu32); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402750;
    loop {
        match pc {
            0x402750 => {
            { let a = c.r[1]; wm(m,a,32,0x66a7dcu32); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_open(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402760;
    loop {
        match pc {
            0x402760 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x402774);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_close(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402780;
    loop {
        match pc {
            0x402780 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x40279a; continue; }
            pc = 0x40278a; continue;
            }
            0x40278a => {
            c.push(m,c.get(0,32));
            c.push(m, 0x402790);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6389a6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            pc = 0x40279a; continue;
            }
            0x40279a => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_read(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4027a0;
    loop {
        match pc {
            0x4027a0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4027bd; continue; }
            pc = 0x4027a7; continue;
            }
            0x4027a7 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4027b7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x6389ac, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4027bd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_seek(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4027d0;
    loop {
        match pc {
            0x4027d0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x402851; continue; }
            pc = 0x4027da; continue;
            }
            0x4027da => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.sub(a,0x0u32,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x40283c; continue; }
            pc = 0x4027e3; continue;
            }
            0x4027e3 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x40281c; continue; }
            pc = 0x4027e6; continue;
            }
            0x4027e6 => {
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(4) { pc = 0x4027fc; continue; }
            pc = 0x4027e9; continue;
            }
            0x4027e9 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4027f5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389be, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4027fc => {
            c.push(m,c.get(0,32));
            c.push(m, 0x402802);
            { let args = []; c.r[0] = host.call(m, 0x6389b8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x402815);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389be, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x40281c => {
            c.push(m,c.get(0,32));
            c.push(m, 0x402822);
            { let args = []; c.r[0] = host.call(m, 0x6389b2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x402835);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389be, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x40283c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40284a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389be, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x402851 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_get_length(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402860;
    loop {
        match pc {
            0x402860 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x402871; continue; }
            pc = 0x402867; continue;
            }
            0x402867 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x40286d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6389b8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x402871 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x402880;
    loop {
        match pc {
            0x402880 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x4),8));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,8); c.and(a,0x1u32,8); }
            { let a = c.r[6]; wm(m,a,32,0x66a7dcu32); }
            if c.cond(4) { pc = 0x40289a; continue; }
            pc = 0x402891; continue;
            }
            0x402891 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x402897);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x40289a; continue;
            }
            0x40289a => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn cstring_get_pipe_field(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40e990;
    loop {
        match pc {
            0x40e990 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(0,32); c.sub(a,0x1u32,32); }
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            if c.cond(14) { pc = 0x40e9e1; continue; }
            pc = 0x40e9a8; continue;
            }
            0x40e9a8 => {
            c.push(m,c.get(3,32));
            c.set(3,32,c.r[0].wrapping_add(0xffffffff));
            pc = 0x40e9ac; continue;
            }
            0x40e9ac => {
            c.push(m,c.get(6,32));
            c.push(m,0x693302u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x40e9b9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64d3d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(6,32,c.get(0,32));
            { let a = c.get(6,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(14) { pc = 0x40e9c1; continue; }
            pc = 0x40e9c0; continue;
            }
            0x40e9c0 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x40e9c1; continue;
            }
            0x40e9c1 => {
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x40e9ac; continue; }
            pc = 0x40e9c4; continue;
            }
            0x40e9c4 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(13) { pc = 0x40e9e3; continue; }
            pc = 0x40e9c9; continue;
            }
            0x40e9c9 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x40e9d9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64d0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x40e9e1 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x40e9e3; continue;
            }
            0x40e9e3 => {
            c.push(m,c.get(6,32));
            c.push(m,0x693304u32);
            c.set(1,32,c.get(7,32));
            c.push(m, 0x40e9f0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64d3d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x40e9f9; continue; }
            pc = 0x40e9f4; continue;
            }
            0x40e9f4 => {
            c.set(0,32,rm(m,c.r[7],32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            pc = 0x40e9f9; continue;
            }
            0x40e9f9 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(6,32));
            c.push(m, 0x40ea09);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64d0c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn cstring_get_pipe_field_as_int(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40ea20;
    loop {
        match pc {
            0x40ea20 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(2,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x40ea30);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x40e990, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[0],32));
            c.push(m,c.get(0,32));
            c.push(m, 0x40ea38);
            { let args = []; c.r[0] = host.call(m, 0x639cda, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.set(6,32,c.get(0,32));
            c.push(m, 0x40ea46);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn cstring_parse_hex_consuming(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40eab0;
    loop {
        match pc {
            0x40eab0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x65b61cu32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x40eb24; continue; }
            pc = 0x40eadc; continue;
            }
            0x40eadc => {
            c.set(7,32,c.get(0,32));
            pc = 0x40eade; continue;
            }
            0x40eade => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.sub(a,0x30u32,8); }
            if c.cond(12) { pc = 0x40eaf6; continue; }
            pc = 0x40eaeb; continue;
            }
            0x40eaeb => {
            { let a = c.get(0,8); c.sub(a,0x39u32,8); }
            if c.cond(15) { pc = 0x40eaf6; continue; }
            pc = 0x40eaef; continue;
            }
            0x40eaef => {
            c.set(2,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.set(6,32,c.r[6].wrapping_add(c.r[2]).wrapping_add(0xffffffd0));
            pc = 0x40eaf6; continue;
            }
            0x40eaf6 => {
            { let a = c.get(0,8); c.sub(a,0x61u32,8); }
            if c.cond(12) { pc = 0x40eb05; continue; }
            pc = 0x40eafa; continue;
            }
            0x40eafa => {
            { let a = c.get(0,8); c.sub(a,0x66u32,8); }
            if c.cond(15) { pc = 0x40eb05; continue; }
            pc = 0x40eafe; continue;
            }
            0x40eafe => {
            c.set(1,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.set(6,32,c.r[6].wrapping_add(c.r[1]).wrapping_add(0xffffffa9));
            pc = 0x40eb05; continue;
            }
            0x40eb05 => {
            { let a = c.get(0,8); c.sub(a,0x41u32,8); }
            if c.cond(12) { pc = 0x40eb14; continue; }
            pc = 0x40eb09; continue;
            }
            0x40eb09 => {
            { let a = c.get(0,8); c.sub(a,0x46u32,8); }
            if c.cond(15) { pc = 0x40eb14; continue; }
            pc = 0x40eb0d; continue;
            }
            0x40eb0d => {
            c.set(2,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.set(6,32,c.r[6].wrapping_add(c.r[2]).wrapping_add(0xffffffc9));
            pc = 0x40eb14; continue;
            }
            0x40eb14 => {
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m, 0x40eb21);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64cc80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(7,32); let r = c.dec(a,32); c.set(7,32,r); }
            if c.cond(5) { pc = 0x40eade; continue; }
            pc = 0x40eb24; continue;
            }
            0x40eb24 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x40eb35);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn cstring_construct_empty(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4103c0;
    loop {
        match pc {
            0x4103c0 => {
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn operator_delete_stdcall_thunk(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x41bfd0;
    loop {
        match pc {
            0x41bfd0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x41bfda);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn two_string_members_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42db70;
    loop {
        match pc {
            0x42db70 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65d238u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x0u32); }
            c.push(m, 0x42db9d);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x42dbac);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
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

pub fn rect_get_width(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x430f80;
    loop {
        match pc {
            0x430f80 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn file_stream_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x434aa0;
    loop {
        match pc {
            0x434aa0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x434aa8);
            { let args = []; c.r[0] = host.call(m, 0x434ac0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x434ab8; continue; }
            pc = 0x434aaf; continue;
            }
            0x434aaf => {
            c.push(m,c.get(6,32));
            c.push(m, 0x434ab5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x434ab8; continue;
            }
            0x434ab8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn file_stream_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x434ac0;
    loop {
        match pc {
            0x434ac0 => {
            { let a = c.r[1]; wm(m,a,32,0x67cd9cu32); }
            { let a = c.get(1,32); let r = c.add(a,0x10u32,32); c.set(1,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn get_first_dword(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x440750;
    loop {
        match pc {
            0x440750 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn load_all_fonts(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x464250;
    loop {
        match pc {
            0x464250 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x65ef1fu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x18u32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x3),8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,0x698ac0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x46428c);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x46429d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4642c0; continue; }
            pc = 0x4642a1; continue;
            }
            0x4642a1 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(6,32));
            c.push(m,0x698ac0u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4642b1);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[2].wrapping_add(c.r[6]); wm(m,a,8,0x0u32); }
            pc = 0x4642c0; continue;
            }
            0x4642c0 => {
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(6,32,0x6dcd28u32);
            pc = 0x4642cb; continue;
            }
            0x4642cb => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x4642d5);
            { let args = []; c.r[0] = host.call(m, 0x464400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4642e2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4642e9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4c1230, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(6,32); let r = c.add(a,0xa8u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(6,32); c.sub(a,0x6dd118u32,32); }
            c.set(3,8,c.get(0,8));
            if c.cond(12) { pc = 0x4642cb; continue; }
            pc = 0x4642fa; continue;
            }
            0x4642fa => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x46431e; continue; }
            pc = 0x464302; continue;
            }
            0x464302 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x464315; continue; }
            pc = 0x46430a; continue;
            }
            0x46430a => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x464315; continue; }
            pc = 0x46430f; continue;
            }
            0x46430f => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x46431e; continue;
            }
            0x464315 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x46431b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x46431e; continue;
            }
            0x46431e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,c.get(3,8));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn copy_string_field_0x34(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4643a0;
    loop {
        match pc {
            0x4643a0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.add(a,0x34u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4643b9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_tidy(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4643c0;
    loop {
        match pc {
            0x4643c0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4643e6; continue; }
            pc = 0x4643ca; continue;
            }
            0x4643ca => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4643dd; continue; }
            pc = 0x4643d2; continue;
            }
            0x4643d2 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4643dd; continue; }
            pc = 0x4643d7; continue;
            }
            0x4643d7 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4643e6; continue;
            }
            0x4643dd => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4643e3);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4643e6; continue;
            }
            0x4643e6 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_c_str(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x464400;
    loop {
        match pc {
            0x464400 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x46440c; continue; }
            pc = 0x464407; continue;
            }
            0x464407 => {
            c.set(0,32,0x66e484u32);
            pc = 0x46440c; continue;
            }
            0x46440c => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_grow(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x464410;
    loop {
        match pc {
            0x464410 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(7,32); c.sub(a,0xfffffffdu32,32); }
            c.set(6,32,c.get(1,32));
            if c.cond(6) { pc = 0x464423; continue; }
            pc = 0x46441e; continue;
            }
            0x46441e => {
            c.push(m, 0x464423);
            { let args = []; c.r[0] = host.call(m, 0x64b1e5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x464423; continue;
            }
            0x464423 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x464456; continue; }
            pc = 0x46442c; continue;
            }
            0x46442c => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x464456; continue; }
            pc = 0x464433; continue;
            }
            0x464433 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x464456; continue; }
            pc = 0x464438; continue;
            }
            0x464438 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x464518; continue; }
            pc = 0x464440; continue;
            }
            0x464440 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x464456 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x14),8));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4644b3; continue; }
            pc = 0x46445e; continue;
            }
            0x46445e => {
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4644a2; continue; }
            pc = 0x464462; continue;
            }
            0x464462 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x464491; continue; }
            pc = 0x464466; continue;
            }
            0x464466 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x464488; continue; }
            pc = 0x46446e; continue;
            }
            0x46446e => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x464488; continue; }
            pc = 0x464473; continue;
            }
            0x464473 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x464488 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x46448e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x464491; continue;
            }
            0x464491 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4644a2 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4644ab; continue; }
            pc = 0x4644a6; continue;
            }
            0x4644a6 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            pc = 0x4644ab; continue;
            }
            0x4644ab => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4644b3 => {
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x464513; continue; }
            pc = 0x4644b7; continue;
            }
            0x4644b7 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); c.sub(a,0x1fu32,32); }
            if c.cond(7) { pc = 0x4644c3; continue; }
            pc = 0x4644bf; continue;
            }
            0x4644bf => {
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x464520; continue; }
            pc = 0x4644c3; continue;
            }
            0x4644c3 => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4644fa; continue; }
            pc = 0x4644c7; continue;
            }
            0x4644c7 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4644f1; continue; }
            pc = 0x4644cf; continue;
            }
            0x4644cf => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4644f1; continue; }
            pc = 0x4644d4; continue;
            }
            0x4644d4 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4644e9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4644f1 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4644f7);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4644fa; continue;
            }
            0x4644fa => {
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x46450b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x464513 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x464520; continue; }
            pc = 0x464518; continue;
            }
            0x464518 => {
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x464520);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x464520; continue;
            }
            0x464520 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn get_cdrom_drive_letters(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x464630;
    loop {
        match pc {
            0x464630 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,0x66a2a4u32,32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            { let a = c.r[0]; wm(m,a,32,0x0u32); }
            { let target = c.get(7,32); c.push(m, 0x46464a); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(6,32,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x464652);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(3,32,c.get(0,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let target = c.get(7,32); c.push(m, 0x46465d); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(7,32,rm(m,0x66a2a8u32,32));
            c.set(6,32,c.get(3,32));
            pc = 0x464665; continue;
            }
            0x464665 => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,0x41u32,8); }
            if c.cond(4) { pc = 0x464685; continue; }
            pc = 0x46466b; continue;
            }
            0x46466b => {
            { let a = c.get(0,8); c.sub(a,0x42u32,8); }
            if c.cond(4) { pc = 0x464685; continue; }
            pc = 0x46466f; continue;
            }
            0x46466f => {
            { let a = c.get(0,8); c.sub(a,0x61u32,8); }
            if c.cond(4) { pc = 0x464685; continue; }
            pc = 0x464673; continue;
            }
            0x464673 => {
            { let a = c.get(0,8); c.sub(a,0x62u32,8); }
            if c.cond(4) { pc = 0x464685; continue; }
            pc = 0x464677; continue;
            }
            0x464677 => {
            c.push(m,c.get(6,32));
            { let target = c.get(7,32); c.push(m, 0x46467a); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            if c.cond(5) { pc = 0x464685; continue; }
            pc = 0x46467f; continue;
            }
            0x46467f => {
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.r[3].wrapping_add(c.r[5]); wm(m,a,8,c.get(1,8)); }
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            pc = 0x464685; continue;
            }
            0x464685 => {
            { let a = rm(m,c.r[6],8); c.sub(a,0x0u32,8); }
            if c.cond(4) { pc = 0x464692; continue; }
            pc = 0x46468a; continue;
            }
            0x46468a => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x46468a; continue; }
            pc = 0x464692; continue;
            }
            0x464692 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x464665; continue; }
            pc = 0x46469a; continue;
            }
            0x46469a => {
            c.push(m,c.get(5,32));
            c.push(m, 0x4646a0);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4646ae);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(3,32));
            c.push(m, 0x4646b4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,c.get(5,32));
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

pub fn swap_u16_array_entries(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x477be0;
    loop {
        match pc {
            0x477be0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[1].wrapping_mul(2)),16));
            c.push(m,c.get(7,32));
            c.set(7,16,rm(m,c.r[0].wrapping_add(c.r[6].wrapping_mul(2)),16));
            { let a = c.r[0].wrapping_add(c.r[1].wrapping_mul(2)); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[0].wrapping_add(c.r[6].wrapping_mul(2)); wm(m,a,16,c.get(2,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn remove_u16_from_array(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x48fdf0;
    loop {
        match pc {
            0x48fdf0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x48fe03; continue; }
            pc = 0x48fdfc; continue;
            }
            0x48fdfc => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x48fe03 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x48fe22; continue; }
            pc = 0x48fe12; continue;
            }
            0x48fe12 => {
            c.set(1,16,rm(m,c.r[4].wrapping_add(0x18),16));
            pc = 0x48fe17; continue;
            }
            0x48fe17 => {
            { let a = rm(m,c.r[7].wrapping_add(c.r[6].wrapping_mul(2)),16); c.sub(a,c.get(1,16),16); }
            if c.cond(4) { pc = 0x48fe29; continue; }
            pc = 0x48fe1d; continue;
            }
            0x48fe1d => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x48fe17; continue; }
            pc = 0x48fe22; continue;
            }
            0x48fe22 => {
            c.set(0,32,c.get(7,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x48fe29 => {
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            if c.cond(14) { pc = 0x48fe3e; continue; }
            pc = 0x48fe31; continue;
            }
            0x48fe31 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x48fe39);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(3,32,c.get(0,32));
            pc = 0x48fe3e; continue;
            }
            0x48fe3e => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x48fe50; continue; }
            pc = 0x48fe42; continue;
            }
            0x48fe42 => {
            c.set(1,32,c.r[6].wrapping_add(c.r[6]));
            c.push(m,c.get(1,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x48fe4d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48fe50; continue;
            }
            0x48fe50 => {
            c.set(0,32,rm(m,c.r[5],32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x48fe6e; continue; }
            pc = 0x48fe59; continue;
            }
            0x48fe59 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0]));
            c.set(0,32,c.r[7].wrapping_add(c.r[6].wrapping_mul(2)).wrapping_add(0x2));
            c.push(m,c.get(2,32));
            c.set(1,32,c.r[3].wrapping_add(c.r[6].wrapping_mul(2)));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x48fe6b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x48fe6e; continue;
            }
            0x48fe6e => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_finder_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4aebc0;
    loop {
        match pc {
            0x4aebc0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6616f1u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(6,32)); }
            c.set(7,32,c.r[6].wrapping_add(0x4));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4aebe9);
            { let args = []; c.r[0] = host.call(m, 0x64e7c9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.set(3,32,c.r[6].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[6].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1u32); }
            c.push(m, 0x4aec08);
            { let args = []; c.r[0] = host.call(m, 0x638f07, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xffffffffu32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,0x2u32); }
            { let a = c.r[6]; wm(m,a,32,0x67041cu32); }
            c.push(m, 0x4aec1e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e85c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.get(3,32));
            c.push(m, 0x4aec25);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
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

pub fn archive_file_finder_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4aec40;
    loop {
        match pc {
            0x4aec40 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66171cu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x67041cu32); }
            c.set(7,32,c.r[6].wrapping_add(0x4));
            c.push(m,0xffffffffu32);
            c.push(m,0x0u32);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,0x2u32); }
            c.push(m, 0x4aec7b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e85c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,c.r[6].wrapping_add(0x18));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4aec85);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[6].wrapping_add(0x20));
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x1u32); }
            c.push(m, 0x4aec99);
            { let args = []; c.r[0] = host.call(m, 0x638f44, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(5,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,0x0u32); }
            c.push(m, 0x4aeca5);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4aecb4);
            { let args = []; c.r[0] = host.call(m, 0x64e7fc, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_finder_find_in_archive(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4aecd0;
    loop {
        match pc {
            0x4aecd0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x66172eu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x0u32);
            c.set(1,32,c.r[7].wrapping_add(0x4));
            c.push(m, 0x4aecf6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e85c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(5,32,c.r[7].wrapping_add(0x18));
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4aed00);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,c.get(5,32));
            c.push(m,c.get(0,32));
            { let a = c.r[7].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            c.push(m, 0x4aed13);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6dccc8u32,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4aee27; continue; }
            pc = 0x4aed20; continue;
            }
            0x4aed20 => {
            c.push(m,c.get(6,32));
            c.push(m,0x5cu32);
            c.push(m,0x2fu32);
            c.set(1,32,c.get(5,32));
            c.push(m, 0x4aed2c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ce30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4aed38);
            { let args = []; c.r[0] = host.call(m, 0x6389d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6c7cccu32,32));
            c.set(6,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,0x0u32); }
            c.push(m, 0x4aed5c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x6389ca, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m, 0x4aed69);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4aee05; continue; }
            pc = 0x4aed78; continue;
            }
            0x4aed78 => {
            c.push(m,c.get(3,32));
            pc = 0x4aed79; continue;
            }
            0x4aed79 => {
            c.set(1,32,rm(m,c.r[5],32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            { let target = rm(m,0x66a2e8u32,32); c.push(m, 0x4aed84); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4aedd8; continue; }
            pc = 0x4aed88; continue;
            }
            0x4aed88 => {
            c.set(3,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(14) { pc = 0x4aedc3; continue; }
            pc = 0x4aed91; continue;
            }
            0x4aed91 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(0,32,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[6].wrapping_mul(4)),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4aeda5);
            { let args = []; c.r[0] = host.call(m, 0x63ac5b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4aedb3; continue; }
            pc = 0x4aedac; continue;
            }
            0x4aedac => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(12) { pc = 0x4aed91; continue; }
            pc = 0x4aedb1; continue;
            }
            0x4aedb1 => {
            pc = 0x4aedc3; continue;
            }
            0x4aedb3 => {
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,0x1u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[7].wrapping_add(0x4));
            c.push(m, 0x4aedc3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x64ea47, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            pc = 0x4aedc3; continue;
            }
            0x4aedc3 => {
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4aedd8; continue; }
            pc = 0x4aedc7; continue;
            }
            0x4aedc7 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(1,32,c.r[7].wrapping_add(0x4));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4aedd8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e9a5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4aedd8; continue;
            }
            0x4aedd8 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(0,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4aede8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389ca, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m, 0x4aedf5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4aed79; continue; }
            pc = 0x4aee04; continue;
            }
            0x4aee04 => {
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x4aee05; continue;
            }
            0x4aee05 => {
            c.set(1,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4aee12);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389c4, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4aee26);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x4aee27; continue;
            }
            0x4aee27 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,8,u32::from(c.cond(15)));
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_finder_find_next(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4aee50;
    loop {
        match pc {
            0x4aee50 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4aee60; continue; }
            pc = 0x4aee57; continue;
            }
            0x4aee57 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.add(a,0x20u32,32); c.set(1,32,r); }
            { let target = rm(m,c.r[0].wrapping_add(0x44),32); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4aee60 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.r[1].wrapping_add(0x1c); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            c.set(2,8,u32::from(c.cond(15)));
            c.set(0,32,c.get(2,32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_finder_clear(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4aee80;
    loop {
        match pc {
            0x4aee80 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x0u32);
            c.set(1,32,c.r[6].wrapping_add(0x4));
            c.push(m, 0x4aee8f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64e85c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[6].wrapping_add(0x18));
            c.push(m, 0x4aee97);
            { let args = []; c.r[0] = host.call(m, 0x64f1de, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_finder_get_file_name(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4aeea0;
    loop {
        match pc {
            0x4aeea0 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4aeec6; continue; }
            pc = 0x4aeeb5; continue;
            }
            0x4aeeb5 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.add(a,0x20u32,32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x4aeebf); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4aeec6 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4aeed4);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4aefe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_finder_get_file_title(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4aeee0;
    loop {
        match pc {
            0x4aeee0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661757u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(6,32));
            if c.cond(5) { pc = 0x4aef2a; continue; }
            pc = 0x4aef08; continue;
            }
            0x4aef08 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.add(a,0x20u32,32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            { let target = rm(m,c.r[0].wrapping_add(0x14),32); c.push(m, 0x4aef16); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4aef2a => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x1c),32));
            c.set(0,32,c.r[4].wrapping_add(0x4));
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4aef3c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4aefe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x2eu32);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x1u32); }
            c.push(m, 0x4aef4f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64d360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4aef68; continue; }
            pc = 0x4aef54; continue;
            }
            0x4aef54 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0xfffffff8),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x4aef68);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64cc80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4aef68; continue;
            }
            0x4aef68 => {
            c.push(m,0x5cu32);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x4aef73);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64d360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4aef85; continue; }
            pc = 0x4aef78; continue;
            }
            0x4aef78 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x4aef85);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64cc80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4aef85; continue;
            }
            0x4aef85 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4aef95);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            c.push(m, 0x4aefab);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn archive_file_finder_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4aefc0;
    loop {
        match pc {
            0x4aefc0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4aefc8);
            { let args = []; c.r[0] = host.call(m, 0x4aec40, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4aefd8; continue; }
            pc = 0x4aefcf; continue;
            }
            0x4aefcf => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4aefd5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4aefd8; continue;
            }
            0x4aefd8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn string_array_get_at_wrapper(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4aefe0;
    loop {
        match pc {
            0x4aefe0 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            c.push(m, 0x4aeff9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4af000, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn string_array_get_at(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4af000;
    loop {
        match pc {
            0x4af000 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            c.push(m, 0x4af020);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn strtok_with_quotes(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b2b00;
    loop {
        match pc {
            0x4b2b00 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4b2b1a; continue; }
            pc = 0x4b2b09; continue;
            }
            0x4b2b09 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            { let a = 0x6d95ccu32; wm(m,a,32,c.get(6,32)); }
            { let a = 0x6d95c8u32; wm(m,a,8,c.get(0,8)); }
            pc = 0x4b2b20; continue;
            }
            0x4b2b1a => {
            c.set(6,32,rm(m,0x6d95ccu32,32));
            pc = 0x4b2b20; continue;
            }
            0x4b2b20 => {
            { let a = c.get(6,32); c.and(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4b2b28; continue; }
            pc = 0x4b2b24; continue;
            }
            0x4b2b24 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4b2b28 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.push(m,c.get(7,32));
            c.set(1,8,rm(m,c.r[0],8));
            c.set(2,32,c.get(0,32));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(7,32,c.get(0,32));
            if c.cond(4) { pc = 0x4b2b6a; continue; }
            pc = 0x4b2b3a; continue;
            }
            0x4b2b3a => {
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4b2b6a; continue; }
            pc = 0x4b2b40; continue;
            }
            0x4b2b40 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4b2b6a; continue; }
            pc = 0x4b2b44; continue;
            }
            0x4b2b44 => {
            c.set(0,8,rm(m,c.r[2],8));
            { let a = c.get(1,8); c.sub(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4b2b4d; continue; }
            pc = 0x4b2b4a; continue;
            }
            0x4b2b4a => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x4b2b66; continue;
            }
            0x4b2b4d => {
            { let a = c.get(1,8); c.sub(a,0x22u32,8); }
            if c.cond(5) { pc = 0x4b2b5c; continue; }
            pc = 0x4b2b52; continue;
            }
            0x4b2b52 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b2b5c; continue; }
            pc = 0x4b2b5a; continue;
            }
            0x4b2b5a => {
            c.set(3,8,0x1u32);
            pc = 0x4b2b5c; continue;
            }
            0x4b2b5c => {
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4b2b3a; continue; }
            pc = 0x4b2b64; continue;
            }
            0x4b2b64 => {
            c.set(3,8,0x1u32);
            pc = 0x4b2b66; continue;
            }
            0x4b2b66 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4b2b3a; continue; }
            pc = 0x4b2b6a; continue;
            }
            0x4b2b6a => {
            { let a = rm(m,c.r[6],8); c.sub(a,0x22u32,8); }
            if c.cond(5) { pc = 0x4b2b7d; continue; }
            pc = 0x4b2b6f; continue;
            }
            0x4b2b6f => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4b2b7d; continue; }
            pc = 0x4b2b77; continue;
            }
            0x4b2b77 => {
            c.set(7,32,0x6bb63cu32);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x4b2b7d; continue;
            }
            0x4b2b7d => {
            c.set(1,8,rm(m,c.r[7],8));
            c.set(0,32,c.get(6,32));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            c.set(2,32,c.get(7,32));
            if c.cond(4) { pc = 0x4b2bab; continue; }
            pc = 0x4b2b87; continue;
            }
            0x4b2b87 => {
            c.set(1,8,rm(m,c.r[6],8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4b2bb0; continue; }
            pc = 0x4b2b8d; continue;
            }
            0x4b2b8d => {
            { let a = rm(m,c.r[2],8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4b2bab; continue; }
            pc = 0x4b2b91; continue;
            }
            0x4b2b91 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(0x18),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4b2b9e; continue; }
            pc = 0x4b2b99; continue;
            }
            0x4b2b99 => {
            { let a = c.get(1,8); c.sub(a,0x22u32,8); }
            if c.cond(4) { pc = 0x4b2bab; continue; }
            pc = 0x4b2b9e; continue;
            }
            0x4b2b9e => {
            c.set(1,8,rm(m,c.r[2].wrapping_add(0x1),8));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4b2b87; continue; }
            pc = 0x4b2ba6; continue;
            }
            0x4b2ba6 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.set(2,32,c.get(7,32));
            pc = 0x4b2b87; continue;
            }
            0x4b2bab => {
            { let a = rm(m,c.r[6],8); c.sub(a,0x0u32,8); }
            if c.cond(5) { pc = 0x4b2bbe; continue; }
            pc = 0x4b2bb0; continue;
            }
            0x4b2bb0 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x6d95ccu32; wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4b2bbe => {
            { let a = c.r[6]; wm(m,a,8,0x0u32); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = 0x6d95ccu32; wm(m,a,32,c.get(6,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn get_edge_color_from_packed_style(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bbd30;
    loop {
        match pc {
            0x4bbd30 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xf00u32,32); }
            if c.cond(15) { pc = 0x4bbd81; continue; }
            pc = 0x4bbd40; continue;
            }
            0x4bbd40 => {
            if c.cond(4) { pc = 0x4bbd6f; continue; }
            pc = 0x4bbd42; continue;
            }
            0x4bbd42 => {
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            if c.cond(4) { pc = 0x4bbd60; continue; }
            pc = 0x4bbd47; continue;
            }
            0x4bbd47 => {
            { let a = c.get(0,32); c.sub(a,0xf0u32,32); }
            if c.cond(5) { pc = 0x4bbd88; continue; }
            pc = 0x4bbd4e; continue;
            }
            0x4bbd4e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dcd08),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bbd60 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dcd08),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bbd6f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.and(a,0xfu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dcd08),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bbd81 => {
            { let a = c.get(0,32); c.sub(a,0xf000u32,32); }
            if c.cond(4) { pc = 0x4bbd92; continue; }
            pc = 0x4bbd88; continue;
            }
            0x4bbd88 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dcd08),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bbd92 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let n = 0xcu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6dcd08),32));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn set_edge_color_index_in_packed_style(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bbdb0;
    loop {
        match pc {
            0x4bbdb0 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x8),8));
            { let a = c.get(0,8); let r = c.and(a,0x7u32,8); c.set(0,8,r); }
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xf00u32,32); }
            if c.cond(15) { pc = 0x4bbe11; continue; }
            pc = 0x4bbdca; continue;
            }
            0x4bbdca => {
            if c.cond(4) { pc = 0x4bbe01; continue; }
            pc = 0x4bbdcc; continue;
            }
            0x4bbdcc => {
            { let a = c.get(0,32); c.sub(a,0xfu32,32); }
            if c.cond(4) { pc = 0x4bbdee; continue; }
            pc = 0x4bbdd1; continue;
            }
            0x4bbdd1 => {
            { let a = c.get(0,32); c.sub(a,0xf0u32,32); }
            if c.cond(5) { pc = 0x4bbe18; continue; }
            pc = 0x4bbdd8; continue;
            }
            0x4bbdd8 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            { let a = c.get(0,8); let r = c.and(a,0xfu32,8); c.set(0,8,r); }
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bbdee => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,8); let r = c.and(a,0xf0u32,8); c.set(0,8,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bbe01 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,8,rm(m,c.r[4].wrapping_add(0x8),8));
            { let a = c.get(4,8); let r = c.and(a,0xf0u32,8); c.set(4,8,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(1,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bbe11 => {
            { let a = c.get(0,32); c.sub(a,0xf000u32,32); }
            if c.cond(4) { pc = 0x4bbe1e; continue; }
            pc = 0x4bbe18; continue;
            }
            0x4bbe18 => {
            c.set(0,16,rm(m,c.r[4].wrapping_add(0xc),16));
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bbe1e => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(4,8); let r = c.and(a,0xfu32,8); c.set(4,8,r); }
            { let a = c.get(2,32); let n = 0xcu32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn read_ini_int_triple(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bd0e0;
    loop {
        match pc {
            0x4bd0e0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,0x1000u32);
            c.push(m,0x6dbb58u32);
            c.push(m,0x6dcd1au32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd524u32);
            c.push(m, 0x4bd10b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x4be060, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bd115; continue; }
            pc = 0x4bd10f; continue;
            }
            0x4bd10f => {
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4bd115 => {
            c.set(0,32,c.r[6].wrapping_add(0x8));
            c.set(1,32,c.r[6].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,0x6bee10u32);
            c.push(m,0x6dbb58u32);
            c.push(m, 0x4bd12d);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,8,0x1u32);
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ini_file_cache_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bde30;
    loop {
        match pc {
            0x4bde30 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661dc6u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x3),8));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0xf),8));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,0x30u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x5); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4bde6c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(5,32,c.get(7,32));
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[7].wrapping_add(0x2c); wm(m,a,32,0x1u32); }
            c.push(m, 0x4bde86);
            { let args = []; c.r[0] = host.call(m, 0x64b52e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,0x6dd564u32,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4bdea1; continue; }
            pc = 0x4bde8e; continue;
            }
            0x4bde8e => {
            { let a = 0x6dd564u32; wm(m,a,32,c.get(7,32)); }
            { let a = c.r[7]; wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,0x6dd564u32,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            pc = 0x4bdea1; continue;
            }
            0x4bdea1 => {
            c.set(1,32,rm(m,0x6dd560u32,32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = 0x6dd560u32; wm(m,a,32,c.get(1,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4bdeb7);
            { let args = []; c.r[0] = host.call(m, 0x64b5ca, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4bdec4; continue; }
            pc = 0x4bdebb; continue;
            }
            0x4bdebb => {
            c.push(m,c.get(5,32));
            c.push(m, 0x4bdec1);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bdec4; continue;
            }
            0x4bdec4 => {
            c.set(7,32,rm(m,0x6dd564u32,32));
            c.push(m,0x30u32);
            c.push(m, 0x4bded1);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.push(m,0xau32);
            c.set(1,32,c.r[6].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4bdef6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64eae9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[6]; wm(m,a,32,0x670f8cu32); }
            { let a = c.r[6].wrapping_add(0x30); wm(m,a,8,0x1u32); }
            c.set(0,32,c.get(6,32));
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

pub fn ini_file_cache_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bdf20;
    loop {
        match pc {
            0x4bdf20 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661ddbu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x10u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[6]; wm(m,a,32,0x670f8cu32); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(1,32,c.r[6].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4bdf56);
            { let args = []; c.r[0] = host.call(m, 0x64eb5d, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x10),32));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,0xffffffffu32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4bdfd1; continue; }
            pc = 0x4bdf71; continue;
            }
            0x4bdf71 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4bdfd1; continue; }
            pc = 0x4bdf77; continue;
            }
            0x4bdf77 => {
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(0,32,rm(m,0x6dd564u32,32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            c.set(7,32,c.get(3,32));
            if c.cond(4) { pc = 0x4bdfaf; continue; }
            pc = 0x4bdf85; continue;
            }
            0x4bdf85 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bdf90);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c0050, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[7],32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[3].wrapping_add(0xc));
            c.push(m, 0x4bdf9b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c08a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.push(m, 0x4bdfa1);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dd564u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            c.set(3,32,c.get(7,32));
            if c.cond(5) { pc = 0x4bdf85; continue; }
            pc = 0x4bdfaf; continue;
            }
            0x4bdfaf => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(2,32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[0]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4bdfcf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bdff5; continue;
            }
            0x4bdfd1 => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bdff5; continue; }
            pc = 0x4bdfd5; continue;
            }
            0x4bdfd5 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x4bdfe0);
            { let args = []; c.r[0] = host.call(m, 0x4c04d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[4].wrapping_add(0x1c));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bdfed);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bfc70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bdfd5; continue; }
            pc = 0x4bdff5; continue;
            }
            0x4bdff5 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bdffe);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m, 0x4be012);
            { let args = []; c.r[0] = host.call(m, 0x64b52e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dd560u32,32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = 0x6dd560u32; wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4be02b; continue; }
            pc = 0x4be01f; continue;
            }
            0x4be01f => {
            c.set(6,32,rm(m,0x6dd564u32,32));
            { let a = 0x6dd564u32; wm(m,a,32,c.get(5,32)); }
            pc = 0x4be02b; continue;
            }
            0x4be02b => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4be034);
            { let args = []; c.r[0] = host.call(m, 0x64b5ca, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4be041; continue; }
            pc = 0x4be038; continue;
            }
            0x4be038 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4be03e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4be041; continue;
            }
            0x4be041 => {
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

pub fn ini_file_cache_get_string(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4be060;
    loop {
        match pc {
            0x4be060 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4be07f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4be1e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4be090; continue; }
            pc = 0x4be083; continue;
            }
            0x4be083 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4be089);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x4be090 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4be09b);
            { let args = []; c.r[0] = host.call(m, 0x63add0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x4be0a1);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ini_file_cache_get_int(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4be0b0;
    loop {
        match pc {
            0x4be0b0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(4,32); let r = c.sub(a,0x204u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x210),32));
            c.push(m,0x200u32);
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x214),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4be0da);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x4be1e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4be0fd; continue; }
            pc = 0x4be0de; continue;
            }
            0x4be0de => {
            c.set(0,32,c.r[4]);
            c.set(1,32,c.r[4].wrapping_add(0x4));
            c.push(m,c.get(0,32));
            c.push(m,0x6bf250u32);
            c.push(m,c.get(1,32));
            c.push(m, 0x4be0f2);
            { let args = []; c.r[0] = host.call(m, 0x639d74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[4],32));
            if c.cond(15) { pc = 0x4be104; continue; }
            pc = 0x4be0fd; continue;
            }
            0x4be0fd => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x210),32));
            pc = 0x4be104; continue;
            }
            0x4be104 => {
            { let a = c.get(4,32); let r = c.add(a,0x204u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ini_file_cache_read_value(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4be1e0;
    loop {
        match pc {
            0x4be1e0 => {
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x661e05u32);
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x24u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x30),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4be414; continue; }
            pc = 0x4be209; continue;
            }
            0x4be209 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0x0u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4be228);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4be231);
            { let args = []; c.r[0] = host.call(m, 0x64f709, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,c.r[7].wrapping_add(0x14));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4be242);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ec25, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4be2aa; continue; }
            pc = 0x4be246; continue;
            }
            0x4be246 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x30),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4be2aa; continue; }
            pc = 0x4be24d; continue;
            }
            0x4be24d => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x58));
            c.push(m, 0x4be269);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x54));
            c.push(m, 0x4be272);
            { let args = []; c.r[0] = host.call(m, 0x64f709, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4be280);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ec25, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4be29c; continue; }
            pc = 0x4be284; continue;
            }
            0x4be284 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4be290);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4be680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4be29c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ebf1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4be29c; continue;
            }
            0x4be29c => {
            c.set(1,32,c.r[4].wrapping_add(0x54));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x0u32); }
            c.push(m, 0x4be2aa);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4be2aa; continue;
            }
            0x4be2aa => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x50),8));
            c.push(m,0x0u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x4be2bd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,8,0x2u32); }
            c.push(m, 0x4be2d0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4be2de);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f5e3, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4be2e7);
            { let args = []; c.r[0] = host.call(m, 0x64f709, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(0,32));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x4be2f3);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4be304);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4be326; continue; }
            pc = 0x4be308; continue;
            }
            0x4be308 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4be314);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,c.get(3,8)); }
            pc = 0x4be328; continue;
            }
            0x4be326 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x4be328; continue;
            }
            0x4be328 => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.set(6,32,c.r[7].wrapping_add(0x4));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4be337);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c0310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4be359; continue; }
            pc = 0x4be342; continue;
            }
            0x4be342 => {
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m, 0x4be34f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c08c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4be359; continue; }
            pc = 0x4be353; continue;
            }
            0x4be353 => {
            c.set(0,32,c.r[4].wrapping_add(0x44));
            pc = 0x4be364; continue;
            }
            0x4be359 => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,c.r[4].wrapping_add(0x48));
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(6,32)); }
            pc = 0x4be364; continue;
            }
            0x4be364 => {
            c.set(0,32,rm(m,c.r[0],32));
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4be3db; continue; }
            pc = 0x4be36d; continue;
            }
            0x4be36d => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x24),32));
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x4be37a; continue; }
            pc = 0x4be378; continue;
            }
            0x4be378 => {
            c.set(6,32,c.get(1,32));
            pc = 0x4be37a; continue;
            }
            0x4be37a => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x4be390; continue; }
            pc = 0x4be382; continue;
            }
            0x4be382 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x20),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4be38d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4be390; continue;
            }
            0x4be390 => {
            { let a = c.r[6].wrapping_add(c.r[7]); wm(m,a,8,c.get(3,8)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4be3b7; continue; }
            pc = 0x4be39b; continue;
            }
            0x4be39b => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4be3ae; continue; }
            pc = 0x4be3a3; continue;
            }
            0x4be3a3 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4be3ae; continue; }
            pc = 0x4be3a8; continue;
            }
            0x4be3a8 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4be3b7; continue;
            }
            0x4be3ae => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4be3b4);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4be3b7; continue;
            }
            0x4be3b7 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4be3d4);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,0x1u32);
            pc = 0x4be66b; continue;
            }
            0x4be3db => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4be3ff; continue; }
            pc = 0x4be3e3; continue;
            }
            0x4be3e3 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4be3f6; continue; }
            pc = 0x4be3eb; continue;
            }
            0x4be3eb => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4be3f6; continue; }
            pc = 0x4be3f0; continue;
            }
            0x4be3f0 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4be3ff; continue;
            }
            0x4be3f6 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4be3fc);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4be3ff; continue;
            }
            0x4be3ff => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            pc = 0x4be65c; continue;
            }
            0x4be414 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x54),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.push(m,c.get(0,32));
            c.push(m,0x6dd55au32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            { let target = rm(m,0x66a2c4u32,32); c.push(m, 0x4be438); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(28);
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4be445; continue; }
            pc = 0x4be43e; continue;
            }
            0x4be43e => {
            c.set(0,8,0x1u32);
            pc = 0x4be66b; continue;
            }
            0x4be445 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[4].wrapping_add(0x24));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0x3u32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4be464);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6dd55bu32);
            c.push(m,0x6bf254u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x4be477);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ce80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6dd55cu32);
            c.push(m,0x6bf258u32);
            c.set(1,32,c.r[4].wrapping_add(0x28));
            c.push(m, 0x4be48a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ce80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4be49e);
            { let args = []; c.r[0] = host.call(m, 0x4bc970, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(0,32));
            c.push(m, 0x4be4a4);
            { let args = []; c.r[0] = host.call(m, 0x6389ee, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4be658; continue; }
            pc = 0x4be4b3; continue;
            }
            0x4be4b3 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x0u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(5,32)); }
            pc = 0x4be4c5; continue;
            }
            0x4be4c5 => {
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x4be641; continue; }
            pc = 0x4be4cd; continue;
            }
            0x4be4cd => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(c.r[5]),8));
            { let a = c.get(0,8); c.sub(a,0x5bu32,8); }
            if c.cond(5) { pc = 0x4be565; continue; }
            pc = 0x4be4d8; continue;
            }
            0x4be4d8 => {
            c.set(1,32,c.r[3].wrapping_add(0xffffffff));
            { let a = c.get(5,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4be565; continue; }
            pc = 0x4be4e3; continue;
            }
            0x4be4e3 => {
            c.set(6,32,c.r[5].wrapping_add(0x1));
            pc = 0x4be4e6; continue;
            }
            0x4be4e6 => {
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x4be4f0; continue; }
            pc = 0x4be4ea; continue;
            }
            0x4be4ea => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[7]),8); c.sub(a,0x20u32,8); }
            if c.cond(4) { pc = 0x4be4f6; continue; }
            pc = 0x4be4f0; continue;
            }
            0x4be4f0 => {
            { let a = rm(m,c.r[6].wrapping_add(c.r[7]),8); c.sub(a,0x9u32,8); }
            if c.cond(5) { pc = 0x4be4f9; continue; }
            pc = 0x4be4f6; continue;
            }
            0x4be4f6 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x4be4e6; continue;
            }
            0x4be4f9 => {
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            c.set(1,32,c.get(6,32));
            if c.cond(3) { pc = 0x4be517; continue; }
            pc = 0x4be4ff; continue;
            }
            0x4be4ff => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(c.r[7]),8));
            { let a = c.get(0,8); c.sub(a,0x5du32,8); }
            if c.cond(4) { pc = 0x4be517; continue; }
            pc = 0x4be506; continue;
            }
            0x4be506 => {
            { let a = c.get(0,8); c.sub(a,0x20u32,8); }
            if c.cond(4) { pc = 0x4be517; continue; }
            pc = 0x4be50a; continue;
            }
            0x4be50a => {
            { let a = c.get(0,8); c.sub(a,0x9u32,8); }
            if c.cond(4) { pc = 0x4be517; continue; }
            pc = 0x4be50e; continue;
            }
            0x4be50e => {
            { let a = c.get(0,8); c.sub(a,0xdu32,8); }
            if c.cond(4) { pc = 0x4be517; continue; }
            pc = 0x4be512; continue;
            }
            0x4be512 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x4be4ff; continue; }
            pc = 0x4be517; continue;
            }
            0x4be517 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x44),32));
            c.set(7,32,c.get(1,32));
            c.push(m,c.get(3,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(6,32),32); c.set(7,32,r); }
            c.push(m, 0x4be525);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4be553; continue; }
            pc = 0x4be52c; continue;
            }
            0x4be52c => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(7,32));
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4be53a);
            { let args = []; c.r[0] = host.call(m, 0x648dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4be553; continue; }
            pc = 0x4be541; continue;
            }
            0x4be541 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x1u32); }
            pc = 0x4be634; continue;
            }
            0x4be553 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,8,0x0u32); }
            pc = 0x4be634; continue;
            }
            0x4be565 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x54),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4be620; continue; }
            pc = 0x4be571; continue;
            }
            0x4be571 => {
            { let a = c.get(0,8); c.sub(a,0x3du32,8); }
            if c.cond(5) { pc = 0x4be620; continue; }
            pc = 0x4be579; continue;
            }
            0x4be579 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x48),32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4be583);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4be595);
            { let args = []; c.r[0] = host.call(m, 0x648dd0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4be620; continue; }
            pc = 0x4be5a4; continue;
            }
            0x4be5a4 => {
            c.set(2,32,c.r[3].wrapping_add(0xffffffff));
            { let a = c.get(5,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4be620; continue; }
            pc = 0x4be5ab; continue;
            }
            0x4be5ab => {
            c.set(6,32,c.r[7].wrapping_add(c.r[5]).wrapping_add(0x1));
            pc = 0x4be5af; continue;
            }
            0x4be5af => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,0x20u32,8); }
            if c.cond(4) { pc = 0x4be5b9; continue; }
            pc = 0x4be5b5; continue;
            }
            0x4be5b5 => {
            { let a = c.get(0,8); c.sub(a,0x9u32,8); }
            if c.cond(5) { pc = 0x4be5bc; continue; }
            pc = 0x4be5b9; continue;
            }
            0x4be5b9 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x4be5af; continue;
            }
            0x4be5bc => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(0,32,c.r[5].wrapping_add(0x1));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(5,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4be5ed; continue; }
            pc = 0x4be5cc; continue;
            }
            0x4be5cc => {
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x4be5ed; continue; }
            pc = 0x4be5d0; continue;
            }
            0x4be5d0 => {
            c.set(2,8,rm(m,c.r[6],8));
            { let a = c.get(2,8); c.sub(a,0xdu32,8); }
            if c.cond(4) { pc = 0x4be5ed; continue; }
            pc = 0x4be5d7; continue;
            }
            0x4be5d7 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(c.r[3]); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x50),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4be5cc; continue; }
            pc = 0x4be5ed; continue;
            }
            0x4be5ed => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[2].wrapping_add(c.r[1]).wrapping_add(0xffffffff); wm(m,a,8,0x0u32); }
            c.set(2,32,c.r[0].wrapping_add(c.r[1]));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[0].wrapping_add(c.r[1]).wrapping_add(0xffffffff));
            pc = 0x4be604; continue;
            }
            0x4be604 => {
            c.set(0,8,rm(m,c.r[1],8));
            { let a = c.get(0,8); c.sub(a,0xdu32,8); }
            if c.cond(4) { pc = 0x4be612; continue; }
            pc = 0x4be60a; continue;
            }
            0x4be60a => {
            { let a = c.get(0,8); c.sub(a,0x20u32,8); }
            if c.cond(4) { pc = 0x4be612; continue; }
            pc = 0x4be60e; continue;
            }
            0x4be60e => {
            { let a = c.get(0,8); c.sub(a,0x9u32,8); }
            if c.cond(5) { pc = 0x4be619; continue; }
            pc = 0x4be612; continue;
            }
            0x4be612 => {
            { let a = c.r[2]; wm(m,a,8,0x0u32); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            { let a = c.get(2,32); let r = c.dec(a,32); c.set(2,32,r); }
            pc = 0x4be604; continue;
            }
            0x4be619 => {
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            pc = 0x4be634; continue;
            }
            0x4be620 => {
            { let a = rm(m,c.r[7].wrapping_add(c.r[5]),8); c.sub(a,0xdu32,8); }
            if c.cond(5) { pc = 0x4be634; continue; }
            pc = 0x4be626; continue;
            }
            0x4be626 => {
            c.set(0,32,c.r[3].wrapping_add(0xfffffffe));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x4be634; continue; }
            pc = 0x4be62d; continue;
            }
            0x4be62d => {
            c.set(1,32,c.r[5].wrapping_add(0x2));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            pc = 0x4be634; continue;
            }
            0x4be634 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4be4c5; continue; }
            pc = 0x4be641; continue;
            }
            0x4be641 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4be652);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            pc = 0x4be66b; continue;
            }
            0x4be658 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            pc = 0x4be65c; continue;
            }
            0x4be65c => {
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4be669);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); let r = c.xor(a,c.get(0,8),8); c.set(0,8,r); }
            pc = 0x4be66b; continue;
            }
            0x4be66b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(24); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ini_file_cache_parse_file(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4be680;
    loop {
        match pc {
            0x4be680 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661e7du32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x4a8u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,0x6c7cccu32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x64); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x4c0),32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[4].wrapping_add(0x40));
            { let a = c.r[4].wrapping_add(0x4bc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4be6c4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6dd55du32);
            c.push(m,0x6bf25cu32);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x4be6d7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ce80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6dd55eu32);
            c.push(m,0x6bf260u32);
            c.set(1,32,c.r[4].wrapping_add(0x44));
            c.push(m, 0x4be6ea);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ce80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0xb),8));
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.r[4].wrapping_add(0x58));
            c.push(m, 0x4be6fc);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(6,32));
            c.push(m, 0x4be702);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x54));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4be710);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bfbc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(2,8,rm(m,c.r[4].wrapping_add(0xb),8));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x4bc); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,8,c.get(2,8)); }
            c.push(m, 0x4be72a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,8,rm(m,c.r[4].wrapping_add(0xb),8));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x4bc); wm(m,a,8,0x2u32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4be744);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0xb),8));
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.r[4].wrapping_add(0x30));
            { let a = c.r[4].wrapping_add(0x4bc); wm(m,a,8,0x3u32); }
            c.push(m, 0x4be75e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x6bf264u32);
            c.push(m,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x4c0); wm(m,a,8,0x4u32); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4be781);
            { let args = []; c.r[0] = host.call(m, 0x63bece, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x44); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4be792; continue; }
            pc = 0x4be78c; continue;
            }
            0x4be78c => {
            { let a = c.r[4].wrapping_add(0xb); wm(m,a,8,c.get(3,8)); }
            pc = 0x4be7bf; continue;
            }
            0x4be792 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.set(0,32,rm(m,0x6dccc8u32,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x13); wm(m,a,8,0x1u32); }
            c.push(m, 0x4be7a7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x6389a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x50); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4be7bf; continue; }
            pc = 0x4be7b2; continue;
            }
            0x4be7b2 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4be7b8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x6389b8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,c.get(0,32)); }
            pc = 0x4be7bf; continue;
            }
            0x4be7bf => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            pc = 0x4be7c1; continue;
            }
            0x4be7c1 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x13),8));
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4be9b6; continue; }
            pc = 0x4be7cd; continue;
            }
            0x4be7cd => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            { let a = rm(m,c.r[6].wrapping_add(0xc),8); c.and(a,0x10u32,8); }
            if c.cond(5) { pc = 0x4beeca; continue; }
            pc = 0x4be7db; continue;
            }
            0x4be7db => {
            c.push(m,0x400u32);
            c.push(m,c.get(3,32));
            c.push(m,0x6dd120u32);
            c.push(m, 0x4be7eb);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m,0x400u32);
            c.push(m,0x6dd120u32);
            c.push(m, 0x4be7fb);
            { let args = []; c.r[0] = host.call(m, 0x63be3b, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x4be7fe; continue;
            }
            0x4be7fe => {
            c.set(0,8,rm(m,0x6dd120u32,8));
            c.set(5,32,0x6dd120u32);
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4be7c1; continue; }
            pc = 0x4be80c; continue;
            }
            0x4be80c => {
            { let a = c.get(0,8); c.sub(a,0x20u32,8); }
            if c.cond(4) { pc = 0x4be814; continue; }
            pc = 0x4be810; continue;
            }
            0x4be810 => {
            { let a = c.get(0,8); c.sub(a,0x9u32,8); }
            if c.cond(5) { pc = 0x4be81c; continue; }
            pc = 0x4be814; continue;
            }
            0x4be814 => {
            c.set(0,8,rm(m,c.r[5].wrapping_add(0x1),8));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4be80c; continue; }
            pc = 0x4be81c; continue;
            }
            0x4be81c => {
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.sub(a,0x5bu32,8); }
            if c.cond(5) { pc = 0x4bea7d; continue; }
            pc = 0x4be827; continue;
            }
            0x4be827 => {
            c.set(1,8,rm(m,c.r[5],8));
            c.set(0,32,c.get(5,32));
            { let a = c.get(1,8); c.sub(a,0x5du32,8); }
            if c.cond(4) { pc = 0x4be83a; continue; }
            pc = 0x4be831; continue;
            }
            0x4be831 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,0x5du32,8); }
            if c.cond(5) { pc = 0x4be831; continue; }
            pc = 0x4be83a; continue;
            }
            0x4be83a => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            c.set(6,32,c.get(5,32));
            if c.cond(4) { pc = 0x4be85e; continue; }
            pc = 0x4be841; continue;
            }
            0x4be841 => {
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4be85e; continue; }
            pc = 0x4be848; continue;
            }
            0x4be848 => {
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4be851);
            { let args = []; c.r[0] = host.call(m, 0x63bf27, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4be848; continue; }
            pc = 0x4be85e; continue;
            }
            0x4be85e => {
            c.push(m,c.get(5,32));
            c.push(m, 0x4be864);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4be872);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bfbc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6bf268u32);
            c.push(m, 0x4be87c);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6bf268u32);
            c.push(m, 0x4be88f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4be8fe; continue; }
            pc = 0x4be894; continue;
            }
            0x4be894 => {
            c.push(m,0x6bf26cu32);
            c.push(m, 0x4be89e);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6bf26cu32);
            c.push(m, 0x4be8b1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.set(6,32,c.get(0,32));
            c.push(m, 0x4be8bc);
            { let args = []; c.r[0] = host.call(m, 0x4bf810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x4be8c6; continue; }
            pc = 0x4be8c4; continue;
            }
            0x4be8c4 => {
            c.set(6,32,c.get(0,32));
            pc = 0x4be8c6; continue;
            }
            0x4be8c6 => {
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x4be8fe; continue; }
            pc = 0x4be8ca; continue;
            }
            0x4be8ca => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[6]));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4be8db);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(6,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4be8ef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4be8fe; continue; }
            pc = 0x4be8f3; continue;
            }
            0x4be8f3 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[2].wrapping_add(c.r[7]); wm(m,a,8,c.get(3,8)); }
            pc = 0x4be8fe; continue;
            }
            0x4be8fe => {
            c.push(m,0x6bf270u32);
            c.push(m, 0x4be908);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6bf270u32);
            c.push(m, 0x4be91b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf6c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4be7c1; continue; }
            pc = 0x4be924; continue;
            }
            0x4be924 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.push(m,0x6bf274u32);
            c.push(m, 0x4be932);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6bf274u32);
            c.push(m, 0x4be945);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf6c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4be954; continue; }
            pc = 0x4be94f; continue;
            }
            0x4be94f => {
            c.push(m, 0x4be954);
            { let args = []; c.r[0] = host.call(m, 0x64b404, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4be954; continue;
            }
            0x4be954 => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m, 0x4be95d);
            { let args = []; c.r[0] = host.call(m, 0x4bf810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(7,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x4be96b; continue; }
            pc = 0x4be969; continue;
            }
            0x4be969 => {
            c.set(6,32,c.get(1,32));
            pc = 0x4be96b; continue;
            }
            0x4be96b => {
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x4be7c1; continue; }
            pc = 0x4be973; continue;
            }
            0x4be973 => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,c.r[0].wrapping_add(c.r[6]));
            { let a = c.get(0,32); let r = c.add(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4be98a);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(6,32),32); c.set(7,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4be99e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4be7c1; continue; }
            pc = 0x4be9a6; continue;
            }
            0x4be9a6 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[2].wrapping_add(c.r[7]); wm(m,a,8,c.get(3,8)); }
            pc = 0x4be7c1; continue;
            }
            0x4be9b6 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.get(5,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4beeba; continue; }
            pc = 0x4be9c2; continue;
            }
            0x4be9c2 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x50),32));
            { let a = c.get(1,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4beeba; continue; }
            pc = 0x4be9cf; continue;
            }
            0x4be9cf => {
            { let a = rm(m,c.r[4].wrapping_add(0x48),32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4beeba; continue; }
            pc = 0x4be9d9; continue;
            }
            0x4be9d9 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x54),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(13) { pc = 0x4bea02; continue; }
            pc = 0x4be9e1; continue;
            }
            0x4be9e1 => {
            c.set(6,32,0x400u32);
            c.set(7,32,c.r[4].wrapping_add(c.r[0]).wrapping_add(0xb8));
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4be9f7);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4be9ff);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24))]; c.r[0] = host.call(m, 0x6389ac, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            pc = 0x4bea02; continue;
            }
            0x4bea02 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x4bea04; continue;
            }
            0x4bea04 => {
            { let a = rm(m,c.r[4].wrapping_add(c.r[6]).wrapping_add(0xb8),8); c.sub(a,0xdu32,8); }
            if c.cond(4) { pc = 0x4bea19; continue; }
            pc = 0x4bea0e; continue;
            }
            0x4bea0e => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x400u32,32); }
            if c.cond(12) { pc = 0x4bea04; continue; }
            pc = 0x4bea17; continue;
            }
            0x4bea17 => {
            pc = 0x4bea20; continue;
            }
            0x4bea19 => {
            { let a = c.r[4].wrapping_add(c.r[6]).wrapping_add(0xb8); wm(m,a,8,c.get(3,8)); }
            pc = 0x4bea20; continue;
            }
            0x4bea20 => {
            c.set(1,32,c.r[4].wrapping_add(0xb8));
            c.push(m,0x400u32);
            c.push(m,c.get(1,32));
            c.push(m,0x6dd120u32);
            c.push(m, 0x4bea37);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.sub(a,0x3feu32,32); }
            if c.cond(13) { pc = 0x4bea62; continue; }
            pc = 0x4bea42; continue;
            }
            0x4bea42 => {
            c.set(2,32,0x3feu32);
            c.set(0,32,c.r[4].wrapping_add(c.r[6]).wrapping_add(0xba));
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0xb8));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bea5f);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            pc = 0x4bea62; continue;
            }
            0x4bea62 => {
            c.set(0,32,0x3feu32);
            c.set(2,32,0x400u32);
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x54); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[4].wrapping_add(0x48),32); let r = c.add(a,c.get(2,32),32); { let a = c.r[4].wrapping_add(0x48); wm(m,a,32,r); } }
            pc = 0x4be7fe; continue;
            }
            0x4bea7d => {
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4be7c1; continue; }
            pc = 0x4bea85; continue;
            }
            0x4bea85 => {
            c.push(m,0x6dd55fu32);
            c.set(6,32,c.get(5,32));
            c.push(m, 0x4bea91);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,0x6dd55fu32);
            c.push(m, 0x4beaa3);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bfbc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[5],8));
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4beabe; continue; }
            pc = 0x4beaaa; continue;
            }
            0x4beaaa => {
            { let a = c.get(0,8); c.sub(a,0x3du32,8); }
            if c.cond(4) { pc = 0x4beabe; continue; }
            pc = 0x4beaae; continue;
            }
            0x4beaae => {
            { let a = c.get(0,8); c.sub(a,0x3bu32,8); }
            if c.cond(4) { pc = 0x4be7c1; continue; }
            pc = 0x4beab6; continue;
            }
            0x4beab6 => {
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4beaaa; continue; }
            pc = 0x4beabe; continue;
            }
            0x4beabe => {
            c.set(0,8,rm(m,c.r[6],8));
            { let a = c.get(0,8); c.sub(a,0x3bu32,8); }
            if c.cond(4) { pc = 0x4be7c1; continue; }
            pc = 0x4beac8; continue;
            }
            0x4beac8 => {
            { let a = c.get(0,8); c.sub(a,0x3du32,8); }
            if c.cond(5) { pc = 0x4be7c1; continue; }
            pc = 0x4bead0; continue;
            }
            0x4bead0 => {
            { let a = c.r[6]; wm(m,a,8,c.get(3,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            c.set(0,32,c.get(6,32));
            if c.cond(4) { pc = 0x4beaf2; continue; }
            pc = 0x4beadc; continue;
            }
            0x4beadc => {
            c.set(1,8,rm(m,c.r[0],8));
            { let a = c.get(1,8); c.sub(a,0xau32,8); }
            if c.cond(4) { pc = 0x4beae8; continue; }
            pc = 0x4beae3; continue;
            }
            0x4beae3 => {
            { let a = c.get(1,8); c.sub(a,0xdu32,8); }
            if c.cond(5) { pc = 0x4beaea; continue; }
            pc = 0x4beae8; continue;
            }
            0x4beae8 => {
            { let a = c.r[0]; wm(m,a,8,c.get(3,8)); }
            pc = 0x4beaea; continue;
            }
            0x4beaea => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x1),8));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4beadc; continue; }
            pc = 0x4beaf2; continue;
            }
            0x4beaf2 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4beaf8);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4beb06);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bfbc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x6bf278u32);
            c.push(m, 0x4beb10);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6bf278u32);
            c.push(m, 0x4beb23);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4beb92; continue; }
            pc = 0x4beb28; continue;
            }
            0x4beb28 => {
            c.push(m,0x6bf27cu32);
            c.push(m, 0x4beb32);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6bf27cu32);
            c.push(m, 0x4beb45);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x4beb50);
            { let args = []; c.r[0] = host.call(m, 0x4bf810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4beb5a; continue; }
            pc = 0x4beb58; continue;
            }
            0x4beb58 => {
            c.set(7,32,c.get(0,32));
            pc = 0x4beb5a; continue;
            }
            0x4beb5a => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x4beb96; continue; }
            pc = 0x4beb5e; continue;
            }
            0x4beb5e => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[7]));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4beb6f);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4beb83);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4beb92; continue; }
            pc = 0x4beb87; continue;
            }
            0x4beb87 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,c.get(3,8)); }
            pc = 0x4beb92; continue;
            }
            0x4beb92 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            pc = 0x4beb96; continue;
            }
            0x4beb96 => {
            c.push(m,0x6bf280u32);
            c.set(6,32,c.get(0,32));
            c.push(m, 0x4beba2);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6bf280u32);
            c.push(m, 0x4bebb6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4bec40; continue; }
            pc = 0x4bebbf; continue;
            }
            0x4bebbf => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            c.push(m,0x6bf284u32);
            c.push(m, 0x4bebcd);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(0,32));
            c.push(m,0xffffffffu32);
            c.push(m,0x6bf284u32);
            c.push(m, 0x4bebe1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x4bebf1; continue; }
            pc = 0x4bebec; continue;
            }
            0x4bebec => {
            c.push(m, 0x4bebf1);
            { let args = []; c.r[0] = host.call(m, 0x64b404, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bebf1; continue;
            }
            0x4bebf1 => {
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m, 0x4bebfa);
            { let args = []; c.r[0] = host.call(m, 0x4bf810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x3c),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4bec06; continue; }
            pc = 0x4bec04; continue;
            }
            0x4bec04 => {
            c.set(7,32,c.get(0,32));
            pc = 0x4bec06; continue;
            }
            0x4bec06 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x4bec40; continue; }
            pc = 0x4bec0a; continue;
            }
            0x4bec0a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[7]));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bec1d);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x48),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4bec31);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bec40; continue; }
            pc = 0x4bec35; continue;
            }
            0x4bec35 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[2].wrapping_add(c.r[6]); wm(m,a,8,c.get(3,8)); }
            pc = 0x4bec40; continue;
            }
            0x4bec40 => {
            c.set(0,8,rm(m,c.r[5],8));
            c.set(6,32,c.get(5,32));
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bec5f; continue; }
            pc = 0x4bec49; continue;
            }
            0x4bec49 => {
            c.set(0,32,sx(c.get(0,8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4bec52);
            { let args = []; c.r[0] = host.call(m, 0x63bf27, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x1),8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(5) { pc = 0x4bec49; continue; }
            pc = 0x4bec5f; continue;
            }
            0x4bec5f => {
            c.push(m,c.get(5,32));
            c.push(m, 0x4bec65);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x4bec76);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bec93; continue; }
            pc = 0x4bec7a; continue;
            }
            0x4bec7a => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bec86);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(6,32));
            c.push(m, 0x4bec93);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf7f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bec93; continue;
            }
            0x4bec93 => {
            c.push(m,0x6bf288u32);
            c.push(m, 0x4bec9d);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6bf288u32);
            c.push(m, 0x4becb0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4bed1f; continue; }
            pc = 0x4becb5; continue;
            }
            0x4becb5 => {
            c.push(m,0x6bf28cu32);
            c.push(m, 0x4becbf);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6bf28cu32);
            c.push(m, 0x4becd2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.set(7,32,c.get(0,32));
            c.push(m, 0x4becdd);
            { let args = []; c.r[0] = host.call(m, 0x4bf810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4bece7; continue; }
            pc = 0x4bece5; continue;
            }
            0x4bece5 => {
            c.set(7,32,c.get(0,32));
            pc = 0x4bece7; continue;
            }
            0x4bece7 => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x4bed1f; continue; }
            pc = 0x4beceb; continue;
            }
            0x4beceb => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[7]));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4becfc);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4bed10);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bed1f; continue; }
            pc = 0x4bed14; continue;
            }
            0x4bed14 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[2].wrapping_add(c.r[6]); wm(m,a,8,c.get(3,8)); }
            pc = 0x4bed1f; continue;
            }
            0x4bed1f => {
            c.push(m,0x6bf290u32);
            c.push(m, 0x4bed29);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6bf290u32);
            c.push(m, 0x4bed3c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf6c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x4bedca; continue; }
            pc = 0x4bed45; continue;
            }
            0x4bed45 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,0x6bf294u32);
            c.push(m, 0x4bed53);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m,0x6bf294u32);
            c.push(m, 0x4bed66);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf6c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(6,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x4bed75; continue; }
            pc = 0x4bed70; continue;
            }
            0x4bed70 => {
            c.push(m, 0x4bed75);
            { let args = []; c.r[0] = host.call(m, 0x64b404, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bed75; continue;
            }
            0x4bed75 => {
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m, 0x4bed7e);
            { let args = []; c.r[0] = host.call(m, 0x4bf810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(6,32),32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4bed8c; continue; }
            pc = 0x4bed8a; continue;
            }
            0x4bed8a => {
            c.set(7,32,c.get(1,32));
            pc = 0x4bed8c; continue;
            }
            0x4bed8c => {
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(6) { pc = 0x4bedca; continue; }
            pc = 0x4bed90; continue;
            }
            0x4bed90 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[7]));
            { let a = c.get(2,32); let r = c.add(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4beda7);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(7,32),32); c.set(6,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4bedbb);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bedca; continue; }
            pc = 0x4bedbf; continue;
            }
            0x4bedbf => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[0].wrapping_add(c.r[6]); wm(m,a,8,c.get(3,8)); }
            pc = 0x4bedca; continue;
            }
            0x4bedca => {
            c.set(1,32,c.r[4].wrapping_add(0x24));
            c.set(2,32,c.r[4].wrapping_add(0x5c));
            c.push(m,c.get(1,32));
            c.set(0,32,c.r[4].wrapping_add(0xac));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bede1);
            { let args = []; c.r[0] = host.call(m, 0x4c0520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.set(2,32,c.r[4].wrapping_add(0xa4));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            { let a = c.r[4].wrapping_add(0x4d8); wm(m,a,8,0x5u32); }
            c.push(m, 0x4bedfc);
            { let args = []; c.r[0] = host.call(m, 0x4c0520, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x18u32,32); c.set(4,32,r); }
            c.set(6,32,c.get(0,32));
            c.set(0,8,rm(m,c.r[6],8));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[4].wrapping_add(0x74));
            { let a = c.r[4].wrapping_add(0x4c4); wm(m,a,8,0x6u32); }
            { let a = c.r[4].wrapping_add(0x74); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4bee19);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0xffffffffu32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[4].wrapping_add(0x7c));
            c.push(m, 0x4bee26);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x4c0); wm(m,a,8,0x7u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0x84));
            c.push(m, 0x4bee3f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf3a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x6c),32));
            c.set(2,32,c.r[4].wrapping_add(0x70));
            c.set(0,32,c.r[4].wrapping_add(0x90));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x4c8); wm(m,a,8,0x8u32); }
            c.push(m, 0x4bee60);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bf940, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.set(1,32,c.r[4].wrapping_add(0x84));
            { let a = c.r[4].wrapping_add(0x4c4); wm(m,a,8,0x9u32); }
            c.push(m, 0x4bee76);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.r[4].wrapping_add(0x74));
            { let a = c.r[4].wrapping_add(0x4c4); wm(m,a,8,0x6u32); }
            c.push(m, 0x4bee89);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.r[4].wrapping_add(0x9c));
            { let a = c.r[4].wrapping_add(0x4c4); wm(m,a,8,0x5u32); }
            c.push(m, 0x4bee9f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x1u32);
            c.set(1,32,c.r[4].wrapping_add(0xac));
            { let a = c.r[4].wrapping_add(0x4c4); wm(m,a,8,0x4u32); }
            c.push(m, 0x4beeb5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4be7c1; continue;
            }
            0x4beeba => {
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4beec6; continue; }
            pc = 0x4beebe; continue;
            }
            0x4beebe => {
            c.push(m,c.get(5,32));
            c.push(m, 0x4beec4);
            { let args = []; c.r[0] = host.call(m, 0x6389a6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4beed0; continue;
            }
            0x4beec6 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x4c),32));
            pc = 0x4beeca; continue;
            }
            0x4beeca => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4beed0);
            { let args = []; c.r[0] = host.call(m, 0x63bdbe, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4beed0; continue;
            }
            0x4beed0 => {
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x34));
            { let a = c.r[4].wrapping_add(0x4c0); wm(m,a,8,0x3u32); }
            c.push(m,0x1u32);
            c.push(m, 0x4beee6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let v = c.pop(m); c.set(5,32,v); }
            if c.cond(4) { pc = 0x4bef0c; continue; }
            pc = 0x4beef0; continue;
            }
            0x4beef0 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bef03; continue; }
            pc = 0x4beef8; continue;
            }
            0x4beef8 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4bef03; continue; }
            pc = 0x4beefd; continue;
            }
            0x4beefd => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4bef0c; continue;
            }
            0x4bef03 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4bef09);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bef0c; continue;
            }
            0x4bef0c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x4bef3c; continue; }
            pc = 0x4bef20; continue;
            }
            0x4bef20 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bef33; continue; }
            pc = 0x4bef28; continue;
            }
            0x4bef28 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4bef33; continue; }
            pc = 0x4bef2d; continue;
            }
            0x4bef2d => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4bef3c; continue;
            }
            0x4bef33 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4bef39);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bef3c; continue;
            }
            0x4bef3c => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x58),32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            if c.cond(4) { pc = 0x4bef6c; continue; }
            pc = 0x4bef50; continue;
            }
            0x4bef50 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bef63; continue; }
            pc = 0x4bef58; continue;
            }
            0x4bef58 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4bef63; continue; }
            pc = 0x4bef5d; continue;
            }
            0x4bef5d => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4bef6c; continue;
            }
            0x4bef63 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4bef69);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bef6c; continue;
            }
            0x4bef6c => {
            c.set(1,32,c.r[4].wrapping_add(0x3c));
            { let a = c.r[4].wrapping_add(0x58); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x5c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x60); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x4b8); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4bef8c);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4b0),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4b4u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ini_file_cache_preload_file(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4befb0;
    loop {
        match pc {
            0x4befb0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661e8fu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x30),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bf037; continue; }
            pc = 0x4befd1; continue;
            }
            0x4befd1 => {
            c.set(0,32,rm(m,0x6c7cccu32,32));
            { let a = c.r[4].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0x0u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,c.r[4].wrapping_add(0xc));
            c.push(m, 0x4beff0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m, 0x4beff9);
            { let args = []; c.r[0] = host.call(m, 0x64f709, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(7,32,c.r[6].wrapping_add(0x14));
            c.push(m,0x0u32);
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4bf00a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x64ec25, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bf026; continue; }
            pc = 0x4bf00e; continue;
            }
            0x4bf00e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf01a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4be680, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x8));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4bf026);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64ebf1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bf026; continue;
            }
            0x4bf026 => {
            c.set(1,32,c.r[4].wrapping_add(0x8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4bf037);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bf037; continue;
            }
            0x4bf037 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ini_file_cache_set_enabled(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf110;
    loop {
        match pc {
            0x4bf110 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x4),8));
            { let a = c.get(4,32); let r = c.sub(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(3,32,c.get(1,32));
            if c.cond(5) { pc = 0x4bf1d6; continue; }
            pc = 0x4bf125; continue;
            }
            0x4bf125 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bf134; continue; }
            pc = 0x4bf12c; continue;
            }
            0x4bf12c => {
            c.set(1,32,c.r[3].wrapping_add(0x14));
            c.push(m, 0x4bf134);
            { let args = []; c.r[0] = host.call(m, 0x64eb28, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bf134; continue;
            }
            0x4bf134 => {
            c.set(6,32,rm(m,c.r[3].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x10),32));
            c.set(7,32,c.r[3].wrapping_add(0x4));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4bf1b2; continue; }
            pc = 0x4bf147; continue;
            }
            0x4bf147 => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4bf1b2; continue; }
            pc = 0x4bf14d; continue;
            }
            0x4bf14d => {
            c.set(5,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,0x6dd564u32,32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            c.set(6,32,c.get(5,32));
            if c.cond(4) { pc = 0x4bf186; continue; }
            pc = 0x4bf15b; continue;
            }
            0x4bf15b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf166);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c0050, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[6],32));
            c.push(m,0x0u32);
            c.set(1,32,c.r[5].wrapping_add(0xc));
            c.push(m, 0x4bf172);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c08a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(5,32));
            c.push(m, 0x4bf178);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dd564u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            c.set(5,32,c.get(6,32));
            if c.cond(5) { pc = 0x4bf15b; continue; }
            pc = 0x4bf186; continue;
            }
            0x4bf186 => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            { let a = c.r[0]; wm(m,a,32,c.get(0,32)); }
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x30),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[3].wrapping_add(0x30); wm(m,a,8,c.get(1,8)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4bf1b2 => {
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4bf1d6; continue; }
            pc = 0x4bf1b6; continue;
            }
            0x4bf1b6 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(5,32,c.get(0,32));
            c.push(m, 0x4bf1c1);
            { let args = []; c.r[0] = host.call(m, 0x4c04d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(5,32));
            c.push(m,c.get(2,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4bf1ce);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bfc70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4bf1b6; continue; }
            pc = 0x4bf1d6; continue;
            }
            0x4bf1d6 => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x1c),8));
            c.set(0,8,rm(m,c.r[3].wrapping_add(0x30),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[3].wrapping_add(0x30); wm(m,a,8,c.get(1,8)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn ini_file_cache_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf210;
    loop {
        match pc {
            0x4bf210 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4bf218);
            { let args = []; c.r[0] = host.call(m, 0x4bdf20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4bf228; continue; }
            pc = 0x4bf21f; continue;
            }
            0x4bf21f => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bf225);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bf228; continue;
            }
            0x4bf228 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_destroy(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf230;
    loop {
        match pc {
            0x4bf230 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(1,32); c.sub(a,c.get(5,32),32); }
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4bf2ab; continue; }
            pc = 0x4bf24b; continue;
            }
            0x4bf24b => {
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4bf2ab; continue; }
            pc = 0x4bf251; continue;
            }
            0x4bf251 => {
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(0,32,rm(m,0x6dd564u32,32));
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            c.set(7,32,c.get(3,32));
            if c.cond(4) { pc = 0x4bf289; continue; }
            pc = 0x4bf25f; continue;
            }
            0x4bf25f => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf26a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c0050, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,rm(m,c.r[7],32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.r[3].wrapping_add(0xc));
            c.push(m, 0x4bf275);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c08a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,c.get(3,32));
            c.push(m, 0x4bf27b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dd564u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            c.set(3,32,c.get(7,32));
            if c.cond(5) { pc = 0x4bf25f; continue; }
            pc = 0x4bf289; continue;
            }
            0x4bf289 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(2,32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.get(6,32));
            { let a = c.r[0]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4bf2a9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf930, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bf2cf; continue;
            }
            0x4bf2ab => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bf2cf; continue; }
            pc = 0x4bf2af; continue;
            }
            0x4bf2af => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(3,32,c.get(0,32));
            c.push(m, 0x4bf2ba);
            { let args = []; c.r[0] = host.call(m, 0x4c04d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf2c7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bfc70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bf2af; continue; }
            pc = 0x4bf2cf; continue;
            }
            0x4bf2cf => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bf2d8);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.push(m, 0x4bf2ec);
            { let args = []; c.r[0] = host.call(m, 0x64b52e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dd560u32,32));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = 0x6dd560u32; wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4bf305; continue; }
            pc = 0x4bf2f9; continue;
            }
            0x4bf2f9 => {
            c.set(6,32,rm(m,0x6dd564u32,32));
            { let a = 0x6dd564u32; wm(m,a,32,c.get(5,32)); }
            pc = 0x4bf305; continue;
            }
            0x4bf305 => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x4bf30e);
            { let args = []; c.r[0] = host.call(m, 0x64b5ca, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4bf31b; continue; }
            pc = 0x4bf312; continue;
            }
            0x4bf312 => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4bf318);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bf31b; continue;
            }
            0x4bf31b => {
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

pub fn std_string_pair_destroy(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf330;
    loop {
        match pc {
            0x4bf330 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4bf359; continue; }
            pc = 0x4bf33d; continue;
            }
            0x4bf33d => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bf350; continue; }
            pc = 0x4bf345; continue;
            }
            0x4bf345 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4bf350; continue; }
            pc = 0x4bf34a; continue;
            }
            0x4bf34a => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4bf359; continue;
            }
            0x4bf350 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf356);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bf359; continue;
            }
            0x4bf359 => {
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x18); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4bf38f; continue; }
            pc = 0x4bf369; continue;
            }
            0x4bf369 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4bf386; continue; }
            pc = 0x4bf371; continue;
            }
            0x4bf371 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4bf386; continue; }
            pc = 0x4bf376; continue;
            }
            0x4bf376 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bf386 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf38c);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bf38f; continue;
            }
            0x4bf38f => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_copy_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf3a0;
    loop {
        match pc {
            0x4bf3a0 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(0,8,rm(m,c.r[3],8));
            c.set(6,32,c.get(1,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x8),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(6) { pc = 0x4bf3c6; continue; }
            pc = 0x4bf3c3; continue;
            }
            0x4bf3c3 => {
            { let a = c.get(7,32); let r = c.or(a,0xffffffffu32,32); c.set(7,32,r); }
            pc = 0x4bf3c6; continue;
            }
            0x4bf3c6 => {
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4bf42c; continue; }
            pc = 0x4bf3ca; continue;
            }
            0x4bf3ca => {
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x4bf3d6; continue; }
            pc = 0x4bf3d1; continue;
            }
            0x4bf3d1 => {
            c.push(m, 0x4bf3d6);
            { let args = []; c.r[0] = host.call(m, 0x64b404, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bf3d6; continue;
            }
            0x4bf3d6 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf3dd);
            { let args = []; c.r[0] = host.call(m, 0x4bf810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(3) { pc = 0x4bf3ed; continue; }
            pc = 0x4bf3e7; continue;
            }
            0x4bf3e7 => {
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x4bf41c; continue; }
            pc = 0x4bf3ed; continue;
            }
            0x4bf3ed => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[3]));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bf3ff);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4bf410);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bf41c; continue; }
            pc = 0x4bf414; continue;
            }
            0x4bf414 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf41c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf7f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bf41c; continue;
            }
            0x4bf41c => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf423);
            { let args = []; c.r[0] = host.call(m, 0x4bf810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4bf42c => {
            { let a = c.get(7,32); c.sub(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x4bf47b; continue; }
            pc = 0x4bf430; continue;
            }
            0x4bf430 => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bf47b; continue; }
            pc = 0x4bf434; continue;
            }
            0x4bf434 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bf440; continue; }
            pc = 0x4bf43b; continue;
            }
            0x4bf43b => {
            c.set(0,32,0x66e484u32);
            pc = 0x4bf440; continue;
            }
            0x4bf440 => {
            { let a = rm(m,c.r[0].wrapping_add(0xffffffff),8); c.sub(a,0xfeu32,8); }
            if c.cond(3) { pc = 0x4bf47b; continue; }
            pc = 0x4bf446; continue;
            }
            0x4bf446 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bf45b; continue; }
            pc = 0x4bf456; continue;
            }
            0x4bf456 => {
            c.set(0,32,0x66e484u32);
            pc = 0x4bf45b; continue;
            }
            0x4bf45b => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4bf47b => {
            c.push(m,0x1u32);
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf485);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bf4ab; continue; }
            pc = 0x4bf489; continue;
            }
            0x4bf489 => {
            c.set(3,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(3,32); c.and(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4bf495; continue; }
            pc = 0x4bf490; continue;
            }
            0x4bf490 => {
            c.set(3,32,0x66e484u32);
            pc = 0x4bf495; continue;
            }
            0x4bf495 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf4a0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4bf4ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf7f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bf4ab; continue;
            }
            0x4bf4ab => {
            c.set(0,32,c.get(6,32));
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

pub fn std_string_assign_substr(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf4c0;
    loop {
        match pc {
            0x4bf4c0 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x4bf4da; continue; }
            pc = 0x4bf4d5; continue;
            }
            0x4bf4d5 => {
            c.push(m, 0x4bf4da);
            { let args = []; c.r[0] = host.call(m, 0x64b404, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bf4da; continue;
            }
            0x4bf4da => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(7,32); let r = c.sub(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4bf4eb; continue; }
            pc = 0x4bf4e9; continue;
            }
            0x4bf4e9 => {
            c.set(7,32,c.get(1,32));
            pc = 0x4bf4eb; continue;
            }
            0x4bf4eb => {
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bf5a2; continue; }
            pc = 0x4bf4f3; continue;
            }
            0x4bf4f3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(7,32); let r = c.add(a,c.get(3,32),32); c.set(7,32,r); }
            { let a = c.get(5,32); let r = c.or(a,0xffffffffu32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4bf504; continue; }
            pc = 0x4bf4ff; continue;
            }
            0x4bf4ff => {
            c.push(m, 0x4bf504);
            { let args = []; c.r[0] = host.call(m, 0x64b404, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bf504; continue;
            }
            0x4bf504 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf50b);
            { let args = []; c.r[0] = host.call(m, 0x4bf810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(7,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(3) { pc = 0x4bf51b; continue; }
            pc = 0x4bf515; continue;
            }
            0x4bf515 => {
            c.set(5,32,c.get(0,32));
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x4bf54d; continue; }
            pc = 0x4bf51b; continue;
            }
            0x4bf51b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[5]));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bf52d);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(5,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4bf53f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bf54d; continue; }
            pc = 0x4bf543; continue;
            }
            0x4bf543 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,8,0x0u32); }
            pc = 0x4bf54d; continue;
            }
            0x4bf54d => {
            c.set(1,32,c.get(6,32));
            c.set(5,32,c.get(3,32));
            c.push(m, 0x4bf556);
            { let args = []; c.r[0] = host.call(m, 0x4bf810, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x4bf55f; continue; }
            pc = 0x4bf55d; continue;
            }
            0x4bf55d => {
            c.set(5,32,c.get(0,32));
            pc = 0x4bf55f; continue;
            }
            0x4bf55f => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x4bf6ae; continue; }
            pc = 0x4bf567; continue;
            }
            0x4bf567 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[5]));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bf577);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(5,32),32); c.set(7,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,0x0u32);
            c.push(m,c.get(7,32));
            c.push(m, 0x4bf589);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bf6ae; continue; }
            pc = 0x4bf591; continue;
            }
            0x4bf591 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf599);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf7f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4bf5a2 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            if c.cond(6) { pc = 0x4bf618; continue; }
            pc = 0x4bf5a8; continue;
            }
            0x4bf5a8 => {
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bf618; continue; }
            pc = 0x4bf5ac; continue;
            }
            0x4bf5ac => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4bf5b8; continue; }
            pc = 0x4bf5b3; continue;
            }
            0x4bf5b3 => {
            c.set(0,32,0x66e484u32);
            pc = 0x4bf5b8; continue;
            }
            0x4bf5b8 => {
            { let a = rm(m,c.r[0].wrapping_add(0xffffffff),8); c.sub(a,0xfeu32,8); }
            if c.cond(3) { pc = 0x4bf618; continue; }
            pc = 0x4bf5be; continue;
            }
            0x4bf5be => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4bf5e3; continue; }
            pc = 0x4bf5c5; continue;
            }
            0x4bf5c5 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4bf5d8; continue; }
            pc = 0x4bf5cd; continue;
            }
            0x4bf5cd => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4bf5d8; continue; }
            pc = 0x4bf5d2; continue;
            }
            0x4bf5d2 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4bf5e3; continue;
            }
            0x4bf5d8 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf5de);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x4bf5e3; continue;
            }
            0x4bf5e3 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4bf5f8; continue; }
            pc = 0x4bf5f3; continue;
            }
            0x4bf5f3 => {
            c.set(0,32,0x66e484u32);
            pc = 0x4bf5f8; continue;
            }
            0x4bf5f8 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4bf618 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf61f);
            { let args = []; c.r[0] = host.call(m, 0x4bfc60, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4bf628; continue; }
            pc = 0x4bf623; continue;
            }
            0x4bf623 => {
            c.push(m, 0x4bf628);
            { let args = []; c.r[0] = host.call(m, 0x64b1e5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bf628; continue;
            }
            0x4bf628 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bf655; continue; }
            pc = 0x4bf62f; continue;
            }
            0x4bf62f => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4bf655; continue; }
            pc = 0x4bf636; continue;
            }
            0x4bf636 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4bf655; continue; }
            pc = 0x4bf63b; continue;
            }
            0x4bf63b => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bf680; continue; }
            pc = 0x4bf63f; continue;
            }
            0x4bf63f => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf64c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4bf655 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bf66b; continue; }
            pc = 0x4bf659; continue;
            }
            0x4bf659 => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf662);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4bf66b => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,0x1fu32,32); }
            if c.cond(7) { pc = 0x4bf677; continue; }
            pc = 0x4bf673; continue;
            }
            0x4bf673 => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4bf688; continue; }
            pc = 0x4bf677; continue;
            }
            0x4bf677 => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf680);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bf680; continue;
            }
            0x4bf680 => {
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4bf688);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bf688; continue;
            }
            0x4bf688 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bf694; continue; }
            pc = 0x4bf68f; continue;
            }
            0x4bf68f => {
            c.set(0,32,0x66e484u32);
            pc = 0x4bf694; continue;
            }
            0x4bf694 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf6a1);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,8,0x0u32); }
            pc = 0x4bf6ae; continue;
            }
            0x4bf6ae => {
            c.set(0,32,c.get(6,32));
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

pub fn std_string_find_first_of(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf6c0;
    loop {
        match pc {
            0x4bf6c0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.push(m,c.get(7,32));
            c.set(3,32,c.get(1,32));
            if c.cond(6) { pc = 0x4bf705; continue; }
            pc = 0x4bf6ce; continue;
            }
            0x4bf6ce => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4bf705; continue; }
            pc = 0x4bf6d9; continue;
            }
            0x4bf6d9 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[1]));
            c.set(6,32,c.r[0].wrapping_add(c.r[2]));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4bf705; continue; }
            pc = 0x4bf6e6; continue;
            }
            0x4bf6e6 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4bf6ea; continue;
            }
            0x4bf6ea => {
            c.set(1,32,sx(rm(m,c.r[6],8),8)&0xffffffffu32);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4bf6f9);
            { let args = []; c.r[0] = host.call(m, 0x63c070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bf70f; continue; }
            pc = 0x4bf700; continue;
            }
            0x4bf700 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x4bf6ea; continue; }
            pc = 0x4bf705; continue;
            }
            0x4bf705 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4bf70f => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_find_first_not_of(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf720;
    loop {
        match pc {
            0x4bf720 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(3,32));
            c.set(3,32,c.get(1,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x8),32));
            { let a = c.get(2,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4bf75d; continue; }
            pc = 0x4bf731; continue;
            }
            0x4bf731 => {
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(7,32,c.r[0].wrapping_add(c.r[1]));
            c.set(6,32,c.r[0].wrapping_add(c.r[2]));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4bf75d; continue; }
            pc = 0x4bf73e; continue;
            }
            0x4bf73e => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x4bf742; continue;
            }
            0x4bf742 => {
            c.set(0,32,sx(rm(m,c.r[6],8),8)&0xffffffffu32);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bf751);
            { let args = []; c.r[0] = host.call(m, 0x63c070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bf767; continue; }
            pc = 0x4bf758; continue;
            }
            0x4bf758 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x4bf742; continue; }
            pc = 0x4bf75d; continue;
            }
            0x4bf75d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4bf767 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_find_last_not_of(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf780;
    loop {
        match pc {
            0x4bf780 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x4bf7de; continue; }
            pc = 0x4bf78d; continue;
            }
            0x4bf78d => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x4bf798; continue; }
            pc = 0x4bf795; continue;
            }
            0x4bf795 => {
            c.set(0,32,c.r[1].wrapping_add(0xffffffff));
            pc = 0x4bf798; continue;
            }
            0x4bf798 => {
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(6,32); let r = c.add(a,c.get(0,32),32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            c.set(0,32,sx(rm(m,c.r[6],8),8)&0xffffffffu32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4bf7b0);
            { let args = []; c.r[0] = host.call(m, 0x63c070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bf7d0; continue; }
            pc = 0x4bf7b7; continue;
            }
            0x4bf7b7 => {
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[5].wrapping_add(0x4),32),32); }
            if c.cond(4) { pc = 0x4bf7de; continue; }
            pc = 0x4bf7bc; continue;
            }
            0x4bf7bc => {
            c.set(1,32,sx(rm(m,c.r[6].wrapping_add(0xffffffff),8),8)&0xffffffffu32);
            { let a = c.get(6,32); let r = c.dec(a,32); c.set(6,32,r); }
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4bf7c9);
            { let args = []; c.r[0] = host.call(m, 0x63c070, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bf7b7; continue; }
            pc = 0x4bf7d0; continue;
            }
            0x4bf7d0 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4bf7de => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_set_length(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf7f0;
    loop {
        match pc {
            0x4bf7f0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(c.r[1]); wm(m,a,8,0x0u32); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_unshare(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf810;
    loop {
        match pc {
            0x4bf810 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bf8d3; continue; }
            pc = 0x4bf820; continue;
            }
            0x4bf820 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4bf8d3; continue; }
            pc = 0x4bf82b; continue;
            }
            0x4bf82b => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4bf8d3; continue; }
            pc = 0x4bf834; continue;
            }
            0x4bf834 => {
            c.set(5,32,c.get(0,32));
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.push(m,c.get(5,32));
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            c.push(m, 0x4bf856);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,c.get(0,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(6) { pc = 0x4bf865; continue; }
            pc = 0x4bf860; continue;
            }
            0x4bf860 => {
            c.push(m, 0x4bf865);
            { let args = []; c.r[0] = host.call(m, 0x64b1e5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bf865; continue;
            }
            0x4bf865 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bf88d; continue; }
            pc = 0x4bf86c; continue;
            }
            0x4bf86c => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4bf88d; continue; }
            pc = 0x4bf873; continue;
            }
            0x4bf873 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4bf88d; continue; }
            pc = 0x4bf878; continue;
            }
            0x4bf878 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bf8b3; continue; }
            pc = 0x4bf87c; continue;
            }
            0x4bf87c => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf889);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bf88d => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bf89e; continue; }
            pc = 0x4bf891; continue;
            }
            0x4bf891 => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf89a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4bf89e => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,0x1fu32,32); }
            if c.cond(7) { pc = 0x4bf8aa; continue; }
            pc = 0x4bf8a6; continue;
            }
            0x4bf8a6 => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4bf8bb; continue; }
            pc = 0x4bf8aa; continue;
            }
            0x4bf8aa => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bf8b3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bf8b3; continue;
            }
            0x4bf8b3 => {
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4bf8bb);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bf8bb; continue;
            }
            0x4bf8bb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf8c6);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[7].wrapping_add(c.r[1]); wm(m,a,8,0x0u32); }
            pc = 0x4bf8d3; continue;
            }
            0x4bf8d3 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_tidy_flag(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf8e0;
    loop {
        match pc {
            0x4bf8e0 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x4),8));
            c.push(m,c.get(6,32));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            c.set(6,32,c.get(1,32));
            if c.cond(4) { pc = 0x4bf90e; continue; }
            pc = 0x4bf8eb; continue;
            }
            0x4bf8eb => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bf90e; continue; }
            pc = 0x4bf8f2; continue;
            }
            0x4bf8f2 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4bf905; continue; }
            pc = 0x4bf8fa; continue;
            }
            0x4bf8fa => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4bf905; continue; }
            pc = 0x4bf8ff; continue;
            }
            0x4bf8ff => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            pc = 0x4bf90e; continue;
            }
            0x4bf905 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf90b);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4bf90e; continue;
            }
            0x4bf90e => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_begin(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf930;
    loop {
        match pc {
            0x4bf930 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_insert(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bf940;
    loop {
        match pc {
            0x4bf940 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(1,32,rm(m,0x6dd564u32,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,8,0x1u32);
            c.set(5,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(6,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4bf995; continue; }
            pc = 0x4bf95d; continue;
            }
            0x4bf95d => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x14),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(5,32,c.get(6,32));
            if c.cond(5) { pc = 0x4bf96e; continue; }
            pc = 0x4bf969; continue;
            }
            0x4bf969 => {
            c.set(0,32,0x66e484u32);
            pc = 0x4bf96e; continue;
            }
            0x4bf96e => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x8),32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m,0x0u32);
            c.push(m, 0x4bf97d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4c0910, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,8,u32::from(c.cond(12)));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4bf98a; continue; }
            pc = 0x4bf986; continue;
            }
            0x4bf986 => {
            c.set(6,32,rm(m,c.r[6],32));
            pc = 0x4bf98d; continue;
            }
            0x4bf98a => {
            c.set(6,32,rm(m,c.r[6].wrapping_add(0x8),32));
            pc = 0x4bf98d; continue;
            }
            0x4bf98d => {
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(5) { pc = 0x4bf95d; continue; }
            pc = 0x4bf995; continue;
            }
            0x4bf995 => {
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x8),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4bfb25; continue; }
            pc = 0x4bf9a0; continue;
            }
            0x4bf9a0 => {
            c.push(m,0x0u32);
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4bf9aa);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4c0450, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,rm(m,0x6dd564u32,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,0x6dd564u32,32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.add(a,0xcu32,32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf9c9);
            { let args = []; c.r[0] = host.call(m, 0x4c0640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x4bfa05; continue; }
            pc = 0x4bf9da; continue;
            }
            0x4bf9da => {
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(5) { pc = 0x4bfa05; continue; }
            pc = 0x4bf9e2; continue;
            }
            0x4bf9e2 => {
            c.set(0,32,c.r[5].wrapping_add(0xc));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bf9ed);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c08c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4bfa05; continue; }
            pc = 0x4bf9f1; continue;
            }
            0x4bf9f1 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x4bfa2c; continue; }
            pc = 0x4bfa00; continue;
            }
            0x4bfa00 => {
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            pc = 0x4bfa2c; continue;
            }
            0x4bfa05 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[5]; wm(m,a,32,c.get(1,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bfa1e; continue; }
            pc = 0x4bfa13; continue;
            }
            0x4bfa13 => {
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x4bfa28; continue;
            }
            0x4bfa1e => {
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[0],32),32); }
            if c.cond(5) { pc = 0x4bfa28; continue; }
            pc = 0x4bfa22; continue;
            }
            0x4bfa22 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4bfa28; continue;
            }
            0x4bfa28 => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            pc = 0x4bfa2c; continue;
            }
            0x4bfa2c => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.set(6,32,c.get(3,32));
            { let a = c.get(3,32); c.sub(a,rm(m,c.r[2].wrapping_add(0x4),32),32); }
            if c.cond(4) { pc = 0x4bfb06; continue; }
            pc = 0x4bfa3a; continue;
            }
            0x4bfa3a => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x2c),32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bfb06; continue; }
            pc = 0x4bfa48; continue;
            }
            0x4bfa48 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[2],32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4bfaaa; continue; }
            pc = 0x4bfa51; continue;
            }
            0x4bfa51 => {
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = rm(m,c.r[1].wrapping_add(0x2c),32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bfa78; continue; }
            pc = 0x4bfa59; continue;
            }
            0x4bfa59 => {
            c.set(2,32,0x1u32);
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x4),32));
            pc = 0x4bfafa; continue;
            }
            0x4bfa78 => {
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x4bfa87; continue; }
            pc = 0x4bfa7d; continue;
            }
            0x4bfa7d => {
            c.set(6,32,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4bfa87);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c0390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bfa87; continue;
            }
            0x4bfa87 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,0x1u32); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4bfaa8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c03f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bfafa; continue;
            }
            0x4bfaaa => {
            { let a = rm(m,c.r[1].wrapping_add(0x2c),32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4bfacb; continue; }
            pc = 0x4bfaaf; continue;
            }
            0x4bfaaf => {
            c.set(2,32,0x1u32);
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            pc = 0x4bfafa; continue;
            }
            0x4bfacb => {
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[0],32),32); }
            if c.cond(5) { pc = 0x4bfad9; continue; }
            pc = 0x4bfacf; continue;
            }
            0x4bfacf => {
            c.set(6,32,c.get(0,32));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4bfad9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c03f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bfad9; continue;
            }
            0x4bfad9 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x2c); wm(m,a,32,0x1u32); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,c.get(7,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4bfafa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c0390, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bfafa; continue;
            }
            0x4bfafa => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4bfa3a; continue; }
            pc = 0x4bfb06; continue;
            }
            0x4bfb06 => {
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,0x1u32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,8,0x1u32); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4bfb25 => {
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x4bfb47; continue; }
            pc = 0x4bfb2d; continue;
            }
            0x4bfb2d => {
            c.set(1,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[1],32),32); }
            if c.cond(5) { pc = 0x4bfb3e; continue; }
            pc = 0x4bfb34; continue;
            }
            0x4bfb34 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(2,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            pc = 0x4bfb60; continue;
            }
            0x4bfb3e => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x4bfb47);
            { let args = []; c.r[0] = host.call(m, 0x4c0470, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bfb47; continue;
            }
            0x4bfb47 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(3,32));
            c.set(1,32,c.r[2].wrapping_add(0xc));
            c.push(m, 0x4bfb54);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c08c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x4bfb7b; continue; }
            pc = 0x4bfb58; continue;
            }
            0x4bfb58 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            pc = 0x4bfb60; continue;
            }
            0x4bfb60 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4bfb67);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x4c00a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(1,32,rm(m,c.r[0],32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,8,0x1u32); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4bfb7b => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,c.r[4].wrapping_add(0x1c));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x0u32); }
            c.push(m, 0x4bfb95);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4bfba0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
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

pub fn std_map_insert_result_init(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bfba0;
    loop {
        match pc {
            0x4bfba0 => {
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(2,8,rm(m,c.r[1],8));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,8,c.get(2,8)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_assign_chars(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bfbc0;
    loop {
        match pc {
            0x4bfbc0 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(6) { pc = 0x4bfbd2; continue; }
            pc = 0x4bfbcd; continue;
            }
            0x4bfbcd => {
            c.push(m, 0x4bfbd2);
            { let args = []; c.r[0] = host.call(m, 0x64b1e5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4bfbd2; continue;
            }
            0x4bfbd2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4bfbfe; continue; }
            pc = 0x4bfbd9; continue;
            }
            0x4bfbd9 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4bfbfe; continue; }
            pc = 0x4bfbe0; continue;
            }
            0x4bfbe0 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4bfbfe; continue; }
            pc = 0x4bfbe5; continue;
            }
            0x4bfbe5 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bfc27; continue; }
            pc = 0x4bfbe9; continue;
            }
            0x4bfbe9 => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4bfbfe => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bfc12; continue; }
            pc = 0x4bfc02; continue;
            }
            0x4bfc02 => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bfc0b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4bfc12 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc),32));
            { let a = c.get(0,32); c.sub(a,0x1fu32,32); }
            if c.cond(7) { pc = 0x4bfc1e; continue; }
            pc = 0x4bfc1a; continue;
            }
            0x4bfc1a => {
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4bfc2f; continue; }
            pc = 0x4bfc1e; continue;
            }
            0x4bfc1e => {
            c.push(m,0x1u32);
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4bfc27);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bfc27; continue;
            }
            0x4bfc27 => {
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4bfc2f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4bfc2f; continue;
            }
            0x4bfc2f => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4bfc3e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,c.get(6,32));
            { let a = c.r[7].wrapping_add(c.r[2]); wm(m,a,8,0x0u32); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_max_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bfc60;
    loop {
        match pc {
            0x4bfc60 => {
            c.set(0,32,0xfffffffdu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_erase_node(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4bfc70;
    loop {
        match pc {
            0x4bfc70 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x1c));
            c.push(m, 0x4bfc84);
            { let args = []; c.r[0] = host.call(m, 0x4c04d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6],32));
            c.set(2,32,rm(m,0x6dd564u32,32));
            { let a = c.get(7,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            if c.cond(5) { pc = 0x4bfcb5; continue; }
            pc = 0x4bfc94; continue;
            }
            0x4bfc94 => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            pc = 0x4bfc97; continue;
            }
            0x4bfc97 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = rm(m,c.r[0].wrapping_add(0x4),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4bfd36; continue; }
            pc = 0x4bfcad; continue;
            }
            0x4bfcad => {
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x4bfd44; continue;
            }
            0x4bfcb5 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4bfcc0; continue; }
            pc = 0x4bfcbc; continue;
            }
            0x4bfcbc => {
            c.set(1,32,c.get(7,32));
            pc = 0x4bfc97; continue;
            }
            0x4bfcc0 => {
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x4bfcce; continue; }
            pc = 0x4bfcc6; continue;
            }
            0x4bfcc6 => {
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[0],32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4bfcc6; continue; }
            pc = 0x4bfcce; continue;
            }
            0x4bfcce => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(4) { pc = 0x4bfc97; continue; }
            pc = 0x4bfcd9; continue;
            }
            0x4bfcd9 => {
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4bfcec; continue; }
            pc = 0x4bfce7; continue;
            }
            0x4bfce7 => {
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bfd03; continue;
            }
            0x4bfcec => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bfd03; continue;
            }
            0x4bfd03 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = rm(m,c.r[2].wrapping_add(0x4),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4bfd10; continue; }
            pc = 0x4bfd0b; continue;
            }
            0x4bfd0b => {
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bfd1e; continue;
            }
            0x4bfd10 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = rm(m,c.r[2],32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4bfd1b; continue; }
            pc = 0x4bfd17; continue;
            }
            0x4bfd17 => {
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4bfd1e; continue;
            }
            0x4bfd1b => {
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bfd1e; continue;
            }
            0x4bfd1e => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x2c),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x2c),32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[6].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bfd9f; continue;
            }
            0x4bfd36 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = rm(m,c.r[0],32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4bfd41; continue; }
            pc = 0x4bfd3d; continue;
            }
            0x4bfd3d => {
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4bfd44; continue;
            }
            0x4bfd41 => {
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x4bfd44; continue;
            }
            0x4bfd44 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = rm(m,c.r[3],32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4bfd71; continue; }
            pc = 0x4bfd4b; continue;
            }
            0x4bfd4b => {
            c.set(7,32,rm(m,0x6dd564u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bfd5f; continue; }
            pc = 0x4bfd58; continue;
            }
            0x4bfd58 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[3]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4bfd71; continue;
            }
            0x4bfd5f => {
            c.set(0,32,rm(m,c.r[1],32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bfd6f; continue; }
            pc = 0x4bfd67; continue;
            }
            0x4bfd67 => {
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bfd67; continue; }
            pc = 0x4bfd6f; continue;
            }
            0x4bfd6f => {
            { let a = c.r[3]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4bfd71; continue;
            }
            0x4bfd71 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = rm(m,c.r[3].wrapping_add(0x8),32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4bfd9f; continue; }
            pc = 0x4bfd79; continue;
            }
            0x4bfd79 => {
            c.set(7,32,rm(m,0x6dd564u32,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bfd8a; continue; }
            pc = 0x4bfd85; continue;
            }
            0x4bfd85 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            pc = 0x4bfd9c; continue;
            }
            0x4bfd8a => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(2,32,c.get(1,32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4bfd9c; continue; }
            pc = 0x4bfd93; continue;
            }
            0x4bfd93 => {
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bfd93; continue; }
            pc = 0x4bfd9c; continue;
            }
            0x4bfd9c => {
            { let a = c.r[3].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bfd9f; continue;
            }
            0x4bfd9f => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,0x1u32);
            { let a = rm(m,c.r[6].wrapping_add(0x2c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4c0026; continue; }
            pc = 0x4bfdb1; continue;
            }
            0x4bfdb1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x4),32),32); }
            if c.cond(4) { pc = 0x4c001f; continue; }
            pc = 0x4bfdbd; continue;
            }
            0x4bfdbd => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x4bfdbf; continue;
            }
            0x4bfdbf => {
            { let a = rm(m,c.r[1].wrapping_add(0x2c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4c001f; continue; }
            pc = 0x4bfdc8; continue;
            }
            0x4bfdc8 => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.get(1,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4bfe80; continue; }
            pc = 0x4bfdd5; continue;
            }
            0x4bfdd5 => {
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = rm(m,c.r[0].wrapping_add(0x2c),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bfe2a; continue; }
            pc = 0x4bfddd; continue;
            }
            0x4bfddd => {
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(6,32,rm(m,c.r[2],32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[2],32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4bfdfe; continue; }
            pc = 0x4bfdfb; continue;
            }
            0x4bfdfb => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bfdfe; continue;
            }
            0x4bfdfe => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4bfe11; continue; }
            pc = 0x4bfe0c; continue;
            }
            0x4bfe0c => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bfe1f; continue;
            }
            0x4bfe11 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6],32),32); }
            if c.cond(5) { pc = 0x4bfe1c; continue; }
            pc = 0x4bfe18; continue;
            }
            0x4bfe18 => {
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4bfe1f; continue;
            }
            0x4bfe1c => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bfe1f; continue;
            }
            0x4bfe1f => {
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            pc = 0x4bfe2a; continue;
            }
            0x4bfe2a => {
            c.set(2,32,rm(m,c.r[0],32));
            { let a = rm(m,c.r[2].wrapping_add(0x2c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4bfe3d; continue; }
            pc = 0x4bfe31; continue;
            }
            0x4bfe31 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = rm(m,c.r[2].wrapping_add(0x2c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4bfeea; continue; }
            pc = 0x4bfe3d; continue;
            }
            0x4bfe3d => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = rm(m,c.r[2].wrapping_add(0x2c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4bff1c; continue; }
            pc = 0x4bfe49; continue;
            }
            0x4bfe49 => {
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[2].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[0],32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4bfe66; continue; }
            pc = 0x4bfe63; continue;
            }
            0x4bfe63 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bfe66; continue;
            }
            0x4bfe66 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4bff01; continue; }
            pc = 0x4bfe78; continue;
            }
            0x4bfe78 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bff10; continue;
            }
            0x4bfe80 => {
            { let a = rm(m,c.r[0].wrapping_add(0x2c),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4bfed3; continue; }
            pc = 0x4bfe85; continue;
            }
            0x4bfe85 => {
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4bfea6; continue; }
            pc = 0x4bfea3; continue;
            }
            0x4bfea3 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bfea6; continue;
            }
            0x4bfea6 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4bfeb9; continue; }
            pc = 0x4bfeb4; continue;
            }
            0x4bfeb4 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bfec8; continue;
            }
            0x4bfeb9 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x4bfec6; continue; }
            pc = 0x4bfec1; continue;
            }
            0x4bfec1 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bfec8; continue;
            }
            0x4bfec6 => {
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4bfec8; continue;
            }
            0x4bfec8 => {
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2],32));
            pc = 0x4bfed3; continue;
            }
            0x4bfed3 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = rm(m,c.r[2].wrapping_add(0x2c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4bff7b; continue; }
            pc = 0x4bfedf; continue;
            }
            0x4bfedf => {
            c.set(2,32,rm(m,c.r[0],32));
            { let a = rm(m,c.r[2].wrapping_add(0x2c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4bff7b; continue; }
            pc = 0x4bfeea; continue;
            }
            0x4bfeea => {
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4bfdbf; continue; }
            pc = 0x4bfefc; continue;
            }
            0x4bfefc => {
            pc = 0x4c001f; continue;
            }
            0x4bff01 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x4bff0e; continue; }
            pc = 0x4bff09; continue;
            }
            0x4bff09 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bff10; continue;
            }
            0x4bff0e => {
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4bff10; continue;
            }
            0x4bff10 => {
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            pc = 0x4bff1c; continue;
            }
            0x4bff1c => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x2c),32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(6,32,rm(m,c.r[2],32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[2],32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4bff49; continue; }
            pc = 0x4bff46; continue;
            }
            0x4bff46 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bff49; continue;
            }
            0x4bff49 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4bff61; continue; }
            pc = 0x4bff57; continue;
            }
            0x4bff57 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4c001c; continue;
            }
            0x4bff61 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6],32),32); }
            if c.cond(5) { pc = 0x4bff71; continue; }
            pc = 0x4bff68; continue;
            }
            0x4bff68 => {
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4c001c; continue;
            }
            0x4bff71 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4c001c; continue;
            }
            0x4bff7b => {
            c.set(2,32,rm(m,c.r[0],32));
            { let a = rm(m,c.r[2].wrapping_add(0x2c),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4bffcb; continue; }
            pc = 0x4bff82; continue;
            }
            0x4bff82 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[2].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(6,32,rm(m,c.r[2],32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[2],32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4bffa0; continue; }
            pc = 0x4bff9d; continue;
            }
            0x4bff9d => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bffa0; continue;
            }
            0x4bffa0 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4bffb3; continue; }
            pc = 0x4bffae; continue;
            }
            0x4bffae => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bffc1; continue;
            }
            0x4bffb3 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6],32),32); }
            if c.cond(5) { pc = 0x4bffbe; continue; }
            pc = 0x4bffba; continue;
            }
            0x4bffba => {
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4bffc1; continue;
            }
            0x4bffbe => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4bffc1; continue;
            }
            0x4bffc1 => {
            { let a = c.r[2]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[0],32));
            pc = 0x4bffcb; continue;
            }
            0x4bffcb => {
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(0x2c),32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[0],32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[0],32));
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4bfff7; continue; }
            pc = 0x4bfff4; continue;
            }
            0x4bfff4 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4bfff7; continue;
            }
            0x4bfff7 => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4c000a; continue; }
            pc = 0x4c0005; continue;
            }
            0x4c0005 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c0019; continue;
            }
            0x4c000a => {
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x4c0017; continue; }
            pc = 0x4c0012; continue;
            }
            0x4c0012 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c0019; continue;
            }
            0x4c0017 => {
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4c0019; continue;
            }
            0x4c0019 => {
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c001c; continue;
            }
            0x4c001c => {
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c001f; continue;
            }
            0x4c001f => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(3,32)); }
            pc = 0x4c0026; continue;
            }
            0x4c0026 => {
            c.set(1,32,c.r[6].wrapping_add(0xc));
            c.push(m, 0x4c002e);
            { let args = []; c.r[0] = host.call(m, 0x4bf330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(6,32));
            c.push(m, 0x4c0034);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xc),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[0]; wm(m,a,32,c.get(1,32)); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_erase_subtree(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0050;
    loop {
        match pc {
            0x4c0050 => {
            c.set(0,32,rm(m,0x6dd564u32,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(3,32,c.get(1,32));
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            c.set(6,32,c.get(7,32));
            if c.cond(4) { pc = 0x4c008d; continue; }
            pc = 0x4c0064; continue;
            }
            0x4c0064 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c006f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c0050, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(6,32,rm(m,c.r[6],32));
            c.set(1,32,c.r[7].wrapping_add(0xc));
            c.push(m, 0x4c0079);
            { let args = []; c.r[0] = host.call(m, 0x4bf330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,c.get(7,32));
            c.push(m, 0x4c007f);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6dd564u32,32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            c.set(7,32,c.get(6,32));
            if c.cond(5) { pc = 0x4c0064; continue; }
            pc = 0x4c008d; continue;
            }
            0x4c008d => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_insert_node(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c00a0;
    loop {
        match pc {
            0x4c00a0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m,0x30u32);
            c.set(3,32,c.get(1,32));
            c.push(m, 0x4c00ad);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(5,32,c.get(0,32));
            c.push(m,c.get(7,32));
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[5].wrapping_add(0x2c); wm(m,a,32,0x0u32); }
            c.set(0,32,rm(m,0x6dd564u32,32));
            c.set(2,32,c.r[5].wrapping_add(0xc));
            { let a = c.r[5]; wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6dd564u32,32));
            c.push(m,c.get(2,32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x4c00dc);
            { let args = []; c.r[0] = host.call(m, 0x4c0640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[3].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[3].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x4c011a; continue; }
            pc = 0x4c00ed; continue;
            }
            0x4c00ed => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(1,32,rm(m,0x6dd564u32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x4c011a; continue; }
            pc = 0x4c00fb; continue;
            }
            0x4c00fb => {
            c.set(1,32,c.r[6].wrapping_add(0xc));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4c0106);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4c08c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4c011a; continue; }
            pc = 0x4c010a; continue;
            }
            0x4c010a => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x4c0134; continue; }
            pc = 0x4c0115; continue;
            }
            0x4c0115 => {
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            pc = 0x4c0134; continue;
            }
            0x4c011a => {
            { let a = c.r[6]; wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(6,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4c012e; continue; }
            pc = 0x4c0123; continue;
            }
            0x4c0123 => {
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            pc = 0x4c0134; continue;
            }
            0x4c012e => {
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[0],32),32); }
            if c.cond(5) { pc = 0x4c0134; continue; }
            pc = 0x4c0132; continue;
            }
            0x4c0132 => {
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            pc = 0x4c0134; continue;
            }
            0x4c0134 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            c.set(0,32,c.get(5,32));
            { let a = c.get(5,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x4),32),32); }
            if c.cond(4) { pc = 0x4c02f1; continue; }
            pc = 0x4c0142; continue;
            }
            0x4c0142 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(0x2c),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4c02f1; continue; }
            pc = 0x4c0150; continue;
            }
            0x4c0150 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4c022a; continue; }
            pc = 0x4c015d; continue;
            }
            0x4c015d => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = rm(m,c.r[2].wrapping_add(0x2c),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4c0184; continue; }
            pc = 0x4c0165; continue;
            }
            0x4c0165 => {
            c.set(6,32,0x1u32);
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[2].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            pc = 0x4c02e5; continue;
            }
            0x4c0184 => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x4c01c6; continue; }
            pc = 0x4c0189; continue;
            }
            0x4c0189 => {
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4c01a0; continue; }
            pc = 0x4c019d; continue;
            }
            0x4c019d => {
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c01a0; continue;
            }
            0x4c01a0 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[2].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4c01b3; continue; }
            pc = 0x4c01ae; continue;
            }
            0x4c01ae => {
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c01c1; continue;
            }
            0x4c01b3 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[2],32),32); }
            if c.cond(5) { pc = 0x4c01be; continue; }
            pc = 0x4c01ba; continue;
            }
            0x4c01ba => {
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4c01c1; continue;
            }
            0x4c01be => {
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c01c1; continue;
            }
            0x4c01c1 => {
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c01c6; continue;
            }
            0x4c01c6 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,0x1u32); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1],32));
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.r[1]; wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4c01f4; continue; }
            pc = 0x4c01f1; continue;
            }
            0x4c01f1 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c01f4; continue;
            }
            0x4c01f4 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4c020d; continue; }
            pc = 0x4c0202; continue;
            }
            0x4c0202 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c02e2; continue;
            }
            0x4c020d => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x4c0220; continue; }
            pc = 0x4c0215; continue;
            }
            0x4c0215 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c02e2; continue;
            }
            0x4c0220 => {
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c02e2; continue;
            }
            0x4c022a => {
            { let a = rm(m,c.r[2].wrapping_add(0x2c),32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4c024e; continue; }
            pc = 0x4c022f; continue;
            }
            0x4c022f => {
            c.set(6,32,0x1u32);
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            { let a = c.r[2].wrapping_add(0x2c); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            pc = 0x4c02e5; continue;
            }
            0x4c024e => {
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1],32),32); }
            if c.cond(5) { pc = 0x4c0291; continue; }
            pc = 0x4c0252; continue;
            }
            0x4c0252 => {
            c.set(0,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[0],32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(2,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4c0269; continue; }
            pc = 0x4c0266; continue;
            }
            0x4c0266 => {
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            pc = 0x4c0269; continue;
            }
            0x4c0269 => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[2].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4c027c; continue; }
            pc = 0x4c0277; continue;
            }
            0x4c0277 => {
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c028b; continue;
            }
            0x4c027c => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[2].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x4c0289; continue; }
            pc = 0x4c0284; continue;
            }
            0x4c0284 => {
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c028b; continue;
            }
            0x4c0289 => {
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4c028b; continue;
            }
            0x4c028b => {
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c0291; continue;
            }
            0x4c0291 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,0x1u32); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[1].wrapping_add(0x2c); wm(m,a,32,c.get(7,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(6,32,rm(m,c.r[2],32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[2],32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4c02bf; continue; }
            pc = 0x4c02bc; continue;
            }
            0x4c02bc => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            pc = 0x4c02bf; continue;
            }
            0x4c02bf => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[6].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4c02d2; continue; }
            pc = 0x4c02cd; continue;
            }
            0x4c02cd => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c02e0; continue;
            }
            0x4c02d2 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.sub(a,rm(m,c.r[6],32),32); }
            if c.cond(5) { pc = 0x4c02dd; continue; }
            pc = 0x4c02d9; continue;
            }
            0x4c02d9 => {
            { let a = c.r[6]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4c02e0; continue;
            }
            0x4c02dd => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c02e0; continue;
            }
            0x4c02e0 => {
            { let a = c.r[2]; wm(m,a,32,c.get(1,32)); }
            pc = 0x4c02e2; continue;
            }
            0x4c02e2 => {
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c02e5; continue;
            }
            0x4c02e5 => {
            c.set(1,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4c0142; continue; }
            pc = 0x4c02f1; continue;
            }
            0x4c02f1 => {
            c.set(2,32,rm(m,c.r[3].wrapping_add(0x4),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,0x1u32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.r[0]; wm(m,a,32,c.get(5,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_lower_bound(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0310;
    loop {
        match pc {
            0x4c0310 => {
            c.push(m,c.get(1,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,rm(m,0x6dd564u32,32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(7,32); c.sub(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x4c0382; continue; }
            pc = 0x4c0326; continue;
            }
            0x4c0326 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(6,32));
            pc = 0x4c032d; continue;
            }
            0x4c032d => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(2,32,0x66e484u32);
            if c.cond(4) { pc = 0x4c033e; continue; }
            pc = 0x4c033c; continue;
            }
            0x4c033c => {
            c.set(2,32,c.get(0,32));
            pc = 0x4c033e; continue;
            }
            0x4c033e => {
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x14),32));
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            c.set(1,32,c.get(6,32));
            if c.cond(2) { pc = 0x4c0349; continue; }
            pc = 0x4c0347; continue;
            }
            0x4c0347 => {
            c.set(1,32,c.get(3,32));
            pc = 0x4c0349; continue;
            }
            0x4c0349 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c0354);
            { let args = []; c.r[0] = host.call(m, 0x63b3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4c0366; continue; }
            pc = 0x4c035b; continue;
            }
            0x4c035b => {
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(2) { pc = 0x4c0368; continue; }
            pc = 0x4c035f; continue;
            }
            0x4c035f => {
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            pc = 0x4c0366; continue;
            }
            0x4c0366 => {
            if c.cond(13) { pc = 0x4c036d; continue; }
            pc = 0x4c0368; continue;
            }
            0x4c0368 => {
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x8),32));
            pc = 0x4c0373; continue;
            }
            0x4c036d => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(7,32)); }
            c.set(7,32,rm(m,c.r[7],32));
            pc = 0x4c0373; continue;
            }
            0x4c0373 => {
            { let a = c.get(7,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(5) { pc = 0x4c032d; continue; }
            pc = 0x4c037b; continue;
            }
            0x4c037b => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x4c0382; continue;
            }
            0x4c0382 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_rotate_left(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0390;
    loop {
        match pc {
            0x4c0390 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            c.set(6,32,rm(m,c.r[0],32));
            { let a = c.r[2].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[0],32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4c03aa; continue; }
            pc = 0x4c03a7; continue;
            }
            0x4c03a7 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c03aa; continue;
            }
            0x4c03aa => {
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4c03c4; continue; }
            pc = 0x4c03b8; continue;
            }
            0x4c03b8 => {
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4c03c4 => {
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1],32),32); }
            if c.cond(5) { pc = 0x4c03d6; continue; }
            pc = 0x4c03cb; continue;
            }
            0x4c03cb => {
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4c03d6 => {
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_rotate_right(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c03f0;
    loop {
        match pc {
            0x4c03f0 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(0,32,rm(m,c.r[2],32));
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.r[2]; wm(m,a,32,c.get(6,32)); }
            c.set(6,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.sub(a,rm(m,0x6dd564u32,32),32); }
            if c.cond(4) { pc = 0x4c040a; continue; }
            pc = 0x4c0407; continue;
            }
            0x4c0407 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            pc = 0x4c040a; continue;
            }
            0x4c040a => {
            c.set(6,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(6,32)); }
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x4),32),32); }
            if c.cond(5) { pc = 0x4c0425; continue; }
            pc = 0x4c0418; continue;
            }
            0x4c0418 => {
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4c0425 => {
            c.set(1,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[1].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x4c043a; continue; }
            pc = 0x4c042d; continue;
            }
            0x4c042d => {
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4c043a => {
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[2].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_buy_node(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0450;
    loop {
        match pc {
            0x4c0450 => {
            c.push(m,0x30u32);
            c.push(m, 0x4c0457);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[0].wrapping_add(0x4); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[0].wrapping_add(0x2c); wm(m,a,32,c.get(2,32)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_iterator_decrement(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0470;
    loop {
        match pc {
            0x4c0470 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,c.get(6,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x2c),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x4c0489; continue; }
            pc = 0x4c047a; continue;
            }
            0x4c047a => {
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = rm(m,c.r[2].wrapping_add(0x4),32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4c0489; continue; }
            pc = 0x4c0482; continue;
            }
            0x4c0482 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4c0489 => {
            c.set(2,32,rm(m,c.r[0],32));
            c.set(6,32,rm(m,0x6dd564u32,32));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4c04a9; continue; }
            pc = 0x4c0495; continue;
            }
            0x4c0495 => {
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4c04a5; continue; }
            pc = 0x4c049c; continue;
            }
            0x4c049c => {
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4c049c; continue; }
            pc = 0x4c04a5; continue;
            }
            0x4c04a5 => {
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4c04a9 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[0],32),32); }
            if c.cond(5) { pc = 0x4c04bd; continue; }
            pc = 0x4c04b2; continue;
            }
            0x4c04b2 => {
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[0],32),32); }
            if c.cond(4) { pc = 0x4c04b2; continue; }
            pc = 0x4c04bd; continue;
            }
            0x4c04bd => {
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_map_iterator_increment(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c04d0;
    loop {
        match pc {
            0x4c04d0 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,0x6dd564u32,32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4c04f2; continue; }
            pc = 0x4c04e0; continue;
            }
            0x4c04e0 => {
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x4c04ee; continue; }
            pc = 0x4c04e6; continue;
            }
            0x4c04e6 => {
            c.set(2,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[2],32));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x4c04e6; continue; }
            pc = 0x4c04ee; continue;
            }
            0x4c04ee => {
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4c04f2 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x8),32),32); }
            if c.cond(5) { pc = 0x4c0508; continue; }
            pc = 0x4c04fc; continue;
            }
            0x4c04fc => {
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); c.sub(a,rm(m,c.r[0].wrapping_add(0x8),32),32); }
            if c.cond(4) { pc = 0x4c04fc; continue; }
            pc = 0x4c0508; continue;
            }
            0x4c0508 => {
            c.set(2,32,rm(m,c.r[1],32));
            { let a = rm(m,c.r[2].wrapping_add(0x8),32); c.sub(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c0511; continue; }
            pc = 0x4c050f; continue;
            }
            0x4c050f => {
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4c0511; continue;
            }
            0x4c0511 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_concat(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0520;
    loop {
        match pc {
            0x4c0520 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661ea8u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(1,8,rm(m,c.r[0],8));
            c.push(m,0xffffffffu32);
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(1,8)); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4c0563);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x34),32));
            { let a = c.get(6,32); let r = c.or(a,0xffffffffu32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(3) { pc = 0x4c0578; continue; }
            pc = 0x4c0576; continue;
            }
            0x4c0576 => {
            c.set(6,32,c.get(0,32));
            pc = 0x4c0578; continue;
            }
            0x4c0578 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(7) { pc = 0x4c058a; continue; }
            pc = 0x4c0585; continue;
            }
            0x4c0585 => {
            c.push(m, 0x4c058a);
            { let args = []; c.r[0] = host.call(m, 0x64b1e5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4c058a; continue;
            }
            0x4c058a => {
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(6) { pc = 0x4c05d5; continue; }
            pc = 0x4c058f; continue;
            }
            0x4c058f => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(3,32));
            c.set(7,32,c.r[1].wrapping_add(c.r[6]));
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(7,32));
            c.push(m, 0x4c05a1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4c05d5; continue; }
            pc = 0x4c05a5; continue;
            }
            0x4c05a5 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4c05b5; continue; }
            pc = 0x4c05b0; continue;
            }
            0x4c05b0 => {
            c.set(0,32,0x66e484u32);
            pc = 0x4c05b5; continue;
            }
            0x4c05b5 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(1,32));
            c.push(m, 0x4c05c7);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(7,32)); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[2].wrapping_add(c.r[7]); wm(m,a,8,c.get(3,8)); }
            pc = 0x4c05d5; continue;
            }
            0x4c05d5 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x30),32));
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x10),8));
            c.push(m,0xffffffffu32);
            c.set(1,32,c.r[4].wrapping_add(0x14));
            c.push(m,c.get(3,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4c05f7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x4c062d; continue; }
            pc = 0x4c0600; continue;
            }
            0x4c0600 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.get(1,8); c.sub(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4c0624; continue; }
            pc = 0x4c0608; continue;
            }
            0x4c0608 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4c0624; continue; }
            pc = 0x4c060d; continue;
            }
            0x4c060d => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            { let a = c.r[0]; wm(m,a,8,c.get(1,8)); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4c0624 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x4c062a);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4c062d; continue;
            }
            0x4c062d => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x20u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_pair_copy_ctor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0640;
    loop {
        match pc {
            0x4c0640 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x661ed9u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            if c.cond(4) { pc = 0x4c0765; continue; }
            pc = 0x4c0670; continue;
            }
            0x4c0670 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,0xffffffffu32);
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.set(0,8,rm(m,c.r[7],8));
            c.set(1,32,c.get(6,32));
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4c068c);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4bf4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x10),8));
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x10u32,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x1u32); }
            { let a = c.r[6]; wm(m,a,8,c.get(1,8)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(3,32,c.get(0,32));
            { let a = c.get(3,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(6) { pc = 0x4c06b2; continue; }
            pc = 0x4c06af; continue;
            }
            0x4c06af => {
            { let a = c.get(3,32); let r = c.or(a,0xffffffffu32,32); c.set(3,32,r); }
            pc = 0x4c06b2; continue;
            }
            0x4c06b2 => {
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4c06dc; continue; }
            pc = 0x4c06b6; continue;
            }
            0x4c06b6 => {
            c.push(m,0xffffffffu32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c06c0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4c0780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,c.get(5,32));
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c06c9);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4c0780, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4c06dc => {
            { let a = c.get(3,32); c.sub(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x4c0735; continue; }
            pc = 0x4c06e0; continue;
            }
            0x4c06e0 => {
            { let a = c.get(3,32); c.sub(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4c0735; continue; }
            pc = 0x4c06e4; continue;
            }
            0x4c06e4 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4c06f0; continue; }
            pc = 0x4c06eb; continue;
            }
            0x4c06eb => {
            c.set(0,32,0x66e484u32);
            pc = 0x4c06f0; continue;
            }
            0x4c06f0 => {
            { let a = rm(m,c.r[0].wrapping_add(0xffffffff),8); c.sub(a,0xfeu32,8); }
            if c.cond(3) { pc = 0x4c0735; continue; }
            pc = 0x4c06f6; continue;
            }
            0x4c06f6 => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(5,32)); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(5,32)); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4c070b; continue; }
            pc = 0x4c0706; continue;
            }
            0x4c0706 => {
            c.set(0,32,0x66e484u32);
            pc = 0x4c070b; continue;
            }
            0x4c070b => {
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            c.set(2,32,rm(m,c.r[7].wrapping_add(0x8),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(2,32)); }
            c.set(1,32,rm(m,c.r[7].wrapping_add(0xc),32));
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(1,8); let r = c.inc(a,8); c.set(1,8,r); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4c0735 => {
            c.push(m,0x1u32);
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c073f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4c0765; continue; }
            pc = 0x4c0743; continue;
            }
            0x4c0743 => {
            c.set(7,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4c074f; continue; }
            pc = 0x4c074a; continue;
            }
            0x4c074a => {
            c.set(7,32,0x66e484u32);
            pc = 0x4c074f; continue;
            }
            0x4c074f => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4c075a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4c0765);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf7f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c0765; continue;
            }
            0x4c0765 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_erase(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0780;
    loop {
        match pc {
            0x4c0780 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4c0795; continue; }
            pc = 0x4c0790; continue;
            }
            0x4c0790 => {
            c.push(m, 0x4c0795);
            { let args = []; c.r[0] = host.call(m, 0x64b404, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4c0795; continue;
            }
            0x4c0795 => {
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x4c07f0; continue; }
            pc = 0x4c079c; continue;
            }
            0x4c079c => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0xffffffff),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4c07f0; continue; }
            pc = 0x4c07a3; continue;
            }
            0x4c07a3 => {
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4c07f0; continue; }
            pc = 0x4c07a7; continue;
            }
            0x4c07a7 => {
            { let a = c.get(0,8); let r = c.dec(a,8); c.set(0,8,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[7].wrapping_add(0xffffffff); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,0x0u32); }
            { let a = c.r[6].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            c.push(m, 0x4c07c7);
            { let args = []; c.r[0] = host.call(m, 0x63abe0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(1,32,c.get(6,32));
            c.push(m,0x1u32);
            c.push(m,c.get(5,32));
            c.push(m, 0x4c07d6);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x464410, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4c07f0; continue; }
            pc = 0x4c07da; continue;
            }
            0x4c07da => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(7,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4c07e5);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x4c07f0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf7f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c07f0; continue;
            }
            0x4c07f0 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(3) { pc = 0x4c0803; continue; }
            pc = 0x4c0801; continue;
            }
            0x4c0801 => {
            c.set(5,32,c.get(0,32));
            pc = 0x4c0803; continue;
            }
            0x4c0803 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(6) { pc = 0x4c088c; continue; }
            pc = 0x4c080b; continue;
            }
            0x4c080b => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(5,32),32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.set(2,32,c.r[1].wrapping_add(c.r[5]));
            c.push(m,c.get(2,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4c0821);
            { let args = []; c.r[0] = host.call(m, 0x63aec0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(5,32),32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0xfffffffdu32,32); }
            if c.cond(6) { pc = 0x4c0833; continue; }
            pc = 0x4c082e; continue;
            }
            0x4c082e => {
            c.push(m, 0x4c0833);
            { let args = []; c.r[0] = host.call(m, 0x64b1e5, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4c0833; continue;
            }
            0x4c0833 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c085f; continue; }
            pc = 0x4c083a; continue;
            }
            0x4c083a => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0xffffffff),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4c085f; continue; }
            pc = 0x4c0841; continue;
            }
            0x4c0841 => {
            { let a = c.get(1,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x4c085f; continue; }
            pc = 0x4c0846; continue;
            }
            0x4c0846 => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4c087a; continue; }
            pc = 0x4c084a; continue;
            }
            0x4c084a => {
            { let a = c.get(1,8); let r = c.dec(a,8); c.set(1,8,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[0].wrapping_add(0xffffffff); wm(m,a,8,c.get(1,8)); }
            c.set(1,32,c.get(6,32));
            c.push(m, 0x4c0857);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4bf8e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4c085f => {
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x4c0875; continue; }
            pc = 0x4c0863; continue;
            }
            0x4c0863 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c088c; continue; }
            pc = 0x4c0867; continue;
            }
            0x4c0867 => {
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[0]; wm(m,a,8,0x0u32); }
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x4c0875 => {
            { let a = rm(m,c.r[6].wrapping_add(0xc),32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4c0882; continue; }
            pc = 0x4c087a; continue;
            }
            0x4c087a => {
            c.set(1,32,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4c0882);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x464530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4c0882; continue;
            }
            0x4c0882 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(7,32)); }
            { let a = c.r[7].wrapping_add(c.r[0]); wm(m,a,8,0x0u32); }
            pc = 0x4c088c; continue;
            }
            0x4c088c => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_pair_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c08a0;
    loop {
        match pc {
            0x4c08a0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m, 0x4c08a8);
            { let args = []; c.r[0] = host.call(m, 0x4bf330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = rm(m,c.r[4].wrapping_add(0x8),8); c.and(a,0x1u32,8); }
            if c.cond(4) { pc = 0x4c08b8; continue; }
            pc = 0x4c08af; continue;
            }
            0x4c08af => {
            c.push(m,c.get(6,32));
            c.push(m, 0x4c08b5);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4c08b8; continue;
            }
            0x4c08b8 => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_compare(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c08c0;
    loop {
        match pc {
            0x4c08c0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[0].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(3,32,0x66e484u32);
            if c.cond(4) { pc = 0x4c08d8; continue; }
            pc = 0x4c08d6; continue;
            }
            0x4c08d6 => {
            c.set(3,32,c.get(0,32));
            pc = 0x4c08d8; continue;
            }
            0x4c08d8 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            c.set(2,32,c.get(6,32));
            if c.cond(2) { pc = 0x4c08e3; continue; }
            pc = 0x4c08e1; continue;
            }
            0x4c08e1 => {
            c.set(2,32,c.get(7,32));
            pc = 0x4c08e3; continue;
            }
            0x4c08e3 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c08ee);
            { let args = []; c.r[0] = host.call(m, 0x63b3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4c0909; continue; }
            pc = 0x4c08f5; continue;
            }
            0x4c08f5 => {
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(3) { pc = 0x4c0902; continue; }
            pc = 0x4c08f9; continue;
            }
            0x4c08f9 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            0x4c0902 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            pc = 0x4c0909; continue;
            }
            0x4c0909 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn std_string_compare_range_chars(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0910;
    loop {
        match pc {
            0x4c0910 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = rm(m,c.r[7].wrapping_add(0x8),32); c.sub(a,c.get(3,32),32); }
            if c.cond(3) { pc = 0x4c0924; continue; }
            pc = 0x4c091f; continue;
            }
            0x4c091f => {
            c.push(m, 0x4c0924);
            { let args = []; c.r[0] = host.call(m, 0x64b404, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4c0924; continue;
            }
            0x4c0924 => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(3) { pc = 0x4c0933; continue; }
            pc = 0x4c0931; continue;
            }
            0x4c0931 => {
            c.set(6,32,c.get(0,32));
            pc = 0x4c0933; continue;
            }
            0x4c0933 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,c.get(6,32));
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(2) { pc = 0x4c093f; continue; }
            pc = 0x4c093d; continue;
            }
            0x4c093d => {
            c.set(1,32,c.get(5,32));
            pc = 0x4c093f; continue;
            }
            0x4c093f => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4c0948; continue; }
            pc = 0x4c0946; continue;
            }
            0x4c0946 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(3,32),32); c.set(0,32,r); }
            pc = 0x4c0948; continue;
            }
            0x4c0948 => {
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4c0954);
            { let args = []; c.r[0] = host.call(m, 0x63b3e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(5) { pc = 0x4c0970; continue; }
            pc = 0x4c095b; continue;
            }
            0x4c095b => {
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(3) { pc = 0x4c0969; continue; }
            pc = 0x4c095f; continue;
            }
            0x4c095f => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4c0969 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            c.set(0,8,u32::from(c.cond(5)));
            pc = 0x4c0970; continue;
            }
            0x4c0970 => {
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

pub fn virtual_stub_return_true(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0e40;
    loop {
        match pc {
            0x4c0e40 => {
            c.set(0,8,0x1u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn virtual_stub_noop(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0e50;
    loop {
        match pc {
            0x4c0e50 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn virtual_stub_return_zero(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c0e60;
    loop {
        match pc {
            0x4c0e60 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn cstring_assign_cstr_thunk(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4cab50;
    loop {
        match pc {
            0x4cab50 => {
            { let a = c.get(1,32); let r = c.add(a,0x164u32,32); c.set(1,32,r); }
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn cstring_add_missing_chars(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4cab60;
    loop {
        match pc {
            0x4cab60 => {
            c.push(m,c.get(3,32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(6,32));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.get(2,32); c.and(a,c.get(2,32),32); }
            if c.cond(14) { pc = 0x4caba6; continue; }
            pc = 0x4cab71; continue;
            }
            0x4cab71 => {
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[1].wrapping_add(0x164));
            pc = 0x4cab78; continue;
            }
            0x4cab78 => {
            c.set(0,8,rm(m,c.r[0].wrapping_add(c.r[6]),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,8,c.get(0,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x4cab8b);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f6ce, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(5) { pc = 0x4cab9d; continue; }
            pc = 0x4cab90; continue;
            }
            0x4cab90 => {
            c.set(2,32,rm(m,c.r[3],32));
            c.set(1,32,c.get(7,32));
            c.set(0,8,rm(m,c.r[2].wrapping_add(c.r[6]),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4cab9d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f60a, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x4cab9d; continue;
            }
            0x4cab9d => {
            c.set(0,32,rm(m,c.r[3],32));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,rm(m,c.r[0].wrapping_add(0xfffffff8),32),32); }
            if c.cond(12) { pc = 0x4cab78; continue; }
            pc = 0x4caba5; continue;
            }
            0x4caba5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x4caba6; continue;
            }
            0x4caba6 => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn merchant_business_get_save_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d3240;
    loop {
        match pc {
            0x4d3240 => {
            c.set(0,32,0x14u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn merchant_business_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d3250;
    loop {
        match pc {
            0x4d3250 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d3261);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x4));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d3270);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x6));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d327f);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0x7));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d328c);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0x8));
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d3299);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0xa));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d32a8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0xc));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d32ba);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0xe));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d32c9);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            c.push(m,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d32d8);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x4));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn merchant_business_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d32f0;
    loop {
        match pc {
            0x4d32f0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x4u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4d3301);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(0,32,c.r[7].wrapping_add(0x4));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d3310);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.set(1,32,c.r[7].wrapping_add(0x6));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4d331f);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x1u32);
            c.set(2,32,c.r[7].wrapping_add(0x7));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4d332c);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(0,32,c.r[7].wrapping_add(0x8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d3339);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(1,32,c.r[7].wrapping_add(0xa));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4d3348);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36)), m.r32(c.r[4].wrapping_add(40)), m.r32(c.r[4].wrapping_add(44)), m.r32(c.r[4].wrapping_add(48)), m.r32(c.r[4].wrapping_add(52)), m.r32(c.r[4].wrapping_add(56)), m.r32(c.r[4].wrapping_add(60)), m.r32(c.r[4].wrapping_add(64)), m.r32(c.r[4].wrapping_add(68)), m.r32(c.r[4].wrapping_add(72))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x48u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0xc));
            c.push(m,0x2u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4d335a);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(0,32,c.r[7].wrapping_add(0xe));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d3369);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x4u32);
            { let a = c.get(7,32); let r = c.add(a,0x10u32,32); c.set(7,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x4d3378);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20)), m.r32(c.r[4].wrapping_add(24)), m.r32(c.r[4].wrapping_add(28)), m.r32(c.r[4].wrapping_add(32)), m.r32(c.r[4].wrapping_add(36))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x24u32,32); c.set(4,32,r); }
            c.set(0,32,c.r[6].wrapping_add(0x4));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn merchant_business_init(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d3490;
    loop {
        match pc {
            0x4d3490 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[1].wrapping_add(0xa); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[1].wrapping_add(0xe); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            { let a = c.r[1].wrapping_add(0x4); wm(m,a,16,c.get(0,16)); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(3,32));
            { let a = c.r[1].wrapping_add(0x7); wm(m,a,8,c.get(0,8)); }
            c.push(m,c.get(6,32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.r[1].wrapping_add(0x6); wm(m,a,8,c.get(2,8)); }
            c.set(7,32,c.get(2,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.and(a,0xffu32,32); c.set(7,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[1].wrapping_add(0x8); wm(m,a,16,0xffffu32); }
            { let a = c.r[1].wrapping_add(0x10); wm(m,a,32,0x3f800000u32); }
            c.set(3,32,rm(m,0x6de508u32,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x848),16));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(3,32); c.sub(a,0x6u32,32); }
            if c.cond(13) { pc = 0x4d351e; continue; }
            pc = 0x4d34f7; continue;
            }
            0x4d34f7 => {
            { let a = c.get(3,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x4d3544; continue; }
            pc = 0x4d34fc; continue;
            }
            0x4d34fc => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)));
            pc = 0x4d3509; continue;
            }
            0x4d3509 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            pc = 0x4d3544; continue;
            }
            0x4d351e => {
            { let a = c.get(3,32); c.sub(a,0x9u32,32); }
            if c.cond(13) { pc = 0x4d3531; continue; }
            pc = 0x4d3523; continue;
            }
            0x4d3523 => {
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x4d3509; continue;
            }
            0x4d3531 => {
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            pc = 0x4d3544; continue;
            }
            0x4d3544 => {
            { let a = c.r[1]; wm(m,a,32,c.get(6,32)); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x6729d0),8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(3,32),32); c.set(2,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(2,16)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn merchant_business_recompute_capacity(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d36e0;
    loop {
        match pc {
            0x4d36e0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6),8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x7),8));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(0,32),32); c.set(3,32,r); }
            c.set(6,16,rm(m,c.r[3].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x848),16));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(3,32); c.sub(a,0x6u32,32); }
            if c.cond(13) { pc = 0x4d373e; continue; }
            pc = 0x4d3717; continue;
            }
            0x4d3717 => {
            { let a = c.get(3,32); c.sub(a,0x3u32,32); }
            if c.cond(12) { pc = 0x4d3764; continue; }
            pc = 0x4d371c; continue;
            }
            0x4d371c => {
            c.set(0,32,c.get(6,32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[6].wrapping_add(c.r[2].wrapping_mul(2)));
            pc = 0x4d3729; continue;
            }
            0x4d3729 => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(6,32,c.get(2,32));
            pc = 0x4d3764; continue;
            }
            0x4d373e => {
            { let a = c.get(3,32); c.sub(a,0x9u32,32); }
            if c.cond(13) { pc = 0x4d3751; continue; }
            pc = 0x4d3743; continue;
            }
            0x4d3743 => {
            c.set(0,32,c.r[6].wrapping_add(c.r[6].wrapping_mul(2)));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(8)));
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x4d3729; continue;
            }
            0x4d3751 => {
            c.set(0,32,0x66666667u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(2,32),32); c.set(6,32,r); }
            pc = 0x4d3764; continue;
            }
            0x4d3764 => {
            { let a = c.r[1]; wm(m,a,32,c.get(6,32)); }
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x6729d0),8));
            { let a = c.get(2,32); let r = c.imul(a,c.get(3,32),32); c.set(2,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.r[1].wrapping_add(0xc); wm(m,a,16,c.get(2,16)); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn merchant_business_get_building_count(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d4a20;
    loop {
        match pc {
            0x4d4a20 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.push(m,c.get(3,32));
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x6),8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(3,8,rm(m,c.r[0].wrapping_add(0x6729d0),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xc),16));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(3,32); c.idiv32(b); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn message_pool_free_array(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d6240;
    loop {
        match pc {
            0x4d6240 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4d6268; continue; }
            pc = 0x4d6249; continue;
            }
            0x4d6249 => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x51a880u32);
            c.push(m,c.get(1,32));
            c.push(m,0x10u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4d625e);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(7,32));
            c.push(m, 0x4d6264);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x4d6268; continue;
            }
            0x4d6268 => {
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn message_pool_get_serialized_size(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d6270;
    loop {
        match pc {
            0x4d6270 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            c.set(5,32,0xcu32);
            { let a = rm(m,c.r[6].wrapping_add(0x6),16); c.sub(a,c.get(7,16),16); }
            if c.cond(6) { pc = 0x4d629f; continue; }
            pc = 0x4d6282; continue;
            }
            0x4d6282 => {
            c.push(m,c.get(3,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            pc = 0x4d6285; continue;
            }
            0x4d6285 => {
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(1,32); let r = c.add(a,c.get(3,32),32); c.set(1,32,r); }
            c.push(m, 0x4d628e);
            { let args = []; c.r[0] = host.call(m, 0x51a890, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,32); let r = c.add(a,c.get(0,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[6].wrapping_add(0x6),16));
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(3,32); let r = c.add(a,0x10u32,32); c.set(3,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(12) { pc = 0x4d6285; continue; }
            pc = 0x4d629e; continue;
            }
            0x4d629e => {
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x4d629f; continue;
            }
            0x4d629f => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(0,32,c.get(5,32));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn message_pool_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d62b0;
    loop {
        match pc {
            0x4d62b0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            c.set(0,32,c.r[4].wrapping_add(0xc));
            c.push(m,0x4u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0xaffe0005u32); }
            c.push(m, 0x4d62cf);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(3,32,c.r[7].wrapping_add(0x6));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d62de);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0x4));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d62ed);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,c.r[7].wrapping_add(0x8));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(2,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d62fc);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,c.r[7].wrapping_add(0xa));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d630b);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x3cu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = rm(m,c.r[3],16); c.sub(a,c.get(1,16),16); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            if c.cond(6) { pc = 0x4d633d; continue; }
            pc = 0x4d631c; continue;
            }
            0x4d631c => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            c.push(m, 0x4d6329);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x51a8d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[3],16));
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            c.set(6,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            if c.cond(2) { pc = 0x4d631c; continue; }
            pc = 0x4d633d; continue;
            }
            0x4d633d => {
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn message_pool_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d6350;
    loop {
        match pc {
            0x4d6350 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6629cbu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,c.get(7,32));
            c.push(m,0x4u32);
            c.set(0,32,c.r[4].wrapping_add(0x14));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.set(7,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4d6383);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            c.set(3,32,c.r[7].wrapping_add(0x6));
            c.push(m,0x2u32);
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(3,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            c.push(m, 0x4d63a0);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(1,32,c.r[7].wrapping_add(0x4));
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.push(m, 0x4d63af);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.set(2,32,c.r[7].wrapping_add(0x8));
            c.push(m,c.get(6,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4d63be);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x40),32));
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); c.sub(a,0x2u32,32); }
            if c.cond(3) { pc = 0x4d63d3; continue; }
            pc = 0x4d63cd; continue;
            }
            0x4d63cd => {
            { let a = c.r[7].wrapping_add(0xa); wm(m,a,16,c.get(5,16)); }
            pc = 0x4d63e5; continue;
            }
            0x4d63d3 => {
            c.push(m,0x2u32);
            c.set(0,32,c.r[7].wrapping_add(0xa));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d63df);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            pc = 0x4d63e5; continue;
            }
            0x4d63e5 => {
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4d6408; continue; }
            pc = 0x4d63eb; continue;
            }
            0x4d63eb => {
            c.set(1,32,rm(m,c.r[0].wrapping_add(0xfffffffc),32));
            c.set(5,32,c.r[0].wrapping_add(0xfffffffc));
            c.push(m,0x51a880u32);
            c.push(m,c.get(1,32));
            c.push(m,0x10u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4d63ff);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x639da8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.push(m,c.get(5,32));
            c.push(m, 0x4d6405);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x4d6408; continue;
            }
            0x4d6408 => {
            c.set(3,16,rm(m,c.r[3],16));
            { let a = c.get(3,16); c.and(a,c.get(3,16),16); }
            if c.cond(6) { pc = 0x4d648a; continue; }
            pc = 0x4d6410; continue;
            }
            0x4d6410 => {
            { let a = c.get(3,32); let r = c.and(a,0xffffu32,32); c.set(3,32,r); }
            c.set(2,32,c.get(3,32));
            { let a = c.get(2,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x4u32,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m, 0x4d6424);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x4d6453; continue; }
            pc = 0x4d6437; continue;
            }
            0x4d6437 => {
            c.push(m,0x51a880u32);
            c.push(m,0x51a850u32);
            c.set(5,32,c.r[0].wrapping_add(0x4));
            c.push(m,c.get(3,32));
            c.push(m,0x10u32);
            c.push(m,c.get(5,32));
            { let a = c.r[0]; wm(m,a,32,c.get(3,32)); }
            c.push(m, 0x4d644f);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x639e9c, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            c.set(0,32,c.get(5,32));
            pc = 0x4d6455; continue;
            }
            0x4d6453 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4d6455; continue;
            }
            0x4d6455 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,32,0xffffffffu32); }
            { let a = rm(m,c.r[7].wrapping_add(0x6),16); c.sub(a,c.get(3,16),16); }
            { let a = c.r[7]; wm(m,a,32,c.get(0,32)); }
            if c.cond(6) { pc = 0x4d6490; continue; }
            pc = 0x4d6467; continue;
            }
            0x4d6467 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            pc = 0x4d6469; continue;
            }
            0x4d6469 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(1,32,rm(m,c.r[7],32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.push(m, 0x4d6478);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x51a9b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x6),16));
            { let a = c.get(5,32); let r = c.add(a,0x10u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(1,32),32); }
            c.set(6,32,c.get(0,32));
            if c.cond(2) { pc = 0x4d6469; continue; }
            pc = 0x4d6488; continue;
            }
            0x4d6488 => {
            pc = 0x4d6490; continue;
            }
            0x4d648a => {
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            pc = 0x4d6490; continue;
            }
            0x4d6490 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d80f0;
    loop {
        match pc {
            0x4d80f0 => {
            c.push(m,c.get(1,32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.push(m,0x2u32);
            c.set(0,32,c.r[7].wrapping_add(0x8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d8107);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[7].wrapping_add(0xa));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d8116);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,c.r[7].wrapping_add(0xc));
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.push(m,0x2u32);
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d8125);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x2u32,32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[5],16));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,c.r[7],32));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4d813e);
            { let args = []; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x30u32,32); c.set(4,32,r); }
            c.set(0,16,rm(m,c.r[5],16));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(3,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(8)));
            if c.cond(6) { pc = 0x4d81cb; continue; }
            pc = 0x4d8159; continue;
            }
            0x4d8159 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x4d815f; continue;
            }
            0x4d815f => {
            c.set(2,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x6),16));
            { let a = c.get(1,16); c.sub(a,0x1bu32,16); }
            if c.cond(5) { pc = 0x4d8190; continue; }
            pc = 0x4d816d; continue;
            }
            0x4d816d => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,16,rm(m,c.r[0].wrapping_add(0x12),16));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(6) { pc = 0x4d81ad; continue; }
            pc = 0x4d8177; continue;
            }
            0x4d8177 => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0xc),32));
            c.set(6,32,c.r[1].wrapping_mul(4));
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(3,32));
            c.push(m, 0x4d8189);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x63a790, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.get(3,32); let r = c.add(a,c.get(6,32),32); c.set(3,32,r); }
            pc = 0x4d81ad; continue;
            }
            0x4d8190 => {
            { let a = c.get(1,16); c.sub(a,0x34u32,16); }
            if c.cond(5) { pc = 0x4d81ad; continue; }
            pc = 0x4d8196; continue;
            }
            0x4d8196 => {
            { let a = rm(m,c.r[0].wrapping_add(0x15),8); c.sub(a,0x7u32,8); }
            if c.cond(2) { pc = 0x4d81ad; continue; }
            pc = 0x4d819c; continue;
            }
            0x4d819c => {
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x4d81ad; continue; }
            pc = 0x4d81a3; continue;
            }
            0x4d81a3 => {
            c.push(m,c.get(3,32));
            c.set(1,32,c.get(0,32));
            c.push(m, 0x4d81ab);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4f39e0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,32,c.get(0,32));
            pc = 0x4d81ad; continue;
            }
            0x4d81ad => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[5],16));
            { let a = c.get(0,32); let r = c.add(a,0x18u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x4d815f; continue; }
            pc = 0x4d81cb; continue;
            }
            0x4d81cb => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,c.get(3,32));
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_sum_merchant_debts(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d9080;
    loop {
        match pc {
            0x4d9080 => {
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.push(m,c.get(7,32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            { let a = c.r[7]; wm(m,a,32,0x0u32); }
            { let a = c.r[6]; wm(m,a,32,0x0u32); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x4d90ef; continue; }
            pc = 0x4d90a6; continue;
            }
            0x4d90a6 => {
            c.push(m,c.get(5,32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4d90ab; continue;
            }
            0x4d90ab => {
            c.set(2,32,rm(m,c.r[1],32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x6),16); c.sub(a,0x1u32,16); }
            c.set(2,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(8)));
            if c.cond(5) { pc = 0x4d90de; continue; }
            pc = 0x4d90bb; continue;
            }
            0x4d90bb => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x11),8));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4d90d0; continue; }
            pc = 0x4d90c4; continue;
            }
            0x4d90c4 => {
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = rm(m,c.r[7],32); let r = c.add(a,c.get(0,32),32); { let a = c.r[7]; wm(m,a,32,r); } }
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = rm(m,c.r[6],32); let r = c.sub(a,c.get(0,32),32); { let a = c.r[6]; wm(m,a,32,r); } }
            pc = 0x4d90de; continue;
            }
            0x4d90d0 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x10),8));
            { let a = c.get(0,32); c.sub(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4d90de; continue; }
            pc = 0x4d90d9; continue;
            }
            0x4d90d9 => {
            c.set(0,32,rm(m,c.r[2].wrapping_add(0x8),32));
            { let a = rm(m,c.r[6],32); let r = c.add(a,c.get(0,32),32); { let a = c.r[6]; wm(m,a,32,r); } }
            pc = 0x4d90de; continue;
            }
            0x4d90de => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(2) { pc = 0x4d90ab; continue; }
            pc = 0x4d90ee; continue;
            }
            0x4d90ee => {
            { let v = c.pop(m); c.set(5,32,v); }
            pc = 0x4d90ef; continue;
            }
            0x4d90ef => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_get_building_auction_info(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4dafe0;
    loop {
        match pc {
            0x4dafe0 => {
            c.push(m,c.get(1,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x8),16));
            c.push(m,c.get(3,32));
            { let a = c.r[2]; wm(m,a,32,0xffffffffu32); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.r[2]; wm(m,a,32,0xffffffffu32); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(6,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); let r = c.and(a,0xffu32,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(6,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x4db0f7; continue; }
            pc = 0x4db017; continue;
            }
            0x4db017 => {
            c.set(7,16,rm(m,c.r[4].wrapping_add(0x1c),16));
            c.set(6,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(6,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.sub(a,c.get(2,32),32); c.set(6,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            { let a = c.get(7,16); c.sub(a,rm(m,c.r[2].wrapping_add(c.r[6].wrapping_mul(8)).wrapping_add(0x776),16),16); }
            if c.cond(3) { pc = 0x4db0f7; continue; }
            pc = 0x4db038; continue;
            }
            0x4db038 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(6,16,rm(m,c.r[1].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(6,32)); }
            if c.cond(3) { pc = 0x4db0f7; continue; }
            pc = 0x4db04a; continue;
            }
            0x4db04a => {
            c.set(5,32,rm(m,c.r[1],32));
            pc = 0x4db04c; continue;
            }
            0x4db04c => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[5].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x6),16); c.sub(a,0x36u32,16); }
            c.set(7,32,c.r[5].wrapping_add(c.r[2].wrapping_mul(8)));
            if c.cond(5) { pc = 0x4db088; continue; }
            pc = 0x4db05b; continue;
            }
            0x4db05b => {
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x18),8));
            c.set(3,8,rm(m,c.r[7].wrapping_add(0x15),8));
            { let a = c.get(3,8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4db088; continue; }
            pc = 0x4db066; continue;
            }
            0x4db066 => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(2,32,c.r[7].wrapping_add(0x8));
            pc = 0x4db06b; continue;
            }
            0x4db06b => {
            c.set(3,16,rm(m,c.r[2],16));
            { let a = c.get(3,32); let r = c.and(a,0x7fffu32,32); c.set(3,32,r); }
            { let a = c.get(3,16); c.sub(a,rm(m,c.r[4].wrapping_add(0x1c),16),16); }
            if c.cond(4) { pc = 0x4db09a; continue; }
            pc = 0x4db07b; continue;
            }
            0x4db07b => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0x2u32,32); c.set(2,32,r); }
            { let a = c.get(6,32); c.sub(a,0x3u32,32); }
            if c.cond(2) { pc = 0x4db06b; continue; }
            pc = 0x4db084; continue;
            }
            0x4db084 => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4db088; continue;
            }
            0x4db088 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x4db04c; continue; }
            pc = 0x4db092; continue;
            }
            0x4db092 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            0x4db09a => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(7,32,rm(m,c.r[2].wrapping_add(c.r[5]),32));
            { let a = c.r[3]; wm(m,a,32,c.get(7,32)); }
            c.set(5,32,rm(m,c.r[1],32));
            { let a = rm(m,c.r[2].wrapping_add(c.r[5]).wrapping_add(0x14),8); c.sub(a,0x2u32,8); }
            if c.cond(7) { pc = 0x4db0ce; continue; }
            pc = 0x4db0b2; continue;
            }
            0x4db0b2 => {
            { let a = c.get(7,32); let r = c.add(a,0x100u32,32); c.set(7,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(7,32)); }
            c.set(5,32,rm(m,c.r[1],32));
            { let a = c.get(2,32); let r = c.add(a,c.get(5,32),32); c.set(2,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(0x14),8); c.sub(a,0x1u32,8); }
            if c.cond(5) { pc = 0x4db0ce; continue; }
            pc = 0x4db0c4; continue;
            }
            0x4db0c4 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.set(5,16,rm(m,c.r[2].wrapping_add(0xe),16));
            { let a = c.get(5,32); let r = c.add(a,c.get(7,32),32); c.set(5,32,r); }
            { let a = c.r[3]; wm(m,a,32,c.get(5,32)); }
            pc = 0x4db0ce; continue;
            }
            0x4db0ce => {
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(2,32,c.r[6].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,rm(m,c.r[1],32));
            c.set(2,16,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(2)).wrapping_add(0x8),16));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(2,32); let n = 0xfu32; let r = c.shr(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,8); let r = c.and(a,0x1u32,8); c.set(2,8,r); }
            c.push(m,c.get(2,32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4db0f1);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4daf10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.r[1]; wm(m,a,32,c.get(0,32)); }
            pc = 0x4db0f7; continue;
            }
            0x4db0f7 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let v = c.pop(m); c.set(1,32,v); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn assign_town_slots(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4dc800;
    loop {
        match pc {
            0x4dc800 => {
            { let a = c.get(4,32); let r = c.sub(a,0x58u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m,c.get(7,32));
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            c.push(m, 0x4dc816);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x10u32);
            c.set(0,32,c.r[4].wrapping_add(0x34));
            c.push(m,0xffu32);
            c.push(m,c.get(0,32));
            c.set(6,32,c.r[4].wrapping_add(0x3c));
            c.push(m, 0x4dc82b);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(7,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            c.set(0,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(1,32)); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,32,c.get(1,32)); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(1,32)); }
            if c.cond(14) { pc = 0x4dc8a7; continue; }
            pc = 0x4dc850; continue;
            }
            0x4dc850 => {
            c.set(1,32,rm(m,0x6de508u32,32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x82c));
            pc = 0x4dc865; continue;
            }
            0x4dc865 => {
            { let a = c.get(2,32); let r = c.sub(a,0x9f8u32,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            c.set(1,8,rm(m,c.r[2],8));
            { let a = c.get(1,32); c.sub(a,0x14u32,32); }
            if c.cond(13) { pc = 0x4dc89a; continue; }
            pc = 0x4dc875; continue;
            }
            0x4dc875 => {
            c.set(3,8,rm(m,c.r[4].wrapping_add(c.r[1]).wrapping_add(0x18),8));
            c.set(1,32,c.r[4].wrapping_add(c.r[1]).wrapping_add(0x18));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x4dc890; continue; }
            pc = 0x4dc881; continue;
            }
            0x4dc881 => {
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x40); wm(m,a,8,0x1u32); }
            { let a = c.r[6]; wm(m,a,8,0xffu32); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            pc = 0x4dc89f; continue;
            }
            0x4dc890 => {
            { let a = c.r[1]; wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x40); wm(m,a,8,0x1u32); }
            pc = 0x4dc89f; continue;
            }
            0x4dc89a => {
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x40); wm(m,a,8,0x0u32); }
            pc = 0x4dc89f; continue;
            }
            0x4dc89f => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(15) { pc = 0x4dc865; continue; }
            pc = 0x4dc8a3; continue;
            }
            0x4dc8a3 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(5,32)); }
            pc = 0x4dc8a7; continue;
            }
            0x4dc8a7 => {
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            pc = 0x4dc8b3; continue;
            }
            0x4dc8af => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x4dc8b3; continue;
            }
            0x4dc8b3 => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(c.r[3]).wrapping_add(0x18),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x4dca60; continue; }
            pc = 0x4dc8bf; continue;
            }
            0x4dc8bf => {
            { let a = c.get(5,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x4dc8ce; continue; }
            pc = 0x4dc8c4; continue;
            }
            0x4dc8c4 => {
            c.set(3,32,0x3e8u32);
            pc = 0x4dca60; continue;
            }
            0x4dc8ce => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6]; wm(m,a,8,0xffu32); }
            { let a = c.get(7,32); c.and(a,c.get(7,32),32); }
            if c.cond(6) { pc = 0x4dc934; continue; }
            pc = 0x4dc8d7; continue;
            }
            0x4dc8d7 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(5,32,c.r[2].wrapping_add(0x2c1));
            pc = 0x4dc8e3; continue;
            }
            0x4dc8e3 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[5],8));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(2,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,8,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x673730),8));
            { let a = rm(m,c.r[2].wrapping_mul(4).wrapping_add(0x6ddbc0),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4dc906; continue; }
            pc = 0x4dc8fe; continue;
            }
            0x4dc8fe => {
            c.set(1,8,rm(m,c.r[4].wrapping_add(c.r[0]).wrapping_add(0x40),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4dc913; continue; }
            pc = 0x4dc906; continue;
            }
            0x4dc906 => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x9f8u32,32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(2) { pc = 0x4dc8e3; continue; }
            pc = 0x4dc911; continue;
            }
            0x4dc911 => {
            pc = 0x4dc934; continue;
            }
            0x4dc913 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[6]; wm(m,a,8,c.get(0,8)); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x40); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(c.r[3]).wrapping_add(0x18); wm(m,a,8,0x1u32); }
            { let a = c.r[6]; wm(m,a,8,c.get(3,8)); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(2,32)); }
            if c.cond(2) { pc = 0x4dca60; continue; }
            pc = 0x4dc934; continue;
            }
            0x4dc934 => {
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            c.push(m, 0x4dc93b);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x673730),8));
            { let b = c.get(7,32); c.div32(b); }
            c.set(7,32,rm(m,0x6de508u32,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2c1),8));
            c.set(0,32,c.get(3,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbc0),8); c.sub(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x4dca3d; continue; }
            pc = 0x4dc972; continue;
            }
            0x4dc972 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4dc987; continue; }
            pc = 0x4dc97c; continue;
            }
            0x4dc97c => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(5) { pc = 0x4dc9b1; continue; }
            pc = 0x4dc980; continue;
            }
            0x4dc980 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(5,32,0x1u32);
            pc = 0x4dc987; continue;
            }
            0x4dc987 => {
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.set(3,8,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2c1),8));
            c.set(0,32,c.get(3,32));
            c.set(3,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[3].wrapping_mul(4)));
            { let a = rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbc0),8); c.sub(a,c.get(1,8),8); }
            if c.cond(5) { pc = 0x4dc972; continue; }
            pc = 0x4dc9a9; continue;
            }
            0x4dc9a9 => {
            { let a = c.get(5,32); c.and(a,c.get(5,32),32); }
            if c.cond(4) { pc = 0x4dca3d; continue; }
            pc = 0x4dc9b1; continue;
            }
            0x4dc9b1 => {
            c.set(7,32,rm(m,0x6de4b0u32,32));
            c.push(m, 0x4dc9bc);
            { let args = []; c.r[0] = host.call(m, 0x639c03, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let b = c.get(7,32); c.div32(b); }
            c.set(7,32,rm(m,0x6de508u32,32));
            c.set(1,8,rm(m,c.r[3].wrapping_mul(4).wrapping_add(0x673731),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2c1),8));
            c.set(0,32,c.get(2,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(2,8,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbc0),8));
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            if c.cond(5) { pc = 0x4dca41; continue; }
            pc = 0x4dc9fd; continue;
            }
            0x4dc9fd => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(2,32); let r = c.inc(a,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x4dca09; continue; }
            pc = 0x4dca07; continue;
            }
            0x4dca07 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            pc = 0x4dca09; continue;
            }
            0x4dca09 => {
            c.set(7,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(8)).wrapping_add(0x2c1),8));
            c.set(0,32,c.get(2,32));
            c.set(7,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[7].wrapping_mul(4)));
            c.set(2,8,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbc0),8));
            { let a = c.get(2,8); c.sub(a,c.get(1,8),8); }
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x14),32));
            if c.cond(4) { pc = 0x4dc9fd; continue; }
            pc = 0x4dca3b; continue;
            }
            0x4dca3b => {
            pc = 0x4dca41; continue;
            }
            0x4dca3d => {
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4dca41; continue;
            }
            0x4dca41 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(7,32,rm(m,0x6de4b0u32,32));
            { let a = c.r[6]; wm(m,a,8,c.get(2,8)); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.r[4].wrapping_add(c.r[2]).wrapping_add(0x40); wm(m,a,8,0x1u32); }
            { let a = c.r[4].wrapping_add(c.r[3]).wrapping_add(0x18); wm(m,a,8,0x1u32); }
            { let a = c.r[6]; wm(m,a,8,c.get(3,8)); }
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            pc = 0x4dca60; continue;
            }
            0x4dca60 => {
            { let a = c.get(3,32); let r = c.inc(a,32); c.set(3,32,r); }
            { let a = c.get(3,32); c.sub(a,0x14u32,32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(3,32)); }
            if c.cond(12) { pc = 0x4dc8af; continue; }
            pc = 0x4dca6e; continue;
            }
            0x4dca6e => {
            c.set(1,32,c.r[4].wrapping_add(0x2c));
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,32,0x79u32); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6df2f0u32);
            c.push(m, 0x4dca85);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x58u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_find_celebration_in_town(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4e2314;
    loop {
        match pc {
            0x4e2314 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(5,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x8),16));
            c.set(6,16,rm(m,c.r[5].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            c.push(m,c.get(7,32));
            if c.cond(3) { pc = 0x4e238b; continue; }
            pc = 0x4e232a; continue;
            }
            0x4e232a => {
            c.set(7,32,rm(m,c.r[5],32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            pc = 0x4e2331; continue;
            }
            0x4e2331 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x6),16); c.sub(a,0x7u32,16); }
            c.set(2,32,c.r[7].wrapping_add(c.r[2].wrapping_mul(8)));
            if c.cond(5) { pc = 0x4e2352; continue; }
            pc = 0x4e233f; continue;
            }
            0x4e233f => {
            c.set(3,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.and(a,0xffu32,32); c.set(3,32,r); }
            { let a = rm(m,c.r[2].wrapping_add(0xc),32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x4e2352; continue; }
            pc = 0x4e234c; continue;
            }
            0x4e234c => {
            { let a = rm(m,c.r[2].wrapping_add(0x10),32); c.sub(a,0x2u32,32); }
            if c.cond(4) { pc = 0x4e236f; continue; }
            pc = 0x4e2352; continue;
            }
            0x4e2352 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(6,32),32); }
            if c.cond(2) { pc = 0x4e2331; continue; }
            pc = 0x4e235c; continue;
            }
            0x4e235c => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0xc),16));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x4e236f => {
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(c.r[2]).wrapping_add(0x8),8));
            { let a = c.r[6]; wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[5],32));
            c.set(2,32,rm(m,c.r[2].wrapping_add(c.r[1]),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.r[1]; wm(m,a,32,c.get(2,32)); }
            pc = 0x4e238b; continue;
            }
            0x4e238b => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let v = c.pop(m); c.set(7,32,v); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0xc),16));
            { let v = c.pop(m); c.set(6,32,v); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn st_resolve_commissioned_burglary(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4e5664;
    loop {
        match pc {
            0x4e5664 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662c14u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x20u32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(1,32,rm(m,c.r[7],32));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(3,32,c.r[1].wrapping_add(c.r[0].wrapping_mul(8)));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4e5697);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6de4b0u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xb),8));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0x0u32); }
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x4e5b55; continue; }
            pc = 0x4e56b2; continue;
            }
            0x4e56b2 => {
            c.set(1,8,rm(m,c.r[3].wrapping_add(0xc),8));
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(2,8,c.get(1,8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4e5b55; continue; }
            pc = 0x4e56c7; continue;
            }
            0x4e56c7 => {
            { let a = c.get(1,8); c.sub(a,rm(m,c.r[3].wrapping_add(0xa),8),8); }
            if c.cond(4) { pc = 0x4e5b55; continue; }
            pc = 0x4e56d0; continue;
            }
            0x4e56d0 => {
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,0x0u32);
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            { let a = c.r[0].wrapping_add(0x83c); wm(m,a,8,0x0u32); }
            c.set(0,16,rm(m,c.r[3].wrapping_add(0xb),8));
            c.set(1,16,rm(m,c.r[3].wrapping_add(0xc),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4e5702);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x4e5711; continue; }
            pc = 0x4e570a; continue;
            }
            0x4e570a => {
            c.set(6,32,0x19au32);
            pc = 0x4e577b; continue;
            }
            0x4e5711 => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2ec),16));
            c.set(1,8,rm(m,0x6734e4u32,8));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,0x6734e5u32,8));
            c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };
            { let b = c.get(1,32); c.idiv32(b); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(2,8,rm(m,c.r[3].wrapping_add(0xc),8));
            c.push(m,c.get(2,32));
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            c.set(6,32,sx(c.get(0,16),16)&0xffffffffu32);
            c.push(m, 0x4e5745);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[0].wrapping_add(0x8),16); c.sub(a,0x0u32,16); }
            if c.cond(4) { pc = 0x4e5756; continue; }
            pc = 0x4e574c; continue;
            }
            0x4e574c => {
            { let a = c.get(6,32); c.sub(a,0x1u32,32); }
            if c.cond(7) { pc = 0x4e5756; continue; }
            pc = 0x4e5751; continue;
            }
            0x4e5751 => {
            c.set(6,32,0x2u32);
            pc = 0x4e5756; continue;
            }
            0x4e5756 => {
            c.set(0,16,rm(m,c.r[3].wrapping_add(0x8),16));
            c.set(1,32,c.r[6].wrapping_add(0xffffffff));
            { let a = c.get(0,32); let r = c.and(a,0x3ffu32,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.imul(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x2c7),8));
            c.set(1,32,c.get(2,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(6,32),32); c.set(1,32,r); }
            { let b = c.get(1,32); c.div32(b); }
            c.set(6,32,c.get(0,32));
            pc = 0x4e577b; continue;
            }
            0x4e577b => {
            c.set(2,16,rm(m,c.r[3].wrapping_add(0xa),8));
            { let a = c.get(2,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(2) { pc = 0x4e57e8; continue; }
            pc = 0x4e5789; continue;
            }
            0x4e5789 => {
            { let a = c.get(6,32); c.sub(a,0x66u32,32); }
            { let a = c.r[3].wrapping_add(0xa); wm(m,a,8,0x0u32); }
            if c.cond(6) { pc = 0x4e579e; continue; }
            pc = 0x4e5792; continue;
            }
            0x4e5792 => {
            { let a = c.get(6,32); c.sub(a,0x19au32,32); }
            if c.cond(7) { pc = 0x4e5957; continue; }
            pc = 0x4e579e; continue;
            }
            0x4e579e => {
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x16u32); }
            c.set(2,8,rm(m,c.r[3].wrapping_add(0xb),8));
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,c.get(2,8)); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xc),8));
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(1,8,c.get(0,8));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6de4aau32,16),16); }
            if c.cond(3) { pc = 0x4e592f; continue; }
            pc = 0x4e57bf; continue;
            }
            0x4e57bf => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x4e57cf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x1c),8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,16,c.get(2,16)); }
            c.set(0,16,rm(m,c.r[0].wrapping_add(0x1d),8));
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,c.get(0,16)); }
            pc = 0x4e593d; continue;
            }
            0x4e57e8 => {
            { let a = c.get(6,32); c.sub(a,0x66u32,32); }
            if c.cond(7) { pc = 0x4e5792; continue; }
            pc = 0x4e57ed; continue;
            }
            0x4e57ed => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xc),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4e57fd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[3].wrapping_add(0xa),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4e580f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0x20u32,32); c.set(6,32,r); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,0x0u32); }
            c.set(1,32,rm(m,c.r[3],32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(6,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(1,32),32); c.set(6,32,r); }
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x4e582c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,0x1u32); }
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x8),16));
            c.set(2,32,c.r[1].wrapping_add(c.r[1].wrapping_mul(2)));
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x5u32); }
            c.set(6,32,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0xa),8));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,8,c.get(2,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xb),8));
            { let a = c.r[0].wrapping_add(0x9); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xc),8));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[0].wrapping_add(0xa); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(0x19),8));
            { let a = c.r[0].wrapping_add(0xb); wm(m,a,8,c.get(2,8)); }
            c.set(2,32,rm(m,c.r[6],32));
            { let a = c.r[0].wrapping_add(0xc); wm(m,a,32,c.get(2,32)); }
            { let a = c.r[0].wrapping_add(0x10); wm(m,a,8,0x3u32); }
            { let a = c.r[0].wrapping_add(0x11); wm(m,a,8,0x0u32); }
            c.set(7,16,rm(m,c.r[6].wrapping_add(0x8),16));
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x9),8));
            { let a = c.get(7,16); let n = 0x8u32; let r = c.shl(a,n,16); c.set(7,16,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x12); wm(m,a,16,c.get(2,16)); }
            c.set(2,16,rm(m,c.r[6].wrapping_add(0x8),16));
            { let a = c.get(2,16); let r = c.imul(a,rm(m,c.r[6],16),16); c.set(2,16,r); }
            { let a = c.r[0].wrapping_add(0x14); wm(m,a,16,c.get(2,16)); }
            { let a = c.r[0].wrapping_add(0x16); wm(m,a,16,0xffffu32); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x74u32); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xb),8));
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,c.get(0,8)); }
            c.set(2,16,rm(m,c.r[5].wrapping_add(0x1c),8));
            c.set(0,16,rm(m,c.r[5].wrapping_add(0x1d),8));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,16,c.get(2,16)); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x19),8));
            c.set(0,16,rm(m,c.r[2].wrapping_add(c.r[5]).wrapping_add(0x39c),8));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,c.get(0,16)); }
            c.set(0,8,rm(m,c.r[2].wrapping_add(0x2c7),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(6) { pc = 0x4e58dc; continue; }
            pc = 0x4e58d7; continue;
            }
            0x4e58d7 => {
            { let a = rm(m,c.r[4].wrapping_add(0x2b),8); let r = c.or(a,0x1u32,8); { let a = c.r[4].wrapping_add(0x2b); wm(m,a,8,r); } }
            pc = 0x4e58dc; continue;
            }
            0x4e58dc => {
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x1c),8));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x1d),8));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x19),8));
            c.set(3,8,rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x39c),8));
            c.push(m, 0x4e5901);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            { let a = c.get(2,16); let r = c.xor(a,c.get(2,16),16); c.set(2,16,r); }
            c.set(1,8,c.get(0,8));
            c.set(2,8,c.get(3,8));
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,c.get(1,16)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0xc),8));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4e592a);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4e5b31; continue;
            }
            0x4e592f => {
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,16,0x14u32); }
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,0x28u32); }
            pc = 0x4e593d; continue;
            }
            0x4e593d => {
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x20));
            c.set(2,8,rm(m,c.r[3].wrapping_add(0xa),8));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4e5952);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4e5b31; continue;
            }
            0x4e5957 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xc),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4e5967);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(7,32,c.get(0,32));
            c.set(1,8,rm(m,c.r[3].wrapping_add(0xa),8));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4e5979);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,8,rm(m,c.r[3].wrapping_add(0xb),8));
            c.set(5,32,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x25); wm(m,a,8,c.get(2,8)); }
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x1c),8));
            c.set(1,16,rm(m,c.r[7].wrapping_add(0x1d),8));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,c.get(0,16)); }
            { let a = c.r[4].wrapping_add(0x2c); wm(m,a,16,c.get(0,16)); }
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(1,32); c.sub(a,0x64u32,32); }
            if c.cond(14) { pc = 0x4e5a0d; continue; }
            pc = 0x4e59a9; continue;
            }
            0x4e59a9 => {
            { let a = c.get(6,32); let r = c.and(a,0x7u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x3u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,0x8u32,32); }
            if c.cond(14) { pc = 0x4e59b9; continue; }
            pc = 0x4e59b4; continue;
            }
            0x4e59b4 => {
            c.set(6,32,0x8u32);
            pc = 0x4e59b9; continue;
            }
            0x4e59b9 => {
            c.set(2,32,c.get(1,32));
            c.set(0,32,0x51eb851fu32);
            { let a = c.get(2,32); let r = c.imul(a,c.get(6,32),32); c.set(2,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(2,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,0xc350u32,32); }
            if c.cond(14) { pc = 0x4e59dc; continue; }
            pc = 0x4e59d7; continue;
            }
            0x4e59d7 => {
            c.set(2,32,0xc350u32);
            pc = 0x4e59dc; continue;
            }
            0x4e59dc => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4bc),32));
            { let a = c.get(1,32); let r = c.sub(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[7]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[7].wrapping_add(0x4bc); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,c.r[5],32));
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x4b8),32));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(2,32),32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(1,32)); }
            { let a = c.r[5].wrapping_add(0x4b8); wm(m,a,32,c.get(0,32)); }
            pc = 0x4e5a12; continue;
            }
            0x4e5a0d => {
            { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,c.get(0,16)); }
            pc = 0x4e5a12; continue;
            }
            0x4e5a12 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4e5a1b);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x3c); wm(m,a,8,0x1u32); }
            c.push(m, 0x4e5a2f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            c.push(m, 0x4e5a38);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x10),32));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(5,32,c.r[2].wrapping_add(0x4));
            pc = 0x4e5a45; continue;
            }
            0x4e5a45 => {
            c.set(7,32,rm(m,c.r[5],32));
            { let a = c.get(7,32); c.sub(a,0xc8u32,32); }
            if c.cond(12) { pc = 0x4e5add; continue; }
            pc = 0x4e5a54; continue;
            }
            0x4e5a54 => {
            c.set(0,32,0x51eb851fu32);
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let (lo,hi) = c.imul1(c.r[0],c.get(7,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(6,32); let r = c.and(a,0x1fu32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0xfu32,32); c.set(6,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x672c14),8));
            { let a = c.get(6,32); let r = c.imul(a,c.get(2,32),32); c.set(6,32,r); }
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x4e5a91; continue; }
            pc = 0x4e5a7c; continue;
            }
            0x4e5a7c => {
            c.set(0,32,0x51eb851fu32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x6u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            pc = 0x4e5aa7; continue;
            }
            0x4e5a91 => {
            c.set(0,32,0x10624dd3u32);
            { let (lo,hi) = c.imul1(c.r[0],c.get(6,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x7u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let r = c.add(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x4e5aa7; continue;
            }
            0x4e5aa7 => {
            { let a = c.get(0,32); c.sub(a,0x3e8u32,32); }
            if c.cond(14) { pc = 0x4e5ab5; continue; }
            pc = 0x4e5aae; continue;
            }
            0x4e5aae => {
            c.set(0,32,0x3e8u32);
            pc = 0x4e5ab9; continue;
            }
            0x4e5ab5 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(14) { pc = 0x4e5ad8; continue; }
            pc = 0x4e5ab9; continue;
            }
            0x4e5ab9 => {
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(2,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(7,32); let r = c.sub(a,c.get(2,32),32); c.set(7,32,r); }
            c.set(2,32,0x1u32);
            { let a = c.get(2,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.r[5]; wm(m,a,32,c.get(7,32)); }
            { let a = rm(m,c.r[4].wrapping_add(0x2a),16); let r = c.or(a,c.get(2,16),16); { let a = c.r[4].wrapping_add(0x2a); wm(m,a,16,r); } }
            { let a = rm(m,c.r[4].wrapping_add(0x2c),16); let r = c.add(a,c.get(0,16),16); { let a = c.r[4].wrapping_add(0x2c); wm(m,a,16,r); } }
            pc = 0x4e5ad8; continue;
            }
            0x4e5ad8 => {
            { let a = rm(m,c.r[4].wrapping_add(0x14),32); let n = 0x2u32; let r = c.shr(a,n,32); { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,r); } }
            pc = 0x4e5add; continue;
            }
            0x4e5add => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(1,32); c.sub(a,0x10u32,32); }
            if c.cond(12) { pc = 0x4e5a45; continue; }
            pc = 0x4e5aea; continue;
            }
            0x4e5aea => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x20u32); }
            c.set(1,8,rm(m,c.r[3].wrapping_add(0xc),8));
            c.set(0,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4e5b04);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            { let a = rm(m,c.r[4].wrapping_add(0x2e),16); let n = 0x1u32; let r = c.shr(a,n,16); { let a = c.r[4].wrapping_add(0x2e); wm(m,a,16,r); } }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x24); wm(m,a,8,0x15u32); }
            c.set(0,8,rm(m,c.r[3].wrapping_add(0xa),8));
            c.set(2,32,c.r[4].wrapping_add(0x20));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4e5b23);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,8,0x0u32); }
            c.push(m, 0x4e5b31);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x4e5b31; continue;
            }
            0x4e5b31 => {
            c.set(1,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x38); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4e5b42);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            0x4e5b55 => {
            c.set(1,32,c.r[4].wrapping_add(0x18));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4e5b66);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x2cu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn scheduled_tasks_alderman_mission_start(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ebf54;
    loop {
        match pc {
            0x4ebf54 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x662dacu32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            { let a = c.get(4,32); let r = c.sub(a,0x28u32,32); c.set(4,32,r); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x38),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(5,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.set(6,32,rm(m,c.r[7],32));
            { let a = c.get(5,32); let n = 0x3u32; let r = c.shl(a,n,32); c.set(5,32,r); }
            { let a = c.get(6,32); let r = c.add(a,c.get(5,32),32); c.set(6,32,r); }
            c.push(m, 0x4ebf89);
            { let args = []; c.r[0] = host.call(m, 0x51a850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xf),8));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            { let a = c.get(0,8); c.sub(a,c.get(3,8),8); }
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,c.get(3,32)); }
            if c.cond(5) { pc = 0x4ec30c; continue; }
            pc = 0x4ebf9a; continue;
            }
            0x4ebf9a => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xf); wm(m,a,8,0x1u32); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xc),8));
            { let a = c.get(0,32); c.sub(a,0x4u32,32); }
            if c.cond(7) { pc = 0x4ec30c; continue; }
            pc = 0x4ebfac; continue;
            }
            0x4ebfac => {
            pc = m.r32(c.r[0].wrapping_mul(4).wrapping_add(0x4ec334)); continue;
            }
            0x4ebfb3 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x10),32));
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x14),8));
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x15),8));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            c.set(0,8,rm(m,0x6de4b0u32,8));
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x32); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[6].wrapping_add(0x16); wm(m,a,8,c.get(0,8)); }
            c.set(0,32,c.r[4].wrapping_add(0x24));
            c.set(1,32,0x6de4a0u32);
            c.push(m,c.get(0,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[4].wrapping_add(0x35); wm(m,a,8,c.get(2,8)); }
            { let a = c.r[4].wrapping_add(0x37); wm(m,a,8,c.get(3,8)); }
            c.push(m, 0x4ebfef);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x531d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x32),32));
            c.set(1,32,rm(m,c.r[7],32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(5,32),32); c.set(1,32,r); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,8,rm(m,c.r[1].wrapping_add(0xd),8));
            c.set(1,32,rm(m,0x6de508u32,32));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(8)).wrapping_add(0x6f1); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(3,32)); }
            c.set(0,32,rm(m,0x6de4b4u32,32));
            c.set(1,32,0x6dd73cu32);
            { let a = c.get(0,32); let r = c.add(a,0x100u32,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x4ec02c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4d8cf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,0x1u32); }
            { let a = c.r[0].wrapping_add(0x6); wm(m,a,16,0x39u32); }
            c.set(2,8,rm(m,c.r[4].wrapping_add(0x32),8));
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,8,c.get(2,8)); }
            pc = 0x4ec30c; continue;
            }
            0x4ec045 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x10),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)));
            c.set(1,32,0x8000u32);
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x76c),32));
            { let a = c.get(2,32); let r = c.or(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.r[0].wrapping_add(0x76c); wm(m,a,32,c.get(2,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x11),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(2,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.sub(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,rm(m,0x6de508u32,32));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(8)));
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x76c),32));
            { let a = c.get(2,32); let r = c.or(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            { let a = c.r[0].wrapping_add(0x76c); wm(m,a,32,c.get(2,32)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0x10),8));
            c.set(7,32,c.get(1,32));
            c.set(5,32,c.get(2,32));
            pc = 0x4ec0a1; continue;
            }
            0x4ec0a1 => {
            c.set(1,8,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x672bf8),8));
            c.set(2,8,rm(m,c.r[5].wrapping_add(0x6de4b8),8));
            { let a = c.get(1,8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4ec0c1; continue; }
            pc = 0x4ec0b2; continue;
            }
            0x4ec0b2 => {
            c.set(3,8,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x672bf9),8));
            { let a = c.get(3,8); c.sub(a,rm(m,c.r[7].wrapping_add(0x6de4b8),8),8); }
            if c.cond(4) { pc = 0x4ec0d2; continue; }
            pc = 0x4ec0c1; continue;
            }
            0x4ec0c1 => {
            { let a = c.get(1,8); c.sub(a,rm(m,c.r[7].wrapping_add(0x6de4b8),8),8); }
            if c.cond(5) { pc = 0x4ec0fb; continue; }
            pc = 0x4ec0c9; continue;
            }
            0x4ec0c9 => {
            { let a = rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x672bf9),8); c.sub(a,c.get(2,8),8); }
            if c.cond(5) { pc = 0x4ec0fb; continue; }
            pc = 0x4ec0d2; continue;
            }
            0x4ec0d2 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x672bfa),8));
            c.set(0,32,c.get(1,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(2,32); let n = 0x1u32; let r = c.shr(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x14));
            { let a = c.get(0,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x3e8u32);
            pc = 0x4ec0fb; continue;
            }
            0x4ec0fb => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x7u32,32); }
            if c.cond(2) { pc = 0x4ec0a1; continue; }
            pc = 0x4ec101; continue;
            }
            0x4ec101 => {
            { let a = c.get(0,32); c.sub(a,0x3e8u32,32); }
            if c.cond(3) { pc = 0x4ec115; continue; }
            pc = 0x4ec108; continue;
            }
            0x4ec108 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.add(a,0x26c00u32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x4ec115; continue;
            }
            0x4ec115 => {
            c.set(0,8,rm(m,0x6de52eu32,8));
            c.set(1,32,c.r[4].wrapping_add(0x19));
            c.set(2,32,c.r[4].wrapping_add(0x18));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6de4a0u32);
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,0x80u32); }
            { let a = c.r[4].wrapping_add(0x1d); wm(m,a,8,c.get(0,8)); }
            c.push(m, 0x4ec137);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5333c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x10),8));
            { let a = c.r[4].wrapping_add(0x1a); wm(m,a,8,c.get(0,8)); }
            c.set(1,8,rm(m,c.r[6].wrapping_add(0x11),8));
            { let a = c.r[4].wrapping_add(0x1b); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(0,32,c.r[4].wrapping_add(0x1e));
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,c.r[4].wrapping_add(0x48));
            c.push(m,c.get(0,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,c.r[4].wrapping_add(0x21));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ec169);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5311d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1e),32));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x48),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[2].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0xffffffff));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(0,16)); }
            c.set(2,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.push(m,c.get(2,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4ec194);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4ec30c; continue;
            }
            0x4ec199 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.add(a,0x1e00u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.or(a,0xffu32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            pc = 0x4ec30c; continue;
            }
            0x4ec1ad => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(1,32,0x6de4a0u32);
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.push(m,c.get(0,32));
            c.push(m, 0x4ec1bd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(7,32,c.get(0,32));
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.add(a,0x1e00u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.or(a,0xffu32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0xffffu32);
            { let a = c.r[6].wrapping_add(0x14); wm(m,a,16,c.get(0,16)); }
            c.set(1,8,rm(m,0x6de52eu32,8));
            { let a = c.r[4].wrapping_add(0x1a); wm(m,a,16,c.get(0,16)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x6eu32); }
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(1,8)); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(3,16)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x11),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,16,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbb0),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,c.get(1,16)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x11),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.set(1,16,rm(m,c.r[0].wrapping_mul(4).wrapping_add(0x6ddbb4),16));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x22); wm(m,a,16,c.get(1,16)); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4ec235);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,8,rm(m,0x6de52eu32,8));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x84u32); }
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(1,8)); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x19),8));
            c.set(1,32,c.get(7,32));
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[7]).wrapping_add(0x39c),8));
            c.push(m, 0x4ec257);
            { let args = []; c.r[0] = host.call(m, 0x4f9430, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(1,16); let r = c.xor(a,c.get(1,16),16); c.set(1,16,r); }
            c.set(0,16,c.get(0,8));
            c.set(1,8,c.get(3,8));
            { let a = c.get(1,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,c.r[4].wrapping_add(0x1e));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,16,c.get(0,16)); }
            c.push(m,c.get(1,32));
            c.set(2,16,rm(m,c.r[7].wrapping_add(0x1c),8));
            c.set(0,16,rm(m,c.r[7].wrapping_add(0x1d),8));
            { let a = c.get(2,32); let n = 0x8u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[4].wrapping_add(0x20));
            { let a = c.r[4].wrapping_add(0x1e); wm(m,a,16,c.get(2,16)); }
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(2,32,c.r[4].wrapping_add(0x21));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x4ec29b);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, 0x5311d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20);
            { let a = rm(m,c.r[4].wrapping_add(0x1c),16); let r = c.add(a,0xff00u32,16); { let a = c.r[4].wrapping_add(0x1c); wm(m,a,16,r); } }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0xd),8));
            c.set(2,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(2,32));
            c.push(m,c.get(0,32));
            c.set(1,32,0x6dd730u32);
            c.push(m, 0x4ec2b7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4d6530, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x4ec30c; continue;
            }
            0x4ec2b9 => {
            c.set(0,32,rm(m,0x6de4b4u32,32));
            { let a = c.get(0,32); let r = c.add(a,0x16d00u32,32); c.set(0,32,r); }
            { let a = c.get(0,8); let r = c.or(a,0xffu32,8); c.set(0,8,r); }
            { let a = c.r[6].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[6].wrapping_add(0x12),8));
            c.set(2,32,rm(m,0x6de508u32,32));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x6u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.sub(a,c.get(0,32),32); c.set(1,32,r); }
            c.set(0,32,0x51eb851fu32);
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[1].wrapping_mul(8)).wrapping_add(0x2d4),16));
            { let a = c.get(1,16); let r = c.add(a,0x7d0u32,16); c.set(1,16,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let (lo,hi) = c.imul1(c.r[0],c.get(1,32),32); c.r[0]=lo; c.r[2]=hi; }
            { let a = c.get(2,32); let n = 0x5u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); let n = 0x1fu32; let r = c.shr(a,n,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,c.get(0,32),32); c.set(2,32,r); }
            c.set(0,32,c.r[2].wrapping_add(c.r[2].wrapping_mul(4)));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.get(1,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.r[6].wrapping_add(0x10); wm(m,a,16,c.get(1,16)); }
            pc = 0x4ec30c; continue;
            }
            0x4ec30c => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            { let a = c.r[4].wrapping_add(0x40); wm(m,a,32,0xffffffffu32); }
            c.push(m, 0x4ec31d);
            { let args = []; c.r[0] = host.call(m, 0x51a880, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x34u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn op_assign_merchant_to_player_slot(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5421e0;
    loop {
        match pc {
            0x5421e0 => {
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,16,rm(m,0x6de4aau32,16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x542370; continue; }
            pc = 0x5421f7; continue;
            }
            0x5421f7 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x542202);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.r[0].wrapping_add(0x8); wm(m,a,16,0x0u32); }
            c.set(0,8,rm(m,0x6df2f0u32,8));
            { let a = c.get(0,8); c.and(a,0xeu32,8); }
            if c.cond(4) { pc = 0x5422d1; continue; }
            pc = 0x542215; continue;
            }
            0x542215 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x542224);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43f660, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x5422d1; continue; }
            pc = 0x54222c; continue;
            }
            0x54222c => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x1d4),32));
            { let a = c.get(0,32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x542253; continue; }
            pc = 0x542241; continue;
            }
            0x542241 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x54224c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(0,32); let r = c.add(a,0x8u32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(0x1),8); let r = c.or(a,0x80u32,8); { let a = c.r[0].wrapping_add(0x1); wm(m,a,8,r); } }
            pc = 0x542253; continue;
            }
            0x542253 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.push(m,c.get(7,32));
            { let a = c.r[1].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1d4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[1].wrapping_add(0x210),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x54227c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43f7c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            { let a = c.r[1].wrapping_add(0x210); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,rm(m,0x6cbb94u32,32));
            { let a = rm(m,c.r[1].wrapping_add(0x210),32); c.sub(a,0xffffffffu32,32); }
            if c.cond(4) { pc = 0x5422ca; continue; }
            pc = 0x542297; continue;
            }
            0x542297 => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5422a0);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43f2b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,0x6df31cu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5422b9; continue; }
            pc = 0x5422ad; continue;
            }
            0x5422ad => {
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5422b9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x43f470, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5422b9; continue;
            }
            0x5422b9 => {
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.push(m, 0x5422c4);
            { let args = []; c.r[0] = host.call(m, 0x43e760, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            pc = 0x5422ca; continue;
            }
            0x5422ca => {
            { let a = c.r[1].wrapping_add(0x210); wm(m,a,32,c.get(7,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            pc = 0x5422d1; continue;
            }
            0x5422d1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x4),32));
            c.set(1,32,rm(m,0x6df31cu32,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x542370; continue; }
            pc = 0x5422e2; continue;
            }
            0x5422e2 => {
            c.set(0,32,rm(m,0x6cbdd8u32,32));
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = 0x6dfc14u32; wm(m,a,32,c.get(1,32)); }
            if c.cond(4) { pc = 0x542370; continue; }
            pc = 0x5422f4; continue;
            }
            0x5422f4 => {
            c.set(2,32,rm(m,0x6de4b0u32,32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x28cc),32));
            c.push(m,c.get(2,32));
            c.push(m,0x0u32);
            c.push(m,c.get(0,32));
            c.push(m, 0x542309);
            { let args = []; c.r[0] = host.call(m, 0x639f20, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[6].wrapping_add(0x8),32));
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x54231a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6de4a8u32,32));
            c.set(2,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[2].wrapping_add(0xc),16));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(3) { pc = 0x542370; continue; }
            pc = 0x542332; continue;
            }
            0x542332 => {
            c.push(m,c.get(0,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x54233d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x530400, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[0].wrapping_add(0x2c6),8));
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x28cc),32));
            { let a = c.r[2].wrapping_add(c.r[1]); wm(m,a,8,0x1u32); }
            c.set(1,32,rm(m,0x6de4a8u32,32));
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            c.set(2,16,rm(m,c.r[0].wrapping_add(0x2c8),16));
            c.set(0,32,c.get(2,32));
            { let a = c.get(0,32); c.sub(a,c.get(1,32),32); }
            if c.cond(2) { pc = 0x542332; continue; }
            pc = 0x542370; continue;
            }
            0x542370 => {
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn custom_map_save_close_file_tail(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x544d4f;
    loop {
        match pc {
            0x544d4f => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x3u32); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x6u32); }
            c.push(m, 0x544d62);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x544d79, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn operations_save_game_finish_main_file(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x547534;
    loop {
        match pc {
            0x547534 => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x4u32); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x8u32); }
            c.push(m, 0x547547);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let args = []; c.r[0] = host.call(m, 0x547566, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn operations_save_game_finish_second_file(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x54762f;
    loop {
        match pc {
            0x54762f => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x4u32); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffc8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xdu32); }
            c.push(m, 0x547642);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let args = []; c.r[0] = host.call(m, 0x547650, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn operations_send_lobby_profile(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x547fd0;
    loop {
        match pc {
            0x547fd0 => {
            { let a = c.get(4,32); let r = c.sub(a,0x14u32,32); c.set(4,32,r); }
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = rm(m,c.r[7],32); c.sub(a,0x2u32,32); }
            if c.cond(4) { pc = 0x548150; continue; }
            pc = 0x547fdf; continue;
            }
            0x547fdf => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m, 0x547feb);
            { let args = []; c.r[0] = host.call(m, 0x535740, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x2c),8));
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(5) { pc = 0x548051; continue; }
            pc = 0x547ff2; continue;
            }
            0x547ff2 => {
            c.set(2,32,rm(m,0x6cbb94u32,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xa4u32); }
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            pc = 0x548004; continue;
            }
            0x548004 => {
            c.set(3,8,rm(m,c.r[2].wrapping_add(c.r[1]).wrapping_add(0x204),8));
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x548018; continue; }
            pc = 0x54800f; continue;
            }
            0x54800f => {
            c.set(6,32,0x1u32);
            { let a = c.get(6,32); let n = c.get(1,8); let r = c.shl(a,n,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.or(a,c.get(6,32),32); c.set(0,32,r); }
            pc = 0x548018; continue;
            }
            0x548018 => {
            { let a = c.get(1,32); let r = c.inc(a,32); c.set(1,32,r); }
            { let a = c.get(1,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x548004; continue; }
            pc = 0x54801e; continue;
            }
            0x54801e => {
            c.set(6,32,0x34u32);
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            { let a = rm(m,c.r[7].wrapping_add(0x56),16); c.sub(a,c.get(6,16),16); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,0x0u32); }
            if c.cond(2) { pc = 0x548043; continue; }
            pc = 0x548035; continue;
            }
            0x548035 => {
            c.push(m,0xau32);
            { let target = rm(m,0x66a2bcu32,32); c.push(m, 0x54803d); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[7].wrapping_add(0x56),16); c.sub(a,c.get(6,16),16); }
            if c.cond(3) { pc = 0x548035; continue; }
            pc = 0x548043; continue;
            }
            0x548043 => {
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x54804f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x548056; continue;
            }
            0x548051 => {
            c.set(6,32,0x34u32);
            pc = 0x548056; continue;
            }
            0x548056 => {
            c.set(0,32,rm(m,0x6cbb94u32,32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x20c),8));
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(4) { pc = 0x5480ad; continue; }
            pc = 0x548065; continue;
            }
            0x548065 => {
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xa8u32); }
            c.set(1,32,rm(m,c.r[0].wrapping_add(0x4cb0),32));
            { let a = rm(m,c.r[7].wrapping_add(0x56),16); c.sub(a,c.get(6,16),16); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(1,32)); }
            c.set(2,32,rm(m,c.r[0].wrapping_add(0x4cb0),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)).wrapping_add(0x1d4),32));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(2) { pc = 0x54809c; continue; }
            pc = 0x54808e; continue;
            }
            0x54808e => {
            c.push(m,0xau32);
            { let target = rm(m,0x66a2bcu32,32); c.push(m, 0x548096); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[7].wrapping_add(0x56),16); c.sub(a,c.get(6,16),16); }
            if c.cond(3) { pc = 0x54808e; continue; }
            pc = 0x54809c; continue;
            }
            0x54809c => {
            c.set(1,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5480a8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(0,32,rm(m,0x6cbb94u32,32));
            pc = 0x5480ad; continue;
            }
            0x5480ad => {
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x2c),8));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0xa2u32); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(2,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1c4),8));
            { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1cc),8));
            { let a = c.r[4].wrapping_add(0x16); wm(m,a,8,c.get(2,8)); }
            c.set(2,8,rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x1fc),8));
            { let a = c.get(2,8); c.and(a,c.get(2,8),8); }
            if c.cond(4) { pc = 0x5480ec; continue; }
            pc = 0x5480e7; continue;
            }
            0x5480e7 => {
            { let a = rm(m,c.r[4].wrapping_add(0x15),8); let r = c.or(a,0x80u32,8); { let a = c.r[4].wrapping_add(0x15); wm(m,a,8,r); } }
            pc = 0x5480ec; continue;
            }
            0x5480ec => {
            c.set(3,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)).wrapping_add(0x164));
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            c.set(0,32,rm(m,c.r[3],32));
            c.set(5,32,rm(m,c.r[0].wrapping_add(0xfffffff8),32));
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,0x0u32); }
            pc = 0x5480ff; continue;
            }
            0x5480ff => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x548101; continue;
            }
            0x548101 => {
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            if c.cond(3) { pc = 0x548110; continue; }
            pc = 0x548105; continue;
            }
            0x548105 => {
            c.set(1,32,rm(m,c.r[3],32));
            c.set(2,8,rm(m,c.r[6].wrapping_add(c.r[1]),8));
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x18); wm(m,a,8,c.get(2,8)); }
            pc = 0x548115; continue;
            }
            0x548110 => {
            { let a = c.r[4].wrapping_add(c.r[0]).wrapping_add(0x18); wm(m,a,8,0x0u32); }
            pc = 0x548115; continue;
            }
            0x548115 => {
            { let a = c.get(6,32); let r = c.inc(a,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0xcu32,32); }
            if c.cond(2) { pc = 0x548101; continue; }
            pc = 0x54811c; continue;
            }
            0x54811c => {
            { let a = rm(m,c.r[7].wrapping_add(0x56),16); c.sub(a,0x34u32,16); }
            if c.cond(2) { pc = 0x548132; continue; }
            pc = 0x548123; continue;
            }
            0x548123 => {
            c.push(m,0xau32);
            { let target = rm(m,0x66a2bcu32,32); c.push(m, 0x54812b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = rm(m,c.r[7].wrapping_add(0x56),16); c.sub(a,0x34u32,16); }
            if c.cond(3) { pc = 0x548123; continue; }
            pc = 0x548132; continue;
            }
            0x548132 => {
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.set(1,32,c.get(7,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x54813e);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x54aa70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,8,rm(m,c.r[4].wrapping_add(0x17),8));
            { let a = c.get(1,8); let r = c.add(a,0xcu32,8); c.set(1,8,r); }
            { let a = c.get(6,32); c.sub(a,c.get(5,32),32); }
            { let a = c.r[4].wrapping_add(0x17); wm(m,a,8,c.get(1,8)); }
            if c.cond(6) { pc = 0x5480ff; continue; }
            pc = 0x54814d; continue;
            }
            0x54814d => {
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            pc = 0x548150; continue;
            }
            0x548150 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0x14u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn do_notification(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x548ca0;
    loop {
        match pc {
            0x548ca0 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.push(m,c.get(7,32));
            { let a = rm(m,c.r[6].wrapping_add(0x928),32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x548d72; continue; }
            pc = 0x548cb7; continue;
            }
            0x548cb7 => {
            { let a = rm(m,c.r[4].wrapping_add(0x24),8); c.sub(a,0x1u32,8); }
            if c.cond(4) { pc = 0x548ffb; continue; }
            pc = 0x548cc2; continue;
            }
            0x548cc2 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x914),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x548cdd; continue; }
            pc = 0x548cd0; continue;
            }
            0x548cd0 => {
            c.set(0,32,c.get(7,32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0x924); wm(m,a,32,c.get(0,32)); }
            pc = 0x548cdd; continue;
            }
            0x548cdd => {
            c.set(1,32,c.r[6].wrapping_add(0x948));
            { let a = c.r[6].wrapping_add(0x92c); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0x914); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(2,32,c.get(1,32));
            pc = 0x548cf3; continue;
            }
            0x548cf3 => {
            { let a = rm(m,c.r[2],16); c.sub(a,c.get(7,16),16); }
            if c.cond(4) { pc = 0x548ffb; continue; }
            pc = 0x548cfc; continue;
            }
            0x548cfc => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.add(a,0xau32,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x548cf3; continue; }
            pc = 0x548d05; continue;
            }
            0x548d05 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x548d07; continue;
            }
            0x548d07 => {
            { let a = rm(m,c.r[1],16); c.sub(a,0x8u32,16); }
            if c.cond(3) { pc = 0x548d20; continue; }
            pc = 0x548d0d; continue;
            }
            0x548d0d => {
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.add(a,0xau32,32); c.set(1,32,r); }
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(12) { pc = 0x548d07; continue; }
            pc = 0x548d16; continue;
            }
            0x548d16 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x548d20 => {
            { let a = c.get(0,32); c.sub(a,0x8u32,32); }
            if c.cond(13) { pc = 0x548ffb; continue; }
            pc = 0x548d29; continue;
            }
            0x548d29 => {
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x28),16));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(0,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(4)).wrapping_add(0x4a6));
            c.set(1,32,c.r[6].wrapping_add(c.r[1].wrapping_mul(2)));
            { let a = c.r[1].wrapping_add(0x94a); wm(m,a,16,c.get(2,16)); }
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x2c),16));
            { let a = c.r[6].wrapping_add(c.r[0].wrapping_mul(2)); wm(m,a,16,c.get(2,16)); }
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x30),8));
            c.set(2,16,rm(m,c.r[4].wrapping_add(0x34),16));
            { let a = c.r[1].wrapping_add(0x948); wm(m,a,16,c.get(7,16)); }
            { let a = c.r[1].wrapping_add(0x94e); wm(m,a,8,c.get(0,8)); }
            { let a = c.r[1].wrapping_add(0x950); wm(m,a,16,c.get(2,16)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x548d72 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x924),32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x548da2; continue; }
            pc = 0x548d86; continue;
            }
            0x548d86 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x34),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffffu32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.and(a,0xffu32,32); c.set(1,32,r); }
            c.push(m,c.get(0,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x548da2);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x5483d0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x548da2; continue;
            }
            0x548da2 => {
            { let a = rm(m,c.r[4].wrapping_add(0x24),8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x548ffb; continue; }
            pc = 0x548dad; continue;
            }
            0x548dad => {
            { let a = rm(m,c.r[6].wrapping_add(0x92c),32); c.sub(a,0x1u32,32); }
            if c.cond(5) { pc = 0x548ffb; continue; }
            pc = 0x548dba; continue;
            }
            0x548dba => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0x924),32));
            { let a = c.r[6].wrapping_add(0x92c); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(0,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x548df2; continue; }
            pc = 0x548dca; continue;
            }
            0x548dca => {
            c.set(2,32,rm(m,0x6c30a4u32,32));
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x548ddd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x548de8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x42b4c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            0x548df2 => {
            c.push(m,c.get(7,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x548dfd);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x5303c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(2,32,rm(m,0x6dd894u32,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x2c),32));
            c.set(1,32,c.get(0,32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.and(a,0xffffu32,32); c.set(2,32,r); }
            { let a = c.get(3,8); let r = c.xor(a,c.get(3,8),8); c.set(3,8,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0xe),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(13) { pc = 0x548e41; continue; }
            pc = 0x548e1b; continue;
            }
            0x548e1b => {
            c.set(7,32,rm(m,0x6dd7a4u32,32));
            pc = 0x548e21; continue;
            }
            0x548e21 => {
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            { let a = c.get(1,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(1,32,r); }
            { let a = c.get(1,32); let r = c.add(a,c.get(7,32),32); c.set(1,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x39),8));
            { let a = c.get(0,16); c.sub(a,c.get(6,16),16); }
            if c.cond(4) { pc = 0x548e3f; continue; }
            pc = 0x548e33; continue;
            }
            0x548e33 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,16,rm(m,c.r[1].wrapping_add(0x4),16));
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(12) { pc = 0x548e21; continue; }
            pc = 0x548e3d; continue;
            }
            0x548e3d => {
            pc = 0x548e41; continue;
            }
            0x548e3f => {
            c.set(3,8,0x1u32);
            pc = 0x548e41; continue;
            }
            0x548e41 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(5,16); c.and(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x548ea0; continue; }
            pc = 0x548e4a; continue;
            }
            0x548e4a => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x548ea0; continue; }
            pc = 0x548e5b; continue;
            }
            0x548e5b => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.push(m,0x0u32);
            c.push(m,c.get(6,32));
            c.push(m,c.get(1,32));
            c.set(1,32,0x6de4a0u32);
            c.push(m, 0x548e6d);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x5308a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16);
            { let a = c.get(0,32); let r = c.neg(a,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.sbb(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.inc(a,32); c.set(0,32,r); }
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            c.set(2,8,u32::from(c.cond(4)));
            { let a = c.get(2,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x548ea0; continue; }
            pc = 0x548e7d; continue;
            }
            0x548e7d => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6de4b8),8));
            c.set(1,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(5,32)); }
            pc = 0x548ea0; continue;
            }
            0x548ea0 => {
            c.set(1,32,rm(m,0x6cc940u32,32));
            c.push(m, 0x548eab);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(5,16); c.and(a,c.get(5,16),16); }
            c.set(3,8,c.get(0,8));
            if c.cond(5) { pc = 0x548ef5; continue; }
            pc = 0x548eb2; continue;
            }
            0x548eb2 => {
            c.set(0,32,rm(m,0x6de4b0u32,32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(7,32); let r = c.and(a,0xffffu32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x548ef5; continue; }
            pc = 0x548ec3; continue;
            }
            0x548ec3 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x548ece);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x548f33; continue; }
            pc = 0x548ed2; continue;
            }
            0x548ed2 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(3,8,0x1u32);
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x6de4b8),8));
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,c.r[0].wrapping_add(c.r[2].wrapping_mul(4)));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(5,16,rm(m,c.r[0].wrapping_add(0x6ddbb0),16));
            c.set(6,16,rm(m,c.r[0].wrapping_add(0x6ddbb4),16));
            pc = 0x548f37; continue;
            }
            0x548ef5 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x548f05); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x548f12); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0xcc),32); c.push(m, 0x548f22); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x34),32); c.push(m, 0x548f2f); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(3,8,0x1u32);
            pc = 0x548f37; continue;
            }
            0x548f33 => {
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x548f37; continue;
            }
            0x548f37 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m, 0x548f42);
            { let args = []; c.r[0] = host.call(m, 0x4b2200, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x548f55; continue; }
            pc = 0x548f46; continue;
            }
            0x548f46 => {
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x548f53);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x44f490, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x548f69; continue;
            }
            0x548f55 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.push(m,0x1u32);
            c.push(m,0x0u32);
            c.push(m,0x1u32);
            c.push(m,c.get(6,32));
            c.push(m, 0x548f69);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16)), m.r32(c.r[4].wrapping_add(20))]; c.r[0] = host.call(m, 0x58a4f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(24);
            pc = 0x548f69; continue;
            }
            0x548f69 => {
            { let a = c.get(3,8); c.and(a,c.get(3,8),8); }
            if c.cond(4) { pc = 0x548f78; continue; }
            pc = 0x548f6d; continue;
            }
            0x548f6d => {
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x548f78);
            { let args = []; c.r[0] = host.call(m, 0x42a6c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x548f78; continue;
            }
            0x548f78 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x30),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.sub(a,0x25u32,32); }
            if c.cond(7) { pc = 0x548ffb; continue; }
            pc = 0x548f86; continue;
            }
            0x548f86 => {
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x549010),8));
            pc = m.r32(c.r[1].wrapping_mul(4).wrapping_add(0x549008)); continue;
            }
            0x548f95 => {
            c.push(m,0x2u32);
            c.push(m, 0x548f9c);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x38),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[0]; wm(m,a,16,c.get(1,16)); }
            { let a = c.get(1,32); let r = c.and(a,0xffffu32,32); c.set(1,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,c.get(1,32));
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,0x1u32); }
            c.set(2,32,c.r[0].wrapping_add(c.r[0].wrapping_mul(2)));
            c.set(0,32,rm(m,0x6dd7a4u32,32));
            { let a = c.get(2,32); let n = 0x7u32; let r = c.shl(a,n,32); c.set(2,32,r); }
            c.set(1,16,rm(m,c.r[2].wrapping_add(c.r[0]).wrapping_add(0x8),16));
            { let a = c.get(1,16); c.sub(a,rm(m,0x6dd896u32,16),16); }
            if c.cond(3) { pc = 0x548fdf; continue; }
            pc = 0x548fd3; continue;
            }
            0x548fd3 => {
            c.set(2,8,rm(m,0x66f354u32,8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(2,8)); }
            pc = 0x548fe8; continue;
            }
            0x548fdf => {
            c.set(0,8,rm(m,0x66f353u32,8));
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,8,c.get(0,8)); }
            pc = 0x548fe8; continue;
            }
            0x548fe8 => {
            c.set(1,32,rm(m,0x6ce6d0u32,32));
            c.set(0,32,c.r[4].wrapping_add(0x10));
            c.push(m,c.get(0,32));
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x134),32); c.push(m, 0x548ffb); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x548ffb; continue;
            }
            0x548ffb => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(28); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn save_last_won_close_file_and_cleanup(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x549b20;
    loop {
        match pc {
            0x549b20 => {
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,32,0x1u32); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            c.push(m, 0x549b33);
            { let args = []; c.r[0] = host.call(m, 0x653e38, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let args = []; c.r[0] = host.call(m, 0x549b3e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn uninitialized_copy_u32(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5800a0;
    loop {
        match pc {
            0x5800a0 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0xc),32));
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(4) { pc = 0x5800c4; continue; }
            pc = 0x5800b0; continue;
            }
            0x5800b0 => {
            c.push(m,c.get(6,32));
            pc = 0x5800b1; continue;
            }
            0x5800b1 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5800b9; continue; }
            pc = 0x5800b5; continue;
            }
            0x5800b5 => {
            c.set(6,32,rm(m,c.r[1],32));
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            pc = 0x5800b9; continue;
            }
            0x5800b9 => {
            { let a = c.get(1,32); let r = c.add(a,0x4u32,32); c.set(1,32,r); }
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); c.sub(a,c.get(2,32),32); }
            if c.cond(5) { pc = 0x5800b1; continue; }
            pc = 0x5800c3; continue;
            }
            0x5800c3 => {
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x5800c4; continue;
            }
            0x5800c4 => {
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn uninitialized_fill_n_u32(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5800d0;
    loop {
        match pc {
            0x5800d0 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5800f2; continue; }
            pc = 0x5800d8; continue;
            }
            0x5800d8 => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0xc),32));
            c.set(1,32,c.get(0,32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            pc = 0x5800e3; continue;
            }
            0x5800e3 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x5800eb; continue; }
            pc = 0x5800e7; continue;
            }
            0x5800e7 => {
            c.set(6,32,rm(m,c.r[2],32));
            { let a = c.r[0]; wm(m,a,32,c.get(6,32)); }
            pc = 0x5800eb; continue;
            }
            0x5800eb => {
            { let a = c.get(0,32); let r = c.add(a,0x4u32,32); c.set(0,32,r); }
            { let a = c.get(1,32); let r = c.dec(a,32); c.set(1,32,r); }
            if c.cond(5) { pc = 0x5800e3; continue; }
            pc = 0x5800f1; continue;
            }
            0x5800f1 => {
            { let v = c.pop(m); c.set(6,32,v); }
            pc = 0x5800f2; continue;
            }
            0x5800f2 => {
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn construct_copy_u32(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x580100;
    loop {
        match pc {
            0x580100 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x580110; continue; }
            pc = 0x580108; continue;
            }
            0x580108 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.r[0]; wm(m,a,32,c.get(2,32)); }
            pc = 0x580110; continue;
            }
            0x580110 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn rect_get_height(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5aa6a0;
    loop {
        match pc {
            0x5aa6a0 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0xc),32));
            c.set(2,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn rect_copy_from(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5be720;
    loop {
        match pc {
            0x5be720 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.push(m,c.get(6,32));
            c.set(6,32,c.get(1,32));
            c.push(m,c.get(0,32));
            c.push(m,c.get(6,32));
            { let target = rm(m,0x66a4a4u32,32); c.push(m, 0x5be72f); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,c.get(6,32));
            { let v = c.pop(m); c.set(6,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn string_vector_clear_and_free(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5c78f0;
    loop {
        match pc {
            0x5c78f0 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            c.set(3,32,rm(m,c.r[7].wrapping_add(0x8),32));
            c.set(6,32,rm(m,c.r[7].wrapping_add(0x4),32));
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5c790d; continue; }
            pc = 0x5c78ff; continue;
            }
            0x5c78ff => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5c7906);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x5c78ff; continue; }
            pc = 0x5c790d; continue;
            }
            0x5c790d => {
            c.set(0,32,rm(m,c.r[7].wrapping_add(0x4),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c7916);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            { let a = c.r[7].wrapping_add(0x4); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[7].wrapping_add(0xc); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn string_vector_element_address(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5c7930;
    loop {
        match pc {
            0x5c7930 => {
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x4),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(0,32,c.r[0].wrapping_add(c.r[1].wrapping_mul(4)));
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn string_vector_erase_range(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5c7940;
    loop {
        match pc {
            0x5c7940 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.set(5,32,c.get(1,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[4].wrapping_add(0x10),32));
            c.push(m,c.get(7,32));
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x18),32));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5c7967; continue; }
            pc = 0x5c7955; continue;
            }
            0x5c7955 => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5c795d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x5c7955; continue; }
            pc = 0x5c7967; continue;
            }
            0x5c7967 => {
            c.set(3,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(7,32,c.get(6,32));
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5c797e; continue; }
            pc = 0x5c7970; continue;
            }
            0x5c7970 => {
            c.set(1,32,c.get(7,32));
            c.push(m, 0x5c7977);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x5c7970; continue; }
            pc = 0x5c797e; continue;
            }
            0x5c797e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(6,32)); }
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

pub fn string_vector_insert_fill(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5c7990;
    loop {
        match pc {
            0x5c7990 => {
            { let a = c.get(4,32); let r = c.sub(a,0xcu32,32); c.set(4,32,r); }
            c.set(2,32,rm(m,c.r[1].wrapping_add(0xc),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(6,32),32); c.set(2,32,r); }
            c.push(m,c.get(7,32));
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x10); wm(m,a,32,c.get(1,32)); }
            if c.cond(3) { pc = 0x5c7ae2; continue; }
            pc = 0x5c79b2; continue;
            }
            0x5c79b2 => {
            c.set(1,32,rm(m,c.r[1].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(4) { pc = 0x5c79ca; continue; }
            pc = 0x5c79b9; continue;
            }
            0x5c79b9 => {
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(1,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(2,32),32); }
            if c.cond(3) { pc = 0x5c79c6; continue; }
            pc = 0x5c79c4; continue;
            }
            0x5c79c4 => {
            c.set(0,32,c.get(2,32));
            pc = 0x5c79c6; continue;
            }
            0x5c79c6 => {
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5c79ce; continue; }
            pc = 0x5c79ca; continue;
            }
            0x5c79ca => {
            { let a = c.get(6,32); let r = c.xor(a,c.get(6,32),32); c.set(6,32,r); }
            pc = 0x5c79d3; continue;
            }
            0x5c79ce => {
            { let a = c.get(6,32); let r = c.sub(a,c.get(1,32),32); c.set(6,32,r); }
            { let a = c.get(6,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(6,32,r); }
            pc = 0x5c79d3; continue;
            }
            0x5c79d3 => {
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            { let a = c.r[4].wrapping_add(0x18); wm(m,a,32,c.get(0,32)); }
            if c.cond(13) { pc = 0x5c79df; continue; }
            pc = 0x5c79dd; continue;
            }
            0x5c79dd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x5c79df; continue;
            }
            0x5c79df => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.push(m,c.get(0,32));
            c.push(m, 0x5c79e8);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x14),32));
            c.set(3,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,32,c.get(0,32)); }
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.set(7,32,c.get(0,32));
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5c7a14; continue; }
            pc = 0x5c7a00; continue;
            }
            0x5c7a00 => {
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            c.push(m, 0x5c7a07);
            { let args = []; c.r[0] = host.call(m, 0x5c7bf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0x4u32,32); c.set(7,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x5c7a00; continue; }
            pc = 0x5c7a14; continue;
            }
            0x5c7a14 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(6,32,c.get(7,32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5c7a42; continue; }
            pc = 0x5c7a1e; continue;
            }
            0x5c7a1e => {
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            pc = 0x5c7a22; continue;
            }
            0x5c7a22 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.push(m,c.get(1,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5c7a2d);
            { let args = []; c.r[0] = host.call(m, 0x5c7bf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(0,32); let r = c.dec(a,32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            if c.cond(5) { pc = 0x5c7a22; continue; }
            pc = 0x5c7a3e; continue;
            }
            0x5c7a3e => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            pc = 0x5c7a42; continue;
            }
            0x5c7a42 => {
            c.set(0,32,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)));
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.get(3,32); c.sub(a,c.get(7,32),32); }
            c.set(6,32,c.get(3,32));
            if c.cond(4) { pc = 0x5c7a6d; continue; }
            pc = 0x5c7a4e; continue;
            }
            0x5c7a4e => {
            { let a = c.get(0,32); let r = c.sub(a,c.get(3,32),32); c.set(0,32,r); }
            { let a = c.r[4].wrapping_add(0x28); wm(m,a,32,c.get(0,32)); }
            pc = 0x5c7a5a; continue;
            }
            0x5c7a56 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x5c7a5a; continue;
            }
            0x5c7a5a => {
            { let a = c.get(0,32); let r = c.add(a,c.get(6,32),32); c.set(0,32,r); }
            c.push(m,c.get(6,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c7a63);
            { let args = []; c.r[0] = host.call(m, 0x5c7bf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x5c7a56; continue; }
            pc = 0x5c7a6d; continue;
            }
            0x5c7a6d => {
            c.set(7,32,rm(m,c.r[5].wrapping_add(0x8),32));
            c.set(6,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(4) { pc = 0x5c7a85; continue; }
            pc = 0x5c7a77; continue;
            }
            0x5c7a77 => {
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5c7a7e);
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x5c7a77; continue; }
            pc = 0x5c7a85; continue;
            }
            0x5c7a85 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0x4),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x5c7a8e);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x1c),32));
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(1,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[5].wrapping_add(0xc); wm(m,a,32,c.get(1,32)); }
            c.set(1,32,rm(m,c.r[5].wrapping_add(0x4),32));
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            if c.cond(5) { pc = 0x5c7ac1; continue; }
            pc = 0x5c7aa6; continue;
            }
            0x5c7aa6 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x24),32));
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,32,c.get(1,32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x5c7ac1 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let v = c.pop(m); c.set(7,32,v); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(1,32),32); c.set(0,32,r); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x20),32));
            { let a = c.get(0,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(1,32),32); c.set(0,32,r); }
            { let a = c.r[5].wrapping_add(0x4); wm(m,a,32,c.get(2,32)); }
            { let v = c.pop(m); c.set(6,32,v); }
            c.set(0,32,c.r[2].wrapping_add(c.r[0].wrapping_mul(4)));
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            0x5c7ae2 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(2,32); c.sub(a,c.get(0,32),32); }
            if c.cond(3) { pc = 0x5c7b68; continue; }
            pc = 0x5c7af1; continue;
            }
            0x5c7af1 => {
            c.set(5,32,c.r[0].wrapping_mul(4));
            c.set(3,32,c.get(7,32));
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(5,32)); }
            { let a = c.get(5,32); let r = c.add(a,c.get(7,32),32); c.set(5,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x5c7b22; continue; }
            pc = 0x5c7b04; continue;
            }
            0x5c7b04 => {
            { let a = c.get(5,32); let r = c.sub(a,c.get(7,32),32); c.set(5,32,r); }
            pc = 0x5c7b06; continue;
            }
            0x5c7b06 => {
            c.set(0,32,c.r[3].wrapping_add(c.r[5]));
            c.push(m,c.get(3,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c7b10);
            { let args = []; c.r[0] = host.call(m, 0x5c7bf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x5c7b06; continue; }
            pc = 0x5c7b1a; continue;
            }
            0x5c7b1a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x24),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5c7b22; continue;
            }
            0x5c7b22 => {
            c.set(6,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(5,32,rm(m,c.r[4].wrapping_add(0x28),32));
            c.set(2,32,c.get(6,32));
            { let a = c.get(2,32); let r = c.sub(a,c.get(7,32),32); c.set(2,32,r); }
            { let a = c.get(2,32); let n = 0x2u32; let r = c.sar(a,n,32); c.set(2,32,r); }
            { let a = c.get(0,32); let r = c.sub(a,c.get(2,32),32); c.set(0,32,r); }
            if c.cond(4) { pc = 0x5c7b4a; continue; }
            pc = 0x5c7b34; continue;
            }
            0x5c7b34 => {
            c.set(3,32,c.get(0,32));
            pc = 0x5c7b36; continue;
            }
            0x5c7b36 => {
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m, 0x5c7b3d);
            { let args = []; c.r[0] = host.call(m, 0x5c7bf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.dec(a,32); c.set(3,32,r); }
            if c.cond(5) { pc = 0x5c7b36; continue; }
            pc = 0x5c7b46; continue;
            }
            0x5c7b46 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5c7b4a; continue;
            }
            0x5c7b4a => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(6,32,c.get(7,32));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5c7bde; continue; }
            pc = 0x5c7b57; continue;
            }
            0x5c7b57 => {
            c.push(m,c.get(5,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5c7b5f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x5c7b57; continue; }
            pc = 0x5c7b66; continue;
            }
            0x5c7b66 => {
            pc = 0x5c7bda; continue;
            }
            0x5c7b68 => {
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(6) { pc = 0x5c7be5; continue; }
            pc = 0x5c7b6c; continue;
            }
            0x5c7b6c => {
            { let a = c.get(0,32); let n = 0x2u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            c.set(3,32,c.get(6,32));
            c.set(5,32,c.get(6,32));
            { let a = c.get(3,32); let r = c.sub(a,c.get(0,32),32); c.set(3,32,r); }
            { let a = c.r[4].wrapping_add(0x20); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(3,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x5c7b99; continue; }
            pc = 0x5c7b7d; continue;
            }
            0x5c7b7d => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m, 0x5c7b84);
            { let args = []; c.r[0] = host.call(m, 0x5c7bf0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(3,32); let r = c.add(a,0x4u32,32); c.set(3,32,r); }
            { let a = c.get(4,32); let r = c.add(a,0x8u32,32); c.set(4,32,r); }
            { let a = c.get(5,32); let r = c.add(a,0x4u32,32); c.set(5,32,r); }
            { let a = c.get(3,32); c.sub(a,c.get(6,32),32); }
            if c.cond(5) { pc = 0x5c7b7d; continue; }
            pc = 0x5c7b91; continue;
            }
            0x5c7b91 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5c7b99; continue;
            }
            0x5c7b99 => {
            c.set(3,32,rm(m,c.r[1].wrapping_add(0x8),32));
            c.set(6,32,c.get(3,32));
            { let a = c.get(6,32); let r = c.sub(a,c.get(0,32),32); c.set(6,32,r); }
            { let a = c.get(7,32); c.sub(a,c.get(6,32),32); }
            if c.cond(4) { pc = 0x5c7bbe; continue; }
            pc = 0x5c7ba4; continue;
            }
            0x5c7ba4 => {
            { let a = c.get(6,32); let r = c.sub(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(3,32); let r = c.sub(a,0x4u32,32); c.set(3,32,r); }
            c.push(m,c.get(6,32));
            c.set(1,32,c.get(3,32));
            c.push(m, 0x5c7bb2);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); c.sub(a,c.get(7,32),32); }
            if c.cond(5) { pc = 0x5c7ba4; continue; }
            pc = 0x5c7bb6; continue;
            }
            0x5c7bb6 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5c7bbe; continue;
            }
            0x5c7bbe => {
            c.set(3,32,c.r[0].wrapping_add(c.r[7]));
            c.set(6,32,c.get(7,32));
            { let a = c.get(7,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x5c7be2; continue; }
            pc = 0x5c7bc7; continue;
            }
            0x5c7bc7 => {
            c.set(7,32,rm(m,c.r[4].wrapping_add(0x28),32));
            pc = 0x5c7bcb; continue;
            }
            0x5c7bcb => {
            c.push(m,c.get(7,32));
            c.set(1,32,c.get(6,32));
            c.push(m, 0x5c7bd3);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(6,32); c.sub(a,c.get(3,32),32); }
            if c.cond(5) { pc = 0x5c7bcb; continue; }
            pc = 0x5c7bda; continue;
            }
            0x5c7bda => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x10),32));
            pc = 0x5c7bde; continue;
            }
            0x5c7bde => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x20),32));
            pc = 0x5c7be2; continue;
            }
            0x5c7be2 => {
            { let a = rm(m,c.r[1].wrapping_add(0x8),32); let r = c.add(a,c.get(0,32),32); { let a = c.r[1].wrapping_add(0x8); wm(m,a,32,r); } }
            pc = 0x5c7be5; continue;
            }
            0x5c7be5 => {
            { let v = c.pop(m); c.set(7,32,v); }
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let v = c.pop(m); c.set(3,32,v); }
            { let a = c.get(4,32); let r = c.add(a,0xcu32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn string_construct_copy_in_place(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5c7bf0;
    loop {
        match pc {
            0x5c7bf0 => {
            c.push(m,0xffffffffu32);
            c.push(m,0x6664c1u32);
            c.set(0,32,rm(m,0x0u32,32));
            c.push(m,c.get(0,32));
            { let a = 0x0u32; wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x14),32));
            { let a = c.r[4]; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(1,32); c.and(a,c.get(1,32),32); }
            { let a = c.r[4].wrapping_add(0xc); wm(m,a,32,0x0u32); }
            if c.cond(4) { pc = 0x5c7c24; continue; }
            pc = 0x5c7c1a; continue;
            }
            0x5c7c1a => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x18),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x5c7c24);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x5c7c24; continue;
            }
            0x5c7c24 => {
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = 0x0u32; wm(m,a,32,c.get(1,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x10u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn unwind_free_string_at_offset_0x18(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5d8290;
    loop {
        match pc {
            0x5d8290 => {
            { let a = c.get(1,32); let r = c.add(a,0x18u32,32); c.set(1,32,r); }
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn clear_all_battles(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x611840;
    loop {
        match pc {
            0x611840 => {
            c.push(m,c.get(3,32));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            c.push(m,c.get(7,32));
            { let a = c.get(3,32); let r = c.xor(a,c.get(3,32),32); c.set(3,32,r); }
            c.set(6,32,c.get(1,32));
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x61184a; continue;
            }
            0x61184a => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(c.r[7].wrapping_mul(4)),32));
            { let a = c.get(1,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x611858; continue; }
            pc = 0x611851; continue;
            }
            0x611851 => {
            c.set(0,32,rm(m,c.r[1],32));
            c.push(m,0x1u32);
            { let target = rm(m,c.r[0].wrapping_add(0x4),32); c.push(m, 0x611858); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x611858; continue;
            }
            0x611858 => {
            { let a = c.r[6].wrapping_add(c.r[7].wrapping_mul(4)); wm(m,a,32,c.get(3,32)); }
            { let a = c.get(7,32); let r = c.inc(a,32); c.set(7,32,r); }
            { let a = c.get(7,32); c.sub(a,0xffu32,32); }
            if c.cond(2) { pc = 0x61184a; continue; }
            pc = 0x611864; continue;
            }
            0x611864 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc34),32));
            { let a = c.get(5,32); let r = c.xor(a,c.get(5,32),32); c.set(5,32,r); }
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            { let a = c.r[6].wrapping_add(0x3fc); wm(m,a,8,0xffu32); }
            { let a = c.r[6].wrapping_add(0x404); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x42c); wm(m,a,8,c.get(3,8)); }
            { let a = c.r[6].wrapping_add(0x3fd); wm(m,a,8,c.get(3,8)); }
            if c.cond(6) { pc = 0x6118af; continue; }
            pc = 0x611889; continue;
            }
            0x611889 => {
            { let a = c.get(7,32); let r = c.xor(a,c.get(7,32),32); c.set(7,32,r); }
            pc = 0x61188b; continue;
            }
            0x61188b => {
            c.set(1,32,rm(m,c.r[6].wrapping_add(0xc30),32));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[1]),32));
            { let a = c.get(0,32); c.sub(a,c.get(3,32),32); }
            if c.cond(4) { pc = 0x6118a1; continue; }
            pc = 0x611898; continue;
            }
            0x611898 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x61189e);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            pc = 0x6118a1; continue;
            }
            0x6118a1 => {
            c.set(0,32,rm(m,c.r[6].wrapping_add(0xc34),32));
            { let a = c.get(5,32); let r = c.inc(a,32); c.set(5,32,r); }
            { let a = c.get(7,32); let r = c.add(a,0xcu32,32); c.set(7,32,r); }
            { let a = c.get(5,32); c.sub(a,c.get(0,32),32); }
            if c.cond(2) { pc = 0x61188b; continue; }
            pc = 0x6118af; continue;
            }
            0x6118af => {
            c.set(2,32,rm(m,c.r[6].wrapping_add(0xc30),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x6118bb);
            { let args = []; c.r[0] = host.call(m, 0x63a1b6, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.or(a,0xffffffffu32,32); c.set(0,32,r); }
            { let a = c.r[6].wrapping_add(0xc30); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc34); wm(m,a,32,c.get(3,32)); }
            { let a = c.r[6].wrapping_add(0xc44); wm(m,a,32,c.get(0,32)); }
            { let a = c.r[6].wrapping_add(0x3fe); wm(m,a,8,0x1u32); }
            { let a = c.r[6].wrapping_add(0x408); wm(m,a,32,c.get(0,32)); }
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

pub fn open_first_battle_view_for_merchant(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x611e50;
    loop {
        match pc {
            0x611e50 => {
            c.push(m,c.get(7,32));
            c.set(7,32,c.get(1,32));
            { let a = rm(m,c.r[7].wrapping_add(0x3fc),8); c.sub(a,0xffu32,8); }
            if c.cond(5) { pc = 0x611f9b; continue; }
            pc = 0x611e60; continue;
            }
            0x611e60 => {
            c.push(m,c.get(3,32));
            c.set(3,8,rm(m,c.r[4].wrapping_add(0xc),8));
            c.push(m,c.get(5,32));
            c.push(m,c.get(6,32));
            { let a = c.get(2,8); let r = c.xor(a,c.get(2,8),8); c.set(2,8,r); }
            c.set(6,32,c.get(7,32));
            pc = 0x611e6b; continue;
            }
            0x611e6b => {
            c.set(0,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x611ecf; continue; }
            pc = 0x611e71; continue;
            }
            0x611e71 => {
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x558),8));
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,0x0u32); }
            { let a = c.get(1,8); c.and(a,c.get(1,8),8); }
            if c.cond(6) { pc = 0x611ecf; continue; }
            pc = 0x611e80; continue;
            }
            0x611e80 => {
            c.set(0,32,rm(m,0x6dfc18u32,32));
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x14),32));
            if c.cond(4) { pc = 0x611e9b; continue; }
            pc = 0x611e8f; continue;
            }
            0x611e8f => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[0].wrapping_add(c.r[1]).wrapping_add(0x5d),8); c.sub(a,0x0u32,8); }
            pc = 0x611eb3; continue;
            }
            0x611e9b => {
            c.set(5,16,c.get(3,8));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            { let a = rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(2)).wrapping_add(0x35a),16); c.sub(a,c.get(5,16),16); }
            if c.cond(5) { pc = 0x611ebb; continue; }
            pc = 0x611eae; continue;
            }
            0x611eae => {
            { let a = rm(m,c.r[1].wrapping_add(c.r[0]).wrapping_add(0x5d),8); c.sub(a,0x0u32,8); }
            pc = 0x611eb3; continue;
            }
            0x611eb3 => {
            if c.cond(5) { pc = 0x611ebb; continue; }
            pc = 0x611eb5; continue;
            }
            0x611eb5 => {
            { let a = c.r[7].wrapping_add(0x3fc); wm(m,a,8,c.get(2,8)); }
            pc = 0x611ebb; continue;
            }
            0x611ebb => {
            c.set(0,8,rm(m,c.r[4].wrapping_add(0x14),8));
            c.set(1,32,rm(m,c.r[6],32));
            { let a = c.get(0,8); let r = c.inc(a,8); c.set(0,8,r); }
            { let a = c.get(0,8); c.sub(a,rm(m,c.r[1].wrapping_add(0x558),8),8); }
            { let a = c.r[4].wrapping_add(0x14); wm(m,a,8,c.get(0,8)); }
            if c.cond(2) { pc = 0x611e80; continue; }
            pc = 0x611ecf; continue;
            }
            0x611ecf => {
            { let a = c.get(2,8); let r = c.inc(a,8); c.set(2,8,r); }
            { let a = c.get(6,32); let r = c.add(a,0x4u32,32); c.set(6,32,r); }
            { let a = c.get(2,8); c.sub(a,0xffu32,8); }
            if c.cond(2) { pc = 0x611e6b; continue; }
            pc = 0x611ed9; continue;
            }
            0x611ed9 => {
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3fc),8));
            { let v = c.pop(m); c.set(6,32,v); }
            { let v = c.pop(m); c.set(5,32,v); }
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            { let v = c.pop(m); c.set(3,32,v); }
            if c.cond(4) { pc = 0x611f9b; continue; }
            pc = 0x611eea; continue;
            }
            0x611eea => {
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.get(0,32));
            c.set(2,32,rm(m,c.r[0],32));
            { let target = rm(m,c.r[2].wrapping_add(0x28),32); c.push(m, 0x611ef9); let args = []; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(0,8); c.and(a,c.get(0,8),8); }
            if c.cond(4) { pc = 0x611f1e; continue; }
            pc = 0x611efd; continue;
            }
            0x611efd => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3fc),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(2,16,rm(m,c.r[1].wrapping_add(0x1e98),8));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(2,32));
            c.push(m, 0x611f1c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x58a310, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            pc = 0x611f41; continue;
            }
            0x611f1e => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.set(0,8,rm(m,c.r[7].wrapping_add(0x3fc),8));
            c.set(0,32,rm(m,c.r[7].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,32,c.r[0].wrapping_add(0xa80));
            { let a = c.get(0,32); let r = c.add(a,0xa7cu32,32); c.set(0,32,r); }
            c.push(m,c.get(1,32));
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,c.get(0,32));
            c.push(m, 0x611f41);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x589ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            pc = 0x611f41; continue;
            }
            0x611f41 => {
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x611f51); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m,0x1u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x611f5e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0xcc),32); c.push(m, 0x611f6e); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbdd8u32,32));
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x34),32); c.push(m, 0x611f7b); let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            { let a = c.get(1,32); let r = c.xor(a,c.get(1,32),32); c.set(1,32,r); }
            c.set(1,8,rm(m,c.r[7].wrapping_add(0x3fc),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[1].wrapping_mul(4)),32));
            c.push(m, 0x611f8b);
            { let args = []; c.r[0] = host.call(m, 0x610379, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(2,32); let r = c.xor(a,c.get(2,32),32); c.set(2,32,r); }
            c.set(2,8,rm(m,c.r[7].wrapping_add(0x3fc),8));
            c.set(1,32,rm(m,c.r[7].wrapping_add(c.r[2].wrapping_mul(4)),32));
            c.push(m, 0x611f9b);
            { let args = []; c.r[0] = host.call(m, 0x6107b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            pc = 0x611f9b; continue;
            }
            0x611f9b => {
            { let v = c.pop(m); c.set(7,32,v); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn get_current_view_wind(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x613070;
    loop {
        match pc {
            0x613070 => {
            c.set(0,8,rm(m,c.r[1].wrapping_add(0x3fc),8));
            { let a = c.get(0,8); c.sub(a,0xffu32,8); }
            if c.cond(4) { pc = 0x61309d; continue; }
            pc = 0x61307a; continue;
            }
            0x61307a => {
            c.set(2,32,rm(m,c.r[4].wrapping_add(0x4),32));
            { let a = c.get(0,32); let r = c.and(a,0xffu32,32); c.set(0,32,r); }
            c.set(0,32,rm(m,c.r[1].wrapping_add(c.r[0].wrapping_mul(4)),32));
            c.set(1,8,rm(m,c.r[0].wrapping_add(0x670),8));
            { let a = c.r[2]; wm(m,a,8,c.get(1,8)); }
            c.set(1,32,rm(m,c.r[4].wrapping_add(0x8),32));
            c.set(0,8,rm(m,c.r[0].wrapping_add(0x671),8));
            { let a = c.r[1]; wm(m,a,8,c.get(0,8)); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            0x61309d => {
            c.set(1,32,0x6e6220u32);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x618d10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn noop_return(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x621750;
    loop {
        match pc {
            0x621750 => {
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn array_element_ptr_stride16(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x621760;
    loop {
        match pc {
            0x621760 => {
            c.set(0,32,rm(m,c.r[4].wrapping_add(0x4),32));
            c.set(2,32,rm(m,c.r[1],32));
            { let a = c.get(0,32); let n = 0x4u32; let r = c.shl(a,n,32); c.set(0,32,r); }
            { let a = c.get(0,32); let r = c.add(a,c.get(2,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn register_miles_shutdown_atexit(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x638980;
    loop {
        match pc {
            0x638980 => {
            c.push(m,0x638990u32);
            c.push(m, 0x63898a);
            { let args = []; c.r[0] = host.call(m, 0x63978e, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "archive_file_ctor" => archive_file_ctor(m, host, regs),
        "archive_file_dtor" => archive_file_dtor(m, host, regs),
        "archive_file_open" => archive_file_open(m, host, regs),
        "archive_file_close" => archive_file_close(m, host, regs),
        "archive_file_read" => archive_file_read(m, host, regs),
        "archive_file_seek" => archive_file_seek(m, host, regs),
        "archive_file_get_length" => archive_file_get_length(m, host, regs),
        "archive_file_scalar_deleting_dtor" => archive_file_scalar_deleting_dtor(m, host, regs),
        "cstring_get_pipe_field" => cstring_get_pipe_field(m, host, regs),
        "cstring_get_pipe_field_as_int" => cstring_get_pipe_field_as_int(m, host, regs),
        "cstring_parse_hex_consuming" => cstring_parse_hex_consuming(m, host, regs),
        "cstring_construct_empty" => cstring_construct_empty(m, host, regs),
        "operator_delete_stdcall_thunk" => operator_delete_stdcall_thunk(m, host, regs),
        "two_string_members_dtor" => two_string_members_dtor(m, host, regs),
        "rect_get_width" => rect_get_width(m, host, regs),
        "file_stream_scalar_deleting_dtor" => file_stream_scalar_deleting_dtor(m, host, regs),
        "file_stream_dtor" => file_stream_dtor(m, host, regs),
        "get_first_dword" => get_first_dword(m, host, regs),
        "load_all_fonts" => load_all_fonts(m, host, regs),
        "copy_string_field_0x34" => copy_string_field_0x34(m, host, regs),
        "std_string_tidy" => std_string_tidy(m, host, regs),
        "std_string_c_str" => std_string_c_str(m, host, regs),
        "std_string_grow" => std_string_grow(m, host, regs),
        "get_cdrom_drive_letters" => get_cdrom_drive_letters(m, host, regs),
        "swap_u16_array_entries" => swap_u16_array_entries(m, host, regs),
        "remove_u16_from_array" => remove_u16_from_array(m, host, regs),
        "archive_file_finder_ctor" => archive_file_finder_ctor(m, host, regs),
        "archive_file_finder_dtor" => archive_file_finder_dtor(m, host, regs),
        "archive_file_finder_find_in_archive" => archive_file_finder_find_in_archive(m, host, regs),
        "archive_file_finder_find_next" => archive_file_finder_find_next(m, host, regs),
        "archive_file_finder_clear" => archive_file_finder_clear(m, host, regs),
        "archive_file_finder_get_file_name" => archive_file_finder_get_file_name(m, host, regs),
        "archive_file_finder_get_file_title" => archive_file_finder_get_file_title(m, host, regs),
        "archive_file_finder_scalar_deleting_dtor" => archive_file_finder_scalar_deleting_dtor(m, host, regs),
        "string_array_get_at_wrapper" => string_array_get_at_wrapper(m, host, regs),
        "string_array_get_at" => string_array_get_at(m, host, regs),
        "strtok_with_quotes" => strtok_with_quotes(m, host, regs),
        "get_edge_color_from_packed_style" => get_edge_color_from_packed_style(m, host, regs),
        "set_edge_color_index_in_packed_style" => set_edge_color_index_in_packed_style(m, host, regs),
        "read_ini_int_triple" => read_ini_int_triple(m, host, regs),
        "ini_file_cache_ctor" => ini_file_cache_ctor(m, host, regs),
        "ini_file_cache_dtor" => ini_file_cache_dtor(m, host, regs),
        "ini_file_cache_get_string" => ini_file_cache_get_string(m, host, regs),
        "ini_file_cache_get_int" => ini_file_cache_get_int(m, host, regs),
        "ini_file_cache_read_value" => ini_file_cache_read_value(m, host, regs),
        "ini_file_cache_parse_file" => ini_file_cache_parse_file(m, host, regs),
        "ini_file_cache_preload_file" => ini_file_cache_preload_file(m, host, regs),
        "ini_file_cache_set_enabled" => ini_file_cache_set_enabled(m, host, regs),
        "ini_file_cache_deleting_dtor" => ini_file_cache_deleting_dtor(m, host, regs),
        "std_map_destroy" => std_map_destroy(m, host, regs),
        "std_string_pair_destroy" => std_string_pair_destroy(m, host, regs),
        "std_string_copy_ctor" => std_string_copy_ctor(m, host, regs),
        "std_string_assign_substr" => std_string_assign_substr(m, host, regs),
        "std_string_find_first_of" => std_string_find_first_of(m, host, regs),
        "std_string_find_first_not_of" => std_string_find_first_not_of(m, host, regs),
        "std_string_find_last_not_of" => std_string_find_last_not_of(m, host, regs),
        "std_string_set_length" => std_string_set_length(m, host, regs),
        "std_string_unshare" => std_string_unshare(m, host, regs),
        "std_string_tidy_flag" => std_string_tidy_flag(m, host, regs),
        "std_map_begin" => std_map_begin(m, host, regs),
        "std_map_insert" => std_map_insert(m, host, regs),
        "std_map_insert_result_init" => std_map_insert_result_init(m, host, regs),
        "std_string_assign_chars" => std_string_assign_chars(m, host, regs),
        "std_string_max_size" => std_string_max_size(m, host, regs),
        "std_map_erase_node" => std_map_erase_node(m, host, regs),
        "std_map_erase_subtree" => std_map_erase_subtree(m, host, regs),
        "std_map_insert_node" => std_map_insert_node(m, host, regs),
        "std_map_lower_bound" => std_map_lower_bound(m, host, regs),
        "std_map_rotate_left" => std_map_rotate_left(m, host, regs),
        "std_map_rotate_right" => std_map_rotate_right(m, host, regs),
        "std_map_buy_node" => std_map_buy_node(m, host, regs),
        "std_map_iterator_decrement" => std_map_iterator_decrement(m, host, regs),
        "std_map_iterator_increment" => std_map_iterator_increment(m, host, regs),
        "std_string_concat" => std_string_concat(m, host, regs),
        "std_string_pair_copy_ctor" => std_string_pair_copy_ctor(m, host, regs),
        "std_string_erase" => std_string_erase(m, host, regs),
        "std_string_pair_scalar_deleting_dtor" => std_string_pair_scalar_deleting_dtor(m, host, regs),
        "std_string_compare" => std_string_compare(m, host, regs),
        "std_string_compare_range_chars" => std_string_compare_range_chars(m, host, regs),
        "virtual_stub_return_true" => virtual_stub_return_true(m, host, regs),
        "virtual_stub_noop" => virtual_stub_noop(m, host, regs),
        "virtual_stub_return_zero" => virtual_stub_return_zero(m, host, regs),
        "cstring_assign_cstr_thunk" => cstring_assign_cstr_thunk(m, host, regs),
        "cstring_add_missing_chars" => cstring_add_missing_chars(m, host, regs),
        "merchant_business_get_save_size" => merchant_business_get_save_size(m, host, regs),
        "merchant_business_serialize" => merchant_business_serialize(m, host, regs),
        "merchant_business_deserialize" => merchant_business_deserialize(m, host, regs),
        "merchant_business_init" => merchant_business_init(m, host, regs),
        "merchant_business_recompute_capacity" => merchant_business_recompute_capacity(m, host, regs),
        "merchant_business_get_building_count" => merchant_business_get_building_count(m, host, regs),
        "message_pool_free_array" => message_pool_free_array(m, host, regs),
        "message_pool_get_serialized_size" => message_pool_get_serialized_size(m, host, regs),
        "message_pool_serialize" => message_pool_serialize(m, host, regs),
        "message_pool_deserialize" => message_pool_deserialize(m, host, regs),
        "scheduled_tasks_serialize" => scheduled_tasks_serialize(m, host, regs),
        "scheduled_tasks_sum_merchant_debts" => scheduled_tasks_sum_merchant_debts(m, host, regs),
        "scheduled_tasks_get_building_auction_info" => scheduled_tasks_get_building_auction_info(m, host, regs),
        "assign_town_slots" => assign_town_slots(m, host, regs),
        "scheduled_tasks_find_celebration_in_town" => scheduled_tasks_find_celebration_in_town(m, host, regs),
        "st_resolve_commissioned_burglary" => st_resolve_commissioned_burglary(m, host, regs),
        "scheduled_tasks_alderman_mission_start" => scheduled_tasks_alderman_mission_start(m, host, regs),
        "op_assign_merchant_to_player_slot" => op_assign_merchant_to_player_slot(m, host, regs),
        "custom_map_save_close_file_tail" => custom_map_save_close_file_tail(m, host, regs),
        "operations_save_game_finish_main_file" => operations_save_game_finish_main_file(m, host, regs),
        "operations_save_game_finish_second_file" => operations_save_game_finish_second_file(m, host, regs),
        "operations_send_lobby_profile" => operations_send_lobby_profile(m, host, regs),
        "do_notification" => do_notification(m, host, regs),
        "save_last_won_close_file_and_cleanup" => save_last_won_close_file_and_cleanup(m, host, regs),
        "uninitialized_copy_u32" => uninitialized_copy_u32(m, host, regs),
        "uninitialized_fill_n_u32" => uninitialized_fill_n_u32(m, host, regs),
        "construct_copy_u32" => construct_copy_u32(m, host, regs),
        "rect_get_height" => rect_get_height(m, host, regs),
        "rect_copy_from" => rect_copy_from(m, host, regs),
        "string_vector_clear_and_free" => string_vector_clear_and_free(m, host, regs),
        "string_vector_element_address" => string_vector_element_address(m, host, regs),
        "string_vector_erase_range" => string_vector_erase_range(m, host, regs),
        "string_vector_insert_fill" => string_vector_insert_fill(m, host, regs),
        "string_construct_copy_in_place" => string_construct_copy_in_place(m, host, regs),
        "unwind_free_string_at_offset_0x18" => unwind_free_string_at_offset_0x18(m, host, regs),
        "clear_all_battles" => clear_all_battles(m, host, regs),
        "open_first_battle_view_for_merchant" => open_first_battle_view_for_merchant(m, host, regs),
        "get_current_view_wind" => get_current_view_wind(m, host, regs),
        "noop_return" => noop_return(m, host, regs),
        "array_element_ptr_stride16" => array_element_ptr_stride16(m, host, regs),
        "register_miles_shutdown_atexit" => register_miles_shutdown_atexit(m, host, regs),
        _ => return None,
    })
}
