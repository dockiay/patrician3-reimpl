//! Figuras de la batalla en la ciudad (C08): el caminante común de la vista (`0x0055E8C0`) y los métodos por
//! fotograma de cada clase que crea el asedio (defensores, milicia, asaltantes, cabecillas, jefe y puertas).
//!
//! Como [`super::siege_battle`], el porte trabaja sobre la memoria del proceso original ([`Mem`]) con sus
//! direcciones: la vista es `[0x006E51AC]`, los registros de asedio `[0x006DF270] + 0x1024 * ciudad` y las ciudades
//! `[0x006DE508] + 0x9F8 * ciudad`. Los métodos virtuales se resuelven como en el x86, por la dirección que hay en
//! la vtabla del objeto ([`vcall`]): los portados se ejecutan aquí y el resto pasa por [`FigureHost::virt`].
//!
//! Submódulos:
//! * [`classes`]: bucles `+0x14`, disparos `+0x64` y avances `+0x58` de las clases 1, 2, 3, 4 y 7.
//! * [`common`]: métodos comunes de las clases: visibilidad `+0x60` (`0x00561140`), paso posible (`0x00561330`),
//!   búsqueda de objetivo `+0x58` (`0x00561520`) y reserva del objetivo (`0x00561F60`).
//! * [`fighter`]: luchador de la cola de la puerta (clase 0): bucle `0x00577950`, formación `0x005790B0`, búsqueda
//!   y paso `0x00577FB0`, visibilidad `0x00579350`, pareja `0x00579380` y nivel de muralla `0x005252F0`.
//! * [`footsteps`]: huellas en la nieve (`0x0055D0B0`).
//! * [`target`]: siguiente destino del caminante libre (`0x0055D9D0`).
//! * [`update`]: bucle por fotograma del caminante (`0x0055E8C0`) con sus ayudantes de escena (`0x00571ED0`,
//!   `0x006319D0`, `0x0062B960`).
//! * [`walker`]: decisiones del caminante en el asalto (`0x0055FFF0`, `0x00560CB0`, `0x00560750`, `0x00560850`,
//!   `0x00560970`) y ayudantes de la escena (`0x00630760`, `0x00630C40`, `0x00630CC0`, `0x00630D40`, `0x00561080`).
//! * [`sprite`]: estado de animación (`0x00557340`, `0x00557730`, `0x005576B0`, `0x005576C0`), rectángulos del
//!   fotograma (`0x00559260`), alta y baja en las listas por casilla de la vista (`+0xC310`, `0x00559810`,
//!   `0x005596C0`, `0x005934E0`, `0x00593540`, `0x00559A50`, `0x00559BB0`) y las del caminante (`0x0055E250`,
//!   `0x0055E430`).
use crate::economy::siege_view::SiegeViewHost;
use crate::flatmem::Mem;

pub mod classes;
pub mod entities;
pub mod effect;
pub mod combat;
pub mod common;
pub mod fighter;
pub mod projectile;
pub mod ctors;
pub mod ram;
pub mod sets;
pub mod siege_unit;
pub mod spawn;
pub mod footsteps;
pub mod sprite;
pub mod target;
pub mod step;
pub mod tables;
pub mod update;
pub mod walker;

/// Puntero global a la vista de ciudad (`Class22`).
pub const VIEW: u32 = 0x006E_51AC;
/// Registros de asedio por ciudad (`[0x006DF270] + 0x1024 * ciudad`).
pub const SIEGES: u32 = 0x006D_F270;
/// Tabla de ciudades (`[0x006DE508] + 0x9F8 * ciudad`).
pub const TOWNS: u32 = 0x006D_E508;
/// Reloj del fotograma en milisegundos.
pub const CLOCK: u32 = 0x006D_CCF8;
/// Tabla de rumbos del EXE (`0x00677928..0x0067792F`, solo lectura en la práctica): 0x00, 0x20, ..., 0xE0.
pub const DIRS: [u8; 8] = [0x00, 0x20, 0x40, 0x60, 0x80, 0xA0, 0xC0, 0xE0];

/// Rectángulo de Windows `[izquierda, arriba, derecha, abajo]`.
pub type Rect = [i32; 4];

/// Lo que las figuras piden fuera de la memoria de la vista.
pub trait FigureHost: SiegeViewHost {
    /// `0x004B9650` (recortador de pantalla `0x006DA5F0`): rectángulo que hay que repintar.
    fn dirty(&mut self, _m: &mut dyn Mem, _r: Rect) {}
    /// `0x00443690` (`thiscall` sobre el gestor de sonido `[0x006CBDB8]`): cuántos sonidos tiene la lista.
    fn sound_list_size(&mut self, _m: &mut dyn Mem, _list: u32) -> u32 {
        0
    }
    /// `0x00595B00` (`thiscall` sobre la vista): sonido en el píxel `(x, y)`.
    fn play_sound(&mut self, _m: &mut dyn Mem, _x: i32, _y: i32, _id: u32) {}
    /// Método virtual sin portar: dirección de la función, `this` y argumentos de la pila. Devuelve `EAX`.
    fn virt(&mut self, m: &mut dyn Mem, f: u32, this: u32, args: &[u32]) -> u32;
    /// Sustituto de un método virtual aunque esté portado (las pruebas lo usan cuando el x86 del caso lo sustituye
    /// también). Por defecto ninguno.
    fn stubbed(&mut self, _m: &mut dyn Mem, _f: u32, _this: u32, _args: &[u32]) -> Option<u32> {
        None
    }
}

pub(crate) fn s16(m: &mut dyn Mem, a: u32) -> i32 {
    i32::from(m.r16(a) as i16)
}

/// `[[a] + off]`.
pub(crate) fn ld32(m: &mut dyn Mem, a: u32, off: u32) -> u32 {
    let b = m.r32(a);
    m.r32(b.wrapping_add(off))
}

/// `word [[a] + off]`.
pub(crate) fn ld16(m: &mut dyn Mem, a: u32, off: u32) -> u16 {
    let b = m.r32(a);
    m.r16(b.wrapping_add(off))
}

pub(crate) fn view(m: &mut dyn Mem) -> u32 {
    m.r32(VIEW)
}

/// Contenedor de asedio de la ciudad (`[0x006DF270] + 0x1024 * ciudad`).
pub(crate) fn siege_of(m: &mut dyn Mem, city: u8) -> u32 {
    m.r32(SIEGES).wrapping_add(u32::from(city) * 0x1024)
}

/// Ciudad (`[0x006DE508] + 0x9F8 * ciudad`).
pub(crate) fn town_of(m: &mut dyn Mem, city: u8) -> u32 {
    m.r32(TOWNS).wrapping_add(u32::from(city) * 0x9F8)
}

/// `0x00629C30(ecx = escena del asedio)`: ¿el asalto sigue activo?
pub fn scene_active(m: &mut dyn Mem, s: u32) -> bool {
    let w = m.r16(s + 0x3A);
    if w != 0 {
        w > m.r16(s + 0x6A)
    } else {
        m.r16(s + 0x21E) != 0
    }
}

/// `0x00633AE0(ecx = contenedor de asedio)`.
pub fn siege_active(m: &mut dyn Mem, rec: u32) -> bool {
    let s = m.r32(rec + 8);
    s != 0 && scene_active(m, s)
}

/// Método virtual `off` de `this` resuelto por la dirección de la vtabla, como el x86.
pub fn vcall(m: &mut dyn Mem, h: &mut dyn FigureHost, this: u32, off: u32, args: &[u32]) -> u32 {
    let vt = m.r32(this);
    let f = m.r32(vt.wrapping_add(off));
    dispatch(m, h, f, this, args)
}

/// Funciones de vtabla portadas.
pub fn dispatch(m: &mut dyn Mem, h: &mut dyn FigureHost, f: u32, this: u32, args: &[u32]) -> u32 {
    if let Some(v) = h.stubbed(m, f, this, args) {
        return v;
    }
    match f {
        // Clase de unidad (`+0x68`).
        0x0061_1730 => 0,
        0x0060_2980 => 1,
        0x0056_2980 => 2,
        0x0057_30E0 => 3,
        0x0057_1FB0 => 4,
        0x0056_F840 => 7,
        // Conjunto de sprites (`+0x4C`).
        0x0057_98E0 => sprite::set_by(m, this + 0x90, 0x006E_2E58),
        0x0056_79E0 => sprite::set_by(m, this + 0x88, 0x006E_174C),
        0x0056_2990 => sprite::set_by(m, this + 0x88, 0x006E_16E0),
        0x0057_30F0 => sprite::set_by(m, this + 0x88, 0x006E_2DD0),
        0x0057_1FC0 => sprite::set_by(m, this + 0x91, 0x006E_2DAC),
        0x0056_AAB0 => sprite::set_by(m, this + 0x88, 0x006E_1854),
        0x0056_F850 => sprite::set_by(m, this + 0x88, 0x006E_2D8C),
        0x0057_7490 => projectile::projectile_set(m, this),
        0x0057_B250 => effect::effect_set(m, this),
        0x0057_58E0 => ram::ram_set(m, this),
        0x0056_6FE0 => siege_unit::unit_set(m, this),
        // Byte de clase del conjunto (`+0x20`).
        0x0055_8640 => {
            let set = vcall(m, h, this, 0x4C, &[]);
            u32::from(m.r8(set + 8))
        }
        // Byte de clase del conjunto propio de los artilleros y los luchadores (`+0x20`).
        0x0057_30B0 | 0x0057_98B0 => {
            let set = if f == 0x0057_30B0 { sprite::set_by(m, this + 0x88, 0x006E_2DD0) } else { sprite::set_by(m, this + 0x90, 0x006E_2E58) };
            u32::from(m.r8(set + 8))
        }
        // Profundidad (`+0x30`).
        0x0055_86D0 => sprite::entity_depth(m, h, this),
        // Fotograma de la animación sin cambios (`+0x64` por defecto).
        0x0056_1510 => args[0] & 0xFF,
        // Bucle común del caminante (`+0x14` de las clases lo llaman con sus argumentos).
        0x0055_E8C0 => u32::from(update::walker_update(m, h, this, args[0], args[1], args[2], args[3], args[4], args[5])),
        // Bucles por clase (`+0x14`).
        0x0056_74F0 => u32::from(classes::archer_update(m, h, this)),
        0x0056_24C0 => u32::from(classes::crossbow_update(m, h, this)),
        0x0057_2BA0 => u32::from(classes::gunner_update(m, h, this)),
        0x0057_10A0 => u32::from(classes::pikeman_update(m, h, this)),
        0x0056_A7F0 => u32::from(classes::infantry_update(m, h, this)),
        0x0056_F1A0 => u32::from(classes::rider_update(m, h, this)),
        // Disparos (`+0x64`).
        0x0057_7950 => u32::from(fighter::fighter_update(m, h, this)),
        // Baja de la vista (`+0x44`): el caminante sale de las listas por casilla con el conjunto de su clase.
        0x0057_7920 | 0x0056_74C0 | 0x0056_2490 | 0x0057_2B70 | 0x0057_1070 | 0x0056_A7C0 | 0x0056_F170 => {
            let (at, table) = match f {
                0x0057_7920 => (0x90, 0x006E_2E58),
                0x0056_74C0 => (0x88, 0x006E_174C),
                0x0056_2490 => (0x88, 0x006E_16E0),
                0x0057_2B70 => (0x88, 0x006E_2DD0),
                0x0057_1070 => (0x91, 0x006E_2DAC),
                0x0056_A7C0 => (0x88, 0x006E_1854),
                _ => (0x88, 0x006E_2D8C),
            };
            let set = sprite::set_by(m, this + at, table);
            sprite::walker_remove(m, h, this, set, false);
            0
        }
        // Aparición (`+0x40(x, y, rumbo, enlace, variante)`).
        0x0057_77B0 | 0x0056_73E0 | 0x0056_23B0 | 0x0057_2A90 | 0x0057_0F70 | 0x0056_A700 | 0x0056_F0B0 => {
            let a = [args[0], args[1], args[2], args[3], args[4]];
            match f {
                0x0057_77B0 => spawn::fighter_init(m, h, this, a),
                0x0056_73E0 => spawn::shooter_init(m, h, spawn::ARCHERS, this, a),
                0x0056_23B0 => spawn::shooter_init(m, h, spawn::CROSSBOWS, this, a),
                0x0057_2A90 => spawn::shooter_init(m, h, spawn::GUNNERS, this, a),
                0x0057_0F70 => spawn::pikeman_init(m, h, this, a),
                0x0056_A700 => spawn::troop_init(m, h, spawn::INFANTRY, this, a),
                _ => spawn::troop_init(m, h, spawn::RIDERS, this, a),
            }
            0
        }
        // Proyectil (clase `0x17`) y bajas por impacto.
        0x0057_6990 => {
            projectile::projectile_init(m, h, this, args[0], args[1], args[2], args[3], args[4]);
            0
        }
        0x0057_6E80 => u32::from(projectile::projectile_update(m, h, this)),
        0x0057_6D20 => {
            projectile::projectile_remove(m, h, this);
            0
        }
        0x0062_FB90 => {
            projectile::defeat_near(m, this, args[0], args[1]);
            0
        }
        // Efectos (clase `0x12`): alta `+0x40`, paso `+0x14`, baja `+0x44`, traslado `+0x48` y fundido.
        0x0057_A150 => {
            effect::effect_init(m, h, this, [args[0], args[1], args[2], args[3], args[4]]);
            0
        }
        0x0057_A8C0 => u32::from(effect::effect_update(m, h, this)),
        0x0057_A650 => {
            effect::effect_remove(m, h, this);
            0
        }
        0x0057_A4A0 => {
            effect::effect_move(m, h, this, args[0], args[1], args[2] as u8);
            0
        }
        // Ariete (clase `0x16`): alta `+0x40`, paso `+0x14`, baja `+0x44`, traslado `+0x48` (nada) y final de animación.
        0x0057_40F0 => {
            ram::ram_init(m, h, this, [args[0], args[1], args[2], args[3], args[4]]);
            0
        }
        0x0057_43A0 => u32::from(ram::ram_update(m, h, this)),
        0x0057_4360 => {
            ram::ram_remove(m, h, this);
            0
        }
        0x0057_4350 => 0,
        0x0057_5810 => {
            ram::ram_anim_finished(m, this);
            0
        }
        // Máquina de asedio (clase `0x15`): alta, paso, baja, traslado (nada), fotograma `+0x50` y final de animación.
        0x0056_4810 => {
            siege_unit::unit_init(m, h, this, [args[0], args[1], args[2], args[3], args[4]]);
            0
        }
        0x0056_4AC0 => u32::from(siege_unit::unit_update(m, h, this)),
        0x0056_4A80 => {
            siege_unit::unit_remove(m, h, this);
            0
        }
        0x0056_4A70 => 0,
        0x0056_6E30 => u32::from(siege_unit::on_frame(m, h, this, args[0] as u8)),
        0x0056_6260 => {
            siege_unit::unit_anim_finished(m, h, this, args[0]);
            0
        }
        // Final de animación de los efectos: nada.
        0x0057_B240 => 0,
        0x0057_AF40 => {
            effect::set_fade(m, this, args[0], args[1]);
            0
        }
        // Final de animación (`+8` del estado de animación).
        0x0055_FFC0 | 0x0056_7720 | 0x0056_26D0 | 0x0057_2DB0 => {
            combat::anim_reset(m, this);
            0
        }
        0x0057_7CB0 => {
            combat::fighter_hit(m, this, args[0]);
            0
        }
        0x0057_1350 => {
            combat::pikeman_anim_finished(m, h, this, args[0]);
            0
        }
        0x0057_9350 => u32::from(fighter::fighter_visible(m, h, this, args[0] as u8)),
        0x0057_90B0 => u32::from(fighter::formation(m, h, this, args[0])),
        0x0057_7FB0 => u32::from(fighter::fighter_target(m, h, this)),
        0x0057_9380 => u32::from(fighter::approach_partner(m, h, this)),
        0x0052_52F0 => u32::from(fighter::wall_level(m, this)),
        0x0056_7730 => u32::from(classes::archer_frame(m, h, this, args[0])),
        0x0056_26E0 => u32::from(classes::crossbow_frame(m, h, this, args[0])),
        0x0057_2DC0 => u32::from(classes::gunner_frame(m, h, this, args[0])),
        // Avances propios (`+0x58`).
        0x0057_19A0 => u32::from(classes::pikeman_rally(m, h, this, args[0] as u8)),
        0x0056_F410 => u32::from(classes::rider_charge(m, h, this, args[0] as u8)),
        0x0056_AA20 => 0,
        0x0057_1DA0 => {
            classes::pikeman_acquire(m, this);
            0
        }
        // Búsqueda de objetivo (`+0x58` común) y visibilidad (`+0x60` común).
        0x0056_1520 => u32::from(common::find_target(m, h, this, args[0] as u8)),
        0x0056_1140 => u32::from(common::visible_and_cells_free(m, h, this, args[0] as u8)),
        0x0056_1280 => u32::from(common::wide_tile_free(m, h, args[0], args[1])),
        0x0056_1330 => u32::from(common::can_step_to_tile(m, h, this, args[0], args[1], args[2] as u8)),
        0x0056_1F60 => {
            common::mark_target_engaged(m, this);
            0
        }
        // Siguiente destino del caminante libre (llamada directa).
        0x0055_D9D0 => {
            target::choose_next_target(m, h, this);
            0
        }
        // Huellas en la nieve (llamada directa).
        0x0055_D0B0 => {
            footsteps::footsteps(m, h, this, args[0], args[1], args[2] as i32, args[3] as i32);
            0
        }
        // Casilla libre (`+0x5C`).
        0x0056_0850 => u32::from(walker::tile_free(m, h, this, args[0], args[1], args[2] as u8)),
        // Estado de animación: avance (`+4`).
        0x0055_7340 => u32::from(sprite::anim_advance(m, h, this, args[0], args[1], args[2])),
        _ => h.virt(m, f, this, args),
    }
}
