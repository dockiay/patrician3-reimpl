//! Confirmaciones de diálogos que envían una operación por la cola `0x54AA70` (oráculo `oracle/x86_dialogs.py`, prueba `tests/x86_dialogs.rs`).
//!
//! Marco común de los diálogos de asedio (`0x5F2E90`, `0x5F2F90`, `0x5F30B0`: `sub esp,0x24; push esi`): el rectángulo del diálogo en
//! `E-0x24..E-0x14` y el registro de operación (5 palabras, `0x535740`) en `E-0x14..E`. El valor residual de ECX que el original empuja como
//! argumento de `0x6365B0` (solo se usa el byte bajo) se enmascara a 8 bits en el gancho del oráculo.

use crate::economy::ops_misc::OpsHost;
use crate::economy::small_fns::operation_record_init;
use crate::flatmem::Mem;
use crate::frame::{peek, Frame};

const G_PLAYER: u32 = 0x006d_fc14;
const G_TICK: u32 = 0x006d_e4b4;
const G_SIEGES: u32 = 0x006d_f270;
const G_DIRTY: u32 = 0x006d_a5f0;
const G_OPS: u32 = 0x006d_f2f0;
/// Tamaño del registro de asedio por ciudad: `idx * 0x1024`.
const SIEGE_STRIDE: u32 = 0x1024;

/// Rectángulo del diálogo (`this+0x14/+0x18` origen, `+0x2C/+0x30` tamaño) en `rect` y aviso al gestor de zonas sucias `0x4B9650`.
fn dirty_dialog_rect(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, rect: u32) {
    let x = m.r32(this + 0x14);
    let y = m.r32(this + 0x18);
    let w = m.r32(this + 0x2c);
    let h = m.r32(this + 0x30);
    m.w32(rect, x);
    m.w32(rect + 4, y);
    m.w32(rect + 8, w.wrapping_add(x));
    m.w32(rect + 0xc, h.wrapping_add(y));
    let args = peek(m, &[rect], rect, 4);
    host.call(m, 0x4b9650, G_DIRTY, &args);
}

fn send(m: &mut dyn Mem, host: &mut dyn OpsHost, rec: u32) {
    let args = peek(m, &[rec], rec, 5);
    host.call(m, 0x54aa70, G_OPS, &args);
}

/// `0x005F2E90` `siege_dialog_confirm_deliver_goods` (thiscall, `this+0x180` botón, `this+0x15E8` ciudad sitiada, `this+0x1614` estado del
/// diálogo). Con el botón pulsado: si el jugador tiene un pedido de suministros pendiente (`0x6365B0`) envía la operación `0x8D` con
/// `[ciudad, jugador, día de entrega]` (byte 6 del pedido, más `rand() % 0x1C00 + tick + 0x2A00` sin el byte bajo); termina con el estado 1.
pub fn siege_dialog_confirm_deliver_goods(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    if host.call(m, 0x4c78b0, this + 0x180, &[]) as u8 == 0 {
        return;
    }
    let (rect, rec) = (f.at(0x24), f.at(0x14));
    operation_record_init(m, rec);
    dirty_dialog_rect(m, host, this, rect);
    let city = u32::from(m.r8(this + 0x15e8));
    let base = m.r32(G_SIEGES).wrapping_add(city.wrapping_mul(SIEGE_STRIDE));
    let player = m.r32(G_PLAYER);
    if host.call(m, 0x6365b0, base, &[player & 0xff]) != 0 {
        m.w32(rec + 4, city);
        m.w32(rec + 8, player);
        m.w32(rec, 0x8d);
        let node = host.call(m, 0x6365b0, base, &[player & 0xff]);
        let tick = m.r32(G_TICK);
        let b6 = u32::from(m.r8(node + 6));
        m.w32(rec + 0xc, b6);
        let r = host.call(m, 0x639c03, 0, &[]) as i32;
        let rem = r.wrapping_rem(0x1c00) as u32;
        m.w32(rec + 0x10, rem.wrapping_add(tick).wrapping_add(0x2a00) & !0xff);
        send(m, host, rec);
    }
    m.w8(this + 0x1614, 1);
}

/// Parte común de `0x5F2F90`/`0x5F30B0` tras el rectángulo: aviso `0x5ABF70`, ¿asedio activo (`0x633AE0`)? y objeto de escena del registro
/// (`+8`). Devuelve `None` si el asedio ya no está activo (estado 9, ya escrito) o el objeto con escena en espera (`+0x3A`, estado 9).
fn siege_scene(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, rect: u32) -> Option<(u32, u32)> {
    dirty_dialog_rect(m, host, this, rect);
    let ecx = m.r32(0x6e5514);
    host.call(m, 0x5abf70, ecx, &[]);
    let city = u32::from(m.r8(this + 0x15e8));
    let base = m.r32(G_SIEGES).wrapping_add(city.wrapping_mul(SIEGE_STRIDE));
    let mut ok = host.call(m, 0x633ae0, base, &[]) as u8 != 0;
    let mut scene = 0;
    if ok {
        scene = m.r32(base.wrapping_add(8));
        if m.r16(scene + 0x3a) != 0 {
            ok = false;
        }
    }
    if !ok {
        m.w8(this + 0x1614, 9);
        return None;
    }
    Some((u32::from(m.r8(this + 0x15e8)), scene))
}

/// Registro `0x90` (orden al asedio) para la ciudad `city`: byte `+0xB` = 1 (rendición) o `+0xC` = 1 (ataque).
fn send_siege_order(m: &mut dyn Mem, host: &mut dyn OpsHost, rec: u32, city: u32, flag_at: u32) {
    m.w8(rec + 8, city as u8);
    m.w32(rec, 0x90);
    m.w32(rec + 4, 0);
    m.w8(rec + 0xc, 0);
    m.w8(rec + 0xa, 0);
    m.w8(rec + 9, 0);
    m.w8(rec + 0xb, 0);
    m.w8(flag_at, 1);
    send(m, host, rec);
}

/// `0x005F2F90` `siege_dialog_confirm_order_attack`: orden `0x90` con la bandera de ataque (`rec+0xC`) si el asedio sigue activo y la
/// escena no está en espera; objeto de escena con `+0x78 != 0` → estado 1; con `+0x42 != 0` → estado 5; si no, estado 3 y se envía.
pub fn siege_dialog_confirm_order_attack(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    if host.call(m, 0x4c78b0, this + 0x180, &[]) as u8 == 0 {
        return;
    }
    let (rect, rec) = (f.at(0x24), f.at(0x14));
    operation_record_init(m, rec);
    let Some((city, scene)) = siege_scene(m, host, this, rect) else { return };
    if m.r8(scene + 0x78) != 0 {
        m.w8(this + 0x1614, 1);
        return;
    }
    if m.r16(scene + 0x42) != 0 {
        m.w8(this + 0x1614, 5);
        return;
    }
    m.w8(this + 0x1614, 3);
    send_siege_order(m, host, rec, city, rec + 0xc);
}

/// `0x005F30B0` `siege_dialog_confirm_surrender`: como el ataque pero con la bandera de rendición (`rec+0xB`) y comprobando `+0x79`;
/// tras enviar fija `this+0x94 = 1`.
pub fn siege_dialog_confirm_surrender(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    if host.call(m, 0x4c78b0, this + 0x180, &[]) as u8 == 0 {
        return;
    }
    let (rect, rec) = (f.at(0x24), f.at(0x14));
    operation_record_init(m, rec);
    let Some((city, scene)) = siege_scene(m, host, this, rect) else { return };
    if m.r8(scene + 0x79) != 0 {
        m.w8(this + 0x1614, 1);
        return;
    }
    m.w8(this + 0x1614, 3);
    send_siege_order(m, host, rec, city, rec + 0xb);
    m.w8(this + 0x94, 1);
}

/// `0x0048D420` `ship_side_menu_confirm_icon_choice` (thiscall, un argumento byte: icono elegido): lo guarda en `this+0x5828`, envía la
/// operación `0x98` con `[jugador (0x6E59CC), icono, fila (u16 +0x138 del objeto `this+0x3858` >> 6), máscara de 32 bits (1 << (valor & 0x1F)) en
/// la mitad baja o alta según el bit 5]`, guarda el icono anterior en `this+0x5829`, pone `this+0x5828 = 0xFF` y repinta (`0x494580`).
pub fn ship_side_menu_confirm_icon_choice(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    let icon = m.r8(f.arg(0));
    let rec = f.at(0x14);
    m.w8(this + 0x5828, icon);
    operation_record_init(m, rec);
    let obj = m.r32(this + 0x3858);
    let who = m.r8(0x6e59cc);
    let v = u32::from(m.r16(obj + 0x138));
    let icon = m.r8(this + 0x5828);
    m.w8(rec + 4, who);
    m.w32(rec, 0x98);
    m.w8(rec + 5, icon);
    m.w8(rec + 6, (v >> 6) as u8);
    let mask = 1u32 << (v & 0x1f);
    if v & 0x20 != 0 {
        m.w32(rec + 0x10, 0);
        m.w32(rec + 8, mask);
    } else {
        m.w32(rec + 8, 0);
        m.w32(rec + 0xc, mask);
    }
    send(m, host, rec);
    let prev = m.r8(this + 0x5828);
    m.w8(this + 0x5828, 0xff);
    m.w8(this + 0x5829, prev);
    host.call(m, 0x494580, this, &[]);
}
