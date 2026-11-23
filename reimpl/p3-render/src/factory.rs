//! Fábrica de objetos de la escena `0x557D10(tipo)`: reserva el objeto y ejecuta su constructor. Constructores comunes: `0x559910` (`+0`, `+4`), `0x558F00` (bloque de
//! posición en `+8`: `+0xC`, `+0x10` a 0, `+0x14` rumbo inicial `[0x67792B]`, `+0x15` fotograma 0, `+0x16` enlace 0, `+0x18` = 1) y `0x558750`, y después los punteros de
//! tablas virtuales propios de cada tipo (`+0`, `+8`, `+0x1C`). El objeto nuevo está a cero (el original no inicia la memoria). Reproducidos los tipos 29 (nieve,
//! `0x576030`), 33 (lluvia, `0x575950`) y 34 (grieta, `0x57FBA0`); el fixture `factory.json` guarda los 36 para ampliar.

/// Tipo de objeto de la fábrica.
pub const SNOW: u32 = 29;
pub const RAIN: u32 = 33;
pub const CRACK: u32 = 34;

fn w16(o: &mut [u8], a: usize, v: u16) {
    o[a..a + 2].copy_from_slice(&v.to_le_bytes());
}

fn w32(o: &mut [u8], a: usize, v: u32) {
    o[a..a + 4].copy_from_slice(&v.to_le_bytes());
}

fn base(o: &mut [u8], dir_byte: u8) {
    o[0x14] = dir_byte;
    o[0x18] = 1;
}

/// Bytes del objeto del tipo `ty` tras su constructor. `dir_byte` es `[0x67792B]` (dato del ejecutable). `None` si el tipo aún no está reproducido.
pub fn construct(ty: u32, dir_byte: u8) -> Option<Vec<u8>> {
    let mut o;
    match ty {
        SNOW => {
            o = vec![0; 0x28];
            base(&mut o, dir_byte);
            w32(&mut o, 0, 0x0067_6190);
            w32(&mut o, 8, 0x0067_618C);
            w32(&mut o, 0x1C, 0x0067_6180);
        }
        RAIN => {
            o = vec![0; 0x24];
            base(&mut o, dir_byte);
            w32(&mut o, 0, 0x0067_6128);
            w32(&mut o, 8, 0x0067_6120);
            w32(&mut o, 0x1C, 0x0067_6114);
            w16(&mut o, 0x20, 0);
        }
        CRACK => {
            o = vec![0; 0x30];
            base(&mut o, dir_byte);
            w32(&mut o, 0, 0x0067_6760);
            w32(&mut o, 8, 0x0067_675C);
            w32(&mut o, 0x1C, 0x0067_6750);
            w32(&mut o, 0x20, 0xFFFF_FFFF);
            w32(&mut o, 0x24, 0xFFFF_FFFF);
            o[0x29] = 0xFF;
        }
        _ => return None,
    }
    Some(o)
}
