//! Informe de caza de los piratas (`0x004F3AF0`, render `0x004F3CE0`) en la llegada del convoy de la Liga (`0x004E09D4`,
//! fase 10): textos del ejecutable, `CString` con semántica real de `Format` y `+=`, nombres de los bancos de p3-sim y
//! entrega del mensaje 0x3D con el publicador de `p3-state`.
use crate::c08_text::{cstr, Arena, NameBanks, BUF, TEXT};
use p3_core::{
    ai::reputation::Merchant,
    economy::{
        cformat::{self, FArg},
        hunt_arrival::{hunt_arrive, ArrivalHost, ArrivalWorld},
        hunt_report::{send_pirate_hunt_report_message, HuntHost, L1, L2, TMP},
    },
    flatmem::{Mem, SparseMem},
    scheduler::TaskRecord,
};
use p3_state::messages::{post_message, Payload, RETENTION};

/// Punteros `0x6B1338..=0x6B1374` del ejecutable inglés (16 textos de formato y fragmentos del informe) y el byte `0x672858`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HuntTexts {
    pub cells: [Vec<u8>; 16],
    /// Clase del nodo raíz que crea la llegada cuando no hay lista de bajas.
    pub class_byte: u8,
}
impl HuntTexts {
    pub fn from_english_exe(bytes: &[u8]) -> Result<Self, String> {
        let image = p3_assets::exe::PeImage::parse(bytes).map_err(str::to_string)?;
        let mut cells: [Vec<u8>; 16] = Default::default();
        for (i, cell) in cells.iter_mut().enumerate() {
            let slot = 0x6B1338 + 4 * i as u32;
            *cell = image.pointed_cstr(slot).map_err(|e| format!("texto {slot:#x}: {e}"))?.to_vec();
        }
        let class_byte = image.byte_at(0x672858).map_err(|e| format!("byte 0x672858: {e}"))?;
        Ok(Self { cells, class_byte })
    }
}

const EMP: u32 = 0x5d00_0000;
const STR: u32 = 0x5f00_0000;
/// Nodos de la lista de bajas sembrados para la llegada.
const LOSSES: u32 = 0x5b00_0000;

struct Host<'a> {
    arena: Arena,
    str_next: u32,
    banks: NameBanks<'a>,
    ship_names: &'a [Vec<u8>],
    pool: &'a mut Vec<u8>,
    merchants: &'a mut [Merchant],
    count: u16,
    date: (u8, u8, u16),
    error: Option<String>,
}
fn content(m: &mut dyn Mem, p: u32) -> Vec<u8> {
    let n = m.r32(p.wrapping_sub(8));
    (0..n).map(|k| m.r8(p + k)).collect()
}
impl Host<'_> {
    /// Nueva cadena `CString`: cabecera de 8 bytes (longitud y cuenta) y datos con NUL.
    fn newstr(&mut self, m: &mut dyn Mem, bytes: &[u8]) -> u32 {
        let h = self.str_next;
        let p = h + 8;
        for (k, x) in (bytes.len() as u32).to_le_bytes().iter().chain([0u8; 4].iter()).enumerate() {
            m.w8_seed(h + k as u32);
            m.w8(h + k as u32, *x);
        }
        for (k, x) in bytes.iter().chain([0u8].iter()).enumerate() {
            m.w8_seed(p + k as u32);
            m.w8(p + k as u32, *x);
        }
        self.str_next = (p + bytes.len() as u32 + 1 + 7) & !7;
        p
    }
    fn fail(&mut self, e: impl Into<String>) {
        self.error.get_or_insert(e.into());
    }
    fn heap(&mut self, m: &mut dyn Mem, size: u32) -> u32 {
        let p = self.arena.buf_next;
        self.arena.buf_next = (p + size + 15) & !15;
        for k in 0..size {
            m.w8_seed(p + k);
        }
        p
    }
}
impl HuntHost for Host<'_> {
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32 {
        self.heap(m, size)
    }
    fn format(&mut self, m: &mut dyn Mem, dst: u32, fmt: u32, args: &[u32]) {
        let f = cstr(m, fmt);
        let a: Vec<FArg> = args.iter().map(|&p| FArg::Str(cstr(m, p))).collect();
        let out = cformat::format(&f, &a).unwrap_or_else(|e| {
            self.fail(e);
            Vec::new()
        });
        let p = self.newstr(m, &out);
        m.w32(dst, p);
    }
    fn append_lit(&mut self, m: &mut dyn Mem, dst: u32, lit: u32) {
        let dp = m.r32(dst);
        let mut c = content(m, dp);
        c.extend(cstr(m, lit));
        let p = self.newstr(m, &c);
        m.w32(dst, p);
    }
    fn append(&mut self, m: &mut dyn Mem, dst: u32, src: u32) {
        let sp = m.r32(src);
        let add = content(m, sp);
        let dp = m.r32(dst);
        let mut c = content(m, dp);
        c.extend(add);
        let p = self.newstr(m, &c);
        m.w32(dst, p);
    }
    fn dtor(&mut self, _c: u32) {}
    fn text(&mut self, m: &mut dyn Mem, v: u32) -> u32 {
        let id = (v & 0xffff) as usize;
        let name = self.ship_names.get(id).cloned().unwrap_or_else(|| {
            self.fail(format!("nombre de barco {id} no modelado (banco 0x512B20)"));
            Vec::new()
        });
        self.arena.intern(m, &name)
    }
    fn text_a(&mut self, m: &mut dyn Mem, v: u32) -> u32 {
        let s = self.banks.family(v).to_vec();
        self.arena.intern(m, &s)
    }
    fn text_b(&mut self, m: &mut dyn Mem, v: u32) -> u32 {
        let s = self.banks.given(v).to_vec();
        self.arena.intern(m, &s)
    }
    fn post(&mut self, m: &mut dyn Mem, code: u32, msg: &mut [u8; 16]) {
        let obj = u32::from_le_bytes([msg[8], msg[9], msg[10], msg[11]]);
        let buf = u32::from_le_bytes([msg[12], msg[13], msg[14], msg[15]]);
        let mut object = [0u8; 16];
        for (k, b) in object.iter_mut().enumerate() {
            *b = m.r8(obj + k as u32);
        }
        let len = u32::from_le_bytes([object[0], object[1], object[2], object[3]]);
        let block: Vec<u8> = (0..len).map(|k| m.r8(buf + k)).collect();
        let (day, month, year) = self.date;
        if let Err(e) = post_message(self.pool, self.merchants, self.count, day, month, year, code as i32, msg, Payload::Sized { object, block: &block }, &RETENTION) {
            self.fail(e);
        }
    }
    fn msg_dtor(&mut self) {}
}
impl ArrivalHost for Host<'_> {
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32 {
        self.heap(m, size)
    }
    fn free(&mut self, _p: u32) {}
    fn report(&mut self, m: &mut dyn Mem, node: u32, merchant: u32, a2: u32, a3: u32, a4: u32) {
        send_pirate_hunt_report_message(m, self, node, merchant, a2, a3, a4);
    }
}

/// Llegada del convoy de caza (fase 10 de `0x004E09D4`): informa a cada humano con el mensaje 0x3D en `pool`, libera la lista de
/// bajas `losses` (cabeza primero, la de `tarea+8`; vacía si `+8` es nulo) y deja convoy y barcos en estado 0xF.
/// `election_town` = byte `0x006DE52E` (ciudad que firma el informe); `town_name(i)` nombra las ciudades.
#[allow(clippy::too_many_arguments)]
pub fn arrive_hunt(
    pool: &mut Vec<u8>,
    merchants: &mut [Merchant],
    merchant_count: u16,
    date: (u8, u8, u16),
    texts: &HuntTexts,
    banks: NameBanks<'_>,
    ship_names: &[Vec<u8>],
    town_name: &dyn Fn(u32) -> Vec<u8>,
    election_town: u8,
    town_count: u32,
    r: &mut TaskRecord,
    losses: &[p3_core::ships::return_convoy::HuntLoss],
    convoy: &mut [u8],
    ships: &mut [Option<Vec<u8>>],
    ship_count: u16,
) -> Result<(), String> {
    let count = usize::from(merchant_count);
    if merchants.len() < count {
        return Err("comerciantes incompletos para la caza".into());
    }
    let humans: Vec<bool> = merchants.iter().take(count).map(|m| m.u16(8) == 0).collect();
    let mut arena = Arena { text_next: TEXT, buf_next: BUF };
    let mut mem = SparseMem::default();
    for (i, text) in texts.cells.iter().enumerate() {
        let p = arena.intern(&mut mem, text);
        mem.seed(0x6B1338 + 4 * i as u32, &p.to_le_bytes());
    }
    mem.seed(EMP, &[0; 12]);
    mem.seed(0x6C7CCC, &(EMP + 8).to_le_bytes());
    mem.seed(L1, &[0; 8]);
    mem.seed(TMP, &[0; 0x40]);
    mem.seed(L2, &[0; 4]);
    for k in 0..256u32 {
        let name = if k < town_count { town_name(k) } else { Vec::new() };
        let p = arena.intern(&mut mem, &name);
        mem.seed(0x6DDA00 + 4 * k, &p.to_le_bytes());
    }
    mem.seed(0x6DE52E, &[election_town]);
    // Lista colgante de `tarea+8`: nodos de 8 bytes `{hijo, clase, marca, texto}` enlazados.
    if losses.is_empty() {
        r.set_u32_at(8, 0);
    } else {
        for (k, n) in losses.iter().enumerate() {
            let at = LOSSES + 8 * k as u32;
            let child = if k + 1 < losses.len() { at + 8 } else { 0 };
            let mut node = child.to_le_bytes().to_vec();
            node.extend_from_slice(&[n.class, n.mark]);
            node.extend_from_slice(&n.text.to_le_bytes());
            mem.seed(at, &node);
        }
        r.set_u32_at(8, LOSSES);
    }
    let mut host = Host { arena, str_next: STR, banks, ship_names, pool, merchants, count: merchant_count, date, error: None };
    let world = ArrivalWorld { humans: &humans, ship_count, class_byte: texts.class_byte };
    let done = hunt_arrive(r, convoy, ships, &world, &mut mem, &mut host);
    match (host.error.take(), done) {
        (Some(e), _) | (None, Err(e)) => Err(e),
        (None, Ok(())) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pool(slots: usize) -> Vec<u8> {
        let mut p = vec![0u8; 12 + slots * 16];
        p[..4].copy_from_slice(&0xaffe0005u32.to_le_bytes());
        p[4..6].copy_from_slice(&(slots as u16).to_le_bytes());
        p[8..10].copy_from_slice(&u16::MAX.to_le_bytes());
        for i in 0..slots {
            p[12 + i * 16 + 4] = 0xff;
            p[12 + i * 16 + 6..12 + i * 16 + 8].copy_from_slice(&((i + 1) as u16).to_le_bytes());
        }
        p
    }

    #[test]
    fn arrival_delivers_the_hunt_letter_and_closes_the_convoy() {
        let first = [vec![b"Hans".to_vec(), b"Karl".to_vec(), b"Otto".to_vec(), b"Jens".to_vec()], vec![b"Anna".to_vec()]];
        let family: Vec<Vec<u8>> = vec![b"Brun".to_vec(), b"Voss".to_vec(), b"Klee".to_vec(), b"Lund".to_vec()];
        let banks = NameBanks { first: &first, family: &family };
        let mut cells: [Vec<u8>; 16] = Default::default();
        for (i, c) in cells.iter_mut().enumerate() {
            *c = format!("<{i}>").into_bytes();
        }
        cells[0] = b"%s %s hunted;".to_vec();
        cells[1] = b"%s %s hunted;".to_vec();
        cells[2] = b"%s %s hunted;".to_vec();
        cells[3] = b"%s %s hunted;".to_vec();
        cells[4] = b"[a]".to_vec();
        cells[5] = b"[b]".to_vec();
        cells[6] = b"[c]".to_vec();
        cells[7] = b"[d]".to_vec();
        cells[8] = b" Signed %s.".to_vec();
        let texts = HuntTexts { cells, class_byte: 3 };
        let mut merchants: Vec<Merchant> = (0..3)
            .map(|i| {
                let mut m = Merchant { bytes: vec![0; 0x650] };
                m.bytes[0x19] = (i % 2) as u8;
                m.bytes[8] = if i == 1 { 1 } else { 0 };
                m
            })
            .collect();
        let mut pool = pool(4);
        let mut r = TaskRecord::default();
        r.set_u8_at(0xd, 2);
        r.set_u8_at(0xe, 1);
        let mut convoy = vec![0u8; 0x40];
        convoy[0xa] = 9;
        let mut ships: Vec<Option<Vec<u8>>> = Vec::new();
        let names = |i: u32| format!("Town{i}").into_bytes();
        arrive_hunt(&mut pool, &mut merchants, 3, (4, 11, 1400), &texts, banks, &[], &names, 1, 2, &mut r, &[], &mut convoy, &mut ships, 5).unwrap();
        let parsed = p3_save::tail::MessagePool::parse(&pool).unwrap();
        let kinds: Vec<u8> = parsed.occupied().map(|(_, l)| l.kind).collect();
        assert_eq!(kinds, vec![0x3d, 0x3d], "una carta por humano (comerciantes 0 y 2)");
        assert!(String::from_utf8_lossy(&parsed.letter(0)[16..]).ends_with("Hans Klee hunted; Signed Town1.\0\0\0\0"));
        assert_eq!(convoy[0x12], 0xf);
        assert_eq!(convoy[0x18] & 0x20, 0x20);
        assert_eq!(r.u32_at(8), 0);
    }
}
