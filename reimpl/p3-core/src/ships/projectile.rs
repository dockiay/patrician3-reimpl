//! Proyectiles del combate: avance y retirada (0x00603130), impacto (0x00603470) y liberación común (0x00602DA0).
//!
//! El proyectil conserva sus 0x38 bytes originales. Los efectos de pantalla (explosiones, salpicaduras, sonido, que
//! sólo existen si este combate es el visible y usan `rand()` de la CRT) y la liberación del recurso gráfico salen como
//! eventos para el anfitrión; el estado de simulación no depende de ellos.
//! Fixture: `oracle/x86_battle_projectile.py`.

use super::spatial::CollisionIndex;

pub const SIZE: usize = 0x38;
pub type Projectile = [u8; SIZE];

/// Restaura el registro actual de 31 bytes mediante `0x00602EF0`.
/// El proyectil ya ha sido construido y registrado en el índice espacial; por
/// eso se conservan sus campos transitorios, incluido el identificador espacial.
/// `random` es el valor de la llamada CRT realizada durante la lectura.
pub fn deserialize_current(
    p: &mut Projectile,
    record: &[u8],
    battle: u32,
    id: u16,
    random: u32,
) -> Result<usize, &'static str> {
    if record.len() < 31 {
        return Err("proyectil truncado");
    }
    p[0x28..0x2c].copy_from_slice(&battle.to_le_bytes());
    p[0x2c..0x2e].copy_from_slice(&id.to_le_bytes());
    p[..0x19].copy_from_slice(&record[..0x19]);
    p[0x1c..0x20].copy_from_slice(&record[0x19..0x1d]);
    p[0x24..0x26].copy_from_slice(&record[0x1d..0x1f]);
    p[0x30] = heading(i32_at(p, 0xc), i32_at(p, 0x10));
    let distance = i32_at(p, 0x1c);
    let product = distance.wrapping_mul(((random & 0x1f) + 0x60) as i32);
    let adjusted = product.wrapping_add(if product < 0 { 0x1ff } else { 0 });
    p[0x32..0x34].copy_from_slice(&((adjusted >> 9) as i16).to_le_bytes());
    Ok(31)
}

fn i32_at(p: &Projectile, o: usize) -> i32 {
    i32::from_le_bytes(p[o..o + 4].try_into().unwrap())
}
fn i16_at(p: &Projectile, o: usize) -> i16 {
    i16::from_le_bytes([p[o], p[o + 1]])
}
fn u16_at(p: &Projectile, o: usize) -> u16 {
    u16::from_le_bytes([p[o], p[o + 1]])
}

/// Operaciones del índice espacial que usan los proyectiles (0x006197A0 y 0x00619560).
pub trait Spatial {
    fn bounds(&mut self, slot: u16, xmin: i16, ymin: i16, xmax: i16, ymax: i16);
    fn remove(&mut self, slot: u16);
    /// 0x00619450.
    fn add(&mut self, kind: u8, reference: u8, xmin: i16, ymin: i16, xmax: i16, ymax: i16) -> Option<u16>;
}

impl Spatial for CollisionIndex {
    fn bounds(&mut self, slot: u16, xmin: i16, ymin: i16, xmax: i16, ymax: i16) {
        CollisionIndex::bounds(self, slot, xmin, ymin, xmax, ymax)
    }
    fn remove(&mut self, slot: u16) {
        CollisionIndex::remove(self, slot)
    }
    fn add(&mut self, kind: u8, reference: u8, xmin: i16, ymin: i16, xmax: i16, ymax: i16) -> Option<u16> {
        CollisionIndex::add(self, kind, reference, xmin, ymin, xmax, ymax)
    }
}

/// Mapa de bits (`battle+0x664`) y tabla de punteros (`battle+0x668`) de proyectiles vivos.
#[derive(Clone, Debug, Default)]
pub struct ProjectileTable {
    pub bitmap: Vec<u32>,
    pub slots: Vec<Option<Projectile>>,
}

impl ProjectileTable {
    pub fn new(slots: usize) -> Self {
        ProjectileTable {
            bitmap: vec![0; (slots + 31) / 32],
            slots: vec![None; slots],
        }
    }

    /// 0x00602DA0: borra el bit y la ranura del proyectil, devuelve su registro espacial y lo libera.
    /// Devuelve el identificador del recurso gráfico (0 = ninguno) que el anfitrión libera si hay vista.
    fn release<S: Spatial>(&mut self, id: usize, spatial: &mut S) -> u32 {
        let p = self.slots[id].expect("proyectil vivo");
        let own = u16_at(&p, 0x2C) as usize;
        self.bitmap[own >> 5] &= !(1 << (own & 31));
        self.slots[own] = None;
        spatial.remove(u16_at(&p, 0x2E));
        u32::from_le_bytes(p[0x20..0x24].try_into().unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Advance {
    Moved,
    /// Fin de recorrido: el proyectil ya fue retirado. `view_effect` indica que, al ser el combate visible, el
    /// anfitrión debe dibujar el final (salpicadura/impacto en la casilla); `graphic` es el recurso a liberar.
    Finished {
        view_effect: bool,
        graphic: u32,
    },
}

/// 0x00603130. `id` es la ranura de `table`; `direct` es `battle+0xAC8 != 0`; `view_active` indica que este
/// combate es el visible.
pub fn advance<S: Spatial>(
    table: &mut ProjectileTable,
    id: usize,
    direct: bool,
    view_active: bool,
    spatial: &mut S,
) -> Advance {
    let p = table.slots[id].as_mut().expect("proyectil vivo");
    let (vx, vy) = (i32_at(p, 0x0C), i32_at(p, 0x10));
    let horizontal = vx.wrapping_abs() > vy.wrapping_abs();
    let (v, position, dest) = if horizontal {
        (vx, i16_at(p, 0x06), i16_at(p, 0x00))
    } else {
        (vy, i16_at(p, 0x0A), i16_at(p, 0x02))
    };
    let next = (v.wrapping_add((v >> 31) & 0xFFFF) >> 16).wrapping_add(i32::from(position));
    let dest = i32::from(dest);
    if (next > dest && v > 0) || (next < dest && v < 0) {
        let view_effect = view_active;
        let graphic = table.release(id, spatial);
        return Advance::Finished { view_effect, graphic };
    }
    let (dx, dy) = if direct {
        (vx.wrapping_shl(4), vy.wrapping_shl(4))
    } else {
        (vx, vy)
    };
    let x = i32_at(p, 4).wrapping_add(dx);
    let y = i32_at(p, 8).wrapping_add(dy);
    p[4..8].copy_from_slice(&x.to_le_bytes());
    p[8..12].copy_from_slice(&y.to_le_bytes());
    let (xi, yi) = (i16_at(p, 6), i16_at(p, 0x0A));
    let m: i16 = if direct { 0x50 } else { 5 };
    spatial.bounds(
        u16_at(p, 0x2E),
        xi.wrapping_sub(m),
        yi.wrapping_sub(m),
        xi.wrapping_add(m),
        yi.wrapping_add(m),
    );
    Advance::Moved
}

/// Avisos del combate (0x00611649) y efectos de pantalla que el anfitrión debe reproducir.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImpactEvent {
    Notify(u8),
    /// Combate visible: efectos del resultado del daño (0 = daño leve, 1, 2, otros) y escombros.
    View {
        result: u8,
    },
}

/// 0x00603470. `battle` son los bytes escalares del combate; `ship(local)` devuelve `(lado +0x13D, banderas +0x12A)`;
/// `apply` ejecuta 0x0061F96F sobre el barco impactado con `(battle, daño)` (bajas -1) y devuelve AL.
/// Devuelve el recurso gráfico a liberar.
pub fn impact<S: Spatial>(
    table: &mut ProjectileTable,
    id: usize,
    target: u16,
    battle: &mut [u8],
    ship: &dyn Fn(u16) -> (u8, u8),
    spatial: &mut S,
    view_active: bool,
    apply: impl FnOnce(&mut [u8], i32) -> u8,
    events: &mut Vec<ImpactEvent>,
) -> u32 {
    let p = table.slots[id].expect("proyectil vivo");
    let shooter = u16_at(&p, 0x24);
    let side_of = |battle: &[u8], local: u16| battle[0x559 + usize::from(ship(local).0)];
    let notify_if_own = |battle: &[u8], events: &mut Vec<ImpactEvent>, code: u8| {
        if u32::from(battle[0xAD2]) == u32::from(side_of(battle, shooter)) + 1 {
            events.push(ImpactEvent::Notify(code));
        }
    };
    if shooter != 0xFFFF {
        let counter = 0xACA + usize::from(side_of(battle, shooter));
        if battle[counter] < 0x7F {
            battle[counter] += 1;
        }
        if ship(target).1 & 0x20 != 0 {
            battle[counter] = 0xFF;
        }
        notify_if_own(battle, events, 9);
        let count = usize::from(u16::from_le_bytes([battle[0xACC], battle[0xACD]]));
        for slot in table.slots.iter_mut().take(count) {
            if let Some(other) = slot {
                if u16_at(other, 0x24) == shooter {
                    other[0x24..0x26].copy_from_slice(&0xFFFFu16.to_le_bytes());
                }
            }
        }
    }
    let result = apply(battle, i32_at(&p, 0x14));
    if result == 2 && shooter != 0xFFFF {
        notify_if_own(battle, events, 0x0B);
    }
    if view_active {
        events.push(ImpactEvent::View { result });
    }
    table.release(id, spatial)
}

// ---------------------------------------------------------------------------------------------- creación

/// Tabla de arcotangente 0x0067AAE0 (65 entradas).
pub(crate) const ATAN: [u8; 65] = [
    0, 1, 1, 2, 3, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9, 9, 10, 11, 11, 12, 12, 13, 13, 14, 15, 15, 16, 16, 17, 17, 18, 18, 19,
    19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 25, 26, 26, 27, 27, 27, 28, 28, 29, 29, 29, 30, 30, 30, 31, 31,
    31, 32, 32,
];

/// `FILD` + `fsqrt` + `__ftol` de la CRT sobre un entero con signo de 32 bits. Un cuadrado desbordado (negativo)
/// llevaría a la rutina de errores matemáticos de la CRT, que no se modela: sólo ocurre fuera del mapa.
fn int_sqrt(d2: i32) -> Result<i32, &'static str> {
    if d2 < 0 {
        Err("raíz de un cuadrado desbordado (ruta matherr de la CRT no modelada)")
    } else {
        Ok(f64::from(d2).sqrt() as i32)
    }
}

/// División `IDIV` de 32 bits (cociente truncado); `Err` si el divisor es cero o el cociente desborda (#DE).
fn idiv(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 || (a == i32::MIN && b == -1) {
        return Err("excepción de división (#DE)");
    }
    Ok(a / b)
}

/// Dirección del vector (0..255) con el cociente por tabla del original (0x00602BC5..0x00602C83).
pub(crate) fn heading(vx: i32, vy: i32) -> u8 {
    let (mut edi, mut ecx) = (vx, vy);
    while edi > 0x7FFF || edi < -0x7FFF {
        edi >>= 1;
        ecx >>= 1;
    }
    if ecx == 0 {
        return if edi <= 0 { 0xC0 } else { 0x40 };
    }
    let q = ((edi as u32) << 16) as i32 / ecx;
    let dl: u8 = if q < 0 {
        if -q > 0x10000 {
            let eax = 0x4000_0000 / q;
            let index = ((0x80 - eax) >> 8) as usize;
            (0x40u8.wrapping_sub(ATAN[index])).wrapping_neg()
        } else {
            ATAN[((0x200 - q) >> 10) as usize].wrapping_neg()
        }
    } else if q > 0x10000 {
        let index = ((0x4000_0000 / q + 0x80) >> 8) as usize;
        0x40u8.wrapping_sub(ATAN[index])
    } else {
        ATAN[((q + 0x200) >> 10) as usize]
    };
    if ecx <= 0 {
        dl.wrapping_neg()
    } else {
        0x80u8.wrapping_sub(dl)
    }
}

/// Datos de 0x00602A90 (`id`, destino, origen, daño, `kind` +0x18, tirador +0x24 = barco local que dispara y +0x34,
/// una constante 8 que pasa el disparo). El octavo argumento del disparo no se guarda.
#[derive(Clone, Copy, Debug)]
pub struct InitArgs {
    pub id: u16,
    pub dest: (i16, i16),
    pub src: (i16, i16),
    pub damage: i32,
    pub kind: u8,
    pub shooter: u16,
    pub tag: u16,
}

/// 0x00602A90: inicializa un proyectil, lo anota en el mapa de bits y en el índice espacial. `direct` es
/// `battle+0xAC8 != 0`; `rand` es el `rand()` de la CRT (sólo +0x32, decorativo). El efecto de pantalla sale al anfitrión.
pub fn init<S: Spatial, F: FnMut() -> u32>(
    table: &mut ProjectileTable,
    battle_pointer: u32,
    direct: bool,
    spatial: &mut S,
    rand: &mut F,
    a: InitArgs,
) -> Result<Projectile, &'static str> {
    let mut p = [0u8; SIZE];
    let id = a.id as usize;
    p[4..6].fill(0);
    p[8..10].fill(0);
    p[0x18] = a.kind;
    p[0x0A..0x0C].copy_from_slice(&a.src.1.to_le_bytes());
    p[0x34..0x36].copy_from_slice(&a.tag.to_le_bytes());
    p[0x2C..0x2E].copy_from_slice(&a.id.to_le_bytes());
    p[0x14..0x18].copy_from_slice(&a.damage.to_le_bytes());
    p[0x24..0x26].copy_from_slice(&a.shooter.to_le_bytes());
    p[0..2].copy_from_slice(&a.dest.0.to_le_bytes());
    p[2..4].copy_from_slice(&a.dest.1.to_le_bytes());
    p[6..8].copy_from_slice(&a.src.0.to_le_bytes());
    p[0x28..0x2C].copy_from_slice(&battle_pointer.to_le_bytes());
    table.bitmap[id >> 5] |= 1 << (id & 31);
    let vx = i32::from(a.dest.0) - i32::from(a.src.0);
    let vy = i32::from(a.dest.1) - i32::from(a.src.1);
    p[0x0C..0x10].copy_from_slice(&vx.to_le_bytes());
    p[0x10..0x14].copy_from_slice(&vy.to_le_bytes());
    let length = int_sqrt(vy.wrapping_mul(vy).wrapping_add(vx.wrapping_mul(vx)))?;
    let nx = u16::from_le_bytes([p[6], p[7]]).wrapping_add(idiv(vx.wrapping_mul(30), length)? as u16);
    p[6..8].copy_from_slice(&nx.to_le_bytes());
    let ny = u16::from_le_bytes([p[0x0A], p[0x0B]]).wrapping_add(idiv(vy.wrapping_mul(30), length)? as u16);
    p[0x0A..0x0C].copy_from_slice(&ny.to_le_bytes());
    let svx = idiv(vx.wrapping_mul(340_000), length)?;
    p[0x0C..0x10].copy_from_slice(&svx.to_le_bytes());
    let svy = idiv(vy.wrapping_mul(340_000), length)?;
    p[0x10..0x14].copy_from_slice(&svy.to_le_bytes());
    p[0x1C..0x20].copy_from_slice(&length.wrapping_sub(30).to_le_bytes());
    p[0x30] = heading(svx, svy);
    let m: i16 = if direct { 0x50 } else { 5 };
    let (xi, yi) = (nx as i16, ny as i16);
    let slot = spatial
        .add(
            1,
            a.id as u8,
            xi.wrapping_sub(m),
            yi.wrapping_sub(m),
            xi.wrapping_add(m),
            yi.wrapping_add(m),
        )
        .ok_or("índice espacial lleno (ampliación 0x0061A1D0 pendiente)")?;
    p[0x2E..0x30].copy_from_slice(&slot.to_le_bytes());
    p[0x20..0x24].fill(0);
    let r = (rand() & 0x1F) as i32 + 0x60;
    let t = r.wrapping_mul(length.wrapping_sub(30));
    let t = t.wrapping_add(((t >> 31) as u32 & 0x1FF) as i32) >> 9;
    p[0x32..0x34].copy_from_slice(&(t as u16).to_le_bytes());
    let d = a.damage;
    p[0x14..0x18].copy_from_slice(&d.wrapping_add(d >> 1).to_le_bytes());
    Ok(p)
}

impl ProjectileTable {
    /// 0x006111B9: primera ranura libre; si no hay, amplía 32 ranuras y un palabra del mapa de bits
    /// (`battle+0xACC` cuenta ranuras, `battle+0xACE` palabras del mapa).
    pub fn allocate_id(&mut self, battle: &mut [u8]) -> u16 {
        if let Some(free) = self.slots.iter().position(|s| s.is_none()) {
            return free as u16;
        }
        let old = self.slots.len();
        let words = u16::from_le_bytes([battle[0xACE], battle[0xACF]]).wrapping_add(1);
        battle[0xACE..0xAD0].copy_from_slice(&words.to_le_bytes());
        let count = (old as u16).wrapping_add(0x20);
        battle[0xACC..0xACE].copy_from_slice(&count.to_le_bytes());
        self.slots.resize(count as usize, None);
        self.bitmap.resize(words as usize, 0);
        if let Some(last) = self.bitmap.last_mut() {
            *last = 0;
        }
        old as u16
    }
}

/// Datos del barco que dispara que el original lee de tablas globales (barco mundial, capitán, comerciante, dificultad).
#[derive(Clone, Copy, Debug)]
pub struct ShotContext {
    pub ship_x: i16,
    pub ship_y: i16,
    pub local_id: u16,
    pub battle_pointer: u32,
    /// Propietario del barco mundial (+0) y mantenimiento con signo (+0x3F).
    pub owner: u8,
    pub maintenance: i8,
    /// Experiencia de combate del capitán (tabla de capitanes, +0xB) si su identificador es válido.
    pub captain_experience: Option<u8>,
    /// Dificultad del juego si el propietario existe y es humano (ajusta el mantenimiento).
    pub human_difficulty: Option<u8>,
}

#[derive(Clone, Copy, Debug)]
pub struct ShotArgs {
    pub target: (i16, i16),
    pub raw_damage: i32,
    pub index: u8,
    pub count: u8,
    /// Séptimo argumento del disparo: no influye en el proyectil.
    pub unused: u16,
}

/// Cadena de ajuste de daño compartida por el disparo (0x006214CB) y la estimación agregada (0x00620F0B): escala
/// por 28, experiencia del capitán, mantenimiento, y tirada del combate (semilla `battle+0xADC`), mínimo 1.
pub fn adjusted_damage(raw: i32, c: &ShotContext, battle: &mut [u8]) -> i32 {
    let mut damage = ((raw >> 6).wrapping_mul(2800)) / 100;
    if let Some(exp) = c.captain_experience {
        let factor = i32::from(exp) * 6 / 17 + 100;
        damage = damage.wrapping_mul(factor) / 100;
    }
    let mut level = c.maintenance.clamp(0, 4);
    if c.owner != 0xFF {
        if let Some(difficulty) = c.human_difficulty {
            if difficulty == 0 {
                level = level.wrapping_add(2);
            } else if difficulty == 2 {
                level = level.wrapping_sub(2);
            }
        }
    }
    if level < 1 {
        damage = damage.wrapping_add((i32::from(level) - 1).wrapping_mul(damage) / 20);
    } else if level > 2 {
        damage = damage.wrapping_add((i32::from(level) - 2).wrapping_mul(damage) / 20);
    }
    let seed = u32::from_le_bytes(battle[0xADC..0xAE0].try_into().unwrap());
    let seed = crate::ships::wind::next_seed(seed);
    battle[0xADC..0xAE0].copy_from_slice(&seed.to_le_bytes());
    let scaled = (seed % 31 + 85).wrapping_mul(damage as u32) / 100;
    if scaled == 0 {
        1
    } else {
        scaled as i32
    }
}

/// 0x006214CB: calcula el daño de un disparo (capitán, mantenimiento, azar del combate), la separación lateral dentro
/// de la banda y crea el proyectil. Devuelve su identificador. `battle` son los bytes escalares del combate.
pub fn create_shot<S: Spatial, F: FnMut() -> u32>(
    table: &mut ProjectileTable,
    battle: &mut [u8],
    spatial: &mut S,
    rand: &mut F,
    c: &ShotContext,
    a: &ShotArgs,
) -> Result<u16, &'static str> {
    let damage = adjusted_damage(a.raw_damage, c, battle);
    let dy = i32::from(c.ship_y.wrapping_sub(a.target.1));
    let dx = i32::from(a.target.0.wrapping_sub(c.ship_x));
    let length = int_sqrt(dy.wrapping_mul(dy).wrapping_add(dx.wrapping_mul(dx)))?;
    let separation = i32::from(((i16::from(a.index) - i16::from(a.count >> 1)) << 3) as i16);
    let off_x = idiv(separation.wrapping_mul(dy), length)?;
    let off_y = idiv(separation.wrapping_mul(dx), length)?;
    let id = table.allocate_id(battle);
    let direct = battle[0xAC8] != 0;
    let args = InitArgs {
        id,
        dest: (
            off_x.wrapping_add(i32::from(a.target.0)) as i16,
            off_y.wrapping_add(i32::from(a.target.1)) as i16,
        ),
        src: (c.ship_x.wrapping_add(off_x as i16), c.ship_y.wrapping_add(off_y as i16)),
        damage,
        kind: 1,
        shooter: c.local_id,
        tag: 8,
    };
    let projectile = init(table, c.battle_pointer, direct, spatial, rand, args)?;
    table.slots[id as usize] = Some(projectile);
    Ok(id)
}
