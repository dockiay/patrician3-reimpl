//! Ayudantes de coordenadas de la escena: `0x58B740` (paso en la rejilla isométrica), `0x58D030` (píxel a casilla) y `0x630CC0` (marca en la máscara del lado).
//! Traducción 1:1 del x86; la memoria pasa por [`Mem`].

/// Memoria plana direccionable (lecturas y escrituras de 8/16/32 bits).
pub trait Mem {
    /// Devuelve al sistema las páginas enteras de `[a, a + len)` si la memoria es dispersa (leerlas después da 0). Por defecto
    /// no hace nada.
    fn release(&mut self, _a: u32, _len: u32) {}
    /// Optional view of the same storage for core ports. UI bridges use this
    /// to avoid stacking a new pair of byte-access adapters at every call.
    /// Other memories retain the ordinary adapter path.
    fn core_view(&mut self) -> Option<&mut dyn p3_core::flatmem::Mem> { None }
    fn read_bytes(&mut self, a: u32, out: &mut [u8]) {
        for (i,b) in out.iter_mut().enumerate() { *b=self.r8(a.wrapping_add(i as u32)); }
    }
    fn write_bytes(&mut self, a: u32, data: &[u8]) {
        for (i,&b) in data.iter().enumerate() { self.w8(a.wrapping_add(i as u32),b); }
    }
    /// Compara dos cadenas C (bytes sin signo, la que acaba antes es menor, como mucho `max` bytes de cada una; con `fold`,
    /// plegando ASCII a minúsculas) si la memoria sabe hacerlo sin pasar byte a byte por la interfaz; `None` si no.
    fn cmp_cstr(&mut self, _a: u32, _b: u32, _fold: bool, _max: usize) -> Option<std::cmp::Ordering> {
        None
    }
    fn r8(&mut self, a: u32) -> u8;
    fn r16(&mut self, a: u32) -> u16 {
        u16::from(self.r8(a)) | u16::from(self.r8(a.wrapping_add(1))) << 8
    }
    fn r32(&mut self, a: u32) -> u32 {
        u32::from(self.r16(a)) | u32::from(self.r16(a.wrapping_add(2))) << 16
    }
    fn w8(&mut self, a: u32, v: u8);
    fn w16(&mut self, a: u32, v: u16) {
        self.w8(a, v as u8);
        self.w8(a.wrapping_add(1), (v >> 8) as u8);
    }
    fn w32(&mut self, a: u32, v: u32) {
        self.w16(a, v as u16);
        self.w16(a.wrapping_add(2), (v >> 16) as u16);
    }
}

/// `0x58B740(&a, &b, dirección)`: mueve la casilla `(a, b)` un paso en la dirección (octantes de 0x20) y devuelve si sigue dentro del mapa.
pub fn step_dir(m: &mut dyn Mem, scene: u32, a: &mut u32, b: &mut u32, dir: u8) -> bool {
    match dir {
        0x00 => *b = b.wrapping_sub(2),
        0x40 => *a = a.wrapping_add(1),
        0x80 => *b = b.wrapping_add(2),
        0xC0 => *a = a.wrapping_sub(1),
        0x20 => {
            *a = a.wrapping_add(*b & 1);
            *b = b.wrapping_sub(1);
        }
        0x60 => {
            *b = b.wrapping_add(1);
            *a = a.wrapping_add(!*b & 1);
        }
        0xA0 => {
            *b = b.wrapping_add(1);
            *a = a.wrapping_sub(*b & 1);
        }
        0xE0 => {
            *b = b.wrapping_sub(1);
            *a = a.wrapping_sub(*b & 1);
        }
        _ => {}
    }
    *a < m.r32(scene + 0xC314) && *b < m.r32(scene + 0xC318)
}

/// `0x58D030(x, y, &salida)`: devuelve `(casilla_x, casilla_y, dentro)`. `dentro` es el valor de `AL`.
pub fn pixel_to_cell(m: &mut dyn Mem, scene: u32, x: i32, y: i32) -> (u32, u32, bool) {
    let w = m.r32(scene + 0xC31C);
    let h = m.r32(scene + 0xC320);
    let mut inside = true;
    let mut px = x as u32;
    if x < 0 {
        px = 0;
        inside = false;
    } else if px >= w {
        px = w.wrapping_sub(1);
        inside = false;
    }
    let mut py = y as u32;
    if y < 0 {
        py = 0;
        inside = false;
    } else if py >= h {
        py = h.wrapping_sub(1);
        inside = false;
    }
    let c514 = m.r32(scene + 0x514).wrapping_add(1) as i32;
    let c510 = m.r32(scene + 0x510) as i32;
    let q = (py as i32).wrapping_div(c514);
    let mut cb = (q.wrapping_mul(2)) as u32;
    let mut ca = ((px as i32).wrapping_div(c510)) as u32;
    let stride = m.r32(scene + 0xC6E8) as i32;
    let base = m.r32(scene + 0xC6D8);
    let row = (py as i32).wrapping_sub(c514.wrapping_mul(q)).wrapping_add(1);
    let addr = (row.wrapping_mul(stride).wrapping_sub(c510.wrapping_mul(ca as i32)) as u32).wrapping_add(base).wrapping_add(1).wrapping_add(px);
    let t = m.r8(addr);
    if t < 0xE1 {
        let ok = step_dir(m, scene, &mut ca, &mut cb, t);
        inside &= ok;
        if !ok {
            let w = m.r32(scene + 0xC314);
            if (ca as i32) < 0 {
                ca = 0;
            } else if ca >= w {
                ca = w.wrapping_sub(1);
            }
            let h = m.r32(scene + 0xC318);
            if (cb as i32) < 0 {
                cb = 0;
            } else if cb >= h {
                cb = h.wrapping_sub(1);
            }
        }
    }
    (ca, cb, inside)
}

/// `0x630CC0(ecx = registro del lado, a, b, bits)`: activa `bits` en la máscara `[registro+0x70]` de la casilla `(a, b)` si el registro está activo.
pub fn mark_mask(m: &mut dyn Mem, rec: u32, scene: u32, a: i16, b: i16, bits: u8) {
    let w3a = m.r16(rec + 0x3A);
    let skip = if w3a == 0 { m.r16(rec + 0x21E) == 0 } else { w3a <= m.r16(rec + 0x6A) };
    let mask = m.r32(rec + 0x70);
    if skip || mask == 0 || a < 0 || b < 0 {
        return;
    }
    let (w, h) = (m.r32(scene + 0xC314), m.r32(scene + 0xC318));
    if (a as u32) < w && (b as u32) < h {
        let i = (b as i32).wrapping_mul(w as i32).wrapping_add(i32::from(a)) as u32;
        let v = m.r8(mask.wrapping_add(i)) | bits;
        m.w8(mask.wrapping_add(i), v);
    }
}
