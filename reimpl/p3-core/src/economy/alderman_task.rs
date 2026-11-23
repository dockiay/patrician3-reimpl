//! Tarea 0x20 (`0x004DBBD0`): elección del Ältermann. Una llamada por tick del planificador sobre el registro de 24 bytes.
//!
//! Fases según el byte bajo del vencimiento (`rec.due & 0xFF`):
//! * 0: reúne candidatos (`0x00529700`), cuenta votantes y reparte los votos por candidato.
//! * 1..=0xFB: recuento animado, un voto por tick (estados 0xC0..: sin condición de máximo).
//! * 0xFC..: cierre, ganador, rangos, banderas de ciudad, mensajes (`0x005299F0`) y reprogramación un año después.
//!
//! Interfaz, avisos de pantalla y rangos (`0x004F75F0`) salen por [`ElectionTaskHost`].
use crate::{
    ai::reputation::Merchant,
    economy::alderman_candidates::{collect, Candidates},
    scheduler::TaskRecord,
    town::Town,
};

/// Estado global que lee o escribe la tarea.
pub struct ElectionWorld<'a> {
    pub towns: &'a mut [Town],
    pub merchants: &'a mut [Merchant],
    /// Byte `0x006DE52D`: comerciante Ältermann.
    pub player: u8,
    /// Byte `0x006DE52E`: ciudad de la elección.
    pub election_town: u8,
    /// Dword `0x006DFC14`: comerciante local.
    pub local_player: u32,
    /// Byte `0x006737B9`: escala de los votos.
    pub vote_scale: u8,
}

pub trait ElectionTaskHost {
    /// `0x00548CA0` (ECX = `0x6DF2F0`): `[comerciante, 0, 0, palabra, código, extra]`; el código es 0xD (candidato) o 0x11 (ganador).
    fn notification(&mut self, w: &mut ElectionWorld, args: [u32; 6]);
    /// `0x00469AF0` con el aviso al ganador humano: código 0x25 si es el comerciante local (seguido de `0x005499F0`), 6 si no.
    fn winner_announcement(&mut self, w: &mut ElectionWorld, local: bool, winner: u32, town_word: u16);
    /// `0x004F75F0`: rango del comerciante en la ciudad.
    fn update_rank(&mut self, w: &mut ElectionWorld, merchant: usize, town: u32);
    /// `0x005299F0`: mensajes de la elección.
    fn post_messages(
        &mut self,
        w: &mut ElectionWorld,
        candidates: [u8; 4],
        votes: [u8; 4],
        kind: u8,
        town: u8,
    ) -> Result<(), String>;
}

/// `0x00529F20` (ECX = ciudad): banderas de ranura 0x6E4..0x6E7.
pub fn mark_town(t: &mut Town, election_town: u8, player: u8, merchant_count: u16) {
    t.set_u8(0x6e4, 5);
    t.set_u8(0x6e5, 5);
    t.set_u8(0x6e6, 4);
    t.set_u8(0x6e7, 4);
    let b = t.u8(0x6f1);
    if u16::from(b) < merchant_count {
        return;
    }
    let slot = usize::from(!b & 3);
    t.set_u8(
        0x6e4 + slot,
        if election_town == t.u8(0x2c0) && player == slot as u8 {
            7
        } else {
            6
        },
    );
}

fn merchant_ok(w: &ElectionWorld, i: usize) -> Result<(), String> {
    if i < w.merchants.len() {
        Ok(())
    } else {
        Err(format!("comerciante {i} fuera de rango"))
    }
}

fn town_ok(w: &ElectionWorld, i: usize) -> Result<(), String> {
    if i < w.towns.len() {
        Ok(())
    } else {
        Err(format!("ciudad {i} fuera de rango"))
    }
}

fn score(w: &ElectionWorld, m: usize) -> i32 {
    i32::from(w.merchants[m].calc_total_reputation_score(w.towns.len() as i32, w.towns) as i16)
}

/// Un paso de la tarea. `Err` cuando el original dividiría entre cero o leería fuera de las tablas.
pub fn step(rec: &mut TaskRecord, w: &mut ElectionWorld, host: &mut dyn ElectionTaskHost) -> Result<(), String> {
    let stage = (rec.due & 0xff) as u8;
    let mc = w.merchants.len() as u16;
    if stage == 0 {
        rec.set_u8_at(0x17, w.election_town);
        rec.set_u8_at(0x14, 0);
        let c = collect(&Candidates {
            towns: w.towns,
            merchants: w.merchants,
            player: w.player,
            election_town: w.election_town,
        });
        for k in 0..4 {
            let id = c[0x14 + k];
            let v = if u16::from(id) > mc { c[k] } else { id | 0x80 };
            rec.set_u8_at(8 + k, v);
            rec.set_u8_at(0xc + k, 0);
            rec.set_u8_at(0x10 + k, 0);
        }
        rec.set_u8_at(0x16, 0);
        for t in w.towns.iter() {
            for j in 0..4 {
                if u16::from(t.u8(0x6dc + j)) < mc {
                    rec.set_u8_at(0x16, rec.u8_at(0x16).wrapping_add(1));
                }
            }
        }
        let mut ebx = u32::from(rec.u8_at(0x16));
        let mut human = 0u32;
        for i in 0..usize::from(mc) {
            if w.merchants[i].u16(8) != 0 {
                continue;
            }
            let home = usize::from(w.merchants[i].bytes[0x19]);
            let rank = u32::from(w.merchants[i].bytes[0x39c + home]);
            if rank >= 4 {
                let bit = if human & 31 < 8 { 1u8 << (human & 31) } else { 0 };
                rec.set_u8_at(0x14, rec.u8_at(0x14) | bit);
                ebx += if rank < 6 {
                    1
                } else if rank == 6 {
                    2
                } else {
                    3
                };
                let t = u32::from(w.election_town);
                host.notification(w, [i as u32, 0, 0, t, 0xd, t]);
            }
            human += 1;
        }
        let local1c: i32;
        if (ebx as i32) < 0x26 {
            rec.set_u8_at(0x15, rec.u8_at(0x16));
            local1c = 0x26 - ebx as i32;
        } else {
            local1c = 0;
            rec.set_u8_at(0x15, rec.u8_at(0x16).wrapping_sub(ebx as u8).wrapping_add(0x26));
        }
        let (mut max, mut min) = (0i32, 0x0fff_ffffi32);
        for k in 0..4 {
            let al = rec.u8_at(8 + k);
            if al & 0x80 != 0 {
                continue;
            }
            town_ok(w, usize::from(al))?;
            let t = &w.towns[usize::from(al)];
            if u16::from(t.u8(0x6f1)) < mc {
                continue;
            }
            let v = t.i32(0x2d4);
            if max < v {
                max = v;
            }
            if min > v {
                min = v;
            }
        }
        if max < 0x40 {
            max = 0x40;
        }
        let off = if min.wrapping_add(1) >= max {
            (max >> 1) - 1
        } else if min < 0x20 {
            0x10
        } else {
            min >> 1
        };
        let mut vals = [0i32; 4];
        let mut sum = 0i32;
        for k in 0..4 {
            let al = rec.u8_at(8 + k);
            let v = if al & 0x80 != 0 {
                let m = usize::from(al & 0x7f);
                if m < w.merchants.len() {
                    score(w, m)
                } else {
                    // Ranura vacía (`0x00529700` con menos de cuatro candidatos deja ciudad y comerciante 0xFF, que
                    // aquí es el comerciante 0x7F): el original puntúa la memoria tras la tabla de comerciantes.
                    // Se toma como ceros (puntuación 0, sin votos), igual que el original con esa memoria a cero;
                    // con bytes arbitrarios el oráculo también da 0 en los casos contrastados.
                    0
                }
            } else {
                town_ok(w, usize::from(al))?;
                let t = &w.towns[usize::from(al)];
                if u16::from(t.u8(0x6f1)) < mc {
                    i32::from(w.vote_scale >> 2)
                } else {
                    let num = (t.i32(0x2d4).wrapping_sub(off)).wrapping_mul(i32::from(w.vote_scale));
                    num.checked_div(max.wrapping_sub(off))
                        .ok_or("división entre cero en el reparto de votos")?
                }
            };
            vals[k] = v;
            sum = sum.wrapping_add(v);
        }
        for k in 0..4 {
            let q = vals[k]
                .wrapping_mul(local1c)
                .checked_div(sum)
                .ok_or("división entre cero en el reparto de votos")?;
            rec.set_u8_at(0x10 + k, q as u8);
        }
        rec.due = rec.due.wrapping_add(0xa);
        return Ok(());
    }
    if stage < 0xfc {
        let c = usize::from(stage & 3);
        if stage < 0xc0 {
            let mine = rec.u8_at(0x10 + c);
            if (0..4).any(|j| mine < rec.u8_at(0x10 + j)) {
                rec.due = rec.due.wrapping_add(1);
                return Ok(());
            }
        }
        let left = rec.u8_at(0x10 + c);
        if left != 0 {
            rec.set_u8_at(0xc + c, rec.u8_at(0xc + c).wrapping_add(1));
            rec.set_u8_at(0x10 + c, left - 1);
        }
        rec.due = rec.due.wrapping_add(1);
        return Ok(());
    }
    // Cierre.
    for k in 0..4 {
        rec.set_u8_at(0xc + k, rec.u8_at(0xc + k).wrapping_add(rec.u8_at(0x10 + k)));
    }
    for t in w.towns.iter_mut() {
        for j in 0..4 {
            t.set_u8(0x6dc + j, 0xff);
        }
    }
    let mut best = 0usize;
    for k in 0..4 {
        if rec.u8_at(0xc + best) < rec.u8_at(0xc + k) {
            best = k;
        }
    }
    let bl = rec.u8_at(8 + best);
    let ebx: u32;
    if bl & 0x80 != 0 {
        let m = usize::from(bl & 0x7f);
        merchant_ok(w, m)?;
        w.election_town = w.merchants[m].bytes[0x19];
        ebx = u32::from(bl & 0x7f);
    } else {
        w.election_town = bl;
        town_ok(w, usize::from(bl))?;
        let owner = w.towns[usize::from(bl)].u8(0x6f1);
        ebx = if u16::from(owner) < mc {
            u32::from(w.player)
        } else {
            u32::from(owner)
        };
    }
    if ebx < u32::from(mc) {
        let m = ebx as usize;
        if w.merchants[m].u16(8) == 0 {
            host.notification(w, [ebx, 0, 0, u32::from(rec.u8_at(0x17)), 0x11, ebx]);
            let town_word = u16::from(w.election_town);
            host.winner_announcement(w, ebx == w.local_player, ebx, town_word);
        }
    }
    let old = w.player;
    w.player = ebx as u8;
    if u16::from(old) < mc {
        for t in 0..w.towns.len() {
            host.update_rank(w, usize::from(old), (t & 0xff) as u32);
        }
    }
    if u16::from(w.player) < mc {
        host.update_rank(w, usize::from(w.player), 0);
    }
    let (et, pl) = (w.election_town, w.player);
    for t in w.towns.iter_mut() {
        mark_town(t, et, pl, mc);
    }
    let cands: [u8; 4] = std::array::from_fn(|k| rec.u8_at(8 + k));
    let votes: [u8; 4] = std::array::from_fn(|k| rec.u8_at(0xc + k));
    host.post_messages(w, cands, votes, rec.u8_at(0x17), w.election_town)?;
    rec.due = rec.due.wrapping_add(0x16d00) & !0xff;
    rec.set_u8_at(0x17, w.election_town);
    Ok(())
}
