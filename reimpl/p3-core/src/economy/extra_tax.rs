//! Cobro del recargo aprobado (`st_extra_tax_collection`, `0x004EAB94`).
use crate::{ai::reputation::Merchant, scheduler::Scheduler, town::Town};

#[derive(Clone, Copy, Debug, Default)]
pub struct TaxOffice {
    pub owner: u16,
    pub next: u16,
    pub businesses: u16,
    pub houses: u16,
}

pub trait ExtraTaxHost {
    fn tax_letter(&mut self, merchant: u16, city: u8, share: u32, total: u32);
}

fn chain_for(
    s: &Scheduler,
    towns: &[Town],
    merchants: &[Merchant],
    offices: &[TaxOffice],
) -> Result<Option<(usize, Vec<usize>, u32)>, String> {
    let idx = usize::from(s.head);
    let Some(task) = s.records.get(idx).copied() else {
        return Err("tarea de recargo inexistente".into());
    };
    let city = usize::from(task.u8_at(0xc));
    if city >= towns.len() {
        return Ok(None);
    }
    let mut chain = Vec::new();
    let mut seen = vec![false; offices.len()];
    let mut at = usize::from(towns[city].u16(0x784));
    let mut weight = 0u32;
    while at < offices.len() {
        if seen[at] {
            return Err("ciclo en oficinas del recargo".into());
        }
        seen[at] = true;
        let o = offices[at];
        if usize::from(o.owner) >= merchants.len() || merchants[usize::from(o.owner)].bytes.len() < 0x650 {
            return Err("comerciante del recargo inexistente".into());
        }
        chain.push(at);
        weight = weight.wrapping_add(u32::from(o.businesses) + u32::from(o.houses) + 1);
        at = usize::from(o.next);
    }
    if chain.is_empty() {
        return Ok(None);
    }
    if weight == 0 {
        return Err("peso de recargo cero".into());
    }
    Ok(Some((city, chain, weight)))
}
pub fn validate(s: &Scheduler, towns: &[Town], merchants: &[Merchant], offices: &[TaxOffice]) -> Result<(), String> {
    chain_for(s, towns, merchants, offices).map(|_| ())
}

/// Ejecuta la tarea de cabeza y deja sus deudas y petición en el planificador.
pub fn collect(
    s: &mut Scheduler,
    towns: &mut [Town],
    merchants: &mut [Merchant],
    offices: &[TaxOffice],
    tick: u32,
    host: &mut dyn ExtraTaxHost,
) -> Result<(), String> {
    let Some((city, chain, weight)) = chain_for(s, towns, merchants, offices)? else {
        return Ok(());
    };
    let task = s.records[usize::from(s.head)];
    let total = task.u32_at(8);
    crate::economy::militia::book_transaction(&mut towns[city], tick, total as i32, 2);
    let mut foreign = false;
    for i in chain.iter().copied() {
        let o = offices[i];
        let share = (u32::from(o.businesses) + u32::from(o.houses) + 1).wrapping_mul(total >> 1) / weight
            + (total >> 1) / (chain.len() as u32);
        let m = &mut merchants[usize::from(o.owner)];
        m.set_i32(0x4bc, m.i32(0x4bc).wrapping_add(share as i32));
        let cash = m.i32(0);
        let mut missing = share;
        if cash > 0 {
            if (cash as u32) > share {
                m.set_i32(0, cash.wrapping_sub(share as i32));
                missing = 0
            } else {
                m.set_i32(0, 0);
                missing = share.wrapping_sub(cash as u32)
            }
        }
        let mut payload = share;
        while missing != 0 {
            payload |= 0x80000000;
            s.dispatching = 0;
            let id = s.add_task(tick.wrapping_add(0x700));
            s.dispatching = 1;
            let r = &mut s.records[id as usize];
            r.kind = 1;
            r.set_u8_at(0x12, 0);
            r.set_u8_at(0x15, 0);
            r.set_u8_at(0x16, 0);
            r.set_u8_at(0x17, 0);
            let n = missing.min(250000);
            r.set_u32_at(8, n);
            missing -= n;
            r.set_u8_at(0x13, 0);
            r.set_u8_at(0x10, 255);
            r.set_u8_at(0x11, o.owner as u8);
            r.set_u32_at(0xc, tick & 0xffff_ff00);
            r.set_u8_at(0x14, city as u8);
        }
        host.tax_letter(o.owner, city as u8, payload, total);
        if m.bytes[0x19] != city as u8
            && o.owner != u16::from(towns[city].u8(0x6f1))
            && towns[city].more_flags() & 0x100 != 0
        {
            foreign = true;
        }
    }
    if foreign && towns[city].flags() & 0x40 == 0 {
        s.dispatching = 0;
        let id = s.add_task(tick.wrapping_add(0x100));
        s.dispatching = 1;
        let r = &mut s.records[id as usize];
        r.kind = 0x30;
        r.set_u32_at(8, tick);
        r.set_u8_at(0x12, city as u8);
        r.set_u8_at(0x13, 1);
        r.set_u8_at(0x14, 255);
        r.set_u8_at(0x15, 0);
        r.set_u8_at(
            0xc,
            if total / 50000 < 5 {
                5 - (total / 50000) as u8
            } else {
                1
            },
        );
        for (j, t) in towns.iter().enumerate() {
            if t.flags() & 0x40 == 0 {
                continue;
            }
            let mayor = t.u8(0x6f1);
            if usize::from(mayor) < merchants.len() {
                let m = &merchants[usize::from(mayor)];
                if m.u16(8) == 0 {
                    continue;
                }
                r.set_u8_at(0xd, m.bytes[0x1c]);
                r.set_u8_at(0xe, m.bytes[0x1d]);
                r.set_u8_at(0xf, j as u8);
                r.set_u8_at(0x10, m.get_merchant_epithet_index());
                let home = usize::from(m.bytes[0x19]);
                r.set_u8_at(0x11, m.bytes[0x39c + home]);
            } else {
                let k = usize::from((0u8.wrapping_sub(mayor).wrapping_sub(1)) & 3);
                r.set_u8_at(0xd, t.u8(0x6d4 + k));
                r.set_u8_at(0xe, t.u8(0x6d8 + k));
                r.set_u8_at(0xf, j as u8);
                r.set_u8_at(0x10, t.u8(0x6e0 + k));
                r.set_u8_at(0x11, t.u8(0x6e4 + k));
            }
            break;
        }
    }
    Ok(())
}
