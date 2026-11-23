//! Campos persistidos de `TownSiege` (0x627920), sin reconstruir la escena viva.

/// Segmentos fijos en el orden escrito por el serializador del original.
const FIXED: &[(usize, usize)] = &[
    (4, 4),
    (0, 4),
    (0x57c, 4),
    (0x580, 4),
    (0x584, 4),
    (0x42, 2),
    (0x66, 2),
    (0x3c, 2),
    (0x68, 2),
    (0x3a, 2),
    (0x6a, 2),
    (0x44, 2),
    (0x78, 1),
    (0x6d, 1),
    (0x5d4, 1),
    (0x61c, 1),
    (0x62c, 1),
];
const IMAGE: usize = 0x650;

fn segments(total: u8, first: u8) -> Result<Vec<(usize, usize)>, &'static str> {
    let mut segments = FIXED.to_vec();
    for i in 0..5 {
        segments.push((0x21c + i * 4, 4));
        segments.push((0x230 + i * 4, 4));
    }
    segments.push((0x6e, 1));
    segments.push((0x6f, 1));
    if first != 0xff {
        for i in usize::from(first)..usize::from(total) {
            let at = 0x254 + i * 8;
            if at + 8 > IMAGE {
                return Err("lista de escena fuera del objeto");
            };
            segments.push((at, 8));
        }
    }
    Ok(segments)
}

/// Produce los campos del formato actual; no serializa los datos derivados ni punteros.
pub fn encode(image: &[u8]) -> Result<Vec<u8>, &'static str> {
    if image.len() != IMAGE {
        return Err("imagen de escena truncada");
    }
    let mut out = Vec::new();
    for (offset, len) in segments(image[0x6e], image[0x6f])? {
        out.extend_from_slice(&image[offset..offset + len]);
    }
    Ok(out)
}

/// Copia los campos guardados sobre una imagen. No ejecuta el constructor ni restaura sus estructuras auxiliares.
/// En caso de error no cambia la imagen de entrada.
pub fn decode_fields(saved: &[u8], image: &mut [u8]) -> Result<(), &'static str> {
    if image.len() != IMAGE {
        return Err("imagen de escena truncada");
    }
    if saved.len() < 0x51 {
        return Err("escena guardada truncada");
    }
    let segments = segments(saved[0x4f], saved[0x50])?;
    let len = segments.iter().map(|s| s.1).sum::<usize>();
    if saved.len() != len {
        return Err("longitud de escena guardada incompatible");
    }
    let mut pos = 0;
    for (offset, len) in segments {
        image[offset..offset + len].copy_from_slice(&saved[pos..pos + len]);
        pos += len;
    }
    Ok(())
}
