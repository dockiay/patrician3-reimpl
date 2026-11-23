//! Existencias objetivo de las oficinas de la IA (C06): `merchant_ai_update_office_trade_stock_targets`
//! (0x004F4F80).

/// Mercancías con redondeo por centenas (`0x00672C14` distinto de cero).
const BY_HUNDRED: [bool; 24] = [
    false, false, false, true, true, true, true, true, true, true, true, false, true, true, false, true, false, false,
    true, false, true, true, true, true,
];

/// Una oficina como la lee esta función.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StockOffice {
    /// `+0x2C6`: ciudad.
    pub town: u8,
    /// `+0x2C8`: siguiente oficina del comerciante.
    pub next: u16,
    /// `+0x64`: cantidad que quiere de cada mercancía.
    pub wants: [i32; 24],
    /// `+0x354`: existencias objetivo (salida).
    pub targets: [i32; 24],
    /// `+0x2D6`: marcas.
    pub marks: u8,
}

/// División por una constante mágica de `imul` (como la emite el compilador): `(value * magic) >> (32 + shift)` más
/// el bit de signo.
fn magic_div(value: i32, magic: i32, shift: u32) -> i32 {
    let high = ((value as i64 * magic as i64) >> 32) as i32 >> shift;
    high.wrapping_add(((high as u32) >> 31) as i32)
}

/// `town_flags` es el dword `+0x2C8` de cada ciudad (40 entradas como mínimo).
pub fn update_office_trade_stock_targets(offices: &mut [StockOffice], head: u16, town_flags: &[u32]) {
    let mut at = head as usize;
    while at < offices.len() {
        let flags = town_flags[offices[at].town as usize];
        let multiplier: i32 = if flags >> 8 & 0xE != 0 {
            0x23
        } else if flags & 0x400_0018 != 0 {
            0x1C
        } else {
            0xE
        };
        for w in 0..24 {
            let wanted = offices[at].wants[w];
            offices[at].targets[w] = if wanted > 0 {
                let value = wanted.wrapping_mul(multiplier);
                if BY_HUNDRED[w] {
                    magic_div(value, 0x51EB_851F, 6).wrapping_add(1).wrapping_mul(200)
                } else {
                    magic_div(value, 0x1062_4DD3, 7).wrapping_add(1).wrapping_mul(2000)
                }
            } else {
                0
            };
        }
        offices[at].marks |= 0x10;
        at = offices[at].next as usize;
    }
}
