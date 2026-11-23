//! `apply_difficulty_settings_and_pirates` (0x0054A480): aplica la dificultad y ajusta las cuadrillas piratas según `0x6CC3E8`.
//! Traducción 1:1 sobre la memoria del original ([`Mem`]).
use super::pirate_small;
use crate::flatmem::Mem;

pub trait DifficultyHost {
    /// `0x0052E1A0(0x6DE3D8, dificultad)`.
    fn apply_difficulty(&mut self, value: u8);
    /// `0x004F9550` (thiscall, ECX = comerciante).
    fn merchant_setup(&mut self, merchant: u32);
    /// `operator new` (0x0064F7B9).
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32;
    /// `rand` del CRT (0x00639C03).
    fn rand(&mut self) -> u32;
    /// `pirate_band_setup` (0x00513D80): (cuadrilla, a, b, c).
    fn band_setup(&mut self, band: u32, a: u32, b: u32, c: u32);
}

/// Devuelve `false` cuando el original no vuelve limpio: si el comerciante existe (`0x6DFC14 < n`) empuja dos
/// argumentos a un getter de uno (`ret 4`), deja un dword en la pila y salta a `0xFFFFFFFF` al retornar.
/// Todo el estado se escribe igualmente antes del fallo.
pub fn apply_difficulty_settings_and_pirates(m: &mut dyn Mem, host: &mut dyn DifficultyHost, flags: u32) -> bool {
    let g = m.r32(0x6CC3E8);
    if g == 0 || m.r8(flags) & 0xc != 0 {
        return true;
    }
    host.apply_difficulty(m.r8(g + 0x10));
    let idx = m.r32(0x6DFC14);
    let leak = idx < u32::from(m.r16(0x6DE4AA));
    if leak {
        let g = m.r32(0x6CC3E8);
        let mer = m.r32(0x6DE4A0 + 0x78).wrapping_add(idx.wrapping_mul(0x650));
        host.merchant_setup(mer);
    }
    let g = m.r32(0x6CC3E8);
    let d = u32::from(m.r8(g + 0x13));
    let limit = 2 * d + 1;
    for i in 0..5u32 {
        let band = m.r32(0x6DD7AC + 4 * i);
        if i >= limit {
            if band != 0 {
                m.w8(band + 0x15, 1);
            }
        } else if band == 0 {
            let block = host.alloc(m, 0x18);
            let nb = if block != 0 {
                pirate_small::band_init(m, block);
                block
            } else {
                0
            };
            m.w32(0x6DD7AC + 4 * i, nb);
            let r1 = host.rand();
            let g = m.r32(0x6CC3E8);
            let b = u32::from(m.r8(g + 0x13));
            let r2 = host.rand();
            host.band_setup(nb, r2, b, r1);
        } else {
            let g = m.r32(0x6CC3E8);
            let v = m.r8(g + 0x13) & 3;
            m.w8(band + 0x14, v);
        }
    }
    !leak
}
