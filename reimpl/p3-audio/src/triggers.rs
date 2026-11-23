//! J04: reglas que eligen el ID de muestra en los disparadores de sonido del juego (solo cálculo, sin dispositivo).
//! Cada función cita su dirección x86. `rand` es el valor de `_rand()` (0..=32767) que aporta el llamador; el original
//! toma el generador del CRT. Evidencia estática (Ghidra + desensamblado), sin Miles ni partida.

/// `RAND_MAX + 1` como float (`DAT_0066CB34` = 32768.0f).
pub const RAND_SCALE: i32 = 32768;
/// IDs que `class22_play_sound_at_position` (0x595B00) desvía a la ruta exclusiva: `0xEA9B < id < 61000` (60060..60999).
pub const EXCLUSIVE_RANGE: core::ops::Range<u16> = 0xEA9C..61000;

/// Sonidos del mapa (`scroll_map_play_ambient_sound`, 0x459790, casos 1..=5): `k*1000 + trunc(rand/32768 * count)`.
/// `rand/32768` es exacto en float y el producto con un recuento pequeño también, por lo que se calcula con enteros.
/// `count` es `FUN_00443690(k*1000)`. Los casos fuera de 1..=5 no tienen sonido en esta rama.
pub fn ambient_sound_id(kind: u32, count: u32, rand: u32) -> Option<u32> {
    if !(1..=5).contains(&kind) {
        return None;
    }
    let pick = ((u64::from(rand) * u64::from(count)) / RAND_SCALE as u64) as u32;
    Some(kind * 1000 + pick)
}

/// Sonido de un edificio pendiente (`class22_start_pending_building_sounds`, 0x594770).
/// `base = entry*SoundsProGebaeude + GebaeudeSoundListBase` (+0xC7E0, +0xC7DC); `count = FUN_00443690(base)`.
/// Solo consume `rand` si `count > 1`.
pub fn building_sound_id(entry: u16, per_building: u32, list_base: u32, count: u32, rand: impl FnOnce() -> u32) -> u32 {
    let base = u32::from(entry) * per_building + list_base;
    let pick = if count > 1 { rand() % count } else { 0 };
    base + pick
}

/// Derrumbe de un edificio (`class22_remove_building_with_effects`, 0x599040): `rand % count + BuildingCollapseList`.
/// `list` es `[escena+0xC994]` (iso.ini `BuildingCollapseList`, 9000); `count = FUN_00443690(list)`.
/// Sin muestras en la lista no suena. El divisor no es cero si `count != 0`.
pub fn collapse_sound_id(list: u32, count: u32, rand: u32) -> Option<u32> {
    (count != 0).then(|| rand % count + list)
}

/// Sonido de barco del puerto (`town_view_play_ship_sound_at_position`, 0x59A090): `[escena+0xCBA0] + arg`.
/// `arg = 1` es llegar (`legtan.wav`), `0` es zarpar (`legtab.wav`).
pub fn harbor_ship_sound_id(list: u32, arg: u32) -> u32 {
    list + arg
}

/// Despacho de `class22_play_sound_at_position` (0x595B00) según el ID recibido (`& 0xFFFF`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    /// 0xEA9C..61000: `class22_play_ship_sound_exclusive` (0x594AA0).
    Exclusive(u16),
    /// Resto: ranura libre entre 4..13 con posición y duración.
    Positional(u16),
}

pub fn route(id: u32) -> Route {
    let id = id as u16;
    if EXCLUSIVE_RANGE.contains(&id) {
        Route::Exclusive(id)
    } else {
        Route::Positional(id)
    }
}

/// Primera ranura posicional libre. `busy[i]` es el byte `+0xCA0E` del registro `i` (de 0x18 bytes desde +0xC998);
/// el original busca entre 4 y 13 y devuelve `None` al agotarlas (retorna 0 sin sonar).
pub fn free_positional_slot(busy: &[bool; 14]) -> Option<usize> {
    (4..14).find(|&i| !busy[i])
}

/// Pestaña 0 del menú de la iglesia (`church_side_menu_update`, 0x5A59A0): literal 0x1BBC.
pub const CHURCH_TAB_SOUND: u32 = 0x1BBC;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ambient_ranges() {
        assert_eq!(ambient_sound_id(1, 1, 32767), Some(1000));
        assert_eq!(ambient_sound_id(3, 4, 0), Some(3000));
        assert_eq!(ambient_sound_id(3, 4, 32767), Some(3003));
        assert_eq!(ambient_sound_id(5, 9, 32767), Some(5008));
        assert_eq!(ambient_sound_id(6, 1, 0), None);
    }

    #[test]
    fn building_uses_rand_only_with_choices() {
        let mut calls = 0;
        assert_eq!(
            building_sound_id(4, 10, 10000, 1, || {
                calls += 1;
                7
            }),
            10040
        );
        assert_eq!(calls, 0);
        assert_eq!(building_sound_id(4, 10, 10000, 7, || 9), 10042);
    }

    #[test]
    fn collapse_and_ship() {
        assert_eq!(collapse_sound_id(9000, 1, 12345), Some(9000));
        assert_eq!(collapse_sound_id(9000, 0, 1), None);
        assert_eq!(harbor_ship_sound_id(13000, 1), 13001);
    }

    #[test]
    fn pan_volume_matches_x86() {
        let fx: serde_json::Value = serde_json::from_str(include_str!("../tests/fixtures/sound_pos.json")).unwrap();
        let mut n = 0;
        for c in fx["cases"].as_array().unwrap() {
            let f: Vec<i32> = c["f"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u32 as i32).collect();
            let s = PanScene { view_w: f[0], view_h: f[1], origin_x: f[2], origin_y: f[3], pan_dist: f[4], vol_dist: f[5] };
            let (x, y) = (c["x"].as_u64().unwrap() as u32 as i32, c["y"].as_u64().unwrap() as u32 as i32);
            let want = (c["result"]["pan"].as_u64().unwrap() as u8, c["result"]["vol"].as_u64().unwrap() as u8);
            assert_eq!(pan_volume(&s, x, y), Some(want), "caso {n}: {c}");
            n += 1;
        }
        eprintln!("probados {n}");
        assert!(n > 5000);
    }

    #[test]
    fn routing_and_slots() {
        assert_eq!(route(60000), Route::Positional(60000));
        assert_eq!(route(60100), Route::Exclusive(60100));
        assert_eq!(route(0xEA9C), Route::Exclusive(0xEA9C));
        assert_eq!(route(0xEA9B), Route::Positional(0xEA9B));
        assert_eq!(route(61000), Route::Positional(61000));
        let mut busy = [false; 14];
        busy[4] = true;
        assert_eq!(free_positional_slot(&busy), Some(5));
        for b in busy.iter_mut().skip(4) {
            *b = true;
        }
        assert_eq!(free_positional_slot(&busy), None);
    }
}

/// Campos de escena que usa `class22_sound_pan_volume`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PanScene {
    /// `+0x2C`, `+0x30`: tamaño de la vista.
    pub view_w: i32,
    pub view_h: i32,
    /// `+0xC650`, `+0xC654`: origen de la cámara.
    pub origin_x: i32,
    pub origin_y: i32,
    /// `+0xC7D4` (`PanDist`) y `+0xC7D8` (`VolDist`) de iso.ini.
    pub pan_dist: i32,
    pub vol_dist: i32,
}

/// `0x594B50`: `(panorámica, volumen)` de un punto del mapa; ambos u8. Los divisores cero (`PanDist` o `VolDist` = 0, o
/// `VolDist²` ≡ 0 mod 2³²) y `i32::MIN / -1` detienen el x86 (#DE): aquí devuelven `None`.
pub fn pan_volume(s: &PanScene, x: i32, y: i32) -> Option<(u8, u8)> {
    let dx = x.wrapping_add(0i32.wrapping_sub(s.view_w / 2).wrapping_sub(s.origin_x));
    let dy = y.wrapping_add(0i32.wrapping_sub(s.view_h / 2).wrapping_sub(s.origin_y));
    let d2 = (dx.wrapping_mul(dx)).wrapping_add(dy.wrapping_mul(dy)) as u32;
    let num = (d2 << 7).wrapping_sub(d2);
    let den = (s.vol_dist.wrapping_mul(s.vol_dist)) as u32;
    let q = num.checked_div(den)?;
    let vol = if q < 0x7F { (0x7Fu32 - q) as u8 } else { 0 };
    let pan = (dx << 6).checked_div(s.pan_dist)?.checked_add(0x40)?;
    let pan = if pan < 0 {
        0
    } else if pan > 0x7F {
        0x7F
    } else {
        pan as u8
    };
    Some((pan, vol))
}
