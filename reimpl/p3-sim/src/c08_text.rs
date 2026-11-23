//! Conexión de los mensajes de C08 con el estado real de la simulación: bancos de nombres, `CString::Format` (subconjunto
//! `printf` contrastado con el original), memoria de trabajo y publicador de mensajes de `p3-state`.
//! Hoy cubre la elección del Ältermann (`0x005299F0`); el llamante original (`0x004DBBD0`) aún no está portado.
use p3_core::{
    ai::reputation::Merchant,
    economy::{
        cformat::{self, FArg},
        election_msgs::{post_alderman_election_messages, ElectionHost},
    },
    flatmem::{Mem, SparseMem},
    town::Town,
};
use p3_state::messages::{post_message, Payload, RETENTION};

/// Textos de `0x6B72F0..=0x6B7300` del ejecutable (edición inglesa): cuatro fragmentos y el formato de cada candidato.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ElectionTexts {
    pub cells: [Vec<u8>; 5],
    /// Textos de `0x6B72DC..=0x6B72EC` para la carta 0x47 de la elección municipal (`0x00528E60`): encabezado, tras
    /// la ciudad, formato de cada candidato, antes del ganador y cierre. `None` si no se cargaron.
    pub mayor: Option<[Vec<u8>; 5]>,
}
impl ElectionTexts {
    pub fn from_english_exe(bytes: &[u8]) -> Result<Self, String> {
        let image = p3_assets::exe::PeImage::parse(bytes).map_err(str::to_string)?;
        let mut cells: [Vec<u8>; 5] = Default::default();
        for (i, slot) in [0x6B72F0u32, 0x6B72F4, 0x6B72F8, 0x6B72FC, 0x6B7300].into_iter().enumerate() {
            cells[i] = image.pointed_cstr(slot).map_err(|e| format!("texto {slot:#x}: {e}"))?.to_vec();
        }
        let mut mayor: [Vec<u8>; 5] = Default::default();
        for (i, slot) in [0x6B72DCu32, 0x6B72E0, 0x6B72E4, 0x6B72E8, 0x6B72EC].into_iter().enumerate() {
            mayor[i] = image.pointed_cstr(slot).map_err(|e| format!("texto {slot:#x}: {e}"))?.to_vec();
        }
        Ok(Self { cells, mayor: Some(mayor) })
    }
}

/// Texto de la carta 0x47 de `run_mayor_election` (`0x00528E60`), terminado en NUL como lo cuenta el original
/// (`+0xC` incluye el terminador). `candidates` son (nombre, apellido, votos); `winner`, (nombre, apellido).
/// El original reserva 0x800 bytes y no comprueba el tamaño: un texto mayor se rechaza.
pub fn mayor_election_letter(texts: &[Vec<u8>; 5], town_name: &[u8], candidates: &[(Vec<u8>, Vec<u8>, u32); 4], winner: (&[u8], &[u8])) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    out.extend_from_slice(&texts[0]);
    out.extend_from_slice(town_name);
    out.extend_from_slice(&texts[1]);
    for (given, family, votes) in candidates {
        let line = cformat::format(&texts[2], &[FArg::Str(given.clone()), FArg::Str(family.clone()), FArg::Int(*votes)])?;
        out.extend_from_slice(&line);
    }
    out.extend_from_slice(&texts[3]);
    out.extend_from_slice(winner.0);
    out.push(b' ');
    out.extend_from_slice(winner.1);
    out.extend_from_slice(&texts[4]);
    out.push(0);
    if out.len() > 0x800 {
        return Err("texto de la elección municipal mayor que el búfer original".into());
    }
    Ok(out)
}

/// Bancos de nombres cargados (nombres de pila `[masculino, femenino]` y apellidos, ambos con las 20 ranuras propias al final).
#[derive(Clone, Copy)]
pub struct NameBanks<'a> {
    pub first: &'a [Vec<Vec<u8>>; 2],
    pub family: &'a [Vec<u8>],
}
impl NameBanks<'_> {
    /// `0x00512A60`: el bit 7 elige la tabla femenina; fuera de rango devuelve el primer nombre.
    pub fn given(&self, id: u32) -> &[u8] {
        let (table, index) = if id & 0x80 == 0x80 { (&self.first[1], (id & 0x7f) as usize) } else { (&self.first[0], id as usize) };
        table.get(index).or_else(|| table.first()).map_or(&[], Vec::as_slice)
    }
    /// `0x00512AE0`.
    pub fn family(&self, id: u32) -> &[u8] {
        self.family.get(id as usize).or_else(|| self.family.first()).map_or(&[], Vec::as_slice)
    }
}

const MERCH: u32 = 0x5800_0000;
const TOWNS: u32 = 0x5c00_0000;
const ARGS: u32 = 0x5300_0000;
pub(crate) const TEXT: u32 = 0x5a00_0000;
pub(crate) const BUF: u32 = 0x5600_0000;

pub(crate) struct Arena {
    pub(crate) text_next: u32,
    pub(crate) buf_next: u32,
}
impl Arena {
    pub(crate) fn intern(&mut self, m: &mut dyn Mem, bytes: &[u8]) -> u32 {
        let p = self.text_next;
        for (k, &b) in bytes.iter().chain([0u8].iter()).enumerate() {
            m.w8_seed(p + k as u32);
            m.w8(p + k as u32, b);
        }
        self.text_next = (p + bytes.len() as u32 + 1 + 3) & !3;
        p
    }
}
pub(crate) fn cstr(m: &mut dyn Mem, mut a: u32) -> Vec<u8> {
    let mut v = Vec::new();
    loop {
        let c = m.r8(a);
        if c == 0 {
            return v;
        }
        v.push(c);
        a += 1;
    }
}

struct Host<'a> {
    arena: &'a mut Arena,
    banks: NameBanks<'a>,
    pool: &'a mut Vec<u8>,
    merchants: &'a mut [Merchant],
    count: u16,
    date: (u8, u8, u16),
    flags: Vec<u8>,
    error: Option<String>,
}
impl ElectionHost for Host<'_> {
    fn alloc(&mut self, m: &mut dyn Mem, size: u32) -> u32 {
        let p = self.arena.buf_next;
        self.arena.buf_next = (p + size + 15) & !15;
        for k in 0..size {
            m.w8_seed(p + k);
        }
        p
    }
    fn text_a(&mut self, m: &mut dyn Mem, v: u32) -> u32 {
        let s = self.banks.family(v).to_vec();
        self.arena.intern(m, &s)
    }
    fn text_b(&mut self, m: &mut dyn Mem, v: u32) -> u32 {
        let s = self.banks.given(v).to_vec();
        self.arena.intern(m, &s)
    }
    fn sprintf(&mut self, m: &mut dyn Mem, fmt: u32, b: u32, a: u32, town_name: u32, q: u32) -> Vec<u8> {
        let f = cstr(m, fmt);
        let args = [FArg::Str(cstr(m, b)), FArg::Str(cstr(m, a)), FArg::Str(cstr(m, town_name)), FArg::Int(q)];
        cformat::format(&f, &args).unwrap_or_else(|e| {
            self.error.get_or_insert(e);
            Vec::new()
        })
    }
    fn post(&mut self, m: &mut dyn Mem, code: i32, msg: &mut [u8; 16]) {
        let kind = msg[4];
        let (day, month, year) = self.date;
        let result = if matches!(kind, 0x29 | 0x47 | 0x63 | 0x83) {
            let p = u32::from_le_bytes([msg[8], msg[9], msg[10], msg[11]]);
            let len = u32::from_le_bytes([msg[12], msg[13], msg[14], msg[15]]);
            let text: Vec<u8> = (0..len).map(|k| m.r8(p + k)).collect();
            post_message(self.pool, self.merchants, self.count, day, month, year, code, msg, Payload::Text(&text), &RETENTION)
        } else {
            post_message(self.pool, self.merchants, self.count, day, month, year, code, msg, Payload::None, &RETENTION)
        };
        if let Err(e) = result {
            self.error.get_or_insert(e);
        }
    }
    fn free(&mut self, _p: u32) {}
    fn msg_dtor(&mut self) {}
    fn merchant_flag(&mut self, mer: u32) -> u8 {
        let i = ((mer - MERCH) / 0x650) as usize;
        self.flags.get(i).copied().unwrap_or(0)
    }
}

/// Publica los dos mensajes de la elección del Ältermann (texto con los candidatos y aviso resumido) en `pool`.
/// `candidates`: byte con bit 7 = comerciante, si no una ciudad; `votes`: votos de cada uno; `kind` y `town` como `0x005299F0`.
/// `town_name(i)` devuelve el nombre de la ciudad `i`. La interfaz (notificación del jugador local) no interviene.
#[allow(clippy::too_many_arguments)]
pub fn post_alderman_election(
    pool: &mut Vec<u8>,
    merchants: &mut [Merchant],
    towns: &[Town],
    merchant_count: u16,
    local_merchant: u8,
    date: (u8, u8, u16),
    texts: &ElectionTexts,
    banks: NameBanks<'_>,
    town_name: &dyn Fn(u32) -> Vec<u8>,
    candidates: [u8; 4],
    votes: [u8; 4],
    kind: u8,
    town: u8,
) -> Result<(), String> {
    let count = usize::from(merchant_count);
    if merchants.len() < count || merchants.iter().take(count).any(|m| m.bytes.len() < 0x650) {
        return Err("comerciantes incompletos para la elección".into());
    }
    let mut arena = Arena { text_next: TEXT, buf_next: BUF };
    let mut mem = SparseMem::default();
    for (i, cell) in [0x6B72F0u32, 0x6B72F4, 0x6B72F8, 0x6B72FC, 0x6B7300].into_iter().enumerate() {
        let p = arena.intern(&mut mem, &texts.cells[i]);
        mem.seed(cell, &p.to_le_bytes());
    }
    for (i, m) in merchants.iter().take(count).enumerate() {
        mem.seed(MERCH + i as u32 * 0x650, &m.bytes[..0x650]);
    }
    // Una ranura vacía (candidato 0xFF, comerciante 0x7F) hace que el original lea tras la tabla de comerciantes;
    // esa memoria se toma como ceros (con otros bytes cambia el texto de la carta).
    if candidates.iter().any(|&c| c & 0x80 != 0 && usize::from(c & 0x7f) >= count) {
        let zero = vec![0u8; 0x650];
        for i in count..0x80 {
            mem.seed(MERCH + i as u32 * 0x650, &zero);
        }
    }
    for (i, t) in towns.iter().enumerate() {
        mem.seed(TOWNS + i as u32 * 0x9f8, &t.0[..]);
    }
    for k in 0..256u32 {
        let name = if (k as usize) < towns.len() { town_name(k) } else { Vec::new() };
        let p = arena.intern(&mut mem, &name);
        mem.seed(0x6DDA00 + 4 * k, &p.to_le_bytes());
    }
    mem.seed(0x6DE4AA, &merchant_count.to_le_bytes());
    mem.seed(0x6DE52D, &[local_merchant]);
    mem.seed(0x6DE4A0 + 0x78, &MERCH.to_le_bytes());
    mem.seed(0x6DE508, &TOWNS.to_le_bytes());
    mem.seed(ARGS, &candidates);
    mem.seed(ARGS + 4, &votes);
    let flags: Vec<u8> = merchants.iter().take(count).map(|m| m.get_merchant_epithet_index()).collect();
    let mut host = Host { arena: &mut arena, banks, pool, merchants, count: merchant_count, date, flags, error: None };
    post_alderman_election_messages(&mut mem, &mut host, ARGS, ARGS + 4, u32::from(kind), u32::from(town));
    match host.error.take() {
        Some(e) => Err(e),
        None => Ok(()),
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
    fn election_text_reaches_every_human_with_real_names_and_format() {
        let first = [vec![b"Hans".to_vec(), b"Karl".to_vec(), b"Otto".to_vec(), b"Jens".to_vec()], vec![b"Anna".to_vec(), b"Greta".to_vec()]];
        let family: Vec<Vec<u8>> = vec![b"Brun".to_vec(), b"Voss".to_vec(), b"Klee".to_vec(), b"Lund".to_vec()];
        let banks = NameBanks { first: &first, family: &family };
        let texts = ElectionTexts { cells: [b"Elect:".to_vec(), b" now ".to_vec(), b"|end ".to_vec(), b"!".to_vec(), b"\\l%s %s from %s:\n\\c%u votes\n".to_vec()], mayor: None };
        let mut merchants: Vec<Merchant> = (0..4)
            .map(|i| {
                let mut m = Merchant { bytes: vec![0; 0x650] };
                m.bytes[0x19] = (i % 2) as u8;
                m.bytes[0x1c] = i as u8;
                m.bytes[0x1d] = (3 - i) as u8;
                m.bytes[0x38] = 1;
                m.bytes[0x39] = 1;
                m.bytes[0x3a] = 1;
                m.bytes[8] = if i == 2 { 1 } else { 0 };
                m
            })
            .collect();
        let towns = vec![Town::default(), Town::default()];
        let mut pool = pool(3);
        let names = |i: u32| format!("Town{i}").into_bytes();
        post_alderman_election(&mut pool, &mut merchants, &towns, 4, 0, (3, 11, 1400), &texts, banks, &names, [0x80, 0x81, 0x82, 0x83], [10, 20, 30, 40], 1, 1).unwrap();
        let parsed = p3_save::tail::MessagePool::parse(&pool).unwrap();
        // El primer mensaje (código −1) llega a los comerciantes humanos 0, 1 y 3.
        let letter = parsed.letter(0);
        assert_eq!(letter[4], 0x47);
        let text = &letter[16..];
        let want = "Elect:Town1 now \\lHans Lund from Town0:\n\\c10 votes\n\\lKarl Klee from Town1:\n\\c20 votes\n\\lOtto Voss from Town0:\n\\c30 votes\n\\lJens Brun from Town1:\n\\c40 votes\n|end Hans Lund!\0\0";
        assert_eq!(String::from_utf8_lossy(text), want);
        assert_eq!(parsed.occupied().filter(|(_, l)| l.kind == 0x47).count(), 3);
    }
}
