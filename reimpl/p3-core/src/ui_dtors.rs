//! Destructores que ejecuta la carga de una partida sobre los vectores del mundo (`0x00530230` los pasa a `0x639DA8`): el de comerciante
//! (`0x004F3FA0`, vector `+0x78` de 0x650 bytes), su base (`0x004F6050`) y el de registro de asedio (`0x00633520`, vector `[0x6DF270]` de
//! 0x1024 bytes). Misma firma que las demás funciones a mano de la interfaz: `ecx` = `this`, `esp` de entrada.
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;

const FREE: u32 = 0x0064_f7e2;

/// `0x004F6050` (`ecx` = comerciante): libera el buffer de `+0xEC` si no es nulo (`0x64F7E2`) y lo anula. Devuelve el resultado de la
/// liberación (EAX) o 0 sin buffer.
pub fn merchant_base_destructor(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _esp: u32) -> u32 {
    let p = m.r32(this.wrapping_add(0xec));
    if p == 0 {
        return p;
    }
    let r = host.call(m, FREE, 0, &[p]);
    m.w32(this.wrapping_add(0xec), 0);
    r
}

/// `0x004F3FA0` (`ecx` = comerciante): libera y anula las once ranuras de punteros de `+0x624` (las no nulas con `0x64F7E2`) y continúa con
/// la base `0x004F6050` (salto final: su EAX es el de este destructor).
pub fn merchant_destructor(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _esp: u32) -> u32 {
    for k in 0..11u32 {
        let a = this.wrapping_add(0x624 + 4 * k);
        let p = m.r32(a);
        if p != 0 {
            host.call(m, FREE, 0, &[p]);
        }
        m.w32(a, 0);
    }
    host.call(m, 0x4f6050, this, &[])
}

/// `0x00633520` (`ecx` = registro de asedio): salto directo a `siege_record_clear` (`0x006339C0`); su EAX es el de este destructor.
pub fn siege_record_destructor(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _esp: u32) -> u32 {
    host.call(m, 0x6339c0, this, &[])
}
