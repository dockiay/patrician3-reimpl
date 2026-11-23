//! Máscara de tierra del ataque a una guarida (`0x005FFC10`).
//! La creación del índice espacial es un efecto del anfitrión, no se simula aquí.
use super::{w16, w32};

#[derive(Clone, Copy, Debug)]
pub struct TownProjection {
    pub width: i32,
    pub height: i32,
    pub x_offset: i32,
    pub y_offset: i32,
}

/// Argumentos de la reserva de colisión `0x00619450` después de construir la máscara.
pub const LAND_COLLISION_ARGS: [u32; 6] = [7, 0, 0, 0, 0x1100, 0xf02];

/// Reconstruye en copia la máscara para un plano de escondite, y escribe el ID
/// devuelto por el índice espacial suministrado por el llamante. No reserva el
/// índice ni ejecuta el resto del inicio de batalla.
pub fn hideout_land_mask(
    object: &mut [u8],
    width: u32,
    height: u32,
    cells: &[u8],
    projection: TownProjection,
    collision_id: u16,
) -> Result<(), &'static str> {
    if object.len() < 0x2ee8 {
        return Err("objeto de ataque urbano truncado");
    }
    if width > 4096 || height > 4096 {
        return Err("dimensiones del plano excesivas");
    }
    let count = width.checked_mul(height).ok_or("plano excesivo")?;
    if count > 0x100000 || cells.len() < count as usize {
        return Err("plano de guarida truncado o excesivo");
    }
    let mut mask = [1u8; 4096];
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (63i32, 63i32, 0i32, 0i32);
    for x in 0..width {
        for y in 0..height {
            let terrain = cells[(x * height + y) as usize];
            if matches!(terrain, 6 | 12 | 14 | 30) {
                continue;
            }
            let half = (y >> 1) as i32;
            let (px, py) = if y & 1 != 0 {
                (
                    (x as i32).wrapping_add(1).wrapping_mul(projection.width),
                    projection
                        .height
                        .wrapping_add(1)
                        .wrapping_mul(half)
                        .wrapping_add(projection.height),
                )
            } else {
                (
                    projection
                        .width
                        .wrapping_mul(x as i32)
                        .wrapping_add(projection.x_offset),
                    projection
                        .height
                        .wrapping_add(1)
                        .wrapping_mul(half)
                        .wrapping_sub(projection.y_offset)
                        .wrapping_add(projection.height),
                )
            };
            let gx = (((px as u32).wrapping_shl(6) / 4352) as u16).min(63) as usize;
            let tmp = (py as u32).wrapping_mul(17) / 11;
            let gy = ((tmp.wrapping_shl(6) / 3842) as u16).min(63) as usize;
            let cell = &mut mask[gx * 64 + gy];
            if *cell != 0 {
                *cell = 0;
                min_x = min_x.min(gx as i32);
                max_x = max_x.max(gx as i32);
                min_y = min_y.min(gy as i32);
                max_y = max_y.max(gy as i32)
            }
        }
    }
    object[0x1ee0..0x2ee0].copy_from_slice(&mask);
    w32(object, 0x2ee0, width as i32);
    w32(object, 0x2ee4, height as i32);
    w16(object, 0x1ed4, collision_id);
    w32(object, 0x1ed8, min_x + max_x - 63);
    w32(object, 0x1edc, 2 * (min_y + max_y - 63));
    Ok(())
}
