//! Tramo de `0x589360` que ordena los objetos del mapa por su clave (`objeto+0x40`) y construye la tabla acumulada `escena+0x574..0x584`.
//! Traducción 1:1 del x86 (`0x58988A..0x5899C8`); el resto del constructor de escena carga ficheros y crea objetos y no se porta aquí.

/// `list`: manejadores (`escena+0x2AC`); `key(h)`: byte `+0x40` del objeto; `hist`: 0x110 bytes desde `escena+0x574`.
pub fn order_objects(list: &mut [u16], key: &dyn Fn(u16) -> u8, hist: &mut [u8]) {
    let n = list.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if key(list[i]) > key(list[j]) {
                list.swap(i, j);
            }
        }
    }
    for &h in list.iter() {
        let k = usize::from(key(h));
        hist[k] = hist[k].wrapping_add(1);
    }
    hist[0x10] = 1;
    let mut acc = 0u8;
    for b in hist.iter_mut().take(0x11) {
        let d = *b;
        *b = acc;
        acc = acc.wrapping_add(d);
    }
}
