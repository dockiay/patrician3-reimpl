//! Ordenaciones de la lista del diálogo de elección (`choice_list_dialog`, oráculo `oracle/x86_dialog_lists.py`, prueba `tests/x86_dialog_lists.rs`).
//!
//! La lista vive en `this+0xAC` (bytes: índice de mercancía, ciudad o comerciante), con un recuento `u16` en `this+0xA6` y, en paralelo, un
//! vector de `i32` en `this+0xC0` (valor mostrado de cada fila). Burbuja con paso decreciente: cada pasada compara filas contiguas y
//! cambia las dos listas a la vez; se repite mientras la pasada anterior cambió algo.

use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use std::cmp::Ordering;

const G_WORLD: u32 = 0x006d_e4a0;
const GOOD_NAMES: u32 = 0x006c_0bb0;

fn count(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r16(this + 0xa6))
}

/// Compara dos cadenas C byte a byte con signo (`cmp dl, bl; jl/jg`) hasta el primer byte distinto o el terminador.
fn cmp_cstr(m: &mut dyn Mem, mut a: u32, mut b: u32) -> Ordering {
    loop {
        let x = m.r8(a) as i8;
        let y = m.r8(b) as i8;
        match x.cmp(&y) {
            Ordering::Equal if x != 0 => {
                a = a.wrapping_add(1);
                b = b.wrapping_add(1);
            }
            o => return o,
        }
    }
}

/// Cambia las filas `i` e `i+1` de la lista de bytes (`this+0xAC`) y, si existe, del vector de valores (`this+0xC0`).
fn swap_rows(m: &mut dyn Mem, this: u32, i: u32, check_values: bool) {
    let vals = m.r32(this + 0xc0);
    if !check_values || vals != 0 {
        let a = m.r32(vals.wrapping_add(i * 4));
        let b = m.r32(vals.wrapping_add(i * 4 + 4));
        m.w32(vals.wrapping_add(i * 4), b);
        let vals = m.r32(this + 0xc0);
        m.w32(vals.wrapping_add(i * 4 + 4), a);
    }
    let ids = m.r32(this + 0xac);
    let (p, q) = (m.r8(ids.wrapping_add(i)), m.r8(ids.wrapping_add(i + 1)));
    m.w8(ids.wrapping_add(i), q);
    let ids = m.r32(this + 0xac);
    m.w8(ids.wrapping_add(i + 1), p);
}

fn bubble(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, check_values: bool, mut swap_wanted: impl FnMut(&mut dyn Mem, &mut dyn OpsHost, u32) -> bool) {
    let mut n = count(m, this);
    if n == 0 {
        return;
    }
    loop {
        n = n.wrapping_sub(1);
        if n == 0 {
            return;
        }
        let mut swapped = false;
        let mut i = 0;
        loop {
            if swap_wanted(m, host, i) {
                swap_rows(m, this, i, check_values);
                swapped = true;
            }
            i += 1;
            if i >= n {
                break;
            }
        }
        if !swapped {
            return;
        }
    }
}

/// `0x00471EA0` `choice_list_dialog_sort_by_value_desc`: de mayor a menor valor (`i32` con signo).
pub fn choice_list_dialog_sort_by_value_desc(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    bubble(m, host, this, false, |m, _h, i| {
        let vals = m.r32(this + 0xc0);
        let a = m.r32(vals.wrapping_add(i * 4)) as i32;
        let b = m.r32(vals.wrapping_add(i * 4 + 4)) as i32;
        a < b
    });
}

/// `0x00471F10` `choice_list_dialog_sort_goods_by_name`: por nombre de mercancía (tabla de punteros `0x6C0BB0`), ascendente.
pub fn choice_list_dialog_sort_goods_by_name(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    bubble(m, host, this, true, |m, _h, i| {
        let ids = m.r32(this + 0xac);
        let a = u32::from(m.r8(ids.wrapping_add(i)));
        let b = u32::from(m.r8(ids.wrapping_add(i + 1)));
        let pa = m.r32(GOOD_NAMES + a * 4);
        let pb = m.r32(GOOD_NAMES + b * 4);
        cmp_cstr(m, pa, pb) == Ordering::Greater
    });
}

/// `0x004720D0` `choice_list_dialog_sort_merchants_by_name`: por nombre de pila (`merchant+0xE4`) y, si empatan, por apellido (`+0xE8`).
pub fn choice_list_dialog_sort_merchants_by_name(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    bubble(m, host, this, true, |m, host, i| {
        let ids = m.r32(this + 0xac);
        let a = u32::from(m.r8(ids.wrapping_add(i)));
        let ma = host.call(m, 0x5303c0, G_WORLD, &[a]);
        let ids = m.r32(this + 0xac);
        let b = u32::from(m.r8(ids.wrapping_add(i + 1)));
        let mb = host.call(m, 0x5303c0, G_WORLD, &[b]);
        let (fa, fb) = (m.r32(ma + 0xe4), m.r32(mb + 0xe4));
        match cmp_cstr(m, fa, fb) {
            Ordering::Less => false,
            Ordering::Greater => true,
            Ordering::Equal => {
                let (la, lb) = (m.r32(ma + 0xe8), m.r32(mb + 0xe8));
                cmp_cstr(m, la, lb) == Ordering::Greater
            }
        }
    });
}
