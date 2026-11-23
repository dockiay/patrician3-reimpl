//! Núcleo del intérprete de guiones de misión `0x004ECF64`.
//!
//! Solo se ejecutan comandos portados. Un comando desconocido, un salto o
//! variable inválidos y un bucle excesivo se rechazan sin modificar el estado.
use crate::scheduler::TaskRecord;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Waiting,
    Finished,
}

/// Valores externos que usa el selector determinista de ciudad `0x21`.
#[derive(Clone, Copy, Debug)]
pub struct TownContext {
    pub scheduler_head: u16,
    pub town_count: u32,
    pub adjacent_dword: u32,
}
/// Estado que utiliza `0x09`; la posición está ausente si no hay barcos en mar.
#[derive(Clone, Copy, Debug)]
pub struct RandomContext {
    pub scheduler_head: u16,
    pub sea_position: Option<(u32, u32)>,
}
/// Generador de nombres compartido con la fábrica naval.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NameContext {
    pub cursor: u16,
    pub step: u16,
    pub count: u16,
    pub scheduler_head: u16,
    pub surname_count: u16,
    pub surname_seed: Option<(u32, u32)>,
    pub first_counts: Option<(u16, u16)>,
}
/// Efectos sobre el depósito, en el mismo orden que los comandos del guion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LetterEffect {
    Deliver(crate::mission_letter::ScriptLetter),
    RemoveTreasure { merchant: u32, entry: u32 },
    Notify { args: [u32; 6], recipient: u32 },
    /// `0xF8` (`0x004ED20D`): `Merchant+0 += amount` y, según el signo, `+0x4B8 += amount` o `+0x4BC -= amount`.
    Gold { merchant: u32, amount: u32 },
    /// `0xE7`/`0xE9`/`0xF9` (`0x004ED125`): operación 0x82 (barco pirata de misión, `0x0053FB00`) que el original
    /// publica con `0x0054AA70` sobre `OPS` (0x006DF2F0) solo si `OPS+0x2C` (0x006DF31C, cliente de red) es 0;
    /// el anfitrión la descarta en cliente de red. Registro de [`mission_pirate_record`].
    MissionPirate([u32; 5]),
}

/// Registro de 0x14 bytes de la operación 0x82 que arman `0xE7`, `0xE9` y `0xF9` (`0x004ED125`) con los bytes
/// `cmd[1..5]` del comando, las variables del guion y la cabeza del planificador (`0x006DD744`):
/// `+4` barco de referencia (`vars[b]`, o 0xFFFF en `0xE9`), `+6/+7` bytes 1 y 0 de `vars[a]` (capitán),
/// `+8` tarea de guion, `+0xA` variable de resultado `d`, `+0xB` modo (`vars[c]`), `+0xC` ciudad de patrulla
/// (`vars[b]` en `0xE9`, 0xFF en los otros) y `+0xD` = 1 solo en `0xE7` (0x0053FB00 no lo lee).
pub fn mission_pirate_record(op: u8, cmd: [u8; 4], vars: &[u32], scheduler_head: u16) -> Result<[u32; 5], String> {
    let [a, b, c, d] = cmd;
    let (va, vb, vc) = (var(vars, a)?, var(vars, b)?, var(vars, c)?);
    let e9 = op == 0xe9;
    let mut r = [0u8; 0x14];
    r[0] = 0x82;
    r[4..6].copy_from_slice(&(if e9 { 0xffff } else { vb as u16 }).to_le_bytes());
    r[6] = (va >> 8) as u8;
    r[7] = va as u8;
    r[8..10].copy_from_slice(&scheduler_head.to_le_bytes());
    r[0xa] = d;
    r[0xb] = vc as u8;
    r[0xc] = if e9 { vb as u8 } else { 0xff };
    r[0xd] = u8::from(op == 0xe7);
    Ok(std::array::from_fn(|i| u32::from_le_bytes(r[i * 4..i * 4 + 4].try_into().expect("tamaño fijo"))))
}
/// Recursos de `0xF7` y efectos de cartas pendientes de publicación.
pub struct LetterContext<'a> {
    pub scheduler_head: u16,
    pub resources_available: bool,
    pub ship_names: &'a [Vec<u8>],
    pub town_names: &'a [Vec<u8>],
    pub family_names: &'a [Vec<u8>],
    pub first_names: &'a [Vec<Vec<u8>>; 2],
    pub effects: Vec<LetterEffect>,
    pub merchant_images: &'a [crate::ai::reputation::Merchant],
    pub gender_words: Option<&'a [[Vec<u8>; 2]; 8]>,
    pub format_extras: Option<&'a dyn crate::mission_format::FormatExtrasSource>,
}
/// Matriz viva de distancias que usa el selector de ruta `0x29`.
#[derive(Clone, Copy)]
pub struct RouteContext<'a> {
    pub town_count: usize,
    pub distances: &'a [u8],
}
/// Alta diferida de cosecha que el anfitrión inserta junto a la tarea actual.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HarvestSpawn {
    pub city: u16,
    pub duration: u32,
}
/// Datos leídos por `0x04`: población por ciudad y marca de control por comerciante.
#[derive(Clone, Copy)]
pub struct HumanContext<'a> {
    pub populations: &'a [i32],
    pub merchant_flags: &'a [u16],
    pub merchants: Option<&'a [crate::ai::reputation::Merchant]>,
    /// Registros del planificador (`0x006DD73C`) que lee el comando 0x16.
    pub tasks: Option<&'a [TaskRecord]>,
    /// `town_get_wall_expansion_level` (0x005252F0) de cada ciudad, que lee el comando 0x6B.
    pub wall_levels: Option<&'a [u8]>,
    /// Los 40 nombres del banco de ciudades (`0x006DDAA0`), que consulta el comando 0x65.
    pub town_name_bank: Option<&'a [Vec<u8>]>,
    /// Nombres de las ciudades de la partida (`0x006DDA00`), en orden de ciudad.
    pub game_town_names: Option<&'a [Vec<u8>]>,
}

/// Comerciante `index` del guion (`get_merchant`, `0x005303C0`): el original no comprueba el índice.
fn script_merchant<'a>(env: &'a HumanContext<'_>, index: u32) -> Result<&'a crate::ai::reputation::Merchant, String> {
    let merchants = env.merchants.ok_or("comerciantes del guion ausentes")?;
    merchants.get(index as usize).filter(|m| m.bytes.len() >= 0x3c4).ok_or_else(|| format!("comerciante {index} del guion fuera de la tabla"))
}
/// Listas urbanas usadas por la consulta de oficina `0x38`.
#[derive(Clone, Copy)]
pub struct OfficeContext<'a> {
    pub town_heads: &'a [u16],
    pub owners: &'a [u16],
    pub next: &'a [u16],
    pub merchant_count: u16,
    pub office_count: u16,
    pub merchant_chains: Option<MerchantOfficeContext<'a>>,
}
/// Cadena por propietario (distinta de la cadena urbana de oficinas).
#[derive(Clone, Copy)]
pub struct MerchantOfficeContext<'a> {
    pub heads: &'a [u16],
    pub next: &'a [u16],
    pub towns: &'a [u8],
}

/// `0x004F15AC`: selecciona `tick % longitud`, recorriendo `merchant+0xC`
/// y `office+0x2C8`. El original divide por cero para una cadena vacía.
pub fn random_office_town(
    context: &MerchantOfficeContext<'_>,
    merchant: u32,
    count: u16,
    tick: u32,
) -> Result<u32, String> {
    let mut at = *context
        .heads
        .get(merchant as usize)
        .ok_or("comerciante de oficinas inválido")?;
    let mut chain = Vec::new();
    let mut seen = vec![false; usize::from(count)];
    while at < count {
        let i = usize::from(at);
        if seen[i] {
            return Err("cadena de oficinas del comerciante cíclica".into());
        }
        seen[i] = true;
        chain.push(*context.towns.get(i).ok_or("ciudad de oficina ausente")?);
        at = *context.next.get(i).ok_or("enlace de oficina del comerciante ausente")?;
    }
    let selected = tick
        .checked_rem(chain.len() as u32)
        .ok_or("selección de oficina con divisor cero")?;
    Ok(u32::from(chain[selected as usize]))
}
/// Flota y cabeceras de comerciantes que usa el comando 0x40.
#[derive(Clone, Copy)]
pub struct ShipContext<'a> {
    pub merchant_heads: &'a [u16],
    pub ships: &'a [u8],
    /// `Merchant+8` de cada comerciante (0 = humano), que filtran los comandos 0x26 y 0x27.
    pub merchant_flags: &'a [u16],
    /// Destino de las órdenes de reserva (0x3A); el llamante las aplica con `apply_ship_reserve` si el guion no falla.
    pub reserve: Option<&'a std::cell::RefCell<Vec<ShipReserveOp>>>,
}

/// Orden `0x3A`: sube (`raise`, `0x0050DF40`) o baja (`0x0050DF70`) la reserva `ship+0x137` del barco.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ShipReserveOp {
    pub ship: u16,
    pub raise: bool,
}

/// `0x0050DF40` / `0x0050DF70` sobre el gestor de barcos (`manager`, `0x006DD7A0`: libres en `+0xE2`, lista de
/// reserva en `+0xE6`, cuenta en `+0xF4`) y la tabla `ships` (0x180 bytes por barco, enlace en `+6`). Al bajar, los
/// barcos de la lista `+0xE6` con reserva 0 pasan a la lista `+0xE2`.
pub fn apply_ship_reserve(manager: &mut [u8], ships: &mut [u8], op: ShipReserveOp) -> Result<(), String> {
    if manager.len() < 0xf6 || ships.len() % 0x180 != 0 {
        return Err("gestor o flota de barcos inválidos".into());
    }
    let word = |m: &[u8], o: usize| u16::from_le_bytes([m[o], m[o + 1]]);
    let count = word(manager, 0xf4);
    if usize::from(count) * 0x180 > ships.len() {
        return Err("cuenta de barcos mayor que la tabla".into());
    }
    let at = usize::from(op.ship) * 0x180 + 0x137;
    if op.raise {
        if op.ship < count && ships[at] < 0xfa {
            ships[at] += 1
        }
        return Ok(());
    }
    if op.ship < count && ships[at] != 0 {
        ships[at] -= 1
    }
    // `esi` apunta a la celda que enlaza al barco actual: primero `manager+0xE6`, luego `ship+6`.
    let mut cell: Option<usize> = None;
    for _ in 0..=usize::from(u16::MAX) * 2 {
        let current = match cell {
            None => word(manager, 0xe6),
            Some(o) => word(ships, o),
        };
        if current >= count {
            return Ok(());
        }
        let base = usize::from(current) * 0x180;
        if ships[base + 0x137] == 0 {
            let next = word(ships, base + 6);
            match cell {
                None => manager[0xe6..0xe8].copy_from_slice(&next.to_le_bytes()),
                Some(o) => ships[o..o + 2].copy_from_slice(&next.to_le_bytes()),
            }
            let free = word(manager, 0xe2);
            ships[base + 6..base + 8].copy_from_slice(&free.to_le_bytes());
            manager[0xe2..0xe4].copy_from_slice(&current.to_le_bytes());
        } else {
            cell = Some(base + 6)
        }
    }
    Err("lista de reserva cíclica".into())
}

/// Comandos 0x26 y 0x27 (`0x004EF875`): de cada comerciante humano del rango (`merchant` o todos si vale -1), el
/// primer barco de su cadena (`merchant+0xE`, enlaces `ship+4`) con estado `ship+0x134 == state` y carga no nula de
/// la mercancía `ware` (`ship+0x54 + 4·ware`). Elige el candidato de índice `Σ ship+0x1E (u16) mod n`; -1 sin
/// candidatos.
pub fn pick_loaded_ship(env: &ShipContext<'_>, merchant: u32, ware: u32, state: u16) -> Result<u32, String> {
    if env.ships.len() % 0x180 != 0 || env.ships.len() / 0x180 > usize::from(u16::MAX) || env.merchant_flags.len() != env.merchant_heads.len() {
        return Err("flota del guion inválida".into());
    }
    let ship_count = env.ships.len() / 0x180;
    let count = env.merchant_heads.len();
    let range = if merchant == u32::MAX {
        0..count
    } else if (merchant as usize) < count {
        merchant as usize..merchant as usize + 1
    } else {
        return Err("comerciante naval del guion inválido".into());
    };
    let cargo = 0x54usize.checked_add(4 * ware as usize).filter(|at| at + 4 <= 0x180).ok_or("mercancía del guion fuera del barco")?;
    let mut candidates = Vec::new();
    let mut sum = 0u16;
    for k in range {
        if env.merchant_flags[k] != 0 {
            continue;
        }
        let mut seen = vec![false; ship_count];
        let mut id = usize::from(env.merchant_heads[k]);
        while id < ship_count {
            if std::mem::replace(&mut seen[id], true) {
                return Err("cadena naval del guion cíclica".into());
            }
            let s = &env.ships[id * 0x180..(id + 1) * 0x180];
            let u16_at = |at: usize| u16::from_le_bytes([s[at], s[at + 1]]);
            if u16_at(0x134) == state && s[cargo..cargo + 4] != [0; 4] {
                candidates.push(id as u32);
                sum = sum.wrapping_add(u16_at(0x1e));
                break;
            }
            id = usize::from(u16_at(4));
        }
    }
    if candidates.is_empty() {
        return Ok(u32::MAX);
    }
    // El contador del original es un byte: más de 255 candidatos lo desbordarían.
    if candidates.len() > 0xff {
        return Err("demasiados barcos candidatos para el guion".into());
    }
    Ok(candidates[usize::from(sum) % candidates.len()])
}

/// Busca por la cadena `merchant+0xe`, con enlaces `ship+4`, como el comando 0x40.
pub fn find_ship_by_health(head: u16, ships: &[u8], sense: u32, threshold: u32, mode: u32) -> Result<u32, String> {
    if ships.len() % 0x180 != 0 || ships.len() / 0x180 > usize::from(u16::MAX) {
        return Err("flota del guion inválida".into());
    }
    let count = ships.len() / 0x180;
    let mut seen = vec![false; count];
    let mut id = usize::from(head);
    while id < count {
        if seen[id] {
            return Err("cadena naval del guion cíclica".into());
        }
        seen[id] = true;
        let s = &ships[id * 0x180..(id + 1) * 0x180];
        let maximum = i32::from_le_bytes(s[0x14..0x18].try_into().unwrap());
        let hull = i32::from_le_bytes(s[0x18..0x1c].try_into().unwrap());
        let percent = hull
            .wrapping_mul(100)
            .checked_div(maximum)
            .ok_or("división de casco inválida")?;
        let matches_health = if sense != 0 {
            percent >= threshold as i32
        } else {
            percent < (threshold as i32)
        };
        let state = u16::from_le_bytes([s[0x134], s[0x135]]);
        let mode = mode as i32;
        let matches_state = if mode >= 2 {
            state == 15 || state < 4
        } else if mode == 0 {
            state == 15
        } else if mode == 1 {
            state < 4
        } else {
            false
        };
        if matches_health && matches_state {
            return Ok(id as u32);
        }
        id = usize::from(u16::from_le_bytes([s[4], s[5]]));
    }
    Ok(u32::MAX)
}

fn u16_at(data: &[u8], at: usize) -> Result<u16, String> {
    let s = data.get(at..at + 2).ok_or("guion truncado")?;
    Ok(u16::from_le_bytes([s[0], s[1]]))
}
fn u32_at(data: &[u8], at: usize) -> Result<u32, String> {
    let s = data.get(at..at + 4).ok_or("guion truncado")?;
    Ok(u32::from_le_bytes(s.try_into().expect("longitud comprobada")))
}
fn var(vars: &[u32], i: u8) -> Result<u32, String> {
    vars.get(usize::from(i))
        .copied()
        .ok_or_else(|| format!("variable {i} fuera de rango"))
}
fn set_var(vars: &mut [u32], i: u8, v: u32) -> Result<(), String> {
    *vars
        .get_mut(usize::from(i))
        .ok_or_else(|| format!("variable {i} fuera de rango"))? = v;
    Ok(())
}
fn set_pc(task: &mut TaskRecord, pc: u16) {
    let b = pc.to_le_bytes();
    task.set_u8_at(0x14, b[0]);
    task.set_u8_at(0x15, b[1]);
}

/// Ejecuta hasta una espera o terminación. El `tick` y los datos del blob se
/// suministran explícitamente; no interpreta punteros del guardado.
pub fn execute(blob: &[u8], task: &mut TaskRecord, vars: &mut [u32], tick: u32) -> Result<Outcome, String> {
    execute_with_context(blob, task, vars, tick, None)
}

pub fn execute_with_context(
    blob: &[u8],
    task: &mut TaskRecord,
    vars: &mut [u32],
    tick: u32,
    context: Option<TownContext>,
) -> Result<Outcome, String> {
    execute_in_world(blob, task, vars, tick, context, None, None, None, None, None, None)
}

pub fn execute_in_world(
    blob: &[u8],
    task: &mut TaskRecord,
    vars: &mut [u32],
    tick: u32,
    context: Option<TownContext>,
    humans: Option<HumanContext<'_>>,
    offices: Option<OfficeContext<'_>>,
    random: Option<RandomContext>,
    mut names: Option<&mut NameContext>,
    mut letters: Option<&mut LetterContext<'_>>,
    route: Option<RouteContext<'_>>,
) -> Result<Outcome, String> {
    execute_in_world_spawning(
        blob,
        task,
        vars,
        tick,
        context,
        humans,
        offices,
        random,
        names.as_deref_mut(),
        letters.as_deref_mut(),
        route,
        None,
    )
}

pub fn execute_in_world_spawning(
    blob: &[u8],
    task: &mut TaskRecord,
    vars: &mut [u32],
    tick: u32,
    context: Option<TownContext>,
    humans: Option<HumanContext<'_>>,
    offices: Option<OfficeContext<'_>>,
    random: Option<RandomContext>,
    mut names: Option<&mut NameContext>,
    mut letters: Option<&mut LetterContext<'_>>,
    route: Option<RouteContext<'_>>,
    mut harvest: Option<&mut Vec<HarvestSpawn>>,
) -> Result<Outcome, String> {
    execute_in_world_with_ships(
        blob,
        task,
        vars,
        tick,
        context,
        humans,
        offices,
        random,
        names.as_deref_mut(),
        letters.as_deref_mut(),
        route,
        harvest.as_deref_mut(),
        None,
    )
}

/// Textos (título, cuerpo) de la ventana de suceso del comando `0x37` (`0x004F0E94`) en el índice `index`: dos
/// desplazamientos u32 desde el inicio del guion a cadenas terminadas en cero, en el orden en que el original las
/// pasa a `0x469AF0` (primero `+1`, luego `+5`).
pub fn event_window_texts(blob: &[u8], index: u16) -> Result<(Vec<u8>, Vec<u8>), String> {
    let count = usize::from(u16_at(blob, 0)?);
    if usize::from(index) >= count {
        return Err(format!("comando {index} fuera de rango"));
    }
    let off = usize::try_from(u32_at(blob, 4 + 4 * usize::from(index))?).map_err(|_| "desplazamiento inválido")?;
    let cmd = blob.get(off..off + 9).ok_or("comando 0x37 truncado")?;
    if cmd[0] != 0x37 {
        return Err(format!("el comando {index} no es 0x37"));
    }
    let text = |at: usize| -> Result<Vec<u8>, String> {
        let start = usize::try_from(u32::from_le_bytes(cmd[at..at + 4].try_into().unwrap())).map_err(|_| "desplazamiento inválido")?;
        let tail = blob.get(start..).ok_or("texto de 0x37 fuera del guion")?;
        let end = tail.iter().position(|&b| b == 0).ok_or("texto de 0x37 sin terminar")?;
        Ok(tail[..end].to_vec())
    };
    Ok((text(1)?, text(5)?))
}

pub fn execute_in_world_with_ships(
    blob: &[u8],
    task: &mut TaskRecord,
    vars: &mut [u32],
    tick: u32,
    context: Option<TownContext>,
    humans: Option<HumanContext<'_>>,
    offices: Option<OfficeContext<'_>>,
    random: Option<RandomContext>,
    names: Option<&mut NameContext>,
    letters: Option<&mut LetterContext<'_>>,
    route: Option<RouteContext<'_>>,
    harvest: Option<&mut Vec<HarvestSpawn>>,
    ships: Option<ShipContext<'_>>,
) -> Result<Outcome, String> {
    execute_in_world_flat(blob, task, vars, tick, context, humans, offices, random, names, letters, route, harvest, ships, None)
}

/// Como [`execute_in_world_with_ships`], con los comandos de `mission_script_flat` sobre la memoria plana del
/// mundo. Si el guion falla, las escrituras en la memoria se deshacen; los efectos que el anfitrión haya
/// guardado en sus llamadas deben descartarse también.
#[allow(clippy::too_many_arguments)]
pub fn execute_in_world_flat(
    blob: &[u8],
    task: &mut TaskRecord,
    vars: &mut [u32],
    tick: u32,
    context: Option<TownContext>,
    humans: Option<HumanContext<'_>>,
    offices: Option<OfficeContext<'_>>,
    random: Option<RandomContext>,
    names: Option<&mut NameContext>,
    letters: Option<&mut LetterContext<'_>>,
    route: Option<RouteContext<'_>>,
    harvest: Option<&mut Vec<HarvestSpawn>>,
    ships: Option<ShipContext<'_>>,
    flat: Option<&mut crate::mission_script_flat::FlatScript<'_>>,
) -> Result<Outcome, String> {
    let Some(f) = flat else {
        return execute_body(blob, task, vars, tick, context, humans, offices, random, names, letters, route, harvest, ships, None);
    };
    let mut journal = crate::mission_script_flat::Journal::new(&mut *f.mem);
    let mut inner = crate::mission_script_flat::FlatScript { mem: &mut journal, calls: &mut *f.calls, task_word_10: f.task_word_10 };
    let out = execute_body(blob, task, vars, tick, context, humans, offices, random, names, letters, route, harvest, ships, Some(&mut inner));
    if out.is_err() {
        journal.undo();
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn execute_body(
    blob: &[u8],
    task: &mut TaskRecord,
    vars: &mut [u32],
    tick: u32,
    context: Option<TownContext>,
    humans: Option<HumanContext<'_>>,
    offices: Option<OfficeContext<'_>>,
    random: Option<RandomContext>,
    mut names: Option<&mut NameContext>,
    mut letters: Option<&mut LetterContext<'_>>,
    route: Option<RouteContext<'_>>,
    mut harvest: Option<&mut Vec<HarvestSpawn>>,
    ships: Option<ShipContext<'_>>,
    mut flat: Option<&mut crate::mission_script_flat::FlatScript<'_>>,
) -> Result<Outcome, String> {
    let count = usize::from(u16_at(blob, 0)?);
    let var_count = usize::from(u16_at(blob, 2)?);
    if blob.len() < 4 + count * 4 || vars.len() != var_count {
        return Err("cabecera o variables del guion incompatibles".into());
    }
    let mut staged_letters = letters.as_deref().map(|ctx| ctx.effects.clone());
    let mut staged_names = names.as_deref().copied();
    let mut staged_harvest = harvest.as_deref().cloned();
    let mut staged = *task;
    let mut values = vars.to_vec();
    let mut pc = u16::from_le_bytes([task.u8_at(0x14), task.u8_at(0x15)]);
    for _ in 0..100_000 {
        let pos = usize::from(pc);
        if pos >= count {
            return Err(format!("comando {pc} fuera de rango"));
        }
        let off = usize::try_from(u32_at(blob, 4 + 4 * pos)?).map_err(|_| "desplazamiento inválido")?;
        let cmd = blob.get(off..).ok_or("comando fuera del guion")?;
        let op = *cmd.first().ok_or("comando truncado")?;
        let need = |n| cmd.get(..n).ok_or_else(|| format!("comando {pc} truncado"));
        match op {
            0x04 => {
                let c = need(2)?;
                let env = humans.as_ref().ok_or("comerciantes del guion ausentes")?;
                if env.populations.is_empty() {
                    return Err("sin ciudades para el guion".into());
                }
                let city = (var(&values, c[1])? as usize) % env.populations.len();
                let count =
                    env.merchant_flags
                        .iter()
                        .fold(0u8, |n, &flags| n.wrapping_add(u8::from(flags == 0))) as i8 as i32;
                let remainder = env.populations[city]
                    .checked_rem(count)
                    .ok_or("selección humana inválida")?;
                let target = i32::from(remainder as u8);
                let mut n = 0xffu8;
                let mut selected = env.merchant_flags.len();
                for (i, &flags) in env.merchant_flags.iter().enumerate() {
                    if flags == 0 {
                        n = n.wrapping_add(1)
                    }
                    if i32::from(n as i8) == target {
                        selected = i;
                        break;
                    }
                }
                set_var(&mut values, c[1], selected as u32)?;
                pc = pc.wrapping_add(1)
            }
            0x05 => {
                let c = need(2)?;
                set_var(&mut values, c[1], tick)?;
                pc = pc.wrapping_add(1)
            }
            0x06 => {
                let c = need(3)?;
                let v = var(&values, c[1])?.wrapping_shl(8).wrapping_add(tick);
                set_var(&mut values, c[2], v)?;
                pc = pc.wrapping_add(1)
            }
            0x09 => {
                let c = need(3)?;
                let env = random.ok_or("flota del generador del guion ausente")?;
                let mut seed = tick;
                if let Some((x, y)) = env.sea_position {
                    seed = seed.wrapping_add(x).wrapping_add(y)
                }
                seed = seed.wrapping_add(u32::from(env.scheduler_head));
                set_var(&mut values, c[2], seed)?;
                let divisor = var(&values, c[1])? as i32;
                let result = (seed as i32).checked_rem(divisor).ok_or("resto inválido del guion")?;
                set_var(&mut values, c[2], result as u32)?;
                pc = pc.wrapping_add(1)
            }
            0x0a => {
                let c = need(6)?;
                set_var(&mut values, c[1], u32_at(c, 2)?)?;
                pc = pc.wrapping_add(1)
            }
            0x0b..=0x0e => {
                let c = need(4)?;
                let a = var(&values, c[1])?;
                let b = var(&values, c[2])?;
                let out = match op {
                    0x0b => a.wrapping_add(b),
                    0x0c => a.wrapping_sub(b),
                    0x0d => a.wrapping_mul(b),
                    _ => {
                        let q = (a as i32).checked_div(b as i32).ok_or("división inválida del guion")?;
                        q as u32
                    }
                };
                set_var(&mut values, c[3], out)?;
                pc = pc.wrapping_add(1)
            }
            0x11 => {
                let c = need(3)?;
                let env = humans
                    .as_ref()
                    .and_then(|ctx| ctx.merchants)
                    .ok_or("imágenes de comerciantes del guion ausentes")?;
                let merchant = env
                    .get(var(&values, c[1])? as usize)
                    .ok_or("comerciante del guion inválido")?;
                let value = *merchant.bytes.get(0x1d).ok_or("comerciante del guion truncado")?;
                set_var(&mut values, c[2], u32::from(value))?;
                pc = pc.wrapping_add(1)
            }
            0x17..=0x1c => {
                let c = need(7)?;
                let a = var(&values, c[1])? as i32;
                let b = var(&values, c[2])? as i32;
                let yes = match op {
                    0x17 => a < b,
                    0x18 => a <= b,
                    0x19 => a == b,
                    0x1a => a != b,
                    0x1b => a >= b,
                    _ => a > b,
                };
                pc = u16_at(c, if yes { 3 } else { 5 })?
            }
            0x1d => pc = u16_at(need(3)?, 1)?,
            0x20 => {
                let c = need(2)?;
                let env = staged_names.as_mut().ok_or("generador de nombres de barco ausente")?;
                let value =
                    crate::ships::factory::next_ship_name_index(&mut env.cursor, env.step, usize::from(env.count))
                        .ok_or("generador de nombres sin salida")?;
                set_var(&mut values, c[1], u32::from(value))?;
                pc = pc.wrapping_add(1)
            }
            0x21 => {
                let c = need(2)?;
                let env = context.ok_or("contexto de ciudad del guion ausente")?;
                if env.town_count == 0 {
                    return Err("divisor de ciudad nulo".into());
                }
                let value = tick
                    .wrapping_add(u32::from(env.scheduler_head))
                    .wrapping_add(env.adjacent_dword)
                    .wrapping_mul(u32::from(pc))
                    % env.town_count;
                set_var(&mut values, c[1], value)?;
                pc = pc.wrapping_add(1)
            }
            // 0x22 (`0x004EF518`): `((tick + cabeza del planificador) · (Town[3]+0x2D4 + 3)) mod 100`.
            0x22 => {
                let c = need(2)?;
                let head = context.ok_or("ciudades del guion ausentes")?.scheduler_head;
                let population = *humans.as_ref().and_then(|h| h.populations.get(3)).ok_or("población de la ciudad 3 del guion ausente")?;
                let value = tick.wrapping_add(u32::from(head)).wrapping_mul((population as u32).wrapping_add(3)) % 100;
                set_var(&mut values, c[1], value)?;
                pc = pc.wrapping_add(1)
            }
            0x23 => {
                let c = need(3)?;
                let env = staged_names.as_ref().ok_or("generador de nombres de pila ausente")?;
                let (_, seed) = env.surname_seed.ok_or("ciudad para nombres de pila ausente")?;
                let (male, female) = env.first_counts.ok_or("contadores de nombres de pila ausentes")?;
                let random = tick.wrapping_add(u32::from(env.scheduler_head)).wrapping_mul(seed) & 0x00ff_ffff;
                set_var(&mut values, c[2], random)?;
                let selector = var(&values, c[1])?;
                let use_male = match selector {
                    1 => true,
                    2 => false,
                    _ => selector & 0x80 == 0,
                };
                let count = if use_male { male } else { female };
                if count == 0 {
                    return Err("banco de nombres de pila vacío".into());
                }
                let value = random % u32::from(count) | if use_male { 0 } else { 0x80 };
                set_var(&mut values, c[2], value)?;
                pc = pc.wrapping_add(1)
            }
            0x24 => {
                let c = need(2)?;
                let env = staged_names.as_ref().ok_or("generador de apellidos ausente")?;
                let (a, b) = env.surname_seed.ok_or("ciudades para apellidos ausentes")?;
                if env.surname_count == 0 {
                    return Err("banco de apellidos vacío".into());
                }
                let seed = u32::from(env.scheduler_head)
                    .wrapping_add(a)
                    .wrapping_add(b)
                    .wrapping_mul(tick & 0xffff)
                    & 0x0fff_ffff;
                set_var(&mut values, c[1], seed % u32::from(env.surname_count))?;
                pc = pc.wrapping_add(1)
            }
            0x29 => {
                let c = need(6)?;
                let env = route.as_ref().ok_or("matriz de rutas de guion ausente")?;
                let n = env.town_count;
                let origin = var(&values, c[4])? as usize;
                if n == 0 || origin >= n || n.checked_mul(n).is_none_or(|size| env.distances.len() < size) {
                    return Err("matriz o ciudad de ruta inválida".into());
                }
                let excluded = [var(&values, c[1])?, var(&values, c[2])?, var(&values, c[3])?];
                let vary = var(&values, c[5])? != 0;
                let mut best = 0x0fff_ffffu32;
                let mut selected = 0u32;
                for city in 0..n {
                    let d = u32::from(env.distances[city * n + origin]);
                    if city != origin && d < best && !excluded.contains(&(city as u32)) {
                        best = d;
                        selected = city as u32
                    }
                }
                if vary {
                    let mut clock = tick;
                    for city in 0..n {
                        if city == origin {
                            continue;
                        }
                        let d = u32::from(env.distances[city * n + origin]);
                        if d < best + 2 {
                            let choose = clock % 3 == 0;
                            clock = clock.wrapping_add(1);
                            if choose && !excluded.contains(&(city as u32)) {
                                selected = city as u32;
                                break;
                            }
                        }
                    }
                }
                let a = var(&values, c[2])?;
                set_var(&mut values, c[1], a)?;
                let b = var(&values, c[3])?;
                set_var(&mut values, c[2], b)?;
                let d = var(&values, c[4])?;
                set_var(&mut values, c[3], d)?;
                set_var(&mut values, c[4], selected)?;
                pc = pc.wrapping_add(1)
            }
            0x38 => {
                let c = need(4)?;
                let env = offices.as_ref().ok_or("oficinas del guion ausentes")?;
                let merchant = var(&values, c[1])? as u16;
                let city = var(&values, c[2])? as u16;
                let mut found = false;
                if merchant < env.merchant_count {
                    if let Some(&head) = env.town_heads.get(usize::from(city)) {
                        let mut at = head;
                        let mut seen = vec![false; usize::from(env.office_count)];
                        while at < env.office_count {
                            let i = usize::from(at);
                            if seen[i] {
                                return Err("cadena urbana de oficinas cíclica".into());
                            }
                            seen[i] = true;
                            let owner = *env.owners.get(i).ok_or("propietario de oficina ausente")?;
                            if owner == merchant {
                                found = true;
                                break;
                            }
                            at = *env.next.get(i).ok_or("enlace de oficina ausente")?;
                        }
                    }
                }
                set_var(&mut values, c[3], u32::from(found))?;
                pc = pc.wrapping_add(1)
            }
            0x39 => {
                let c = need(2)?;
                let env = humans.as_ref().ok_or("comerciantes del guion ausentes")?;
                let count = env.merchant_flags.iter().filter(|&&flags| flags == 0).count();
                set_var(&mut values, c[1], count as u32)?;
                pc = pc.wrapping_add(1)
            }
            0x3f => {
                let c = need(3)?;
                let env = humans.as_ref().ok_or("comerciantes del guion ausentes")?;
                let start = var(&values, c[1])?;
                let count = env.merchant_flags.len();
                if count == 0 || (start != u32::MAX && start as usize >= count) {
                    return Err("índice inicial de jugador inválido".into());
                }
                if (start == u32::MAX || start == 0) && !env.merchant_flags.contains(&0) {
                    return Err("búsqueda humana sin salida".into());
                }
                let mut result = start;
                let mut at = if start == u32::MAX {
                    0
                } else {
                    (start as usize + 1) % count
                };
                for _ in 0..count {
                    if start != u32::MAX && at == start as usize {
                        break;
                    }
                    if env.merchant_flags[at] == 0 {
                        result = at as u32;
                        break;
                    }
                    at = (at + 1) % count
                }
                set_var(&mut values, c[2], result)?;
                pc = pc.wrapping_add(1)
            }
            // 0x01 (`0x004EEA22`) oro; 0x07 (`0x004EED2D`) ciudad natal; 0x12–0x15 (`0x004EF161`…`0x004EF260`):
            // datos del comerciante `vars[a]` en `vars[b]`.
            0x01 | 0x07 | 0x12..=0x15 => {
                let c = need(3)?;
                let env = humans.as_ref().ok_or("comerciantes del guion ausentes")?;
                let m = script_merchant(env, var(&values, c[1])?)?;
                if op == 0x01 {
                    set_var(&mut values, c[2], m.i32(0) as u32)?;
                    pc = pc.wrapping_add(1);
                    continue;
                }
                let value = match op {
                    0x07 => m.bytes[0x19],
                    0x12 => m.bytes[0x1c],
                    0x13 => m.bytes[0x1a],
                    0x14 => m.get_merchant_epithet_index(),
                    // merchant_get_hometown_rank (0x004F3760)
                    _ => *m.bytes.get(0x39c + usize::from(m.bytes[0x19])).ok_or("ciudad natal fuera del rango")?,
                };
                set_var(&mut values, c[2], u32::from(value))?;
                pc = pc.wrapping_add(1)
            }
            // 0x34 (`0x004F0A27`): población (`Town+0x2D4`) de la ciudad `vars[a]`.
            0x34 => {
                let c = need(3)?;
                let env = humans.as_ref().ok_or("ciudades del guion ausentes")?;
                let city = var(&values, c[1])? as usize;
                let population = *env.populations.get(city).ok_or_else(|| format!("ciudad {city} del guion fuera de la tabla"))?;
                set_var(&mut values, c[2], population as u32)?;
                pc = pc.wrapping_add(1)
            }
            // 0x6B (`0x004ED332`): nivel de ampliación de muralla de la ciudad `vars[a]`; 0xFF pasa a -1.
            0x6b => {
                let c = need(3)?;
                let levels = humans.as_ref().and_then(|h| h.wall_levels).ok_or("murallas del guion ausentes")?;
                let city = var(&values, c[1])? as usize;
                let level = *levels.get(city).ok_or_else(|| format!("ciudad {city} del guion fuera de la tabla"))?;
                set_var(&mut values, c[2], if level == 0xff { u32::MAX } else { u32::from(level) })?;
                pc = pc.wrapping_add(1)
            }
            // 0x16 (`0x004EF2B6`): palabra `+8` de la tarea `vars[a]` del planificador.
            0x16 => {
                let c = need(3)?;
                let env = humans.as_ref().ok_or("planificador del guion ausente")?;
                let tasks = env.tasks.ok_or("planificador del guion ausente")?;
                let index = var(&values, c[1])? as usize;
                let record = tasks.get(index).ok_or_else(|| format!("tarea {index} del guion fuera del planificador"))?;
                set_var(&mut values, c[2], u32::from(u16::from_le_bytes([record.payload[0], record.payload[1]])))?;
                pc = pc.wrapping_add(1)
            }
            // 0x1E (`0x004EF3F0`): ciudad (`ship+0x39`) del barco `vars[a]` si su estado (`+0x134`) es < 0xF y no
            // 2 ni 3; si no, -1. 0x28 (`0x004EF9DF`): dueño (`ship+0`) si el barco tiene casco (`+0x18 > 0`); si
            // no, el número de comerciantes.
            0x1e | 0x28 => {
                let c = need(3)?;
                let env = ships.as_ref().ok_or("flota del guion ausente")?;
                if env.ships.len() % 0x180 != 0 {
                    return Err("flota del guion inválida".into());
                }
                let count = env.ships.len() / 0x180;
                let merchants = env.merchant_heads.len() as u32;
                // 0x1E escribe primero -1 en `vars[b]` y después lee `vars[a]` (con `a == b` lee el barco -1);
                // 0x28 lee `vars[a]` antes de escribir el número de comerciantes.
                if op == 0x1e {
                    set_var(&mut values, c[2], u32::MAX)?;
                }
                let id = var(&values, c[1])?;
                let ship = |i: usize| &env.ships[i * 0x180..(i + 1) * 0x180];
                let result = if op == 0x1e {
                    // Comparación con signo: un índice negativo leería fuera de la tabla.
                    if (id as i32) >= count as i32 {
                        u32::MAX
                    } else if (id as i32) < 0 {
                        return Err("barco negativo del guion".into());
                    } else {
                        let s = ship(id as usize);
                        let state = u16::from_le_bytes([s[0x134], s[0x135]]);
                        if state >= 0xf || state == 2 || state == 3 { u32::MAX } else { u32::from(s[0x39]) }
                    }
                } else {
                    let id = id & 0xffff;
                    if (id as usize) < count && i32::from_le_bytes(ship(id as usize)[0x18..0x1c].try_into().unwrap()) > 0 {
                        u32::from(ship(id as usize)[0])
                    } else {
                        merchants
                    }
                };
                set_var(&mut values, c[2], result)?;
                pc = pc.wrapping_add(1)
            }
            0x26 | 0x27 => {
                let c = need(4)?;
                let env = ships.as_ref().ok_or("flota del guion ausente")?;
                let result = pick_loaded_ship(env, var(&values, c[1])?, var(&values, c[2])?, if op == 0x27 { 0xf } else { 0 })?;
                set_var(&mut values, c[3], result)?;
                pc = pc.wrapping_add(1)
            }
            0x40 => {
                let c = need(6)?;
                let env = ships.as_ref().ok_or("flota del guion ausente")?;
                let sense = var(&values, c[1])?;
                let threshold = var(&values, c[2])?;
                let merchant = var(&values, c[3])? as usize;
                let mode = var(&values, c[4])?;
                let head = *env
                    .merchant_heads
                    .get(merchant)
                    .ok_or("comerciante naval del guion inválido")?;
                let result = find_ship_by_health(head, env.ships, sense, threshold, mode)?;
                set_var(&mut values, c[5], result)?;
                pc = pc.wrapping_add(1)
            }
            0x43 => {
                let c = need(2)?;
                let env = context.ok_or("ciudades del guion ausentes")?;
                set_var(&mut values, c[1], env.town_count)?;
                pc = pc.wrapping_add(1)
            }
            0x44 => {
                let c = need(3)?;
                let env = offices.as_ref().ok_or("oficinas del guion ausentes")?;
                let chains = env
                    .merchant_chains
                    .as_ref()
                    .ok_or("cadenas de oficinas por comerciante ausentes")?;
                let merchant = var(&values, c[1])?;
                let town = random_office_town(chains, merchant, env.office_count, tick)?;
                set_var(&mut values, c[2], town)?;
                pc = pc.wrapping_add(1)
            }
            0x4f => {
                let c = need(4)?;
                let env = offices.as_ref().ok_or("oficinas del guion ausentes")?;
                let merchants = humans.as_ref().ok_or("comerciantes del guion ausentes")?;
                set_var(&mut values, c[3], 0)?;
                let city = var(&values, c[2])? as usize;
                let mut at = *env.town_heads.get(city).ok_or("ciudad de oficinas inválida")?;
                let mut seen = vec![false; usize::from(env.office_count)];
                while at < env.office_count {
                    let i = usize::from(at);
                    if seen[i] {
                        return Err("cadena urbana de oficinas cíclica".into());
                    }
                    seen[i] = true;
                    let mode = var(&values, c[1])?;
                    let include = if mode == 2 {
                        true
                    } else if mode <= 1 {
                        let owner = usize::from(*env.owners.get(i).ok_or("propietario de oficina ausente")?);
                        let flags = *merchants
                            .merchant_flags
                            .get(owner)
                            .ok_or("comerciante de oficina ausente")?;
                        (mode == 0) == (flags == 0)
                    } else {
                        false
                    };
                    if include {
                        let n = var(&values, c[3])?.wrapping_add(1);
                        set_var(&mut values, c[3], n)?
                    }
                    at = *env.next.get(i).ok_or("enlace de oficina ausente")?
                }
                pc = pc.wrapping_add(1)
            }
            0x00 => {
                let c = need(2)?;
                let due = var(&values, c[1])?;
                if due.wrapping_sub(0x100) >= tick {
                    staged.due = due;
                    pc = pc.wrapping_add(1);
                    set_pc(&mut staged, pc);
                    if let (Some(dst), Some(value)) = (names.as_deref_mut(), staged_names) {
                        *dst = value
                    }
                    if let (Some(dst), Some(value)) = (letters.as_deref_mut(), staged_letters) {
                        dst.effects = value
                    }
                    if let (Some(dst), Some(value)) = (harvest.as_deref_mut(), staged_harvest) {
                        *dst = value
                    }
                    *task = staged;
                    vars.copy_from_slice(&values);
                    return Ok(Outcome::Waiting);
                }
                staged.set_u32_at(0xc, 0);
                set_pc(&mut staged, pc);
                if let (Some(dst), Some(value)) = (names.as_deref_mut(), staged_names) {
                    *dst = value
                }
                if let (Some(dst), Some(value)) = (letters.as_deref_mut(), staged_letters) {
                    dst.effects = value
                }
                if let (Some(dst), Some(value)) = (harvest.as_deref_mut(), staged_harvest) {
                    *dst = value
                }
                *task = staged;
                vars.copy_from_slice(&values);
                return Ok(Outcome::Finished);
            }
            0x53 => {
                let c = need(4)?;
                let env = route.as_ref().ok_or("matriz de rutas de guion ausente")?;
                let a = var(&values, c[1])? as usize;
                let b = var(&values, c[2])? as usize;
                let n = env.town_count;
                let value = if a >= n || b >= n || env.distances.is_empty() {
                    255
                } else {
                    let size = n.checked_mul(n).ok_or("matriz de distancias excesiva")?;
                    if env.distances.len() < size {
                        return Err("matriz de distancias truncada".into());
                    }
                    u32::from(env.distances[a * n + b])
                };
                set_var(&mut values, c[3], value)?;
                pc = pc.wrapping_add(1)
            }
            // 0x37 (`0x004F0E94`): ventana de suceso con dos textos del guion; solo presentación.
            // 0x3A (`0x004F0FCF`): reserva del barco `vars[a]` (u16) sube o baja según `vars[b] != 0`.
            0x3a => {
                let c = need(3)?;
                let env = ships.as_ref().ok_or("flota del guion ausente")?;
                let sink = env.reserve.ok_or("destino de reservas ausente")?;
                sink.borrow_mut().push(ShipReserveOp { ship: var(&values, c[1])? as u16, raise: var(&values, c[2])? != 0 });
                pc = pc.wrapping_add(1)
            }
            // 0x65 (`0x004F26C0`): índice de la ciudad de la partida cuyo nombre es el nº `vars[a]` del banco; -1 si no existe.
            0x65 => {
                let c = need(3)?;
                let env = humans.as_ref().ok_or("contexto de ciudades ausente")?;
                let bank = env.town_name_bank.ok_or("banco de nombres urbanos ausente")?;
                let towns = env.game_town_names.ok_or("nombres de ciudades ausentes")?;
                let n = var(&values, c[1])?;
                let found: u8 = if n >= 0x28 {
                    0xff
                } else {
                    let name = bank.get(n as usize).ok_or("banco de nombres urbanos incompleto")?;
                    let cstr = |v: &Vec<u8>| v.split(|&b| b == 0).next().unwrap_or_default().to_vec();
                    let name = cstr(name);
                    // El original recorre de la última ciudad a la primera y devuelve la primera coincidencia.
                    (0..towns.len()).rev().find(|&i| cstr(&towns[i]) == name).map_or(0xff, |i| i as u8)
                };
                let result = if u32::from(found) >= towns.len() as u32 { u32::MAX } else { u32::from(found) };
                set_var(&mut values, c[2], result)?;
                pc = pc.wrapping_add(1)
            }
            // 0xE7/0xE9/0xF9 (`0x004ED125`): barco pirata de misión diferido (operación 0x82) y `vars[d] = -1`
            // hasta que 0x0053FB00 escriba en esa variable el barco creado (o -2 si no lo hay).
            0xe7 | 0xe9 | 0xf9 => {
                let c = need(5)?;
                let env = letters.as_deref().ok_or("cola de operaciones del guion ausente")?;
                let record = mission_pirate_record(op, [c[1], c[2], c[3], c[4]], &values, env.scheduler_head)?;
                set_var(&mut values, c[4], u32::MAX)?;
                staged_letters.as_mut().ok_or("depósito de cartas de guion ausente")?.push(LetterEffect::MissionPirate(record));
                pc = pc.wrapping_add(1)
            }
            // 0x37 (`0x004F0E94`): ventana de suceso (`0x469380(0)` y `0x469AF0(0x16, task+8, task+0xA, título,
            // texto)` del gestor de interfaz `0x6CC7E8`). No cambia estado de la partida; los textos se obtienen
            // con `event_window_texts`.
            0x37 => {
                need(9)?;
                pc = pc.wrapping_add(1)
            }
            0xf8 => {
                let c = need(3)?;
                let merchant = var(&values, c[1])?;
                let count = humans.as_ref().and_then(|h| h.merchants).map(<[_]>::len).ok_or("comerciantes del guion ausentes")?;
                // Con un comerciante fuera de la tabla (los guiones usan -1 como «ninguno») el original escribe en la
                // memoria anterior a la tabla de comerciantes, que no es estado de la partida: no hay efecto.
                if (merchant as usize) < count {
                    let effect = LetterEffect::Gold { merchant, amount: var(&values, c[2])? };
                    staged_letters.as_mut().ok_or("depósito de cartas de guion ausente")?.push(effect);
                }
                pc = pc.wrapping_add(1)
            }
            0xf0 => {
                let c = need(3)?;
                let effect = LetterEffect::RemoveTreasure {
                    merchant: var(&values, c[1])?,
                    entry: var(&values, c[2])?,
                };
                staged_letters
                    .as_mut()
                    .ok_or("depósito de cartas de guion ausente")?
                    .push(effect);
                pc = pc.wrapping_add(1)
            }
            0xf7 => {
                let c = need(12)?;
                let env = letters
                    .as_deref()
                    .filter(|ctx| ctx.resources_available)
                    .ok_or("recursos de cartas de guion ausentes")?;
                let flags = env.merchant_images.iter().map(|m| m.u16(8)).collect::<Vec<_>>();
                let (prepared, notification) = crate::mission_letter::prepare_letters(
                    blob,
                    c,
                    &mut values,
                    env.scheduler_head,
                    |i| env.ship_names.get(usize::from(i)).map(Vec::as_slice),
                    env.town_names,
                    env.family_names,
                    env.first_names,
                    env.gender_words
                        .map(|gender_words| crate::mission_letter::GrammarContext {
                            merchants: env.merchant_images,
                            gender_words,
                            extras: env.format_extras.map(|source| source.format_extras()),
                        }),
                    &flags,
                )?;
                let effects = staged_letters.as_mut().expect("contexto comprobado");
                effects.extend(prepared.into_iter().map(LetterEffect::Deliver));
                if let Some((args, recipient)) = notification {
                    effects.push(LetterEffect::Notify { args, recipient })
                }
                pc = pc.wrapping_add(1)
            }
            0xfb => {
                let c = need(3)?;
                let city = var(&values, c[1])? as u16;
                let duration = var(&values, c[2])?;
                staged_harvest
                    .as_mut()
                    .ok_or("planificador de cosecha ausente")?
                    .push(HarvestSpawn { city, duration });
                pc = pc.wrapping_add(1)
            }
            0xfd => {
                let c = need(4)?;
                staged.due = tick.wrapping_add(var(&values, c[1])?);
                pc = u16_at(c, 2)?;
                set_pc(&mut staged, pc);
                if let (Some(dst), Some(value)) = (names.as_deref_mut(), staged_names) {
                    *dst = value
                }
                if let (Some(dst), Some(value)) = (letters.as_deref_mut(), staged_letters) {
                    dst.effects = value
                }
                if let (Some(dst), Some(value)) = (harvest.as_deref_mut(), staged_harvest) {
                    *dst = value
                }
                *task = staged;
                vars.copy_from_slice(&values);
                return Ok(Outcome::Waiting);
            }
            0xfe => {
                let c = need(2)?;
                staged.due = tick.wrapping_add(var(&values, c[1])?.wrapping_shl(8));
                set_pc(&mut staged, 0);
                if let (Some(dst), Some(value)) = (names.as_deref_mut(), staged_names) {
                    *dst = value
                }
                if let (Some(dst), Some(value)) = (letters.as_deref_mut(), staged_letters) {
                    dst.effects = value
                }
                if let (Some(dst), Some(value)) = (harvest.as_deref_mut(), staged_harvest) {
                    *dst = value
                }
                *task = staged;
                vars.copy_from_slice(&values);
                return Ok(Outcome::Waiting);
            }
            0xff | 0x30 | 0x3d => {
                staged.set_u32_at(0xc, 0);
                set_pc(&mut staged, pc);
                if let (Some(dst), Some(value)) = (names.as_deref_mut(), staged_names) {
                    *dst = value
                }
                if let (Some(dst), Some(value)) = (letters.as_deref_mut(), staged_letters) {
                    dst.effects = value
                }
                if let (Some(dst), Some(value)) = (harvest.as_deref_mut(), staged_harvest) {
                    *dst = value
                }
                *task = staged;
                vars.copy_from_slice(&values);
                return Ok(Outcome::Finished);
            }
            _ => match flat.as_deref_mut() {
                Some(f) if crate::mission_script_flat::handles(op) => {
                    crate::mission_script_flat::step(f, op, cmd, &mut values)?;
                    pc = pc.wrapping_add(1)
                }
                _ => return Err(format!("comando {op:#04x} sin implementar en índice {pc}")),
            },
        }
    }
    Err("límite de instrucciones del guion".into())
}
