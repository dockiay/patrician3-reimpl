//! Escombros de un edificio caído: `0x599040(x, y)` (celdas del objeto, polvo `0x12` y sonido ambiente).
//! Traducción 1:1 del x86; la escena se maneja con [`SceneMem`](crate::ambience::SceneMem) y los externos pasan por [`DebrisEnv`].
use crate::ambience::SceneMem;

pub trait DebrisEnv {
    fn rand(&mut self) -> i32;
    /// `0x58DAC0(&x, &salida)`: devuelve el par de salida.
    fn pair(&mut self, x: u32, y: u32) -> (u32, u32);
    /// `0x58D030(a, b, &salida)`
    fn cell_of(&mut self, a: i32, b: i32) -> (u32, u32);
    /// `0x590960(&p, &q, 1)`
    fn adjust(&mut self, p: u32, q: u32) -> (u32, u32);
    /// Muestra apuntada por `registro+0xC` (identificada por el puntero).
    fn sample(&mut self, ptr: u32) -> u32;
    /// `AIL_end_sample(muestra)` y `0x4430D0(muestra)`
    fn end_sample(&mut self, s: u32);
    /// Valor de la casilla `indice = c314 * y + x`
    fn cell(&mut self, idx: u32) -> u16;
    /// Métodos virtuales `+0x48`, `+0x4C` y `+0x64` del objeto del mapa
    fn obj_w(&mut self, h: u16) -> u32;
    fn obj_h(&mut self, h: u16) -> u32;
    fn obj_r(&mut self, h: u16) -> i32;
    /// `0x58DD70(x, y, h, 1, 0)`
    fn f58dd70(&mut self, x: u32, y: u32, h: u16);
    /// `0x58DA10(c6ec, x+1, y+1)`: `true` si la casilla está ocupada.
    fn occupied(&mut self, table: u32, x: u32, y: u32) -> bool;
    /// `0x58B740(&x, &y, dir)`
    fn advance(&mut self, x: &mut u32, y: &mut u32, dir: u32) -> bool;
    fn make(&mut self, ty: u32) -> u32;
    fn new_link(&mut self) -> u16;
    fn register(&mut self, h: u32, link: u16);
    fn spawn(&mut self, h: u32, x: u32, y: u32, a: u8, link: u16, b: u32);
    /// `0x4B2200()`
    fn audio_ready(&mut self) -> bool;
    /// `0x4B9730() == [0x6CBB40]`
    fn audio_match(&mut self) -> bool;
    /// `0x443690(c994)`
    fn sound_count(&mut self, base: u32) -> i32;
    /// `0x594B50(a, b, &salida18, &param2)`: devuelve `(salida18, param2)`.
    fn f594b50(&mut self, a: u32, b: u32) -> (u32, u32);
    /// `0x443150(...)`
    /// `0x443150(id, 0, 0, a, b, 0, 0)`
    fn play(&mut self, id: i32, a: u32, b: u32);
}

pub fn update(m: &mut SceneMem, arg_x: u32, arg_y: u32, env: &mut dyn DebrisEnv) {
    let mut x = arg_x.wrapping_add(m.r32(0x528));
    let mut y = arg_y.wrapping_add(m.r32(0x52C));
    if !(x < m.r32(0xC314) && y < m.r32(0xC318)) {
        return;
    }
    let (o1, o2) = env.pair(x, y);
    for k in 0..4 {
        let r = 0xC998 + 0x18 * k;
        if m.r8(r + 0x16) != 0 && u16::from_le_bytes([m.r8(r + 0x14), m.r8(r + 0x15)]) == 0 {
            let half = |v: i32| (v - (v >> 31)) >> 1; // cdq/sub/sar: división entre 2 hacia cero
            let cx = half(m.r32(0x510) as i32).wrapping_add(m.r32(r) as i32);
            let ay = (m.r32(r + 4) as i32).wrapping_sub(half(m.r32(0x514) as i32));
            let (p, q) = env.cell_of(cx, ay);
            let (p, q) = env.adjust(p, q);
            if p == x && q == y {
                let ptr = m.r32(r + 0xC);
                let s = env.sample(ptr);
                env.end_sample(s);
                for i in 0..0x18 {
                    m.w8(r + i, 0);
                }
            }
        }
    }
    m.w32(0x508, 1);
    let idx = m.r32(0xC314).wrapping_mul(y).wrapping_add(x);
    let h = env.cell(idx);
    if h == 0 {
        return;
    }
    let ww = env.obj_w(h);
    let hh = env.obj_h(h);
    if env.obj_r(h) <= 1 {
        env.f58dd70(x, y, h);
    }
    let mut al = x < m.r32(0xC314) && y < m.r32(0xC318);
    let mut outer = hh;
    while outer != 0 {
        let (sx, sy) = (x, y);
        let mut inner = ww;
        while inner != 0 {
            if al {
                let odd = y & 1 != 0;
                let yh = y >> 1;
                for _ in 0..3 {
                    let r = env.rand();
                    let cw = m.r32(0x510) as i32;
                    let ch = m.r32(0x514) as i32;
                    let (px, py_);
                    if odd {
                        px = (x.wrapping_add(1) as i32).wrapping_mul(cw);
                        py_ = ((ch + 1).wrapping_mul(yh as i32)).wrapping_add(ch);
                    } else {
                        px = cw.wrapping_mul(x as i32).wrapping_add(m.r32(0x4D0) as i32);
                        py_ = ((ch + 1).wrapping_mul(yh as i32)).wrapping_sub(m.r32(0x4D4) as i32).wrapping_add(ch);
                    }
                    let rx = r % cw;
                    let ry = r % ch;
                    if !env.occupied(m.r32(0xC6EC), (rx + 1) as u32, (ry + 1) as u32) {
                        let y_out = ((ry.wrapping_sub(m.r32(0x4D4) as i32)).wrapping_add(py_)) << 16;
                        let x_out = ((rx.wrapping_sub(m.r32(0x4D0) as i32)).wrapping_add(px)) << 16;
                        let h = env.make(0x12);
                        let link = env.new_link();
                        env.register(h, link);
                        env.spawn(h, x_out as u32, y_out as u32, 0x20, link, 0x24);
                    }
                }
            }
            al = env.advance(&mut x, &mut y, 0xE0);
            inner -= 1;
        }
        x = sx;
        y = sy;
        al = env.advance(&mut x, &mut y, 0x20);
        outer -= 1;
    }
    if !env.audio_ready() || !env.audio_match() {
        return;
    }
    let base = m.r32(0xC994);
    let n = env.sound_count(base);
    if n == 0 {
        return;
    }
    let (l18, p2) = env.f594b50(o1, o2);
    if p2 as u8 == 0 {
        return;
    }
    let r = env.rand();
    env.play(r % n + base as i32, p2 & 0xFF, l18 & 0xFF);
}
