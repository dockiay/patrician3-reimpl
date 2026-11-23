//! Nativas del grupo `lib2` (segunda pasada, CRT): las dos rutinas del CRT de las que dependen `_errno()`/`gmtime`/`localtime` y que el
//! enlazador no puede dejar a la importación `TlsGetValue`/`HeapAlloc` (ausentes: devuelven 0).
//! `CoreFn` como las de `grp_lib_native.rs`: leen los argumentos de la pila de entrada (`regs[4]` apunta a la dirección de retorno).
use super::core_table::CoreFn;
use p3_core::economy::ops_misc::OpsHost;
use p3_core::flatmem::Mem;

type Regs = [u32; 8];

fn arg(m: &mut dyn Mem, r: &Regs, i: u32) -> u32 {
    m.r32(r[4].wrapping_add(4 + 4 * i))
}

/// Ranura con el puntero a los datos del hilo (`_tiddata`). En el original es el índice TLS (`__tlsindex`, `0x006C8740`) que
/// `_getptd` pasa a `TlsGetValue`; el enlazador es monohilo y no tiene TLS, así que la ranura guarda el puntero directamente.
const PTD_SLOT: u32 = 0x006C_8740;

/// `_calloc_base` (`0x645243`, cdecl `(n, tamaño)`): `malloc` del enlazador (que ya devuelve el bloque a cero) de `n * tamaño` bytes.
/// El original usaba el montón de bloques pequeños del CRT o `HeapAlloc`, que aquí no existen.
fn calloc(m: &mut dyn Mem, host: &mut dyn OpsHost, mut r: Regs) -> Regs {
    let n = arg(m, &r, 0).wrapping_mul(arg(m, &r, 1));
    r[0] = host.call(m, 0x63cb41, 0, &[n]);
    r
}

/// `_getptd` (`0x63EDB3`): datos del hilo del CRT. El original llama a `TlsGetValue` y, si está vacío, reserva con `_calloc_base`
/// 0x74 bytes, los inicializa con `_initptd` (`0x63EDA0`: `+0x14` = 1, `+0x50` = tabla de excepciones `0x6CAFC0`), `TlsSetValue` y
/// pone el identificador del hilo en `+0`, `-1` en `+4`. Aquí se crean una vez y se reutilizan, de modo que `errno` (`+8`) y la
/// estructura `tm` de `gmtime` (`+0x40`) persisten entre llamadas.
fn getptd(m: &mut dyn Mem, host: &mut dyn OpsHost, mut r: Regs) -> Regs {
    let cur = m.r32(PTD_SLOT);
    if cur >= 0x1_0000 && cur != u32::MAX {
        r[0] = cur;
        return r;
    }
    let p = host.call(m, 0x63cb41, 0, &[0x74]);
    m.w32(p + 0x50, 0x006C_AFC0);
    m.w32(p + 0x14, 1);
    m.w32(p, 1);
    m.w32(p + 4, u32::MAX);
    m.w32(PTD_SLOT, p);
    r[0] = p;
    r
}

/// `_tzset` (`0x644CAF`): lee la zona con `GetTimeZoneInformation`. El enlazador no conoce la del sistema y fija la hora local en UTC
/// (`timezone` `0x6CB100` = 0, `daylight` `0x6CB104` = 0, `_dstbias` `0x6CB108` = 0; el CRT arranca en «PST8PDT»: 28800 s y horario de
/// verano), igual que el `CTime::Format` nativo de `menus.rs`. `localtime`/`mktime` llaman a `__tzset` (`0x644C81`) antes de convertir.
fn tzset(m: &mut dyn Mem, _: &mut dyn OpsHost, r: Regs) -> Regs {
    for a in [0x006C_B100, 0x006C_B104, 0x006C_B108] {
        m.w32(a, 0);
    }
    r
}

/// Creador del estado MFC por hilo `_AFX_THREAD_STATE` (`0x6593F0`, sin argumentos; Ghidra no lo cataloga porque solo se llama por
/// puntero desde `CThreadLocalObject::GetData` `0x65A5CF`): `LocalAlloc(0x40, 0x118)` (bloque a cero) y su constructor `0x6596AE`
/// (`+0xD4` y `+0x104` = -1, vtable `0x67C9D4`).
fn create_thread_state(m: &mut dyn Mem, host: &mut dyn OpsHost, mut r: Regs) -> Regs {
    let p = host.call(m, 0x63cb41, 0, &[0x118]);
    m.w32(p + 0xd4, u32::MAX);
    m.w32(p + 0x104, u32::MAX);
    m.w32(p, 0x0067_C9D4);
    r[0] = p;
    r
}

/// Creador del estado MFC por módulo e hilo `AFX_MODULE_THREAD_STATE` (`0x6595B2`, por puntero igual que el anterior): 0x3C bytes a cero
/// y su constructor `0x659878` (vtable `0x67C9E4`, `+0xC` = 0x54, `+0x28` = `0x64F79F`).
fn create_module_thread_state(m: &mut dyn Mem, host: &mut dyn OpsHost, mut r: Regs) -> Regs {
    let p = host.call(m, 0x63cb41, 0, &[0x3c]);
    m.w32(p, 0x0067_C9E4);
    m.w32(p + 0xc, 0x54);
    m.w32(p + 0x28, 0x0064_F79F);
    r[0] = p;
    r
}

pub static GRP_LIB2_NATIVE: [(u32, CoreFn); 5] = [
    (0x645243, calloc),
    (0x63edb3, getptd),
    (0x644caf, tzset),
    (0x6593f0, create_thread_state),
    (0x6595b2, create_module_thread_state),
];
