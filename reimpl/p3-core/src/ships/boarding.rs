//! Velocidad efectiva (0x00612930), fuerza de artillería (0x0062125B), posibilidad de captura (0x006212FB) y filtro de
//! abordaje de `update_weapons` (0x0061F02F). Fixture: `oracle/x86_battle_boarding.py`.

const TYPE_FACTOR: [u32; 4] = [0x2B5, 0x2B5, 0x242, 0x242];

fn u16_at(b: &[u8], o: usize) -> u32 {
    u32::from(u16::from_le_bytes([b[o], b[o + 1]]))
}
fn i32_at(b: &[u8], o: usize) -> i32 {
    i32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

/// Tabla de capitanes (16 bytes por entrada) y su recuento.
#[derive(Clone, Copy)]
pub struct Captains<'a> {
    pub table: &'a [u8],
    pub count: u16,
}

impl Captains<'_> {
    pub(crate) fn byte(&self, id: u32, offset: usize) -> Option<u8> {
        if id < u32::from(self.count) {
            Some(self.table[(id as usize) * 16 + offset])
        } else {
            None
        }
    }
}

/// 0x00612930: velocidad efectiva del barco mundial (0x180 bytes).
pub fn effective_speed(world: &[u8], captains: Captains) -> Result<u32, &'static str> {
    let (capacity, max_health) = (i32_at(world, 0x10), i32_at(world, 0x14));
    if capacity == 0 || max_health == 0 {
        return Ok(1);
    }
    let cargo = i32_at(world, 0x118);
    let load = cargo.wrapping_mul(614);
    if capacity == -1 && load == i32::MIN {
        return Err("excepción de división (#DE)");
    }
    let hull = 0x1000i32.wrapping_sub(load / capacity);
    let table = TYPE_FACTOR[usize::from(world[0x0E] & 3)];
    let hull = ((hull.wrapping_mul(table as i32)) as u32) >> 12;
    let health = (i32_at(world, 0x18) as u32).wrapping_mul(179);
    let mut factor = (health / (max_health as u32)).wrapping_add(0x71);
    if factor < 0xA6 {
        factor = 0xA6;
    } else if factor > 0x100 {
        factor = 0x100;
    }
    let value = (factor.wrapping_mul(hull)) >> 10;
    match captains.byte(u16_at(world, 0x42), 9) {
        Some(skill) => Ok((u32::from(skill) + 0x9F6).wrapping_mul(value) / 0x9F6),
        None => Ok(value),
    }
}

/// 0x0062125B: fuerza de artillería ponderada por vida, capitán y mantenimiento.
pub fn artillery_strength(world: &[u8], captains: Captains) -> i32 {
    let mut value = i32_at(world, 0x120).wrapping_mul(i32_at(world, 0x18)) >> 10;
    if let Some(experience) = captains.byte(u16_at(world, 0x42), 0x0B) {
        let product = i64::from(i32::from(experience).wrapping_mul(value) as i32);
        let hi = ((product * 0x134679ad) >> 32) as i32;
        let hi = hi >> 6;
        value = value.wrapping_add(hi.wrapping_add(((hi as u32) >> 31) as i32));
    }
    let maintenance = world[0x3F] as i8;
    if maintenance < 1 {
        let hi = ((i64::from(value) * -0x6666_6667) >> 32) as i32 >> 3;
        value.wrapping_add(hi.wrapping_add(((hi as u32) >> 31) as i32))
    } else if maintenance <= 2 {
        value
    } else {
        let level = i32::from(maintenance.min(4)) - 2;
        let product = level.wrapping_mul(value);
        let hi = ((i64::from(product) * 0x6666_6667) >> 32) as i32 >> 3;
        value.wrapping_add(hi.wrapping_add(((hi as u32) >> 31) as i32))
    }
}

fn crew_pair(world: &[u8]) -> u32 {
    // eax = tripulación (+0x40) o su suma con +0x154; el original la usa solo con los 16 bits bajos (`and eax, 0xFFFF`)
    let (a, c) = (u16_at(world, 0x40), u16_at(world, 0x154));
    (if a < c { a + a } else { a + c }) & 0xFFFF
}

/// Lo que el abordaje lee del combate, de los barcos del mundo y de los capitanes.
pub struct Boarding<'a, 'w> {
    pub battle: &'a [u8],
    /// Resultado del método virtual 10 del combate (0x611730 devuelve falso).
    pub virtual_10: bool,
    pub world: &'a dyn Fn(u32) -> &'w [u8],
    pub captains: Captains<'a>,
}

impl Boarding<'_, '_> {
    fn side_map(&self, ship: &[u8]) -> u8 {
        self.battle[0x559 + usize::from(ship[0x13D])]
    }
    fn participant(&self, ship: &[u8]) -> u16 {
        let at = 0x35A + 2 * usize::from(ship[0x13D]);
        u16::from_le_bytes([self.battle[at], self.battle[at + 1]])
    }
    fn aggressive(&self, ship: &[u8]) -> bool {
        self.battle[0x5D + usize::from(ship[0x13D])] != 0
    }
    fn world_of(&self, ship: &[u8]) -> &[u8] {
        (self.world)(u32::from_le_bytes(ship[4..8].try_into().unwrap()))
    }

    fn open_target(&self, cand: &[u8]) -> bool {
        self.side_map(cand) == 0 || self.virtual_10 || self.participant(cand) == 0xFF
    }

    /// 0x006212FB: ¿puede `this` capturar a `cand`? `flag` es el segundo argumento.
    pub fn can_capture(&self, this: &[u8], cand: &[u8], flag: bool) -> bool {
        if flag && self.open_target(cand) {
            return true;
        }
        if !self.aggressive(this) {
            return this[0x12A] >> 7 != 0;
        }
        let own = crew_pair(self.world_of(this));
        let theirs = 2 * u16_at(self.world_of(cand), 0x40);
        if own < theirs {
            return false;
        }
        if self.open_target(cand) {
            return true;
        }
        if cand[0x12A] & 0x40 != 0 || self.participant(this) != 0xFF {
            return false;
        }
        let side = usize::from(self.side_map(cand));
        let sum = u32::from(u16::from_le_bytes([cand[0x138], cand[0x139]]))
            .wrapping_add(u32::from(u16::from_le_bytes([this[0x138], this[0x139]])))
            .wrapping_add(u32::from_le_bytes(cand[4..8].try_into().unwrap()))
            .wrapping_add(u32::from_le_bytes(this[4..8].try_into().unwrap()));
        match self.battle[0xACA + side] {
            0 | 1 => false,
            2 => sum % 11 < 3,
            3 => sum % 11 < 6,
            _ => true,
        }
    }

    /// 0x0061F02F: filtro de objetivo de abordaje de `update_weapons`.
    pub fn board_filter(&self, this: &[u8], cand: &[u8], flag: bool) -> Result<bool, &'static str> {
        if this[0x12A] & 0x20 != 0 {
            return Ok(false);
        }
        if !self.aggressive(this) {
            let selected = u16::from_le_bytes([this[0x128], this[0x129]]);
            let id = u16::from_le_bytes([cand[0x138], cand[0x139]]);
            return Ok(if id == selected || selected == 0xFFFF {
                this[0x13E] != 0
            } else {
                false
            });
        }
        let (mine, theirs) = (self.world_of(this), self.world_of(cand));
        if !self.can_capture(this, cand, false) {
            if theirs[0x136] & 0x40 != 0 {
                return Ok(false);
            }
            let free = i32_at(mine, 0x10)
                .wrapping_sub(i32_at(mine, 0x11C))
                .wrapping_sub(i32_at(mine, 0x118));
            if free < 0x7D0 {
                return Ok(false);
            }
            if cand[0x12A] & 0x20 != 0 {
                return Ok(true);
            }
        }
        let own = crew_pair(mine);
        let their_crew = 2 * u16_at(theirs, 0x40) as i32;
        if !flag || cand[0x12A] & 0x20 != 0 {
            return Ok(own as i32 > their_crew);
        }
        if own as i32 <= their_crew {
            return Ok(false);
        }
        let cand_speed = effective_speed(theirs, self.captains)?;
        let own_speed = effective_speed(mine, self.captains)?;
        if own_speed.wrapping_mul(19) <= cand_speed.wrapping_mul(20) {
            return Ok(false);
        }
        Ok(cand[0x12A] & 8 == 0)
    }
}

/// Efecto del combate cuerpo a cuerpo que decide quién queda capturado (0x0060C1C9): se registra con sus dos argumentos
/// de pila en el orden en que los lee la función.
pub trait BoardingHost {
    fn capture(&mut self, battle: &mut [u8], first: u16, second: u16);
    /// Variante con los dos barcos mutables para ejecutar el efecto original
    /// en el mismo punto del abordaje. Los anfitriones de prueba pueden seguir
    /// registrando únicamente la llamada.
    fn capture_pair(
        &mut self,
        battle: &mut [u8],
        first: u16,
        second: u16,
        _first_ship: &mut [u8],
        _second_ship: &mut [u8],
        _first_world: &mut [u8],
        _second_world: &mut [u8],
    ) {
        self.capture(battle, first, second);
    }
}

/// Datos que 0x0061F1BF lee y modifica. `world_*` son los barcos del mundo (0x180 bytes) de cada bando, distintos entre sí.
pub struct BoardingStep<'a> {
    pub battle: &'a mut [u8],
    pub this: &'a mut [u8],
    pub target: &'a mut [u8],
    pub world_this: &'a mut [u8],
    pub world_target: &'a mut [u8],
    pub captains: Captains<'a>,
    /// Comerciante humano (`word [+8] == 0`) del propietario del barco mundial.
    pub human_owner: &'a dyn Fn(u8) -> bool,
    /// Dificultad (`settings+0x12`).
    pub difficulty: u8,
    pub virtual_10: bool,
    /// La batalla está en pantalla: el original crea un efecto gráfico (rama no modelada).
    pub view_active: bool,
}

struct Side {
    crew: u32,
    pairs: u32,
    strength: u32,
}

fn side_strength(world: &[u8], captains: Captains, human: &dyn Fn(u8) -> bool, difficulty: u8) -> Side {
    let captain = captains.byte(u16_at(world, 0x42), 0x0B);
    let crew = u16_at(world, 0x40) + u32::from(captain.is_some());
    let extra = u16_at(world, 0x154);
    let pairs = if extra < crew { extra + crew } else { crew + crew };
    let mut level = (world[0x3F] as i8).clamp(0, 4);
    if world[0] != 0xFF && human(world[0]) {
        if difficulty == 0 {
            level += 2;
        } else if difficulty == 2 {
            level -= 2;
        }
    }
    let ecx = ((i32::from(level) + 0x12) as u32).wrapping_mul(pairs).wrapping_add(0xA);
    let mut strength = ((u64::from(ecx) * 0xCCCC_CCCD) >> 32) as u32 >> 4;
    if let Some(experience) = captain {
        let value = i64::from(6 * u32::from(experience));
        let hi = ((value * 0x7878_7879) >> 32) as i32 >> 3;
        let factor = hi.wrapping_add(((hi as u32) >> 31) as i32).wrapping_add(0x64) as u32;
        strength = ((u64::from(factor.wrapping_mul(strength)) * 0x51EB_851F) >> 32) as u32 >> 5;
    }
    Side { crew, pairs, strength }
}

fn damage_roll(random: u32, ratio_num: u32, ratio_den: u32, own_crew: u32, enemy_strength: u32, direct: bool) -> u32 {
    let base = (random % 26 + 0x5C)
        .wrapping_mul(
            (ratio_num.wrapping_mul(100) / ratio_den)
                .wrapping_add(own_crew.wrapping_add(enemy_strength).wrapping_mul(3)),
        )
        .wrapping_mul(40);
    let mut value = ((u64::from(base) * 0x1062_4DD3) >> 32) as u32 >> 6;
    if direct {
        value <<= 4;
    }
    value
}

fn next_random(battle: &mut [u8]) -> u32 {
    let mut seed = u32::from_le_bytes(battle[0xADC..0xAE0].try_into().unwrap());
    let value = crate::rng::sea_battle_next(&mut seed);
    battle[0xADC..0xAE0].copy_from_slice(&seed.to_le_bytes());
    value
}

fn set_u16(bytes: &mut [u8], at: usize, value: u16) {
    bytes[at..at + 2].copy_from_slice(&value.to_le_bytes());
}

fn kill_extras(battle: &mut [u8], world: &mut [u8], losses: u32, crew: u32) {
    for _ in 0..losses {
        let mut extra = u16_at(world, 0x154);
        if extra > crew {
            extra = crew;
        }
        if next_random(battle) % (2 * crew) < extra {
            let again = (u16_at(world, 0x154) as u16).wrapping_sub(1);
            set_u16(world, 0x154, again);
        }
    }
}

/// 0x0061F1BF: un paso de combate cuerpo a cuerpo entre el barco y su objetivo.
pub fn boarding_step<H: BoardingHost>(s: BoardingStep, host: &mut H) -> Result<(), &'static str> {
    let BoardingStep {
        battle,
        this,
        target,
        world_this,
        world_target,
        captains,
        human_owner,
        difficulty,
        virtual_10,
        view_active,
    } = s;
    if this[0x14A] == 0 && view_active {
        return Err("rama gráfica de 0x0061F1BF no modelada");
    }
    if this[0x26] > 2 || target[0x26] > 2 {
        return Ok(());
    }
    let (this_id, selected) = (u16_at(this, 0x138) as u16, u16_at(this, 0x128) as u16);
    let this_world = u32::from_le_bytes(this[4..8].try_into().unwrap());
    let target_world = u32::from_le_bytes(target[4..8].try_into().unwrap());
    let capture = |battle: &[u8], world_this: &[u8], world_target: &[u8], who: &[u8], other: &[u8]| {
        can_capture_pair(
            battle,
            virtual_10,
            captains,
            (this_world, world_this),
            (target_world, world_target),
            who,
            other,
        )
    };
    let side_of = |battle: &[u8], ship: &[u8]| usize::from(battle[0x559 + usize::from(ship[0x13D])]);
    if this[0x12A] & 0x20 != 0 {
        if !capture(battle, world_this, world_target, target, this) {
            host.capture_pair(battle, selected, this_id, target, this, world_target, world_this);
            return Ok(());
        }
        let side = side_of(battle, this);
        battle[0xACB - side] = 0xFF;
    }
    if target[0x12A] & 0x20 != 0 {
        if !capture(battle, world_this, world_target, this, target) {
            host.capture_pair(battle, this_id, selected, this, target, world_this, world_target);
            return Ok(());
        }
        let side = side_of(battle, this);
        battle[0xACA + side] = 0xFF;
    }
    let mine = side_strength(world_this, captains, human_owner, difficulty);
    let theirs = side_strength(world_target, captains, human_owner, difficulty);
    let (strength_mine, strength_theirs) = (mine.strength.max(1), theirs.strength.max(1));
    let direct = battle[0xAC8] != 0;
    let random = next_random(battle);
    let damage_to_target = damage_roll(
        random,
        strength_mine,
        strength_theirs,
        theirs.crew,
        strength_mine,
        direct,
    );
    let total = damage_to_target.wrapping_add(u32::from(u16_at(target, 0x13A) as u16));
    set_u16(target, 0x13A, total as u16);
    let mut losses_target = total >> 16;
    let random = next_random(battle);
    let damage_to_this = damage_roll(
        random,
        strength_theirs,
        strength_mine,
        mine.crew,
        strength_theirs,
        direct,
    );
    let total = damage_to_this.wrapping_add(u32::from(u16_at(this, 0x13A) as u16));
    set_u16(this, 0x13A, total as u16);
    let mut losses_this = total >> 16;
    if losses_target != 0 {
        losses_target = losses_target.min(theirs.crew);
        if losses_target > 0 {
            kill_extras(battle, world_target, losses_target, theirs.crew);
        }
        if losses_target == theirs.crew {
            set_u16(world_target, 0x40, 1);
            host.capture_pair(battle, this_id, selected, this, target, world_this, world_target);
            return Ok(());
        }
    }
    if losses_this != 0 {
        losses_this = losses_this.min(mine.crew);
        if losses_this > 0 {
            kill_extras(battle, world_this, losses_this, mine.crew);
        }
        if losses_this == mine.crew {
            set_u16(world_this, 0x40, 1);
            host.capture_pair(battle, selected, this_id, target, this, world_target, world_this);
            return Ok(());
        }
        if losses_this != 0 {
            world_this[0x3C] |= 2;
        }
    }
    if losses_target != 0 {
        world_target[0x3C] |= 2;
    }
    let crew_this = (u16_at(world_this, 0x40) as u16).wrapping_sub(losses_this as u16);
    set_u16(world_this, 0x40, crew_this);
    let crew_target = (u16_at(world_target, 0x40) as u16).wrapping_sub(losses_target as u16);
    set_u16(world_target, 0x40, crew_target);
    let side = usize::from(this[0x13D]);
    if battle[0x5D + side] != 0 && 3 * strength_mine < 2 * theirs.crew && participant_word(battle, side) != 0xFF {
        this[0x12A] |= 0x20;
        return Ok(());
    }
    let side = usize::from(target[0x13D]);
    if battle[0x5D + side] != 0 && 3 * strength_theirs < 2 * mine.crew && participant_word(battle, side) != 0xFF {
        target[0x12A] |= 0x20;
    }
    Ok(())
}

fn can_capture_pair<'a>(
    battle: &[u8],
    virtual_10: bool,
    captains: Captains,
    first: (u32, &'a [u8]),
    second: (u32, &'a [u8]),
    who: &[u8],
    other: &[u8],
) -> bool {
    let lookup = move |index: u32| -> &'a [u8] {
        if index == first.0 {
            first.1
        } else {
            second.1
        }
    };
    let view = Boarding {
        battle,
        virtual_10,
        world: &lookup,
        captains,
    };
    view.can_capture(who, other, true)
}

fn participant_word(battle: &[u8], side: usize) -> u16 {
    u16::from_le_bytes([battle[0x35A + 2 * side], battle[0x35B + 2 * side]])
}
