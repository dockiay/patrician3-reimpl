//! Modelo mínimo y exacto de la CPU x86 de 32 bits para las funciones portadas por `reimpl/tools/lift.py`: ocho registros, banderas
//! CF/ZF/SF/OF, aritmética por ancho (8/16/32) con la semántica de cada instrucción y memoria por [`Mem`]. El código generado es una máquina
//! de bloques básicos que replica instrucción a instrucción el original; las llamadas a otras funciones salen por [`OpsHost`] igual que en
//! el resto de portes con oráculo. La paridad se comprueba con trazas de memoria (`oracle/lifttrace.py`, `tests/x86_lifted.rs`).
use crate::flatmem::Mem;

pub const EAX: usize = 0;
pub const ECX: usize = 1;
pub const EDX: usize = 2;
pub const EBX: usize = 3;
pub const ESP: usize = 4;
pub const EBP: usize = 5;
pub const ESI: usize = 6;
pub const EDI: usize = 7;

#[derive(Clone, Debug, Default)]
pub struct Cpu {
    pub r: [u32; 8],
    pub cf: bool,
    pub zf: bool,
    pub sf: bool,
    pub of: bool,
    /// Bandera de paridad (PF): paridad par de los 8 bits bajos del último resultado lógico/aritmético.
    pub pf: bool,
    /// Pila x87 (el último es ST0). Precisión de 53 bits como el CRT (CW 0x027F): `f64` equivale.
    pub fpu: Vec<f64>,
    pub fscratch: f64,
    /// Bits C0, C2 y C3 de la palabra de estado (8, 10 y 14).
    pub fsw_cc: u16,
}

#[inline]
pub fn mask(w: u32) -> u32 {
    match w {
        8 => 0xff,
        16 => 0xffff,
        _ => 0xffff_ffff,
    }
}
#[inline]
fn sign(w: u32) -> u32 {
    1 << (w - 1)
}
#[inline]
pub fn sx(v: u32, w: u32) -> u32 {
    match w {
        8 => v as u8 as i8 as i32 as u32,
        16 => v as u16 as i16 as i32 as u32,
        _ => v,
    }
}

#[inline(always)]
pub fn rm(m: &mut dyn Mem, a: u32, w: u32) -> u32 {
    match w {
        8 => u32::from(m.r8(a)),
        16 => u32::from(m.r16(a)),
        _ => m.r32(a),
    }
}
#[inline(always)]
pub fn wm(m: &mut dyn Mem, a: u32, w: u32, v: u32) {
    match w {
        8 => m.w8(a, v as u8),
        16 => m.w16(a, v as u16),
        _ => m.w32(a, v),
    }
}

thread_local! {
    /// Profundidad y ST0 de la pila x87 que dejó la última función elevada con `fpu_exit` (lo lee la prueba de trazas).
    pub static FPU_EXIT: std::cell::Cell<(u32, u64)> = const { std::cell::Cell::new((0, 0)) };
    /// Valores (bits de f64) que dejan en ST0 los llamados enganchados que devuelven float; los carga la prueba de trazas.
    pub static FRET: std::cell::RefCell<std::collections::VecDeque<u64>> = const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

impl Cpu {
    pub fn new(esp: u32, ecx: u32) -> Self {
        let mut c = Cpu::default();
        c.r[ESP] = esp;
        c.r[ECX] = ecx;
        c
    }
    /// Lee un registro de ancho `w`; para 8 bits `i` 0..3 = AL,CL,DL,BL y 4..7 = AH,CH,DH,BH.
    #[inline]
    pub fn get(&self, i: usize, w: u32) -> u32 {
        match w {
            8 => {
                if i < 4 {
                    self.r[i] & 0xff
                } else {
                    (self.r[i - 4] >> 8) & 0xff
                }
            }
            16 => self.r[i] & 0xffff,
            _ => self.r[i],
        }
    }
    #[inline]
    pub fn set(&mut self, i: usize, w: u32, v: u32) {
        match w {
            8 => {
                if i < 4 {
                    self.r[i] = (self.r[i] & !0xff) | (v & 0xff);
                } else {
                    self.r[i - 4] = (self.r[i - 4] & !0xff00) | ((v & 0xff) << 8);
                }
            }
            16 => self.r[i] = (self.r[i] & !0xffff) | (v & 0xffff),
            _ => self.r[i] = v,
        }
    }
    fn zs(&mut self, r: u32, w: u32) {
        self.zf = r & mask(w) == 0;
        self.sf = r & sign(w) != 0;
        self.pf = (r & 0xff).count_ones() % 2 == 0;
    }
    pub fn add(&mut self, a: u32, b: u32, w: u32) -> u32 {
        let (a, b) = (a & mask(w), b & mask(w));
        let r = a.wrapping_add(b) & mask(w);
        self.cf = u64::from(a) + u64::from(b) > u64::from(mask(w));
        self.of = ((a ^ r) & (b ^ r)) & sign(w) != 0;
        self.zs(r, w);
        r
    }
    pub fn adc(&mut self, a: u32, b: u32, w: u32) -> u32 {
        let (a, b) = (a & mask(w), b & mask(w));
        let c = u32::from(self.cf);
        let r = a.wrapping_add(b).wrapping_add(c) & mask(w);
        self.cf = u64::from(a) + u64::from(b) + u64::from(c) > u64::from(mask(w));
        self.of = ((a ^ r) & (b ^ r)) & sign(w) != 0;
        self.zs(r, w);
        r
    }
    pub fn sub(&mut self, a: u32, b: u32, w: u32) -> u32 {
        let (a, b) = (a & mask(w), b & mask(w));
        let r = a.wrapping_sub(b) & mask(w);
        self.cf = a < b;
        self.of = ((a ^ b) & (a ^ r)) & sign(w) != 0;
        self.zs(r, w);
        r
    }
    pub fn sbb(&mut self, a: u32, b: u32, w: u32) -> u32 {
        let (a, b) = (a & mask(w), b & mask(w));
        let c = u32::from(self.cf);
        let r = a.wrapping_sub(b).wrapping_sub(c) & mask(w);
        self.cf = u64::from(a) < u64::from(b) + u64::from(c);
        self.of = ((a ^ b) & (a ^ r)) & sign(w) != 0;
        self.zs(r, w);
        r
    }
    pub fn and(&mut self, a: u32, b: u32, w: u32) -> u32 {
        let r = a & b & mask(w);
        self.cf = false;
        self.of = false;
        self.zs(r, w);
        r
    }
    pub fn or(&mut self, a: u32, b: u32, w: u32) -> u32 {
        let r = (a | b) & mask(w);
        self.cf = false;
        self.of = false;
        self.zs(r, w);
        r
    }
    pub fn xor(&mut self, a: u32, b: u32, w: u32) -> u32 {
        let r = (a ^ b) & mask(w);
        self.cf = false;
        self.of = false;
        self.zs(r, w);
        r
    }
    pub fn inc(&mut self, a: u32, w: u32) -> u32 {
        let cf = self.cf;
        let r = self.add(a, 1, w);
        self.cf = cf;
        r
    }
    pub fn dec(&mut self, a: u32, w: u32) -> u32 {
        let cf = self.cf;
        let r = self.sub(a, 1, w);
        self.cf = cf;
        r
    }
    pub fn neg(&mut self, a: u32, w: u32) -> u32 {
        let a = a & mask(w);
        let r = a.wrapping_neg() & mask(w);
        self.cf = a != 0;
        self.of = a == sign(w);
        self.zs(r, w);
        r
    }
    pub fn shl(&mut self, a: u32, n: u32, w: u32) -> u32 {
        let n = n & 31;
        let a = a & mask(w);
        if n == 0 {
            return a;
        }
        let wide = u64::from(a) << n;
        let r = (wide as u32) & mask(w);
        self.cf = (wide >> w) & 1 != 0;
        self.of = (r & sign(w) != 0) != self.cf;
        self.zs(r, w);
        r
    }
    pub fn shr(&mut self, a: u32, n: u32, w: u32) -> u32 {
        let n = n & 31;
        let a = a & mask(w);
        if n == 0 {
            return a;
        }
        let r = if n >= 32 { 0 } else { a >> n };
        self.cf = if n > w { false } else { (a >> (n - 1)) & 1 != 0 };
        self.of = a & sign(w) != 0;
        self.zs(r, w);
        r
    }
    pub fn sar(&mut self, a: u32, n: u32, w: u32) -> u32 {
        let n = n & 31;
        let a = a & mask(w);
        if n == 0 {
            return a;
        }
        let s = sx(a, w) as i32;
        let r = (s >> n.min(31)) as u32 & mask(w);
        self.cf = ((s >> (n - 1).min(31)) & 1) != 0;
        self.of = false;
        self.zs(r, w);
        r
    }
    /// `imul` de dos o tres operandos: resultado truncado a `w`; CF = OF = hubo desbordamiento con signo.
    pub fn imul(&mut self, a: u32, b: u32, w: u32) -> u32 {
        let p = i64::from(sx(a, w) as i32) * i64::from(sx(b, w) as i32);
        let r = (p as u32) & mask(w);
        let over = i64::from(sx(r, w) as i32) != p;
        self.cf = over;
        self.of = over;
        self.zs(r, w);
        r
    }
    /// `imul r/m` de un operando: devuelve (bajo, alto) del producto con signo de `eax`/`ax`/`al` por `b`.
    pub fn imul1(&mut self, a: u32, b: u32, w: u32) -> (u32, u32) {
        let p = i64::from(sx(a, w) as i32) * i64::from(sx(b, w) as i32);
        let lo = (p as u32) & mask(w);
        let hi = ((p >> w) as u32) & mask(w);
        let over = i64::from(sx(lo, w) as i32) != p;
        self.cf = over;
        self.of = over;
        (lo, hi)
    }
    pub fn mul1(&mut self, a: u32, b: u32, w: u32) -> (u32, u32) {
        let p = u64::from(a & mask(w)) * u64::from(b & mask(w));
        let lo = (p as u32) & mask(w);
        let hi = ((p >> w) as u32) & mask(w);
        self.cf = hi != 0;
        self.of = hi != 0;
        (lo, hi)
    }
    /// `div r/m32`: `edx:eax / b`. Divide por cero o cociente que no cabe: el original lanza una excepción; aquí se detiene la prueba.
    pub fn div32(&mut self, b: u32) {
        let n = (u64::from(self.r[EDX]) << 32) | u64::from(self.r[EAX]);
        assert!(b != 0 && n / u64::from(b) <= u64::from(u32::MAX), "excepción de división del original");
        self.r[EAX] = (n / u64::from(b)) as u32;
        self.r[EDX] = (n % u64::from(b)) as u32;
    }
    pub fn idiv32(&mut self, b: u32) {
        let n = ((u64::from(self.r[EDX]) << 32) | u64::from(self.r[EAX])) as i64;
        let b = i64::from(b as i32);
        assert!(b != 0 && (n / b) >= i64::from(i32::MIN) && (n / b) <= i64::from(i32::MAX), "excepción de división del original");
        self.r[EAX] = (n / b) as u32;
        self.r[EDX] = (n % b) as u32;
    }
    /// Condiciones de salto (`cc` = código x86 de 4 bits).
    pub fn cond(&self, cc: u8) -> bool {
        let r = match cc >> 1 {
            0 => self.of,
            1 => self.cf,
            2 => self.zf,
            3 => self.cf || self.zf,
            4 => self.sf,
            5 => self.pf,
            6 => self.sf != self.of,
            _ => self.zf || (self.sf != self.of),
        };
        r != (cc & 1 != 0)
    }
    #[inline]
    pub fn push(&mut self, m: &mut dyn Mem, v: u32) {
        self.r[ESP] = self.r[ESP].wrapping_sub(4);
        m.w32(self.r[ESP], v);
    }
    #[inline]
    pub fn pop(&mut self, m: &mut dyn Mem) -> u32 {
        let v = m.r32(self.r[ESP]);
        self.r[ESP] = self.r[ESP].wrapping_add(4);
        v
    }

    // ---- x87 (subconjunto de los cuerpos portados: fld/fild/fstp/fadd/fmul/fdiv/fcomp/fnstsw)
    pub fn fld(&mut self, v: f64) {
        self.fpush(v);
    }
    /// Apila como el x87 que emula Unicorn/QEMU: con 8 valores el nuevo sobrescribe el más antiguo (anillo de ocho registros, sin excepción de pila).
    fn fpush(&mut self, v: f64) {
        if self.fpu.len() >= 8 {
            self.fpu.remove(0);
        }
        self.fpu.push(v);
    }
    /// Desapila; con la pila vacía el anillo devuelve el contenido del registro (0 tras el reinicio del oráculo).
    fn fpop(&mut self) -> f64 {
        self.fpu.pop().unwrap_or(0.0)
    }
    pub fn ld(&self, m: &mut dyn Mem, a: u32, size: u32) -> f64 {
        if size == 4 {
            f32::from_bits(m.r32(a)) as f64
        } else {
            f64::from_bits((m.r32(a) as u64) | ((m.r32(a.wrapping_add(4)) as u64) << 32))
        }
    }
    pub fn fld32(&mut self, m: &mut dyn Mem, a: u32) {
        self.fpush(f32::from_bits(m.r32(a)) as f64);
    }
    pub fn fld64(&mut self, m: &mut dyn Mem, a: u32) {
        let v = (m.r32(a) as u64) | ((m.r32(a.wrapping_add(4)) as u64) << 32);
        self.fpush(f64::from_bits(v));
    }
    pub fn fild32(&mut self, m: &mut dyn Mem, a: u32) {
        self.fpush(m.r32(a) as i32 as f64);
    }
    pub fn fild64(&mut self, m: &mut dyn Mem, a: u32) {
        let v = (m.r32(a) as u64) | ((m.r32(a.wrapping_add(4)) as u64) << 32);
        self.fpush(v as i64 as f64);
    }
    pub fn fstp32(&mut self, m: &mut dyn Mem, a: u32) {
        let v = self.fpop();
        m.w32(a, (v as f32).to_bits());
    }
    pub fn fstp64(&mut self, m: &mut dyn Mem, a: u32) {
        let v = self.fpop().to_bits();
        m.w32(a, v as u32);
        m.w32(a.wrapping_add(4), (v >> 32) as u32);
    }
    /// `fadd/fmul/fdiv/fsub/fsubr/fdivr` con operando de memoria: ST0 = ST0 op v.
    pub fn fop(&mut self, op: char, v: f64) {
        let t = self.st(0);
        *t = match op {
            '+' => *t + v,
            '*' => *t * v,
            '/' => *t / v,
            '-' => *t - v,
            'r' => v - *t,
            _ => v / *t,
        };
    }
    /// `faddp st(1)`/`fmulp`...: ST1 = ST1 op ST0 y desapila.
    pub fn fopp(&mut self, op: char) {
        let a = self.fpop();
        self.fop(op, a);
        // fop calculó ST0(nuevo) op a; para '-' y '/' el operando izquierdo es el destino: coincide.
    }
    pub fn fcomp(&mut self, v: f64) {
        let t = self.fpop();
        self.fsw_cc = if t.is_nan() || v.is_nan() {
            0x4500
        } else if t > v {
            0
        } else if t < v {
            0x0100
        } else {
            0x4000
        };
    }
    pub fn fnstsw(&self) -> u32 {
        let top = ((8 - self.fpu.len() as u16) & 7) << 11;
        (top | self.fsw_cc) as u32
    }
    // ---- x87: formas con registros y operaciones sueltas (ELV-X87)
    #[inline]
    fn st(&mut self, i: usize) -> &mut f64 {
        let n = self.fpu.len();
        if i >= n {
            // registro vacío del anillo: se lee 0 y la escritura se pierde
            self.fscratch = 0.0;
            return &mut self.fscratch;
        }
        &mut self.fpu[n - 1 - i]
    }
    /// `fchs`: cambia el signo de ST0.
    pub fn fchs(&mut self) {
        let t = self.st(0);
        *t = -*t;
    }
    pub fn fabs(&mut self) {
        let t = self.st(0);
        *t = t.abs();
    }
    pub fn fsqrt(&mut self) {
        let t = self.st(0);
        *t = t.sqrt();
    }
    /// `fld st(i)`.
    pub fn fld_st(&mut self, i: usize) {
        let v = *self.st(i);
        self.fpush(v);
    }
    /// `fxch st(i)`.
    pub fn fxch(&mut self, i: usize) {
        let n = self.fpu.len();
        self.fpu.swap(n - 1, n - 1 - i);
    }
    /// `fstp st(i)` (con `i` = 0 solo desapila) y `fst st(i)`.
    pub fn fstp_st(&mut self, i: usize) {
        let v = self.fpop();
        if i > 0 {
            *self.st(i - 1) = v;
        }
    }
    pub fn fst_st(&mut self, i: usize) {
        let v = *self.st(0);
        *self.st(i) = v;
    }
    pub fn fst32(&mut self, m: &mut dyn Mem, a: u32) {
        let v = *self.st(0);
        m.w32(a, (v as f32).to_bits());
    }
    pub fn fst64(&mut self, m: &mut dyn Mem, a: u32) {
        let v = (*self.st(0)).to_bits();
        m.w32(a, v as u32);
        m.w32(a.wrapping_add(4), (v >> 32) as u32);
    }
    /// Operación aritmética entre registros: `dst = (i_dst) op (i_src)`; `pop` desapila después (`faddp`...).
    pub fn fop_st(&mut self, op: char, dst: usize, src: usize, pop: bool) {
        let a = *self.st(dst);
        let b = *self.st(src);
        let r = match op {
            '+' => a + b,
            '*' => a * b,
            '/' => a / b,
            '-' => a - b,
            'r' => b - a,
            _ => b / a,
        };
        *self.st(dst) = r;
        if pop {
            self.fpu.pop();
        }
    }
    /// `fcom st(i)` / `fcomp`: compara ST0 con `v` y fija C0/C2/C3; `pop` desapila.
    pub fn fcom_v(&mut self, v: f64, pop: bool) {
        let t = *self.st(0);
        self.fsw_cc = if t.is_nan() || v.is_nan() {
            0x4500
        } else if t > v {
            0
        } else if t < v {
            0x0100
        } else {
            0x4000
        };
        if pop {
            self.fpu.pop();
        }
    }
    pub fn fcom_st(&mut self, i: usize, pop: bool) {
        let v = *self.st(i);
        self.fcom_v(v, pop);
    }
    /// `fistp` a memoria de 32 o 64 bits (redondeo al más cercano par, como la palabra de control 0x27F).
    pub fn fistp(&mut self, m: &mut dyn Mem, a: u32, size: u32) {
        let v = self.fpop();
        let r = v.round_ties_even();
        if size == 4 {
            let x = if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 { 0x8000_0000u32 } else { r as i32 as u32 };
            m.w32(a, x);
        } else {
            let x = if r.is_nan() || r >= 9.223372036854775808e18 || r < -9.223372036854775808e18 { 0x8000_0000_0000_0000u64 } else { r as i64 as u64 };
            m.w32(a, x as u32);
            m.w32(a.wrapping_add(4), (x >> 32) as u32);
        }
    }
    /// Estado de la pila x87 a la salida de la función (profundidad y ST0 como bits `f64`), para compararlo con el original.
    /// Rutinas matemáticas del CRT del juego (`_ftol`, `sqrt`, `cos`, `sin`, `acos`, `asin`; argumento `double` en pila, resultado en ST0) modeladas
    /// con el efecto que dejan en registros y FPU. Se llama justo después de apilar la dirección de retorno.
    /// Devuelve false si la dirección no es una de ellas (para usar `host.call`).
    pub fn crt_call(&mut self, m: &mut dyn Mem, addr: u32) -> bool {
        if addr == 0x639c28 {
            // _ftol: trunca ST0 hacia cero; fuera de rango da el "entero indefinido".
            let v = self.fpop();
            let i: i64 = if v.is_nan() || v >= 9.223372036854775807e18 || v < -9.223372036854775808e18 { i64::MIN } else { v.trunc() as i64 };
            self.r[0] = i as u32;
            self.r[2] = (i >> 32) as u32;
            return true;
        }
        if addr == 0x63a5ce {
            // ceil (cdecl, argumento double en la pila): deja el redondeo hacia arriba en ST0. EAX/ECX/EDX quedan como estaban (los
            // llamantes elevados no los leen tras la llamada; el original deja en ellos restos de `_ctrlfp`).
            let x = self.ld(m, self.r[4].wrapping_add(4), 8);
            self.fpush(x.ceil());
            return true;
        }
        if addr == 0x63b1f5 {
            // fabs (marco propio, restaura la palabra de control): deja |x| en ST0, EAX = 0x133f, ECX = 0x27f.
            let x = self.ld(m, self.r[4].wrapping_add(4), 8);
            self.fpush(x.abs());
            self.r[0] = 0x133f;
            self.r[1] = 0x27f;
            return true;
        }
        let (ecx, f): (u32, fn(f64) -> f64) = match addr {
            0x63c264 => (0x6c83d0, f64::cos),
            0x63c314 => (0x6c83e0, f64::sin),
            0x63c3c4 => (0x6c83f0, f64::asin), // fpatan(x, sqrt(1-x²))
            0x63b2c4 => (0x6c83c0, f64::acos), // tras fxch: atan2(sqrt(1-x²), x)
            0x63aae4 => (0x6c8390, f64::sqrt),
            _ => return false,
        };
        let sp = self.r[4];
        let x = self.ld(m, sp.wrapping_add(4), 8);
        let hi = m.r32(sp.wrapping_add(8));
        self.fpush(f(x));
        self.r[0] = (hi & 0x7ff0_0000) | 0x27f;
        self.r[1] = ecx;
        self.r[2] = (sp.wrapping_add(4) & 0xffff_0000) | 0x27f;
        true
    }
    /// El llamado (enganchado en el oráculo) devolvió un float en ST0: se apila el siguiente valor grabado en la traza.
    pub fn fret(&mut self) {
        let v = FRET.with(|q| q.borrow_mut().pop_front()).unwrap_or(0);
        self.fpush(f64::from_bits(v));
    }
    pub fn fpu_exit(&self) {
        FPU_EXIT.with(|c| c.set((self.fpu.len() as u32 & 7, self.fpu.last().map_or(0, |v| v.to_bits()))));
    }
    /// `rep movs/stos` con DF = 0, tamaño 1/2/4. `rep` false = una sola vez.
    pub fn stos(&mut self, m: &mut dyn Mem, w: u32, rep: bool) {
        loop {
            if rep && self.r[ECX] == 0 {
                break;
            }
            wm(m, self.r[EDI], w, self.get(0, w));
            self.r[EDI] = self.r[EDI].wrapping_add(w / 8);
            if !rep {
                break;
            }
            self.r[ECX] -= 1;
        }
    }
    pub fn movs(&mut self, m: &mut dyn Mem, w: u32, rep: bool) {
        loop {
            if rep && self.r[ECX] == 0 {
                break;
            }
            let v = rm(m, self.r[ESI], w);
            wm(m, self.r[EDI], w, v);
            self.r[ESI] = self.r[ESI].wrapping_add(w / 8);
            self.r[EDI] = self.r[EDI].wrapping_add(w / 8);
            if !rep {
                break;
            }
            self.r[ECX] -= 1;
        }
    }
    /// `rep movsd` con DF = 0.
    pub fn rep_movsd(&mut self, m: &mut dyn Mem) {
        while self.r[ECX] != 0 {
            let v = m.r32(self.r[ESI]);
            m.w32(self.r[EDI], v);
            self.r[ESI] = self.r[ESI].wrapping_add(4);
            self.r[EDI] = self.r[EDI].wrapping_add(4);
            self.r[ECX] -= 1;
        }
    }
}
