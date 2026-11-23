//! Funciones que llama el cuadro de la pantalla de juego (`0x423840`, ver `ui_input`) y que antes iban interceptadas. Mismas convenciones:
//! `ecx` = `this`, `esp` de entrada, locales en `esp - desplazamiento` (`Frame::at`). Los registros guardados con `push` (esi, edi) llevan en
//! las pruebas el valor 0: el oráculo pone a cero los registros antes de cada caso y el porte escribe esos huecos con 0.
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::frame::Frame;

fn vcall(m: &mut dyn Mem, host: &mut dyn OpsHost, obj: u32, off: u32, args: &[u32]) -> u32 {
    let vt = m.r32(obj);
    let f = m.r32(vt.wrapping_add(off));
    host.call(m, f, obj, args)
}

fn ld64(m: &mut dyn Mem, a: u32) -> f64 {
    f64::from_bits(u64::from(m.r32(a)) | (u64::from(m.r32(a.wrapping_add(4))) << 32))
}

fn st64(m: &mut dyn Mem, a: u32, v: f64) {
    let b = v.to_bits();
    m.w32(a, b as u32);
    m.w32(a.wrapping_add(4), (b >> 32) as u32);
}

/// `0x00401300` (`ecx` = vista): pone a 0 `+0x24C` y `+0x260` y recalcula las escalas `+0x268 = [+0x2C] / [0x66A7C8]` y
/// `+0x270 = [+0x30] / [0x66A7C0]` (dobles). Devuelve 0.
pub fn view_reset_scroll_and_scale(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let w = f64::from(m.r32(this.wrapping_add(0x2c)) as i32);
    m.w32(this.wrapping_add(0x24c), 0);
    m.w32(this.wrapping_add(0x260), 0);
    let d = ld64(m, 0x66a7c8);
    st64(m, this.wrapping_add(0x268), w / d);
    let h = f64::from(m.r32(this.wrapping_add(0x30)) as i32);
    st64(m, f.at(8), h);
    let d = ld64(m, 0x66a7c0);
    st64(m, this.wrapping_add(0x270), h / d);
    0
}

/// `0x004C5850` (`ret 4`, `ecx` = lista de punteros: `+4` cuenta con signo, `+8` tabla): elemento `i` o 0 si `i >= cuenta` (con signo).
pub fn ptr_array_get_checked(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let i = m.r32(f.arg(0));
    let n = m.r32(this.wrapping_add(4));
    if (i as i32) < (n as i32) {
        let t = m.r32(this.wrapping_add(8));
        m.r32(t.wrapping_add(i << 2))
    } else {
        0
    }
}

/// `0x00495BB0` (`ecx` = ventana): llama a `0x440F70(ventana, 0)` salvo que sea la ventana activa (`0x6CBD90`) con `+0x94` distinto de
/// `0x66D500`; sin llamada EAX es `+0x94`.
pub fn window_release_unless_active(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let mut eax = 0;
    let mut call = true;
    if this == m.r32(0x6cbd90) {
        eax = m.r32(this.wrapping_add(0x94));
        call = eax == m.r32(0x66d500);
    }
    if call {
        m.w32(f.at(4), 0);
        m.w32(f.at(8), this);
        eax = host.call(m, 0x440f70, 0, &[this, 0]);
    }
    eax
}

/// `0x0059A1E0` (`ecx` = escena): copia `+0xC640/+0xC644` a `+0xC648/+0xC64C`, pone `+0xC5F4` a 0, pasa el rectángulo
/// `[x0, y0, x0 + [+0x2C], y0 + [+0x30]]` (de `+0x14/+0x18`) a `0x4B9650` (ecx = `0x6DA5F0`), llama al método `+0x34` del objeto `+0x530`
/// y alterna el byte `+0x564`.
pub fn scene_refresh_clip_and_toggle(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let o = |off: u32| this.wrapping_add(off);
    m.w32(f.at(0x14), 0);
    let (a, b) = (m.r32(o(0xc640)), m.r32(o(0xc644)));
    m.w32(o(0xc648), a);
    m.w32(o(0xc64c), b);
    let (x0, y0) = (m.r32(o(0x14)), m.r32(o(0x18)));
    let (w, h) = (m.r32(o(0x2c)), m.r32(o(0x30)));
    m.w32(f.at(0x10), x0);
    m.w32(f.at(0xc), y0);
    m.w8(o(0xc5f4), 0);
    m.w32(f.at(0x18), f.at(0x10));
    m.w32(f.at(8), w.wrapping_add(x0));
    m.w32(f.at(4), h.wrapping_add(y0));
    host.call(m, 0x4b9650, 0x6da5f0, &[f.at(0x10)]);
    let obj = m.r32(o(0x530));
    m.w32(f.at(0x18), 0x59a23f); // dirección de retorno de la llamada virtual
    let r = vcall(m, host, obj, 0x34, &[]);
    let v = m.r8(o(0x564)) ^ 1;
    m.w8(o(0x564), v);
    (r & 0xffff_ff00) | u32::from(v)
}

/// `0x0047D1F0` (`ecx` = ventana de lista): libera `+0xA0` (`0x64F7E2`) si no es nulo, pone `+0xA0 = 0` y el mensaje `+0xD4 = 2`, llama a
/// `0x440F70(ventana, 0)` si no es la activa o `+0x94` coincide con `0x66D500`, ejecuta el método `+0x34(0)` del objeto `+0x560`,
/// `0x4869A0` y termina con salto a `0x42C020` (ecx = `0x6CBB40`), cuyo valor devuelve.
pub fn list_screen_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let o = |off: u32| this.wrapping_add(off);
    m.w32(f.at(4), 0);
    let p = m.r32(o(0xa0));
    if p != 0 {
        host.call(m, 0x64f7e2, 0, &[p]);
    }
    m.w32(o(0xa0), 0);
    m.w16(o(0xd4), 2);
    if this != m.r32(0x6cbd90) || m.r32(o(0x94)) == m.r32(0x66d500) {
        host.call(m, 0x440f70, 0, &[this, 0]);
    }
    // restos de pila: argumento 0 y direcciones de retorno de las dos últimas llamadas (método virtual y 0x4869A0)
    m.w32(f.at(8), 0x47d254);
    m.w32(f.at(0xc), 0x47d24d);
    vcall(m, host, o(0x560), 0x34, &[0]);
    host.call(m, 0x4869a0, this, &[]);
    let g = m.r32(0x6cbb40);
    host.call(m, 0x42c020, g, &[])
}

/// `0x0042D230` (SEH, `ecx` = pantalla de juego): guarda `[0x6DFBC4]` en `+0x800`; si coincide con una de las seis velocidades
/// (`[0x66DE7C + 4i] * 1000 / 256` con división hacia cero) crea la cadena formateada (`0x696F54`, `[0x69A934]`, `[0x69A938 + 4i]`),
/// la pasa por valor a `0x42B4C0` y avisa a `0x4605E0(i)` en `[0x6CBDF0]`.
pub fn game_screen_update_speed_text(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x65d1fd);
    m.w32(f.at(0x18), 0);
    m.w32(f.at(0x1c), 0);
    let cur = m.r32(0x6dfbc4);
    m.w32(this.wrapping_add(0x800), cur);
    let mut i: u32 = 0;
    loop {
        let e = (m.r32(0x66de7c + 4 * i) as i32).wrapping_mul(1000);
        let q = e.wrapping_add(((e >> 31) as u32 & 0xff) as i32) >> 8;
        if q as u32 == cur {
            break;
        }
        i += 1;
        if i >= 6 {
            break;
        }
    }
    if i != 6 {
        let local = f.at(0x14);
        let empty = m.r32(0x6c7ccc);
        m.w32(local, empty);
        let a = m.r32(0x69a934);
        let b = m.r32(0x69a938 + 4 * i);
        m.w32(f.at(0x2c), local);
        m.w32(f.at(0x28), 0x696f54);
        m.w32(f.at(0x24), a);
        m.w32(f.at(0x20), b);
        f.seh_state(m, 0);
        host.call(m, 0x64df80, 0, &[local, 0x696f54, a, b]);
        let temp = f.at(0x20);
        m.w32(f.at(0x10), temp);
        m.w32(f.at(0x24), local);
        m.w32(f.at(0x28), 0x42d2cf);
        host.call(m, 0x64efc8, temp, &[local]);
        let t = m.r32(temp);
        m.w32(f.at(0x24), 0x42d2d6);
        host.call(m, 0x42b4c0, this, &[t]);
        m.w32(temp, i);
        m.w32(f.at(0x24), 0x42d2e2);
        let g = m.r32(0x6cbdf0);
        host.call(m, 0x4605e0, g, &[i]);
        f.seh_state(m, 0xffff_ffff);
        m.w32(temp, 0x42d2f3);
        host.call(m, 0x64f253, local, &[]);
    }
    f.seh_leave(m);
}

/// Cuerpo común de `0x428E00` y `0x428F50`: para cada pareja (objeto, texto) pide al objeto su posición (método `+0x60`, dos enteros), mide el
/// texto con la fuente `[+0x2608]` (`0x4C1B20`), pide su rectángulo (método `+0x40`) y pasa el rectángulo `[x − ancho, y + [+0x1264], x, y + alto]`
/// a `0x4B9650`. Locales relativos al `esp` tras el `push esi` (marco `0x28`): `L(x) = at(0x2C - x)`.
fn place_text_pairs(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32, pairs: [(u32, u32); 2]) -> u32 {
    let f = Frame::new(esp);
    let l = |x: u32| f.at(0x2c - x);
    let o = |off: u32| this.wrapping_add(off);
    m.w32(f.at(0x2c), 0);
    let mut r = 0;
    for (ob, so) in pairs {
        vcall(m, host, o(ob), 0x60, &[l(4)]);
        let y = m.r32(o(0x1264)).wrapping_add(m.r32(l(8)));
        let x = m.r32(l(4));
        m.w32(l(0x24), x);
        m.w32(l(0x20), y);
        let s = m.r32(o(so));
        let font = m.r32(o(0x2608)).wrapping_mul(168).wrapping_add(0x6dcd28);
        let w = host.call(m, 0x4c1b20, font, &[s]);
        m.w32(l(0x1c), x.wrapping_sub(w));
        vcall(m, host, font, 0x40, &[l(0x10)]);
        let b = m.r32(l(0x14)).wrapping_add(m.r32(l(0x20)));
        m.w32(l(0x28), b);
        r = host.call(m, 0x4b9650, 0x6da5f0, &[l(0x1c)]);
    }
    r
}

/// `0x00428E00` (`ecx` = pantalla de juego): coloca las dos etiquetas de coordenada X (`+0x1268`/`+0x2620`, `+0x1350`/`+0x261C`).
pub fn game_screen_place_x_labels(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    place_text_pairs(m, host, this, esp, [(0x1268, 0x2620), (0x1350, 0x261c)])
}

/// `0x00428F50` (`ecx` = pantalla de juego): igual para las coordenadas Y (`+0x1438`/`+0x2628`, `+0x1520`/`+0x2624`).
pub fn game_screen_place_y_labels(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    place_text_pairs(m, host, this, esp, [(0x1438, 0x2628), (0x1520, 0x2624)])
}

/// `0x0042B920` (`ecx` = pantalla de juego): con cola de avisos (`+0x488` > 0) y `[+0x7D0] < [0x6DCCF0]`: guarda en `+0x48C` el alto del primer
/// aviso (`+0x168`, método `+0x6C`) y en `+0x490` su alto más el del método `+0x44`; descuenta uno de `+0x488`, recoloca los restantes (`+0x58`
/// con el ancho de su texto + 16, copia de la cadena `+0x13C` a `+0x9C`), desplaza la tabla de plazos `+0x7D4 → +0x7D0` y oculta el último
/// (método `+0xCC(0)`). EAX sin valor definido.
pub fn game_screen_pop_notice(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    let o = |off: u32| this.wrapping_add(off);
    m.w32(f.at(4), 0);
    if m.r8(o(0x488)) == 0 || m.r32(o(0x7d0)) >= m.r32(0x6dccf0) {
        return;
    }
    m.w32(f.at(8), 0);
    m.w32(f.at(0xc), 0);
    let mut esi = o(0x168);
    let h = vcall(m, host, esi, 0x6c, &[]);
    m.w32(o(0x48c), h);
    m.w32(f.at(0x10), 0x42b958);
    let ebx = vcall(m, host, esi, 0x6c, &[]);
    m.w32(f.at(0x10), 0x42b965);
    let a = vcall(m, host, esi, 0x44, &[]);
    m.w32(f.at(0x10), 0x42b96e);
    m.w32(o(0x490), ebx.wrapping_add(a));
    let n = m.r8(o(0x488)).wrapping_sub(1);
    m.w8(o(0x488), n);
    if n != 0 {
        m.w32(f.at(0x10), 0);
        let mut i: u32 = 0;
        loop {
            vcall(m, host, esi, 0xd4, &[]);
            let s = m.r32(esi.wrapping_add(0x13c));
            let font = m.r32(esi.wrapping_add(0x98)).wrapping_mul(168).wrapping_add(0x6dcd28);
            let w = host.call(m, 0x4c1b20, font, &[s]);
            vcall(m, host, esi, 0x58, &[w.wrapping_add(0x10)]);
            let s = m.r32(esi.wrapping_add(0x13c));
            host.call(m, 0x64f390, esi.wrapping_add(0x9c), &[s]);
            vcall(m, host, esi, 0xd4, &[]);
            i += 1;
            esi = esi.wrapping_add(0xa0);
            if (i as i32) >= m.r8(o(0x488)) as i32 {
                break;
            }
        }
    }
    let cnt = u32::from(m.r8(o(0x488)));
    m.w32(f.at(0x10), cnt << 2);
    let tmp: Vec<u8> = (0..cnt << 2).map(|k| m.r8(o(0x7d4 + k))).collect();
    for (k, b) in tmp.into_iter().enumerate() {
        m.w8(o(0x7d0 + k as u32), b);
    }
    let cnt = u32::from(m.r8(o(0x488)));
    m.w32(o(0x7d0 + 4 * cnt), 0);
    let cnt = u32::from(m.r8(o(0x488)));
    m.w32(f.at(0x10), 0);
    vcall(m, host, o(0x168 + 0xa0 * cnt), 0xcc, &[0]);
}

/// `0x004AF740` (`ret 0x14`, `ecx` = botón animado; argumentos: `índice, a, b, c, modo`): ajusta la animación. Con `modo` (byte) = 0 y `+0xC8` ≠ 0
/// continúa la actual (`+0xB8 = min(índice, n−1)`, `+0xC0 = c`, `+0xBC = b`, `+0xC4 = a / (ancho(índice) · c)` si `a > 0`, si no 0; devuelve
/// `+0xC4`); en otro caso la reinicia (`+0xAC`, `+0xA8 = b`, `+0xC8 = c`, `+0xCC = ancho(índice)`, `+0xD0`, `+0xD4 = [0x6DCCF8]`, `+0xB4 = 0`;
/// devuelve `[0x6DCCF8]`). `0x4C5840` da la cuenta y `0x4C5850` el ancho del elemento.
pub fn anim_button_set_animation(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let o = |off: u32| this.wrapping_add(off);
    for k in 1..5 {
        m.w32(f.at(4 * k), 0);
    }
    let (a0, a1, a2, a3) = (m.r32(f.arg(0)), m.r32(f.arg(1)), m.r32(f.arg(2)), m.r32(f.arg(3)));
    let flag = m.r32(f.arg(4)) as u8;
    let list = o(0x94);
    if flag == 0 && m.r32(o(0xc8)) != 0 {
        m.w32(f.at(0x14), 0x4af765);
        let cnt = host.call(m, 0x4c5840, list, &[]).wrapping_sub(1);
        let i = if (a0 as i32) > (cnt as i32) { cnt } else { a0 };
        m.w32(o(0xb8), i);
        m.w32(o(0xc0), a3);
        m.w32(o(0xbc), a2);
        if (a1 as i32) > 0 {
            m.w32(f.at(0x14), i);
            let w = host.call(m, 0x4c5850, list, &[i]);
            let q = (a1 as i32).wrapping_div((w as i32).wrapping_mul(a3 as i32)) as u32;
            m.w32(o(0xc4), q);
            q
        } else {
            m.w32(o(0xc4), 0);
            0
        }
    } else {
        m.w32(f.at(0x14), 0x4af7cd);
        let cnt = host.call(m, 0x4c5840, list, &[]).wrapping_sub(1);
        m.w32(o(0xb8), 0);
        m.w32(o(0xc0), 0);
        m.w32(o(0xc4), 0);
        m.w32(o(0xbc), 0);
        let i = if (a0 as i32) > (cnt as i32) { cnt } else { a0 };
        m.w32(o(0xac), i);
        m.w32(o(0xa8), a2);
        m.w32(o(0xc8), a3);
        m.w32(f.at(0x14), i);
        let w = host.call(m, 0x4c5850, list, &[i]);
        m.w32(o(0xcc), w);
        let q = if (a1 as i32) > 0 && (a3 as i32) > 0 { (a1 as i32).wrapping_div((w as i32).wrapping_mul(a3 as i32)) as u32 } else { 0 };
        m.w32(o(0xd0), q);
        let t = m.r32(0x6dccf8);
        m.w32(o(0xd4), t);
        m.w32(o(0xb4), 0);
        t
    }
}

/// `0x00549040` (`ecx` = pantalla de pedidos): con `+0x928` ≠ 0, avisa a `0x4D8FB0` (ecx = `0x6DD73C`) si `+0x924 < 0xFFFF`, borra de las 8 plazas
/// de `+0x948` (paso 10) las que valen `+0x924` y recorre las que quedan: la primera con id < `[0x6DE4AA]` cuya ficha (`0x5303C0`) tiene `+8 = 0` se
/// envía a `0x548860` y `0x5483D0`; las demás se vacían. Si no hay ninguna marca `+0x92C = +0x914 = 1` y salta a `0x50DC50` (ecx = `0x6DD7A0`).
pub fn order_screen_refresh_slots(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let o = |off: u32| this.wrapping_add(off);
    for k in 1..4 {
        m.w32(f.at(4 * k), 0);
    }
    if m.r32(o(0x928)) == 0 {
        return 0;
    }
    let e = m.r32(o(0x924));
    if e < 0xffff {
        m.w32(f.at(0x10), e);
        m.w32(f.at(0x14), 0x54906b);
        host.call(m, 0x4d8fb0, 0x6dd73c, &[e]);
    }
    for k in 0..8 {
        let p = o(0x948 + 10 * k);
        if u32::from(m.r16(p)) == m.r32(o(0x924)) {
            m.w16(p, 0xffff);
        }
    }
    for s in 0..8u32 {
        let edi = o(0x948 + 10 * s);
        let ax = u32::from(m.r16(edi));
        if ax < u32::from(m.r16(0x6de4aa)) {
            m.w32(f.at(0x10), ax);
            m.w32(f.at(0x14), 0x5490b0);
            let p = host.call(m, 0x5303c0, 0x6de4a0, &[ax]);
            if m.r16(p.wrapping_add(8)) == 0 {
                let dx = u32::from(m.r16(this.wrapping_add((s * 5 + 0x4a6) * 2)));
                let cx = u32::from(m.r16(this.wrapping_add(s * 10 + 0x948)));
                let ax1 = u32::from(m.r16(this.wrapping_add(s * 10 + 0x94a)));
                m.w32(f.at(0x10), dx);
                m.w32(f.at(0x14), ax1);
                m.w32(f.at(0x18), cx);
                m.w32(f.at(0x1c), 0x54910d);
                host.call(m, 0x548860, this, &[cx, ax1, dx]);
                let dx2 = u32::from(m.r16(this.wrapping_add(s * 10 + 0x950)));
                let al = u32::from(m.r8(this.wrapping_add(s * 10 + 0x94e)));
                m.w32(f.at(0x10), dx2);
                m.w32(f.at(0x14), al);
                m.w32(f.at(0x18), 0x549127);
                return host.call(m, 0x5483d0, this, &[al, dx2]);
            }
            m.w16(edi, 0xffff);
        }
    }
    m.w32(o(0x92c), 1);
    m.w32(o(0x914), 1);
    host.call(m, 0x50dc50, 0x6dd7a0, &[])
}

/// `0x0042BE40` (`ecx` = pantalla de juego): como `0x42B920` para la segunda cola de avisos (`+0x7B4` cuenta, `+0x494` objetos, `+0x7E8` plazos):
/// el local guarda el resultado del método `+0x44` del objeto `[0x6E51AC]`; cada aviso restante se recoloca con `+0x58` (ancho del texto + 16) y
/// `+0x64(local − alto, +0x70(), 0x64)`. EAX sin valor definido.
pub fn game_screen_pop_notice_b(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    let o = |off: u32| this.wrapping_add(off);
    m.w32(f.at(8), 0);
    m.w32(f.at(0xc), 0x42be4f);
    let g = m.r32(0x6e51ac);
    let r = vcall(m, host, g, 0x44, &[]);
    m.w32(f.at(4), r);
    if m.r8(o(0x7b4)) == 0 || m.r32(o(0x7e8)) >= m.r32(0x6dccf0) {
        return;
    }
    for k in 3..6 {
        m.w32(f.at(4 * k), 0);
    }
    let mut esi = o(0x494);
    let h = vcall(m, host, esi, 0x6c, &[]);
    m.w32(o(0x7b8), h);
    let b = vcall(m, host, esi, 0x6c, &[]);
    let a = vcall(m, host, esi, 0x44, &[]);
    m.w32(o(0x7bc), b.wrapping_add(a));
    let n = m.r8(o(0x7b4)).wrapping_sub(1);
    m.w8(o(0x7b4), n);
    if n != 0 {
        let mut i: u32 = 0;
        loop {
            vcall(m, host, esi, 0xd4, &[]);
            let s = m.r32(esi.wrapping_add(0x13c));
            let font = m.r32(esi.wrapping_add(0x98)).wrapping_mul(168).wrapping_add(0x6dcd28);
            let w = host.call(m, 0x4c1b20, font, &[s]);
            vcall(m, host, esi, 0x58, &[w.wrapping_add(0x10)]);
            let t = vcall(m, host, esi, 0x70, &[]);
            let u = vcall(m, host, esi, 0x44, &[]);
            let v = m.r32(f.at(4)).wrapping_sub(u);
            vcall(m, host, esi, 0x64, &[v, t, 0x64]);
            let s = m.r32(esi.wrapping_add(0x13c));
            host.call(m, 0x64f390, esi.wrapping_add(0x9c), &[s]);
            vcall(m, host, esi, 0xd4, &[]);
            i += 1;
            esi = esi.wrapping_add(0xa0);
            if (i as i32) >= m.r8(o(0x7b4)) as i32 {
                break;
            }
        }
    }
    let cnt = u32::from(m.r8(o(0x7b4)));
    let tmp: Vec<u8> = (0..cnt << 2).map(|k| m.r8(o(0x7ec + k))).collect();
    for (k, b) in tmp.into_iter().enumerate() {
        m.w8(o(0x7e8 + k as u32), b);
    }
    let cnt = u32::from(m.r8(o(0x7b4)));
    m.w32(o(0x7e8 + 4 * cnt), 0);
    let cnt = u32::from(m.r8(o(0x7b4)));
    m.w32(f.at(0x18), 0);
    m.w32(f.at(0x1c), 0x42bfa4);
    m.w32(f.at(0x20), o(0x7e8));
    m.w32(f.at(0x24), 0x42bf68);
    vcall(m, host, o(0x494 + 0xa0 * cnt), 0xcc, &[0]);
}
