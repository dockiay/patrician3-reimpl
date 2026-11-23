//! Reglas numéricas del original (C02).
//!
//! - `(int)x` pasa siempre por `__ftol` (0x00639C28): fuerza redondeo hacia cero en la palabra de
//!   control x87, hace `fistp qword` y devuelve 64 bits en `EDX:EAX`. Fuera de rango o con NaN, x87
//!   escribe el «entero indefinido» `0x8000000000000000`; `as` de Rust satura, por eso no sirve.
//! - El juego no cambia la palabra de control: la del CRT es `0x27F` (53 bits, al más cercano), así que
//!   la aritmética x87 equivale a `f64`, salvo el rango de exponente extendido de los intermedios.
//! - Las divisiones enteras truncan hacia cero y el resto lleva el signo del dividendo (`/` y `%` de
//!   Rust). Lo que importa es la signatura: una división del original sin signo debe hacerse en `u32`.
//!   `i32::MIN / -1` y el divisor cero producen una excepción en x86: aquí, pánico.

/// Entero indefinido de x87.
pub const X87_INDEFINITE: i64 = i64::MIN;

/// `__ftol`: trunca hacia cero a 64 bits con la semántica de `fistp qword`.
pub fn ftol(x: f64) -> i64 {
    // Rango representable: [-2^63, 2^63). Tras truncar, -2^63 cabe; 2^63 no.
    const LIMIT: f64 = 9_223_372_036_854_775_808.0;
    if x.is_nan() {
        return X87_INDEFINITE;
    }
    let t = x.trunc();
    if !(-LIMIT..LIMIT).contains(&t) {
        X87_INDEFINITE
    } else {
        t as i64
    }
}

/// `(int)x` tal como lo usa el juego: los 32 bits bajos (`EAX`) del resultado de `__ftol`.
pub fn ftol_i32(x: f64) -> i32 {
    ftol(x) as i32
}

/// División con signo de x86 (`idiv`): trunca hacia cero.
pub fn idiv(a: i32, b: i32) -> i32 {
    a.checked_div(b)
        .expect("idiv: división por cero o desbordamiento (excepción en x86)")
}

/// Resto con signo de x86 (`idiv`, `EDX`): signo del dividendo.
pub fn irem(a: i32, b: i32) -> i32 {
    a.checked_rem(b)
        .expect("idiv: división por cero o desbordamiento (excepción en x86)")
}

/// División sin signo (`div`).
pub fn udiv(a: u32, b: u32) -> u32 {
    a.checked_div(b).expect("div: división por cero (excepción en x86)")
}

/// Resto sin signo (`div`, `EDX`).
pub fn urem(a: u32, b: u32) -> u32 {
    a.checked_rem(b).expect("div: división por cero (excepción en x86)")
}

/// Saturación `cmp x, k; jle; mov x, k` con signo (por ejemplo el tope de oro).
pub fn clamp_max_signed(x: i32, k: i32) -> i32 {
    if x > k {
        k
    } else {
        x
    }
}
