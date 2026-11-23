//! Constructores de las clases del asalto que crea la fábrica `sprite_entity_factory` (`0x00557D10`), con la cadena
//! de bases del original: `sprite_entity_ctor` (`0x00557A00`: enlaces de casilla `0x00559910`, instancia de sprite
//! `0x00558F00` en `+8` y cargador `0x00558750` en `+0x1C`), `town_mover_ctor` (`0x0055C630`, reglas de movimiento
//! `0x0055A260` en `+0x20`) y `town_walker_ctor` (`0x0055DFC0`), y el reinicio de movimiento `0x0055E050`.
//!
//! Solo escriben lo que escribe el original; el resto del objeto queda como lo deja el asignador. Los marcos SEH de
//! los constructores no cambian el objeto.
use crate::flatmem::Mem;

/// Byte de `.rdata` que copia la instancia de sprite (`0x0067792B`, primera velocidad de animación: `0x60`).
pub const ANIM_STEP: u8 = 0x60;

/// Clase, bytes que reserva la fábrica y constructor.
pub const CLASSES: &[(u8, u32, u32)] = &[
    (0x08, 0x94, 0x0057_75A0),
    (0x09, 0x8C, 0x0056_70F0),
    (0x0A, 0x8C, 0x0056_20C0),
    (0x0B, 0x8C, 0x0057_27A0),
    (0x12, 0x48, 0x0057_9A00),
    (0x15, 0x78, 0x0056_4540),
    (0x16, 0x68, 0x0057_3F30),
    (0x17, 0x44, 0x0057_6690),
    (0x20, 0x94, 0x0057_0DB0),
    (0x23, 0x8C, 0x0056_A540),
    (0x24, 0x8C, 0x0056_EEF0),
];

/// `sprite_entity_ctor` (`0x00557A00`).
pub fn sprite_entity(m: &mut dyn Mem, e: u32) {
    // `entity_cell_links_ctor` (0x00559910).
    m.w32(e, 0x0067_4B64);
    m.w32(e + 4, 0);
    // `sprite_instance_ctor` (0x00558F00) sobre `+8`.
    let s = e + 8;
    m.w32(s, 0x0067_4B54);
    m.w32(s + 4, 0);
    m.w32(s + 8, 0);
    m.w8(s + 0xC, ANIM_STEP);
    m.w8(s + 0xD, 0);
    m.w16(s + 0xE, 0);
    m.w8(s + 0x10, 1);
    // `entity_sprite_loader_ctor` (0x00558750) sobre `+0x1C`.
    m.w32(e + 0x1C, 0x0067_4B44);
    m.w32(e + 8, 0x0067_4B38);
    m.w32(e, 0x0067_4AE8);
    m.w32(e + 0x1C, 0x0067_4AD8);
}

/// Base común de los objetos de un solo sprite (efecto, máquina, ariete, proyectil): tras `0x00557A00` ponen
/// la regla de movimiento vacía en `+0x20` y la limpian.
fn simple_base(m: &mut dyn Mem, e: u32) {
    sprite_entity(m, e);
    m.w32(e + 0x20, 0x0067_4E54);
    m.w8(e + 0x28, 0);
    m.w8(e + 0x29, 0);
    m.w32(e + 0x24, 0);
}

/// `town_mover_ctor` (`0x0055C630`).
pub fn town_mover(m: &mut dyn Mem, e: u32) {
    sprite_entity(m, e);
    // `move_rules_ctor` (0x0055A260) sobre `+0x20`: base 0x0055A110 y su tabla.
    m.w32(e + 0x20, 0x0067_4B74);
    m.w32(e + 0x20, 0x0067_4BC4);
    m.w32(e + 0x2C, 0x0067_4E54);
    m.w8(e + 0x34, 0);
    m.w8(e + 0x35, 0);
    m.w32(e + 0x30, 0);
    let step = m.r8(e + 0x14);
    m.w32(e + 0x20, 0x0067_4E50);
    m.w8(e + 0x3C, step);
    m.w8(e + 0x3D, step);
    m.w32(e, 0x0067_4E00);
    m.w32(e + 8, 0x0067_4DFC);
    m.w32(e + 0x1C, 0x0067_4DF0);
    m.w32(e + 0x2C, 0x0067_4DE4);
    m.w16(e + 0x38, 0);
    m.w16(e + 0x3A, 0);
    for off in [0x40, 0x44, 0x48] {
        m.w32(e + off, 0);
    }
    for off in 0x54..0x58 {
        m.w8(e + off, 0);
    }
}

/// `town_walker_reset_motion_state` (`0x0055E050`).
pub fn walker_reset_motion(m: &mut dyn Mem, e: u32) {
    m.w16(e + 0x38, 0);
    m.w16(e + 0x3A, 0);
    m.w8(e + 0x58, 0);
    m.w8(e + 0x70, 0);
    m.w8(e + 0x7A, 0);
    m.w16(e + 0x68, 0);
}

/// `town_walker_ctor` (`0x0055DFC0`).
pub fn town_walker(m: &mut dyn Mem, e: u32) {
    town_mover(m, e);
    m.w32(e, 0x0067_4E90);
    walker_reset_motion(m, e);
    m.w32(e + 8, 0x0067_4E88);
    m.w32(e + 0x1C, 0x0067_4E7C);
    m.w32(e + 0x20, 0x0067_4E78);
    m.w32(e + 0x2C, 0x0067_4E6C);
}

/// Caminante de la batalla: base `0x0055DFC0`, tablas propias, byte de conjunto a `0xFF` (con el byte siguiente a
/// cero en algunas clases) y `0x0055E050`.
fn walker_class(m: &mut dyn Mem, e: u32, vt: [u32; 5], set: u32, next_zero: bool) {
    town_walker(m, e);
    for (off, v) in [0, 8, 0x1C, 0x20, 0x2C].into_iter().zip(vt) {
        m.w32(e + off, v);
    }
    m.w8(e + set, 0xFF);
    if next_zero {
        m.w8(e + set + 1, 0);
    }
    walker_reset_motion(m, e);
}

/// Tablas `+0, +8, +0x1C, +0x20` de los objetos de un solo sprite.
fn simple_vt(m: &mut dyn Mem, e: u32, vt: [u32; 4]) {
    for (off, v) in [0, 8, 0x1C, 0x20].into_iter().zip(vt) {
        m.w32(e + off, v);
    }
}

/// Constructor de la clase `class` sobre el objeto `e` (los de [`CLASSES`]). Devuelve falso si no es de esa lista.
pub fn construct(m: &mut dyn Mem, class: u32, e: u32) -> bool {
    match class {
        // 0x005775A0: luchador de la cola.
        0x08 => walker_class(m, e, [0x0067_62C0, 0x0067_62B8, 0x0067_62AC, 0x0067_62A8, 0x0067_629C], 0x90, false),
        // 0x005670F0, 0x005620C0, 0x005727A0: arqueros, ballesteros y arcabuceros.
        0x09 => walker_class(m, e, [0x0067_5358, 0x0067_5354, 0x0067_5348, 0x0067_5344, 0x0067_5338], 0x88, true),
        0x0A => walker_class(m, e, [0x0067_4F28, 0x0067_4F20, 0x0067_4F14, 0x0067_4F10, 0x0067_4F04], 0x88, true),
        0x0B => walker_class(m, e, [0x0067_5E08, 0x0067_5E00, 0x0067_5DF4, 0x0067_5DF0, 0x0067_5DE4], 0x88, true),
        // 0x00570DB0, 0x0056A540, 0x0056EEF0: lanceros, infantería y jinetes.
        0x20 => walker_class(m, e, [0x0067_5BB0, 0x0067_5BAC, 0x0067_5BA0, 0x0067_5B9C, 0x0067_5B90], 0x91, false),
        0x23 => walker_class(m, e, [0x0067_5710, 0x0067_5708, 0x0067_56FC, 0x0067_56F8, 0x0067_56EC], 0x88, false),
        0x24 => walker_class(m, e, [0x0067_5A68, 0x0067_5A64, 0x0067_5A58, 0x0067_5A54, 0x0067_5A48], 0x88, false),
        // 0x00579A00: efecto.
        0x12 => {
            simple_base(m, e);
            m.w32(e + 0x34, 0);
            m.w16(e + 0x38, 0);
            m.w32(e + 0x40, 0);
            m.w32(e + 0x44, 0);
            simple_vt(m, e, [0x0067_6398, 0x0067_6394, 0x0067_6388, 0x0067_637C]);
            m.w8(e + 0x2C, 0xFF);
            m.w8(e + 0x30, 0xFF);
            m.w8(e + 0x31, 0xFF);
        }
        // 0x00564540: máquina de asedio.
        0x15 => {
            simple_base(m, e);
            m.w8(e + 0x30, 0);
            m.w8(e + 0x2C, 0);
            m.w8(e + 0x64, 0);
            simple_vt(m, e, [0x0067_52E0, 0x0067_52DC, 0x0067_52D0, 0x0067_52C4]);
            m.w8(e + 0x31, 0xFF);
        }
        // 0x00573F30: ariete.
        0x16 => {
            simple_base(m, e);
            m.w8(e + 0x34, 0);
            m.w8(e + 0x2C, 0);
            simple_vt(m, e, [0x0067_60C0, 0x0067_60BC, 0x0067_60B0, 0x0067_60A4]);
            m.w8(e + 0x35, 0xFF);
        }
        // 0x00576690: proyectil.
        0x17 => {
            simple_base(m, e);
            m.w8(e + 0x2D, 0);
            m.w32(e + 0x30, 0);
            m.w8(e + 0x36, 0);
            simple_vt(m, e, [0x0067_6248, 0x0067_6244, 0x0067_6238, 0x0067_622C]);
            m.w16(e + 0x34, 1);
            m.w8(e + 0x2C, 0xFF);
        }
        _ => return false,
    }
    true
}
