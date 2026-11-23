//! Entidades de la vista para las figuras del asalto: fábrica `0x00557D10` (clases portadas), lista de entidades
//! de la vista (`vista+0x3F8`: `0x00568A70`, `0x00568B20`), aparición `class22_spawn_town_entity` (`0x00590610`) y
//! bucle por fotograma `town_entity_list_update_all` (`0x00568C80`).
//!
//! La lista guarda `+4` el vector de entidades por identificador, `+0xC` el mayor identificador, `+0x14` la
//! capacidad, `+0x18` el reloj del fotograma anterior y `+0x1C` el tiempo transcurrido (`vista+0x414`, como mucho
//! 1000 ms). Los objetos nuevos se construyen con los constructores portados ([`super::ctors`]) sobre memoria a cero
//! y sus tablas virtuales son las de [`super::tables::VTABLES`], que hay que tener en memoria ([`install_vtables`]).
use super::ctors::{construct, CLASSES};
use super::tables::VTABLES;
use super::{vcall, view, FigureHost, CLOCK};
use crate::flatmem::Mem;

/// Desplazamiento de la lista de entidades en la vista.
pub const LIST: u32 = 0x3F8;

/// Escribe las tablas virtuales de las clases portadas en sus direcciones del EXE.
pub fn install_vtables(m: &mut dyn Mem) {
    for &(at, words) in VTABLES {
        for (i, &w) in words.iter().enumerate() {
            m.w32(at + 4 * i as u32, w);
        }
    }
}

/// Rango de direcciones que ocupan las tablas virtuales.
pub fn vtable_span() -> std::ops::Range<u32> {
    let lo = VTABLES.iter().map(|v| v.0).min().unwrap_or(0);
    let hi = VTABLES.iter().map(|v| v.0 + 4 * v.1.len() as u32).max().unwrap_or(0);
    lo..hi
}

/// ¿La fábrica de este porte construye la clase?
pub fn has_class(class: u32) -> bool {
    CLASSES.iter().any(|t| u32::from(t.0) == class)
}

/// `sprite_entity_factory` (`0x00557D10`) para las clases de [`CLASSES`]: reserva (`0x0064F7B9`) y construye el
/// objeto. El original no limpia la reserva; aquí la memoria nueva empieza a cero.
pub fn create(m: &mut dyn Mem, h: &mut dyn FigureHost, class: u32) -> u32 {
    let Some(&(_, size, _)) = CLASSES.iter().find(|t| u32::from(t.0) == class) else { return 0 };
    let e = h.alloc(m, size);
    if e == 0 {
        return 0;
    }
    for i in 0..size {
        m.w8(e + i, 0);
    }
    construct(m, class, e);
    e
}

/// `town_entity_list_find_free_id` (`0x00568A70(ecx = lista, desde)`).
pub fn find_free_id(m: &mut dyn Mem, list: u32, from: u32) -> u16 {
    let cap = m.r32(list + 0x14);
    let arr = m.r32(list + 4);
    let mut i = from;
    while i < cap {
        if m.r32(arr.wrapping_add(4 * i)) == 0 {
            return i as u16;
        }
        i += 1;
    }
    if cap != 0 {
        cap as u16
    } else {
        1
    }
}

/// `town_entity_list_set_entity` (`0x00568B20(ecx = lista, entidad, id)`): amplía el vector de 100 en 100.
pub fn set_entity(m: &mut dyn Mem, h: &mut dyn FigureHost, list: u32, e: u32, id: u16) {
    let id = u32::from(id);
    if id >= m.r32(list + 0x14) {
        let cap = id + 0x64;
        m.w32(list + 0x14, cap);
        let new = h.alloc(m, cap * 4);
        let max = m.r32(list + 0xC);
        let old = m.r32(list + 4);
        if max != 0 {
            for i in 0..(max + 1) * 4 {
                let b = m.r8(old.wrapping_add(i));
                m.w8(new + i, b);
            }
        } else {
            m.w32(new, 0);
        }
        let used = max + 1;
        for i in used * 4..cap * 4 {
            m.w8(new + i, 0);
        }
        if old != 0 {
            h.free(m, old);
        }
        m.w32(list + 4, new);
    }
    if id > m.r32(list + 0xC) {
        m.w32(list + 0xC, id);
    }
    let arr = m.r32(list + 4);
    m.w32(arr + 4 * id, e);
}

/// `class22_spawn_town_entity` (`0x00590610(ecx = vista, x, y, clase, variante, rumbo, id, objeto)`): construye el
/// objeto si no viene hecho, le da un identificador libre si no lo trae, lo apunta en la lista y llama a su `+0x40`
/// con `(x, y, rumbo, id, variante)`.
pub fn spawn(m: &mut dyn Mem, h: &mut dyn FigureHost, args: [u32; 7]) -> u32 {
    let [x, y, class, variant, heading, id, object] = args;
    let e = if object != 0 { object } else { create(m, h, class) };
    if e == 0 {
        return 0;
    }
    let list = view(m) + LIST;
    let id = if id as u16 != 0 { id as u16 } else { find_free_id(m, list, 1) };
    set_entity(m, h, list, e, id);
    vcall(m, h, e, 0x40, &[x, y, heading, u32::from(id), variant]);
    e
}

/// `town_entity_list_update_all` (`0x00568C80(ecx = lista)`): tiempo desde el fotograma anterior (nada si es 0,
/// como mucho 1000 ms) y método `+0x14` de cada entidad; la que devuelve 0 se retira (`+4` de la lista).
pub fn update_all(m: &mut dyn Mem, h: &mut dyn FigureHost) {
    let list = view(m) + LIST;
    let now = m.r32(CLOCK);
    let dt = now.wrapping_sub(m.r32(list + 0x18));
    m.w32(list + 0x1C, dt);
    if dt == 0 {
        return;
    }
    if dt > 0x3E8 {
        m.w32(list + 0x1C, 0x3E8);
    }
    let mut id = 1u32;
    while id <= m.r32(list + 0xC) {
        let arr = m.r32(list + 4);
        let e = m.r32(arr + 4 * id);
        if e != 0 && vcall(m, h, e, 0x14, &[]) as u8 == 0 {
            h.release_entity(m, e);
        }
        id += 1;
    }
    let now = m.r32(CLOCK);
    m.w32(list + 0x18, now);
}
