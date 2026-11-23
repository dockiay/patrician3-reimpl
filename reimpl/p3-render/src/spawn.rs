//! Aparición por tipo (método virtual `+0x40` de cada figura de la ciudad) y fábrica `0x557D10` (J02).
//!
//! Cada tipo elige la variante de su juego de sprites (`objeto+0x78`) y llama a la colocación `0x559D60` ([`crate::walk::place_entity`]) con ese juego.
//! Paridad: `tests/x86_typed_spawn.rs` (`oracle/x86_typed_spawn.py`).

/// Tabla de juegos de sprites de un tipo (`cantidad`, `punteros`).
#[derive(Clone, Debug, Default)]
pub struct SetTable {
    pub count: u32,
    pub sets: Vec<u32>,
}

/// Qué función de aparición es.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpawnKind {
    /// `0x563500` (tipo 4): siempre elige `rand % cantidad`.
    Always,
    /// `0x567C80`, `0x568440`, `0x568060`, `0x56EBD0`: elige `rand % cantidad` solo si la bandera `escena+0xC594` está puesta; si no, variante 0.
    Flagged,
}

/// Dirección de la función de aparición de cada tipo de entidad que reparte el gestor.
pub fn spawn_address(entity_type: u8) -> Option<u32> {
    Some(match entity_type {
        4 => 0x563500,
        0 => 0x567C80,
        1 => 0x568440,
        2 => 0x568060,
        3 => 0x56EBD0,
        _ => return None,
    })
}

/// Resultado: nueva variante (`+0x78`) y juego de sprites (0 si la variante cae fuera de la tabla).
pub fn choose_variant(kind: SpawnKind, flag: bool, table: &SetTable, current: u8, rand: &mut dyn FnMut() -> u32) -> (u8, u32) {
    let variant = match kind {
        SpawnKind::Always if table.count >= 1 => (rand() % table.count) as u8,
        SpawnKind::Always => current,
        SpawnKind::Flagged if !flag => 0,
        SpawnKind::Flagged if table.count >= 1 => (rand() % table.count) as u8,
        SpawnKind::Flagged => current,
    };
    let set = if u32::from(variant) < table.count { table.sets.get(usize::from(variant)).copied().unwrap_or(0) } else { 0 };
    (variant, set)
}

/// Fila de la fábrica `0x557D10`: tipo, tamaño reservado, constructor y desplazamiento del puntero devuelto.
#[derive(Clone, Copy, Debug)]
pub struct FactoryRow {
    pub entity_type: u8,
    pub size: u8,
    pub ctor: u32,
    pub adjust: u8,
}

/// Tabla completa de la fábrica (37 tipos, `0..=0x24`); el tipo 12 no construye nada (devuelve nulo).
pub const FACTORY: [FactoryRow; 36] = {
    macro_rules! f {
        ($t:expr, $s:expr, $c:expr) => {
            FactoryRow { entity_type: $t, size: $s, ctor: $c, adjust: 0 }
        };
    }
    [
        f!(0, 0x7c, 0x567b00), f!(1, 0x7c, 0x5682c0), f!(2, 0x7c, 0x567ee0), f!(3, 0x7c, 0x56ea50), f!(4, 0x7c, 0x563380),
        f!(5, 0x30, 0x5720e0), f!(6, 0x30, 0x56e4e0), f!(7, 0x44, 0x562ba0), f!(8, 0x94, 0x5775a0), f!(9, 0x8c, 0x5670f0),
        f!(10, 0x8c, 0x5620c0), f!(11, 0x8c, 0x5727a0),
        FactoryRow { entity_type: 13, size: 0x9c, ctor: 0x56ad30, adjust: 4 },
        f!(14, 0x24, 0x568640), f!(15, 0x30, 0x5639f0), f!(16, 0x3c, 0x57e500), f!(17, 0x3c, 0x57bf00), f!(18, 0x48, 0x579a00),
        f!(19, 0x38, 0x57b360), f!(20, 0x38, 0x580240), f!(21, 0x78, 0x564540), f!(22, 0x68, 0x573f30), f!(23, 0x44, 0x576690),
        f!(24, 0x44, 0x569bb0), f!(25, 0x34, 0x57d310), f!(26, 0x3c, 0x5735a0), f!(27, 0x40, 0x5690c0), f!(28, 0x24, 0x573210),
        f!(29, 0x28, 0x576030), f!(30, 0x28, 0x568d80), f!(31, 0x68, 0x56f970), f!(32, 0x94, 0x570db0), f!(33, 0x24, 0x575950),
        f!(34, 0x30, 0x57fba0), f!(35, 0x8c, 0x56a540), f!(36, 0x8c, 0x56eef0),
    ]
};

/// Fila de la fábrica de un tipo (`None` para el 12 y los mayores de `0x24`).
pub fn factory_row(entity_type: u8) -> Option<&'static FactoryRow> {
    FACTORY.iter().find(|r| r.entity_type == entity_type)
}
