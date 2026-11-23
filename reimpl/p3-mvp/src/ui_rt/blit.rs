//! Four RGBA pixels at a time, retaining the SGL integer/SSE blend rules.
//! x86-64 guarantees SSE2; WebAssembly built with `simd128` uses the same algorithm
//! (WEB-6); other targets use the existing scalar rasterizer.
pub(super) fn row(dst: &mut [u8], src: &[u8], routine: u8, color: u32, key: u32, keyed: bool) {
    assert_eq!(dst.len(), src.len());
    assert_eq!(src.len() % 4, 0);
    #[cfg(target_arch = "x86_64")]
    if routine <= 10 && src.len() >= 16 {
        // SAFETY: SSE2 is part of x86-64. Equal complete-pixel slices were
        // checked above; vector loads/stores below stay within both slices.
        unsafe { return vector_row(dst, src, routine, color, key, keyed); }
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    if routine <= 10 && src.len() >= 16 {
        return wasm_row(dst, src, routine, color, key, keyed);
    }
    scalar_row(dst, src, routine, color, key, keyed);
}

/// Operación por operación la misma que `blend16`/`vector_row` de SSE2 (carriles de 16 bits), con `simd128`.
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
#[inline(always)]
fn wasm_blend16(s: std::arch::wasm32::v128, d: std::arch::wasm32::v128, c: std::arch::wasm32::v128, routine: u8) -> std::arch::wasm32::v128 {
    use std::arch::wasm32::*;
    match routine {
        2..=4 => {
            let m = u16x8_shr(i16x8_mul(s, c), 8);
            let a = i16x8_shuffle::<3, 3, 3, 3, 7, 7, 7, 7>(m, m);
            let t = i16x8_shr(i16x8_mul(i16x8_sub(m, d), a), 8);
            v128_and(i16x8_add(t, d), i16x8_splat(255))
        }
        5 => i16x8_add(u16x8_shr(i16x8_mul(s, c), 8), d),
        6 => i16x8_add(s, d),
        7 => u16x8_shr(i16x8_mul(u16x8_shr(i16x8_mul(s, c), 8), d), 8),
        8 => u16x8_shr(i16x8_mul(s, d), 8),
        9 | 10 => u16x8_shr(i16x8_mul(s, d), 7),
        _ => s,
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
fn wasm_row(dst: &mut [u8], src: &[u8], routine: u8, color: u32, key: u32, keyed: bool) {
    use std::arch::wasm32::*;
    let zero = i32x4_splat(0);
    let alpha = i32x4_splat(0xff00_0000u32 as i32);
    let marker = i32x4_splat(i32::MIN);
    let rgba_key = (key & 0xff) << 16 | (key & 0xff00) | ((key >> 16) & 0xff);
    let key4 = i32x4_splat(rgba_key as i32);
    let [a, r, g, b] = color.to_be_bytes();
    let c = i16x8(r as i16, g as i16, b as i16, a as i16, r as i16, g as i16, b as i16, a as i16);
    let mut offset = 0;
    while offset + 16 <= src.len() {
        // SAFETY: el intervalo `offset..offset + 16` está dentro de los dos cortes (misma longitud, comprobada en `row`);
        // las cargas y el guardado no exigen alineación.
        let (s, d) = unsafe { (v128_load(src.as_ptr().add(offset).cast()), v128_load(dst.as_ptr().add(offset).cast())) };
        let value = if routine <= 1 {
            s
        } else {
            let lo = wasm_blend16(u16x8_extend_low_u8x16(s), u16x8_extend_low_u8x16(d), c, routine);
            let hi = wasm_blend16(u16x8_extend_high_u8x16(s), u16x8_extend_high_u8x16(d), c, routine);
            u8x16_narrow_i16x8(lo, hi)
        };
        let mut keep = zero;
        if keyed || routine == 1 {
            keep = i32x4_eq(s, key4);
        }
        if keyed {
            keep = v128_or(keep, i32x4_eq(s, marker));
        }
        let result = v128_or(v128_and(keep, d), v128_andnot(value, keep));
        // SAFETY: como en la carga.
        unsafe { v128_store(dst.as_mut_ptr().add(offset).cast(), v128_or(result, alpha)) };
        offset += 16;
    }
    scalar_row(&mut dst[offset..], &src[offset..], routine, color, key, keyed);
}

/// Comprueba que la ruta vectorial de esta plataforma da lo mismo que la escalar (modos, claves, desbordes con signo y colas
/// sin alinear). Es el cuerpo de la prueba unitaria, expuesto para ejecutarlo también dentro del `.wasm` (allí no hay `cargo test`).
/// Devuelve el número de casos comprobados o el primero que difiere.
pub fn self_check() -> Result<u32, String> {
    let mut cases = 0;
    for width in 0..=35 {
        for routine in 0..=10 {
            for color in [0,0xffff_ffff,0x80ff_307f,0xff00_ff01,0x01ff_ffff] {
                for key in [0,0x123456] {
                    for keyed in [false,true] {
                        let mut source:Vec<u8>=(0..width*4+7).map(|i| (i*73+i/7) as u8).collect();
                        // Cover key with zero/nonzero alpha, the bridge's
                        // legacy marker, and maximum/half channel values.
                        let special=[[0x12,0x34,0x56,0],[0,0,0,128],[0x12,0x34,0x56,255],[255,255,255,255],[255,0,128,128],[0,0,0,0]];
                        for (i,p) in special.iter().enumerate().take(width) {source[3+4*i..7+4*i].copy_from_slice(p);}
                        let original:Vec<u8>=(0..width*4+9).map(|i| (i*91+i/3+7) as u8).collect();
                        let mut expected=original.clone();let mut actual=original.clone();
                        scalar_row(&mut expected[5..5+width*4],&source[3..3+width*4],routine,color,key,keyed);
                        row(&mut actual[5..5+width*4],&source[3..3+width*4],routine,color,key,keyed);
                        if actual!=expected { return Err(format!("width={width} routine={routine} color={color:08x} key={key:x} keyed={keyed}")); }
                        cases += 1;
                    }
                }
            }
        }
    }
    Ok(cases)
}

fn scalar_row(dst: &mut [u8], src: &[u8], routine: u8, color: u32, key: u32, keyed: bool) {
    let argb = |p: &[u8]| u32::from_le_bytes([p[2], p[1], p[0], p[3]]);
    for (s,d) in src.chunks_exact(4).zip(dst.chunks_exact_mut(4)) {
        let pixel=argb(s);
        if !(keyed && (pixel==key & 0xff_ffff || pixel==0x8000_0000)) {
            if let Some(v)=p3_render::sgl::blend_pixel(routine,pixel,argb(d),color,key) {
                let [b,g,r,_]=v.to_le_bytes();
                d[..3].copy_from_slice(&[r,g,b]);
            }
        }
        d[3]=255;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn blend16(s: std::arch::x86_64::__m128i, d: std::arch::x86_64::__m128i, c: std::arch::x86_64::__m128i, routine:u8) -> std::arch::x86_64::__m128i {
    use std::arch::x86_64::*;
    match routine {
        2..=4 => {
            let m=_mm_srli_epi16::<8>(_mm_mullo_epi16(s,c));
            let a=_mm_shufflehi_epi16::<255>(_mm_shufflelo_epi16::<255>(m));
            let t=_mm_srai_epi16::<8>(_mm_mullo_epi16(_mm_sub_epi16(m,d),a));
            _mm_and_si128(_mm_add_epi16(t,d),_mm_set1_epi16(255))
        }
        5 => _mm_add_epi16(_mm_srli_epi16::<8>(_mm_mullo_epi16(s,c)),d),
        6 => _mm_add_epi16(s,d),
        7 => _mm_srli_epi16::<8>(_mm_mullo_epi16(_mm_srli_epi16::<8>(_mm_mullo_epi16(s,c)),d)),
        8 => _mm_srli_epi16::<8>(_mm_mullo_epi16(s,d)),
        9 | 10 => _mm_srli_epi16::<7>(_mm_mullo_epi16(s,d)),
        _ => s,
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
unsafe fn vector_row(dst: &mut [u8], src: &[u8], routine:u8, color:u32, key:u32, keyed:bool) {
    use std::arch::x86_64::*;
    let zero=_mm_setzero_si128();
    let alpha=_mm_set1_epi32(0xff00_0000u32 as i32);
    let marker=_mm_set1_epi32(i32::MIN);
    let rgba_key=(key & 0xff)<<16 | (key & 0xff00) | ((key>>16) & 0xff);
    let key4=_mm_set1_epi32(rgba_key as i32);
    let [a,r,g,b]=color.to_be_bytes();
    let c=_mm_setr_epi16(r as i16,g as i16,b as i16,a as i16,r as i16,g as i16,b as i16,a as i16);
    let mut offset=0;
    while offset+16 <= src.len() {
        // Both unaligned operations cover exactly the four complete pixels
        // selected by this checked interval. Rust keeps src/dst non-aliasing.
        let s=_mm_loadu_si128(src.as_ptr().add(offset).cast());
        let d=_mm_loadu_si128(dst.as_ptr().add(offset).cast());
        let value=if routine<=1 {s} else {
            let lo=blend16(_mm_unpacklo_epi8(s,zero),_mm_unpacklo_epi8(d,zero),c,routine);
            let hi=blend16(_mm_unpackhi_epi8(s,zero),_mm_unpackhi_epi8(d,zero),c,routine);
            _mm_packus_epi16(lo,hi)
        };
        let mut keep=zero;
        if keyed || routine==1 {keep=_mm_cmpeq_epi32(s,key4);}
        if keyed {keep=_mm_or_si128(keep,_mm_cmpeq_epi32(s,marker));}
        let result=_mm_or_si128(_mm_and_si128(keep,d),_mm_andnot_si128(keep,value));
        _mm_storeu_si128(dst.as_mut_ptr().add(offset).cast(),_mm_or_si128(result,alpha));
        offset+=16;
    }
    scalar_row(&mut dst[offset..],&src[offset..],routine,color,key,keyed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_rows_match_sgl_for_modes_keys_signed_overflow_and_unaligned_tails() {
        assert_eq!(self_check(), Ok(36 * 11 * 5 * 2 * 2));
    }
}
