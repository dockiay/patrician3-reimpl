//! `0x55E8C0`: paso por fotograma de `TownWalker` (avance de la animación, huellas, velocidad, carro, asedio, giro y avance).
//!
//! El original encadena muchos caminos que comparten registros: algunos argumentos de las llamadas llevan en sus bits altos lo que
//! quedaba en el registro (el original solo carga el byte o la palabra baja). El porte sigue los registros del original con las
//! llamadas interceptadas, que solo cambian EAX, para que esos bits coincidan. Las variables locales viven en la pila del original
//! (`esp0 = esp - 0x44`): `+0x10` animación nueva, `+0x14` estado de animación (`this+0x2C`), `+0x18` velocidad, `+0x1C`/`+0x20`
//! casilla de destino, `+0x2C`/`+0x30` casilla actual y `+0x34` rectángulo para las listas de casillas.

use super::mover::{
    BEGIN_STEP, FIND_ANIMATION_INDEX, FLOW_DISTANCE, SIEGE_GRIDS, SIEGE_TABLE, STEP_CELL,
};
use super::entity::{sx16, PIXEL_TO_CELL};
use super::sprite::UI;
use super::{Host, Mem};

const SET_DEFAULT_ANIM: u32 = 0x0055_76B0;
const ADVANCE_FRAME: u32 = 0x0055_7340;
const SELECT_ANIM: u32 = 0x0055_7730;
const CHECK_STEP_CLEAR: u32 = 0x0055_A2C0;
const CLEAR_CELL_MARK: u32 = 0x0063_0D40;
const REMOVE_TILES: u32 = 0x0055_E250;
const INSERT_TILES: u32 = 0x0055_E430;
const SIEGE_ACTIVE: u32 = 0x0063_3AE0;
const BEGIN_MOVE: u32 = 0x0055_CF60;
const HEADING_TO_TARGET: u32 = 0x0055_D910;
const CHOOSE_TARGET: u32 = 0x0055_D9D0;
const RESET_FLOW: u32 = 0x0062_B860;
const ADD_FLOW_SEED: u32 = 0x0062_B960;
const BUILDING_RECORD: u32 = 0x0051_F1F0;
const SOUND_LIST_SIZE: u32 = 0x0044_3690;
const RAND: u32 = 0x0063_9C03;
const SELECT_STATE_ANIM: u32 = 0x0056_0970;
const PICK_QUEUE_STEP: u32 = 0x0056_0CB0;
const PICK_NEXT_TILE: u32 = 0x0055_FFF0;
const SINGLE_OCCUPANT: u32 = 0x0055_D700;
const UPDATE_ENTRY_MARKERS: u32 = 0x0062_DD70;
const TRACKED_DISTANCE: u32 = 0x0063_19D0;
const EMIT_FOOTSTEPS: u32 = 0x0055_D0B0;
const WALL_LEVEL: u32 = 0x0052_52F0;
const APPROACH_PARTNER: u32 = 0x0057_9380;
const CAN_ENGAGE: u32 = 0x0057_1ED0;
const ROTATE_TOWARD: u32 = 0x0055_A1C0;
const PLAY_SOUND_AT: u32 = 0x0059_5B00;
const CELL_WALKABLE: u32 = 0x0059_3580;
const GAME_TICK: u32 = 0x006D_CCF8;
const SOUND_MANAGER: u32 = 0x006C_BDB8;
const DIR_FOOT_ALT: u32 = 0x0067_792D;

macro_rules! lo8 {
    ($r:ident, $v:expr) => {
        $r = ($r & 0xFFFF_FF00) | u32::from($v as u8)
    };
}
macro_rules! lo16 {
    ($r:ident, $v:expr) => {
        $r = ($r & 0xFFFF_0000) | u32::from($v as u16)
    };
}

#[derive(Clone, Copy)]
enum L {
    E8F3,
    E918,
    E99B,
    E9B6,
    EA99,
    EC52,
    EC5E,
    ED36,
    EDBD,
    EDF0,
    EE5E,
    EEAD,
    EF19,
    EF83,
    EFBC,
    F113,
    F14B,
    F18F,
    F1BE,
    F1FE,
    F232,
    F261,
    F31A,
    F324,
    F387,
    F48A,
    F4E3,
    F58F,
    F5DF,
    F690,
    F6E2,
    F6F4,
    F700,
    F722,
    F803,
    F8DE,
    F915,
    F922,
    FA83,
    FB3A(u32),
    FB4B,
    FB4F,
    FADB(u32),
    F9FD,
    FBB7,
    FBEF,
    FC01,
    FCB3,
    FCD0,
    FCEF,
    FCF3,
}

/// `0x55E8C0` (this; conjunto, campo de asedio, mín, máx, sin pausa, permitir giro): devuelve AL.
pub fn walker_update(m: &mut dyn Mem, h: &mut dyn Host, esi: u32, esp: u32) -> u8 {
    let s0 = esp.wrapping_sub(0x44);
    let l = move |o: u32| s0.wrapping_add(o);
    let (a0, a1, a2, a3, a4, a5) = (l(0x48), l(0x4C), l(0x50), l(0x54), l(0x58), l(0x5C));
    let mut eax: u32 = 0;
    let mut ebx: u32 = 0;
    let mut ecx: u32 = esi;
    let mut edx: u32 = 0;
    let mut edi: u32 = 0;
    let mut ebp: u32 = 0;
    // Registros de la tabla de asedio (`0x6DF270`, 4132 bytes por bando): EAX base, EDX índice, ECX registro.
    let side = |m: &mut dyn Mem| u32::from(m.r8(esi + 0x6A));
    macro_rules! rec {
        () => {{
            let s = side(m);
            ecx = s << 7;
            ecx = ecx.wrapping_add(s);
            edx = s.wrapping_add(ecx.wrapping_mul(8));
            eax = m.r32(SIEGE_TABLE);
            eax.wrapping_add(edx.wrapping_mul(4))
        }};
    }
    // Rejilla del asedio (`0x6DE508`, 2552 bytes por bando).
    macro_rules! grid_off {
        () => {{
            let s = side(m);
            s.wrapping_mul(5).wrapping_shl(6).wrapping_sub(s).wrapping_mul(8)
        }};
    }
    macro_rules! call {
        ($t:expr, $c:expr, $($a:expr),*) => {{
            eax = h.call(m, $t, $c, &[$($a),*]);
            eax
        }};
    }
    macro_rules! vcall {
        ($slot:expr, $($a:expr),*) => {{
            let vt = m.r32(esi);
            let f = m.r32(vt.wrapping_add($slot));
            ecx = esi;
            eax = h.call(m, f, esi, &[$($a),*]);
            eax
        }};
    }

    let mut pc;
    edx = m.r32(esi + 0x64);
    if edx != 0 {
        if m.r8(esi + 0x58) == 0 {
            pc = L::E918;
        } else {
            m.w32(esi + 0x74, 0);
            eax = m.r32(UI);
            ecx = eax.wrapping_add(0x3F8);
            eax = m.r32(ecx);
            let f = m.r32(eax.wrapping_add(4));
            eax = h.call(m, f, ecx, &[edx]);
            pc = L::E8F3;
        }
    } else {
        pc = L::E8F3;
    }
    loop {
        match pc {
            L::E8F3 => {
                lo8!(eax, m.r8(a5));
                if eax as u8 != 0 {
                    pc = L::E9B6;
                    continue;
                }
                lo8!(eax, m.r8(a4));
                if eax as u8 == 0 {
                    m.w8(a5, 1);
                } else {
                    lo8!(ecx, m.r8(a1));
                    m.w8(a5, ecx as u8);
                }
                pc = L::E9B6;
            }
            L::E918 => {
                lo8!(eax, m.r8(esi + 0x59));
                if eax as u8 == 0 {
                    pc = L::E99B;
                    continue;
                }
                edi = m.r32(SIEGE_GRIDS);
                let s = side(m);
                eax = s;
                ecx = grid_off!() / 8;
                ebx = m.r32(edi.wrapping_add(ecx.wrapping_mul(8)).wrapping_add(0x40));
                if ebx == 0 {
                    pc = L::E99B;
                    continue;
                }
                let r = {
                    ecx = s << 7;
                    ecx = ecx.wrapping_add(s);
                    eax = s.wrapping_add(ecx.wrapping_mul(8));
                    ecx = m.r32(SIEGE_TABLE);
                    ecx.wrapping_add(eax.wrapping_mul(4))
                };
                eax = r;
                if m.r32(r + 0xC) == esi {
                    ecx = m.r32(r + 8);
                    ecx = m.r32(ecx.wrapping_add(0x214));
                    edi = m.r32(ecx.wrapping_add(0x8C));
                    if edi != 0 {
                        m.w32(esi + 0x74, ecx);
                        return 1;
                    }
                }
                if m.r32(r + 0x10) != esi {
                    pc = L::E99B;
                    continue;
                }
                eax = m.r32(r + 8);
                eax = m.r32(eax.wrapping_add(0x210));
                ecx = m.r32(eax.wrapping_add(0x8C));
                if ecx == 0 {
                    pc = L::E99B;
                    continue;
                }
                m.w32(esi + 0x74, eax);
                return 1;
            }
            L::E99B => {
                h.call(m, SET_DEFAULT_ANIM, edx.wrapping_add(0x20), &[2]);
                return 1;
            }
            L::E9B6 => {
                edi = m.r32(a0);
                eax = esi + 8;
                ecx = esi + 0x2C;
                m.w32(l(0x14), ecx);
                call!(ADVANCE_FRAME, ecx, edi, eax, 0);
                lo8!(ebx, eax as u8);
                lo8!(eax, m.r8(esi + 0x15));
                m.w8(l(0x10), ebx as u8);
                if ebx as u8 == eax as u8 {
                    pc = L::EA99;
                    continue;
                }
                edx = m.r32(esi);
                vcall!(0x60, 0);
                if eax as u8 != 0 {
                    eax = sx16(m, esi + 0x84);
                    ecx = sx16(m, esi + 0x82);
                    lo8!(edx, m.r8(esi + 0x59));
                    ecx = esi;
                    let (sy, sx) = (eax, sx16(m, esi + 0x82));
                    call!(EMIT_FOOTSTEPS, esi, edi, edx, sx, sy);
                }
                if ebx as u8 == m.r8(esi + 0x15) {
                    pc = L::EA99;
                    continue;
                }
                edi = m.r32(esi + 0x30);
                ebp = m.r32(GAME_TICK);
                eax = 0;
                edi = edi.wrapping_sub(ebp);
                lo8!(eax, m.r8(esi + 0x86));
                edi = edi.wrapping_mul(eax);
                lo8!(eax, m.r8(esi + 0x34));
                edi = edi.wrapping_add(0x40) >> 7;
                let st = eax as u8;
                if matches!(st, 5 | 0xD | 0x18 | 0x19) {
                    edi <<= 2;
                }
                if st == 2 {
                    lo8!(eax, m.r8(esi + 0x59));
                    if eax as u8 != 0 {
                        let r = rec!();
                        ecx = r;
                        call!(SIEGE_ACTIVE, ecx,);
                        if eax as u8 != 0 {
                            let r = rec!();
                            ecx = m.r32(r + 8);
                            if m.r16(ecx.wrapping_add(0x3A)) == 0 {
                                edi >>= 1;
                            }
                        }
                    }
                }
                edx = m.r32(GAME_TICK).wrapping_add(edi);
                m.w32(esi + 0x30, edx);
                pc = L::EA99;
            }
            L::EA99 => {
                eax = m.r32(UI);
                ecx = u32::from(m.r8(esi + 0x86));
                eax = m.r32(eax.wrapping_add(0x414));
                edx = ecx;
                eax <<= 7;
                edx >>= 1;
                eax = eax.wrapping_add(edx);
                let q = if ecx == 0 { 0 } else { eax / ecx };
                edx = if ecx == 0 { 0 } else { eax % ecx };
                eax = q;
                edi = eax;
                lo8!(eax, m.r8(esi + 0x34));
                m.w32(l(0x18), edi);
                if eax as u8 == 2 {
                    lo8!(eax, m.r8(esi + 0x59));
                    if eax as u8 != 0 {
                        let r = rec!();
                        ecx = r;
                        call!(SIEGE_ACTIVE, ecx,);
                        if eax as u8 != 0 {
                            let r = rec!();
                            ecx = m.r32(r + 8);
                            if m.r16(ecx.wrapping_add(0x3A)) == 0 {
                                edx = edi.wrapping_add(edi);
                                m.w32(l(0x18), edx);
                            }
                        }
                    }
                }
                eax = sx16(m, esi + 0x12);
                ebx = sx16(m, esi + 0xE);
                ecx = l(0x2C);
                m.w32(l(0x20), eax);
                let cell = ecx;
                ecx = m.r32(UI);
                let y = m.r32(l(0x20));
                call!(PIXEL_TO_CELL, ecx, ebx, y, cell);
                lo8!(eax, m.r8(esi + 0x59));
                if eax as u8 == 0 {
                    pc = L::EC5E;
                    continue;
                }
                edx = m.r32(esi);
                vcall!(0x60, 1);
                if eax as u8 == 0 && m.r8(esi + 0x5A) > 0x28 {
                    let r = rec!();
                    ecx = r;
                    call!(SIEGE_ACTIVE, ecx,);
                    if eax as u8 != 0 {
                        pc = L::EC52;
                        continue;
                    }
                }
                // 0x55EB73: registros de edificios del bando.
                eax = 0;
                edx = m.r32(SIEGE_GRIDS);
                let go = grid_off!();
                ecx = go / 8;
                eax = u32::from(m.r16(edx.wrapping_add(go).wrapping_add(0x768)));
                ecx = edx.wrapping_add(go);
                call!(BUILDING_RECORD, ecx, eax);
                edi = eax;
                if edi == 0 {
                    pc = L::EC52;
                    continue;
                }
                loop {
                    // 0x55EBA6
                    ecx = m.r32(l(0x2C));
                    eax = u32::from(m.r8(edi)).wrapping_sub(ecx);
                    eax = (eax as i32).wrapping_abs() as u32;
                    let mut far = (eax as i32) >= 3;
                    if !far {
                        eax = m.r32(l(0x30));
                        ecx = u32::from(m.r8(edi + 1)).wrapping_sub(eax);
                        eax = (ecx as i32).wrapping_abs() as u32;
                        far = (eax as i32) >= 3;
                    }
                    if !far {
                        lo8!(eax, m.r8(edi + 6));
                        far = eax as u8 != 0;
                        if !far {
                            lo8!(eax, m.r8(edi + 3));
                            far = eax as u8 == 0x30 || eax as u8 == 0x31;
                        }
                        if !far {
                            eax = 0;
                            let go = grid_off!();
                            edx = go / 8;
                            eax = m.r32(SIEGE_GRIDS);
                            ecx = eax.wrapping_add(go);
                            call!(WALL_LEVEL, ecx,);
                            eax &= 0xFF;
                            ecx = 0xFF - eax;
                            edx = u32::from(m.r8(edi + 2));
                            if edx == ecx {
                                pc = L::EC5E;
                                break;
                            }
                        }
                    }
                    // 0x55EC18
                    eax = 0;
                    edx = m.r32(SIEGE_GRIDS);
                    let go = grid_off!();
                    lo16!(edi, m.r16(edi + 4));
                    ecx = go / 8;
                    let lim = m.r16(edx.wrapping_add(go).wrapping_add(0x776));
                    ecx = edx.wrapping_add(go);
                    if edi as u16 >= lim {
                        pc = L::EC52;
                        break;
                    }
                    edi &= 0xFFFF;
                    call!(BUILDING_RECORD, ecx, edi);
                    edi = eax;
                    if edi == 0 {
                        pc = L::EC52;
                        break;
                    }
                }
            }
            L::EC52 => return 0,
            L::EC5E => {
                let r = rec!();
                ebp = m.r32(r + 8);
                ecx = r;
                lo8!(eax, m.r8(esi + 0x34));
                if eax as u8 != 0xB1 {
                    pc = L::ED36;
                    continue;
                }
                lo16!(eax, m.r16(esi + 0x68));
                if eax as u16 == 0 {
                    m.w16(esi + 0x68, 0xFFFF);
                } else {
                    ecx = m.r32(l(0x18));
                    edi = eax & 0xFFFF;
                    edx = ecx << 4;
                    if edi <= edx {
                        // 0x55ECDD
                        eax = m.r32(l(0x20));
                        ecx = m.r32(UI);
                        edx = l(0x2C);
                        call!(PIXEL_TO_CELL, ecx, ebx, eax, edx);
                        edi = m.r32(a1);
                        lo16!(ecx, m.r16(esi + 0x6E));
                        lo16!(edx, m.r16(esi + 0x6C));
                        call!(CLEAR_CELL_MARK, ebp, edx, ecx, edi);
                        if m.r8(esi + 0x70) == 0 {
                            return 0;
                        }
                        eax = m.r32(l(0x30));
                        ecx = m.r32(l(0x2C));
                        call!(CLEAR_CELL_MARK, ebp, ecx, eax, edi);
                        m.w8(esi + 0x70, 0);
                        return 0;
                    }
                    ecx <<= 4;
                    eax = eax.wrapping_sub(ecx);
                    m.w16(esi + 0x68, eax as u16);
                }
                edi = m.r32(a0);
                eax = l(0x1C);
                call!(REMOVE_TILES, esi, edi, eax);
                call!(INSERT_TILES, esi, edi, l(0x1C));
                return 1;
            }
            L::ED36 => {
                lo8!(edx, m.r8(esi + 0x58));
                if edx as u8 == 0 {
                    pc = L::EDF0;
                    continue;
                }
                if eax as u8 == 0x91 {
                    lo8!(ebx, m.r8(l(0x10)));
                    pc = L::EDBD;
                    continue;
                }
                edx = m.r32(esi);
                vcall!(0x68,);
                if eax as u8 != 4 {
                    lo8!(eax, m.r8(DIR_FOOT_ALT));
                    m.w8(esi + 0x3C, eax as u8);
                }
                ecx = m.r32(a0);
                let st = m.r32(l(0x14));
                call!(SELECT_ANIM, st, ecx, 0x91);
                ecx = m.r32(SOUND_MANAGER);
                lo8!(ebx, eax as u8);
                call!(SOUND_LIST_SIZE, ecx, 0xEE48);
                if eax != 0 {
                    ecx = m.r32(SOUND_MANAGER);
                    call!(SOUND_LIST_SIZE, ecx, 0xEE48);
                    edi = eax;
                    call!(RAND, crate::j07::CDECL,);
                    let n = edi as i32;
                    let rem = if n == 0 { 0 } else { (eax as i32).wrapping_rem(n) };
                    edx = rem as u32;
                    eax = sx16(m, esi + 0xE);
                    ecx = m.r32(UI);
                    edx = edx.wrapping_sub(0x11B8);
                    let y = sx16(m, esi + 0x12);
                    call!(PLAY_SOUND_AT, ecx, eax, y, edx);
                    edx = y;
                }
                pc = L::EDBD;
            }
            L::EDBD => {
                edi = m.r32(a0);
                ecx = l(0x1C);
                call!(REMOVE_TILES, esi, edi, ecx);
                lo8!(edx, m.r8(esi + 0x3C));
                m.w8(esi + 0x14, edx as u8);
                m.w8(esi + 0x15, ebx as u8);
                call!(INSERT_TILES, esi, edi, l(0x1C));
                return 1;
            }
            L::EDF0 => {
                call!(SIEGE_ACTIVE, ecx,);
                let mut skip = eax as u8 != 0 || ebp == 0;
                if !skip {
                    skip = m.r16(ebp + 0x3A) == 0 || m.r8(esi + 0x7B) != 1;
                }
                if !skip {
                    lo16!(ecx, m.r16(ebp + 0x10));
                    skip = ecx as u16 != m.r16(ebp + 0x36);
                    if !skip {
                        lo16!(edx, m.r16(ebp + 0x12));
                        skip = edx as u16 != m.r16(ebp + 0x38);
                    }
                    if !skip {
                        lo8!(eax, m.r8(esi + 0x59));
                        skip = eax as u8 != 0;
                    }
                    if !skip {
                        edx = m.r32(SIEGE_GRIDS);
                        let go = grid_off!();
                        eax = edx.wrapping_add(go).wrapping_add(0x7A4);
                        ecx = m.r32(edx.wrapping_add(go).wrapping_add(0x7AC));
                        edx = m.r32(eax + 0x68);
                        ecx = ecx.wrapping_mul(m.r32(l(0x2C))).wrapping_add(edx);
                        edx = m.r32(l(0x30));
                        if (m.r8(ecx.wrapping_add(edx)) as i8) >= 0 {
                            return 0;
                        }
                    }
                }
                pc = L::EE5E;
            }
            L::EE5E => {
                let r = rec!();
                ecx = r;
                call!(SIEGE_ACTIVE, ecx,);
                if eax as u8 != 0 {
                    if ebp == 0 || m.r16(ebp + 0x3A) == 0 {
                        pc = L::F4E3;
                        continue;
                    }
                    lo16!(ecx, m.r16(ebp + 0x10));
                    if ecx as u16 != m.r16(ebp + 0x3E) {
                        pc = L::F4E3;
                        continue;
                    }
                    lo16!(edx, m.r16(ebp + 0x12));
                    if edx as u16 != m.r16(ebp + 0x40) {
                        pc = L::F4E3;
                        continue;
                    }
                }
                pc = L::EEAD;
            }
            L::EEAD => {
                if m.r8(esi + 0x7B) != 1 {
                    return 0;
                }
                eax = 0;
                edx = m.r32(SIEGE_GRIDS);
                let go = grid_off!();
                eax = edx.wrapping_add(go).wrapping_add(0x7A4);
                ecx = m.r32(l(0x2C));
                edx = m.r32(eax + 8);
                edi = m.r32(eax + 0x68);
                edx = edx.wrapping_mul(ecx).wrapping_add(edi);
                eax = m.r32(l(0x30));
                if (m.r8(edx.wrapping_add(eax)) as i8) >= 0 {
                    pc = L::F4E3;
                    continue;
                }
                lo8!(edx, m.r8(esi + 0x59));
                if edx as u8 != 0 {
                    pc = L::F4E3;
                    continue;
                }
                if ebp != 0 {
                    edi = m.r32(a1);
                    call!(CLEAR_CELL_MARK, ebp, ecx, eax, edi);
                    lo16!(eax, m.r16(esi + 0x6C));
                    ecx = ebp;
                    call!(CLEAR_CELL_MARK, ebp, eax, eax, edi);
                }
                pc = L::EF19;
            }
            L::EF19 => {
                lo8!(eax, m.r8(esi + 0x14));
                lo8!(ecx, m.r8(esi + 0x3C));
                if eax as u8 == ecx as u8 {
                    pc = L::EFBC;
                    continue;
                }
                lo8!(ebx, m.r8(l(0x10)));
                lo8!(eax, m.r8(esi + 0x15));
                if ebx as u8 != eax as u8 {
                    edi = m.r32(a0);
                    ecx = l(0x1C);
                    call!(REMOVE_TILES, esi, edi, ecx);
                    ecx = esi;
                    lo8!(eax, m.r8(esi + 0x14));
                    lo8!(edx, m.r8(esi + 0x3C).wrapping_sub(eax as u8));
                    if (edx as u8 as i8) < 0 {
                        lo8!(eax, (eax as u8).wrapping_sub(0x20));
                        m.w8(esi + 0x57, 0);
                        m.w8(esi + 0x14, eax as u8);
                        lo8!(eax, m.r8(esi + 0x56).wrapping_add(1));
                        m.w8(esi + 0x56, eax as u8);
                    } else {
                        lo8!(eax, (eax as u8).wrapping_add(0x20));
                        m.w8(esi + 0x56, 0);
                        m.w8(esi + 0x14, eax as u8);
                        lo8!(eax, m.r8(esi + 0x57).wrapping_add(1));
                        m.w8(esi + 0x57, eax as u8);
                    }
                    eax = l(0x1C);
                    m.w8(esi + 0x15, ebx as u8);
                    call!(INSERT_TILES, esi, edi, eax);
                }
                pc = L::EF83;
            }
            L::EF83 => {
                ecx = m.r32(l(0x14));
                call!(SET_DEFAULT_ANIM, ecx, 2);
                lo8!(eax, m.r8(esi + 0x56));
                m.w8(esi + 0x55, 3);
                if eax as u8 > 3 || m.r8(esi + 0x57) > 3 {
                    edx = m.r32(esi);
                    vcall!(0x60, 1);
                    if eax as u8 == 0 {
                        return 0;
                    }
                }
                return 1;
            }
            L::EFBC => {
                lo16!(edx, m.r16(esi + 0x38));
                lo16!(ecx, m.r16(esi + 0x3A));
                if edx as u16 != ecx as u16 {
                    pc = L::F387;
                    continue;
                }
                if m.r8(esi + 0x56) > 2 || m.r8(esi + 0x57) > 2 {
                    ecx = esi;
                    call!(CHOOSE_TARGET, esi,);
                }
                ecx = esi;
                call!(HEADING_TO_TARGET, esi,);
                lo8!(edx, m.r8(esi + 0x3D));
                ecx = m.r32(l(0x2C));
                m.w8(a4, eax as u8);
                lo8!(eax, m.r8(esi + 0x14));
                let same = edx as u8 == eax as u8;
                edx = m.r32(l(0x30));
                let dir = eax;
                m.w32(l(0x1C), ecx);
                eax = l(0x20);
                ecx = l(0x1C);
                m.w32(l(0x20), edx);
                lo8!(ebx, u8::from(same));
                ecx = m.r32(UI);
                call!(STEP_CELL, crate::j07::CDECL, l(0x1C), l(0x20), dir);
                lo16!(eax, m.r16(esi + 0x16));
                edx = l(0x1C);
                ecx = esi;
                let w = eax;
                call!(SINGLE_OCCUPANT, esi, edx, w);
                edi = eax;
                if ebx as u8 == 0 {
                    pc = L::F113;
                    continue;
                }
                let mut free = edi != 0 && m.r8(edi + 0x14) == m.r8(esi + 0x14).wrapping_add(0x80);
                if edi != 0 {
                    lo8!(eax, m.r8(esi + 0x14));
                    lo8!(ecx, m.r8(edi + 0x14));
                    lo8!(eax, (eax as u8).wrapping_add(0x80));
                }
                if !free {
                    lo8!(ecx, m.r8(esi + 0x14));
                    eax = esi + 8;
                    let d = ecx;
                    ecx = esi + 0x20;
                    call!(CHECK_STEP_CLEAR, ecx, eax, d, 1, 1);
                    free = eax as u8 != 0;
                }
                if free {
                    // 0x55F05F
                    lo8!(edx, m.r8(esi + 0x14));
                    lo16!(eax, m.r16(l(0x1C)));
                    lo16!(ecx, m.r16(l(0x20)));
                    m.w8(esi + 0x3C, edx as u8);
                    edx = m.r32(a0);
                    m.w16(esi + 0x6C, eax as u16);
                    eax = l(0x1C);
                    m.w16(esi + 0x6E, ecx as u16);
                    ecx = esi;
                    call!(BEGIN_MOVE, esi, eax, edx);
                    ecx = m.r32(l(0x14));
                    call!(SET_DEFAULT_ANIM, ecx, 2);
                    return 1;
                }
                // 0x55F09F
                if edi == 0 {
                    pc = L::F113;
                    continue;
                }
                lo8!(eax, m.r8(edi + 0x34));
                if eax as u8 != 2 && eax as u8 != 6 {
                    pc = L::F113;
                    continue;
                }
                lo8!(eax, m.r8(esi + 0x34));
                edi = m.r32(a0);
                let mut reload = true;
                if eax as u8 != 5 {
                    ecx = m.r32(l(0x14));
                    call!(SELECT_ANIM, ecx, edi, 5);
                    lo8!(ebx, eax as u8);
                    if ebx as u8 != 0xFF {
                        reload = false;
                    } else {
                        lo8!(ecx, m.r8(esi + 0x15));
                        m.w8(l(0x10), ecx as u8);
                    }
                }
                if reload {
                    lo8!(ebx, m.r8(l(0x10)));
                }
                lo8!(edx, m.r8(esi + 0x14));
                eax = l(0x1C);
                ecx = esi;
                m.w8(esi + 0x3C, edx as u8);
                call!(BEGIN_MOVE, esi, eax, edi);
                ecx = l(0x34);
                call!(REMOVE_TILES, esi, edi, ecx);
                edx = l(0x34);
                ecx = esi;
                m.w8(esi + 0x15, ebx as u8);
                call!(INSERT_TILES, esi, edi, edx);
                return 1;
            }
            L::F113 => {
                lo8!(eax, m.r8(esi + 0x3D));
                lo8!(edx, m.r8(esi + 0x14));
                lo8!(ecx, eax as u8);
                ecx = ecx.wrapping_sub(0x80);
                if ecx as u8 != edx as u8 {
                    let d = eax;
                    eax = esi + 8;
                    ecx = esi + 0x20;
                    call!(CHECK_STEP_CLEAR, ecx, esi + 8, d, 1, 1);
                    if eax as u8 != 0 {
                        lo8!(edx, m.r8(esi + 0x3D));
                        lo8!(eax, m.r8(esi + 0x14));
                        m.w8(esi + 0x3C, edx as u8);
                        m.w8(esi + 0x3D, eax as u8);
                        pc = L::F324;
                        continue;
                    }
                }
                pc = L::F14B;
            }
            L::F14B => {
                lo8!(ebx, m.r8(a4));
                edx = u32::from(m.r8(esi + 0x14));
                lo8!(ecx, ebx as u8);
                ecx = ecx.wrapping_sub(0x80);
                edi = ecx & 0xFF;
                if edi != edx {
                    eax = m.r32(a4);
                    let d = eax;
                    eax = esi + 8;
                    ecx = esi + 0x20;
                    call!(CHECK_STEP_CLEAR, ecx, esi + 8, d, 1, 1);
                    if eax as u8 != 0 {
                        lo8!(ecx, m.r8(esi + 0x14));
                        m.w8(esi + 0x3C, ebx as u8);
                        m.w8(esi + 0x3D, ecx as u8);
                        pc = L::F324;
                        continue;
                    }
                }
                pc = L::F18F;
            }
            L::F18F => {
                if ebx as u8 != m.r8(esi + 0x14) {
                    lo8!(ebx, (ebx as u8).wrapping_add(0x80));
                    eax = esi + 8;
                    ecx = esi + 0x20;
                    call!(CHECK_STEP_CLEAR, ecx, esi + 8, ebx, 1, 1);
                    if eax as u8 != 0 {
                        lo8!(edx, m.r8(esi + 0x14));
                        m.w8(esi + 0x3C, ebx as u8);
                        m.w8(esi + 0x3D, edx as u8);
                        pc = L::F324;
                        continue;
                    }
                    lo8!(ebx, m.r8(a4));
                }
                pc = L::F1BE;
            }
            L::F1BE => {
                lo8!(eax, m.r8(esi + 0x3D));
                lo8!(edx, m.r8(esi + 0x14));
                lo8!(ecx, eax as u8);
                m.w8(esi + 0x24, 0);
                ecx = ecx.wrapping_sub(0x80);
                if ecx as u8 != edx as u8 {
                    let d = eax;
                    eax = esi + 8;
                    ecx = esi + 0x20;
                    call!(CHECK_STEP_CLEAR, ecx, esi + 8, d, 1, 1);
                    if eax as u8 != 0 {
                        lo8!(edx, m.r8(esi + 0x3D));
                        lo8!(eax, m.r8(esi + 0x14));
                        m.w8(esi + 0x3C, edx as u8);
                        m.w8(esi + 0x3D, eax as u8);
                        m.w8(esi + 0x24, 1);
                        pc = L::F324;
                        continue;
                    }
                }
                pc = L::F1FE;
            }
            L::F1FE => {
                ecx = u32::from(m.r8(esi + 0x14));
                if edi != ecx {
                    edx = m.r32(a4);
                    eax = esi + 8;
                    ecx = esi + 0x20;
                    call!(CHECK_STEP_CLEAR, ecx, esi + 8, edx, 1, 1);
                    if eax as u8 != 0 {
                        lo8!(eax, m.r8(esi + 0x14));
                        m.w8(esi + 0x3C, ebx as u8);
                        m.w8(esi + 0x3D, eax as u8);
                        m.w8(esi + 0x24, 1);
                        pc = L::F324;
                        continue;
                    }
                }
                pc = L::F232;
            }
            L::F232 => {
                if ebx as u8 != m.r8(esi + 0x14) {
                    lo8!(ebx, (ebx as u8).wrapping_add(0x80));
                    eax = esi + 8;
                    ecx = esi + 0x20;
                    call!(CHECK_STEP_CLEAR, ecx, esi + 8, ebx, 1, 1);
                    if eax as u8 != 0 {
                        lo8!(ecx, m.r8(esi + 0x14));
                        m.w8(esi + 0x3C, ebx as u8);
                        m.w8(esi + 0x3D, ecx as u8);
                        m.w8(esi + 0x24, 1);
                        pc = L::F324;
                        continue;
                    }
                }
                pc = L::F261;
            }
            L::F261 => {
                ecx = esi;
                m.w8(esi + 0x24, 1);
                call!(CHOOSE_TARGET, esi,);
                ecx = esi;
                call!(HEADING_TO_TARGET, esi,);
                lo8!(eax, m.r8(esi + 0x3D));
                edi = esi + 0x20;
                ebx = esi + 8;
                ecx = edi;
                m.w8(esi + 0x3C, eax as u8);
                call!(CHECK_STEP_CLEAR, edi, ebx, eax, 1, 1);
                if eax as u8 != 0 {
                    pc = L::F31A;
                    continue;
                }
                call!(RAND, edi,);
                lo8!(ecx, m.r8(esi + 0x3D));
                let v = ((eax as u8) & 1).wrapping_shl(7).wrapping_add(ecx as u8).wrapping_sub(0x40);
                lo8!(eax, v);
                ecx = edi;
                m.w8(esi + 0x3C, eax as u8);
                call!(CHECK_STEP_CLEAR, edi, ebx, eax, 1, 1);
                if eax as u8 != 0 {
                    pc = L::F31A;
                    continue;
                }
                lo8!(eax, m.r8(esi + 0x3C).wrapping_add(0x80));
                ecx = edi;
                m.w8(esi + 0x3C, eax as u8);
                call!(CHECK_STEP_CLEAR, edi, ebx, eax, 1, 1);
                if eax as u8 != 0 {
                    pc = L::F31A;
                    continue;
                }
                eax = m.r32(esi + 0x48);
                ebp = u32::MAX;
                if eax != ebp {
                    edx = m.r32(GAME_TICK);
                    m.w32(esi + 0x48, edx);
                }
                lo8!(eax, m.r8(esi + 0x3D).wrapping_add(0x80));
                ecx = edi;
                m.w8(esi + 0x3C, eax as u8);
                call!(CHECK_STEP_CLEAR, edi, ebx, eax, 1, 1);
                if eax as u8 != 0 {
                    pc = L::F31A;
                    continue;
                }
                lo8!(eax, m.r8(esi + 0x14));
                edx = m.r32(esi);
                m.w8(esi + 0x3C, eax as u8);
                m.w32(esi + 0x48, ebp);
                vcall!(0x60, 1);
                return eax as u8;
            }
            L::F31A => {
                eax = 0;
                m.w16(esi + 0x3A, 0);
                m.w16(esi + 0x38, 0);
                pc = L::F324;
            }
            L::F324 => {
                eax = m.r32(l(0x2C));
                ecx = m.r32(l(0x30));
                lo8!(edx, m.r8(esi + 0x3C));
                m.w32(l(0x1C), eax);
                m.w32(l(0x20), ecx);
                eax = l(0x20);
                ecx = m.r32(UI);
                call!(STEP_CELL, crate::j07::CDECL, l(0x1C), l(0x20), edx);
                edx = m.r32(a0);
                eax = l(0x1C);
                ecx = esi;
                call!(BEGIN_MOVE, esi, eax, edx);
                lo16!(ecx, m.r16(l(0x1C)));
                lo16!(edx, m.r16(l(0x20)));
                m.w16(esi + 0x6C, ecx as u16);
                ecx = m.r32(l(0x14));
                m.w16(esi + 0x6E, edx as u16);
                call!(SET_DEFAULT_ANIM, ecx, 2);
                return 1;
            }
            L::F387 => {
                lo8!(eax, m.r8(esi + 0x34));
                ebp = m.r32(esi + 0xC);
                edi = m.r32(esi + 0x10);
                m.w32(a1, ebp);
                let st = eax as u8;
                ebx = m.r32(l(0x18));
                if st == 6 || st == 0xB {
                    ebx = ebx.wrapping_add(ebx);
                }
                if st == 2 || st == 6 || st == 0xB {
                    eax = edx & 0xFFFF;
                    ecx &= 0xFFFF;
                    ecx = ecx.wrapping_sub(eax);
                    if ebx > ecx {
                        ebx = ecx;
                    }
                    ecx = m.r32(esi + 0x40);
                    edx = edx.wrapping_add(ebx);
                    m.w16(esi + 0x38, edx as u16);
                    edx = m.r32(esi + 0x44);
                    ecx = ecx.wrapping_mul(ebx);
                    edx = edx.wrapping_mul(ebx);
                    ebp = ebp.wrapping_add(ecx);
                    edi = edi.wrapping_add(edx);
                    m.w32(a1, ebp);
                }
                eax = sx16(m, esi + 0xE);
                ecx = ebp >> 16;
                let mut moved = ecx != eax;
                if !moved {
                    eax = sx16(m, esi + 0x12);
                    edx = edi >> 16;
                    moved = edx != eax;
                }
                if !moved {
                    lo8!(ebx, 0);
                    pc = L::F48A;
                    continue;
                }
                eax = edi >> 16;
                edx = l(0x1C);
                let px = ecx;
                ecx = m.r32(UI);
                lo8!(ebx, 1);
                call!(PIXEL_TO_CELL, ecx, px, eax, edx);
                eax = u32::from(m.r16(esi + 0x38));
                ecx = u32::from(m.r16(esi + 0x3A));
                ecx = ecx.wrapping_add(ecx);
                eax = eax.wrapping_mul(3);
                if (eax as i32) <= (ecx as i32) {
                    pc = L::F48A;
                    continue;
                }
                lo8!(eax, m.r8(esi + 0x54));
                if eax as u8 != 0 {
                    pc = L::F48A;
                    continue;
                }
                edx = m.r32(a0);
                lo16!(ecx, m.r16(esi + 0x16));
                let w = ecx;
                ecx = m.r32(UI);
                lo8!(eax, m.r8(edx + 8));
                edx = a4;
                m.w8(a4, eax as u8);
                eax = l(0x1C);
                call!(CELL_WALKABLE, ecx, eax, edx, w);
                if eax as u8 == 0 {
                    ecx = m.r32(a0);
                    edx = l(0x1C);
                    let c = ecx;
                    ecx = esi;
                    call!(BEGIN_MOVE, esi, edx, c);
                    lo8!(eax, m.r8(esi + 0x54));
                    if eax as u8 != 0 {
                        return ebx as u8;
                    }
                }
                ebp = m.r32(a1);
                pc = L::F48A;
            }
            L::F48A => {
                lo8!(eax, m.r8(l(0x10)));
                lo8!(ecx, m.r8(esi + 0x15));
                let ch = (eax as u8 != ecx as u8) as u8 | ebx as u8;
                lo8!(ecx, ch);
                if ch == 0 {
                    m.w32(esi + 0x10, edi);
                    m.w32(esi + 0xC, ebp);
                    return 1;
                }
                ebx = m.r32(a0);
                edx = l(0x34);
                ecx = esi;
                call!(REMOVE_TILES, esi, ebx, edx);
                lo8!(eax, m.r8(l(0x10)));
                ecx = l(0x34);
                m.w32(esi + 0xC, ebp);
                m.w32(esi + 0x10, edi);
                m.w8(esi + 0x15, eax as u8);
                call!(INSERT_TILES, esi, ebx, ecx);
                return 1;
            }
            L::F4E3 => {
                lo8!(edx, m.r8(l(0x10)));
                lo8!(eax, m.r8(esi + 0x15));
                if edx as u8 != eax as u8 {
                    ecx = m.r32(l(0x10));
                    let c = ecx;
                    eax = m.r32(esi);
                    vcall!(0x64, c);
                    m.w8(l(0x10), eax as u8);
                }
                ecx = sx16(m, esi + 0x12);
                eax = sx16(m, esi + 0xE);
                edx = l(0x2C);
                let y = ecx;
                ecx = m.r32(UI);
                call!(PIXEL_TO_CELL, ecx, eax, y, edx);
                lo8!(eax, m.r8(esi + 0x7B));
                ecx = m.r32(l(0x30));
                let flag = eax as u8 & 1;
                eax = m.r32(l(0x2C));
                if flag != 0 {
                    pc = L::F5DF;
                    continue;
                }
                lo16!(ebx, m.r16(ebp + 0x10));
                lo16!(edi, m.r16(ebp + 0x36));
                if ebx as u16 != edi as u16 {
                    pc = L::F5DF;
                    continue;
                }
                if m.r16(ebp + 0x12) == edi as u16 {
                    edx = edi & 0xFFFF;
                    if eax == edx && ecx == edx {
                        pc = L::F58F;
                        continue;
                    }
                }
                lo16!(edx, m.r16(ebp + 0x12));
                if edx as u16 != m.r16(ebp + 0x38) {
                    pc = L::F5DF;
                    continue;
                }
                edx = m.r32(UI);
                edi = m.r32(edx + 0xC314).wrapping_sub(3);
                if eax > edi || eax < 1 {
                    pc = L::F58F;
                    continue;
                }
                edx = m.r32(edx + 0xC318).wrapping_sub(5);
                if ecx > edx || ecx < 4 {
                    pc = L::F58F;
                    continue;
                }
                pc = L::F5DF;
            }
            L::F58F => {
                edx = m.r32(a1);
                call!(CLEAR_CELL_MARK, ebp, eax, ecx, edx);
                eax = m.r32(esi);
                vcall!(0x68,);
                eax &= 0xFF;
                if m.r16(ebp.wrapping_add(eax * 4).wrapping_add(0x21E)) == 0 {
                    return 0;
                }
                edx = m.r32(esi);
                vcall!(0x68,);
                eax &= 0xFF;
                let p = ebp.wrapping_add(eax * 4).wrapping_add(0x21E);
                let v = m.r16(p).wrapping_sub(1);
                m.w16(p, v);
                return 0;
            }
            L::F5DF => {
                lo16!(edx, m.r16(esi + 0x38));
                if edx as u16 != m.r16(esi + 0x3A) {
                    pc = L::FCF3;
                    continue;
                }
                if ebp != 0 {
                    lo8!(edx, m.r8(esi + 0x60));
                    if edx as u8 != 0 {
                        m.w16(esi + 0x80, 0);
                    } else {
                        ecx = ebp;
                        call!(TRACKED_DISTANCE, ebp,);
                        m.w16(esi + 0x80, eax as u16);
                        if eax as u16 > 0x19 {
                            lo8!(eax, m.r8(ebp + 0x78));
                            if eax as u8 == 0 {
                                m.w16(esi + 0x80, 0x19);
                            }
                        }
                        ecx = m.r32(l(0x30));
                        eax = m.r32(l(0x2C));
                    }
                    let (x, y) = (eax, ecx);
                    ecx = ebp;
                    call!(FLOW_DISTANCE, ebp, x, y);
                    ecx = m.r32(l(0x30));
                    ebx = eax;
                    eax = m.r32(l(0x2C));
                } else {
                    ebx = m.r32(a4);
                }
                // 0x55F648
                if m.r8(esi + 0x7B) & 1 == 0 {
                    lo16!(edx, m.r16(ebp + 0x3E));
                    if m.r16(ebp + 0x10) == edx as u16 {
                        lo16!(edi, m.r16(ebp + 0x40));
                        if m.r16(ebp + 0x12) == edi as u16 {
                            edx &= 0xFFFF;
                            if eax == edx {
                                edi &= 0xFFFF;
                                if ecx == edi {
                                    edx = m.r32(a1);
                                    call!(CLEAR_CELL_MARK, ebp, eax, ecx, edx);
                                    return 0;
                                }
                            }
                        }
                    }
                }
                pc = L::F690;
            }
            L::F690 => {
                edx = u32::from(m.r16(esi + 0x6C));
                if eax != edx {
                    pc = L::F922;
                    continue;
                }
                edi = u32::from(m.r16(esi + 0x6E));
                if ecx != edi {
                    pc = L::F922;
                    continue;
                }
                lo8!(edx, m.r8(esi + 0x59));
                if edx as u8 == 0 {
                    pc = L::F722;
                    continue;
                }
                let (x, y) = (eax, ecx);
                edx = m.r32(a5);
                let p5 = edx;
                edx = m.r32(a1);
                eax = m.r32(a0);
                ecx = esi;
                call!(PICK_QUEUE_STEP, esi, eax, x, y, edx, p5);
                m.w8(a5, eax as u8);
                if eax as u8 != 0 {
                    lo8!(ebx, 0);
                    pc = L::F6F4;
                    continue;
                }
                edx = m.r32(esi);
                vcall!(0x58, 1);
                lo8!(ebx, eax as u8);
                pc = L::F6E2;
            }
            L::F6E2 => {
                if ebx as u8 == 1 {
                    // 0x55FCCD: `cmp bl, 4` deja ZF a 0.
                    pc = L::FCD0;
                    continue;
                }
                if ebx as u8 == 4 {
                    pc = L::FCD0;
                    continue;
                }
                pc = L::F6F4;
            }
            L::F6F4 => {
                lo8!(eax, m.r8(a5));
                if eax as u8 == 0 {
                    pc = L::FBB7;
                    continue;
                }
                pc = L::F700;
            }
            L::F700 => {
                eax = u32::from(m.r8(esi + 0x34)).wrapping_sub(2);
                if eax > 0xCC {
                    pc = L::FB4F;
                    continue;
                }
                let st = eax + 2;
                edx = 0;
                let want = m.r8(a4) != 0;
                edi = m.r32(l(0x14));
                let set = m.r32(a0);
                let find = |m: &mut dyn Mem, h: &mut dyn Host, key: u32| h.call(m, FIND_ANIMATION_INDEX, edi, &[set, key]);
                pc = match st {
                    2 => {
                        if !want {
                            L::FB4F
                        } else {
                            eax = find(m, h, 0x8C);
                            if eax as u8 != 0xFF { L::FB3A(0x8C) } else { L::FB3A(0xB) }
                        }
                    }
                    0x13 => {
                        if !want {
                            L::FB4F
                        } else {
                            L::FA83
                        }
                    }
                    0xB => {
                        if want {
                            L::FB4F
                        } else {
                            eax = find(m, h, 0x13);
                            if eax as u8 != 0xFF {
                                call!(SELECT_ANIM, edi, set, 0x13);
                                L::FB4B
                            } else {
                                eax = find(m, h, 0x12);
                                if eax as u8 != 0xFF {
                                    L::FADB(0x92)
                                } else {
                                    eax = find(m, h, 0xC);
                                    if eax as u8 != 0xFF { L::FADB(0xEC) } else { L::F9FD }
                                }
                            }
                        }
                    }
                    5 | 0xA4 | 0xB2 | 0xCC => {
                        if want {
                            eax = find(m, h, 0x8C);
                            if eax as u8 != 0xFF { L::FB3A(0x8C) } else { L::FB3A(0xB) }
                        } else {
                            call!(SELECT_ANIM, edi, set, 2);
                            L::FB4B
                        }
                    }
                    0xD | 0xF | 0x14 | 0xAC | 0xCE => {
                        if want {
                            L::FA83
                        } else {
                            eax = find(m, h, 0x13);
                            if eax as u8 != 0xFF {
                                call!(SELECT_ANIM, edi, set, 0x13);
                                L::FB4B
                            } else {
                                eax = find(m, h, 0x12);
                                if eax as u8 != 0xFF {
                                    L::FADB(0x92)
                                } else {
                                    eax = find(m, h, 0xC);
                                    if eax as u8 != 0xFF { L::FADB(0xEC) } else { L::F9FD }
                                }
                            }
                        }
                    }
                    0x10 | 0xAE => L::FB3A(0xEE),
                    _ => L::FB4F,
                };
            }
            L::FA83 => {
                let st = m.r32(l(0x14));
                let set = m.r32(a0);
                call!(SELECT_ANIM, st, set, 0xB);
                pc = L::FB4B;
            }
            L::FB3A(key) => {
                let st = m.r32(l(0x14));
                let set = m.r32(a0);
                call!(SELECT_ANIM, st, set, key);
                m.w16(esi + 0x3A, 1);
                m.w16(esi + 0x38, 0);
                pc = L::FB4B;
            }
            L::FADB(key) => {
                let st = m.r32(l(0x14));
                let set = m.r32(a0);
                call!(SELECT_ANIM, st, set, key);
                let r = eax;
                call!(SET_DEFAULT_ANIM, st, 0xCC);
                m.w8(l(0x10), r as u8);
                m.w16(esi + 0x3A, 1);
                m.w16(esi + 0x38, 0);
                pc = L::FB4F;
            }
            L::F9FD => {
                let st = m.r32(l(0x14));
                let set = m.r32(a0);
                call!(SELECT_ANIM, st, set, 2);
                m.w8(l(0x10), eax as u8);
                m.w16(esi + 0x3A, 1);
                m.w16(esi + 0x38, 0);
                pc = L::FB4F;
            }
            L::FB4B => {
                m.w8(l(0x10), eax as u8);
                pc = L::FB4F;
            }
            L::FB4F => {
                if m.r8(esi + 0x70) == 0 {
                    pc = L::FCF3;
                    continue;
                }
                let st = m.r8(esi + 0x34);
                if st != 2 && st != 0xB && st != 0x13 {
                    pc = L::FCF3;
                    continue;
                }
                if ebp != 0 {
                    let (x, y, k) = (m.r32(l(0x2C)), m.r32(l(0x30)), m.r32(a1));
                    call!(CLEAR_CELL_MARK, ebp, x, y, k);
                }
                if m.r8(esi + 0x59) != 0 {
                    let a5v = m.r8(esi + 0x5A);
                    if m.r8(esi + 0x3C) == m.r8(esi + 0x79) {
                        m.w8(esi + 0x70, 0);
                        m.w8(esi + 0x5A, a5v.wrapping_add(1));
                        pc = L::FCF3;
                        continue;
                    }
                    m.w8(esi + 0x5A, a5v.wrapping_sub(1));
                }
                m.w8(esi + 0x70, 0);
                pc = L::FCF3;
            }
            L::F722 => {
                eax = m.r32(esi);
                vcall!(0x68,);
                let mut to803 = eax as u8 != 0;
                if !to803 {
                    eax = m.r32(esi + 0x8C);
                    to803 = eax == 0;
                }
                if to803 {
                    pc = L::F803;
                    continue;
                }
                ecx = esi;
                call!(APPROACH_PARTNER, esi,);
                lo8!(ebx, eax as u8);
                if ebx as u8 != 0 {
                    pc = L::F915;
                    continue;
                }
                ecx = m.r32(esi + 0x8C);
                if ecx == 0 {
                    pc = L::F8DE;
                    continue;
                }
                eax = m.r32(ecx + 0x8C);
                if eax == 0 {
                    call!(CAN_ENGAGE, ecx,);
                    if eax as u8 != 0 {
                        m.w16(esi + 0x80, 0);
                    }
                }
                let r = rec!();
                edi = m.r32(r + 8);
                lo8!(eax, m.r8(edi + 0x78));
                if eax as u8 != 0 {
                    pc = L::F8DE;
                    continue;
                }
                ecx = m.r32(esi + 0x8C);
                lo8!(edx, m.r8(ecx + 0x90));
                m.w8(l(0x1C), edx as u8);
                eax = m.r32(l(0x1C)) & 0xFF;
                lo16!(ecx, m.r16(edi + eax * 2 + 0x5CA));
                if m.r16(edi + 0x10) == ecx as u16 {
                    lo16!(edx, m.r16(edi + 0x12));
                    if edx as u16 == m.r16(edi + eax * 2 + 0x5CE) {
                        pc = L::F8DE;
                        continue;
                    }
                }
                lo16!(eax, m.r16(edi + eax * 2 + 0x5CE));
                let c = ecx;
                ecx = edi;
                call!(RESET_FLOW, edi, c, eax, 0);
                lo16!(ecx, m.r16(edi + 0xE));
                lo16!(edx, m.r16(edi + 0xC));
                let c = ecx;
                ecx = edi;
                call!(ADD_FLOW_SEED, edi, edx, c, 0);
                ecx = edi;
                call!(UPDATE_ENTRY_MARKERS, edi, 1);
                pc = L::F8DE;
            }
            L::F803 => {
                let b = ebx as u16;
                let yes = if b == m.r16(a3) {
                    true
                } else if b > m.r16(a2) {
                    false
                } else {
                    eax = m.r32(esi);
                    vcall!(0x68,);
                    eax as u8 == 4
                };
                lo8!(eax, u8::from(yes));
                edx = m.r32(esi);
                let v = eax;
                vcall!(0x58, v);
                lo8!(ebx, eax as u8);
                eax = m.r32(esi);
                vcall!(0x68,);
                if eax as u8 != 4 {
                    pc = L::F8DE;
                    continue;
                }
                if ebx as u8 != 0 {
                    pc = L::F915;
                    continue;
                }
                if m.r8(ebp + 0x6F) != 0xFF {
                    pc = L::F8DE;
                    continue;
                }
                lo16!(ecx, m.r16(ebp + 0x42));
                if ecx as u16 >= m.r16(ebp + 0x44) || m.r16(ebp + 0x3A) != 0 || m.r16(ebp + 0x3C) != 0 {
                    pc = L::F8DE;
                    continue;
                }
                edx = m.r32(l(0x30));
                eax = m.r32(l(0x2C));
                ecx = ebp;
                call!(FLOW_DISTANCE, ebp, eax, edx);
                if eax as u16 >= 0x14 {
                    pc = L::F8DE;
                    continue;
                }
                lo8!(ecx, m.r8(esi + 0x90));
                m.w8(l(0x1C), ecx as u8);
                eax = m.r32(l(0x1C)) & 0xFF;
                lo16!(ecx, m.r16(ebp + eax * 2 + 0x5CA));
                if m.r16(ebp + 0x10) == ecx as u16 {
                    lo16!(edx, m.r16(ebp + 0x12));
                    if edx as u16 == m.r16(ebp + eax * 2 + 0x5CE) {
                        pc = L::F8DE;
                        continue;
                    }
                }
                lo16!(eax, m.r16(ebp + eax * 2 + 0x5CE));
                let c = ecx;
                ecx = ebp;
                call!(RESET_FLOW, ebp, c, eax, 0);
                lo16!(ecx, m.r16(ebp + 0xE));
                lo16!(edx, m.r16(ebp + 0xC));
                let c = ecx;
                ecx = ebp;
                call!(ADD_FLOW_SEED, ebp, edx, c, 0);
                ecx = ebp;
                call!(UPDATE_ENTRY_MARKERS, ebp, 1);
                pc = L::F8DE;
            }
            L::F8DE => {
                if ebx as u8 != 0 {
                    pc = L::F915;
                    continue;
                }
                let args = [m.r32(a0), m.r32(l(0x2C)), m.r32(l(0x30)), m.r32(a3), m.r32(a1), m.r32(a5), m.r32(a2)];
                eax = m.r32(a0);
                ecx = esi;
                edx = m.r32(l(0x2C));
                call!(PICK_NEXT_TILE, esi, args[0], args[1], args[2], args[3], args[4], args[5], args[6]);
                m.w8(a5, eax as u8);
                pc = L::F6F4;
            }
            L::F915 => {
                m.w8(a5, u8::from(ebx as u8 == 2));
                pc = L::F6E2;
            }
            L::F922 => {
                ecx = m.r32(a0);
                eax = u32::from(m.r16(esi + 0x6E));
                let c = ecx;
                ecx = esi;
                call!(BEGIN_STEP, esi, edx, eax, c);
                pc = L::F700;
            }
            L::FBB7 => {
                if ebx as u8 != 0 {
                    pc = L::FBEF;
                    continue;
                }
                lo8!(eax, m.r8(esi + 0x59));
                if eax as u8 != 0 {
                    pc = L::FC01;
                    continue;
                }
                lo8!(eax, m.r8(esi + 0x7B) & 1);
                if eax as u8 != 0 && ebp != 0 {
                    lo8!(ecx, m.r8(ebp + 0x79));
                    if ecx as u8 != 0 {
                        pc = L::FC01;
                        continue;
                    }
                }
                lo16!(ecx, m.r16(a2));
                if ecx as u16 > m.r16(a3) && eax as u8 == 0 {
                    pc = L::FC01;
                    continue;
                }
                edx = m.r32(esi);
                vcall!(0x58, 1);
                lo8!(ebx, eax as u8);
                pc = L::FBEF;
            }
            L::FBEF => {
                if ebx as u8 == 1 || ebx as u8 == 4 {
                    pc = L::FCB3;
                    continue;
                }
                pc = L::FC01;
            }
            L::FC01 => {
                eax = 0;
                m.w16(esi + 0x3A, 1);
                lo8!(eax, m.r8(esi + 0x34));
                m.w16(esi + 0x38, 0);
                eax = eax.wrapping_sub(2);
                if eax > 0xCC {
                    pc = L::FCF3;
                    continue;
                }
                let st = eax + 2;
                ecx = 0;
                let set = m.r32(a0);
                let sa = m.r32(l(0x14));
                match st {
                    2 | 5 | 0xA4 | 0xB2 | 0xCC => {
                        call!(SELECT_ANIM, sa, set, 5);
                        pc = L::FCEF;
                    }
                    0xB | 0xD | 0xF | 0x13 | 0x14 | 0xAC | 0xCE => {
                        ebx = set;
                        edi = sa;
                        ecx = edi;
                        call!(FIND_ANIMATION_INDEX, edi, set, 0xD);
                        if eax as u8 != 0xFF {
                            call!(SELECT_ANIM, edi, set, 0xD);
                            pc = L::FCEF;
                        } else {
                            call!(FIND_ANIMATION_INDEX, edi, set, 0x12);
                            let key = if eax as u8 != 0xFF { 0x92 } else { 0xEC };
                            call!(SELECT_ANIM, edi, set, key);
                            let r = eax;
                            call!(SET_DEFAULT_ANIM, edi, 0xCC);
                            m.w8(l(0x10), r as u8);
                            pc = L::FCF3;
                        }
                    }
                    0x10 | 0xAE => {
                        call!(SELECT_ANIM, sa, set, 0xEE);
                        pc = L::FCEF;
                    }
                    _ => pc = L::FCF3,
                }
            }
            L::FCB3 => {
                edx = m.r32(a3);
                eax = m.r32(l(0x30));
                lo8!(ecx, u8::from(ebx as u8 == 4));
                let c = ecx;
                let (x, set) = (m.r32(l(0x2C)), m.r32(a0));
                call!(SELECT_STATE_ANIM, esi, set, x, eax, edx, c);
                pc = L::FCEF;
            }
            L::FCD0 => {
                ecx = m.r32(a3);
                edx = m.r32(l(0x30));
                lo8!(eax, u8::from(ebx as u8 == 4));
                let v = eax;
                let (x, set) = (m.r32(l(0x2C)), m.r32(a0));
                call!(SELECT_STATE_ANIM, esi, set, x, edx, ecx, v);
                pc = L::FCEF;
            }
            L::FCEF => {
                m.w8(l(0x10), eax as u8);
                pc = L::FCF3;
            }
            L::FCF3 => {
                lo8!(edx, m.r8(esi + 0x14));
                lo8!(eax, m.r8(esi + 0x3C));
                edi = m.r32(esi + 0xC);
                ebp = m.r32(esi + 0x10);
                if edx as u8 == eax as u8 {
                    lo8!(eax, m.r8(esi + 0x34));
                    ecx = m.r32(l(0x18));
                    let st = eax as u8;
                    if st == 0xB {
                        ecx = ecx.wrapping_add(ecx);
                    }
                    if st == 2 || st == 0x13 || st == 0xB {
                        lo16!(edx, m.r16(esi + 0x38));
                        eax = u32::from(m.r16(esi + 0x3A));
                        ebx = edx & 0xFFFF;
                        eax = eax.wrapping_sub(ebx);
                        if ecx > eax {
                            ecx = eax;
                        }
                        eax = m.r32(esi + 0x44);
                        edx = edx.wrapping_add(ecx);
                        m.w16(esi + 0x38, edx as u16);
                        edx = m.r32(esi + 0x40);
                        edx = edx.wrapping_mul(ecx);
                        eax = eax.wrapping_mul(ecx);
                        edi = edi.wrapping_add(edx);
                        ebp = ebp.wrapping_add(eax);
                    }
                }
                edx = sx16(m, esi + 0xE);
                ecx = edi >> 16;
                if ecx == edx {
                    ecx = sx16(m, esi + 0x12);
                    eax = ebp >> 16;
                    if eax == ecx {
                        lo8!(edx, m.r8(l(0x10)));
                        lo8!(eax, m.r8(esi + 0x15));
                        if edx as u8 == eax as u8 {
                            m.w32(esi + 0xC, edi);
                            m.w32(esi + 0x10, ebp);
                            return 1;
                        }
                    }
                }
                ebx = m.r32(a0);
                eax = l(0x34);
                ecx = esi;
                call!(REMOVE_TILES, esi, ebx, eax);
                lo8!(eax, m.r8(esi + 0x3C));
                lo8!(edx, m.r8(esi + 0x14));
                ecx = esi + 0x14;
                m.w32(esi + 0xC, edi);
                m.w32(esi + 0x10, ebp);
                if edx as u8 != eax as u8 {
                    lo8!(edx, m.r8(esi + 0x15));
                    if edx as u8 != m.r8(l(0x10)) {
                        lo8!(edx, m.r8(esi + 0x34));
                        if edx as u8 != 0x8E && edx as u8 != 0xEE {
                            lo8!(edx, m.r8(ebx + 0x14));
                            call!(ROTATE_TOWARD, crate::j07::CDECL, ecx, eax, edx);
                        }
                    }
                }
                lo8!(eax, m.r8(l(0x10)));
                ecx = l(0x34);
                m.w8(esi + 0x15, eax as u8);
                call!(INSERT_TILES, esi, ebx, ecx);
                return 1;
            }
        }
    }
}
