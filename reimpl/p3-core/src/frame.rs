//! Marco de pila del x86 modelado en memoria: forma común para portar funciones con marco SEH (`fs:[0]`) o que pasan direcciones de
//! variables locales a otras funciones.
//!
//! El oráculo (`oracle/hookcase.py`) llama a la función original con un ESP de entrada fijo y lo guarda en el caso (`esp`). El porte recibe
//! ese mismo `esp` y coloca sus variables locales en `esp - desplazamiento`, igual que el original. Así:
//! - las direcciones de los locales que se pasan a otras funciones son las mismas en los dos lados y se comparan como argumentos;
//! - el contenido de esos locales en el momento de cada llamada se compara copiando sus palabras al final del registro de llamada
//!   (`peek` del gancho en el oráculo, [`peek`] en el porte);
//! - el registro SEH (`[esp-0xC]` enlace previo, `[esp-8]` manejador, `[esp-4]` estado de destrucción) y `fs:[0]` (base de FS 0, página
//!   cero) se escriben como en el original y quedan en la imagen comparada.
//!
//! Los manejadores nunca se ejecutan: ninguna llamada interceptada lanza excepciones. El estado de destrucción se escribe en los mismos
//! puntos que el original porque es memoria observable del marco.
use crate::flatmem::Mem;

/// Marco de una función con ESP de entrada `esp` (apunta a la dirección de retorno).
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub esp: u32,
}

impl Frame {
    pub fn new(esp: u32) -> Self {
        Self { esp }
    }
    /// Dirección del local en `esp - off`.
    pub fn at(&self, off: u32) -> u32 {
        self.esp.wrapping_sub(off)
    }
    /// Dirección del argumento `i` de la pila (`esp + 4 + 4i`).
    pub fn arg(&self, i: u32) -> u32 {
        self.esp.wrapping_add(4 + 4 * i)
    }
    /// Hueco de pila por debajo de los locales (un `push` que reserva un argumento pasado por valor, p. ej. un CString que construye la
    /// función llamada): declara los 4 bytes en `esp - off` y escribe el valor empujado.
    pub fn push(&self, m: &mut dyn Mem, off: u32, v: u32) -> u32 {
        let a = self.at(off);
        for k in 0..4 {
            m.w8_seed(a.wrapping_add(k));
        }
        m.w32(a, v);
        a
    }
    /// Prólogo SEH de MSVC (`push -1; push handler; push fs:[0]; mov fs:[0], esp`). Devuelve el enlace previo.
    pub fn seh_enter(&self, m: &mut dyn Mem, handler: u32) -> u32 {
        let prev = m.r32(0);
        m.w32(self.at(4), 0xffff_ffff);
        m.w32(self.at(8), handler);
        m.w32(self.at(0xc), prev);
        m.w32(0, self.at(0xc));
        prev
    }
    /// Estado de destrucción del marco (`[esp-4]`).
    pub fn seh_state(&self, m: &mut dyn Mem, state: u32) {
        m.w32(self.at(4), state);
    }
    /// Epílogo SEH: `fs:[0]` vuelve al enlace guardado en el marco.
    pub fn seh_leave(&self, m: &mut dyn Mem) {
        let prev = m.r32(self.at(0xc));
        m.w32(0, prev);
    }
}

/// Argumentos de una llamada más `words` palabras leídas en `ptr` (el contenido de un local pasado por dirección), en el orden del
/// registro del oráculo: `args..., [ptr], [ptr+4], ...`.
pub fn peek(m: &mut dyn Mem, args: &[u32], ptr: u32, words: u32) -> Vec<u32> {
    let mut v = args.to_vec();
    for k in 0..words {
        v.push(m.r32(ptr.wrapping_add(4 * k)));
    }
    v
}
