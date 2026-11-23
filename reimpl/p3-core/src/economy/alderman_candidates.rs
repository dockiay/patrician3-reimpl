//! `collect_alderman_candidates` (0x00529700): los cuatro candidatos a alcalde y sus datos de presentación.
use crate::ai::reputation::Merchant;
use crate::town::Town;

/// Estado de partida que lee la función.
pub struct Candidates<'a> {
    /// `0x006DE4B0` ciudades.
    pub towns: &'a [Town],
    pub merchants: &'a [Merchant],
    /// Byte `0x006DE52D`: comerciante humano.
    pub player: u8,
    /// Byte `0x006DE52E`: ciudad que elige alcalde.
    pub election_town: u8,
}

/// Salida: seis tablas de cuatro bytes (`+0` ciudad, `+4`, `+8`, `+0xC`, `+0x10`, `+0x14` comerciante).
pub fn collect(c: &Candidates) -> [u8; 24] {
    let town_count = c.towns.len() as u32;
    let mc = c.merchants.len() as i32;
    let town_of = |i: usize| c.towns.get(i).cloned().unwrap_or_default();
    let score = |m: &Merchant| i32::from(m.calc_total_reputation_score(town_count as i32, c.towns) as i16);
    // Cinco ranuras: comerciante, ciudad, puntuación, desempate.
    let mut a = [0xffi32; 5];
    let mut b = [0xffi32; 5];
    let mut s = [0i32; 5];
    let mut d = [0i32; 5];
    if u32::from(c.election_town) < town_count {
        b[0] = i32::from(c.election_town);
        if i32::from(c.player) < mc {
            s[0] = score(&c.merchants[usize::from(c.player)]);
            a[0] = i32::from(c.player);
        } else {
            s[0] = 0xc8;
            a[0] = 0xff;
        }
        d[0] = 0x00ff_ffff;
    }
    let mut edi = 0u32;
    if town_count > 0 {
        for i in 0..town_count {
            let t = &c.towns[i as usize];
            if t.u32(0x2c8) & 0x40 == 0x40 && i != u32::from(c.election_town) && i32::from(t.u8(0x6f1)) >= mc {
                let key = t.i32(0x2d4);
                for k in (0..4usize).rev() {
                    if s[k] < 0xc8 || (s[k] == 0xc8 && key > d[k]) {
                        b[k + 1] = b[k];
                        s[k + 1] = s[k];
                        d[k + 1] = d[k];
                        a[k + 1] = a[k];
                        b[k] = i as i32;
                        s[k] = 0xc8;
                        d[k] = key;
                        a[k] = 0xff;
                    } else {
                        break;
                    }
                }
            }
            edi = i + 1;
        }
    }
    for i in 0..mc as usize {
        if i == usize::from(c.player) {
            continue;
        }
        let m = &c.merchants[i];
        let home = u32::from(m.bytes[0x19]);
        edi = home;
        if town_of(home as usize).u32(0x2c8) & 0x40 == 0x40 && m.bytes[0x37] >= 4 {
            let v = score(m);
            for k in (0..4usize).rev() {
                if v > s[k] {
                    b[k + 1] = b[k];
                    s[k + 1] = s[k];
                    d[k + 1] = d[k];
                    a[k + 1] = a[k];
                    b[k] = home as i32;
                    s[k] = v;
                    d[k] = 0;
                    a[k] = i as i32;
                } else {
                    break;
                }
            }
        }
    }
    let mut out = [0u8; 24];
    for i in 0..4 {
        out[i] = b[i] as u8;
        let town = town_of(b[i] as usize);
        if a[i] < mc {
            let m = &c.merchants[a[i] as usize];
            out[4 + i] = m.get_merchant_epithet_index();
            out[8 + i] = m.bytes[0x39c + usize::from(m.bytes[0x19])];
            out[0xc + i] = m.bytes[0x1c];
            out[0x10 + i] = m.bytes[0x1d];
        } else {
            let mut dl = town.u8(0x6f1);
            if i32::from(dl) < mc {
                dl = c.player;
            }
            let k = (!u32::from(dl) & 3) as usize;
            out[4 + i] = town.u8(0x6e0 + k);
            out[8 + i] = u8::from(u32::from(c.election_town) == edi) + 6;
            out[0xc + i] = town.u8(0x6d4 + k);
            out[0x10 + i] = town.u8(0x6d8 + k);
        }
        out[0x14 + i] = a[i] as u8;
    }
    out
}
