//! `flush_pending_message_notices` (`0x004D7D50`, stdcall, un argumento: ciudad; SEH): recorre la cadena de mensajes pendientes de la ciudad
//! (`0x6DD730`, registros de 16 bytes: `+1` banderas, `+4` tipo, `+6` siguiente, `+0xC` valor) y, por cada uno con la bandera `0x40`, avisa por
//! `0x443150` según las banderas de la ciudad (`+0x26`) o formatea una cadena (`0x64DF80`) en la cadena local (que ocupa la ranura del argumento,
//! `esp+4`) y la guarda en un vector de `CString`; después avisa a `0x4237D0`, entrega cada cadena a `0x42BB20` en orden inverso y libera el vector.
//! Marco modelado con el `esp` de entrada: `E-4` estado SEH, `E-8` manejador, `E-0xC` cadena SEH previa, `E-0x14` objeto de la ciudad, `E-0x10` vector
//! reservado (o puntero al temporal), `E+4` cadena local. El ECX residual empujado a `0x42BB20` se enmascara a 0.

use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;

pub fn flush_pending_message_notices(m: &mut dyn Mem, host: &mut dyn OpsHost, esp: u32, arg: u32) -> u32 {
    let e = esp;
    let at = |o: u32| e.wrapping_sub(o);
    let local = e.wrapping_add(4);
    let old_fs = m.r32(0);
    m.w32(at(4), 0xffff_ffff);
    m.w32(at(8), 0x66_2a21);
    m.w32(at(0xc), old_fs);
    m.w32(0, at(0xc));
    let leave = |m: &mut dyn Mem, ret: u32| -> u32 {
        let saved = m.r32(at(0xc));
        m.w32(0, saved);
        ret
    };
    if m.r32(0x6e51ac) == 0 {
        return leave(m, 0);
    }
    if (arg as u16) >= m.r16(0x6de4aa) {
        return leave(m, arg);
    }
    let obj = host.call(m, 0x5303c0, 0x6de4a0, &[arg & 0xffff]);
    let mut di = u32::from(m.r16(obj.wrapping_add(0xa)));
    let count = u32::from(m.r16(0x6dd736));
    m.w32(at(0x14), obj);
    let alloc = host.call(m, 0x64f7b9, 0, &[count.wrapping_mul(4).wrapping_add(4)]);
    m.w32(at(0x10), alloc);
    m.w32(at(4), 0);
    let mut arr = 0u32;
    if alloc != 0 {
        arr = alloc.wrapping_add(4);
        m.w32(alloc, count);
        host.call(m, 0x639e9c, 0, &[arr, 4, count, 0x4103c0, 0x64f253]);
    }
    let empty = m.r32(0x6c7ccc);
    m.w32(local, empty);
    let mut n = 0u32; // `ebp`: contador de cadenas guardadas
    m.w32(at(4), 1);
    let total = u32::from(m.r16(0x6dd736));
    if (di as u16) < (total as u16) {
        loop {
            let total = u32::from(m.r16(0x6dd736));
            let rec = if (di & 0xffff) > total {
                0
            } else {
                let base = m.r32(0x6dd730);
                base.wrapping_add((di & 0xffff) << 4)
            };
            let flags = m.r8(rec.wrapping_add(1));
            if flags & 0x40 == 0 {
                break;
            }
            let al = u32::from(m.r8(obj.wrapping_add(0x26)));
            let ty = u32::from(m.r8(rec.wrapping_add(4)));
            let kind = u32::from(m.r8(0x6c0198 + ty));
            if (al & 1 != 0 && kind == 1) || (al & 2 != 0 && kind == 2) || (al & 4 != 0 && kind == 3) {
                let t = m.r32(0x6cbdb8);
                host.call(m, 0x443150, t, &[0x1770, 0, 0, 0xffff_ffff, 0x40, 0, 1]);
            }
            let dl = u32::from(m.r8(obj.wrapping_add(0x26)));
            let ty = u32::from(m.r8(rec.wrapping_add(4)));
            let kind = u32::from(m.r8(0x6c0198 + ty));
            let value = |m: &mut dyn Mem| -> u32 {
                if (0x3c..=0x40).contains(&ty) {
                    m.r32(rec.wrapping_add(0xc))
                } else {
                    m.r32(0x6a52d0 + ty * 4)
                }
            };
            let fmt_addr = if dl & 8 != 0 && kind == 1 {
                Some(0x6a4570)
            } else if dl & 0x10 != 0 && kind == 2 {
                Some(0x6a4574)
            } else if dl & 0x20 != 0 && kind == 3 {
                Some(0x6a4578)
            } else {
                None
            };
            if let Some(fa) = fmt_addr {
                let v = value(m);
                let fmt = m.r32(fa);
                host.call(m, 0x64df80, 0, &[local, fmt, v]);
                let elem = arr.wrapping_add((n & 0xffff).wrapping_mul(4));
                n = n.wrapping_add(1);
                host.call(m, 0x64f340, elem, &[local]);
            }
            let f = m.r8(rec.wrapping_add(1)) & 0xbf;
            m.w8(rec.wrapping_add(1), f);
            di = u32::from(m.r16(rec.wrapping_add(6)));
            let total = u32::from(m.r16(0x6dd736));
            if (di as u16) >= (total as u16) {
                break;
            }
        }
        if (n as u16) != 0 {
            let t = m.r32(0x6cbb40);
            host.call(m, 0x4237d0, t, &[]);
        }
    }
    let cnt = n & 0xffff;
    for k in (0..cnt).rev() {
        let elem = arr.wrapping_add(k.wrapping_mul(4));
        m.w32(at(0x10), at(0x28));
        host.call(m, 0x64efc8, at(0x28), &[elem]);
        let t = m.r32(0x6cbb40);
        host.call(m, 0x42bb20, t, &[0]);
    }
    if arr != 0 {
        let c = m.r32(arr.wrapping_sub(4));
        host.call(m, 0x639da8, 0, &[arr, 4, c, 0x64f253]);
        host.call(m, 0x64f7e2, 0, &[arr.wrapping_sub(4)]);
    }
    m.w32(at(4), 0xffff_ffff);
    let r = host.call(m, 0x64f253, local, &[]);
    leave(m, r)
}
