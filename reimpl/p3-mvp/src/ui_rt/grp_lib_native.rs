//! Nativas del grupo `lib` (CRT/MFC; hilo «Enlazar las funciones que faltan», 14/11): rutinas que `tools/lift.py` no puede elevar
//! (x87 con `frndint`/`fldz`, `rcr`/`shrd`/`sahf`, cabecera SEH del CRT) o que el enlazador resuelve mejor con el sistema operativo.
//! Todas son `CoreFn`: leen los argumentos de la pila de entrada (`regs[4]` apunta a la dirección de retorno) y devuelven los registros;
//! las que devuelven un `double` en ST0 lo dejan en `FPU_EXIT` para que el llamante elevado con `--fpu-exit` lo recoja con `fret`.
//!
//! Importante: `Runtime::call` solo devuelve EAX y no ve la pila x87 del llamante, así que `__ftol` (`0x639C28`, ST0 → EDX:EAX) y
//! `__allshr` (`0x63C610`, EDX:EAX y CL) solo funcionan en línea (`Cpu::crt_call` y `seh_pending::native`, que usan los elevados).
//! Aquí se registran las que sí tienen la convención de pila: `__alldiv`, `__aulldiv`, `__aullrem`, `sqrt`, `acos`, `asin`, `cos`,
//! `sin`, `floor`, `__frnd`, `_controlfp`, `toupper`, `AfxLoadString`/`LoadStringA` (texto de los recursos del EXE) y `DeleteFile`.
use super::core_table::CoreFn;
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;
use p3_core::x86rt::{Cpu, FPU_EXIT};
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Mutex;

type Regs = [u32; 8];

fn arg(m: &mut dyn Mem, r: &Regs, i: u32) -> u32 {
    m.r32(r[4].wrapping_add(4 + 4 * i))
}
fn dbl(m: &mut dyn Mem, r: &Regs, i: u32) -> f64 {
    f64::from_bits(u64::from(arg(m, r, i)) | (u64::from(arg(m, r, i + 1)) << 32))
}
/// Devuelve `x` en ST0 (el llamante elevado con `--fpu-exit` lo recoge con `fret`).
fn st0(r: Regs, x: f64) -> Regs {
    FPU_EXIT.with(|c| c.set((1, x.to_bits())));
    r
}
fn eax(mut r: Regs, v: u32) -> Regs {
    r[0] = v;
    r
}
fn edx_eax(mut r: Regs, v: u64) -> Regs {
    r[0] = v as u32;
    r[2] = (v >> 32) as u32;
    r
}

thread_local! {
    /// Palabra de control x87 que ven `_controlfp` y `__frnd` (por defecto 0x27F: doble precisión, redondeo al más cercano).
    static CW: Cell<u32> = const { Cell::new(0x27f) };
}

fn sqrt(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    st0(r, dbl(m, &r, 0).sqrt())
}
fn acos(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    st0(r, dbl(m, &r, 0).acos())
}
fn asin(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    st0(r, dbl(m, &r, 0).asin())
}
fn cos(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    st0(r, dbl(m, &r, 0).cos())
}
fn sin(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    st0(r, dbl(m, &r, 0).sin())
}
/// `floor` (`0x63C70C`): fija el redondeo hacia abajo, `frndint` y restaura; en NaN/infinito devuelve el propio valor.
fn floor(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    st0(r, dbl(m, &r, 0).floor())
}
/// `__frnd` (`0x641B70`): `frndint` con el redondeo de la palabra de control vigente.
fn frnd(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    let x = dbl(m, &r, 0);
    let y = match (CW.with(Cell::get) >> 10) & 3 {
        0 => x.round_ties_even(),
        1 => x.floor(),
        2 => x.ceil(),
        _ => x.trunc(),
    };
    st0(r, y)
}
/// `_controlfp`-like (`0x641CE3`, `(nuevo, máscara)`): devuelve la palabra anterior con signo extendido y aplica `(nuevo & máscara) | (vieja & !máscara)`.
fn control87(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    let (new, mask) = (arg(m, &r, 0), arg(m, &r, 1));
    let old = CW.with(Cell::get);
    CW.with(|c| c.set((new & mask) | (old & !mask)));
    eax(r, i32::from(old as u16 as i16) as u32)
}
/// Controladores de excepción matemática del CRT (`0x6414B8 (código, x, cw)` y `0x64150C (código, código, x, y, cw)`): solo se llaman
/// con NaN/infinito; devuelven el primer operando, que es lo que dejaría el original sin `_matherr` instalada.
fn math_exc1(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    st0(r, dbl(m, &r, 1))
}
fn math_exc2(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    st0(r, dbl(m, &r, 2))
}
/// `toupper` (`0x63C7DB`, cdecl) en la configuración regional "C" del juego.
fn toupper(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    let c = arg(m, &r, 0);
    eax(r, if (0x61..=0x7a).contains(&c) { c - 0x20 } else { c })
}
/// `__alldiv` (`0x63A6E0`, `ret 0x10`): `seh_pending::native` conserva también el ECX de la rama del divisor grande.
fn alldiv(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    let mut c = Cpu { r, ..Cpu::default() };
    let divisor = (u64::from(arg(m, &r, 2)) | (u64::from(arg(m, &r, 3)) << 32)) as i64;
    if divisor == 0 {
        return r; // el original lanza #DE; aquí no se aborta el cliente
    }
    p3_core::economy::seh_pending::native(&mut c, m, 0x63a6e0);
    c.r
}
/// `__allshr` (`0x63C610`): los operandos van en EDX:EAX y CL (solo válido si el llamante los pone en `regs`).
fn allshr(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    let mut c = Cpu { r, ..Cpu::default() };
    p3_core::economy::seh_pending::native(&mut c, m, 0x63c610);
    c.r
}
fn u64_args(m: &mut dyn Mem, r: &Regs) -> (u64, u64) {
    (u64::from(arg(m, r, 0)) | (u64::from(arg(m, r, 1)) << 32), u64::from(arg(m, r, 2)) | (u64::from(arg(m, r, 3)) << 32))
}
/// `__aulldiv` (`0x6461A0`, `ret 0x10`): división de 64 bits sin signo; el cociente va en EDX:EAX.
fn aulldiv(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    let (a, b) = u64_args(m, &r);
    edx_eax(r, a.checked_div(b).unwrap_or(0))
}
/// `__aullrem` (`0x646210`, `ret 0x10`): resto de 64 bits sin signo en EDX:EAX.
fn aullrem(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    let (a, b) = u64_args(m, &r);
    edx_eax(r, a.checked_rem(b).unwrap_or(0))
}

fn cstr(m: &mut dyn Mem, p: u32) -> Vec<u8> {
    (0..4096u32).map(|i| m.r8(p.wrapping_add(i))).take_while(|&b| b != 0).collect()
}
/// `CFile::Remove` (`0x653F49`, `ret 4`; ruta): borra el fichero del disco. Devuelve 1 si lo borró y 0 si no (el original, al fallar,
/// lanza `CFileException` por `0x653761`, que el llamante recoge: aquí no hay excepciones y el llamante ve el 0).
fn delete_file(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    let p = arg(m, &r, 0);
    let path = cstr(m, p);
    let ok = std::str::from_utf8(&path).is_ok_and(|p| crate::hostfs::current().remove_file(std::path::Path::new(&p.replace('\\', "/"))).is_ok());
    eax(r, u32::from(ok))
}

/// Textos del recurso `RT_STRING` del EXE (identificador → UTF-16), cargados por `Runtime::new` con [`register_exe`].
static STRINGS: Mutex<Option<HashMap<u32, Vec<u16>>>> = Mutex::new(None);

/// Lee los textos `RT_STRING` del PE (los de los tooltips `20000..`, el menú de opciones, los mensajes de MFC...).
pub fn register_exe(exe: &[u8]) {
    *STRINGS.lock().unwrap() = Some(parse_strings(exe).unwrap_or_default());
}

fn parse_strings(exe: &[u8]) -> Option<HashMap<u32, Vec<u16>>> {
    let u16_at = |o: usize| exe.get(o..o + 2).map(|b| u32::from(u16::from_le_bytes([b[0], b[1]])));
    let u32_at = |o: usize| exe.get(o..o + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
    let pe = u32_at(0x3c)? as usize;
    let (nsec, opt_size) = (u16_at(pe + 6)? as usize, u16_at(pe + 20)? as usize);
    let opt = pe + 24;
    let rsrc_rva = u32_at(opt + 96 + 16)?;
    let sections: Vec<(u32, u32, u32)> = (0..nsec)
        .map(|s| {
            let h = opt + opt_size + 40 * s;
            Some((u32_at(h + 12)?, u32_at(h + 16)?, u32_at(h + 20)?))
        })
        .collect::<Option<_>>()?;
    let off = |rva: u32| sections.iter().find(|&&(va, size, _)| rva >= va && rva < va + size.max(1)).map(|&(va, _, raw)| (rva - va + raw) as usize);
    let root = off(rsrc_rva)?;
    // Entradas de un directorio: (nombre o id, desplazamiento respecto a la raíz, es subdirectorio).
    let dir = |at: usize| -> Option<Vec<(u32, usize, bool)>> {
        let n = (u16_at(at + 12)? + u16_at(at + 14)?) as usize;
        (0..n).map(|i| { let e = at + 16 + 8 * i; let (id, o) = (u32_at(e)?, u32_at(e + 4)?); Some((id, (o & 0x7fff_ffff) as usize, o >> 31 == 1)) }).collect()
    };
    let mut out = HashMap::new();
    for (ty, o, sub) in dir(root)? {
        if ty != 6 || !sub {
            continue;
        }
        for (block, o2, sub2) in dir(root + o)? {
            let Some(&(_, o3, sub3)) = dir(root + o2).filter(|_| sub2)?.first() else { continue };
            let data = if sub3 { *dir(root + o3)?.first().map(|(_, o4, _)| o4)? } else { o3 };
            let (rva, size) = (u32_at(root + data)?, u32_at(root + data + 4)? as usize);
            let at = off(rva)?;
            let mut p = at;
            for i in 0..16u32 {
                if p + 2 > at + size {
                    break;
                }
                let n = u16_at(p)? as usize;
                p += 2;
                if n > 0 {
                    let s: Vec<u16> = (0..n).map(|k| u16_at(p + 2 * k).unwrap_or(0) as u16).collect();
                    out.insert((block - 1) * 16 + i, s);
                }
                p += 2 * n;
            }
        }
    }
    Some(out)
}

/// Unicode → página 1252 (lo que devuelve `LoadStringA` en un Windows occidental).
fn to_ansi(c: u16) -> u8 {
    const HIGH: [(u16, u8); 27] = [
        (0x20AC, 0x80), (0x201A, 0x82), (0x0192, 0x83), (0x201E, 0x84), (0x2026, 0x85), (0x2020, 0x86), (0x2021, 0x87), (0x02C6, 0x88),
        (0x2030, 0x89), (0x0160, 0x8A), (0x2039, 0x8B), (0x0152, 0x8C), (0x017D, 0x8E), (0x2018, 0x91), (0x2019, 0x92), (0x201C, 0x93),
        (0x201D, 0x94), (0x2022, 0x95), (0x2013, 0x96), (0x2014, 0x97), (0x02DC, 0x98), (0x2122, 0x99), (0x0161, 0x9A), (0x203A, 0x9B),
        (0x0153, 0x9C), (0x017E, 0x9E), (0x0178, 0x9F),
    ];
    if c < 0x100 && !(0x80..0xa0).contains(&c) {
        c as u8
    } else {
        HIGH.iter().find(|&&(u, _)| u == c).map_or(b'?', |&(_, a)| a)
    }
}

/// Texto del recurso `id` como bytes ANSI (sin terminador), `None` si no existe.
pub fn resource_string(id: u32) -> Option<Vec<u8>> {
    STRINGS.lock().unwrap().as_ref()?.get(&id).map(|s| s.iter().map(|&c| to_ansi(c)).collect())
}

/// `AfxLoadString`-ayuda (`0x654CDC`, `ret 0xC`; id, búfer, tamaño): copia el texto del recurso con terminador y devuelve cuántos
/// caracteres copió (0 si no existe, y entonces deja el búfer vacío, como el original).
fn load_string(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    let (id, buf, size) = (arg(m, &r, 0), arg(m, &r, 1), arg(m, &r, 2));
    if size == 0 {
        return eax(r, 0);
    }
    let text = resource_string(id).unwrap_or_default();
    let n = text.len().min(size as usize - 1);
    for (i, &b) in text[..n].iter().enumerate() {
        m.w8(buf + i as u32, b);
    }
    m.w8(buf + n as u32, 0);
    eax(r, n as u32)
}

pub static GRP_LIB_NATIVE: [(u32, CoreFn); 18] = [
    (0x63a6e0, alldiv),
    (0x63aae4, sqrt),
    (0x63b2c4, acos),
    (0x63c264, cos),
    (0x63c314, sin),
    (0x63c3c4, asin),
    (0x63c610, allshr),
    (0x63c70c, floor),
    (0x63c7db, toupper),
    (0x6414b8, math_exc1),
    (0x64150c, math_exc2),
    (0x641b70, frnd),
    (0x641ce3, control87),
    (0x6461a0, aulldiv),
    (0x646210, aullrem),
    (0x653f49, delete_file),
    (0x654cdc, load_string),
    (0x639c28, ftol_stub),
];

/// `__ftol` (`0x639C28`): el valor viaja en ST0 del llamante, que `Runtime::call` no ve. Los llamantes elevados la hacen en línea
/// (`Cpu::crt_call`); si algún porte antiguo llega aquí por `host.call` se toma el `double` que haya en `[esp+4]` (nunca es el caso de
/// ST0, pero deja el valor truncado en EDX:EAX en vez de perder la llamada).
fn ftol_stub(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    let v = dbl(m, &r, 0);
    edx_eax(r, if v.is_nan() || v >= 9.223372036854775807e18 || v < -9.223372036854775808e18 { i64::MIN as u64 } else { v.trunc() as i64 as u64 })
}
