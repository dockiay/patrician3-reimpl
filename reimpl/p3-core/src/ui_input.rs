//! Entrada de la interfaz (ratón, arrastre, cursor y teclado de las ventanas) con paridad x86: enrutado de eventos de `WindowBase` y
//! `ScreenWindow`, hijos con captura y arrastre, cursor del sistema y traducción de teclas.
//!
//! Convenciones (ver `FRAME_SEH.md`): cada función recibe `ecx` (`this`) y el `esp` de entrada; los argumentos de pila están en
//! `esp+4..` y los locales en `esp - desplazamiento`, igual que el original. Las llamadas virtuales (`call [vtable+off]`) y las
//! funciones del original que no se portan aquí van por [`OpsHost`]; la dirección que se pasa es la que lee el original de la tabla.
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;
use crate::frame::Frame;

fn vcall(m: &mut dyn Mem, host: &mut dyn OpsHost, obj: u32, off: u32, args: &[u32]) -> u32 {
    let vt = m.r32(obj);
    let f = m.r32(vt.wrapping_add(off));
    host.call(m, f, obj, args)
}

/// Dirección del hijo `i` de `this` (tabla `+0x94`, entradas de 16 bytes: objeto, x, y, ancho).
fn child_slot(m: &mut dyn Mem, this: u32, i: u32) -> u32 {
    m.r32(this.wrapping_add(0x94)).wrapping_add(i << 4)
}

fn child_obj(m: &mut dyn Mem, this: u32, i: u32) -> u32 {
    let s = child_slot(m, this, i);
    m.r32(s)
}

/// `0x004B1720` (`ret 8`, `ecx` = ventana; argumento 1 = modo con signo): según el modo (0 → 3, 1 → 2, 2 → 1, −2/−3 → 3) fija el estado
/// de ratón de la ventana con el método virtual `+0x2C` si el actual (`+0x28`) es distinto. Con la ventana inactiva (`+0xAC` = 0) y
/// `+0x41` activo solo fija el estado 1 si `+0xC4` es falso; con `+0x41` = 0 pone el estado 0. EAX sin valor definido.
pub fn window_handle_mouse_event_mode(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    let mode = m.r32(f.arg(1));
    let active = vcall(m, host, this, 0xac, &[]) as u8;
    let b41 = m.r8(this.wrapping_add(0x41));
    let set = |m: &mut dyn Mem, host: &mut dyn OpsHost, st: u32| {
        let cur = vcall(m, host, this, 0x28, &[]);
        if cur != st {
            vcall(m, host, this, 0x2c, &[st]);
        }
    };
    if active != 0 {
        if b41 == 0 {
            let cur = vcall(m, host, this, 0x28, &[]);
            if cur != 0 {
                vcall(m, host, this, 0x2c, &[0]);
            }
            return;
        }
        if mode > 2 {
            if mode == 0xffff_fffd || mode == 0xffff_fffe {
                set(m, host, 3);
            }
        } else if mode == 2 {
            set(m, host, 1);
        } else if mode == 0 {
            set(m, host, 3);
        } else {
            set(m, host, 2);
        }
    } else if b41 == 0 {
        let cur = vcall(m, host, this, 0x28, &[]);
        if cur != 0 {
            vcall(m, host, this, 0x2c, &[0]);
        }
    } else {
        let pressed = vcall(m, host, this, 0xc4, &[]) as u8;
        if pressed != 0 {
            return;
        }
        set(m, host, 1);
    }
}

/// `0x004B2280` (`ret 4`): copia el punto `{x, y}` del argumento a un local `{x, y, 0}` y devuelve lo que devuelva el método virtual
/// `+0x24` (que recibe la dirección del local).
pub fn window_hit_test_point_copy(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let pt = m.r32(f.arg(0));
    let local = f.at(0xc);
    let x = m.r32(pt);
    let y = m.r32(pt.wrapping_add(4));
    m.w32(local.wrapping_add(8), 0);
    m.w32(local.wrapping_add(4), y);
    m.w32(local, x);
    vcall(m, host, this, 0x24, &[local])
}

/// `0x004B22B0` (`ret 4`): `true` si el punto (argumento) cae dentro del rectángulo de la ventana: obtiene el origen (`+0x60`) y el
/// tamaño (`+0x40`) con métodos virtuales y compara con signo; 0 si la ventana no es visible (`+0xC8`).
pub fn window_hit_test_point(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    if vcall(m, host, this, 0xc8, &[]) as u8 == 0 {
        return 0;
    }
    let origin = f.at(0x18);
    vcall(m, host, this, 0x60, &[origin]);
    let size = f.at(0xc);
    vcall(m, host, this, 0x40, &[size]);
    let pt = m.r32(f.arg(0));
    let x = m.r32(pt).wrapping_sub(m.r32(origin)) as i32;
    let y = m.r32(pt.wrapping_add(4)).wrapping_sub(m.r32(origin.wrapping_add(4))) as i32;
    let w = m.r32(size) as i32;
    let h = m.r32(size.wrapping_add(4)) as i32;
    u32::from(x >= 0 && x < w && y >= 0 && y < h)
}

/// `0x004B2320` (`ret 4`): pulsación sobre la ventana. Si no estaba pulsada (`+0x43` = 0) y es activable (`+0x38`) con sonido válido
/// (`+0x7C` ≠ −1) y gestor de sonido (`0x6DCCDC`), reproduce el sonido (`0x4C45A0`); después marca `+0x43` y fija el estado 3
/// (`+0x2C`). Devuelve 1 en AL.
pub fn window_on_mouse_press(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _esp: u32) -> u32 {
    if m.r8(this.wrapping_add(0x43)) == 0 && vcall(m, host, this, 0x38, &[]) as u8 != 0 {
        let snd = m.r32(this.wrapping_add(0x7c));
        if snd != 0xffff_ffff {
            let mgr = m.r32(0x6dccdc);
            if mgr != 0 {
                host.call(m, 0x4c45a0, mgr, &[snd]);
            }
        }
    }
    m.w8(this.wrapping_add(0x43), 1);
    vcall(m, host, this, 0x2c, &[3]);
    1
}

/// `0x004B2360` (`ret 4`): suelta el botón. Devuelve (AL) el estado anterior de `+0x43`; con `+0xC4` verdadero y argumento no nulo fija
/// el estado 3, si no el 1.
pub fn window_on_mouse_release(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let prev = m.r8(this.wrapping_add(0x43));
    m.w8(this.wrapping_add(0x43), 0);
    let hovered = vcall(m, host, this, 0xc4, &[]) as u8;
    let arg = m.r32(f.arg(0));
    if hovered != 0 && arg != 0 {
        vcall(m, host, this, 0x2c, &[3]);
    } else {
        vcall(m, host, this, 0x2c, &[1]);
    }
    u32::from(prev)
}

/// `0x004B6B70` (`ret 8`): manejador vacío de `ScreenWindow`.
pub fn screen_window_on_mouse_event_noop(_m: &mut dyn Mem, _host: &mut dyn OpsHost, _this: u32, _esp: u32) {}

/// `0x004B6B80` (`ret 0xC`): el ratón sale de la ventana. Pone `+0x11C` a −1 (y `+0x121` si `+0x120`); con hijo capturado (`+0xC4`) cuyo
/// método `+0xC4` sea verdadero, cede con salto al método `+0x1C` del hijo (con los argumentos de esta llamada); si no, llama al `+0xC0`
/// y suelta la captura. Devuelve el EAX de lo último llamado, o −1 sin captura.
pub fn screen_window_on_mouse_leave(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let flag = m.r8(this.wrapping_add(0x120));
    m.w32(this.wrapping_add(0x11c), 0xffff_ffff);
    if flag != 0 {
        m.w8(this.wrapping_add(0x121), 1);
    }
    let cap = m.r32(this.wrapping_add(0xc4));
    if cap != 0xffff_ffff {
        let obj = child_obj(m, this, cap);
        if vcall(m, host, obj, 0xc4, &[]) as u8 != 0 {
            let cap = m.r32(this.wrapping_add(0xc4));
            let obj = child_obj(m, this, cap);
            let args: Vec<u32> = (0..3).map(|i| m.r32(f.arg(i))).collect();
            return vcall(m, host, obj, 0x1c, &args);
        }
    }
    let cap = m.r32(this.wrapping_add(0xc4));
    let eax = if cap != 0xffff_ffff {
        let obj = child_obj(m, this, cap);
        vcall(m, host, obj, 0xc0, &[])
    } else {
        cap
    };
    m.w32(this.wrapping_add(0xc4), 0xffff_ffff);
    eax
}

/// `0x004B6CC0`: el hijo con la captura del ratón (`+0xC4`), o 0.
pub fn screen_window_get_capture_child_window(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, _esp: u32) -> u32 {
    let cap = m.r32(this.wrapping_add(0xc4));
    if cap == 0xffff_ffff {
        return 0;
    }
    child_obj(m, this, cap)
}

/// `0x004B7240` (`ret 4`): índice del hijo más alto (se recorre desde el último) visible (`+0x38`) cuyo método `+0x20` acepta el punto
/// (argumento) relativo a él; −1 si ninguno. El punto se pasa en un local `{x, y}` relativo al origen de la ventana (`+0x60`).
pub fn screen_window_hit_test_children(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let pt = m.r32(f.arg(0));
    let px = m.r32(pt);
    let py = m.r32(pt.wrapping_add(4));
    let origin = f.at(0xc);
    vcall(m, host, this, 0x60, &[origin]);
    let ox = m.r32(origin);
    let oy = m.r32(origin.wrapping_add(4));
    let rel_x = px.wrapping_sub(ox);
    let rel_y = py.wrapping_sub(oy);
    let mut i = (m.r32(this.wrapping_add(0xc0)) as i32).wrapping_sub(1);
    m.w32(f.at(0x1c), rel_x);
    m.w32(f.at(0x18), rel_y);
    while i >= 0 {
        let slot = child_slot(m, this, i as u32);
        let cx = rel_x.wrapping_sub(m.r32(slot.wrapping_add(4)));
        let cy = rel_y.wrapping_sub(m.r32(slot.wrapping_add(8)));
        m.w32(f.at(0x14), cx);
        m.w32(f.at(0x10), cy);
        let obj = m.r32(slot);
        if obj != 0 && vcall(m, host, obj, 0x38, &[]) as u8 != 0 {
            let slot = child_slot(m, this, i as u32);
            let obj = m.r32(slot);
            if vcall(m, host, obj, 0x20, &[f.at(0x14)]) == 1 {
                return i as u32;
            }
        }
        i -= 1;
    }
    0xffff_ffff
}

/// Busca la ventana `wnd` entre los hijos de `this` con la caché de índice `0x6BBABC` (primero la comprueba; si la encuentra al recorrer,
/// la actualiza). Devuelve −1 si `wnd` es nulo o no está.
fn child_index_of(m: &mut dyn Mem, this: u32, wnd: u32) -> u32 {
    if wnd == 0 {
        return 0xffff_ffff;
    }
    let cached = m.r32(0x6bbabc);
    if cached != 0xffff_ffff && (cached as i32) < m.r32(this.wrapping_add(0xc0)) as i32 && child_obj(m, this, cached) == wnd {
        return cached;
    }
    let count = m.r32(this.wrapping_add(0xc0)) as i32;
    let mut i = 0i32;
    while i < count {
        let obj = child_obj(m, this, i as u32);
        if obj != 0 && obj == wnd {
            m.w32(0x6bbabc, i as u32);
            return i as u32;
        }
        i += 1;
    }
    0xffff_ffff
}

/// `0x004B72F0` (`ret 8`): arrastra el hijo `wnd` (argumento 0): busca su índice y salta a `0x4B7370(índice, punto)`.
pub fn screen_window_set_dragged_child_by_window(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let wnd = m.r32(f.arg(0));
    let pt = m.r32(f.arg(1));
    let idx = child_index_of(m, this, wnd);
    host.call(m, 0x4b7370, this, &[idx, pt])
}

/// `0x004B7BF0` (`ret 8`): como la anterior pero copia el punto a un local `{x, y, 0}` y devuelve 1 en AL (los 24 bits altos son los de
/// la llamada a `0x4B7370`).
pub fn screen_window_begin_drag_child(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let wnd = m.r32(f.arg(0));
    let pt = m.r32(f.arg(1));
    let x = m.r32(pt);
    let y = m.r32(pt.wrapping_add(4));
    let local = f.at(0xc);
    m.w32(local, x);
    m.w32(local.wrapping_add(4), y);
    m.w32(local.wrapping_add(8), 0);
    let idx = child_index_of(m, this, wnd);
    let r = host.call(m, 0x4b7370, this, &[idx, local]);
    (r & 0xffff_ff00) | 1
}

/// `0x004B7C80` (`ret 8`): suelta el arrastre: salta a `0x4B7370(−1, 0)`.
pub fn screen_window_end_drag_child(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _esp: u32) -> u32 {
    host.call(m, 0x4b7370, this, &[0xffff_ffff, 0])
}

/// `0x004B7370` (`ret 8`): fija el hijo arrastrado `idx` (−1 = ninguno) con el punto de agarre (argumento 1). Con hijo válido y que
/// admita arrastre (`+0xE8`) guarda en `+0x130/+0x134` el desplazamiento del punto respecto al hijo, suelta el anterior (`0x4B1C00(0)`),
/// lo marca (`+0xD4`/`+0xC4`/`+0x123`) y lo activa (`0x4B1C00(1)`); en otro caso termina el arrastre en curso. Devuelve `idx`.
pub fn screen_window_set_dragged_child(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let idx = m.r32(f.arg(0));
    let pt = m.r32(f.arg(1));
    let local = f.at(0xc);
    if idx != 0xffff_ffff {
        let slot = child_slot(m, this, idx);
        let child = m.r32(slot);
        if child != 0 && vcall(m, host, child, 0xe8, &[]) as u8 != 0 {
            vcall(m, host, this, 0x60, &[local]);
            let slot = child_slot(m, this, idx);
            let z = m.r32(local.wrapping_add(8));
            let x = m.r32(local).wrapping_add(m.r32(slot.wrapping_add(4)));
            m.w32(local, x);
            let y = m.r32(local.wrapping_add(4)).wrapping_add(m.r32(slot.wrapping_add(8)));
            m.w32(local.wrapping_add(4), y);
            let w = m.r32(slot.wrapping_add(0xc));
            m.w32(local.wrapping_add(8), z.wrapping_add(w));
            let obj = m.r32(slot);
            vcall(m, host, obj, 0xd4, &[]);
            let obj = child_obj(m, this, idx);
            host.call(m, 0x4b96c0, 0x6da5f0, &[local, obj]);
            let obj = child_obj(m, this, idx);
            vcall(m, host, obj, 0x60, &[this.wrapping_add(0x130)]);
            let slot = child_slot(m, this, idx);
            let dx = m.r32(pt).wrapping_sub(m.r32(slot.wrapping_add(4))).wrapping_sub(m.r32(this.wrapping_add(0x130)));
            m.w32(this.wrapping_add(0x130), dx);
            let dy = m.r32(pt.wrapping_add(4)).wrapping_sub(m.r32(slot.wrapping_add(8))).wrapping_sub(m.r32(this.wrapping_add(0x134)));
            m.w32(this.wrapping_add(0x134), dy);
            let cur = m.r32(this.wrapping_add(0xd4));
            if cur != idx && cur != 0xffff_ffff {
                let obj = child_obj(m, this, cur);
                if obj != 0 {
                    host.call(m, 0x4b1c00, obj, &[0]);
                }
            }
            m.w32(this.wrapping_add(0xd4), idx);
            m.w32(this.wrapping_add(0xc4), idx);
            m.w8(this.wrapping_add(0x123), 1);
            let obj = child_obj(m, this, idx);
            host.call(m, 0x4b1c00, obj, &[1]);
            let cur = m.r32(this.wrapping_add(0xd4));
            let obj = child_obj(m, this, cur);
            vcall(m, host, obj, 0xd4, &[]);
            return idx;
        }
        if child != 0 {
            return idx;
        }
    }
    let cur = m.r32(this.wrapping_add(0xd4));
    if cur != 0xffff_ffff && child_obj(m, this, cur) != 0 {
        vcall(m, host, this, 0x60, &[local]);
        let slot = child_slot(m, this, cur);
        let x = m.r32(local).wrapping_add(m.r32(slot.wrapping_add(4)));
        m.w32(local, x);
        let y = m.r32(local.wrapping_add(4)).wrapping_add(m.r32(slot.wrapping_add(8)));
        m.w32(local.wrapping_add(4), y);
        let z = m.r32(local.wrapping_add(8)).wrapping_add(m.r32(slot.wrapping_add(0xc)));
        m.w32(local.wrapping_add(8), z);
        let obj = m.r32(slot);
        vcall(m, host, obj, 0xd4, &[]);
        let obj = child_obj(m, this, cur);
        host.call(m, 0x4b96c0, 0x6da5f0, &[local, obj]);
        let obj = child_obj(m, this, cur);
        host.call(m, 0x4b1c00, obj, &[0]);
        let cur = m.r32(this.wrapping_add(0xd4));
        let obj = child_obj(m, this, cur);
        vcall(m, host, obj, 0xd4, &[]);
    }
    m.w32(this.wrapping_add(0xd4), 0xffff_ffff);
    m.w8(this.wrapping_add(0x123), 0);
    idx
}

/// `0x004B77F0` (`ret 4`): posición del cursor guardada en la ventana (`+0xA0`, `+0xA4`) al punto de salida (argumento).
pub fn screen_window_get_cursor_position(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, esp: u32) {
    let out = m.r32(Frame::new(esp).arg(0));
    let x = m.r32(this.wrapping_add(0xa0));
    m.w32(out, x);
    let y = m.r32(this.wrapping_add(0xa4));
    m.w32(out.wrapping_add(4), y);
}

/// `0x004B9430` (`ret 4`, `ScreenList`): copia la última posición del cursor (tres palabras en `+0x208`) a la salida si no es nula.
pub fn screen_list_get_last_cursor_position(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, esp: u32) {
    let out = m.r32(Frame::new(esp).arg(0));
    if out != 0 {
        for k in 0..3 {
            let v = m.r32(this.wrapping_add(0x208 + 4 * k));
            m.w32(out.wrapping_add(4 * k), v);
        }
    }
}

/// `0x004B9540` (`ScreenList`): cursor actual, `+0x1E8 + 4 * índice` con `índice = +0x214` (con signo); 0 si es negativo.
pub fn screen_list_get_current_cursor(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, _esp: u32) -> u32 {
    let i = m.r32(this.wrapping_add(0x214)) as i32;
    if i < 0 {
        return 0;
    }
    m.r32(this.wrapping_add(0x1e8).wrapping_add((i as u32).wrapping_mul(4)))
}

/// `0x004BD340`: mantiene el cursor dentro de la pantalla. Reinicia el rectángulo `0x6BED64..0x6BED70` a `{0, 0, 0x6DAB44, 0x6DAB4C}` y
/// recorta la posición `0x6DAB50` (y) a `[0, alto)` y `0x6DAB48` (x) a `[0, ancho)`; si cambió, convierte a coordenadas de pantalla
/// (`[0x66A520]`, ventana `0x6DCBB4`) y mueve el cursor (`[0x66A524]`).
pub fn clamp_cursor_to_screen(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, esp: u32) {
    let f = Frame::new(esp);
    let mut y = m.r32(0x6dab50);
    let h = m.r32(0x6dab4c);
    let w = m.r32(0x6dab44);
    let mut changed = false;
    m.w32(0x6bed68, 0);
    m.w32(0x6bed64, 0);
    m.w32(0x6bed70, w);
    m.w32(0x6bed6c, h);
    if (y as i32) < 0 {
        y = 0;
        changed = true;
        m.w32(0x6dab50, y);
    } else if (y as i32) >= h as i32 {
        y = h.wrapping_sub(1);
        changed = true;
        m.w32(0x6dab50, y);
    }
    let mut x = m.r32(0x6dab48);
    if (x as i32) < 0 {
        x = 0;
        changed = true;
        m.w32(0x6dab48, x);
    }
    if (x as i32) >= w as i32 {
        x = w.wrapping_sub(1);
        m.w32(0x6dab48, x);
    } else if !changed {
        return;
    }
    let pt = f.at(8);
    m.w32(pt, y);
    m.w32(pt.wrapping_add(4), x);
    let hwnd = m.r32(0x6dcbb4);
    let fp = m.r32(0x66a520);
    host.call(m, fp, 0, &[hwnd, pt]);
    let (px, py) = (m.r32(pt), m.r32(pt.wrapping_add(4)));
    let fp = m.r32(0x66a524);
    host.call(m, fp, 0, &[px, py]);
}

/// `0x004C0F50`: traduce una tecla virtual a ASCII con el estado actual del teclado (`GetKeyboardState` `[0x66A4FC]`, `ToAscii`
/// `[0x66A500]`); argumento 0 = tecla, argumento 2 = código de escaneo (byte con signo). Devuelve en AX los dos caracteres.
pub fn virtual_key_to_ascii(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let buf = f.at(0x104);
    let state = f.at(0x100);
    m.w16(buf, 0);
    m.w16(buf.wrapping_add(2), 0);
    let fp = m.r32(0x66a4fc);
    host.call(m, fp, 0, &[state]);
    let vk = m.r32(f.arg(0));
    let scan = m.r8(f.arg(2)) as i8 as i32 as u32;
    let fp = m.r32(0x66a500);
    let eax = host.call(m, fp, 0, &[vk, scan, state, buf, 0]);
    (eax & 0xffff_0000) | u32::from(m.r16(buf))
}

/// `0x004C0E80` (`ret 0xC`, `GameWindow`): al fijar el cursor la primera vez (`+0x3D` = 0) llama a `[0x66A50C](+0x1C)` y, si existe el
/// gestor `0x6DCB90`, a `0x65058B` con su resultado; fija `+0x3D` y llama a `[0x66A504](0)`. Siempre acaba en `0x6504F2` (base) y
/// devuelve su EAX.
pub fn game_window_on_set_cursor(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _esp: u32) -> u32 {
    if m.r8(this.wrapping_add(0x3d)) == 0 {
        if m.r32(0x6dcb90) != 0 {
            let a = m.r32(this.wrapping_add(0x1c));
            let fp = m.r32(0x66a50c);
            let r = host.call(m, fp, 0, &[a]);
            host.call(m, 0x65058b, 0, &[r]);
        }
        m.w8(this.wrapping_add(0x3d), 1);
        let fp = m.r32(0x66a504);
        host.call(m, fp, 0, &[0]);
    }
    host.call(m, 0x6504f2, this, &[])
}

/// Transformación de una tecla «consumida» de la tabla `0x6DCBE0..`: guarda `((v >> 1) & 0x7E) | (v & 0xFE)` y devuelve el valor previo.
fn key_state_take(m: &mut dyn Mem, addr: u32) -> u8 {
    let v = m.r8(addr);
    m.w8(addr, ((v >> 1) & 0x7e) | (v & 0xfe));
    v
}

/// `0x004CB6C0` (`ret 0xC`, `ViperListBox`): tras el manejador base `0x4B1850(a0, a1, a2)`, con la lista activa (`+0x310`) las teclas de
/// `0x6DCBE8` (siguiente) y `0x6DCBE6` (anterior) mueven la selección `+0x208` en ±1 y llaman a `0x4CD9F0(selección)` marcando `+0x312`.
/// Sin lista activa, EAX conserva los 24 bits altos del manejador base y `+0x310` en AL.
pub fn list_box_handle_key(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let args: Vec<u32> = (0..3).map(|i| m.r32(f.arg(i))).collect();
    let base = host.call(m, 0x4b1850, this, &args);
    let active = m.r8(this.wrapping_add(0x310));
    if active == 0 {
        return (base & 0xffff_ff00) | u32::from(active);
    }
    let mut sel = m.r32(this.wrapping_add(0x208));
    if key_state_take(m, 0x6dcbe8) != 0 {
        sel = sel.wrapping_add(1);
        m.w8(this.wrapping_add(0x312), 1);
        return host.call(m, 0x4cd9f0, this, &[sel]);
    }
    if key_state_take(m, 0x6dcbe6) != 0 {
        sel = sel.wrapping_sub(1);
    }
    m.w8(this.wrapping_add(0x312), 1);
    host.call(m, 0x4cd9f0, this, &[sel])
}

/// `0x004B9D30` (`ret 0xC`, `SGL2DScrollbar`): igual con la posición `+0x98` y el máximo `+0x94` (`0 ≤ pos ≤ máx`, con signo): si cambia llama
/// a `0x4BA810` y, con objeto de aviso (`+0x148`), a su método virtual `+0` solo cuando la posición cambió.
pub fn sgl_scrollbar_handle_scroll_keys(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let args: Vec<u32> = (0..3).map(|i| m.r32(f.arg(i))).collect();
    let base = host.call(m, 0x4b1850, this, &args);
    let mut pos = m.r32(this.wrapping_add(0x98));
    let mut al;
    let v = key_state_take(m, 0x6dcbe8);
    al = v;
    if v != 0 {
        pos = pos.wrapping_add(1);
    } else {
        let v = key_state_take(m, 0x6dcbe6);
        al = v;
        if v != 0 {
            pos = pos.wrapping_sub(1);
        }
    }
    let mut eax = (base & 0xffff_ff00) | u32::from(al & 0xfe);
    if (pos as i32) >= 0 && (pos as i32) <= m.r32(this.wrapping_add(0x94)) as i32 {
        let old = m.r32(this.wrapping_add(0x98));
        m.w32(this.wrapping_add(0x98), pos);
        eax = host.call(m, 0x4ba810, this, &[]);
        let cb = m.r32(this.wrapping_add(0x148));
        if cb != 0 && old != m.r32(this.wrapping_add(0x98)) {
            let vt = m.r32(cb);
            let fnp = m.r32(vt);
            eax = host.call(m, fnp, cb, &[]);
        }
    }
    eax
}

/// `0x004B6A90` (`ret 8`, `ScreenWindow::on_mouse_wheel`): acumula el giro (`+0x160`, el argumento 1 es un i16) y convierte a pasos de 120
/// (división truncada hacia cero). Con pasos ≠ 0 reinicia el ratón (`+0x11C`/`+0x121`) y busca en la lista `+0x14C` el primer elemento visible
/// (`0x4B2200`) cuyo rectángulo (`+0x380`) contenga el cursor (`PtInRect` `[0x66A4F4]`); desplaza su barra con `0x461210` (pasos ≥ 0) o `0x461320`
/// (negativos) por `paso * +0x374`. El argumento 1 queda sobrescrito con los pasos.
pub fn screen_window_on_mouse_wheel(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let delta = i32::from(m.r16(f.arg(1)) as i16);
    let sum = (m.r32(this.wrapping_add(0x160)) as i32).wrapping_add(delta);
    let hi = ((i64::from(sum) * i64::from(0x8888_8889u32 as i32)) >> 32) as i32;
    let t = hi.wrapping_add(sum) >> 6;
    let steps = t.wrapping_add(((t as u32) >> 31) as i32);
    m.w32(this.wrapping_add(0x160), sum as u32);
    m.w32(f.arg(1), steps as u32);
    if steps == 0 {
        return 0;
    }
    let flag = m.r8(this.wrapping_add(0x120));
    let h118 = m.r32(this.wrapping_add(0x118));
    m.w32(this.wrapping_add(0x160), 0);
    m.w32(this.wrapping_add(0x11c), h118);
    if flag != 0 {
        m.w8(this.wrapping_add(0x121), 1);
    }
    let mut node = m.r32(this.wrapping_add(0x14c));
    let mut eax = 0;
    while node != 0 {
        let wnd = m.r32(node.wrapping_add(8));
        let visible = host.call(m, 0x4b2200, wnd, &[]) as u8;
        if visible != 0 {
            let y = m.r32(this.wrapping_add(0xa4));
            let x = m.r32(this.wrapping_add(0xa0));
            let fp = m.r32(0x66a4f4);
            eax = host.call(m, fp, 0, &[wnd.wrapping_add(0x380), x, y]);
            if eax != 0 {
                let steps = m.r32(f.arg(1)) as i32;
                let unit = m.r32(wnd.wrapping_add(0x374)) as i32;
                let d = unit.wrapping_mul(steps);
                return if steps < 0 { host.call(m, 0x461320, wnd, &[d.wrapping_neg() as u32]) } else { host.call(m, 0x461210, wnd, &[d as u32]) };
            }
        }
        node = m.r32(node.wrapping_add(4));
    }
    eax
}

/// `0x00402310` (`ret 8`, `GameScreenHost`): con el código −2 ó −3 (argumento 1) y el flag `+0xD` del objeto `0x6CC3E8`, activa `+0x24C`.
/// EAX es `0x6CC3E8` si el código era −2/−3 y el código en otro caso.
pub fn game_screen_host_handle_input_code(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let code = m.r32(Frame::new(esp).arg(1));
    if code == 0xffff_fffd || code == 0xffff_fffe {
        let o = m.r32(0x6cc3e8);
        if m.r8(o.wrapping_add(0xd)) != 0 {
            m.w32(this.wrapping_add(0x24c), 1);
        }
        return o;
    }
    code
}

/// `0x004AF910` (`AnimButton`): devuelve (AL) y borra el clic pendiente `+0xD8`.
pub fn anim_button_consume_click(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, _esp: u32) -> u32 {
    let v = m.r8(this.wrapping_add(0xd8));
    m.w8(this.wrapping_add(0xd8), 0);
    u32::from(v)
}

/// `0x004AF920` (`ret 8`): evento de ratón del botón animado: delega en `0x4B1720(a0, a1)`; con el modo −2 (pulsación) marca el clic `+0xD8`;
/// con otro modo y clic marcado, lo borra si el botón ya no está activo (`+0xAC` falso).
pub fn anim_button_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    let a0 = m.r32(f.arg(0));
    let mode = m.r32(f.arg(1));
    host.call(m, 0x4b1720, this, &[a0, mode]);
    if mode == 0xffff_fffe {
        m.w8(this.wrapping_add(0xd8), 1);
        return;
    }
    if m.r8(this.wrapping_add(0xd8)) != 0 && vcall(m, host, this, 0xac, &[]) as u8 == 0 {
        m.w8(this.wrapping_add(0xd8), 0);
    }
}

/// `0x00429950` (`ret 0xC`, `GameScreen::on_mouse_down(a0, x, y)`): con una ventana activa (`0x462410`) no bloqueada (`+0x11C`) y el punto fuera de
/// su rectángulo absoluto (`+0x3C` desplazado por `+0x60`, `PtInRect`), avisa al hijo capturado (`0x4B6CC0`, método `+0xB4(a0)`), cierra
/// (`0x4B8380(0)`), borra `+0xB4`/`+0xB6` y suena el rechazo (`0x462420`). En otro caso delega en `0x4B5FA0(a0, x, y)`.
pub fn game_screen_on_mouse_down(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let a0 = m.r32(f.arg(0));
    let x = m.r32(f.arg(1));
    let y = m.r32(f.arg(2));
    let wnd = host.call(m, 0x462410, 0, &[]);
    let delegate = |m: &mut dyn Mem, host: &mut dyn OpsHost| host.call(m, 0x4b5fa0, this, &[a0, x, y]);
    if wnd == 0 || vcall(m, host, wnd, 0x11c, &[]) as u8 != 0 {
        return delegate(m, host);
    }
    let rect = f.at(0x10);
    let origin = f.at(0x1c);
    vcall(m, host, wnd, 0x3c, &[rect]);
    vcall(m, host, wnd, 0x60, &[origin]);
    let (ox, oy) = (m.r32(origin), m.r32(origin.wrapping_add(4)));
    for (k, o) in [(0u32, ox), (1, oy), (2, ox), (3, oy)] {
        let a = rect.wrapping_add(4 * k);
        let v = m.r32(a).wrapping_add(o);
        m.w32(a, v);
    }
    let fp = m.r32(0x66a4f4);
    if host.call(m, fp, 0, &[rect, x, y]) != 0 {
        return delegate(m, host);
    }
    let cap = host.call(m, 0x4b6cc0, this, &[]);
    if cap != 0 {
        vcall(m, host, cap, 0xb4, &[a0]);
    }
    host.call(m, 0x4b8380, this, &[0]);
    m.w8(this.wrapping_add(0xb4), 0);
    m.w8(this.wrapping_add(0xb6), 0);
    host.call(m, 0x462420, 0, &[])
}

/// `0x004298F0` (`ret 0xC`, `GameScreen::on_mouse_up(a0, x, y)`): delega en `0x4B6400(a0, x, y)`; si `0x462310` lo pedía llama a `0x462230`;
/// y con los contadores `0x6CBD90 == 0x6CC95C` y la marca `+0xC6B2` de `0x6E51AC` llama a `0x440F70(0, 0)`. EAX: lo último que escribe
/// el original (el contador `0x6CBD90` o lo que devuelva `0x440F70`).
pub fn game_screen_on_mouse_up(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let flag = host.call(m, 0x462310, 0, &[]) as u8;
    let args: Vec<u32> = (0..3).map(|i| m.r32(f.arg(i))).collect();
    host.call(m, 0x4b6400, this, &args);
    if flag != 0 {
        host.call(m, 0x462230, 0, &[]);
    }
    let a = m.r32(0x6cbd90);
    let b = m.r32(0x6cc95c);
    if a == b {
        let o = m.r32(0x6e51ac);
        if m.r16(o.wrapping_add(0xc6b2)) != 0 {
            return host.call(m, 0x440f70, 0, &[0, 0]);
        }
    }
    a
}

/// `0x004B6650` (`ret 0xC`, `ScreenWindow::on_mouse_move(a0, x, y)`): movimiento del ratón. Guarda la posición `+0xA0/+0xA4` (y copia
/// `+0x118 → +0x11C` si cambió), halla el hijo bajo el cursor (`0x4B7240`, punto = argumentos 1 y 2), actualiza el hover con `0x4B5C20`, envía
/// al hijo con hover el movimiento relativo (método `+0x18`, modo 1/2/0 según `+0xB6`/`+0xB5`) y, con captura (`+0xC4`) o arrastre
/// (`+0xD4`), el movimiento al hijo capturado/arrastrado (umbral `0x639F78`, métodos `+0x154`, `+0x68`, `0x4B96C0`). Termina con el método
/// `+0x18` de la ventana (punto, `a0`) y devuelve su EAX.
pub fn screen_window_on_mouse_move(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let a0 = m.r32(f.arg(0));
    let x = m.r32(f.arg(1));
    let y = m.r32(f.arg(2));
    let xy = f.arg(1);
    if x != m.r32(this.wrapping_add(0xa0)) || y != m.r32(this.wrapping_add(0xa4)) {
        let v = m.r32(this.wrapping_add(0x118));
        m.w32(this.wrapping_add(0x11c), v);
        if m.r8(this.wrapping_add(0x120)) != 0 {
            m.w8(this.wrapping_add(0x121), 1);
        }
    }
    let (l10, l14, l18) = (f.at(0x20), f.at(0x1c), f.at(0x18));
    m.w32(l14, y);
    m.w32(l10, x);
    vcall(m, host, this, 0x60, &[l18]);
    let ox = m.r32(l18);
    let oy = m.r32(f.at(0x14));
    let rx = x.wrapping_sub(ox);
    let ry = y.wrapping_sub(oy);
    m.w32(l10, rx);
    m.w32(this.wrapping_add(0xa0), x);
    m.w32(l14, ry);
    m.w32(this.wrapping_add(0xa4), y);
    let idx = host.call(m, 0x4b7240, this, &[xy]);
    let b4 = m.r8(this.wrapping_add(0xb4));
    m.w32(this.wrapping_add(0xcc), idx);
    let c8 = m.r32(this.wrapping_add(0xc8));
    if b4 == 0 && idx != c8 {
        host.call(m, 0x4b5c20, this, &[idx, 0]);
    } else if idx != c8 {
        host.call(m, 0x4b5c20, this, &[c8, 0xffff_ffff]);
    } else if b4 != 0 {
        host.call(m, 0x4b5c20, this, &[c8, 0]);
    }
    let c8 = m.r32(this.wrapping_add(0xc8));
    let hover_mode = |m: &mut dyn Mem| -> u32 {
        if m.r8(this.wrapping_add(0xb6)) != 0 {
            1
        } else if m.r8(this.wrapping_add(0xb5)) != 0 {
            2
        } else {
            0
        }
    };
    if c8 != 0xffff_ffff {
        if (c8 as i32) >= 0 && (c8 as i32) < m.r32(this.wrapping_add(0xc0)) as i32 {
            let slot = child_slot(m, this, c8);
            if m.r32(slot) != 0 {
                let cx = m.r32(l10).wrapping_sub(m.r32(slot.wrapping_add(4)));
                m.w32(l10, cx);
                let cy = m.r32(l14).wrapping_sub(m.r32(slot.wrapping_add(8)));
                m.w32(l14, cy);
            }
        }
        let mode = hover_mode(m);
        let child = child_obj(m, this, c8);
        vcall(m, host, child, 0x18, &[l10, mode]);
    }
    let cap = m.r32(this.wrapping_add(0xc4));
    if cap == 0xffff_ffff {
        if m.r8(this.wrapping_add(0x123)) != 0 {
            let base = m.r32(this.wrapping_add(0x94));
            let first = m.r32(base.wrapping_sub(0x10));
            vcall(m, host, this, 0x158, &[first, xy]);
        }
    } else {
        let drag = m.r32(this.wrapping_add(0xd4));
        if drag == 0xffff_ffff {
            if m.r8(this.wrapping_add(0xb6)) != 0 {
                let child = child_obj(m, this, cap);
                let th = u32::from(m.r8(child.wrapping_add(0x40)));
                m.w8(f.at(0x24), th as u8);
                let dx = m.r32(this.wrapping_add(0x124)).wrapping_sub(x);
                let ax = host.call(m, 0x639f78, 0, &[dx]);
                let mut fire = (ax as i32) > (th as i32);
                if !fire {
                    let cap = m.r32(this.wrapping_add(0xc4));
                    let child = child_obj(m, this, cap);
                    let th = u32::from(m.r8(child.wrapping_add(0x40)));
                    m.w8(f.at(0x24), th as u8);
                    let dy = m.r32(this.wrapping_add(0x128)).wrapping_sub(y);
                    let ay = host.call(m, 0x639f78, 0, &[dy]);
                    fire = (ay as i32) > (th as i32);
                }
                if fire {
                    let (px, py) = (m.r32(this.wrapping_add(0x124)), m.r32(this.wrapping_add(0x128)));
                    m.w32(f.at(0x14), py);
                    m.w32(l18, px);
                    let cap = m.r32(this.wrapping_add(0xc4));
                    let child = child_obj(m, this, cap);
                    vcall(m, host, this, 0x154, &[child, l18]);
                }
            }
            let cap = m.r32(this.wrapping_add(0xc4));
            m.w32(l10, x);
            m.w32(l14, y);
            if (cap as i32) >= 0 && (cap as i32) < m.r32(this.wrapping_add(0xc0)) as i32 {
                let slot = child_slot(m, this, cap);
                if m.r32(slot) != 0 {
                    let cx = x.wrapping_sub(m.r32(slot.wrapping_add(4)));
                    m.w32(l10, cx);
                    let cy = y.wrapping_sub(m.r32(slot.wrapping_add(8)));
                    m.w32(l14, cy);
                }
            }
            let mode = hover_mode(m);
            let child = child_obj(m, this, cap);
            vcall(m, host, child, 0x18, &[l10, mode]);
        } else {
            let slot = child_slot(m, this, drag);
            let dx = m.r32(0x6dab50).wrapping_sub(m.r32(slot.wrapping_add(4))).wrapping_sub(m.r32(this.wrapping_add(0x130)));
            m.w32(f.at(0xc), dx);
            let dy = m.r32(0x6dab48).wrapping_sub(m.r32(slot.wrapping_add(8))).wrapping_sub(m.r32(this.wrapping_add(0x134)));
            m.w32(f.at(8), dy);
            if (drag as i32) >= 0 && (drag as i32) < m.r32(this.wrapping_add(0xc0)) as i32 && m.r32(slot) != 0 {
                let cx = m.r32(l10).wrapping_sub(m.r32(slot.wrapping_add(4)));
                m.w32(l10, cx);
                let cy = m.r32(l14).wrapping_sub(m.r32(slot.wrapping_add(8)));
                m.w32(l14, cy);
            }
            vcall(m, host, this, 0x60, &[l18]);
            let slot = child_slot(m, this, drag);
            let nx = m.r32(l18).wrapping_add(m.r32(slot.wrapping_add(4)));
            m.w32(l18, nx);
            let ny = m.r32(f.at(0x14)).wrapping_add(m.r32(slot.wrapping_add(8)));
            m.w32(f.at(0x14), ny);
            let nz = m.r32(f.at(0x10)).wrapping_add(m.r32(slot.wrapping_add(0xc)));
            m.w32(f.at(0x10), nz);
            let obj = m.r32(slot);
            vcall(m, host, obj, 0xd4, &[]);
            let drag = m.r32(this.wrapping_add(0xd4));
            let obj = child_obj(m, this, drag);
            host.call(m, 0x4b96c0, 0x6da5f0, &[l18, obj]);
            let drag = m.r32(this.wrapping_add(0xd4));
            let obj = child_obj(m, this, drag);
            vcall(m, host, obj, 0x68, &[f.at(0xc)]);
            let mode = hover_mode(m);
            let drag = m.r32(this.wrapping_add(0xd4));
            let obj = child_obj(m, this, drag);
            vcall(m, host, obj, 0x18, &[l10, mode]);
        }
    }
    vcall(m, host, this, 0x18, &[xy, a0])
}

/// `0x0046CD10` (`ret 8`, `SliderButton`): evento de ratón: base `0x4C6B90(a0, a1)`; si el botón está arrastrándose (`0x4B1BF0`) copia
/// `+0xEC → +0x14` y recorta `+0x18` a `[+0xF0, +0xF0 + +0xE8]` (con signo). EAX: `+0xF0 + +0xE8` si arrastraba, o el de `0x4B1BF0`.
pub fn slider_button_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let (a0, a1) = (m.r32(f.arg(0)), m.r32(f.arg(1)));
    host.call(m, 0x4c6b90, this, &[a0, a1]);
    let r = host.call(m, 0x4b1bf0, this, &[]);
    if r as u8 == 0 {
        return r;
    }
    let lo = m.r32(this.wrapping_add(0xf0));
    let cur = m.r32(this.wrapping_add(0x18));
    let v = m.r32(this.wrapping_add(0xec));
    m.w32(this.wrapping_add(0x14), v);
    if (cur as i32) < (lo as i32) {
        m.w32(this.wrapping_add(0x18), lo);
    }
    let hi = lo.wrapping_add(m.r32(this.wrapping_add(0xe8)));
    if (m.r32(this.wrapping_add(0x18)) as i32) > (hi as i32) {
        m.w32(this.wrapping_add(0x18), hi);
    }
    hi
}

/// Paso común de los selectores: fija `+0xB0` en `eax` si está en `[-1, +0xA4)` y avisa con el método `+0xD4`.
fn state_switch_set(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, eax: u32) {
    let count = m.r32(this.wrapping_add(0xa4)) as i32;
    if (eax as i32) >= count || (eax as i32) < -1 {
        return;
    }
    m.w32(this.wrapping_add(0xb0), eax);
    vcall(m, host, this, 0xd4, &[]);
}

/// `0x004D1610` (`ret 8`, `ViperStateSwitch`): evento de ratón: base `0x4B1720(a0, modo)`; con el ratón encima (`+0xC4`) y modo −2 pasa al
/// estado siguiente (`+0xB0 + 1`, vuelve a 0 al llegar a `+0xA4`); con modo −3 al anterior (de 0 salta al último). EAX sin valor definido.
pub fn state_switch_handle_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    let (a0, mode) = (m.r32(f.arg(0)), m.r32(f.arg(1)));
    host.call(m, 0x4b1720, this, &[a0, mode]);
    if vcall(m, host, this, 0xc4, &[]) as u8 != 0 && mode == 0xffff_fffe {
        let mut eax = m.r32(this.wrapping_add(0xb0)).wrapping_add(1);
        let count = m.r32(this.wrapping_add(0xa4));
        if (eax as i32) >= (count as i32) {
            eax = 0;
        }
        state_switch_set(m, host, this, eax);
        return;
    }
    if vcall(m, host, this, 0xc4, &[]) as u8 == 0 || mode != 0xffff_fffd {
        return;
    }
    let mut eax = m.r32(this.wrapping_add(0xb0)).wrapping_sub(1);
    if (eax as i32) < 0 {
        eax = m.r32(this.wrapping_add(0xa4)).wrapping_sub(1);
    }
    state_switch_set(m, host, this, eax);
}

/// `0x004D16A0` (`ret 0xC`): con el ratón encima y la tecla `0x6DCBE0` consumida avanza al estado siguiente como el clic. EAX sin valor.
pub fn state_switch_on_click_advance(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _esp: u32) {
    if vcall(m, host, this, 0xc4, &[]) as u8 == 0 {
        return;
    }
    if key_state_take(m, 0x6dcbe0) == 0 {
        return;
    }
    let count = m.r32(this.wrapping_add(0xa4));
    let mut eax = m.r32(this.wrapping_add(0xb0)).wrapping_add(1);
    if (eax as i32) >= (count as i32) {
        eax = 0;
        if (eax as i32) >= (count as i32) {
            return;
        }
    }
    state_switch_set(m, host, this, eax);
}

/// `0x004C49E0` (`ret 8`, `SGL2DStatusBar`): evento de ratón: base `0x4B1720(a0, modo)` y reenvío al hijo del estado actual (método `+0x28`,
/// 0..3, tabla `+0x94`) con su método `+0x18(a0, modo)`. EAX: lo que devuelve el hijo, o el estado si no hay hijo.
pub fn sgl2d_status_bar_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let (a0, mode) = (m.r32(f.arg(0)), m.r32(f.arg(1)));
    host.call(m, 0x4b1720, this, &[a0, mode]);
    let st = vcall(m, host, this, 0x28, &[]);
    if (st as i32) < 0 || (st as i32) >= 4 || m.r32(this.wrapping_add(0x94).wrapping_add(st.wrapping_mul(4))) == 0 {
        return st;
    }
    let st = vcall(m, host, this, 0x28, &[]);
    let child = if (st as i32) >= 0 && (st as i32) < 4 { m.r32(this.wrapping_add(0x94).wrapping_add(st.wrapping_mul(4))) } else { 0 };
    vcall(m, host, child, 0x18, &[a0, mode])
}

/// Punto `{x, y, 0}` en un local y llamada al método virtual `0x11C` de `obj` con su dirección.
fn hit_test_local_point(m: &mut dyn Mem, host: &mut dyn OpsHost, obj: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let pt = m.r32(f.arg(0));
    let local = f.at(0xc);
    let x = m.r32(pt);
    let y = m.r32(pt.wrapping_add(4));
    m.w32(local.wrapping_add(8), 0);
    m.w32(local.wrapping_add(4), y);
    m.w32(local, x);
    vcall(m, host, obj, 0x11c, &[local])
}

/// `0x004BACD0` (`ret 4`, `SGL2DScrollbar`): prueba de punto (copia `{x, y, 0}` y llama al método `+0x11C`).
pub fn sgl2d_scrollbar_hit_test_xy(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    hit_test_local_point(m, host, this, esp)
}

/// `0x004C9120` (`ViperComboBox`): si el desplegable (`this + 0x180`) está visible (`+0xC8`) avisa a la lista (`+0xD4`) y lo cierra (`+0xCC(0)`);
/// devuelve 1 en AL.
pub fn combo_box_close_dropdown_on_focus_lost(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _esp: u32) -> u32 {
    let sub = this.wrapping_add(0x180);
    if vcall(m, host, sub, 0xc8, &[]) as u8 != 0 {
        vcall(m, host, this, 0xd4, &[]);
        vcall(m, host, sub, 0xcc, &[0]);
    }
    1
}

/// `0x004D0B20` (`ret 4`, `ViperSlider`): suelta el botón: base `0x4B2360(a0)` y avisa a la barra (`this + 0x244`, método `+0xA8(a0)`);
/// devuelve (AL) el resultado de la base.
pub fn viper_slider_on_mouse_release(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let a0 = m.r32(Frame::new(esp).arg(0));
    let base = host.call(m, 0x4b2360, this, &[a0]);
    let r = vcall(m, host, this.wrapping_add(0x244), 0xa8, &[a0]);
    (r & 0xffff_ff00) | (base & 0xff)
}

/// `0x004CA6F0` (`ViperEditBox`): al perder el foco borra `0x6DD5DE` y `+0x42`; devuelve 1 en AL.
pub fn edit_box_on_focus_lost(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, _esp: u32) -> u32 {
    m.w8(0x6dd5de, 0);
    m.w8(this.wrapping_add(0x42), 0);
    1
}

/// `0x0045C1F0` (`NumberInputField`): reinicia el campo: base `0x4C99F0`, `+0x188` = −1 y `+0x180`, `+0x184`, `+0x18C` a 0.
pub fn number_input_field_reset(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _esp: u32) {
    host.call(m, 0x4c99f0, this, &[]);
    m.w32(this.wrapping_add(0x188), 0xffff_ffff);
    m.w32(this.wrapping_add(0x184), 0);
    m.w32(this.wrapping_add(0x180), 0);
    m.w8(this.wrapping_add(0x18c), 0);
}

/// `0x004C6B90` (`ret 8`, `ButtonBase`): evento de ratón del botón (modo = argumento 1: 1 pulsar, −2 soltar, 0 salir). Base `0x4B1720(a0, modo)`;
/// el objeto enlazado `obj` (método `+0x14`) sustituye al argumento 1 en la pila. Con el botón activable (`+0x38`) lleva el estado de pulsación
/// (`+0xBC`/`+0xBD`) según el tipo `+0xB8` (0 pulsador, 1 conmutador con `+0x120/+0x124`, 2 mantenido) y notifica al enlazado con `0x4B04C0`
/// pasando la dirección de un byte local. EAX sin valor definido.
pub fn button_base_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    m.w32(f.at(4), this);
    let a0 = m.r32(f.arg(0));
    let mode = m.r32(f.arg(1));
    host.call(m, 0x4b1720, this, &[a0, mode]);
    let linked = vcall(m, host, this, 0x14, &[]);
    m.w32(f.arg(1), linked);
    if vcall(m, host, this, 0x38, &[]) as u8 == 0 {
        return;
    }
    if mode == 1 {
        m.w8(this.wrapping_add(0xbd), 1);
    }
    let kind = m.r32(this.wrapping_add(0xb8));
    if kind == 2 {
        if mode == 1 {
            if m.r8(this.wrapping_add(0xbc)) == 0 {
                m.w32(this.wrapping_add(0xd8), 0);
                m.w32(this.wrapping_add(0xe4), 0);
                m.w8(this.wrapping_add(0xbc), 1);
            }
        } else if mode == 0xffff_fffe {
            m.w8(this.wrapping_add(0xbc), 0);
        }
    } else if vcall(m, host, this, 0xac, &[]) as u8 != 0 && mode == 0xffff_fffe && m.r8(this.wrapping_add(0xbd)) != 0 {
        m.w8(this.wrapping_add(0xbc), 1);
    }
    let obj = m.r32(f.arg(1));
    if m.r8(this.wrapping_add(0xbd)) != 0 && (mode == 0xffff_fffe || mode == 0) {
        m.w8(this.wrapping_add(0xbd), 0);
    }
    if obj == 0 {
        return;
    }
    match kind {
        0 => {
            m.w8(f.at(2), 0);
            host.call(m, 0x4b04c0, obj, &[f.at(2)]);
            if vcall(m, host, this, 0xac, &[]) as u8 != 0 && mode == 0xffff_fffe {
                m.w8(f.at(1), 1);
                host.call(m, 0x4b04c0, obj, &[f.at(1)]);
            }
        }
        1 => {
            if vcall(m, host, this, 0xac, &[]) as u8 != 0 && mode == 0xffff_fffe {
                let r = vcall(m, host, this, 0x120, &[]);
                let arg = (r & 0xffff_ff00) | u32::from(r as u8 == 0);
                vcall(m, host, this, 0x124, &[arg]);
            }
        }
        2 => {
            if mode == 1 {
                m.w8(f.arg(1), 1);
                host.call(m, 0x4b04c0, obj, &[f.arg(1)]);
            } else if mode == 0xffff_fffe {
                m.w8(f.arg(0), 0);
                host.call(m, 0x4b04c0, obj, &[f.arg(0)]);
            }
        }
        _ => {}
    }
}

/// `0x004C8000` (`ret 8`, `ViperComboBox::on_mouse_event(a0, modo)`): base `0x4B1720(a0, modo)`; reenvía el evento a la lista (`this + 0x94`) y al
/// desplegable (`this + 0x180`) visibles (`+0xC8`): sobre cada uno, si el punto (`+0x20`) cae dentro y el combo está activo (`+0xC4`) pone el
/// estado resaltado (`+0xBC`), si no el normal (`+0xC0`); luego `+0x18(a0, modo)`. Con modo −2 sobre la lista y sin arrastre (`0x4B1BF0`)
/// conmuta la visibilidad del desplegable (`+0xCC`); con −2 fuera del desplegable lo cierra. EAX sin valor definido.
pub fn combo_box_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) {
    let f = Frame::new(esp);
    let (a0, mode) = (m.r32(f.arg(0)), m.r32(f.arg(1)));
    host.call(m, 0x4b1720, this, &[a0, mode]);
    let list = this.wrapping_add(0x94);
    let drop = this.wrapping_add(0x180);
    if vcall(m, host, list, 0xc8, &[]) as u8 != 0 {
        let inside = vcall(m, host, list, 0x20, &[a0]);
        if inside != 0 && vcall(m, host, this, 0xc4, &[]) as u8 != 0 {
            vcall(m, host, list, 0xbc, &[]);
        } else {
            vcall(m, host, list, 0xc0, &[]);
        }
        vcall(m, host, list, 0x18, &[a0, mode]);
    }
    if vcall(m, host, drop, 0xc8, &[]) as u8 != 0 {
        let inside = vcall(m, host, drop, 0x20, &[a0]);
        if inside != 0 && vcall(m, host, this, 0xc4, &[]) as u8 != 0 {
            vcall(m, host, drop, 0xbc, &[]);
        } else {
            vcall(m, host, drop, 0xc0, &[]);
        }
        vcall(m, host, drop, 0x18, &[a0, mode]);
        vcall(m, host, this, 0xd4, &[]);
    }
    let over_list = vcall(m, host, list, 0x20, &[a0]);
    let close_check = if over_list == 1 {
        if mode != 0xffff_fffe {
            return;
        }
        if host.call(m, 0x4b1bf0, this, &[]) as u8 == 0 {
            let vis = vcall(m, host, drop, 0xc8, &[]) as u8;
            vcall(m, host, drop, 0xcc, &[(drop & 0xffff_ff00) | u32::from(vis == 0)]);
            vcall(m, host, this, 0xd4, &[]);
            vcall(m, host, drop, 0xd4, &[]);
            return;
        }
        true
    } else {
        mode == 0xffff_fffe
    };
    if !close_check {
        return;
    }
    if host.call(m, 0x4cdb20, drop, &[]) as u8 == 0 {
        return;
    }
    if vcall(m, host, drop, 0x20, &[a0]) == 0 {
        return;
    }
    if host.call(m, 0x4cc190, drop, &[a0]) != 0 {
        return;
    }
    if host.call(m, 0x4b1bf0, this, &[]) as u8 != 0 {
        return;
    }
    vcall(m, host, drop, 0xcc, &[0]);
    vcall(m, host, drop, 0xc0, &[]);
    vcall(m, host, this, 0xd4, &[]);
}

/// `0x004BABE0` (`ret 4`, `SGL2DScrollbar`): zona de la barra bajo el punto (argumento): 0 fuera, 2 el cuerpo (`+0x120` / `+0xD8`), 3 el hueco
/// antes, 4 el hueco después del deslizador (comparaciones con signo del punto relativo al origen `+0x60` y al tamaño `+0x40`). Deja en el
/// local `esp-4` la palabra `+0xE0`.
pub fn sgl2d_scrollbar_hit_test_region(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let pt = m.r32(f.arg(0));
    if vcall(m, host, this, 0x24, &[pt]) == 0 {
        return 0;
    }
    let origin = f.at(0x24);
    vcall(m, host, this, 0x60, &[origin]);
    let relx = m.r32(pt).wrapping_sub(m.r32(origin));
    let rely = m.r32(pt.wrapping_add(4)).wrapping_sub(m.r32(origin.wrapping_add(4)));
    m.w32(origin, relx);
    m.w32(f.at(0x20), rely);
    let size = f.at(0x18);
    vcall(m, host, this, 0x40, &[size]);
    let a = m.r32(this.wrapping_add(0x120)) as i32;
    let b = m.r32(this.wrapping_add(0x124)) as i32;
    let _ = m.r32(this.wrapping_add(0x128));
    let w = m.r32(this.wrapping_add(0xd8)) as i32;
    let h = m.r32(this.wrapping_add(0xdc)) as i32;
    let last = m.r32(this.wrapping_add(0xe0));
    m.w32(f.at(4), last);
    let (x, y) = (relx as i32, rely as i32);
    if x < a {
        return 0;
    }
    if !(x > a.wrapping_add(w) || y < b || y > h.wrapping_add(b)) {
        return 2;
    }
    if x <= a || x >= a.wrapping_add(w) {
        return 0;
    }
    if !(y <= h.wrapping_add(b)) && y < m.r32(f.at(0x14)) as i32 {
        return 4;
    }
    if y < b {
        3
    } else {
        0
    }
}

/// Cabecera `col` (tabla `+0xA0`).
fn hdr_ctrl(m: &mut dyn Mem, this: u32, col: u32) -> u32 {
    let t = m.r32(this.wrapping_add(0xa0));
    m.r32(t.wrapping_add(col << 2))
}

/// Fila `row`, columna `col` de la tabla de celdas de la lista (`+0x98`: matriz de punteros a filas; cada fila, un puntero por columna);
/// 0 si la fila o la celda no existen.
fn list_cell(m: &mut dyn Mem, this: u32, row: u32, col: u32) -> u32 {
    let rows = m.r32(this.wrapping_add(0x98));
    let r = m.r32(rows.wrapping_add(row << 2));
    if r == 0 {
        return 0;
    }
    m.r32(r.wrapping_add(col << 2))
}

/// `0x004CB2A0` (`ret 8`, `ecx` = ViperListBox): ratón sobre la lista. Reenvía el evento a la barra de desplazamiento propia (`+0xB4`);
/// si la posición de la barra cambia reposiciona las celdas (oculta las filas fuera de ventana, pone la celda de cada fila visible
/// en la columna acumulada con `+0x68`/`+0xCC`/`+0xD4` y marca `+0x313`). Si no hay arrastre de la barra (`+0x308` = 0) pasa el
/// evento a las cabeceras (`+0xA0`, con `+0x215`) y a las celdas visibles: si una celda contiene el punto selecciona la fila
/// (`0x4CD9F0`, `+0x312`) y la notifica con `+0x18`. Termina guardando el modo en `+0x30C`.
pub fn list_box_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    // locales según el desplazamiento [esp+X] del original con ESP = entrada - 0x38
    let l = |x: u32| f.at(0x38 - x);
    let (origin, orig_y) = (l(0x2c), l(0x30));
    vcall(m, host, this, 0x60, &[origin]);
    let (a0, mode) = (m.r32(f.arg(0)), m.r32(f.arg(1)));
    host.call(m, 0x4b1720, this, &[a0, mode]);
    let bar = this.wrapping_add(0xb4);
    let rect = l(0x18);
    if m.r8(this.wrapping_add(0x228)) != 0 {
        let px = m.r32(a0).wrapping_sub(m.r32(origin));
        m.w32(rect, px);
        let py = m.r32(a0.wrapping_add(4)).wrapping_sub(m.r32(orig_y));
        m.w32(rect.wrapping_add(4), py);
        vcall(m, host, bar, 0x18, &[rect, mode]);
        let hit = vcall(m, host, bar, 0x118, &[rect]);
        let mut drag = hit != 0;
        if !drag && mode == 1 && m.r8(this.wrapping_add(0x308)) != 0 {
            drag = true;
        }
        if drag {
            m.w8(this.wrapping_add(0x308), 1);
            vcall(m, host, this, 0xd4, &[]);
            let pos = host.call(m, 0x4b9ec0, bar, &[]);
            m.w32(l(0x10), pos);
            if pos != m.r32(this.wrapping_add(0x230)) {
                reposition_rows(m, host, this, esp, pos);
            }
        }
    }
    if m.r8(this.wrapping_add(0x308)) == 0 {
        forward_to_cells(m, host, this, esp, a0, mode);
    }
    if mode != 1 && m.r8(this.wrapping_add(0x308)) != 0 {
        m.w8(this.wrapping_add(0x308), 0);
    }
    m.w32(this.wrapping_add(0x30c), mode);
    vcall(m, host, this, 0xd4, &[])
}

/// Segunda parte de [`list_box_on_mouse_event`] tras mover la barra: recoloca las celdas para la posición `pos`.
fn reposition_rows(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32, pos0: u32) {
    let f = Frame::new(esp);
    let l = |x: u32| f.at(0x38 - x);
    let bar = this.wrapping_add(0xb4);
    let mut pos = pos0;
    let ps = host.call(m, 0x4ba040, bar, &[]);
    let rows = m.r32(this.wrapping_add(0x9c));
    if (rows as i32) > (ps as i32) {
        let ps2 = host.call(m, 0x4ba040, bar, &[]);
        let lim = m.r32(this.wrapping_add(0x9c)).wrapping_sub(ps2);
        if (pos as i32) > (lim as i32) {
            let ps3 = host.call(m, 0x4ba040, bar, &[]);
            pos = m.r32(this.wrapping_add(0x9c)).wrapping_sub(ps3);
            m.w32(l(0x10), pos);
        }
    }
    let cols = m.r32(this.wrapping_add(0xa4));
    m.w32(l(0x20), 0);
    m.w32(l(0x24), 0);
    m.w32(l(0x28), 0);
    let mut col: u32 = 0;
    if (cols as i32) > 0 {
        loop {
            let header = m.r8(this.wrapping_add(0x215)) != 0;
            let y0 = if header { m.r32(this.wrapping_add(0xb0)) } else { 0 };
            m.w32(l(0x24), y0);
            let ps = host.call(m, 0x4ba040, bar, &[]);
            let mut end = ps.wrapping_add(pos);
            let nrows = m.r32(this.wrapping_add(0x9c));
            if (end as i32) > (nrows as i32) {
                end = nrows;
            }
            let mut row: u32 = 0;
            if (m.r32(l(0x10)) as i32) > 0 {
                loop {
                    let c = list_cell(m, this, row, col);
                    if c != 0 {
                        vcall(m, host, c, 0xcc, &[0]);
                    }
                    row = row.wrapping_add(1);
                    if (row as i32) >= (m.r32(l(0x10)) as i32) {
                        break;
                    }
                }
            }
            while (row as i32) < (end as i32) {
                let c = list_cell(m, this, row, col);
                if c != 0 {
                    vcall(m, host, c, 0x68, &[l(0x20)]);
                    let c = list_cell(m, this, row, col);
                    vcall(m, host, c, 0xcc, &[1]);
                    let c = list_cell(m, this, row, col);
                    vcall(m, host, c, 0xd4, &[]);
                }
                let step = m.r32(this.wrapping_add(0xb0));
                let y = m.r32(l(0x24)).wrapping_add(step);
                m.w32(l(0x24), y);
                row = row.wrapping_add(1);
            }
            while (row as i32) < (m.r32(this.wrapping_add(0x9c)) as i32) {
                let c = list_cell(m, this, row, col);
                if c != 0 {
                    vcall(m, host, c, 0xcc, &[0]);
                }
                row = row.wrapping_add(1);
            }
            let widths = m.r32(this.wrapping_add(0xac));
            let wcol = m.r32(widths.wrapping_add(col << 2));
            let x = m.r32(l(0x20)).wrapping_add(wcol);
            col = col.wrapping_add(1);
            m.w32(l(0x20), x);
            let ncols = m.r32(this.wrapping_add(0xa4));
            if (col as i32) >= (ncols as i32) {
                break;
            }
        }
    }
    let pos = m.r32(l(0x10));
    if m.r32(this.wrapping_add(0x230)) != pos {
        m.w8(this.wrapping_add(0x313), 1);
    }
    m.w32(this.wrapping_add(0x230), pos);
    vcall(m, host, this, 0xd4, &[]);
}

/// Parte de [`list_box_on_mouse_event`] sin arrastre de la barra: pasa el evento a cada cabecera y a las celdas visibles.
fn forward_to_cells(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32, a0: u32, mode: u32) {
    let f = Frame::new(esp);
    let l = |x: u32| f.at(0x38 - x);
    let bar = this.wrapping_add(0xb4);
    // copia del punto y puesta a cero de los acumuladores (el original llama a `memset` real)
    let (px, py) = (m.r32(a0), m.r32(a0.wrapping_add(4)));
    m.w32(l(0x10), px);
    m.w32(l(0x14), py);
    for k in 0..3 {
        m.w32(l(0x20).wrapping_add(4 * k), 0);
    }
    let header = m.r8(this.wrapping_add(0x215)) != 0;
    let hdrs = m.r32(this.wrapping_add(0xa0));
    let cols0 = m.r32(this.wrapping_add(0xa4));
    let mut ycur;
    if header && hdrs != 0 {
        let mut x = m.r32(l(0x20));
        ycur = m.r32(l(0x24));
        let mut i: u32 = 0;
        if (cols0 as i32) > 0 {
            loop {
                if m.r32(hdrs.wrapping_add(i << 2)) != 0 {
                    let widths = m.r32(this.wrapping_add(0xac));
                    x = x.wrapping_add(m.r32(widths.wrapping_add(i << 2)));
                    m.w32(l(0x20), x);
                }
                if ycur == 0 {
                    ycur = m.r32(this.wrapping_add(0xb0));
                    m.w32(l(0x24), ycur);
                }
                i += 1;
                if (i as i32) >= (cols0 as i32) {
                    break;
                }
            }
        }
    } else {
        ycur = m.r32(l(0x24));
    }
    let ox = m.r32(l(0x2c));
    let oy = m.r32(l(0x30));
    let nx = m.r32(l(0x10)).wrapping_sub(ox);
    let ny = m.r32(l(0x14)).wrapping_sub(ycur.wrapping_add(oy));
    m.w32(l(0x10), nx);
    m.w32(l(0x14), ny);
    let mut col: u32 = 0;
    if (m.r32(this.wrapping_add(0xa4)) as i32) > 0 {
        loop {
            if m.r8(this.wrapping_add(0x215)) != 0 {
                let h = hdr_ctrl(m, this, col);
                vcall(m, host, h, 0x18, &[a0, mode]);
                let h = hdr_ctrl(m, this, col);
                if vcall(m, host, h, 0x20, &[a0]) == 1 {
                    let h = hdr_ctrl(m, this, col);
                    vcall(m, host, h, 0x28, &[]);
                }
            }
            let mut row = m.r32(this.wrapping_add(0x230));
            let ps = host.call(m, 0x4ba040, bar, &[]);
            let mut end = ps.wrapping_add(m.r32(this.wrapping_add(0x230)));
            let nrows = m.r32(this.wrapping_add(0x9c));
            if (end as i32) > (nrows as i32) {
                end = nrows;
            }
            while (row as i32) < (end as i32) {
                let c = list_cell(m, this, row, col);
                if c != 0 && vcall(m, host, c, 0x20, &[l(0x10)]) == 1 {
                    if mode == 0xffff_fffe || (mode == 1 && m.r32(this.wrapping_add(0x30c)) != 1) {
                        m.w8(this.wrapping_add(0x312), 1);
                        host.call(m, 0x4cd9f0, this, &[row]);
                    }
                    let c = list_cell(m, this, row, col);
                    vcall(m, host, c, 0x18, &[l(0x10), 0]);
                }
                row = row.wrapping_add(1);
            }
            col = col.wrapping_add(1);
            if (col as i32) >= (m.r32(this.wrapping_add(0xa4)) as i32) {
                break;
            }
        }
    }
}

/// `0x004CA640` (`edit_box_on_focus_gained`, SEH): al recibir el foco, construye una copia del texto (`vcall +0x14` + 8, CString) y, si el
/// límite de caracteres (`+0x168`) es −1, lo fija a la longitud del texto y copia `+0x15B` a `+0x15C` (`+0x118`). Después pasa el
/// texto a `0x4B0500` sobre el subcontrol `+0x120`, repinta (`+0xD4`), marca el campo activo (`0x6DD5DE`) y `+0x42`. Devuelve 1 en AL.
pub fn edit_box_on_focus_gained(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66249c);
    let slot = f.at(0x10);
    m.w32(slot, this); // `push ecx` reserva el hueco del CString
    let text = vcall(m, host, this, 0x14, &[]);
    host.call(m, 0x64efc8, slot, &[text.wrapping_add(8)]);
    f.seh_state(m, 0);
    if m.r32(this.wrapping_add(0x168)) == 0xffff_ffff {
        let s = m.r32(slot);
        let flag = m.r8(this.wrapping_add(0x15b));
        let len = m.r32(s.wrapping_sub(8));
        m.w32(this.wrapping_add(0x168), len);
        m.w8(this.wrapping_add(0x15c), flag);
        vcall(m, host, this, 0x118, &[]);
    }
    let text = vcall(m, host, this, 0x14, &[]);
    host.call(m, 0x4b0500, this.wrapping_add(0x120), &[text]);
    vcall(m, host, this, 0xd4, &[]);
    m.w8(0x6dd5de, 1);
    m.w8(this.wrapping_add(0x42), 1);
    f.seh_state(m, 0xffff_ffff);
    host.call(m, 0x64f253, slot, &[]);
    f.seh_leave(m);
    1
}

/// `0x0045C230` (`number_input_field_normalize_text`, SEH): quita los ceros a la izquierda del texto del campo numérico (mientras haya más
/// de un carácter y el primero sea `'0'`, `CString::Delete(0,1)` = `0x64CC80`), recorta el límite `+0x168` a la nueva longitud y escribe
/// el texto en el campo de edición (`0x4B0450`). Antes llama a `edit_box_on_focus_lost` (`0x4CA6F0`). Devuelve 1 en AL.
pub fn number_input_field_normalize_text(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x65ea0c);
    let slot = f.at(0x10);
    m.w32(slot, this);
    host.call(m, 0x4ca6f0, this, &[]);
    let edit = vcall(m, host, this, 0x14, &[]);
    host.call(m, 0x64efc8, slot, &[edit.wrapping_add(8)]);
    f.seh_state(m, 0);
    let mut s = m.r32(slot);
    let len = m.r32(s.wrapping_sub(8)) as i32;
    if len > 0 {
        let leading = |m: &mut dyn Mem, s: u32| m.r8(s) == 0x30 && (m.r32(s.wrapping_sub(8)) as i32) > 1;
        while leading(m, s) {
            host.call(m, 0x64cc80, slot, &[0, 1]);
            s = m.r32(slot);
            let l = m.r32(s.wrapping_sub(8));
            if (m.r32(this.wrapping_add(0x168)) as i32) > (l as i32) {
                m.w32(this.wrapping_add(0x168), l);
            }
        }
        host.call(m, 0x4b0450, edit, &[s]);
    }
    f.seh_state(m, 0xffff_ffff);
    host.call(m, 0x64f253, slot, &[]);
    f.seh_leave(m);
    1
}

/// `0x0045C930` (`number_input_field_set_value`, `ret 4`, SEH): fija el valor limitado a [`+0x184`, `+0x180`]; si cambia marca `+0x18C` y
/// guarda `+0x188`. Formatea el valor (`0x64DF80` con `0x698118`) en un CString local que ocupa el hueco del argumento y lo pasa al
/// campo de edición (`0x4CAAD0`).
pub fn number_input_field_set_value(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x65ea80);
    let mut v = m.r32(f.arg(0));
    let hi = m.r32(this.wrapping_add(0x180));
    if (v as i32) > (hi as i32) {
        v = hi;
    } else {
        let lo = m.r32(this.wrapping_add(0x184));
        if (v as i32) < (lo as i32) {
            v = lo;
        }
    }
    if v != m.r32(this.wrapping_add(0x188)) {
        m.w8(this.wrapping_add(0x18c), 1);
    }
    m.w32(this.wrapping_add(0x188), v);
    let empty = m.r32(0x6c7ccc);
    let obj = f.arg(0);
    m.w32(obj, empty);
    f.seh_state(m, 0);
    host.call(m, 0x64df80, 0, &[obj, 0x698118, v]);
    host.call(m, 0x4caad0, this, &[obj]);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x64f253, obj, &[]);
    f.seh_leave(m);
    r
}

/// `0x004C9F80` (`edit_box_on_mouse_event`, `ret 8`, SEH): pulsación (modo 1) sobre un campo de texto: coloca el cursor (`+0x168`) en el
/// carácter cuyo borde derecho supera la posición del ratón, midiendo prefijos del texto con la fuente de `0x6DCD28 + 168·(+0xD8)`
/// (`0x4C1B20`); `+0xF0` = 1 alinea a la izquierda, 3 al centro (resta media anchura). Si el ratón queda a la derecha del texto el
/// cursor va al final y si queda a la izquierda al principio; después notifica con `+0x118`. El CString local ocupa el hueco del modo.
pub fn edit_box_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x662400);
    let slot2 = f.at(0x10);
    m.w32(slot2, this);
    let (a0, mode) = (m.r32(f.arg(0)), m.r32(f.arg(1)));
    host.call(m, 0x4b1720, this, &[a0, mode]);
    if mode == 1 {
        let obj = f.arg(1);
        let text = vcall(m, host, this, 0x14, &[]);
        host.call(m, 0x64efc8, obj, &[text.wrapping_add(8)]);
        let x94 = m.r32(this.wrapping_add(0x94));
        let px = m.r32(a0);
        let mut edi = x94.wrapping_sub(px);
        f.seh_state(m, 0);
        let font = m.r32(this.wrapping_add(0xd8)).wrapping_mul(168).wrapping_add(0x6dcd28);
        let kind = m.r32(this.wrapping_add(0xf0));
        if kind == 1 {
            edi = x94;
        } else if kind == 3 {
            let sp = m.r32(obj);
            let w = host.call(m, 0x4c1b20, font, &[sp]) as i32;
            let half = (w.wrapping_sub(w >> 31)) >> 1;
            edi = m.r32(this.wrapping_add(0x94)).wrapping_sub(half as u32);
        }
        let sp = m.r32(obj);
        let len = m.r32(sp.wrapping_sub(8)) as i32;
        if len > 0 {
            let mut i: u32 = 0;
            loop {
                let r = host.call(m, 0x64d230, obj, &[slot2, i]);
                m.w8(f.at(4), 1);
                let d = m.r32(r);
                let font = m.r32(this.wrapping_add(0xd8)).wrapping_mul(168).wrapping_add(0x6dcd28);
                let w = host.call(m, 0x4c1b20, font, &[d]);
                m.w8(f.at(4), 0);
                let xp = m.r32(f.arg(0));
                let x = m.r32(xp);
                let over = (w.wrapping_add(edi) as i32) > (x as i32);
                host.call(m, 0x64f253, slot2, &[]);
                if over {
                    break;
                }
                let sp = m.r32(obj);
                m.w32(this.wrapping_add(0x168), i);
                i = i.wrapping_add(1);
                if (i as i32) >= (m.r32(sp.wrapping_sub(8)) as i32) {
                    break;
                }
            }
        }
        let sp = m.r32(obj);
        let xp = m.r32(f.arg(0));
                let x = m.r32(xp);
        let font = m.r32(this.wrapping_add(0xd8)).wrapping_mul(168).wrapping_add(0x6dcd28);
        let w = host.call(m, 0x4c1b20, font, &[sp]);
        let end = w.wrapping_add(edi);
        if (x as i32) > (end as i32) {
            let sp = m.r32(obj);
            let l = m.r32(sp.wrapping_sub(8));
            m.w32(this.wrapping_add(0x168), l);
        } else if (x as i32) < (edi as i32) {
            m.w32(this.wrapping_add(0x168), 0);
        }
        vcall(m, host, this, 0x118, &[]);
        f.seh_state(m, 0xffff_ffff);
        host.call(m, 0x64f253, obj, &[]);
    }
    f.seh_leave(m);
    0
}

/// `0x004CA110` (`edit_box_handle_key`, `ret 0xC`, SEH): teclado del campo de texto activo (`vcall +0xC4`). Copia el texto en un CString
/// (que ocupa el hueco del tercer argumento) y, con el cursor (`+0x168`) en −1, lo lleva al final. Flechas (0x25/0x27) mueven el cursor,
/// Supr (0x2E) y Retroceso (8) recortan el texto con `Right`/`Left`/`+`/`=` de CString (`0x64D180`, `0x64D230`, `0x64F436`, `0x64F340`),
/// Esc (0x1B) restaura el texto original (`+0x120`) y suelta el foco (`+0xC0`). Un carácter imprimible (0x20..0xFE) válido según la lista
/// de permitidos (`+0x164`, `0x64F6CE`) se inserta en el cursor si el texto no supera el ancho (`+0x170` con la fuente de `+0x174`) ni la
/// longitud máxima (`+0x16C`). Sin carácter, consume Inicio/Fin de la tabla de teclas (`0x6DCBCD/E3/E4`). Los CString temporales se
/// colocan en los huecos de pila y de argumentos del original, con su contador de destrucción SEH.
pub fn edit_box_handle_key(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x66248a);
    let active = vcall(m, host, this, 0xc4, &[]);
    if active as u8 == 0 {
        f.seh_leave(m);
        return active;
    }
    let (vk, a1, a2) = (m.r32(f.arg(0)), m.r32(f.arg(1)), m.r32(f.arg(2)));
    let a = f.arg(2);
    let (t1, t1b, t0c, t2, t3) = (f.at(0x18), f.at(0x14), f.at(0x10), f.arg(0), f.arg(1));
    let ch = host.call(m, 0x4c0f50, 0, &[vk, a1, a2]) & 0xffff;
    let edit = vcall(m, host, this, 0x14, &[]);
    host.call(m, 0x64efc8, a, &[edit.wrapping_add(8)]);
    f.seh_state(m, 0);
    let pos = |m: &mut dyn Mem| m.r32(this.wrapping_add(0x168));
    let len = |m: &mut dyn Mem| {
        let s = m.r32(a);
        m.r32(s.wrapping_sub(8))
    };
    let st = |m: &mut dyn Mem, v: u8| m.w8(f.at(4), v);
    if pos(m) == 0xffff_ffff {
        host.call(m, 0x4b0500, this.wrapping_add(0x100), &[edit]);
        let l = len(m);
        m.w32(this.wrapping_add(0x168), l);
    }
    let mut cursor_keys = true;
    match vk {
        0x25 => {
            let p = pos(m);
            if (p as i32) > 0 {
                m.w32(this.wrapping_add(0x168), p.wrapping_sub(1));
            }
        }
        0x27 => {
            let l = len(m);
            let p = pos(m);
            if (p as i32) < (l as i32) {
                m.w32(this.wrapping_add(0x168), p.wrapping_add(1));
            }
        }
        0x2e => {
            let n = len(m).wrapping_sub(pos(m)).wrapping_sub(1);
            let right = host.call(m, 0x64d180, a, &[t1, n]);
            let p = pos(m);
            st(m, 1);
            let left = host.call(m, 0x64d230, a, &[t2, p]);
            st(m, 2);
            let sum = host.call(m, 0x64f436, 0, &[t3, left, right]);
            st(m, 3);
            host.call(m, 0x64f340, a, &[sum]);
            st(m, 2);
            host.call(m, 0x64f253, t3, &[]);
            st(m, 1);
            host.call(m, 0x64f253, t2, &[]);
            st(m, 0);
            host.call(m, 0x64f253, t1, &[]);
            let s = m.r32(a);
            host.call(m, 0x4b0450, edit, &[s]);
            m.w8(this.wrapping_add(0x17b), 0);
        }
        8 => {
            if (pos(m) as i32) > 0 {
                let n = len(m).wrapping_sub(pos(m));
                let right = host.call(m, 0x64d180, a, &[t1, n]);
                let p = pos(m).wrapping_sub(1);
                st(m, 4);
                let left = host.call(m, 0x64d230, a, &[t2, p]);
                st(m, 5);
                let sum = host.call(m, 0x64f436, 0, &[t3, left, right]);
                st(m, 6);
                host.call(m, 0x64f340, a, &[sum]);
                st(m, 5);
                host.call(m, 0x64f253, t3, &[]);
                st(m, 4);
                host.call(m, 0x64f253, t2, &[]);
                st(m, 0);
                host.call(m, 0x64f253, t1, &[]);
                let s = m.r32(a);
                host.call(m, 0x4b0450, edit, &[s]);
                m.w8(this.wrapping_add(0x17b), 0);
                let p = pos(m).wrapping_sub(1);
                m.w32(this.wrapping_add(0x168), p);
            }
        }
        0x1b => {
            host.call(m, 0x4b0500, edit, &[this.wrapping_add(0x120)]);
            vcall(m, host, this, 0xc0, &[]);
        }
        _ => cursor_keys = false,
    }
    if !cursor_keys {
        let mut inserted = false;
        let mut accepted = false;
        if (ch as i32) >= 0x20 && (ch as i32) < 0xff {
            accepted = true;
            let list = m.r32(this.wrapping_add(0x164));
            if m.r32(list.wrapping_sub(8)) != 0 {
                let r = host.call(m, 0x64f6ce, this.wrapping_add(0x164), &[ch]);
                if r == 0xffff_ffff {
                    accepted = false;
                }
            }
        }
        if accepted {
            inserted = true;
            let empty = m.r32(0x6c7ccc);
            m.w32(t3, empty);
            let n = len(m).wrapping_sub(pos(m));
            st(m, 7);
            let right = host.call(m, 0x64d180, a, &[t0c, n]);
            let p = pos(m);
            st(m, 8);
            let left = host.call(m, 0x64d230, a, &[t1b, p]);
            st(m, 9);
            let plus = host.call(m, 0x64cb60, 0, &[t1, left, ch]);
            st(m, 0xa);
            let sum = host.call(m, 0x64f436, 0, &[t2, plus, right]);
            st(m, 0xb);
            host.call(m, 0x64f340, t3, &[sum]);
            st(m, 0xa);
            host.call(m, 0x64f253, t2, &[]);
            st(m, 9);
            host.call(m, 0x64f253, t1, &[]);
            st(m, 8);
            host.call(m, 0x64f253, t1b, &[]);
            st(m, 7);
            host.call(m, 0x64f253, t0c, &[]);
            let flag = m.r8(this.wrapping_add(0x179));
            m.w8(this.wrapping_add(0x17b), 0);
            if flag != 0 {
                let tp = m.r32(t3);
                let font = m.r32(this.wrapping_add(0x174)).wrapping_mul(168).wrapping_add(0x6dcd28);
                let w = host.call(m, 0x4c1b20, font, &[tp]);
                if (w as i32) >= (m.r32(this.wrapping_add(0x170)) as i32) {
                    m.w8(this.wrapping_add(0x17b), 1);
                }
            }
            let tp = m.r32(t3);
            let lim = m.r32(this.wrapping_add(0x16c));
            if (m.r32(tp.wrapping_sub(8)) as i32) > (lim as i32) {
                m.w8(this.wrapping_add(0x17b), 1);
            }
            if m.r8(this.wrapping_add(0x17b)) == 0 {
                let n = len(m).wrapping_sub(pos(m));
                let right = host.call(m, 0x64d180, a, &[t1, n]);
                let p = pos(m);
                st(m, 0xc);
                let left = host.call(m, 0x64d230, a, &[t1b, p]);
                st(m, 0xd);
                let plus = host.call(m, 0x64cb60, 0, &[t0c, left, ch]);
                st(m, 0xe);
                let sum = host.call(m, 0x64f436, 0, &[t2, plus, right]);
                st(m, 0xf);
                host.call(m, 0x64f340, a, &[sum]);
                st(m, 0xe);
                host.call(m, 0x64f253, t2, &[]);
                st(m, 0xd);
                host.call(m, 0x64f253, t0c, &[]);
                st(m, 0xc);
                host.call(m, 0x64f253, t1b, &[]);
                st(m, 7);
                host.call(m, 0x64f253, t1, &[]);
                let s = m.r32(a);
                host.call(m, 0x4b0450, edit, &[s]);
                let p = pos(m).wrapping_add(1);
                m.w32(this.wrapping_add(0x168), p);
            }
            st(m, 0);
            host.call(m, 0x64f253, t3, &[]);
        }
        if !inserted {
            if key_state_take(m, 0x6dcbcd) != 0 {
                vcall(m, host, this, 0xc0, &[]);
            } else if key_state_take(m, 0x6dcbe3) != 0 {
                let l = len(m);
                m.w32(this.wrapping_add(0x168), l);
            } else if key_state_take(m, 0x6dcbe4) != 0 {
                m.w32(this.wrapping_add(0x168), 0);
            }
        }
    }
    vcall(m, host, this, 0xd4, &[]);
    host.call(m, 0x4b1b10, this, &[]);
    vcall(m, host, this, 0x118, &[]);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x64f253, a, &[]);
    f.seh_leave(m);
    r
}

fn copy_bytes(m: &mut dyn Mem, dst: u32, src: u32, n: u32) {
    for i in 0..n {
        let b = m.r8(src.wrapping_add(i));
        m.w8(dst.wrapping_add(i), b);
    }
}

/// Cabecera de una orden de la cola (`0x00535740`: `-1` y cuatro ceros).
fn op_record_init(m: &mut dyn Mem, rec: u32) {
    m.w32(rec, 0xffff_ffff);
    for k in 1..5 {
        m.w32(rec.wrapping_add(4 * k), 0);
    }
}

/// `0x004247A0` (`game_screen_on_key_down`, `ret 0xC`, SEH): tecla pulsada en la pantalla de juego. Tras el manejador base (`0x4B6B80`):
/// Esc (0x1B) cierra el menú o activa la salida (`+0x2636`); dígitos 0–9 con la tecla consumida (`0x6DCBD1`) buscan el objeto elegido
/// (`0x4885A0`) y llaman a `0x4F5500`, o cambian de ciudad (`0x42D750`); Intro (o la tecla consumida `0x6DCBCD`) envía el texto del chat
/// (`+0x26D8`) troceado en bloques de 14 caracteres como órdenes `0xCD` (`0x54AA70`), con el comando `0xD1` si el texto es el comando
/// reservado (`0x6964D4`); Tab consulta Bloq Mayús/Mayús y actúa según la ventana de ciudad o el panel (`0x59B7B0`, `0x59AFE0`, `0x4290A0`,
/// `0x4292A0`); `+`/`−`/Pausa envían las órdenes `0xC9` de velocidad. Los CString viven en los huecos de los tres argumentos.
pub fn game_screen_on_key_down(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let l = |x: u32| f.at(0x38 - x);
    f.seh_enter(m, 0x65ce46);
    let modal = m.r8(0x6dd5de);
    let (vk, a1, a2) = (m.r32(f.arg(0)), m.r32(f.arg(1)), m.r32(f.arg(2)));
    host.call(m, 0x4b6b80, this, &[vk, a1, a2]);
    let rec = l(0x18);
    let (cs1, cs2, cs3) = (f.arg(0), f.arg(2), f.arg(1));
    let st = |m: &mut dyn Mem, v: u8| m.w8(f.at(4), v);
    'body: {
        if vk == 0x1b {
            m.w8(0x6dcbdb, 0);
            if host.call(m, 0x462310, 0, &[]) as u8 != 0 {
                host.call(m, 0x462230, 0, &[]);
            } else {
                m.w8(this.wrapping_add(0x2636), 1);
            }
            break 'body;
        }
        if vk >= 0x30 && vk <= 0x39 {
            let d = vk - 0x30;
            if key_state_take(m, 0x6dcbd1) != 0 {
                m.w8(0x6dcbd1, 0);
                m.w32(cs3, 0);
                m.w8(cs2, 0);
                let owner = m.r32(0x6ce6d0);
                let target = host.call(m, 0x4885a0, owner, &[cs3, cs2, 0]);
                if target == 0 {
                    break 'body;
                }
                let sel = m.r32(0x6dfc14);
                let obj = host.call(m, 0x5303c0, 0x6de4a0, &[sel]);
                let shift = u32::from(m.r8(0x6df2f0) & 0xc != 0);
                let cl = m.r8(cs2);
                let flag = u32::from(m.r8(0x66f354) & cl != 0);
                let v1 = m.r32(cs3);
                host.call(m, 0x4f5500, obj, &[d, target, v1, flag, shift]);
                break 'body;
            }
            if modal != 0 {
                break 'body;
            }
            if host.call(m, 0x462310, 0, &[]) as u8 != 0 {
                break 'body;
            }
            host.call(m, 0x42d750, this, &[d]);
            break 'body;
        }
        let v = key_state_take(m, 0x6dcbcd);
        m.w8(cs2, v);
        if vk == 0xd || v != 0 {
            let win = this.wrapping_add(0x2640);
            if vcall(m, host, win, 0xc8, &[]) as u8 == 0 {
                if modal != 0 {
                    break 'body;
                }
                if host.call(m, 0x462310, 0, &[]) as u8 != 0 {
                    break 'body;
                }
                vcall(m, host, win, 0x120, &[]);
                break 'body;
            }
            vcall(m, host, win, 0x118, &[]);
            let chat = this.wrapping_add(0x26d8);
            let t = host.call(m, 0x4d2ec0, chat, &[]);
            host.call(m, 0x64efc8, cs1, &[t]);
            let p = m.r32(cs1);
            f.seh_state(m, 0);
            if host.call(m, 0x63ac5b, 0, &[p, 0x6964d4]) == 0 {
                op_record_init(m, rec);
                let g = m.r32(0x6df31c);
                m.w32(rec, 0xd1);
                m.w32(rec.wrapping_add(4), g);
                host.call(m, 0x54aa70, 0x6df2f0, &[rec]);
            }
            let t = host.call(m, 0x4d2ec0, chat, &[]);
            host.call(m, 0x64efc8, cs2, &[t]);
            st(m, 1);
            let p2 = m.r32(cs2);
            if m.r32(p2.wrapping_sub(8)) != 0 {
                op_record_init(m, rec);
                let cl = m.r8(0x6dfc14);
                let g = m.r32(0x6df31c);
                let empty = m.r32(0x6c7ccc);
                m.w32(rec, 0xcd);
                m.w8(rec.wrapping_add(5), cl);
                m.w8(rec.wrapping_add(4), g as u8);
                m.w32(cs3, empty);
                let p2 = m.r32(cs2);
                st(m, 2);
                let tmp_a = l(0x10);
                let len = m.r32(p2.wrapping_sub(8));
                let cnt;
                if (len as i32) < 0xe {
                    let r = host.call(m, 0x64d230, cs2, &[tmp_a, len]);
                    st(m, 3);
                    host.call(m, 0x64f340, cs3, &[r]);
                    st(m, 2);
                    host.call(m, 0x64f253, tmp_a, &[]);
                    let p = m.r32(cs2);
                    cnt = m.r32(p.wrapping_sub(8));
                } else {
                    let r = host.call(m, 0x64d230, cs2, &[tmp_a, 0xe]);
                    st(m, 4);
                    host.call(m, 0x64f340, cs3, &[r]);
                    st(m, 2);
                    host.call(m, 0x64f253, tmp_a, &[]);
                    cnt = 0xe;
                }
                host.call(m, 0x64cc80, cs2, &[0, cnt]);
                let mut p3 = m.r32(cs3);
                while (m.r32(p3.wrapping_sub(8)) as i32) > 0xd {
                    copy_bytes(m, l(0x1e), p3, 0xe);
                    host.call(m, 0x54aa70, 0x6df2f0, &[rec]);
                    let p = m.r32(cs2);
                    let len = m.r32(p.wrapping_sub(8));
                    let cnt;
                    if (len as i32) < 0xe {
                        let r = host.call(m, 0x64d230, cs2, &[tmp_a, len]);
                        st(m, 5);
                        host.call(m, 0x64f340, cs3, &[r]);
                        st(m, 2);
                        host.call(m, 0x64f253, tmp_a, &[]);
                        let p = m.r32(cs2);
                        cnt = m.r32(p.wrapping_sub(8));
                    } else {
                        let tmp_b = l(0x14);
                        let r = host.call(m, 0x64d230, cs2, &[tmp_b, 0xe]);
                        st(m, 6);
                        host.call(m, 0x64f340, cs3, &[r]);
                        st(m, 2);
                        host.call(m, 0x64f253, tmp_b, &[]);
                        cnt = 0xe;
                    }
                    host.call(m, 0x64cc80, cs2, &[0, cnt]);
                    p3 = m.r32(cs3);
                }
                let n = m.r32(p3.wrapping_sub(8));
                copy_bytes(m, l(0x1e), p3, n);
                let n2 = m.r32(p3.wrapping_sub(8));
                m.w8(l(0x1e).wrapping_add(n2), 0);
                host.call(m, 0x54aa70, 0x6df2f0, &[rec]);
                st(m, 1);
                host.call(m, 0x64f253, cs3, &[]);
            }
            st(m, 0);
            host.call(m, 0x64f253, cs2, &[]);
            f.seh_state(m, 0xffff_ffff);
            host.call(m, 0x64f253, cs1, &[]);
            break 'body;
        }
        match vk {
            0x6b | 0x6d | 0x13 => {
                if modal != 0 && vk != 0x13 {
                    break 'body;
                }
                if m.r32(0x6dfc18) != 0 || m.r32(0x6df31c) != 0 || m.r8(0x6e59cc) != 0xff {
                    break 'body;
                }
                op_record_init(m, rec);
                m.w32(rec, 0xc9);
                let val = if vk == 0x13 { 4 } else { u32::from(vk != 0x6b) };
                m.w32(rec.wrapping_add(4), val);
                host.call(m, 0x54aa70, 0x6df2f0, &[rec]);
            }
            9 => {
                let fp = m.r32(0x66a544);
                let caps = host.call(m, fp, 0, &[0x14]);
                let shift = host.call(m, fp, 0, &[0x10]);
                let ne = u32::from(((caps & 1) | (shift & 0x8000)) != 0);
                let w = m.r32(0x6e51ac);
                if host.call(m, 0x4b2200, w, &[]) as u8 == 0 {
                    host.call(m, 0x4292a0, this, &[1, ne]);
                    break 'body;
                }
                let o = m.r32(0x6e54e8);
                if vcall(m, host, o, 0xc8, &[]) as u8 != 0 {
                    let p = m.r32(o.wrapping_add(0x94));
                    if p != m.r32(0x66d4f8) && p != m.r32(0x66d500) {
                        let w = m.r32(0x6e51ac);
                        host.call(m, 0x59b7b0, w, &[ne]);
                        break 'body;
                    }
                }
                let w = m.r32(0x6e51ac);
                let thr = i32::from(m.r8(0x677937));
                let val = m.r32(w.wrapping_add(0xc324));
                if (val as i32) >= thr {
                    host.call(m, 0x59afe0, w, &[ne]);
                } else {
                    host.call(m, 0x4290a0, this, &[ne, val]);
                }
            }
            _ => {}
        }
    }
    f.seh_leave(m);
    0
}

/// Valor `f32` guardado en `addr` como doble (lo que carga `fld dword`).
fn ld32(m: &mut dyn Mem, addr: u32) -> f64 {
    f64::from(f32::from_bits(m.r32(addr)))
}

/// `fstp dword`: redondea a `f32` y guarda (la FPU está a 53 bits: la operación se hace en `f64` y se redondea al guardar).
fn st32(m: &mut dyn Mem, addr: u32, v: f64) {
    m.w32(addr, (v as f32).to_bits());
}

/// `_ftol` (`0x00639C28`): trunca ST0 a entero de 64 bits y devuelve la palabra baja; fuera de rango o NaN da el «indefinido» (0).
fn crt_ftol(v: f64) -> u32 {
    if v.is_nan() || v.abs() >= 9.223_372_036_854_775_8e18 {
        0
    } else {
        (v.trunc() as i64) as u32
    }
}

/// Valor del control deslizante tomado de la posición del indicador: con límite fijo `+0x374` = −1.0f vale `+0x358`; si no interpola entre
/// `+0x350` y `+0x354` según la posición (`0x66F4A4`, `0x66BC58`) con las operaciones `f32`/53 bits del original. Los temporales ocupan
/// los huecos de los tres argumentos (`slot0..2`).
fn slider_value_from_thumb(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, slots: [u32; 3]) -> u32 {
    let sub = this.wrapping_add(0x244);
    let pos = vcall(m, host, sub, 0x44, &[]);
    m.w32(slots[2], pos);
    st32(m, slots[2], f64::from(pos as i32));
    let q = ld32(m, slots[2]) / ld32(m, 0x66f4a4);
    st32(m, slots[2], q);
    if m.r32(this.wrapping_add(0x374)) == 0xbf80_0000 {
        return m.r32(this.wrapping_add(0x358));
    }
    let span = m.r32(this.wrapping_add(0x354)).wrapping_sub(m.r32(this.wrapping_add(0x350)));
    m.w32(slots[1], span);
    st32(m, slots[1], f64::from(span as i32));
    let t = (ld32(m, this.wrapping_add(0x374)) - ld32(m, slots[2])) * ld32(m, slots[1]);
    st32(m, slots[0], t);
    let w = vcall(m, host, this, 0x44, &[]);
    m.w32(slots[1], w);
    st32(m, slots[1], f64::from(w as i32));
    let q2 = ld32(m, slots[2]) + ld32(m, slots[2]);
    let y = ld32(m, slots[1]) - q2;
    let z = ld32(m, slots[0]) / y;
    let lo = f64::from(m.r32(this.wrapping_add(0x350)) as i32);
    st32(m, slots[2], lo);
    let z = z + ld32(m, slots[2]) + ld32(m, 0x66bc58);
    st32(m, slots[2], z);
    crt_ftol(ld32(m, slots[2]))
}

/// `0x004CFB30` (`ret 0xC`): teclas del control deslizante: tras el manejador base (`0x4B1850`) calcula el valor actual y, con la tecla
/// `0x6DCBE5` consumida lo reduce en 1, con `0x6DCBE7` lo aumenta en 1, y lo fija con `0x4D0BD0`.
pub fn slider_handle_key(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let args: Vec<u32> = (0..3).map(|i| m.r32(f.arg(i))).collect();
    host.call(m, 0x4b1850, this, &args);
    let mut value = slider_value_from_thumb(m, host, this, [f.arg(0), f.arg(1), f.arg(2)]);
    if key_state_take(m, 0x6dcbe5) != 0 {
        value = value.wrapping_sub(1);
    } else if key_state_take(m, 0x6dcbe7) != 0 {
        value = value.wrapping_add(1);
    }
    host.call(m, 0x4d0bd0, this, &[value])
}

/// `0x004CF890` (`ret 8`): ratón sobre el control deslizante. Mide el indicador (`+0x244`) y el control, calcula el centro del indicador
/// con el valor actual (límite fijo `+0x374` = −1.0f → `+0x358` entre `+0x350/+0x354`, si no interpolado en `f32`), coloca el punto
/// en coordenadas del indicador y, según el modo: −2 (fuera del indicador) mueve el valor ±1 hacia el ratón con `0x4D0BD0`; 1 (pulsación
/// sobre el indicador o arrastre en curso `+0x395`) inicia el arrastre con `0x4D0CB0`. Los temporales `f32` usan los huecos de los
/// argumentos.
pub fn slider_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let l = |x: u32| f.at(0x44 - x);
    let (a0, mode) = (m.r32(f.arg(0)), m.r32(f.arg(1)));
    host.call(m, 0x4b1720, this, &[a0, mode]);
    let sub = this.wrapping_add(0x244);
    vcall(m, host, sub, 0x40, &[l(0x2c)]);
    vcall(m, host, this, 0x40, &[l(0x20)]);
    let half = |v: u32| (((v as i32).wrapping_sub((v as i32) >> 31)) >> 1) as u32;
    let sub_w = vcall(m, host, sub, 0x44, &[]);
    let ebx = half(sub_w);
    let ebp;
    if m.r32(this.wrapping_add(0x374)) == 0xbf80_0000 {
        let cur = vcall(m, host, this, 0x44, &[]);
        let lo = m.r32(this.wrapping_add(0x350));
        let hi = m.r32(this.wrapping_add(0x354));
        let base = m.r32(this.wrapping_add(0x358));
        m.w32(l(0x24), 0);
        let span = hi.wrapping_sub(lo) as i32;
        let q = ((base as i32 as i64) / (span as i64)) as i32;
        ebp = cur.wrapping_sub(ebx.wrapping_mul(2)).wrapping_mul(q as u32).wrapping_add(ebx);
    } else {
        ebp = crt_ftol(ld32(m, this.wrapping_add(0x374)));
        let e = crt_ftol(ld32(m, this.wrapping_add(0x378)));
        m.w32(l(0x24), e);
    }
    vcall(m, host, this, 0x40, &[l(0x38)]);
    let px = m.r32(a0);
    let x = half(m.r32(l(0x2c))).wrapping_sub(ebp).wrapping_add(px);
    m.w32(l(0x18), x);
    let h = vcall(m, host, this, 0x48, &[]).wrapping_sub(m.r32(l(0x30)));
    let py = m.r32(a0.wrapping_add(4));
    m.w32(l(0x1c), py.wrapping_sub(half(h)));
    let inside = vcall(m, host, sub, 0x20, &[l(0x18)]);
    let mut grab = false;
    if inside == 1 && vcall(m, host, this, 0xac, &[]) as u8 != 0 {
        vcall(m, host, sub, 0xa4, &[mode]);
        grab = true;
    }
    if !grab && vcall(m, host, this, 0xac, &[]) as u8 != 0 {
        vcall(m, host, sub, 0xa8, &[0]);
    }
    vcall(m, host, sub, 0x18, &[l(0x18), mode]);
    let value = slider_value_from_thumb(m, host, this, [l(0x14), l(0x10), f.arg(1)]);
    m.w32(f.arg(1), value);
    if mode == 0xffff_fffe {
        if vcall(m, host, sub, 0x20, &[l(0x18)]) == 0 {
            let mut v = m.r32(f.arg(1));
            let px = m.r32(a0) as i32;
            if px < ebp as i32 {
                v = v.wrapping_sub(1);
            } else if px > m.r32(l(0x24)) as i32 {
                v = v.wrapping_add(1);
            }
            m.w32(f.arg(1), v);
            let r = host.call(m, 0x4d0bd0, this, &[v]);
            m.w8(this.wrapping_add(0x395), 0);
            return r;
        }
        m.w8(this.wrapping_add(0x395), 0);
        return 0;
    }
    if mode == 1 {
        let r = vcall(m, host, sub, 0x20, &[l(0x18)]);
        if r == 1 || m.r8(this.wrapping_add(0x395)) != 0 {
            m.w8(this.wrapping_add(0x395), 1);
            m.w32(l(0x28), 0);
            let (cx, cy) = (m.r32(a0) as i32, m.r32(a0.wrapping_add(4)) as i32);
            st32(m, l(0x20), f64::from(cx));
            st32(m, l(0x24), f64::from(cy));
            return host.call(m, 0x4d0cb0, this, &[l(0x20)]);
        }
    }
    m.w8(this.wrapping_add(0x395), 0);
    0
}

/// `0x00461990` (`ret 8`): ratón en la zona de arrastre de la lista con barra vertical propia: con modo −2 y estado 3 (`+4`), si el punto
/// está por encima/debajo de la cabecera de la barra (`+0x264`) desplaza el desfase `+0x36C` en `+0x370` (hacia arriba con tope 0, hacia
/// abajo con tope `+0x364 − +0x370`), recoloca el contenido (`+0x264` método `0x60`/`0x68`) en la posición proporcional (`f32` y `_ftol`) y
/// recompone el rectángulo de recorte con la API `0x66A4CC` (dos veces: la segunda es la del cierre común).
pub fn scroll_area_on_mouse_event(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let l = |x: u32| f.at(0x24 - x);
    let (pt, mode) = (m.r32(f.arg(0)), m.r32(f.arg(1)));
    let state = m.r32(this.wrapping_add(4));
    host.call(m, 0x4b1720, this, &[pt, mode]);
    let sub = this.wrapping_add(0x264);
    let (s28, s2c) = (f.arg(0), f.arg(1));
    let rect_call = |m: &mut dyn Mem, host: &mut dyn OpsHost| {
        let (w, h) = (m.r32(this.wrapping_add(0x2c)), m.r32(this.wrapping_add(0x30)));
        m.w32(l(0x14), 0);
        m.w32(l(0x1c), w);
        m.w32(l(0x18), 0);
        m.w32(l(0x20), h);
        let dst = this.wrapping_add(0x4c);
        let fp = m.r32(0x66a4cc);
        host.call(m, fp, 0, &[dst, dst, l(0x14)])
    };
    if mode == 0xffff_fffe && state == 3 {
        let y = m.r32(pt.wrapping_add(4)) as i32;
        let top = vcall(m, host, sub, 0x70, &[]) as i32;
        // `compute` = Some(ebp) cuando hay que recolocar
        let mut reposition = false;
        if y < top {
            if m.r8(this.wrapping_add(0x360)) != 0 && m.r32(this.wrapping_add(0x36c)) != 0 {
                let v = m.r32(this.wrapping_add(0x36c)).wrapping_sub(m.r32(this.wrapping_add(0x370)));
                m.w32(this.wrapping_add(0x36c), v);
                if (v as i32) < 0 {
                    m.w32(this.wrapping_add(0x36c), 0);
                }
                reposition = true;
            }
        } else {
            let a = vcall(m, host, sub, 0x70, &[]);
            let b = vcall(m, host, sub, 0x48, &[]);
            if y > a.wrapping_add(b) as i32 && m.r8(this.wrapping_add(0x360)) != 0 {
                let step = m.r32(this.wrapping_add(0x370));
                let maxv = m.r32(this.wrapping_add(0x364)).wrapping_sub(step);
                let cur = m.r32(this.wrapping_add(0x36c));
                if cur != maxv {
                    let nv = step.wrapping_add(cur);
                    m.w32(this.wrapping_add(0x36c), if (nv as i32) > (maxv as i32) { maxv } else { nv });
                    reposition = true;
                }
            }
        }
        if reposition {
            let h = vcall(m, host, sub, 0x48, &[]);
            let l10 = m.r32(this.wrapping_add(0x368)).wrapping_sub(h);
            m.w32(l(0x10), l10);
            let off = m.r32(this.wrapping_add(0x36c));
            m.w32(s2c, off);
            if off != 0 {
                st32(m, s28, f64::from(off as i32));
                let rng = m.r32(this.wrapping_add(0x364)).wrapping_sub(m.r32(this.wrapping_add(0x370)));
                m.w32(s2c, rng);
                st32(m, s2c, f64::from(rng as i32));
                let q = ld32(m, s28) / ld32(m, s2c);
                st32(m, s2c, q);
            } else {
                m.w32(s2c, 0);
            }
            let inner = vcall(m, host, this.wrapping_add(0x94), 0x48, &[]);
            st32(m, s28, f64::from(l10 as i32));
            let prod = ld32(m, s28) * ld32(m, s2c);
            let ftol = crt_ftol(prod);
            let ebp = inner.wrapping_add(ftol).wrapping_add(m.r32(this.wrapping_add(0x18)));
            vcall(m, host, sub, 0x60, &[l(0x14)]);
            m.w32(l(0x18), ebp);
            vcall(m, host, sub, 0x68, &[l(0x14)]);
            rect_call(m, host);
        }
        return rect_call(m, host);
    }
    0
}

/// `atof` del CRT (`0x0063B37B`) restringido a `[espacios][signo]dígitos[.dígitos]` (sin exponente): devuelve el `f64` correctamente
/// redondeado (el original devuelve un doble en ST0).
fn crt_atof(m: &mut dyn Mem, p: u32) -> f64 {
    let mut i = 0u32;
    while matches!(m.r8(p.wrapping_add(i)), 9..=13 | 0x20) {
        i += 1;
    }
    let mut s = String::new();
    let c = m.r8(p.wrapping_add(i));
    if c == b'-' || c == b'+' {
        if c == b'-' {
            s.push('-');
        }
        i += 1;
    }
    let mut digits = 0;
    while m.r8(p.wrapping_add(i)).is_ascii_digit() {
        s.push(m.r8(p.wrapping_add(i)) as char);
        i += 1;
        digits += 1;
    }
    if m.r8(p.wrapping_add(i)) == b'.' {
        s.push('.');
        i += 1;
        while m.r8(p.wrapping_add(i)).is_ascii_digit() {
            s.push(m.r8(p.wrapping_add(i)) as char);
            i += 1;
            digits += 1;
        }
    }
    if digits == 0 {
        return 0.0;
    }
    s.parse::<f64>().unwrap_or(0.0)
}

/// Recorta el texto `a` del campo con `Right`/`Left`/`+`/`=` de CString y deja los temporales en los huecos `t1..t3`, con los contadores
/// SEH `s1..s3` (Supr 1,2,3; Retroceso 4,5,6). Después escribe el texto en el campo de edición.
#[allow(clippy::too_many_arguments)]
fn cs_cut(m: &mut dyn Mem, host: &mut dyn OpsHost, f: &Frame, a: u32, t: [u32; 3], right_n: u32, left_n: u32, s: [u8; 3]) {
    let right = host.call(m, 0x64d180, a, &[t[0], right_n]);
    m.w8(f.at(4), s[0]);
    let left = host.call(m, 0x64d230, a, &[t[1], left_n]);
    m.w8(f.at(4), s[1]);
    let sum = host.call(m, 0x64f436, 0, &[t[2], left, right]);
    m.w8(f.at(4), s[2]);
    host.call(m, 0x64f340, a, &[sum]);
    m.w8(f.at(4), s[1]);
    host.call(m, 0x64f253, t[2], &[]);
    m.w8(f.at(4), s[0]);
    host.call(m, 0x64f253, t[1], &[]);
    m.w8(f.at(4), 0);
    host.call(m, 0x64f253, t[0], &[]);
}

/// `Format(a, fmt, val)` + escribir en el campo + cursor al final (final común de los recortes por límite).
fn num_reformat(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: u32, edit: u32, fmt: u32, val: u32) {
    host.call(m, 0x64df80, 0, &[a, fmt, val]);
    let p = m.r32(a);
    host.call(m, 0x4b0450, edit, &[p]);
    let p = m.r32(a);
    let l = m.r32(p.wrapping_sub(8));
    m.w32(this.wrapping_add(0x168), l);
}

/// `0x0045C300` (`number_input_field_handle_key_input`, `ret 0xC`, SEH, x87): teclado del campo numérico. Como `edit_box_handle_key` pero:
/// Supr/Retroceso (teclas `0x6DCBEE`/`0x6DCBC8`) dejan el texto con `"0"` si queda vacío y lo vuelven a formatear con el mínimo `+0x184`
/// si es menor; solo se aceptan dígitos, tras los que se compara el valor (`atoi`/`atof`) con el máximo `+0x180` y el mínimo `+0x184`
/// (reformateando) y se quita un cero inicial si el valor es menor que la constante de `0x66DCD8`. Al final guarda el valor (`+0x188`).
pub fn number_input_field_handle_key_input(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    f.seh_enter(m, 0x65ea6e);
    let active = vcall(m, host, this, 0xc4, &[]);
    if active as u8 == 0 {
        f.seh_leave(m);
        return active;
    }
    let (vk, a1, a2) = (m.r32(f.arg(0)), m.r32(f.arg(1)), m.r32(f.arg(2)));
    let (a, t2, t3) = (f.arg(1), f.arg(0), f.arg(2));
    let (t1, t1b, t0) = (f.at(0x1c), f.at(0x18), f.at(0x14));
    m.w8(this.wrapping_add(0x18c), 1);
    let ch = host.call(m, 0x4c0f50, 0, &[vk, a1, a2]) & 0xffff;
    let edit = vcall(m, host, this, 0x14, &[]);
    host.call(m, 0x64efc8, a, &[edit.wrapping_add(8)]);
    f.seh_state(m, 0);
    if m.r32(this.wrapping_add(0x168)) == 0xffff_ffff {
        host.call(m, 0x4b0500, this.wrapping_add(0x100), &[edit]);
        let p = m.r32(a);
        let l = m.r32(p.wrapping_sub(8));
        m.w32(this.wrapping_add(0x168), l);
    }
    let pos = |m: &mut dyn Mem| m.r32(this.wrapping_add(0x168));
    let len = |m: &mut dyn Mem| {
        let p = m.r32(a);
        m.r32(p.wrapping_sub(8))
    };
    // tecla consumida: el original deja el valor previo en el byte bajo del hueco del tercer argumento
    let take = |m: &mut dyn Mem, addr: u32| {
        let v = key_state_take(m, addr);
        m.w8(t3, v);
        v
    };
    'body: {
        if take(m, 0x6dcbe5) != 0 {
            let p = pos(m);
            if (p as i32) > 0 {
                m.w32(this.wrapping_add(0x168), p.wrapping_sub(1));
            }
            break 'body;
        }
        if take(m, 0x6dcbe7) != 0 {
            let l = len(m);
            let p = pos(m);
            if (p as i32) < (l as i32) {
                m.w32(this.wrapping_add(0x168), p.wrapping_add(1));
            }
            break 'body;
        }
        if take(m, 0x6dcbee) != 0 {
            let n = len(m).wrapping_sub(pos(m)).wrapping_sub(1);
            let p = pos(m);
            cs_cut(m, host, &f, a, [t1, t2, t3], n, p, [1, 2, 3]);
            let s = m.r32(a);
            host.call(m, 0x4b0450, edit, &[s]);
            if len(m) == 0 {
                host.call(m, 0x64f390, a, &[0x698100]);
                let s = m.r32(a);
                host.call(m, 0x4b0450, edit, &[s]);
                let l = len(m);
                m.w32(this.wrapping_add(0x168), l);
            }
            let s = m.r32(a);
            let v = host.call(m, 0x639cda, 0, &[s]);
            let lo = m.r32(this.wrapping_add(0x184));
            if (v as i32) < (lo as i32) {
                num_reformat(m, host, this, a, edit, 0x698104, lo);
            }
            break 'body;
        }
        if take(m, 0x6dcbc8) != 0 {
            let n = len(m).wrapping_sub(pos(m));
            let p = pos(m).wrapping_sub(1);
            cs_cut(m, host, &f, a, [t1, t2, t3], n, p, [4, 5, 6]);
            let s = m.r32(a);
            host.call(m, 0x4b0450, edit, &[s]);
            let p = pos(m);
            if (p as i32) > 0 {
                m.w32(this.wrapping_add(0x168), p.wrapping_sub(1));
            }
            if len(m) == 0 {
                host.call(m, 0x64f390, a, &[0x698108]);
                let s = m.r32(a);
                host.call(m, 0x4b0450, edit, &[s]);
                let l = len(m);
                m.w32(this.wrapping_add(0x168), l);
            }
            let s = m.r32(a);
            let v = host.call(m, 0x639cda, 0, &[s]);
            let lo = m.r32(this.wrapping_add(0x184));
            if (v as i32) < (lo as i32) {
                num_reformat(m, host, this, a, edit, 0x69810c, lo);
            }
            break 'body;
        }
        if take(m, 0x6dcbdb) != 0 {
            host.call(m, 0x4b0500, edit, &[this.wrapping_add(0x120)]);
            break 'body;
        }
        if ch >= 0x30 && ch <= 0x39 {
            let n = len(m).wrapping_sub(pos(m));
            let right = host.call(m, 0x64d180, a, &[t0, n]);
            let p = pos(m);
            m.w8(f.at(4), 7);
            let left = host.call(m, 0x64d230, a, &[t1b, p]);
            m.w8(f.at(4), 8);
            let plus = host.call(m, 0x64cb60, 0, &[t1, left, ch]);
            m.w8(f.at(4), 9);
            let sum = host.call(m, 0x64f436, 0, &[t2, plus, right]);
            m.w8(f.at(4), 0xa);
            host.call(m, 0x64f340, a, &[sum]);
            m.w8(f.at(4), 9);
            host.call(m, 0x64f253, t2, &[]);
            m.w8(f.at(4), 8);
            host.call(m, 0x64f253, t1, &[]);
            m.w8(f.at(4), 7);
            host.call(m, 0x64f253, t1b, &[]);
            m.w8(f.at(4), 0);
            host.call(m, 0x64f253, t0, &[]);
            let s = m.r32(a);
            host.call(m, 0x4b0450, edit, &[s]);
            let np = pos(m).wrapping_add(1);
            m.w32(this.wrapping_add(0x168), np);
            let s = m.r32(a);
            let iv = host.call(m, 0x639cda, 0, &[s]);
            let s = m.r32(a);
            host.call(m, 0x63b37b, 0, &[s]);
            let fv = crt_atof(m, s);
            let hi = m.r32(this.wrapping_add(0x180));
            st32(m, t3, fv);
            m.w32(f.at(0x10), hi);
            st32(m, f.at(0x10), f64::from(hi as i32));
            let fv32 = ld32(m, t3);
            let reformat = if !(fv32 > ld32(m, f.at(0x10))) {
                let lo = m.r32(this.wrapping_add(0x184));
                if (iv as i32) < (lo as i32) { Some((0x698114, lo)) } else { None }
            } else {
                Some((0x698110, hi))
            };
            if let Some((fmt, val)) = reformat {
                num_reformat(m, host, this, a, edit, fmt, val);
            }
            let small = ld32(m, t3) < ld32(m, 0x66dcd8);
            if small && (len(m) as i32) > 1 {
                host.call(m, 0x64cc80, a, &[0, 1]);
                let s = m.r32(a);
                host.call(m, 0x4b0450, edit, &[s]);
                let l = len(m);
                m.w32(this.wrapping_add(0x168), l);
            }
            break 'body;
        }
        if key_state_take(m, 0x6dcbcd) != 0 {
            vcall(m, host, this, 0xc0, &[]);
        } else if key_state_take(m, 0x6dcbe3) != 0 {
            let l = len(m);
            m.w32(this.wrapping_add(0x168), l);
        } else if key_state_take(m, 0x6dcbe4) != 0 {
            m.w32(this.wrapping_add(0x168), 0);
        }
    }
    vcall(m, host, this, 0xd4, &[]);
    host.call(m, 0x4b1b10, this, &[]);
    vcall(m, host, this, 0x118, &[]);
    let s = m.r32(a);
    let v = host.call(m, 0x639cda, 0, &[s]);
    m.w32(this.wrapping_add(0x188), v);
    f.seh_state(m, 0xffff_ffff);
    let r = host.call(m, 0x64f253, a, &[]);
    f.seh_leave(m);
    r
}

fn r_u8(m: &mut dyn Mem, a: u32) -> u32 {
    u32::from(m.r8(a))
}

/// Mayor potencia de diez (1, 10, 100…) que alcanza `|v|/10` (escalones del contador animado de `0x423840`): `0x639F78` = `abs`.
fn pow10_step(m: &mut dyn Mem, host: &mut dyn OpsHost, diff: i32) -> u32 {
    let q = ((i64::from(diff) * 0x6666_6667) >> 32) as i32 >> 2;
    let q = q.wrapping_add(((q as u32) >> 31) as i32);
    let a = host.call(m, 0x639f78, 0, &[q as u32]);
    let mut p: u32 = 1;
    if a > p {
        loop {
            p = p.wrapping_mul(10);
            if !(p < a) {
                break;
            }
        }
    }
    p
}

/// `0x00423840` (`game_screen_update_hud_and_hotkeys`, SEH): actualización por cuadro de la pantalla de juego (HUD y atajos). Por orden:
/// cierra el aviso si venció (`+0x2604`); con el modo de pantalla de `0x6DFC1C` reinicia ventanas y vuelve al cambio de resolución
/// (`0x4BCCC0`) y lo reconstruye; reaplica el menú; actualiza el contador animado de dinero (`+0x1238`, `+0x7C0` con saltos de potencia
/// de diez, texto con puntos de millar), las coordenadas del cursor (`+0x2610..`), los botones de la barra de la ciudad
/// (`610D69` → bits), y despacha los atajos de las barras (teclas consumidas `0x6DCC35/36/3B`) a las ventanas que corresponden.
/// Devuelve −5 / −1 (según `+0x1234`) o −3 si hay que salir.
pub fn game_screen_update_hud_and_hotkeys(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let f = Frame::new(esp);
    let l = |x: u32| f.at(0xa4 - x);
    f.seh_enter(m, 0x65ce04);
    let o = |off: u32| this.wrapping_add(off);
    // --- aviso (ventana +0x2528)
    let win = o(0x2528);
    if vcall(m, host, win, 0xc8, &[]) as u8 != 0 {
        let a = m.r32(o(0x2604));
        let b = m.r32(0x6dccf8);
        let mut go = true;
        if a > b {
            go = vcall(m, host, win, 0x28, &[]) == 3;
        }
        if go {
            host.call(m, 0x4ad8d0, win, &[0, 1]);
            vcall(m, host, win, 0xcc, &[0]);
            vcall(m, host, o(0x18c0), 0xcc, &[1]);
            host.call(m, 0x4b4eb0, this, &[win]);
            m.w32(o(0x2604), 0);
        }
    }
    // --- cambio de modo de pantalla
    let ebp = o(0x16f0);
    let mut exit_val: Option<u32> = None;
    'frame: {
        if m.r32(0x6dfc1c) == m.r32(0x673ca8) {
            let c = m.r32(0x6cc7e8);
            if vcall(m, host, c, 0xc8, &[]) as u8 != 0 {
                vcall(m, host, c, 0x118, &[]);
                if (m.r8(o(0x2637)) | m.r8(o(0x2639))) != 0 {
                    host.call(m, 0x42ce50, this, &[]);
                }
                if m.r8(o(0x2637)) != 0 {
                    exit_val = Some(0xffff_fffd);
                    break 'frame;
                }
                m.w8(o(0x2639), 0);
            }
            m.w8(o(0x125c), 0);
            let n = r_u8(m, o(0x488));
            if n != 0 {
                let mut i: u32 = 0;
                let mut ob = o(0x168);
                let mut slot = o(0x7d0);
                loop {
                    m.w32(slot, 0);
                    vcall(m, host, ob, 0xcc, &[0]);
                    i += 1;
                    let n = r_u8(m, o(0x488));
                    slot = slot.wrapping_add(4);
                    ob = ob.wrapping_add(0xa0);
                    if (i as i32) >= (n as i32) {
                        break;
                    }
                }
            }
            m.w8(o(0x488), 0);
            m.w32(o(0x48c), 0);
            m.w32(o(0x490), 0);
            host.call(m, 0x441810, 0, &[]);
            host.call(m, 0x462280, 0, &[1]);
            let (e1, e2) = (m.r32(0x66e0ac), m.r32(0x6cba64));
            host.call(m, 0x4b90e0, 0x6da5f0, &[e2, e1]);
            host.call(m, 0x4b9150, 0x6da5f0, &[this]);
            let d = m.r32(0x6cba64);
            vcall(m, host, d, 0xcc, &[1]);
            let d = m.r32(0x6cba64);
            host.call(m, 0x401300, d, &[]);
            let d = m.r32(0x6cba64);
            vcall(m, host, d, 0x120, &[]);
            let d = m.r32(0x6cba64);
            vcall(m, host, d, 0xcc, &[0]);
            let e1 = m.r32(0x66e090);
            host.call(m, 0x4b90e0, 0x6da5f0, &[this, e1]);
            let d = m.r32(0x6cba64);
            host.call(m, 0x4b9150, 0x6da5f0, &[d]);
            let e = m.r32(0x6cbb40);
            vcall(m, host, e, 0xf4, &[]);
            let fo = m.r32(0x6cbdd8);
            if host.call(m, 0x4b2200, fo, &[]) as u8 != 0 {
                let fo = m.r32(0x6cbdd8);
                host.call(m, 0x453df0, fo, &[]);
                let fo = m.r32(0x6cbdd8);
                m.w8(fo.wrapping_add(0x2b22), 1);
            } else {
                let g = m.r32(0x6e51ac);
                let v = m.r32(g.wrapping_add(0xc324));
                host.call(m, 0x58a4f0, g, &[v, 0, 0, 0, 0]);
            }
        }
        // --- reinicio por petición (+0x2636) o botón (+0x16f0)
        let hit = host.call(m, 0x4c78b0, ebp, &[]) as u8;
        if (hit | m.r8(o(0x2636))) != 0 {
            let j = m.r32(0x6cbdb8);
            host.call(m, 0x443940, j, &[]);
            let j = m.r32(0x6cbdb8);
            host.call(m, 0x4438a0, j, &[]);
            m.w8(o(0x125c), 0);
            m.w8(o(0x2636), 0);
            for k in 0..0x40 {
                m.w32(0x6dcbc0 + 4 * k, 0);
            }
            host.call(m, 0x441810, 0, &[]);
            host.call(m, 0x462280, 0, &[1]);
            host.call(m, 0x440f70, 0, &[0, 0]);
            let (e1, e2) = (m.r32(0x66e098), m.r32(0x6cbdf0));
            host.call(m, 0x4b90e0, 0x6da5f0, &[e2, e1]);
            host.call(m, 0x4b9150, 0x6da5f0, &[this]);
            let h = m.r32(0x6cbdf0);
            host.call(m, 0x45e3d0, h, &[]);
            let h = m.r32(0x6cbdf0);
            if vcall(m, host, h, 0x120, &[]) == 0xffff_fffb {
                vcall(m, host, ebp, 0x2c, &[1]);
                exit_val = Some(0xffff_fffd);
                break 'frame;
            }
            // estructura de modo de vídeo en el marco: dos CString y los tamaños
            let empty = m.r32(0x6c7ccc);
            let (cs_a, cs_b) = (l(0x44), l(0x48));
            m.w32(cs_a, empty);
            m.w32(cs_b, empty);
            m.w32(l(0xa0), 0);
            m.w8(l(0xa0), 1);
            host.call(m, 0x64f390, cs_a, &[0x6cbb4c]);
            host.call(m, 0x64f390, cs_b, &[0x6cbb4c]);
            for k in 0..5 {
                m.w32(l(0x4c) + 4 * k, 0);
            }
            for k in 0..6 {
                m.w32(l(0x60) + 4 * k, 0);
            }
            m.w32(l(0x78), 1);
            m.w32(l(0x7c), 0xac44);
            m.w32(l(0x80), 0x10);
            m.w32(l(0x84), 2);
            let md = m.r32(0x6cc3e8);
            m.w32(l(0xa0), 2);
            match m.r8(md.wrapping_add(0x1f)) {
                0 => {
                    m.w32(l(0x4c), 0x320);
                    m.w32(l(0x50), 0x258);
                }
                1 => {
                    m.w32(l(0x4c), 0x400);
                    m.w32(l(0x50), 0x300);
                }
                2 => {
                    m.w32(l(0x4c), 0x500);
                    m.w32(l(0x50), 0x400);
                }
                _ => {}
            }
            match m.r8(md.wrapping_add(0x20)) {
                0 => m.w32(l(0x54), 0x10),
                1 => m.w32(l(0x54), 0x20),
                _ => {}
            }
            let a0 = m.r32(0x6cc40c);
            if host.call(m, 0x4bccc0, 0, &[a0, cs_a]) != 0 {
                host.call(m, 0x429a40, this, &[]);
                let (w, h) = (m.r32(l(0x4c)), m.r32(l(0x50)));
                vcall(m, host, this, 0x4c, &[w, h, 0]);
                vcall(m, host, this, 0xd4, &[]);
            } else {
                let md = m.r32(0x6cc3e8);
                m.w8(md.wrapping_add(0x1f), 0);
                let md = m.r32(0x6cc3e8);
                m.w8(md.wrapping_add(0x20), 0);
            }
            let e1 = m.r32(0x66e090);
            host.call(m, 0x4b90e0, 0x6da5f0, &[this, e1]);
            let h = m.r32(0x6cbdf0);
            host.call(m, 0x4b9150, 0x6da5f0, &[h]);
            vcall(m, host, ebp, 0x2c, &[1]);
            let md = m.r32(0x6cc3e8);
            let bl = (ebp & 0xffff_ff00) | u32::from(m.r8(md.wrapping_add(0x25)) != 0);
            host.call(m, 0x4b8520, this, &[bl]);
            host.call(m, 0x4b8380, this, &[0]);
            let g = m.r32(0x6e51ac);
            let t = m.r32(0x6dccf8);
            m.w32(g.wrapping_add(0x410), t);
            let e = m.r32(0x6cbb40);
            vcall(m, host, e, 0xf4, &[]);
            m.w32(l(0xa0), 3);
            host.call(m, 0x64f253, cs_b, &[]);
            m.w32(l(0xa0), 0xffff_ffff);
            host.call(m, 0x64f253, cs_a, &[]);
        }
        // --- avisos y llamadas fijas
        if (m.r8(o(0x2639)) | m.r8(o(0x2637))) != 0 {
            host.call(m, 0x42ce50, this, &[]);
            if m.r8(o(0x2637)) != 0 {
                exit_val = Some(0xffff_fffd);
                break 'frame;
            }
        }
        host.call(m, 0x42c080, this, &[]);
        let c = m.r32(0x6cc7e8);
        if m.r8(c.wrapping_add(0x3ea)) != 0 {
            host.call(m, 0x469700, c, &[]);
        }
        let sel = m.r32(0x6dfc14);
        m.w32(l(0x14), sel);
        if host.call(m, 0x4c78b0, o(0x1d48), &[]) as u8 != 0 {
            let i = m.r32(0x6cba90);
            vcall(m, host, i, 0x120, &[]);
        } else if host.call(m, 0x4c78b0, o(0x1e30), &[]) as u8 != 0 {
            let g = m.r32(0x6e51ac);
            host.call(m, 0x59a1e0, g, &[]);
        }
        // --- texto de la fecha en el recuadro (+0x1238)
        let mut ebx = m.r32(0x6de4b4);
        if ((m.r32(o(0x1258)) ^ ebx) & 0xffff_ff00) != 0 {
            let text = o(0x1260);
            let place = |m: &mut dyn Mem, host: &mut dyn OpsHost, str_ptr: u32| {
                let c = m.r32(o(0x1238));
                vcall(m, host, c, 0x84, &[l(0x38)]);
                let c = m.r32(o(0x1238));
                let h = vcall(m, host, c, 0x6c, &[]);
                let w = m.r32(l(0x38));
                let right = h.wrapping_add(w).wrapping_add(2);
                let font_ix = m.r32(o(0x2608));
                let font = font_ix.wrapping_mul(168).wrapping_add(0x6dcd28);
                m.w32(l(0x20), right);
                let tw = host.call(m, 0x4c1b20, font, &[str_ptr]);
                let x = m.r32(l(0x20)).wrapping_sub(tw);
                m.w32(l(0x18), x);
                let y = m.r32(o(0x1264)).wrapping_add(m.r32(l(0x3c)));
                m.w32(l(0x1c), y);
                let fo = m.r32(o(0x2608)).wrapping_mul(168).wrapping_add(0x6dcd28);
                vcall(m, host, fo, 0x40, &[l(0x8c)]);
                let yy = m.r32(l(0x1c));
                let bottom = m.r32(l(0x90)).wrapping_add(yy);
                m.w32(l(0x24), bottom);
                host.call(m, 0x4b9650, 0x6da5f0, &[l(0x18)]);
            };
            let s0 = m.r32(text);
            place(m, host, s0);
            let a = m.r32(0x6de4a0);
            let cx = u32::from(m.r16(0x6de4a2));
            let idx = (a >> 8) & 0xff;
            let tb = m.r32(0x6c0d04 + idx * 4);
            host.call(m, 0x64df80, 0, &[text, 0x6964b4, a & 0xff, tb, cx]);
            let s1 = m.r32(text);
            place(m, host, s1);
        }
        // --- animación del indicador +0x1238 (barra de progreso de la fecha)
        let ebx_low = ebx & 0xff;
        let a1254 = m.r32(o(0x1254));
        let obj1238 = m.r32(o(0x1238));
        let mut skip_to_store = false;
        if m.r8(o(0x125c)) == 0 {
            if ebx_low <= a1254 {
                let r = host.call(m, 0x4c5850, obj1238.wrapping_add(0x94), &[1]);
                let q1 = r.wrapping_sub(1).wrapping_mul(ebx_low) / a1254;
                let q2 = 200u32.wrapping_mul(a1254.wrapping_sub(ebx_low)) / a1254;
                host.call(m, 0x4af740, obj1238, &[1, q2, q1, 1, 1]);
                m.w8(o(0x125c), 1);
                skip_to_store = true;
            }
        }
        if !skip_to_store && ebx >= m.r32(o(0x1258)) {
            let c = m.r32(o(0x1254));
            let low = ebx & 0xff;
            if low > c {
                let q = low / c;
                let obj = m.r32(o(0x1238));
                let cur = m.r32(obj.wrapping_add(0xa8));
                let mut go = true;
                if q == cur {
                    go = host.call(m, 0x4afa40, obj, &[]) == 1;
                }
                if go {
                    let obj = m.r32(o(0x1238));
                    host.call(m, 0x4af740, obj, &[0, 0, q, 0, 1]);
                    m.w8(o(0x125c), 0);
                }
            }
        }
        // --- contador animado
        let sel = m.r32(l(0x14));
        m.w32(o(0x1258), ebx);
        let tptr = host.call(m, 0x5303c0, 0x6de4a0, &[sel]);
        let target = m.r32(tptr);
        if target != m.r32(o(0x164)) {
            let mut cur = m.r32(o(0x7c0));
            let up = (cur as i32) > (target as i32);
            m.w8(l(0x13), u8::from(up));
            ebx = 0;
            loop {
                let step = pow10_step(m, host, cur.wrapping_sub(target) as i32);
                ebx = ebx.wrapping_add(m.r32(o(0x1250)));
                if up {
                    cur = cur.wrapping_add(0xffff_fff9u32.wrapping_sub(step));
                    if (cur as i32) >= (target as i32) {
                        continue;
                    }
                } else {
                    cur = cur.wrapping_add(step).wrapping_add(7);
                    if (cur as i32) <= (target as i32) {
                        continue;
                    }
                }
                break;
            }
            let old = m.r32(o(0x164));
            let ob = m.r32(o(0x1230));
            if (target as i32) > (old as i32) {
                if host.call(m, 0x4afa40, ob, &[]) != 0 {
                    let ob = m.r32(o(0x1230));
                    host.call(m, 0x4af740, ob, &[0, ebx, 0, 1, 1]);
                }
            } else if (target as i32) < (old as i32) {
                if host.call(m, 0x4afa40, ob, &[]) != 1 {
                    let ob = m.r32(o(0x1230));
                    host.call(m, 0x4af740, ob, &[1, ebx, 0, 1, 1]);
                }
            }
            m.w32(o(0x164), target);
            let t = m.r32(0x6dccf8);
            m.w32(o(0x124c), t);
        }
        let cur164 = m.r32(o(0x164));
        let c7c0 = m.r32(o(0x7c0));
        if cur164 != c7c0 && m.r32(0x6dccf8) >= m.r32(o(0x124c)) {
            host.call(m, 0x428c70, this, &[]);
            let dt = m.r32(0x6dccf8).wrapping_sub(m.r32(o(0x124c)));
            let mut n = dt / m.r32(o(0x1250));
            if (n as i32) >= 0 {
                loop {
                    let a = m.r32(o(0x7c0));
                    let b = m.r32(o(0x164));
                    if a == b {
                        break;
                    }
                    let step = pow10_step(m, host, a.wrapping_sub(b) as i32);
                    let t = m.r32(o(0x164));
                    let c = m.r32(o(0x7c0));
                    if (t as i32) < (c as i32) {
                        let nv = c.wrapping_sub(step).wrapping_sub(7);
                        m.w32(o(0x7c0), nv);
                        if (nv as i32) < (t as i32) {
                            m.w32(o(0x7c0), t);
                        }
                    } else if (t as i32) > (c as i32) {
                        let nv = c.wrapping_add(step).wrapping_add(7);
                        m.w32(o(0x7c0), nv);
                        if (nv as i32) > (t as i32) {
                            m.w32(o(0x7c0), t);
                        }
                    }
                    n = n.wrapping_sub(1);
                    if (n as i32) < 0 {
                        break;
                    }
                }
            }
            // texto con separadores de millar
            let buf = o(0x123c);
            let v = m.r32(o(0x7c0));
            host.call(m, 0x63ab8a, 0, &[buf, 0x10, 0x6964c0, v]);
            let len = host.call(m, 0x63abe0, 0, &[buf]);
            if (len as i32) >= 4 {
                let mut end = len.wrapping_add(buf);
                let mut p = end.wrapping_sub(3);
                if p > buf {
                    loop {
                        if m.r8(p.wrapping_sub(1)) == 0x2d {
                            break;
                        }
                        let n = end.wrapping_sub(p);
                        memmove_bytes(m, p.wrapping_add(1), p, n);
                        m.w8(p, 0x2e);
                        end = end.wrapping_add(1);
                        p = p.wrapping_sub(3);
                        m.w8(end, 0);
                        if !(p > buf) {
                            break;
                        }
                    }
                }
            }
            host.call(m, 0x428c70, this, &[]);
            let t = m.r32(o(0x1250)).wrapping_add(m.r32(0x6dccf8));
            m.w32(o(0x124c), t);
        }
        // --- coordenadas del cursor
        let mode = m.r8(o(0x2634));
        if mode == 1 {
            let mut x_changed = !(((m.r32(0x6dd8a0) & 0xffff) == m.r32(o(0x2610))) && (u32::from(m.r16(0x6dd8a2)) == m.r32(o(0x260c))));
            if x_changed {
                host.call(m, 0x428e00, this, &[]);
                let v = u32::from(m.r16(0x6dd8a2));
                m.w32(o(0x260c), v);
                let a = m.r32(0x6dd8a0) & 0xffff;
                m.w32(o(0x2610), a);
                host.call(m, 0x64df80, 0, &[o(0x2620), 0x6964c4, a]);
                let v = u32::from(m.r16(0x6dd8a2));
                host.call(m, 0x64df80, 0, &[o(0x261c), 0x6964c8, v]);
                host.call(m, 0x428e00, this, &[]);
                x_changed = false;
            }
            let _ = x_changed;
            let y_changed = !((u32::from(m.r16(0x6dd8a6)) == m.r32(o(0x2614))) && ((m.r32(0x6dd8a4) & 0xffff) == m.r32(o(0x2618))));
            if y_changed {
                host.call(m, 0x428f50, this, &[]);
                let v = u32::from(m.r16(0x6dd8a6));
                m.w32(o(0x2614), v);
                let a = m.r32(0x6dd8a4) & 0xffff;
                m.w32(o(0x2618), a);
                host.call(m, 0x64df80, 0, &[o(0x2628), 0x6964cc, a]);
                let v = u32::from(m.r16(0x6dd8a6));
                host.call(m, 0x64df80, 0, &[o(0x2624), 0x6964d0, v]);
                host.call(m, 0x428f50, this, &[]);
            }
        } else if mode == 0 {
            host.call(m, 0x42afc0, this, &[]);
        }
        // --- botones de la ciudad según el estado de 0x610D69
        if m.r8(0x6e59cc) != 0xff {
            let ix = r_u8(m, 0x6e59cc);
            let town = m.r32(0x6e55d0 + 4 * ix);
            let bits = host.call(m, 0x610d69, town, &[]);
            // los bits se pasan con los 24 bits altos que dejó el registro (`mov al, …` no limpia EAX)
            let hi = |n: u32| (bits >> n) & 0xffff_ff00;
            let b = |n: u32| (bits >> n) & 1;
            vcall(m, host, o(0x2000), 0x34, &[(bits & 0xffff_ff00) | b(4)]);
            vcall(m, host, o(0x20e8), 0x34, &[b(8)]);
            let r3 = vcall(m, host, o(0x21d0), 0x34, &[hi(12) | b(12)]);
            vcall(m, host, o(0x22b8), 0x34, &[(r3 & 0xffff_ff00) | b(0)]);
            vcall(m, host, o(0x23a0), 0xcc, &[hi(15) | b(15)]);
            let vp = m.r32(o(0x22b8));
            m.w32(l(0x14), vp);
            let vis = vcall(m, host, o(0x23a0), 0xc8, &[]);
            let rc = vcall(m, host, o(0x22b8), 0xcc, &[(vis & 0xffff_ff00) | u32::from(vis as u8 == 0)]);
            vcall(m, host, o(0x2000), 0x124, &[(rc & 0xffff_ff00) | b(5)]);
            vcall(m, host, o(0x20e8), 0x124, &[hi(9) | b(9)]);
            vcall(m, host, o(0x21d0), 0x124, &[hi(13) | b(13)]);
            vcall(m, host, o(0x22b8), 0x124, &[(bits & 0xffff_ff00) | b(1)]);
        }
        // --- atajos de las barras
        hud_hotkeys(m, host, this, &f);
    }
    if let Some(v) = exit_val {
        f.seh_leave(m);
        return v;
    }
    // --- cierre común
    if m.r32(o(0x800)) != m.r32(0x6dfbc4) {
        host.call(m, 0x42d230, this, &[]);
    }
    host.call(m, 0x42b920, this, &[]);
    host.call(m, 0x42be40, this, &[]);
    let z = u32::from(m.r8(o(0x1234)) == 0);
    f.seh_leave(m);
    z.wrapping_mul(4).wrapping_sub(5)
}

/// Copia con solapamiento correcto (`memcpy` del CRT con rutina de retroceso).
fn memmove_bytes(m: &mut dyn Mem, dst: u32, src: u32, n: u32) {
    let tmp: Vec<u8> = (0..n).map(|i| m.r8(src.wrapping_add(i))).collect();
    for (i, b) in tmp.into_iter().enumerate() {
        m.w8(dst.wrapping_add(i as u32), b);
    }
}

/// Atajos del cuadro (`0x424452..0x42473A`): teclas consumidas de las barras de la ciudad y del menú.
fn hud_hotkeys(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, f: &Frame) {
    let l = |x: u32| f.at(0xa4 - x);
    let o = |off: u32| this.wrapping_add(off);
    'keys: {
        // (1) barra 0x1c60 → 0x6dcc35
        let w1 = o(0x1c60);
        let mut to_f2 = host.call(m, 0x4c78b0, w1, &[]) as u8 != 0;
        if !to_f2 && vcall(m, host, w1, 0x38, &[]) as u8 != 0 {
            to_f2 = key_state_take(m, 0x6dcc35) != 0;
        }
        if to_f2 {
            // 4246f2
            let lo = m.r32(0x6cc950);
            if vcall(m, host, lo, 0xc8, &[]) as u8 != 1 {
                host.call(m, 0x462280, 0, &[0]);
                let cx = m.r16(0x6dfc14);
                let al = m.r8(0x66ecf4);
                m.w16(l(0x16), cx);
                let lo = m.r32(0x6cc950);
                m.w8(l(0x14), al);
                host.call(m, 0x475e40, lo, &[l(0x14)]);
            }
            m.w8(0x6dcc35, 0);
            break 'keys;
        }
        // (2) barra 0x18c0 → 0x6dcc36
        let w2 = o(0x18c0);
        let mut to_df = host.call(m, 0x4c78b0, w2, &[]) as u8 != 0;
        if !to_df && vcall(m, host, w2, 0x38, &[]) as u8 != 0 {
            to_df = key_state_take(m, 0x6dcc36) != 0;
        }
        if to_df {
            host.call(m, 0x47d1f0, 0x6cc960, &[]);
            m.w8(0x6dcc36, 0);
            break 'keys;
        }
        let ob = m.r32(o(0x1230));
        if host.call(m, 0x4af910, ob, &[]) as u8 != 0 {
            host.call(m, 0x495bb0, 0x6ce6d8, &[]);
            break 'keys;
        }
        let ob = m.r32(o(0x1238));
        if host.call(m, 0x4af910, ob, &[]) as u8 != 0 {
            host.call(m, 0x47dd40, 0x6cc960, &[]);
            break 'keys;
        }
        // (3) barra 0x1608 → 0x6dcc3b
        let w3 = o(0x1608);
        let mut to_691 = host.call(m, 0x4c78b0, w3, &[]) as u8 != 0;
        if !to_691 && vcall(m, host, w3, 0x38, &[]) as u8 != 0 {
            to_691 = key_state_take(m, 0x6dcc3b) != 0;
        }
        if to_691 {
            for k in 0..0x40 {
                m.w32(0x6dcbc0 + 4 * k, 0);
            }
            let rec = l(0x18);
            m.w32(rec, 0xffff_ffff);
            for k in 1..5 {
                m.w32(rec + 4 * k, 0);
            }
            m.w32(rec, 0xc8);
            m.w32(rec + 4, 0xffff_ffff);
            m.w32(rec + 8, 0xffff_ffff);
            m.w32(rec + 12, 1);
            m.w32(rec + 16, 0xffff_ffff);
            host.call(m, 0x54aa70, 0x6df2f0, &[rec]);
            m.w8(0x6dcc3b, 0);
            break 'keys;
        }
        if host.call(m, 0x4c78b0, o(0x17d8), &[]) as u8 != 0 {
            host.call(m, 0x441810, 0, &[]);
            host.call(m, 0x462280, 0, &[1]);
            for k in 0..0x40 {
                m.w32(0x6dcbc0 + 4 * k, 0);
            }
            host.call(m, 0x549040, 0x6df2f0, &[]);
            break 'keys;
        }
        if host.call(m, 0x4c78b0, o(0x19a8), &[]) as u8 != 0 {
            let k = m.r32(0x6cc95c);
            vcall(m, host, k, 0x118, &[0]);
            break 'keys;
        }
        if host.call(m, 0x4c78b0, o(0x1f18), &[]) as u8 == 0 {
            break 'keys;
        }
        // lista de edificios del comerciante → petición 0x134
        let ix = r_u8(m, 0x6e59cc);
        let town = m.r32(0x6e55d0 + 4 * ix);
        m.w32(l(0x2c), 0);
        m.w32(l(0x30), 0);
        m.w8(l(0x34), 0);
        let sel = m.r32(0x6dfc14);
        let first = u32::from(m.r16(town.wrapping_add(0x35a)));
        let (mut di, count);
        if first == sel {
            di = u32::from(m.r16(town.wrapping_add(0x10)));
            count = u32::from(m.r16(town.wrapping_add(0x658)));
        } else {
            di = u32::from(m.r16(town.wrapping_add(0x65c)));
            count = u32::from(m.r16(town.wrapping_add(0x65a)));
        }
        m.w32(l(0x2c), count);
        if count == 0 {
            break 'keys;
        }
        let buf = host.call(m, 0x64f7b9, 0, &[count.wrapping_mul(2)]);
        m.w32(l(0x30), buf);
        if di != 0xffff {
            let mut off: u32 = 0;
            loop {
                let t4 = m.r32(town.wrapping_add(4));
                let ent = m.r32(t4.wrapping_add(di * 4));
                let w = m.r16(ent.wrapping_add(4));
                off = off.wrapping_add(2);
                m.w16(buf.wrapping_add(off).wrapping_sub(2), w);
                let tc = m.r32(town.wrapping_add(0xc));
                di = u32::from(m.r16(tc.wrapping_add(di * 2)));
                if di == 0xffff {
                    break;
                }
            }
        }
        if m.r32(l(0x2c)) == 1 {
            let a = m.r8(0x66f352);
            let c = m.r8(l(0x34));
            m.w8(l(0x34), c | a);
        }
        let mo = m.r32(0x6ce6d0);
        vcall(m, host, mo, 0x134, &[l(0x2c)]);
    }
    // 42473a: tras los atajos
}
