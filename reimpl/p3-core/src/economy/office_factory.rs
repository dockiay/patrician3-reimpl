//! Reserva, crecimiento y enlace de oficinas (`0x005308A0`).
use crate::{ai::reputation::Merchant, town::Town};
pub const OFFICE_SIZE: usize = 0x44c;
fn r16(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes([b[p], b[p + 1]])
}
fn w16(b: &mut [u8], p: usize, v: u16) {
    b[p..p + 2].copy_from_slice(&v.to_le_bytes())
}
fn w32(b: &mut [u8], p: usize, v: u32) {
    b[p..p + 4].copy_from_slice(&v.to_le_bytes())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OfficePool {
    pub records: Vec<Vec<u8>>,
    pub free: u16,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OfficeResult {
    pub index: Option<u16>,
    pub letter: Option<[u8; 16]>,
}

fn chain(pool: &OfficePool, mut head: u16, link: usize) -> Result<Vec<u16>, &'static str> {
    let mut seen = vec![false; pool.records.len()];
    let mut result = Vec::new();
    while usize::from(head) < pool.records.len() {
        let i = usize::from(head);
        if seen[i] {
            return Err("ciclo en oficinas");
        };
        seen[i] = true;
        let r = &pool.records[i];
        if r.len() != OFFICE_SIZE {
            return Err("oficina truncada");
        };
        result.push(head);
        head = r16(r, link)
    }
    Ok(result)
}
fn bounded_count(pool: &OfficePool, mut head: u16, link: usize, limit: usize) -> u16 {
    let mut count = 0usize;
    while usize::from(head) < pool.records.len() && count < limit {
        count += 1;
        head = r16(&pool.records[usize::from(head)], link)
    }
    count as u16
}

/// Inicializa los campos que escribe `office_init`; conserva los huecos sin inicializar.
pub fn init(record: &mut [u8], additional: bool) -> Result<(), &'static str> {
    if record.len() != OFFICE_SIZE {
        return Err("oficina truncada");
    }
    record[..0x2c0].fill(0);
    for o in [0x2c4, 0x2c8, 0x2ca, 0x2cc, 0x2ce, 0x2d0, 0x2f2] {
        w16(record, o, u16::MAX)
    }
    record[0x2c6] = 255;
    record[0x2c7] = 0;
    for o in [0x2d2, 0x2d4, 0x2d6, 0x2d8, 0x2da, 0x2dc, 0x2de, 0x2e0, 0x2e2] {
        w16(record, o, 0)
    }
    record[0x2e4..0x2e8].fill(2);
    w32(record, 0x2e8, 0);
    if additional {
        w16(record, 0x2ec, 0);
        w16(record, 0x2ee, 50);
        w16(record, 0x2f0, 40)
    } else {
        w16(record, 0x2ec, 50);
        w16(record, 0x2ee, 0);
        w16(record, 0x2f0, 0)
    }
    record[0x2f4..0x44c].fill(0);
    Ok(())
}

/// `suppress_human` corresponde a GameWorld+0x8A: si vale uno, no se
/// emite el aviso global de nueva oficina para comerciantes humanos.
pub fn find_or_create(
    pool: &mut OfficePool,
    towns: &mut [Town],
    merchants: &mut [Merchant],
    merchant_count: u16,
    owner: u16,
    city: u16,
    create: bool,
    tick: u32,
    suppress_human: bool,
) -> Result<OfficeResult, &'static str> {
    let none = OfficeResult {
        index: None,
        letter: None,
    };
    let c = usize::from(city);
    let m = usize::from(owner);
    if c >= towns.len() || owner >= merchant_count {
        return Ok(none);
    }
    if pool.records.len() > u16::MAX as usize {
        return Err("pool de oficinas demasiado grande");
    }
    if pool.records.iter().any(|r| r.len() != OFFICE_SIZE) {
        return Err("oficina truncada");
    }
    let merchant = merchants.get(m).ok_or("comerciante ausente")?;
    if merchant.bytes.len() < 0x650 {
        return Err("comerciante truncado");
    }
    let town_chain = chain(pool, towns[c].u16(0x784), 0x2ca)?;
    if let Some(&i) = town_chain
        .iter()
        .find(|&&i| r16(&pool.records[usize::from(i)], 0x2c4) == owner)
    {
        return Ok(OfficeResult {
            index: Some(i),
            letter: None,
        });
    }
    if !create {
        return Ok(none);
    }
    chain(pool, merchant.u16(0xc), 0x2c8)?;
    let mut staged = pool.clone();
    if usize::from(staged.free) >= staged.records.len() {
        let old = staged.records.len();
        let capacity = if old == 0 {
            64
        } else {
            old.checked_add(32).ok_or("pool demasiado grande")?
        };
        if capacity > u16::MAX as usize {
            return Err("pool demasiado grande");
        }
        staged.free = old as u16;
        for i in old..capacity {
            let mut record = vec![0; OFFICE_SIZE];
            init(&mut record, false)?;
            w16(&mut record, 0x2c8, (i + 1) as u16);
            staged.records.push(record);
        }
    }
    // El original compara el enlace con la capacidad ya ampliada. Un índice que
    // antes era centinela puede convertirse en ranura válida tras crecer el pool.
    let additional = usize::from(merchant.u16(0xc)) < staged.records.len();
    let index = staged.free;
    let i = usize::from(index);
    if staged.records[i].len() != OFFICE_SIZE {
        return Err("oficina libre truncada");
    };
    let next_free = r16(&staged.records[i], 0x2c8);
    let mut record = staged.records[i].clone();
    init(&mut record, additional)?;
    w16(&mut record, 0x2c4, owner);
    w16(&mut record, 0x2c8, merchant.u16(0xc));
    record[0x2c6] = city as u8;
    w16(&mut record, 0x2ca, towns[c].u16(0x784));
    w32(&mut record, 0x2c0, tick);
    staged.records[i] = record;
    let letter = if index >= merchant_count && (merchant.u16(8) != 0 || !suppress_human) {
        let mut letter = [0u8; 16];
        letter[1] = 0x80;
        letter[2] = 255;
        letter[3] = 255;
        letter[4] = 0x0d;
        letter[5] = city as u8;
        w16(&mut letter, 8, owner);
        w16(&mut letter, 10, bounded_count(&staged, index, 0x2c8, towns.len() + 2));
        w16(
            &mut letter,
            12,
            bounded_count(&staged, index, 0x2ca, usize::from(merchant_count) + 2),
        );
        let home = usize::from(merchant.bytes[0x19]);
        let rank = merchant.bytes[0x39c + home];
        w16(
            &mut letter,
            14,
            u16::from(merchant.get_merchant_epithet_index() & 15) + (u16::from(rank) << 4),
        );
        Some(letter)
    } else {
        None
    };
    staged.free = next_free;
    *pool = staged;
    merchants[m].set_u16(0xc, index);
    towns[c].set_u16(0x784, index);
    Ok(OfficeResult {
        index: Some(index),
        letter,
    })
}
