//! Modelo de precios para el banco de comparación B06 (`reimpl/bench`): lee casos por la entrada estándar y escribe el resultado de cada uno.
//!
//! Entrada, una línea por caso: `mercancía comercia(0/1) existencias t0 t1 t2 t3 marcada cantidad dificultad_bits`.
//! Salida: `compra venta`.

use p3_core::economy::prices::{buy_price, sell_price, Market};
use std::io::{BufRead, Write};

fn main() {
    let stdin = std::io::stdin();
    let mut out = std::io::stdout().lock();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let f: Vec<i64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let ware = f[0] as usize;
        let market = Market {
            trading: f[1] != 0,
            stock: f[2] as i32,
            thresholds: [f[3] as i32, f[4] as i32, f[5] as i32, f[6] as i32],
            marked_ware: f[7] as u8,
        };
        let amount = f[8] as i32;
        let difficulty = f32::from_bits(f[9] as u32);
        writeln!(
            out,
            "{} {}",
            buy_price(ware, &market, amount),
            sell_price(ware, &market, amount, difficulty)
        )
        .unwrap();
    }
}
