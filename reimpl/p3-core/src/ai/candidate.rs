//! Candidato a capitán para la taberna de un comerciante de IA (`0x004F69B0`, C06).

use crate::ai::reputation::Merchant;

/// Capitán de la tabla `0x006DD7A0` (16 bytes): siguiente `+0`, aspecto `+2`, hora de contratación `+4`.
#[derive(Clone, Copy, Default)]
pub struct CaptainEntry {
    pub next: u16,
    pub look: u8,
    pub hired: u32,
}

/// Globales: ciudades `0x006DE4B0`, reloj `0x006DE4B4`, `0x006DDB70`/`0x006DDB72` (tamaños de las tablas de aspecto) y
/// recuento de capitanes `0x006DD892`.
pub struct CandidateEnv<'a> {
    pub town_count: u32,
    pub clock: u32,
    pub look_a: u32,
    pub look_b: u16,
    pub captain_count: u16,
    pub captains: &'a [CaptainEntry],
}

/// Ganchos: longitud del nombre `0x512A60` (texto), `rand` `0x639C03` y cola de operaciones `0x54AA70`.
pub trait CandidateHost {
    fn name_length(&mut self, index: u32) -> usize;
    fn rand(&mut self) -> u32;
    fn enqueue(&mut self, record: &[u8; 0x14]);
}

impl Merchant {
    /// `0x004F69B0` (`ret 4`; argumento `index`, byte): encola la operación 0x87 (candidato a capitán) si el comerciante tiene
    /// ciudad válida (`+0x31`), `+0x28` anterior al reloj, `+0x2C` no posterior a `reloj - 0x11700` y menos candidatos (`+0x36`) que
    /// `longitud del nombre % 3 + 2`, y ningún capitán de su lista (`+0x34`) contratado en los últimos 0x11700 ticks. Elige el
    /// aspecto con `rand`, saltando los usados (el reajuste usa los módulos y el desplazamiento del original). Devuelve si encoló.
    pub fn try_enqueue_captain_candidate(&self, index: u8, env: &CandidateEnv, host: &mut dyn CandidateHost) -> bool {
        let len = host.name_length(self.bytes[0x1C] as u32);
        let limit = env.clock.wrapping_sub(0x11700);
        if env.town_count <= self.bytes[0x31] as u32
            || env.clock <= self.i32(0x28) as u32
            || limit < self.i32(0x2C) as u32
        {
            return false;
        }
        if ((len as u8 % 3) as i32 + 2) <= self.bytes[0x36] as i32 {
            return false;
        }
        let a = (env.look_a & 0xFFFF) as usize;
        let mut buf = [0u8; 0x100];
        buf[..a].fill(1);
        buf[0x80..0x100].fill(1);
        let mut si = self.u16(0x34);
        if si < env.captain_count {
            loop {
                let cap = &env.captains[si as usize];
                if limit < cap.hired {
                    return false;
                }
                buf[cap.look as usize] = 0;
                si = cap.next;
                if si >= env.captain_count {
                    break;
                }
            }
        }
        let mut dl = host.rand() as u8;
        if buf[dl as usize] == 0 {
            loop {
                dl = if dl > 0x7F {
                    (((dl & 0x7F) as u32 % env.look_b as u32) as u8).wrapping_sub(0x7F) as u32
                } else {
                    ((dl as u32 + 1) & 0xFF) % (env.look_a & 0xFFFF)
                } as u8;
                if buf[dl as usize] != 0 {
                    break;
                }
            }
        }
        let mut r = [0u8; 0x14];
        r[0..4].copy_from_slice(&0x87u32.to_le_bytes());
        r[4] = index;
        r[5] = dl;
        r[6] = host.rand() as u8 & 0x7F;
        r[7] = host.rand() as u8 & 0x7F;
        r[8] = host.rand() as u8 & 0x7F;
        r[9] = host.rand() as u8 & 0x7F;
        host.enqueue(&r);
        true
    }
}

/// Resultado de `op_add_captain_candidate` (0x0053FEA0).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AddedCandidate {
    pub merchant: usize,
    pub captain: u16,
    pub town: u8,
    pub look: u8,
}
/// Inserta el candidato en el pool de capitanes y la lista del comerciante.
/// Copia las entradas y no publica cambios si hay datos inválidos.
pub fn apply_candidate(
    op: crate::pump::Operation,
    merchants: &mut [Merchant],
    towns: &[crate::town::Town],
    pool: &mut crate::ships::captains::CaptainPool,
    tick: u32,
    day_of_year: u16,
    day: u8,
    first_names: u16,
    last_names: u16,
) -> Result<Option<AddedCandidate>, &'static str> {
    use crate::ships::captains::{CaptainHost, CaptainPool};
    if op.0[0] != 0x87 {
        return Err("operación de candidato incorrecta");
    };
    let owner = usize::from(op.0[1] as u8);
    let Some(src) = merchants.get(owner) else {
        return Ok(None);
    };
    if src.bytes.len() < 0x650 {
        return Err("comerciante candidato truncado");
    };
    if usize::from(src.bytes[0x31]) >= towns.len()
        || tick <= src.i32(0x28) as u32
        || tick < (src.i32(0x2c) as u32).wrapping_add(0x11700)
    {
        return Ok(None);
    }
    if pool.entries.len() != usize::from(pool.capacity) * 16 || pool.free_head > pool.capacity {
        return Err("pool de candidatos inválido");
    };
    let limit = tick.wrapping_sub(0x11700);
    let mut at = src.u16(0x34);
    let mut seen = vec![false; usize::from(pool.capacity)];
    while at < pool.capacity {
        let i = usize::from(at);
        if seen[i] {
            return Err("lista de candidatos cíclica");
        };
        seen[i] = true;
        let cap = &pool.entries[i * 16..i * 16 + 16];
        if limit < u32::from_le_bytes(cap[4..8].try_into().unwrap()) {
            return Ok(None);
        }
        at = u16::from_le_bytes([cap[0], cap[1]])
    }
    if first_names == 0 || last_names == 0 {
        return Err("banco de nombres de capitán ausente");
    };
    let population = towns
        .get(usize::from(day_of_year & 15))
        .ok_or("ciudad de inicialización de capitán ausente")?
        .u32(0x2e0);
    if pool.free_head == pool.capacity && pool.capacity > u16::MAX - 64 {
        return Err("pool de capitanes agotado");
    }
    struct Init;
    impl CaptainHost for Init {
        fn init(&mut self, _: u16, _: bool) {}
    }
    let mut staged: CaptainPool = pool.clone();
    let mut merchant = src.clone();
    let id = staged.allocate(1, &mut Init);
    let cap = &mut staged.entries[usize::from(id) * 16..usize::from(id + 1) * 16];
    crate::ships::factory::init_captain(
        cap,
        u32::from(id),
        false,
        tick,
        day_of_year,
        day,
        population,
        first_names,
        last_names,
    )
    .ok_or("inicialización de candidato")?;
    cap[..2].copy_from_slice(&merchant.u16(0x34).to_le_bytes());
    merchant.set_u16(0x34, id);
    merchant.bytes[0x36] = merchant.bytes[0x36].wrapping_add(1);
    let bytes = op.0[1].to_le_bytes();
    let second = op.0[2].to_le_bytes();
    cap[2] = bytes[1];
    cap[3] = merchant.bytes[0x1d];
    cap[4..8].copy_from_slice(&tick.to_le_bytes());
    cap[8] = bytes[2];
    cap[9] = bytes[3];
    cap[10] = second[0];
    cap[11] = second[1];
    cap[15] = 255;
    let result = AddedCandidate {
        merchant: owner,
        captain: id,
        town: merchant.bytes[0x19],
        look: bytes[1],
    };
    *pool = staged;
    merchants[owner] = merchant;
    Ok(Some(result))
}
