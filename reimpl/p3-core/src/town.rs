//! Ciudad (`Town`, 0x9F8 bytes) con la misma disposición que el original.
//!
//! La simulación de C04 lee y escribe más de cien campos de la ciudad, varios con anchuras y signos
//! que el original mezcla (bytes que se leen como palabras, palabras que se suman como enteros). Para
//! no perder paridad la ciudad se guarda como sus 0x9F8 bytes y se accede por desplazamiento con los
//! nombres de `research/types/Town.h`. Esto también permite comparar el estado completo con el
//! x86 original tras cada paso.

/// Tamaño de `Town`.
pub const TOWN_SIZE: usize = 0x9F8;
/// Mercancías con existencias y umbrales.
pub const WARES: usize = 24;
/// Instalaciones de la ciudad (`Town+0x840`, 16 bytes cada una).
pub const FACILITIES: usize = 21;

/// Desplazamientos de `Town` (nombres de `research/types/Town.h`).
pub mod off {
    pub const INPUT_SHORTAGE: usize = 0x0;
    pub const STOCK: usize = 0x4;
    pub const BUSINESS_CONSUMPTION: usize = 0x64;
    pub const DAILY_PRODUCTION: usize = 0xC4;
    pub const SHIP_WEAPONS: usize = 0x124;
    pub const PRODUCTION_HISTORY: usize = 0x13C;
    pub const CUTLASSES: usize = 0x2BC;
    pub const TOWN_INDEX: usize = 0x2C0;
    pub const FAMINE_COUNTER: usize = 0x2C3;
    pub const TOWN_FLAGS: usize = 0x2C8;
    pub const FIELD_2CC: usize = 0x2CC;
    pub const CELEBRATION: usize = 0x2D0;
    pub const CITIZENS_TOTAL: usize = 0x2D4;
    /// Ricos, acomodados, pobres y mendigos.
    pub const POPULATION: usize = 0x2D8;
    pub const CITIZENS_OLD: usize = 0x2E8;
    /// Viviendas por grupo (`u16`).
    pub const HOUSING: usize = 0x2F8;
    pub const SATISFACTION: usize = 0x300;
    pub const SATISFACTION_TARGET: usize = 0x308;
    pub const CITIZEN_CONSUMPTION: usize = 0x310;
    pub const STOCK_AT_LAST_TICK: usize = 0x3D0;
    pub const TRADE_FLOW_AVERAGE: usize = 0x430;
    pub const PRODUCTION_VALUES: usize = 0x490;
    pub const PRICE_THRESHOLDS: usize = 0x4F0;
    pub const WEAPON_PRIORITY_WARE: usize = 0x6D0;
    pub const MAYOR_ID: usize = 0x6F1;
    pub const RECENT_EXTRA_TAXES: usize = 0x6F4;
    /// Byte bajo: tasa de impuesto; resto: fecha del último impuesto extraordinario.
    pub const TAX_RATE_AND_STAMP: usize = 0x6F8;
    pub const MONEY: usize = 0x700;
    pub const TRANSACTIONS: usize = 0x704;
    /// Registros de edificios (puntero en el original; aquí vive en `buildings::Site`).
    pub const BUILDING_RECORDS: usize = 0x75C;
    pub const FREE_BUILDING_HEAD: usize = 0x760;
    pub const CONSTRUCTION_HEAD: usize = 0x762;
    pub const PUBLIC_BUILDING_HEAD: usize = 0x764;
    pub const WALL_HEAD: usize = 0x766;
    pub const DEFENSE_HEAD: usize = 0x768;
    pub const BURNING_HEAD: usize = 0x76A;
    pub const BUILDING_RECORD_COUNT: usize = 0x776;
    /// Filas de la rejilla del plano urbano.
    pub const PLAN_ROWS: usize = 0x7A8;
    /// Bytes por fila de la rejilla del plano urbano (`tile_grid_height` en `Town.h`).
    pub const PLAN_STRIDE: usize = 0x7AC;
    pub const PLAN_CELLS: usize = 0x80C;
    pub const MORE_FLAGS: usize = 0x76C;
    pub const CHAPELS: usize = 0x771;
    pub const HOSPITALS: usize = 0x773;
    pub const WEAPONSMITH_SHORTAGE: usize = 0x775;
    pub const FIRST_OFFICE: usize = 0x784;
    pub const DEFENSE_CAPACITY: usize = 0x786;
    pub const TOWERS: usize = 0x788;
    pub const WELLS: usize = 0x789;
    pub const FACILITIES_INSIDE_WALLS: usize = 0x78C;
    pub const FACILITIES_TOTAL: usize = 0x78E;
    pub const STREETS_BUILT: usize = 0x790;
    pub const STREETS_TOTAL: usize = 0x792;
    pub const CHURCH_LEVEL: usize = 0x7A3;
    pub const SHIPYARD_EXPERIENCE: usize = 0x810;
    pub const SHIPYARD_SOMETHING: usize = 0x818;
    pub const CURRENT_SHIP_LEVEL: usize = 0x824;
    pub const MILITIA_UNPAID: usize = 0x835;
    pub const FACILITY: usize = 0x840;
    pub const OUTRIGGER_VALUE: usize = 0x990;
    pub const OUTRIGGER_ID: usize = 0x994;
    pub const MILITIA_UNITS: usize = 0x998;
    pub const MILITIA_RECRUITS: usize = 0x99C;
    pub const MILITIA_LIMIT: usize = 0x9A0;
    pub const BEGGAR_HISTORY: usize = 0x9A4;
}

/// Desplazamientos dentro de una instalación (16 bytes).
pub mod fac {
    pub const EFFICIENCY: usize = 0x0;
    pub const EMPLOYEES: usize = 0x4;
    pub const TYPE: usize = 0x6;
    pub const TOWN: usize = 0x7;
    pub const PRODUCTIVITY: usize = 0x8;
    pub const IDLE: usize = 0xA;
    pub const DISMISSAL_FRACTION: usize = 0xC;
}

/// Una ciudad como sus bytes originales.
#[derive(Clone, PartialEq, Eq)]
pub struct Town(pub Box<[u8; TOWN_SIZE]>);

impl Default for Town {
    fn default() -> Self {
        Town(Box::new([0; TOWN_SIZE]))
    }
}

impl std::fmt::Debug for Town {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Town#{}", self.u8(off::TOWN_INDEX))
    }
}

impl Town {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut town = Town::default();
        town.0.copy_from_slice(bytes);
        town
    }

    pub fn u8(&self, o: usize) -> u8 {
        self.0[o]
    }
    pub fn set_u8(&mut self, o: usize, v: u8) {
        self.0[o] = v;
    }
    pub fn u16(&self, o: usize) -> u16 {
        u16::from_le_bytes([self.0[o], self.0[o + 1]])
    }
    pub fn i8(&self, o: usize) -> i8 {
        self.0[o] as i8
    }
    pub fn i16(&self, o: usize) -> i16 {
        self.u16(o) as i16
    }
    pub fn set_u16(&mut self, o: usize, v: u16) {
        self.0[o..o + 2].copy_from_slice(&v.to_le_bytes());
    }
    pub fn u32(&self, o: usize) -> u32 {
        u32::from_le_bytes(self.0[o..o + 4].try_into().unwrap())
    }
    pub fn i32(&self, o: usize) -> i32 {
        self.u32(o) as i32
    }
    pub fn set_u32(&mut self, o: usize, v: u32) {
        self.0[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    pub fn set_i32(&mut self, o: usize, v: i32) {
        self.set_u32(o, v as u32);
    }
    pub fn add_i32(&mut self, o: usize, d: i32) {
        self.set_i32(o, self.i32(o).wrapping_add(d));
    }
    pub fn add_u16(&mut self, o: usize, d: u16) {
        self.set_u16(o, self.u16(o).wrapping_add(d));
    }
    pub fn f32(&self, o: usize) -> f32 {
        f32::from_bits(self.u32(o))
    }
    pub fn set_f32(&mut self, o: usize, v: f32) {
        self.set_u32(o, v.to_bits());
    }

    pub fn stock(&self, w: usize) -> i32 {
        self.i32(off::STOCK + 4 * w)
    }
    pub fn set_stock(&mut self, w: usize, v: i32) {
        self.set_i32(off::STOCK + 4 * w, v);
    }
    /// Umbral `j` (0..4) de la mercancía `w`.
    pub fn threshold(&self, w: usize, j: usize) -> i32 {
        self.i32(off::PRICE_THRESHOLDS + 16 * w + 4 * j)
    }
    pub fn set_threshold(&mut self, w: usize, j: usize, v: i32) {
        self.set_i32(off::PRICE_THRESHOLDS + 16 * w + 4 * j, v);
    }
    /// Grupo de población: 0 ricos, 1 acomodados, 2 pobres, 3 mendigos.
    pub fn pop(&self, g: usize) -> i32 {
        self.i32(off::POPULATION + 4 * g)
    }
    pub fn set_pop(&mut self, g: usize, v: i32) {
        self.set_i32(off::POPULATION + 4 * g, v);
    }
    pub fn flags(&self) -> u32 {
        self.u32(off::TOWN_FLAGS)
    }
    pub fn set_flags(&mut self, v: u32) {
        self.set_u32(off::TOWN_FLAGS, v);
    }
    pub fn more_flags(&self) -> u32 {
        self.u32(off::MORE_FLAGS)
    }
    pub fn index(&self) -> u8 {
        self.u8(off::TOWN_INDEX)
    }
    /// Desplazamiento del campo `field` de la instalación `f`.
    pub fn fac(f: usize, field: usize) -> usize {
        off::FACILITY + 16 * f + field
    }
    pub fn employees(&self, f: usize) -> u16 {
        self.u16(Town::fac(f, fac::EMPLOYEES))
    }
    pub fn set_employees(&mut self, f: usize, v: u16) {
        self.set_u16(Town::fac(f, fac::EMPLOYEES), v);
    }
    pub fn idle(&self, f: usize) -> u16 {
        self.u16(Town::fac(f, fac::IDLE))
    }
    pub fn set_idle(&mut self, f: usize, v: u16) {
        self.set_u16(Town::fac(f, fac::IDLE), v);
    }
    pub fn efficiency(&self, f: usize) -> i32 {
        self.i32(Town::fac(f, fac::EFFICIENCY))
    }
    pub fn facility_type(&self, f: usize) -> u8 {
        self.u8(Town::fac(f, fac::TYPE))
    }
}
