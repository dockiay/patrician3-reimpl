//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.
#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::x86rt::*;

pub fn thunk_FUN_0064ff74_408ed0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x408ed0;
    loop {
        match pc {
            0x408ed0 => {
            { let args = []; c.r[0] = host.call(m, 0x64ff74, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn thunk_FUN_0064f253_40a3d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40a3d0;
    loop {
        match pc {
            0x40a3d0 => {
            { let args = []; c.r[0] = host.call(m, 0x64f253, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn noop_return_40afd0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40afd0;
    loop {
        match pc {
            0x40afd0 => {
            { let args = []; c.r[0] = host.call(m, 0x621750, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_handle_mouse_event_mode_40bc30(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x40bc30;
    loop {
        match pc {
            0x40bc30 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_handle_mouse_event_mode_410310(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x410310;
    loop {
        match pc {
            0x410310 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn CDialog_41c110(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x41c110;
    loop {
        match pc {
            0x41c110 => {
            { let args = []; c.r[0] = host.call(m, 0x64f967, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_mark_all_dirty_421540(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x421540;
    loop {
        match pc {
            0x421540 => {
            { let args = []; c.r[0] = host.call(m, 0x4b1b10, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn screen_window_on_mouse_leave_42f980(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42f980;
    loop {
        match pc {
            0x42f980 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4b6b80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn thunk_FUN_00653743_434ad0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x434ad0;
    loop {
        match pc {
            0x434ad0 => {
            { let args = []; c.r[0] = host.call(m, 0x653743, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn screen_window_on_mouse_leave_435940(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x435940;
    loop {
        match pc {
            0x435940 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4b6b80, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_noop_slot7_452890(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x452890;
    loop {
        match pc {
            0x452890 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4b1850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn edit_box_on_focus_gained_45c220(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x45c220;
    loop {
        match pc {
            0x45c220 => {
            { let args = []; c.r[0] = host.call(m, 0x4ca640, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_noop_slot7_466c20(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x466c20;
    loop {
        match pc {
            0x466c20 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4b1850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_noop_slot7_46af00(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x46af00;
    loop {
        match pc {
            0x46af00 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4b1850, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn button_base_dtor_478660(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x478660;
    loop {
        match pc {
            0x478660 => {
            { let args = []; c.r[0] = host.call(m, 0x4c6a30, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4ad530(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4ad530;
    loop {
        match pc {
            0x4ad530 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b18c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn list_box_on_mouse_event_4afe90(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4afe90;
    loop {
        match pc {
            0x4afe90 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4cb2a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn list_box_handle_key_4afea0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4afea0;
    loop {
        match pc {
            0x4afea0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12))]; c.r[0] = host.call(m, 0x4cb6c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(16); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4b0d50(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b0d50;
    loop {
        match pc {
            0x4b0d50 => {
            { let args = []; c.r[0] = host.call(m, 0x4c74b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4b57b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b57b0;
    loop {
        match pc {
            0x4b57b0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b18c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_add_dirty_rect_4b8660(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4b8660;
    loop {
        match pc {
            0x4b8660 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b1980, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn WritePrivateProfileStringA_4be110(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4be110;
    loop {
        match pc {
            0x4be110 => {
            { let target = rm(m,0x66a2c0u32,32); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8)), m.r32(c.r[4].wrapping_add(12)), m.r32(c.r[4].wrapping_add(16))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(20); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4c4eb0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c4eb0;
    loop {
        match pc {
            0x4c4eb0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b18c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4c74b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c74b0;
    loop {
        match pc {
            0x4c74b0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b18c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4c88d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c88d0;
    loop {
        match pc {
            0x4c88d0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b18c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4c94f0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c94f0;
    loop {
        match pc {
            0x4c94f0 => {
            { let args = []; c.r[0] = host.call(m, 0x4b57b0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4c9dc0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4c9dc0;
    loop {
        match pc {
            0x4c9dc0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b18c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn text_box_update_borders_4caac0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4caac0;
    loop {
        match pc {
            0x4caac0 => {
            { let args = []; c.r[0] = host.call(m, 0x4d30f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4cbcf0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4cbcf0;
    loop {
        match pc {
            0x4cbcf0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b18c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4cf480(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4cf480;
    loop {
        match pc {
            0x4cf480 => {
            { let args = []; c.r[0] = host.call(m, 0x4c94f0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4d0290(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d0290;
    loop {
        match pc {
            0x4d0290 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b18c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_on_mouse_press_4d0b10(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d0b10;
    loop {
        match pc {
            0x4d0b10 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b2320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4d1700(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d1700;
    loop {
        match pc {
            0x4d1700 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b18c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_draw_at_origin_4d27b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d27b0;
    loop {
        match pc {
            0x4d27b0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x4b18c0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn letter_free_and_reset_51a880(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51a880;
    loop {
        match pc {
            0x51a880 => {
            { let args = []; c.r[0] = host.call(m, 0x51aab0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_free_building_records_51e380(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x51e380;
    loop {
        match pc {
            0x51e380 => {
            { let args = []; c.r[0] = host.call(m, 0x51e8a0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn operation_queue_free_543940(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x543940;
    loop {
        match pc {
            0x543940 => {
            { let args = []; c.r[0] = host.call(m, 0x543a70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_anim_state_reset_frame_5626d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5626d0;
    loop {
        match pc {
            0x5626d0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x55ffc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_anim_state_reset_frame_567720(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x567720;
    loop {
        match pc {
            0x567720 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x55ffc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn town_anim_state_reset_frame_572db0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x572db0;
    loop {
        match pc {
            0x572db0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x55ffc0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_base44_begin_close_5a8660(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5a8660;
    loop {
        match pc {
            0x5a8660 => {
            { let args = []; c.r[0] = host.call(m, 0x440ed0, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_handle_mouse_event_mode_5cb900(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5cb900;
    loop {
        match pc {
            0x5cb900 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_handle_mouse_event_mode_5dd8d0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5dd8d0;
    loop {
        match pc {
            0x5dd8d0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_handle_mouse_event_mode_5eb3b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5eb3b0;
    loop {
        match pc {
            0x5eb3b0 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn window_handle_mouse_event_mode_5f3500(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x5f3500;
    loop {
        match pc {
            0x5f3500 => {
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x4b1720, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn siege_state_reset_6278e0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x6278e0;
    loop {
        match pc {
            0x6278e0 => {
            { let args = []; c.r[0] = host.call(m, 0x627f70, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00425e83_425e83(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x425e83;
    loop {
        match pc {
            0x425e83 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffa4));
            c.push(m, 0x425e8b);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0xebu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffa4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x13u32); }
            c.push(m, 0x425e9c);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffa4));
            c.push(m, 0x425ea4);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0x1u32);
            { let a = c.r[1].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffa4));
            c.push(m, 0x425eba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6e51acu32,32));
            c.push(m, 0x425ec5);
            { let args = []; c.r[0] = host.call(m, 0x589360, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,c.r[5].wrapping_add(0xffffffa4));
            c.push(m, 0x425ecd);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffa4));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x12u32); }
            { let a = c.r[2].wrapping_add(0x7e4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x425ee5);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x425eebu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_0042ef2c_42ef2c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x42ef2c;
    loop {
        match pc {
            0x42ef2c => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x42ef34);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x6u32); }
            c.push(m, 0x42ef42);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(2,32,c.r[5].wrapping_add(0x8));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,c.get(2,32));
            c.push(m, 0x42ef53);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64efc8, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x8),32); c.push(m, 0x42ef5e); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x42ef66);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb94u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x5u32); }
            { let a = c.r[1].wrapping_add(0x218); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x42ef7e);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x42ef84u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_004338a9_4338a9(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4338a9;
    loop {
        match pc {
            0x4338a9 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x4338b1);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x9u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x5u32); }
            c.push(m, 0x4338bf);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x4338c7);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let a = c.r[1].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697404u32);
            c.push(m, 0x4338e5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x4338f0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x424d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x4338fa);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x433902);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xfu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[2].wrapping_add(0x7e4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x433918);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x433920);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x4u32); }
            { let a = c.r[1].wrapping_add(0x7fc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x433938);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x43393eu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00433d4c_433d4c(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x433d4c;
    loop {
        match pc {
            0x433d4c => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            { let a = c.r[5].wrapping_add(0xffffffdc); wm(m,a,32,c.get(0,32)); }
            c.set(0,32,0x433d58u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00433d7e_433d7e(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x433d7e;
    loop {
        match pc {
            0x433d7e => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffd4),32));
            c.set(0,32,0x433d8au32);
            { let a = c.r[5].wrapping_add(0xffffffd0); wm(m,a,32,c.get(2,32)); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_0043c4fe_43c4fe(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43c4fe;
    loop {
        match pc {
            0x43c4fe => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c506);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x9u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x2u32); }
            c.push(m, 0x43c514);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c51c);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let a = c.r[1].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x697840u32);
            c.push(m, 0x43c53a);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x43c545);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x424d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c54f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c557);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xfu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            { let a = c.r[2].wrapping_add(0x7e4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43c56d);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c575);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x1u32); }
            { let a = c.r[1].wrapping_add(0x7fc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe0));
            c.push(m, 0x43c58d);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x43c593u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_0043dfa8_43dfa8(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43dfa8;
    loop {
        match pc {
            0x43dfa8 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43dfb0);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x9u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x8u32); }
            c.push(m, 0x43dfbe);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43dfc6);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6978a4u32);
            { let a = c.r[2].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43dfe4);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x43dfef);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x424d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43dff9);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43e001);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xfu32);
            { let a = c.r[1].wrapping_add(0x7e4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43e017);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            c.push(m, 0x43e01f);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffcc));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x7u32); }
            { let a = c.r[2].wrapping_add(0x7fc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43e037);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x43e03du32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_0043e0b0_43e0b0(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x43e0b0;
    loop {
        match pc {
            0x43e0b0 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e0b8);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x9u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xbu32); }
            c.push(m, 0x43e0c6);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e0ce);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffc4); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6978dcu32);
            { let a = c.r[2].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43e0ec);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x43e0f7);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x424d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e101);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e109);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xfu32);
            { let a = c.r[1].wrapping_add(0x7e4); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e11f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            c.push(m, 0x43e127);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.set(1,32,c.r[5].wrapping_add(0xffffffe8));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0xau32); }
            { let a = c.r[2].wrapping_add(0x7fc); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x43e13f);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x43e145u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00464583_464583(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x464583;
    loop {
        match pc {
            0x464583 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0x8),32));
            { let a = c.r[5].wrapping_add(0xffffffe8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(0,32); let r = c.add(a,0x2u32,32); c.set(0,32,r); }
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(13) { pc = 0x464592; continue; }
            pc = 0x464590; continue;
            }
            0x464590 => {
            { let a = c.get(0,32); let r = c.xor(a,c.get(0,32),32); c.set(0,32,r); }
            pc = 0x464592; continue;
            }
            0x464592 => {
            c.push(m,c.get(0,32));
            c.push(m, 0x464598);
            { let args = []; c.r[0] = host.call(m, 0x64f7b9, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,c.get(0,32)); }
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.set(0,32,0x4645a4u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_0047252d_47252d(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x47252d;
    loop {
        match pc {
            0x47252d => {
            c.set(0,32,0x472533u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00499869_499869(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x499869;
    loop {
        match pc {
            0x499869 => {
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x499871);
            { let args = []; c.r[0] = host.call(m, 0x46c320, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.push(m,0x9u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x6u32); }
            c.push(m, 0x49987f);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x499887);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m,0x0u32);
            c.push(m,c.get(1,32));
            { let a = c.r[1].wrapping_add(0x7cc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.get(4,32));
            { let a = c.r[5].wrapping_add(0xffffffec); wm(m,a,32,c.get(4,32)); }
            c.push(m,0x6ba838u32);
            c.push(m, 0x4998a5);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x64f2c1, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            c.push(m, 0x4998b0);
            { let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, 0x424d50, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.push(m,0x1u32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x4998ba);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x4998c2);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(2,32,rm(m,0x6cbb40u32,32));
            c.push(m,0xfu32);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            { let a = c.r[2].wrapping_add(0x7e4); wm(m,a,32,c.get(0,32)); }
            c.push(m, 0x4998d8);
            { let args = [m.r32(c.r[4].wrapping_add(4))]; c.r[0] = host.call(m, 0x46c340, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(8);
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x4998e0);
            { let args = []; c.r[0] = host.call(m, 0x46c350, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(1,32,rm(m,0x6cbb40u32,32));
            { let a = c.r[5].wrapping_add(0xfffffffc); wm(m,a,8,0x5u32); }
            { let a = c.r[1].wrapping_add(0x7fc); wm(m,a,32,c.get(0,32)); }
            c.set(1,32,c.r[5].wrapping_add(0xffffffd0));
            c.push(m, 0x4998f8);
            { let args = []; c.r[0] = host.call(m, 0x46c330, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x4998feu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_004d5e78_4d5e78(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d5e78;
    loop {
        match pc {
            0x4d5e78 => {
            c.set(2,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.push(m,c.get(2,32));
            c.push(m, 0x4d5e81);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x4d5e9cu32);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_004d5e8a_4d5e8a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x4d5e8a;
    loop {
        match pc {
            0x4d5e8a => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffe0),32));
            c.push(m,c.get(0,32));
            c.push(m, 0x4d5e93);
            { let args = []; c.r[0] = host.call(m, 0x64f7e2, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(4);
            c.set(0,32,0x4d5e9cu32);
            { let a = c.get(4,32); let r = c.add(a,0x4u32,32); c.set(4,32,r); }
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00544d3a_544d3a(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x544d3a;
    loop {
        match pc {
            0x544d3a => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(2,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[2].wrapping_add(0x10),32); c.push(m, 0x544d46); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,0x544d4cu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00544d64_544d64(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x544d64;
    loop {
        match pc {
            0x544d64 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffe4),32));
            c.push(m,0x0u32);
            c.push(m,0x0u32);
            c.set(0,32,rm(m,c.r[1],32));
            { let target = rm(m,c.r[0].wrapping_add(0x10),32); c.push(m, 0x544d70); let args = [m.r32(c.r[4].wrapping_add(4)), m.r32(c.r[4].wrapping_add(8))]; c.r[0] = host.call(m, target, c.r[1], &args); }
            c.r[4] = c.r[4].wrapping_add(12);
            c.set(0,32,0x544d76u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00547511_547511(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x547511;
    loop {
        match pc {
            0x547511 => {
            c.set(0,32,rm(m,c.r[5].wrapping_add(0xffffffd8),32));
            c.set(0,32,rm(m,c.r[0].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.sub(a,0xdu32,32); }
            if c.cond(4) { pc = 0x547521; continue; }
            pc = 0x54751c; continue;
            }
            0x54751c => {
            { let a = c.get(0,32); c.sub(a,0x5u32,32); }
            if c.cond(5) { pc = 0x547528; continue; }
            pc = 0x547521; continue;
            }
            0x547521 => {
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            pc = 0x547528; continue;
            }
            0x547528 => {
            c.set(0,32,0x54752eu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00547549_547549(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x547549;
    loop {
        match pc {
            0x547549 => {
            c.set(1,32,rm(m,c.r[5].wrapping_add(0xffffffd8),32));
            c.set(0,32,rm(m,c.r[1].wrapping_add(0x8),32));
            { let a = c.get(0,32); c.and(a,c.get(0,32),32); }
            if c.cond(4) { pc = 0x54755a; continue; }
            pc = 0x547553; continue;
            }
            0x547553 => {
            { let a = c.r[5].wrapping_add(0x8); wm(m,a,32,0x1u32); }
            pc = 0x54755a; continue;
            }
            0x54755a => {
            c.set(0,32,0x547560u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00547623_547623(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x547623;
    loop {
        match pc {
            0x547623 => {
            c.set(0,32,0x547629u32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00547644_547644(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x547644;
    loop {
        match pc {
            0x547644 => {
            c.set(0,32,0x54764au32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00549b17_549b17(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x549b17;
    loop {
        match pc {
            0x549b17 => {
            c.set(0,32,0x549b1du32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn Catch_00549b35_549b35(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {
    let mut c = Cpu { r: regs, ..Cpu::default() };
    let mut pc: u32 = 0x549b35;
    loop {
        match pc {
            0x549b35 => {
            c.set(0,32,0x549b3bu32);
            c.r[4] = c.r[4].wrapping_add(4); return c.r;
            }
            _ => unreachable!("pc fuera de la función: {pc:#x}"),
        }
    }
}

pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {
    Some(match name {
        "thunk_FUN_0064ff74_408ed0" => thunk_FUN_0064ff74_408ed0(m, host, regs),
        "thunk_FUN_0064f253_40a3d0" => thunk_FUN_0064f253_40a3d0(m, host, regs),
        "noop_return_40afd0" => noop_return_40afd0(m, host, regs),
        "window_handle_mouse_event_mode_40bc30" => window_handle_mouse_event_mode_40bc30(m, host, regs),
        "window_handle_mouse_event_mode_410310" => window_handle_mouse_event_mode_410310(m, host, regs),
        "CDialog_41c110" => CDialog_41c110(m, host, regs),
        "window_mark_all_dirty_421540" => window_mark_all_dirty_421540(m, host, regs),
        "screen_window_on_mouse_leave_42f980" => screen_window_on_mouse_leave_42f980(m, host, regs),
        "thunk_FUN_00653743_434ad0" => thunk_FUN_00653743_434ad0(m, host, regs),
        "screen_window_on_mouse_leave_435940" => screen_window_on_mouse_leave_435940(m, host, regs),
        "window_noop_slot7_452890" => window_noop_slot7_452890(m, host, regs),
        "edit_box_on_focus_gained_45c220" => edit_box_on_focus_gained_45c220(m, host, regs),
        "window_noop_slot7_466c20" => window_noop_slot7_466c20(m, host, regs),
        "window_noop_slot7_46af00" => window_noop_slot7_46af00(m, host, regs),
        "button_base_dtor_478660" => button_base_dtor_478660(m, host, regs),
        "window_draw_at_origin_4ad530" => window_draw_at_origin_4ad530(m, host, regs),
        "list_box_on_mouse_event_4afe90" => list_box_on_mouse_event_4afe90(m, host, regs),
        "list_box_handle_key_4afea0" => list_box_handle_key_4afea0(m, host, regs),
        "window_draw_at_origin_4b0d50" => window_draw_at_origin_4b0d50(m, host, regs),
        "window_draw_at_origin_4b57b0" => window_draw_at_origin_4b57b0(m, host, regs),
        "window_add_dirty_rect_4b8660" => window_add_dirty_rect_4b8660(m, host, regs),
        "WritePrivateProfileStringA_4be110" => WritePrivateProfileStringA_4be110(m, host, regs),
        "window_draw_at_origin_4c4eb0" => window_draw_at_origin_4c4eb0(m, host, regs),
        "window_draw_at_origin_4c74b0" => window_draw_at_origin_4c74b0(m, host, regs),
        "window_draw_at_origin_4c88d0" => window_draw_at_origin_4c88d0(m, host, regs),
        "window_draw_at_origin_4c94f0" => window_draw_at_origin_4c94f0(m, host, regs),
        "window_draw_at_origin_4c9dc0" => window_draw_at_origin_4c9dc0(m, host, regs),
        "text_box_update_borders_4caac0" => text_box_update_borders_4caac0(m, host, regs),
        "window_draw_at_origin_4cbcf0" => window_draw_at_origin_4cbcf0(m, host, regs),
        "window_draw_at_origin_4cf480" => window_draw_at_origin_4cf480(m, host, regs),
        "window_draw_at_origin_4d0290" => window_draw_at_origin_4d0290(m, host, regs),
        "window_on_mouse_press_4d0b10" => window_on_mouse_press_4d0b10(m, host, regs),
        "window_draw_at_origin_4d1700" => window_draw_at_origin_4d1700(m, host, regs),
        "window_draw_at_origin_4d27b0" => window_draw_at_origin_4d27b0(m, host, regs),
        "letter_free_and_reset_51a880" => letter_free_and_reset_51a880(m, host, regs),
        "town_free_building_records_51e380" => town_free_building_records_51e380(m, host, regs),
        "operation_queue_free_543940" => operation_queue_free_543940(m, host, regs),
        "town_anim_state_reset_frame_5626d0" => town_anim_state_reset_frame_5626d0(m, host, regs),
        "town_anim_state_reset_frame_567720" => town_anim_state_reset_frame_567720(m, host, regs),
        "town_anim_state_reset_frame_572db0" => town_anim_state_reset_frame_572db0(m, host, regs),
        "window_base44_begin_close_5a8660" => window_base44_begin_close_5a8660(m, host, regs),
        "window_handle_mouse_event_mode_5cb900" => window_handle_mouse_event_mode_5cb900(m, host, regs),
        "window_handle_mouse_event_mode_5dd8d0" => window_handle_mouse_event_mode_5dd8d0(m, host, regs),
        "window_handle_mouse_event_mode_5eb3b0" => window_handle_mouse_event_mode_5eb3b0(m, host, regs),
        "window_handle_mouse_event_mode_5f3500" => window_handle_mouse_event_mode_5f3500(m, host, regs),
        "siege_state_reset_6278e0" => siege_state_reset_6278e0(m, host, regs),
        "Catch_00425e83_425e83" => Catch_00425e83_425e83(m, host, regs),
        "Catch_0042ef2c_42ef2c" => Catch_0042ef2c_42ef2c(m, host, regs),
        "Catch_004338a9_4338a9" => Catch_004338a9_4338a9(m, host, regs),
        "Catch_00433d4c_433d4c" => Catch_00433d4c_433d4c(m, host, regs),
        "Catch_00433d7e_433d7e" => Catch_00433d7e_433d7e(m, host, regs),
        "Catch_0043c4fe_43c4fe" => Catch_0043c4fe_43c4fe(m, host, regs),
        "Catch_0043dfa8_43dfa8" => Catch_0043dfa8_43dfa8(m, host, regs),
        "Catch_0043e0b0_43e0b0" => Catch_0043e0b0_43e0b0(m, host, regs),
        "Catch_00464583_464583" => Catch_00464583_464583(m, host, regs),
        "Catch_0047252d_47252d" => Catch_0047252d_47252d(m, host, regs),
        "Catch_00499869_499869" => Catch_00499869_499869(m, host, regs),
        "Catch_004d5e78_4d5e78" => Catch_004d5e78_4d5e78(m, host, regs),
        "Catch_004d5e8a_4d5e8a" => Catch_004d5e8a_4d5e8a(m, host, regs),
        "Catch_00544d3a_544d3a" => Catch_00544d3a_544d3a(m, host, regs),
        "Catch_00544d64_544d64" => Catch_00544d64_544d64(m, host, regs),
        "Catch_00547511_547511" => Catch_00547511_547511(m, host, regs),
        "Catch_00547549_547549" => Catch_00547549_547549(m, host, regs),
        "Catch_00547623_547623" => Catch_00547623_547623(m, host, regs),
        "Catch_00547644_547644" => Catch_00547644_547644(m, host, regs),
        "Catch_00549b17_549b17" => Catch_00549b17_549b17(m, host, regs),
        "Catch_00549b35_549b35" => Catch_00549b35_549b35(m, host, regs),
        _ => return None,
    })
}
