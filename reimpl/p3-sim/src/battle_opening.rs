//! Apertura de batalla sin vista: 0x608230 -> 0x6079D0 -> 0x60F479.
//! La vista manual se entrega como requisito explícito; aquí solo se confirma
//! la apertura automática, cuya colocación y obstáculos también son reales.
//! Generaliza la colocación y reutiliza el generador de obstáculos de los
//! commits C07 812e8d67/3e55142a, contrastados aquí con convoyes completos.
//!
//! Procedencia de la geometría: `0x58937F` pasa sección 0 a la ranura de carga
//! (`0x587ED0`); `0x5882AA..0x5882B0` lee `TileSize` en `Class22+0x510`.
//! `scripts/iso.ini`, sección `LIGHTNINGVIEW0`, contiene `TileSize=34 21 0`
//! en el CPR inglés (SHA256 del recurso:
//! `4cf94cd1c0c14d1cb771ecaedf95f6eef1edfc9fe60028a1032b6dc55b71bc4f`).
//! El ancho se carga del recurso, no del bitmap ni del gestor de batallas.
//!
//! Orden RNG de `0x605420`: ambos sorteos de viento terminan en `0x606416`;
//! la llamada virtual de apertura se ejecuta después, en `0x606C23`.
//! Por eso esta apertura debe seguir inmediatamente al inicio pendiente,
//! usando su semilla final, antes de avanzar la simulación de la batalla.
use p3_core::ships::battle::{advance_battle_rng, local_ship_artillery_strength, CommercialBattleStart};

/// Lee el ancho de la sección empleada por `class22_load_scene_resources`.
/// Entrada del anfitrión: exige un ancho positivo representable en el word que
/// consume `0x60F487`; no reproduce los errores de configuración del original.
pub fn tile_width_from_iso(data: &[u8]) -> Result<i16, String> {
    let mut section = false;
    let mut width = None;
    for raw in data.split(|&b| b == b'\n' || b == b'\r') {
        let text = String::from_utf8_lossy(raw);
        let line = text.split(';').next().unwrap_or("").trim();
        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len() - 1].trim().eq_ignore_ascii_case("LIGHTNINGVIEW0");
        } else if section {
            if let Some((key, value)) = line.split_once('=') {
                if key.trim().eq_ignore_ascii_case("TileSize") {
                    let values = value
                        .split_whitespace()
                        .take(3)
                        .map(str::parse::<i32>)
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|_| "TileSize inválido")?;
                    if values.len() != 3 || values[0] <= 0 || values[0] > i32::from(i16::MAX) {
                        return Err("TileSize requiere tres enteros y un ancho positivo de 16 bits".into());
                    }
                    if width.replace(values[0] as i16).is_some() {
                        return Err("TileSize duplicado en LIGHTNINGVIEW0".into());
                    }
                }
            }
        }
    }
    width.ok_or("falta LIGHTNINGVIEW0/TileSize en scripts/iso.ini".into())
}

/// Usa la misma precedencia de recursos que el resto de la sesión: el último
/// CPR tiene prioridad. `None` indica ausencia; una entrada inválida no cae al
/// archivo anterior ni inventa un tamaño por defecto.
pub fn tile_width_from_archives(archives: &[&[u8]]) -> Result<Option<i16>, String> {
    for bytes in archives.iter().rev() {
        let archive = p3_assets::cpr::Archive::parse(bytes).map_err(|e| format!("CPR: {e}"))?;
        if let Some(entry) = archive.find(b"scripts/iso.ini") {
            let data = archive.read(entry).ok_or("iso.ini comprimido o truncado")?;
            return tile_width_from_iso(data).map(Some);
        }
    }
    Ok(None)
}

fn r16(v: &[u8], p: usize) -> u16 {
    u16::from_le_bytes(v[p..p + 2].try_into().unwrap())
}
fn r32(v: &[u8], p: usize) -> i32 {
    i32::from_le_bytes(v[p..p + 4].try_into().unwrap())
}
fn w16(v: &mut [u8], p: usize, n: u16) {
    v[p..p + 2].copy_from_slice(&n.to_le_bytes())
}
fn w32(v: &mut [u8], p: usize, n: i32) {
    v[p..p + 4].copy_from_slice(&n.to_le_bytes())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenRequirement {
    Ready,
    Finish,
    ManualView,
}

/// No consume una petición que requiera finish o una vista manual.
pub fn requirement(battle: &CommercialBattleStart, player: u32) -> Result<OpenRequirement, &'static str> {
    let b = &battle.object;
    if b.len() < 0x1e98 {
        return Err("batalla truncada");
    }
    if b[0x5c] != 0 {
        return Ok(OpenRequirement::Finish);
    }
    let n = usize::from(b[0x558]);
    if n > 255 || 0x35a + n * 2 > 0x558 {
        return Err("participantes inválidos");
    }
    if (0..n).any(|i| u32::from(r16(b, 0x35a + i * 2)) == player && b[0x5d + i] == 0) {
        return Ok(OpenRequirement::ManualView);
    }
    Ok(OpenRequirement::Ready)
}

/// Ejecuta ambas fases antes de consumir open_for_player_requested. El tamaño
/// de baldosa procede de Class22+0x510; no se inventa un valor gráfico.
pub fn open_automatic(
    battle: &mut CommercialBattleStart,
    world: &[u8],
    captains: &[u8],
    player: u32,
    tile_width: i16,
) -> Result<(), &'static str> {
    if !battle.open_for_player_requested {
        return Err("apertura no solicitada");
    }
    if requirement(battle, player)? != OpenRequirement::Ready {
        return Err("apertura requiere finish o vista manual");
    }
    let mut staged = battle.clone();
    place_ships(&mut staged, world, captains)?;
    generate_obstacles(&mut staged, tile_width, 0xff)?;
    staged.open_for_player_requested = false;
    *battle = staged;
    Ok(())
}

/// Colocación original de los dos bandos, con filas y transición de artillería.
pub fn place_ships(battle: &mut CommercialBattleStart, world: &[u8], captains: &[u8]) -> Result<(), &'static str> {
    const RADII: [u16; 12] = [55, 66, 70, 84, 102, 121, 121, 148, 38, 42, 50, 44];
    if battle.object.len() < 0x1e98
        || world.len() % 0x180 != 0
        || captains.len() % 16 != 0
        || captains.len() / 16 > 65535
    {
        return Err("estado de colocación truncado");
    }
    let counts = [
        usize::from(r16(&battle.object, 0x658)),
        usize::from(r16(&battle.object, 0x65a)),
    ];
    let n = counts[0] + counts[1];
    if counts.contains(&0) || n != battle.local_ships.len() || n != battle.spatial_1.len() {
        return Err("bandos de colocación incompatibles");
    }
    let mut object = battle.object.clone();
    let mut locals = battle.local_ships.clone();
    let mut pool = battle.collisions.clone();
    let mut worlds = Vec::with_capacity(n);
    let mut strengths = Vec::with_capacity(n);
    for local in &locals {
        if local.len() < 0x164 {
            return Err("barco local truncado");
        }
        let id = r32(local, 4) as u32 as usize;
        let at = id.checked_mul(0x180).ok_or("índice mundial excesivo")?;
        let ship = world.get(at..at + 0x180).ok_or("barco mundial ausente")?;
        if usize::from(ship[0xe]) >= RADII.len() {
            return Err("tipo de casco desconocido");
        }
        worlds.push(ship);
        strengths.push(local_ship_artillery_strength(
            ship,
            captains,
            (captains.len() / 16) as u16,
        )?);
    }
    let dx = r32(worlds[0], 0x1c).wrapping_sub(r32(&object, 0xad4)) >> 13;
    let dy = r32(worlds[0], 0x20).wrapping_sub(r32(&object, 0xad8)) >> 13;
    let squared = dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy));
    if squared < 0 {
        return Err("distancia de colocación fuera del dominio");
    }
    let magnitude = (f64::from(squared).sqrt()) as i32;
    let (nx, ny) = if magnitude == 0 {
        (181, 181)
    } else {
        (
            dx.wrapping_shl(8)
                .checked_div(magnitude)
                .ok_or("división de colocación inválida")?,
            dy.wrapping_shl(8)
                .checked_div(magnitude)
                .ok_or("división de colocación inválida")?,
        )
    };
    let mut rng = r32(&object, 0xadc) as u32;
    let mut begin = 0;
    for side in 0..2 {
        let count = counts[side];
        let columns = ((count - 1) as f64).sqrt() as i32 + 1;
        let sign = if side == 0 { 1i32 } else { -1i32 };
        let initial = if side == 0 { 250 } else { 300 };
        let (mut radial_x, mut radial_y) = (nx.wrapping_mul(300), ny.wrapping_mul(300));
        let centered = |rx: i32, ry: i32| -> (i32, i32) {
            (
                0x1100 + sign.wrapping_mul(rx >> 8) - ny.wrapping_mul((columns - 1).wrapping_mul(200)).wrapping_shr(9),
                0xf02 + sign.wrapping_mul(ry >> 8) + nx.wrapping_mul((columns - 1).wrapping_mul(200)).wrapping_shr(9),
            )
        };
        let (mut x, mut y) = centered(nx.wrapping_mul(initial), ny.wrapping_mul(initial));
        let mut rear = false;
        for j in 0..count {
            let i = begin + j;
            rng = advance_battle_rng(rng);
            let px = x.wrapping_add(((rng & 127) as i32 - 64).wrapping_mul(ny) >> 8);
            rng = advance_battle_rng(rng);
            let py = y.wrapping_sub(((rng & 127) as i32 - 64).wrapping_mul(nx) >> 8);
            w16(&mut locals[i], 0xa, px as u16);
            w16(&mut locals[i], 0xe, py as u16);
            let heading = worlds[i][1].wrapping_sub(5).wrapping_mul(16);
            locals[i][0x11] = heading;
            w16(&mut locals[i], 0x24, u16::from(heading));
            if side == 1 && rear {
                locals[i][0x12a] |= 1
            }
            let radius = i32::from(RADII[usize::from(worlds[i][0xe])]) + 360;
            pool.set_bounds(
                battle.spatial_1[i],
                px.wrapping_sub(radius) as u16,
                py.wrapping_sub(radius) as u16,
                px.wrapping_add(radius) as u16,
                py.wrapping_add(radius) as u16,
            )?;
            if j + 1 == count {
                continue;
            }
            if strengths[i] > 0 && strengths[i + 1] == 0 {
                radial_x = radial_x.wrapping_add(nx.wrapping_mul(400));
                radial_y = radial_y.wrapping_add(ny.wrapping_mul(400));
                (x, y) = centered(radial_x, radial_y);
                if side == 1 {
                    rear = true
                }
            } else if (j + 1) % (columns as usize) == 0 {
                radial_x = radial_x.wrapping_add(nx.wrapping_mul(200));
                radial_y = radial_y.wrapping_add(ny.wrapping_mul(200));
                (x, y) = centered(radial_x, radial_y)
            } else {
                x = x.wrapping_add(ny.wrapping_mul(200) >> 8);
                y = y.wrapping_sub(nx.wrapping_mul(200) >> 8)
            }
        }
        begin += count;
    }
    w32(&mut object, 0xadc, rng as i32);
    battle.object = object;
    battle.local_ships = locals;
    battle.collisions = pool;
    Ok(())
}

/// Parche que el anfitrión debe dibujar al abrir la vista de combate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleObstacleDraw {
    pub kind: u8,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
/// Generación de obstáculos de 0x60F479 para los barcos de ambos
/// bandos. La vista devuelve sus operaciones de dibujo al anfitrión.
pub fn generate_obstacles(
    battle: &mut CommercialBattleStart,
    tile_width: i16,
    selected: u8,
) -> Result<Vec<BattleObstacleDraw>, &'static str> {
    const SIZES12: [(u16, u16); 5] = [(2, 2), (3, 2), (4, 4), (6, 4), (6, 5)];
    const SIZES13: [(u16, u16); 3] = [(4, 4), (4, 4), (6, 4)];
    if battle.object.len() < 0x1e98
        || usize::from(r16(&battle.object, 0x658)) + usize::from(r16(&battle.object, 0x65a)) != battle.local_ships.len()
        || battle.spatial_1.len() != battle.local_ships.len()
        || tile_width <= 0
    {
        return Err("batalla o tamaño de baldosa no admitidos");
    }
    let mut object = battle.object.clone();
    let mut pool = battle.collisions.clone();
    let mut rng = r32(&object, 0xadc) as u32;
    let mut draws = Vec::new();
    let half = i32::from(tile_width >> 1);
    let mut roll = || {
        rng = advance_battle_rng(rng);
        rng
    };
    for kind in [12u8, 13] {
        let modulus = if kind == 12 { 65 } else { 25 };
        let sum = roll() % modulus + roll() % modulus + roll() % modulus;
        let mut count = if kind == 12 {
            if sum < 96 {
                64i32 - sum as i32
            } else {
                sum as i32 - 128
            }
        } else if sum < 36 {
            48i32 - sum as i32
        } else {
            sum as i32 - 24
        };
        if count < 0 {
            count = 0
        }
        let offset = if kind == 12 { 0x872 } else { 0x672 };
        let mut i = 0usize;
        let mut failed = 0u8;
        while i < 64 {
            let at = offset + i * 8;
            if i >= count as usize {
                w16(&mut object, at, u16::MAX);
                i += 1;
                continue;
            }
            let size = roll();
            let (w, h) = if kind == 12 {
                SIZES12[size as usize % SIZES12.len()]
            } else {
                SIZES13[size as usize % SIZES13.len()]
            };
            let wx = u32::from(w - 1);
            let hy = u32::from(h - 1);
            let x = (roll() % (256 - (u32::from(h) >> 1) - (u32::from(w) >> 1)) + (u32::from(w) >> 1)) as u16;
            let y = (roll() % (452 - wx - hy) + wx + hy) as u16;
            for (j, v) in [x, y, w, h].into_iter().enumerate() {
                w16(&mut object, at + j * 2, v)
            }
            let mut valid = true;
            let overlaps = |other_x: u16, other_y: u16| {
                i32::from(other_x) <= i32::from(x) + 8
                    && i32::from(x) <= i32::from(other_x) + 8
                    && i32::from(other_y) <= i32::from(y) + 16
                    && i32::from(y) <= i32::from(other_y) + 16
            };
            if kind == 13 {
                for j in 0..64 {
                    let p = 0x872 + j * 8;
                    let ox = r16(&object, p);
                    if ox != u16::MAX && (ox as i16) > 0 && overlaps(ox, r16(&object, p + 2)) {
                        valid = false
                    }
                }
            }
            for j in 0..i {
                let p = offset + j * 8;
                if overlaps(r16(&object, p), r16(&object, p + 2)) {
                    valid = false
                }
            }
            for id in &battle.spatial_1 {
                let p = usize::from(*id) * 2;
                if p + 1 >= pool.x.len() || p + 1 >= pool.y.len() {
                    return Err("tabla espacial truncada");
                }
                let left = i32::from(pool.x[p] as i16) / 34;
                let right = i32::from(pool.x[p + 1] as i16) / 34;
                let top = i32::from(pool.y[p] as i16) / 17;
                let bottom = i32::from(pool.y[p + 1] as i16) / 17;
                if left <= i32::from(x) && i32::from(x) <= right && top <= i32::from(y) && i32::from(y) <= bottom {
                    valid = false
                }
            }
            if !valid {
                failed = failed.wrapping_add(1);
                if failed == 0 {
                    count -= 1
                }
                continue;
            }
            if u16::from(selected) == r16(&object, 0x662) {
                draws.push(BattleObstacleDraw {
                    kind,
                    x,
                    y,
                    width: w,
                    height: h,
                })
            }
            let x2 = i32::from(x) * 2 + i32::from(y & 1);
            let rect = [
                (x2 - i32::from(w) + 1) * half,
                (i32::from(y) - i32::from(w) - i32::from(h) + 1) * half,
                (x2 + 1 + i32::from(h)) * half,
                (i32::from(y) + 1) * half,
            ];
            let first = pool.add_growing(
                if kind == 12 { 2 } else { 3 },
                i as u8,
                rect[0] as u16,
                rect[1] as u16,
                rect[2] as u16,
                rect[3] as u16,
            )?;
            let p = usize::from(first) * 2;
            pool.add_growing(
                if kind == 12 { 4 } else { 5 },
                i as u8,
                pool.x[p].wrapping_sub(200),
                pool.y[p].wrapping_sub(200),
                pool.x[p + 1].wrapping_add(200),
                pool.y[p + 1].wrapping_add(200),
            )?;
            if kind == 12 {
                failed = 0
            }
            i += 1;
        }
    }
    w32(&mut object, 0xadc, rng as i32);
    for (at, v) in [
        (0x20, pool.free_head),
        (0x34, pool.x_head),
        (0x44, pool.y_head),
        (0x54, pool.high_water),
        (0x56, pool.kinds.len() as u16),
        (0x58, (pool.kinds.len() * 2) as u16),
        (0x5a, pool.matrices[0].len() as u16),
    ] {
        w16(&mut object, at, v)
    }
    battle.object = object;
    battle.collisions = pool;
    Ok(draws)
}
