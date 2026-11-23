//! Recortes 2D del controlador SGL (`ddraw_Dll.dll`): las ranuras `sgl_SetClipRect*`, `sgl_Get/Set2dClippingMode`,
//! `sgl_IsRectInClippingRegion`, `sgl_CreateClipper`/`GetRectClipper`, `sgl_FreeRectClipper`, `sgl_AddClipRect_4i`,
//! `sgl_SubClipRect_4i/_r`, `sgl_ResetClipper` y `sgl_CopyClipper` del EXE.
//!
//! El controlador guarda:
//! - el modo 2D (`[0x0506E898]`): 0 sin recorte, 1 contra el rectángulo de recorte, bit 2 contra la región del
//!   recortador activo (3 = región ∩ rectángulo). `sgl_Set2dClippingMode` también pone el byte `0x0508D084`
//!   (`BlitState::clip` de [`crate::sgl`]) a «modo ≠ 0»;
//! - el rectángulo de recorte (`0x0506E758`);
//! - el recortador activo (`[0x0506E92C]`). Un recortador (objeto de 0x70 bytes, tabla `0x05050F64`) es una región de
//!   rectángulos disjuntos guardada en un árbol ordenado por x (nodos de 0x24 bytes: hermano, hijo, lista de salida
//!   doble, enlace de pendientes, rectángulo). Los nodos con arriba = abajo = 0 son grupos por x que crea el reparto de
//!   `0x050183A0` cuando un padre pasa de 16 hijos. El recortador por defecto (`[0x05068500]`, tabla `0x05050664`) es
//!   nulo: no guarda nada y su región está vacía.
//!
//! Los rasterizadores dibujan una vez por rectángulo: `0x05001A90` (primero) y `0x05001DD0` (siguiente), o
//! `0x050018F0`/`0x050019F0`, dejan el rectángulo vigente en `0x0506E314` ([`SglClip::cur`]); la comprobación previa
//! `0x05001840` (que deshace `0x050018E0`) prepara la región para el primero.
//!
//! El porte reproduce el árbol nodo a nodo, con el mismo orden de la lista de salida y los mismos contadores
//! (`sgl_GetActiveClipperInfo`). Probado contra la DLL en emulación: `oracle/x86_sgl_clip.py`, `tests/sgl_clip.rs`.

/// Rectángulo `[izquierda, arriba, derecha, abajo]` (derecha y abajo excluidos).
pub type Rect = [i32; 4];

/// Prueba de solape del controlador: los cuatro restos negativos (aritmética de 32 bits con desbordamiento).
pub fn overlaps(a: Rect, b: Rect) -> bool {
    (b[1].wrapping_sub(a[3]) & a[1].wrapping_sub(b[3]) & b[0].wrapping_sub(a[2]) & a[0].wrapping_sub(b[2])) < 0
}

#[derive(Clone, Copy, Default, Debug)]
struct Node {
    next: u32,  // +0x00 hermano
    child: u32, // +0x04 primer hijo
    prev: u32,  // +0x08 lista de salida
    nout: u32,  // +0x0C lista de salida
    link: u32,  // +0x10 pendientes
    r: Rect,    // +0x14
}

const NIL: u32 = 0;
const ROOT: u32 = 1; // nodo incrustado en +0x38

#[derive(Clone, Copy)]
enum Slot {
    Child(u32),
    Next(u32),
}

/// Contadores `+4..+0x14` que devuelve `sgl_GetActiveClipperInfo`: sumas, restas, nodos vivos, rectángulos en la lista
/// de salida y un quinto campo que el controlador no escribe nunca.
pub type Info = [i32; 5];

/// Recortador (`0x05017E90`). `null` es el recortador por defecto, cuyos métodos no hacen nada.
#[derive(Clone, Debug)]
pub struct Clipper {
    null: bool,
    nodes: Vec<Node>,
    free: Vec<u32>,
    info: Info,
    split_ok: bool, // +0x18
    cache: Rect,    // +0x1C, último rectángulo de consulta
    pend_a: u32,    // +0x2C
    pend_b: u32,    // +0x30
    head: u32,      // +0x34
    bound: Rect,    // +0x5C
    bounded: bool,  // +0x6C
    state: u8,      // +0x6D: 0 lista libre, 2 lista de la consulta `cache`
}

impl Clipper {
    pub fn new() -> Self {
        Clipper {
            null: false,
            nodes: vec![Node::default(); 2],
            free: Vec::new(),
            info: [0; 5],
            split_ok: false,
            cache: [0; 4],
            pend_a: NIL,
            pend_b: NIL,
            head: NIL,
            bound: [0; 4],
            bounded: false,
            state: 0,
        }
    }

    /// Recortador nulo (tabla `0x05050664`).
    pub fn null() -> Self {
        Clipper { null: true, ..Self::new() }
    }

    pub fn is_null(&self) -> bool {
        self.null
    }

    fn alloc(&mut self) -> u32 {
        if let Some(i) = self.free.pop() {
            self.nodes[i as usize] = Node::default();
            i
        } else {
            self.nodes.push(Node::default());
            (self.nodes.len() - 1) as u32
        }
    }

    fn release(&mut self, n: u32) {
        self.free.push(n);
    }

    fn n(&self, i: u32) -> &Node {
        &self.nodes[i as usize]
    }

    fn m(&mut self, i: u32) -> &mut Node {
        &mut self.nodes[i as usize]
    }

    fn get(&self, s: Slot) -> u32 {
        match s {
            Slot::Child(p) => self.n(p).child,
            Slot::Next(p) => self.n(p).next,
        }
    }

    fn set(&mut self, s: Slot, v: u32) {
        match s {
            Slot::Child(p) => self.m(p).child = v,
            Slot::Next(p) => self.m(p).next = v,
        }
    }

    fn is_group(&self, n: u32) -> bool {
        let r = self.n(n).r;
        r[3] == 0 && r[1] == 0 && n != ROOT
    }

    /// Añade un nodo hoja al principio de la lista de salida (`0x05018080` y sus copias en línea).
    fn push_out(&mut self, n: u32) {
        if self.is_group(n) {
            return;
        }
        let head = self.head;
        self.m(n).prev = NIL;
        self.m(n).nout = head;
        if head != NIL {
            self.m(head).prev = n;
        }
        self.head = n;
        self.info[3] += 1;
    }

    /// `0x050180C0`.
    fn unlink_out(&mut self, n: u32) {
        let (nx, pv) = (self.n(n).nout, self.n(n).prev);
        let mut changed = false;
        if nx != NIL {
            self.m(nx).prev = pv;
            changed = true;
        }
        if pv != NIL {
            self.m(pv).nout = nx;
            changed = true;
        }
        if self.head == n {
            self.head = nx;
        } else if !changed {
            return;
        }
        self.info[3] -= 1;
        self.m(n).nout = NIL;
        self.m(n).prev = NIL;
    }

    /// `0x05018E20`: todas las hojas desde `n` y sus descendientes.
    fn collect_all(&mut self, mut n: u32) {
        while n != NIL {
            self.push_out(n);
            let c = self.n(n).child;
            self.collect_all(c);
            n = self.n(n).next;
        }
    }

    /// `0x05018E80`: lista de salida con toda la región.
    fn build_all(&mut self) {
        if self.state == 1 {
            return;
        }
        self.info[3] = 0;
        self.head = NIL;
        self.state = 0;
        let c = self.n(ROOT).child;
        self.collect_all(c);
    }

    /// `0x05018C70`: hojas que solapan `r` desde `n`.
    fn query_collect(&mut self, mut n: u32, r: Rect) {
        while n != NIL {
            let nr = self.n(n).r;
            if nr[0] > r[2] {
                return;
            }
            if nr[2] > r[0] {
                if overlaps(nr, r) {
                    self.push_out(n);
                }
                let c = self.n(n).child;
                if c != NIL {
                    self.query_collect(c, r);
                }
            }
            n = self.n(n).next;
        }
    }

    /// `0x05018D00`: lista de salida con las hojas que solapan `r` (se guarda en `cache`).
    fn query(&mut self, r: Rect) {
        if self.state == 2 && self.cache == r {
            return;
        }
        self.info[3] = 0;
        self.head = NIL;
        self.state = 0;
        if r[2].wrapping_sub(r[0]) <= 0 || r[3].wrapping_sub(r[1]) <= 0 {
            return;
        }
        self.cache = r;
        let c = self.n(ROOT).child;
        self.query_collect(c, r);
        self.state = 2;
    }

    /// `0x05018B30` / bucle de `0x05018BC0`.
    fn any(&self, mut n: u32, r: Rect) -> bool {
        while n != NIL {
            let nr = self.n(n).r;
            if nr[0] > r[2] {
                return false;
            }
            if nr[2] > r[0] {
                if overlaps(nr, r) && !self.is_group(n) {
                    return true;
                }
                let c = self.n(n).child;
                if c != NIL && self.any(c, r) {
                    return true;
                }
            }
            n = self.n(n).next;
        }
        false
    }

    /// `0x05018480`: inserta `node` bajo `parent` (orden por x; baja al hijo que contiene su tramo x y adopta los
    /// hermanos contenidos en el suyo). Con más de 16 hijos reparte el padre.
    fn insert(&mut self, parent0: u32, node: u32) {
        let mut parent = parent0;
        let mut slot = Slot::Child(parent);
        let mut cur = self.n(parent).child;
        if cur == NIL {
            self.set(slot, node);
            return;
        }
        let mut count: i32;
        'a: loop {
            count = 0;
            loop {
                let c = *self.n(cur);
                let nl = self.n(node).r[0];
                if c.r[0] >= nl {
                    break 'a;
                }
                if c.r[2] > nl {
                    let nr = self.n(node).r[2];
                    let v = c.r[2].wrapping_sub(nl) | c.r[2].wrapping_sub(nr) | nl.wrapping_sub(c.r[0]) | nr.wrapping_sub(c.r[0]);
                    if v >= 0 {
                        parent = cur;
                        slot = Slot::Child(cur);
                        if c.child == NIL {
                            self.set(slot, node);
                            return;
                        }
                        cur = c.child;
                        continue 'a;
                    }
                }
                count += 1;
                slot = Slot::Next(cur);
                cur = c.next;
                if cur == NIL {
                    break 'a;
                }
            }
        }
        let first = self.get(slot);
        self.m(node).next = first;
        self.set(slot, node);
        let mut prev = node;
        count += 2;
        while cur != NIL {
            let nr = self.n(node).r[2];
            let nl = self.n(node).r[0];
            let c = *self.n(cur);
            if c.r[0] > nr {
                break;
            }
            let v = nr.wrapping_sub(c.r[2]) | nr.wrapping_sub(c.r[0]) | c.r[2].wrapping_sub(nl) | c.r[0].wrapping_sub(nl);
            if v < 0 {
                count += 1;
                prev = cur;
            } else {
                self.m(prev).next = c.next;
                self.m(cur).next = NIL;
                self.insert(node, cur);
                count -= 1;
            }
            cur = self.n(prev).next;
        }
        if count > 16 {
            self.split(parent);
        }
    }

    /// `0x050183A0`: dos grupos que parten por la mitad el tramo x de los hijos de `parent`.
    fn split(&mut self, parent: u32) {
        if !self.split_ok || self.is_group(parent) {
            return;
        }
        let mut c = self.n(parent).child;
        let l = self.n(c).r[0];
        let mut r = 0;
        while c != NIL {
            r = self.n(c).r[2];
            c = self.n(c).next;
        }
        let half = r.wrapping_sub(l) / 2;
        let mid = half.wrapping_add(l);
        let g1 = self.alloc();
        self.m(g1).r[0] = l;
        self.m(g1).r[2] = mid;
        let g2 = self.alloc();
        self.info[2] += 2;
        self.m(g2).r[2] = r;
        self.m(g2).r[0] = mid;
        let saved = self.split_ok;
        self.split_ok = false;
        self.insert(parent, g1);
        self.insert(parent, g2);
        self.split_ok = saved;
    }

    /// `0x050185A0`: quita `target` del árbol bajo `parent`; sus hijos vuelven a insertarse en `parent`.
    fn remove(&mut self, parent: u32, target: u32) -> bool {
        let mut slot = Slot::Child(parent);
        let mut s = self.n(parent).child;
        while s != NIL {
            let t = self.n(target).r;
            let n = *self.n(s);
            if n.r[0] > t[2] {
                return false;
            }
            if n.r[2] > t[0] {
                if s == target {
                    self.set(slot, n.next);
                    let mut c = n.child;
                    while c != NIL {
                        let nx = self.n(c).next;
                        self.m(c).next = NIL;
                        self.insert(parent, c);
                        c = nx;
                    }
                    self.m(s).next = NIL;
                    self.m(s).child = NIL;
                    return true;
                }
                if n.child != NIL && self.remove(s, target) {
                    return true;
                }
            }
            slot = Slot::Next(s);
            s = self.n(s).next;
        }
        false
    }

    /// `0x05018120`: une `n` con una hoja de la lista de salida que comparte un lado entero; la hoja ampliada sale del
    /// árbol y pasa a los pendientes B (se volverá a intentar unir).
    fn merge(&mut self, n: u32) -> bool {
        let nr = self.n(n).r;
        let mut e = self.head;
        while e != NIL {
            let er = self.n(e).r;
            let side = if er[0] == nr[0] && er[2] == nr[2] {
                if er[1] == nr[3] {
                    Some(1)
                } else if er[3] == nr[1] {
                    Some(3)
                } else {
                    None
                }
            } else if er[3] == nr[3] && er[1] == nr[1] {
                if er[0] == nr[2] {
                    Some(0)
                } else if er[2] == nr[0] {
                    Some(2)
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(k) = side {
                self.unlink_out(e);
                self.remove(ROOT, e);
                self.m(e).r[k] = nr[k];
                let pb = self.pend_b;
                self.m(e).link = pb;
                self.pend_b = e;
                return true;
            }
            e = self.n(e).nout;
        }
        false
    }

    /// `0x05018790`: trozos de `(l, t, r, b)` que no cubre ninguna hoja de la lista desde `list` → pendientes A.
    fn cut(&mut self, l: i32, t: i32, mut r: i32, mut b: i32, list: u32) {
        let mut n = list;
        while n != NIL {
            let nr = self.n(n).r;
            if nr[2] < r && nr[2] > l && nr[1] < b && nr[3] > t {
                self.cut(nr[2], t, r, b, n);
                r = nr[2];
                continue;
            }
            if nr[0] > l && nr[0] < r && nr[1] < b && nr[3] > t {
                self.cut(nr[0], t, r, b, n);
                r = nr[0];
                continue;
            }
            if nr[1] > t && nr[1] < b && nr[2] > l && nr[0] < r {
                self.cut(l, nr[1], r, b, n);
                b = nr[1];
                continue;
            }
            if nr[3] > t && nr[3] < b && nr[2] > l && nr[0] < r {
                self.cut(l, nr[3], r, b, n);
                b = nr[3];
                continue;
            }
            if l >= nr[0] && l < nr[2] && t >= nr[1] && t < nr[3] {
                return;
            }
            n = self.n(n).nout;
        }
        self.new_pending([l, t, r, b]);
    }

    fn new_pending(&mut self, r: Rect) {
        let p = self.alloc();
        self.info[2] += 1;
        let pa = self.pend_a;
        let node = self.m(p);
        node.r = r;
        node.link = pa;
        self.pend_a = p;
    }

    /// `0x050188D0`: trozos de `(l, t, r, b)` fuera de `cut` → pendientes A.
    fn carve(&mut self, mut l: i32, mut t: i32, r: i32, b: i32, c: Rect) {
        while overlaps([l, t, r, b], c) {
            if c[2] < r && c[2] > l {
                self.carve(l, t, c[2], b, c);
                l = c[2];
            } else if c[0] > l && c[0] < r {
                self.carve(l, t, c[0], b, c);
                l = c[0];
            } else if c[1] > t && c[1] < b {
                self.carve(l, t, r, c[1], c);
                t = c[1];
            } else if c[3] > t && c[3] < b {
                self.carve(l, t, r, c[3], c);
                t = c[3];
            } else {
                return;
            }
        }
        self.new_pending([l, t, r, b]);
    }

    /// Final común de suma y resta: une o inserta los pendientes A y B.
    fn settle(&mut self) {
        while self.pend_a != NIL {
            let n = self.pend_a;
            self.pend_a = self.n(n).link;
            self.m(n).link = NIL;
            self.place(n);
        }
        while self.pend_b != NIL {
            let n = self.pend_b;
            self.pend_b = self.n(n).link;
            let r = self.n(n).r;
            self.m(n).link = NIL;
            self.query([r[0].wrapping_sub(1), r[1].wrapping_sub(1), r[2].wrapping_add(1), r[3].wrapping_add(1)]);
            self.place(n);
        }
    }

    fn place(&mut self, n: u32) {
        if self.merge(n) {
            self.info[2] -= 1;
            self.release(n);
            return;
        }
        if self.n(ROOT).child == NIL {
            self.m(ROOT).child = n;
        } else {
            self.insert(ROOT, n);
        }
        if self.state != 0 && overlaps(self.n(n).r, self.cache) {
            self.push_out(n);
        }
    }

    /// `sgl_AddClipRect` (`0x05018EE0`): une `r` a la región.
    pub fn add(&mut self, r: Rect) {
        if self.null || r[0] >= r[2] || r[1] >= r[3] {
            return;
        }
        self.query(r);
        self.split_ok = true;
        let head = self.head;
        self.cut(r[0], r[1], r[2], r[3], head);
        self.split_ok = false;
        self.settle();
        self.info[0] += 1;
        self.info[3] = 0;
        self.head = NIL;
        self.state = 0;
    }

    /// `sgl_SubClipRect` (`0x05019080`): quita `r` de la región.
    pub fn sub(&mut self, r: Rect) {
        if self.null || r[0] >= r[2] || r[1] >= r[3] {
            return;
        }
        self.query([r[0].wrapping_sub(1), r[1].wrapping_sub(1), r[2].wrapping_add(1), r[3].wrapping_add(1)]);
        self.split_ok = true;
        let mut e = self.head;
        while e != NIL {
            let next = self.n(e).nout;
            let er = self.n(e).r;
            if overlaps(er, r) {
                self.unlink_out(e);
                self.remove(ROOT, e);
                self.carve(er[0], er[1], er[2], er[3], r);
                self.info[2] -= 1;
                self.release(e);
            }
            e = next;
        }
        self.split_ok = false;
        self.settle();
        self.info[3] = 0;
        self.head = NIL;
        self.state = 0;
        self.info[1] += 1;
    }

    /// `sgl_ResetClipper` (`0x05019280`): región vacía. Conserva el límite de `set_bound`.
    pub fn reset(&mut self) {
        if self.null {
            return;
        }
        self.build_all();
        self.nodes.truncate(2);
        self.nodes[1] = Node::default();
        self.free.clear();
        self.pend_a = NIL;
        self.pend_b = NIL;
        self.head = NIL;
        self.info = [0; 5];
    }

    /// Método `+0x0C` (`0x05019350`): limita (o no) la siguiente enumeración a las hojas que solapan `r`.
    pub fn set_bound(&mut self, r: Option<Rect>) {
        if self.null {
            return;
        }
        match r {
            None => self.bounded = false,
            Some(r) => {
                self.bound = r;
                self.bounded = true;
            }
        }
    }

    /// Método `+0x14` (`0x05019380`): primera hoja de la enumeración (0 si no hay). Se avanza con [`Clipper::next`].
    pub fn first(&mut self) -> u32 {
        if self.null {
            return NIL;
        }
        if self.bounded {
            let b = self.bound;
            self.query(b);
        } else {
            self.build_all();
        }
        self.head
    }

    /// Método `+0x18` (`0x05019400`): rectángulo de la hoja `*it` y avance.
    pub fn next(&self, it: &mut u32) -> Rect {
        let n = self.n(*it);
        *it = n.nout;
        n.r
    }

    /// Todos los rectángulos de la enumeración, en el orden del controlador.
    pub fn rects(&mut self) -> Vec<Rect> {
        let mut it = self.first();
        let mut v = Vec::new();
        while it != NIL {
            v.push(self.next(&mut it));
        }
        v
    }

    /// Método `+0x2C` (`0x05018BC0`): ¿solapa `r` la región?
    pub fn is_rect_in(&self, r: Rect) -> bool {
        if self.null || r[0] >= r[2] || r[1] >= r[3] {
            return false;
        }
        self.any(self.n(ROOT).child, r)
    }

    /// Método `+0x20` (`0x05019330`).
    pub fn info(&self) -> Info {
        if self.null {
            [0; 5]
        } else {
            self.info
        }
    }

    /// Método `+0x24` (`0x05019420`): copia los hermanos desde `src_first` (de `src`) en `dst`.
    fn copy_tree(&mut self, dst: Slot, src: &Clipper, src_first: u32) {
        let (mut dst, mut s) = (dst, src_first);
        while s != NIL {
            let n = self.alloc();
            self.info[2] += 1;
            self.set(dst, n);
            self.m(n).r = src.n(s).r;
            self.copy_tree(Slot::Child(n), src, src.n(s).child);
            dst = Slot::Next(n);
            s = src.n(s).next;
        }
    }

    /// `sgl_CopyClipper` (`0x050194A0`): vacía este recortador y copia `src` (región, contadores, consulta, límite y
    /// estado de la lista, aunque la lista de salida quede vacía).
    pub fn copy_from(&mut self, src: &Clipper) {
        if self.null {
            return;
        }
        self.reset();
        let first = src.n(ROOT).child;
        if first != NIL {
            self.copy_tree(Slot::Child(ROOT), src, first);
        }
        self.cache = src.cache;
        self.info = src.info;
        self.bound = src.bound;
        self.bounded = src.bounded;
        self.state = src.state;
    }

    /// Copia de sí mismo: el controlador vacía primero y no queda nada que copiar.
    fn copy_self(&mut self) {
        if !self.null {
            self.reset();
        }
    }
}

impl Default for Clipper {
    fn default() -> Self {
        Self::new()
    }
}

/// Identificador de recortador; 0 es el nulo por defecto.
pub type ClipperId = usize;

/// Estado de recorte del controlador.
#[derive(Clone, Debug)]
pub struct SglClip {
    /// `[0x0506E898]`.
    pub mode: i32,
    /// `0x0506E758`.
    pub clip: Rect,
    /// `0x0506E314`: rectángulo de la pasada de dibujo vigente.
    pub cur: Rect,
    /// Byte `0x0508D084`.
    pub blit_clip: bool,
    clippers: Vec<Option<Clipper>>,
    active: ClipperId,
    pretested: bool, // byte 0x05068508
    iter: u32,       // [0x05068504]
}

impl Default for SglClip {
    fn default() -> Self {
        Self::new()
    }
}

impl SglClip {
    pub fn new() -> Self {
        SglClip { mode: 0, clip: [0; 4], cur: [0; 4], blit_clip: false, clippers: vec![Some(Clipper::null())], active: 0, pretested: false, iter: NIL }
    }

    pub fn clipper(&self, id: ClipperId) -> Option<&Clipper> {
        self.clippers.get(id).and_then(|c| c.as_ref())
    }

    pub fn active_id(&self) -> ClipperId {
        self.active
    }

    fn act(&mut self) -> &mut Clipper {
        self.clippers[self.active].as_mut().expect("recortador activo liberado")
    }

    /// `sgl_CreateClipper` / `sgl_GetRectClipper` (`0x05001650`).
    pub fn create_clipper(&mut self) -> ClipperId {
        self.clippers.push(Some(Clipper::new()));
        self.clippers.len() - 1
    }

    /// `sgl_SetActiveClipper` (`0x05001630`): devuelve el anterior; `None` vuelve al nulo.
    pub fn set_active(&mut self, id: Option<ClipperId>) -> ClipperId {
        let old = self.active;
        self.active = id.unwrap_or(0);
        old
    }

    /// `sgl_FreeClipper` / `sgl_FreeRectClipper` (`0x05001680`/`0x050016B0`).
    pub fn free_clipper(&mut self, id: ClipperId) {
        if id == self.active {
            self.active = 0;
        }
        if id != 0 {
            self.clippers[id] = None;
        }
    }

    pub fn add_clip_rect(&mut self, r: Rect) {
        self.act().add(r);
    }

    pub fn sub_clip_rect(&mut self, r: Rect) {
        self.act().sub(r);
    }

    pub fn reset_clipper(&mut self) {
        self.act().reset();
    }

    /// `sgl_CopyClipper` (`0x050017A0`): copia `src` en el recortador activo.
    pub fn copy_clipper(&mut self, src: ClipperId) {
        if src == self.active {
            self.act().copy_self();
        } else {
            let s = self.clippers[src].clone().expect("recortador liberado");
            self.act().copy_from(&s);
        }
    }

    /// `sgl_GetActiveClipperInfo` (`0x05001450`).
    pub fn active_info(&self) -> Info {
        self.clippers[self.active].as_ref().map_or([0; 5], |c| c.info())
    }

    /// `sgl_Set2dClippingMode` (`0x050013C0`): devuelve el modo anterior.
    pub fn set_mode(&mut self, m: i32) -> i32 {
        let old = self.mode;
        self.mode = m;
        self.blit_clip = m != 0;
        old
    }

    /// `sgl_SetClipRect`, `_4i` y `_r` (`0x05001390`/`0x05001360`/`0x05001330`).
    pub fn set_clip_rect(&mut self, r: Rect) {
        self.clip = r;
    }

    /// `sgl_IsRectInClippingRegion`, `_r` y `_4i` (`0x050015C0`/`0x050014C0`/`0x05001530`).
    pub fn is_rect_in(&self, r: Rect) -> bool {
        match self.mode {
            1 => overlaps(r, self.clip),
            2 | 3 => self.clippers[self.active].as_ref().is_some_and(|c| c.is_rect_in(r)),
            _ => true,
        }
    }

    fn clamp(&mut self, r: Rect) {
        self.cur = r;
        if self.mode == 3 {
            let c = self.clip;
            if r[0] < c[0] {
                self.cur[0] = c[0];
            }
            if r[1] < c[1] {
                self.cur[1] = c[1];
            }
            if r[2] > c[2] {
                self.cur[2] = c[2];
            }
            if r[3] > c[3] {
                self.cur[3] = c[3];
            }
        }
    }

    fn step(&mut self) {
        let mut it = self.iter;
        let r = self.clippers[self.active].as_ref().unwrap().next(&mut it);
        self.iter = it;
        self.clamp(r);
    }

    /// `0x05001840`: comprobación previa del rectángulo que se va a dibujar.
    pub fn pretest(&mut self, r: Rect) -> bool {
        if self.mode == 0 {
            return true;
        }
        if self.mode == 1 {
            if overlaps(r, self.clip) {
                self.pretested = true;
                return true;
            }
            return false;
        }
        if self.mode & 2 == 0 {
            return false;
        }
        if self.act().is_rect_in(r) {
            self.act().set_bound(Some(r));
            self.pretested = true;
            return true;
        }
        false
    }

    /// `0x050018E0`.
    pub fn end_pretest(&mut self) {
        self.pretested = false;
    }

    /// `0x05001A90`: primera pasada para dibujar en `r` (`None`: toda la región). Deja el recorte en [`SglClip::cur`]
    /// salvo en modo 0.
    pub fn begin(&mut self, r: Option<Rect>) -> bool {
        if self.mode == 0 {
            return true;
        }
        if self.pretested {
            if self.mode == 1 {
                self.cur = self.clip;
                return true;
            }
            if self.mode & 2 == 0 {
                return false;
            }
        } else {
            if self.mode == 1 {
                if let Some(r) = r {
                    if !overlaps(r, self.clip) {
                        return false;
                    }
                }
                self.cur = self.clip;
                return true;
            }
            if self.mode & 2 == 0 {
                return false;
            }
            match r {
                Some(r) => {
                    if !self.act().is_rect_in(r) {
                        return false;
                    }
                    self.act().set_bound(Some(r));
                }
                None => self.act().set_bound(None),
            }
        }
        self.iter = self.act().first();
        if self.iter == NIL {
            return false;
        }
        self.step();
        true
    }

    /// `0x05001DD0`: pasada siguiente.
    pub fn next(&mut self) -> bool {
        if self.mode == 1 || self.mode & 2 == 0 || self.iter == NIL {
            return false;
        }
        self.step();
        true
    }

    /// `0x050018F0`: como [`SglClip::begin`] pero siempre con la región, sea cual sea el modo.
    pub fn begin_region(&mut self, r: Option<Rect>) -> bool {
        match r {
            Some(r) => {
                if !self.act().is_rect_in(r) {
                    return false;
                }
                self.act().set_bound(Some(r));
            }
            None => self.act().set_bound(None),
        }
        self.iter = self.act().first();
        if self.iter == NIL {
            return false;
        }
        self.step();
        true
    }

    /// `0x050019F0`.
    pub fn next_region(&mut self) -> bool {
        if self.iter == NIL {
            return false;
        }
        self.step();
        true
    }

    /// Rectángulos de recorte con que el controlador dibujaría algo que ocupa `r` (`None`: sin rectángulo previo):
    /// `None` en la lista significa «sin recorte» (modo 0).
    pub fn passes(&mut self, r: Option<Rect>) -> Vec<Option<Rect>> {
        let mut v = Vec::new();
        if !self.begin(r) {
            return v;
        }
        if self.mode == 0 {
            v.push(None);
            return v;
        }
        v.push(Some(self.cur));
        while self.next() {
            v.push(Some(self.cur));
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_and_difference() {
        let mut c = Clipper::new();
        c.add([0, 0, 10, 10]);
        c.add([10, 0, 20, 10]);
        assert_eq!(c.rects(), vec![[10, 0, 20, 10], [0, 0, 10, 10]]); // el controlador solo une al sustituir
        c.sub([5, 5, 15, 15]);
        let area: i32 = c.rects().iter().map(|r| (r[2] - r[0]) * (r[3] - r[1])).sum();
        assert_eq!(area, 200 - 50);
        assert!(c.is_rect_in([0, 0, 1, 1]));
        assert!(!c.is_rect_in([6, 6, 14, 9]));
    }
}
