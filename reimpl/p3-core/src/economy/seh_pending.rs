//! Remaining SEH routines. The fixed x86 stack, locals and FS chain follow FRAME_SEH.md.
//! Calls outside each body use OpsHost; the native CRT below also returns EDX/x87.
use crate::flatmem::Mem;
use crate::x86rt::{Cpu, EAX, ECX, EDX, ESP};

pub use crate::lifted::seh_pending::{script_blob_pool_load_for_game_mode, trade_center_handle_convoy_contract};

/// CRT subset used by these two functions. No allocator, resource reader or game
/// function is silently substituted here; those remain explicit OpsHost calls.
pub fn native(c: &mut Cpu, m: &mut dyn Mem, target: u32) {
    let stack = c.r[ESP];
    let arg = |m: &mut dyn Mem, i: u32| m.r32(stack.wrapping_add(4 + i * 4));
    match target {
        0x639f20 => {
            let (dst, value, n) = (arg(m, 0), arg(m, 1) as u8, arg(m, 2));
            for i in 0..n { m.w8(dst.wrapping_add(i), value); }
            c.r[EAX] = dst;
            c.r[ECX] = if n >= 4 { 0 } else { dst };
            c.r[EDX] = 0;
        }
        0x63a6a0 | 0x63a6e0 => {
            let (al, ah, bl, bh) = (arg(m, 0), arg(m, 1), arg(m, 2), arg(m, 3));
            let a = u64::from(al) | (u64::from(ah) << 32);
            let b = u64::from(bl) | (u64::from(bh) << 32);
            let value = if target == 0x63a6a0 {
                c.r[ECX] = bl;
                a.wrapping_mul(b)
            } else {
                let (aa, bb) = ((a as i64).unsigned_abs(), (b as i64).unsigned_abs());
                assert_ne!(bb, 0, "original CRT division by zero");
                // Preserve the scratch ECX of __alldiv, including its large-divisor path.
                if bb >> 32 == 0 {
                    c.r[ECX] = bb as u32;
                } else {
                    let mut divisor = bb;
                    let mut dividend = aa;
                    while divisor >> 32 != 0 { divisor >>= 1; dividend >>= 1; }
                    let estimate = (dividend / divisor) as u32;
                    c.r[ECX] = ((bb >> 32).wrapping_mul(u64::from(estimate))) as u32;
                }
                (a as i64).wrapping_div(b as i64) as u64
            };
            c.r[EAX] = value as u32;
            c.r[EDX] = (value >> 32) as u32;
        }
        0x63c610 => {
            let shift = c.r[ECX] & 0xff;
            let value = (u64::from(c.r[EAX]) | (u64::from(c.r[EDX]) << 32)) as i64;
            let result = if shift >= 64 { value >> 63 } else { value >> shift } as u64;
            if (32..64).contains(&shift) { c.r[ECX] &= !0xe0; }
            c.r[EAX] = result as u32;
            c.r[EDX] = (result >> 32) as u32;
        }
        0x639c28 => {
            let value = crate::numeric::ftol(c.fpu.pop().expect("original __ftol needs ST0")) as u64;
            c.r[EAX] = value as u32;
            c.r[EDX] = (value >> 32) as u32;
        }
        _ => unreachable!("CRT address {target:#x}"),
    }
}
