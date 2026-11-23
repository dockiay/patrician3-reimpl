//! Vista del combate naval sobre la imagen de memoria del original: funciones que montan registros en la pila y llaman a la interfaz
//! por [`OpsHost`] con la dirección del original. `esp` es el puntero de pila a la entrada (apunta a la dirección de retorno): los
//! registros de pila del original viven a distancia fija de él.

use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;

fn virt(m: &mut dyn Mem, host: &mut dyn OpsHost, obj_ptr: u32, slot: u32, arg: u32) -> u32 {
    let obj = m.r32(obj_ptr);
    let vt = m.r32(obj);
    let f = m.r32(vt.wrapping_add(slot));
    host.call(m, f, obj, &[arg])
}

/// 0x006102A9 `enter_view` (thiscall, sin argumentos): crea en la pila el registro de operación `{0x96, bandera `+0x662`, jugador, 0}`,
/// lo entrega a la ventana `0x6DF2F0`, avisa a la interfaz (`0x589ED0`, virtuales `+0xCC` y `+0x34` de `0x6E51AC` y de `0x6CBDD8`),
/// cierra el elemento activo de la lista `0x6DA5F0` y continúa en `0x6107B9` y `0x610379` (devuelve lo de esta última).
pub fn battle_enter_view(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let rec = esp.wrapping_sub(0x14);
    host.call(m, 0x535740, rec, &[]);
    let player = m.r32(0x6dfc14);
    let flag = m.r16(this + 0x662);
    m.w32(rec + 8, player);
    m.w32(rec, 0x96);
    m.w32(rec + 4, u32::from(flag));
    m.w32(rec + 0xc, 0);
    host.call(m, 0x54aa70, 0x6df2f0, &[rec]);
    let lo = m.r8(this + 0x662);
    m.w8(0x6e59cc, lo);
    let ui = m.r32(0x6e51ac);
    host.call(m, 0x589ed0, ui, &[this + 0xa7c, this + 0xa80]);
    virt(m, host, 0x6e51ac, 0xcc, 1);
    virt(m, host, 0x6e51ac, 0x34, 1);
    virt(m, host, 0x6cbdd8, 0xcc, 0);
    virt(m, host, 0x6cbdd8, 0x34, 0);
    let list = host.call(m, 0x4b9730, 0x6da5f0, &[]);
    if list != 0 {
        host.call(m, 0x4b8380, list, &[0]);
    }
    host.call(m, 0x6107b9, this, &[]);
    host.call(m, 0x610379, this, &[])
}
