//! Memoria plana direccionable para portes que siguen punteros del original (listas enlazadas, tablas de tareas y cartas).

/// Lecturas y escrituras de 8/16/32 bits en direcciones del proceso original.
pub trait Mem {
    /// Devuelve al sistema las páginas enteras de `[a, a + len)` si la memoria es dispersa (leerlas después da 0). Por defecto
    /// no hace nada.
    fn release(&mut self, _a: u32, _len: u32) {}
    /// La memoria concreta, para quien necesite recuperarla tras pasar por la interfaz (el enlazador de `p3-mvp` evita así
    /// apilar adaptadores). Por defecto no se ofrece.
    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        None
    }
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
    /// Declara un byte nuevo (memoria recién asignada); las memorias densas no necesitan hacer nada.
    fn w8_seed(&mut self, a: u32) {
        self.w8(a, 0);
    }
    fn w16(&mut self, a: u32, v: u16) {
        self.w8(a, v as u8);
        self.w8(a.wrapping_add(1), (v >> 8) as u8);
    }
    fn w32(&mut self, a: u32, v: u32) {
        self.w16(a, v as u16);
        self.w16(a.wrapping_add(2), (v >> 16) as u16);
    }
}

const PAGE_BITS: u32 = 12;
const PAGE_SIZE: usize = 1 << PAGE_BITS;
const PAGE_WORDS: usize = PAGE_SIZE / 64;
const PAGE_ALLOC: usize = PAGE_SIZE + PAGE_WORDS * 8;
const PAGE_MASK: u32 = PAGE_SIZE as u32 - 1;
const NO_PAGE: u32 = u32::MAX;

/// Página de 4 KiB con un bit por byte sembrado. Los bytes sin sembrar no son observables (su contenido es indiferente).
#[derive(Clone)]
struct Page {
    // Datos y bitmap comparten una única reserva, como en la versión anterior.
    // Las palabras del bitmap se guardan en little endian, sin reinterpretar punteros.
    data: Box<[u8; PAGE_ALLOC]>,
}

impl Page {
    fn new() -> Self {
        // Vec inicializa los bytes a ceros directamente en el montón. La
        // conversión a array conserva esa reserva, sin copiar 4 KiB por la pila.
        Self {
            data: vec![0u8; PAGE_ALLOC].into_boxed_slice().try_into().unwrap(),
        }
    }
    #[inline]
    fn word(&self, w: usize) -> u64 {
        let at = PAGE_SIZE + w * 8;
        u64::from_le_bytes(self.data[at..at + 8].try_into().unwrap())
    }
    #[inline]
    fn set_word(&mut self, w: usize, bits: u64) {
        let at = PAGE_SIZE + w * 8;
        self.data[at..at + 8].copy_from_slice(&bits.to_le_bytes());
    }
    fn words(&self) -> impl Iterator<Item = u64> + '_ {
        (0..PAGE_WORDS).map(|w| self.word(w))
    }
    #[inline]
    fn has(&self, off: usize) -> bool {
        self.word(off >> 6) >> (off & 63) & 1 != 0
    }
    #[inline]
    fn mark(&mut self, off: usize) {
        let w = off >> 6;
        self.set_word(w, self.word(w) | (1 << (off & 63)));
    }
    /// Máscara de los bits `[lo, hi)` de la palabra `w` que caen en `[off, off + n)`.
    #[inline]
    fn word_mask(w: usize, off: usize, end: usize) -> u64 {
        let lo = off.max(w * 64) - w * 64;
        let hi = end.min(w * 64 + 64) - w * 64;
        let upper = if hi == 64 { u64::MAX } else { (1u64 << hi) - 1 };
        upper & !((1u64 << lo) - 1)
    }
    fn mark_range(&mut self, off: usize, n: usize) {
        if n == 0 {
            return;
        }
        let end = off + n;
        for w in off >> 6..=(end - 1) >> 6 {
            self.set_word(w, self.word(w) | Self::word_mask(w, off, end));
        }
    }
    fn has_range(&self, off: usize, n: usize) -> bool {
        if n == 0 {
            return true;
        }
        let end = off + n;
        (off >> 6..=(end - 1) >> 6).all(|w| {
            let m = Self::word_mask(w, off, end);
            self.word(w) & m == m
        })
    }
}

/// Memoria dispersa: solo existen los bytes sembrados; leer o escribir fuera de ellos es un error (el porte tocó algo no previsto).
///
/// Se guarda en páginas de 4 KiB con un mapa de bits de bytes sembrados (antes era un `BTreeMap<u32, u8>`, un nodo por byte).
/// La semántica observable es la misma: mismos contenidos, mismos pánicos al tocar un byte no sembrado y recorrido en orden de
/// dirección ([`SparseMem::iter`], [`SparseMem::to_map`]).
#[derive(Clone)]
pub struct SparseMem {
    /// Número de página (`dirección >> 12`) → posición en `pages`.
    index: std::collections::BTreeMap<u32, u32>,
    pages: Vec<Page>,
    /// Última página resuelta por un acceso con `&mut self` (los portes leen y escriben casi siempre en la misma).
    last_page: u32,
    last_slot: u32,
}

impl Default for SparseMem {
    fn default() -> Self {
        Self { index: Default::default(), pages: Vec::new(), last_page: NO_PAGE, last_slot: 0 }
    }
}

impl std::fmt::Debug for SparseMem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SparseMem").field("bytes", &self.to_map()).finish()
    }
}

/// Dos memorias son iguales si tienen sembrados los mismos bytes con los mismos valores.
impl PartialEq for SparseMem {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}
impl Eq for SparseMem {}

impl SparseMem {
    #[inline]
    fn page(&self, a: u32) -> Option<&Page> {
        let n = a >> PAGE_BITS;
        if n == self.last_page {
            return Some(&self.pages[self.last_slot as usize]);
        }
        self.index.get(&n).map(|&slot| &self.pages[slot as usize])
    }
    #[inline]
    fn page_mut(&mut self, a: u32) -> Option<&mut Page> {
        let n = a >> PAGE_BITS;
        if n != self.last_page {
            let slot = *self.index.get(&n)?;
            self.last_page = n;
            self.last_slot = slot;
        }
        Some(&mut self.pages[self.last_slot as usize])
    }
    fn page_or_new(&mut self, a: u32) -> &mut Page {
        let n = a >> PAGE_BITS;
        if n != self.last_page {
            let slot = match self.index.get(&n) {
                Some(&slot) => slot,
                None => {
                    let slot = self.pages.len() as u32;
                    self.pages.push(Page::new());
                    self.index.insert(n, slot);
                    slot
                }
            };
            self.last_page = n;
            self.last_slot = slot;
        }
        &mut self.pages[self.last_slot as usize]
    }

    pub fn seed(&mut self, base: u32, data: &[u8]) {
        // Por tramos de página; la dirección da la vuelta en 2^32 igual que al insertar byte a byte (los bytes nuevos ganan).
        let mut a = base;
        let mut rest = data;
        while !rest.is_empty() {
            let off = (a & PAGE_MASK) as usize;
            let n = rest.len().min(PAGE_SIZE - off);
            let page = self.page_or_new(a);
            page.data[off..off + n].copy_from_slice(&rest[..n]);
            page.mark_range(off, n);
            rest = &rest[n..];
            a = a.wrapping_add(n as u32);
        }
    }
    pub fn read(&self, base: u32, len: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(len);
        let mut a = base;
        while out.len() < len {
            let off = (a & PAGE_MASK) as usize;
            let n = (len - out.len()).min(PAGE_SIZE - off);
            match self.page(a) {
                Some(page) if page.has_range(off, n) => out.extend_from_slice(&page.data[off..off + n]),
                // Mismo pánico que el `Index` de `BTreeMap` de la versión por bytes.
                _ => panic!("no entry found for key"),
            }
            a = a.wrapping_add(n as u32);
        }
        out
    }
    /// Copia `out.len()` bytes desde `base` sin reservar; mismo pánico que [`SparseMem::read`] si falta alguno.
    pub fn read_into(&self, base: u32, out: &mut [u8]) {
        let mut a = base;
        let mut done = 0;
        while done < out.len() {
            let off = (a & PAGE_MASK) as usize;
            let n = (out.len() - done).min(PAGE_SIZE - off);
            match self.page(a) {
                Some(page) if page.has_range(off, n) => out[done..done + n].copy_from_slice(&page.data[off..off + n]),
                _ => panic!("no entry found for key"),
            }
            done += n;
            a = a.wrapping_add(n as u32);
        }
    }
    /// ¿Está sembrado el byte `a`? (antes `bytes.contains_key(&a)`).
    #[inline]
    pub fn contains(&self, a: u32) -> bool {
        self.page(a).is_some_and(|p| p.has((a & PAGE_MASK) as usize))
    }
    /// ¿Están sembrados los `len` bytes desde `base`? La dirección da la vuelta en 2^32.
    pub fn contains_range(&self, base: u32, len: usize) -> bool {
        let mut a = base;
        let mut done = 0;
        while done < len {
            let off = (a & PAGE_MASK) as usize;
            let n = (len - done).min(PAGE_SIZE - off);
            if !self.page(a).is_some_and(|p| p.has_range(off, n)) {
                return false;
            }
            done += n;
            a = a.wrapping_add(n as u32);
        }
        true
    }
    /// ¿Está sembrado alguno de los `len` bytes desde `base`? La dirección da la vuelta en 2^32.
    pub fn contains_any(&self, base: u32, len: usize) -> bool {
        let mut a = base;
        let mut done = 0;
        while done < len {
            let off = (a & PAGE_MASK) as usize;
            let n = (len - done).min(PAGE_SIZE - off);
            if let Some(p) = self.page(a) {
                let end = off + n;
                if (off >> 6..=(end - 1) >> 6).any(|w| p.word(w) & Page::word_mask(w, off, end) != 0) {
                    return true;
                }
            }
            done += n;
            a = a.wrapping_add(n as u32);
        }
        false
    }
    /// Byte sembrado en `a` (antes `bytes.get(&a)`).
    #[inline]
    pub fn get(&self, a: u32) -> Option<u8> {
        let off = (a & PAGE_MASK) as usize;
        self.page(a).filter(|p| p.has(off)).map(|p| p.data[off])
    }
    /// Referencia mutable al byte sembrado en `a` (antes `bytes.get_mut(&a)`).
    #[inline]
    pub fn get_mut(&mut self, a: u32) -> Option<&mut u8> {
        let off = (a & PAGE_MASK) as usize;
        self.page_mut(a).filter(|p| p.has(off)).map(|p| &mut p.data[off])
    }
    /// Retira el byte `a` de lo sembrado y devuelve su valor (antes `bytes.remove(&a)`).
    pub fn unseed(&mut self, a: u32) -> Option<u8> {
        let off = (a & PAGE_MASK) as usize;
        let page = self.page_mut(a).filter(|p| p.has(off))?;
        let w = off >> 6;
        page.set_word(w, page.word(w) & !(1 << (off & 63)));
        Some(page.data[off])
    }
    /// Número de bytes sembrados (antes `bytes.len()`).
    pub fn len(&self) -> usize {
        self.pages.iter().map(|p| p.words().map(|w| w.count_ones() as usize).sum::<usize>()).sum()
    }
    pub fn is_empty(&self) -> bool {
        self.pages.iter().all(|p| p.words().all(|w| w == 0))
    }
    /// Bytes sembrados en orden creciente de dirección, como el recorrido del `BTreeMap` anterior.
    pub fn iter(&self) -> impl Iterator<Item = (u32, u8)> + '_ {
        self.index.iter().flat_map(move |(&n, &slot)| {
            let page = &self.pages[slot as usize];
            page.words().enumerate().filter(|(_, w)| *w != 0).flat_map(move |(w, bits)| {
                (0..64usize).filter(move |b| bits >> b & 1 != 0).map(move |b| {
                    let off = w * 64 + b;
                    (n << PAGE_BITS | off as u32, page.data[off])
                })
            })
        })
    }
    /// Copia como mapa ordenado dirección → byte (la representación anterior del campo `bytes`).
    pub fn to_map(&self) -> std::collections::BTreeMap<u32, u8> {
        self.iter().collect()
    }
    /// Igual que [`SparseMem::to_map`] consumiendo la memoria.
    pub fn into_map(self) -> std::collections::BTreeMap<u32, u8> {
        self.to_map()
    }
}

impl Mem for SparseMem {
    #[inline]
    fn r8(&mut self, a: u32) -> u8 {
        let off = (a & PAGE_MASK) as usize;
        match self.page_mut(a) {
            Some(p) if p.has(off) => p.data[off],
            _ => panic!("lectura fuera de lo sembrado: {a:#x}"),
        }
    }
    #[inline]
    fn r16(&mut self, a: u32) -> u16 {
        let off = (a & PAGE_MASK) as usize;
        if off + 2 <= PAGE_SIZE {
            if let Some(p) = self.page_mut(a) {
                if p.word(off >> 6) >> (off & 63) & 3 == 3 || p.has_range(off, 2) {
                    return u16::from_le_bytes([p.data[off], p.data[off + 1]]);
                }
            }
        }
        // Camino general: mismo orden de lectura (y mismo pánico) que la versión byte a byte.
        u16::from(self.r8(a)) | u16::from(self.r8(a.wrapping_add(1))) << 8
    }
    #[inline]
    fn r32(&mut self, a: u32) -> u32 {
        let off = (a & PAGE_MASK) as usize;
        if off + 4 <= PAGE_SIZE {
            if let Some(p) = self.page_mut(a) {
                if p.word(off >> 6) >> (off & 63) & 15 == 15 || p.has_range(off, 4) {
                    return u32::from_le_bytes([p.data[off], p.data[off + 1], p.data[off + 2], p.data[off + 3]]);
                }
            }
        }
        u32::from(self.r16(a)) | u32::from(self.r16(a.wrapping_add(2))) << 16
    }
    fn w8_seed(&mut self, a: u32) {
        let off = (a & PAGE_MASK) as usize;
        let page = self.page_or_new(a);
        page.data[off] = 0;
        page.mark(off);
    }
    #[inline]
    fn w8(&mut self, a: u32, v: u8) {
        let off = (a & PAGE_MASK) as usize;
        match self.page_mut(a) {
            Some(p) if p.has(off) => p.data[off] = v,
            _ => panic!("escritura fuera de lo sembrado: {a:#x}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    /// Modelo de referencia: la versión anterior, un `BTreeMap<u32, u8>`.
    #[derive(Default, Clone)]
    struct Model(BTreeMap<u32, u8>);
    impl Model {
        fn seed(&mut self, base: u32, data: &[u8]) {
            for (i, b) in data.iter().enumerate() {
                self.0.insert(base.wrapping_add(i as u32), *b);
            }
        }
        fn read(&self, base: u32, len: usize) -> Vec<u8> {
            (0..len).map(|i| self.0[&base.wrapping_add(i as u32)]).collect()
        }
    }
    impl Mem for Model {
        fn r8(&mut self, a: u32) -> u8 {
            *self.0.get(&a).unwrap_or_else(|| panic!("lectura fuera de lo sembrado: {a:#x}"))
        }
        fn w8_seed(&mut self, a: u32) {
            self.0.insert(a, 0);
        }
        fn w8(&mut self, a: u32, v: u8) {
            *self.0.get_mut(&a).unwrap_or_else(|| panic!("escritura fuera de lo sembrado: {a:#x}")) = v;
        }
    }

    fn outcome<T: PartialEq + std::fmt::Debug>(f: impl FnOnce() -> T) -> Result<T, String> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).map_err(|e| {
            e.downcast_ref::<String>().cloned().or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string())).unwrap_or_default()
        })
    }

    #[test]
    fn pages_match_the_per_byte_map() {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let mut state = 0x1234_5678_9abc_def0u64;
        let mut rnd = move |n: u32| {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((state >> 33) as u32) % n
        };
        // Direcciones cerca de fronteras de página, de palabra del mapa de bits y de la vuelta en 2^32.
        let bases = [0u32, 0xfc0, 0xffe, 0x1000, 0x1fff, 0x3000, 0x40_0ff0, 0x6de_4a0, 0xffff_efff, 0xffff_ffc0, 0xffff_fffd];
        let mut mem = SparseMem::default();
        let mut model = Model::default();
        for step in 0..20_000 {
            let a = bases[rnd(bases.len() as u32) as usize].wrapping_add(rnd(0x90));
            match rnd(12) {
                0 => {
                    let len = [1, 2, 3, 7, 63, 64, 65, 200, 4096, 5000, 9000][rnd(11) as usize];
                    let data: Vec<u8> = (0..len).map(|_| rnd(256) as u8).collect();
                    mem.seed(a, &data);
                    model.seed(a, &data);
                }
                1 => {
                    mem.w8_seed(a);
                    model.w8_seed(a);
                }
                2 => assert_eq!(mem.unseed(a), model.0.remove(&a)),
                3 => {
                    let v = rnd(256) as u8;
                    assert_eq!(outcome(|| mem.w8(a, v)), outcome(|| model.w8(a, v)), "w8 {a:#x}");
                }
                4 => {
                    let v = rnd(u32::MAX);
                    assert_eq!(outcome(|| mem.w32(a, v)), outcome(|| model.w32(a, v)), "w32 {a:#x}");
                }
                5 => {
                    let v = rnd(0x10000) as u16;
                    assert_eq!(outcome(|| mem.w16(a, v)), outcome(|| model.w16(a, v)), "w16 {a:#x}");
                }
                6 => assert_eq!(outcome(|| mem.r8(a)), outcome(|| model.r8(a)), "r8 {a:#x}"),
                7 => assert_eq!(outcome(|| mem.r16(a)), outcome(|| model.r16(a)), "r16 {a:#x}"),
                8 => assert_eq!(outcome(|| mem.r32(a)), outcome(|| model.r32(a)), "r32 {a:#x}"),
                9 => {
                    let len = [0, 1, 4, 64, 100, 4096, 6000][rnd(7) as usize];
                    let want = outcome(|| model.read(a, len));
                    assert_eq!(outcome(|| mem.read(a, len)), want, "read {a:#x}+{len}");
                    let mut buf = vec![0u8; len];
                    assert_eq!(outcome(|| mem.read_into(a, &mut buf)).map(|_| buf), want);
                    let all = (0..len).all(|i| model.0.contains_key(&a.wrapping_add(i as u32)));
                    let any = (0..len).any(|i| model.0.contains_key(&a.wrapping_add(i as u32)));
                    assert_eq!(mem.contains_range(a, len), all);
                    assert_eq!(mem.contains_any(a, len), any);
                }
                10 => {
                    assert_eq!(mem.contains(a), model.0.contains_key(&a));
                    assert_eq!(mem.get(a), model.0.get(&a).copied());
                    assert_eq!(mem.get_mut(a).map(|b| *b), model.0.get(&a).copied());
                }
                _ => {
                    if let (Some(x), Some(y)) = (mem.get_mut(a), model.0.get_mut(&a)) {
                        *x = x.wrapping_add(1);
                        *y = y.wrapping_add(1);
                    }
                }
            }
            if step % 500 == 0 {
                assert_eq!(mem.len(), model.0.len());
                assert_eq!(mem.is_empty(), model.0.is_empty());
                assert!(mem.iter().eq(model.0.iter().map(|(a, b)| (*a, *b))), "recorrido ordenado, paso {step}");
                assert_eq!(format!("{mem:?}"), format!("SparseMem {{ bytes: {:?} }}", model.0));
                let copy = mem.clone();
                assert_eq!(copy, mem);
            }
        }
        assert_eq!(mem.to_map(), model.0);
        let mut other = mem.clone();
        let (a, _) = mem.iter().next().unwrap();
        other.unseed(a);
        assert_ne!(other, mem);
        other.w8_seed(a);
        other.w8(a, mem.get(a).unwrap());
        assert_eq!(other, mem);
        std::panic::set_hook(hook);
    }
}
