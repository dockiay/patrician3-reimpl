//! Calendario derivado del tick: `update_calendar` (0x005310D0).
//!
//! El mundo guarda en `+0` el día del mes, `+1` el mes, `+2` el año (contador desde 0) y `+4` el día
//! del año. Un día son 256 ticks, un año 365 días (93.440 ticks) y no hay bisiestos.

/// Ticks por día (`tick >> 8`).
pub const TICKS_PER_DAY: u32 = 0x100;
/// Ticks por semana; la semana está alineada con el tick 0.
pub const TICKS_PER_WEEK: u32 = 0x700;
/// Ticks por año de 365 días.
pub const TICKS_PER_YEAR: u32 = 0x16D00;
/// Tabla acumulada de meses en 0x00672D78.
pub const MONTH_START: [u16; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];

/// Campos de calendario del mundo (`GameWorld+0..+5`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Calendar {
    /// `+0`: día del mes, desde 1.
    pub day: u8,
    /// `+1`: mes, desde 0.
    pub month: u8,
    /// `+2`: año desde el tick 0.
    pub year: u16,
    /// `+4`: día del año, desde 0.
    pub day_of_year: u16,
}

impl Calendar {
    /// Recalcula el calendario para `tick`, como `update_calendar`.
    ///
    /// El original busca el mes partiendo del mes guardado (sube o baja paso a paso), así que el
    /// resultado solo depende del tick mientras el mes de partida esté entre 0 y 11, que es lo único
    /// que producen la construcción del mundo y este mismo método.
    pub fn update(&mut self, tick: u32) {
        // División mágica 0x6719F361: igual a tick / 93440 en todo el rango de u32.
        self.year = (tick / TICKS_PER_YEAR) as u16;
        let doy = ((tick >> 8) % 365) as u16;
        self.day_of_year = doy;
        assert!(self.month < 12, "mes de partida fuera de la tabla del original");
        let mut m = self.month as usize;
        if doy >= MONTH_START[m + 1] {
            while doy >= MONTH_START[m + 1] {
                m += 1;
            }
        } else {
            while doy < MONTH_START[m] {
                m -= 1;
            }
        }
        self.month = m as u8;
        // Resta de bytes bajos, como el original (`sub al, bl; inc al`).
        self.day = (doy as u8).wrapping_sub(MONTH_START[m] as u8).wrapping_add(1);
    }

    /// Calendario recién calculado desde el mes 0.
    pub fn for_tick(tick: u32) -> Self {
        let mut calendar = Calendar::default();
        calendar.update(tick);
        calendar
    }
}

/// `true` si el tick abre una semana (`tick % 0x700 == 0`), bandera de `charge_daily_taxes_and_close_week`.
pub fn is_week_start(tick: u32) -> bool {
    tick.is_multiple_of(TICKS_PER_WEEK)
}
