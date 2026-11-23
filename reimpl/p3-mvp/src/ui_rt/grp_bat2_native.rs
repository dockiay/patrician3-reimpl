//! Nativas del grupo `bat2` (14/11): funciones que no figuran en el catálogo y que `tools/lift.py` no puede elevar.
use super::core_table::CoreFn;
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;

type Regs = [u32; 8];

/// `0x004D6C50` (sin nombre, cdecl con un argumento): si el índice es menor que el contador `u16 [0x6DD736]`, calcula
/// `[[0x6CBB10] + 0x5A4] - [0x66BDC0]` y, si no es negativo, llama a `0x4D6C90` con `ecx = 0x6DD730` y `(índice, diferencia)`
/// (`message_answer_to_operation`).
fn answer_if_in_range(m: &mut dyn Mem, host: &mut dyn OpsHost, mut r: Regs) -> Regs {
    let index = m.r32(r[4].wrapping_add(4));
    if index < u32::from(m.r16(0x006D_D736)) {
        let base = m.r32(0x006C_BB10);
        let diff = m.r32(base.wrapping_add(0x5A4)).wrapping_sub(m.r32(0x0066_BDC0));
        if (diff as i32) >= 0 {
            r[0] = host.call(m, 0x004D_6C90, 0x006D_D730, &[index, diff]);
        }
    }
    r
}

/// `get_merchant_epithet_index` (0x004F9430, thiscall sin argumentos): misma lógica que `p3_core::ai::reputation` (paridad x86 allí);
/// aquí se lee el comerciante de la memoria del enlazador. El elevador no admite su `fcompp`.
fn merchant_epithet_index(m: &mut dyn Mem, _host: &mut dyn OpsHost, mut r: Regs) -> Regs {
    let this = r[1];
    let income = m.r32(this.wrapping_add(0x46C)) as i32;
    let mut renown = i32::from(m.r8(this.wrapping_add(0x15)));
    if income >= 0x9C40 {
        let high = ((i64::from(income) * 0x68DB_8BAD_i64) >> 32) as i32 >> 14;
        renown += high.wrapping_add(((high as u32) >> 31) as i32);
    }
    let guild = i32::from(m.r8(this.wrapping_add(0x1B)));
    let al = if f64::from(guild) / 120.0 > f64::from(renown) / 200.0 {
        match guild {
            ..=4 => 0,
            5..=9 => 8,
            10..=19 => 9,
            20..=34 => 10,
            35..=64 => 11,
            65..=89 => 12,
            90..=119 => 13,
            _ => 14,
        }
    } else {
        match renown {
            ..=9 => 0,
            10..=19 => 1,
            20..=29 => 2,
            30..=49 => 3,
            50..=99 => 4,
            100..=149 => 5,
            150..=199 => 6,
            _ => 7,
        }
    };
    r[0] = (r[0] & !0xFF) | al;
    r
}

pub static GRP_BAT2_NATIVE: [(u32, CoreFn); 2] = [(0x004D_6C50, answer_if_in_range), (0x004F_9430, merchant_epithet_index)];
