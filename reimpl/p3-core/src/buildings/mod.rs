//! Edificios de la ciudad: registros de 8 bytes enlazados por listas, rejilla del plano urbano y los
//! recuentos que derivan de ellos. C05 reimplementa el sistema por partes; cada función cita su
//! dirección original.
//!
//! El estado de una ciudad son sus 0x9F8 bytes (`Town`) más dos bloques que el original guarda por
//! puntero: los registros de edificios (`Town+0x75C`) y las celdas del plano (`Town+0x80C`). Aquí van
//! en [`Site`]; los punteros de `Town` quedan a 0.

pub mod advance;
pub mod burn;
pub mod church;
pub mod construction;
pub mod counts;
pub mod defense;
pub mod defpiece;
pub mod donation;
pub mod evaluate;
pub mod fill;
pub mod grid;
pub mod housing;
pub mod initial;
pub mod office;
pub mod place;
pub mod planlife;
pub mod planmask;
pub mod query;
pub mod records;
pub mod remove;
pub mod road;
pub mod save;
pub mod tables;
pub mod transfer;
pub mod type2f;
pub mod wall_task;
pub mod wallplan;
pub mod walls;

pub use grid::Grid;

/// Registros y celdas de una ciudad.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Site {
    /// `Town+0x75C`: `building_record_count` registros de 8 bytes. Bytes 0 y 1: fila y columna;
    /// 2: propietario (0xFF libre); 3: tipo; 4–5: siguiente registro de la lista; 6 y 7: auxiliares.
    pub records: Vec<u8>,
    /// `Town+0x80C`: `PLAN_ROWS * PLAN_STRIDE` celdas.
    pub cells: Vec<u8>,
}

/// Fin de lista en los enlaces de los registros.
pub const NIL: u16 = 0xFFFF;
