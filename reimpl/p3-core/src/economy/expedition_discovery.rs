//! `grant_first_discovery_renown` (0x005319D0).
//! El llamante publica de una vez el comerciante, la bandera y la carta global.
use crate::ai::reputation::{Merchant, RepEnv};

#[derive(Clone, Debug)]
pub struct FirstDiscovery {
    pub merchant: Merchant,
    pub world_flag: u8,
    pub letter: [u8; 16],
    pub notify: [u32; 6],
    pub event: [u32; 3],
}

/// `large_renown` es el i16 original de 0x00672DD2, leído del ejecutable.
/// El acceso fuera del pool se rechaza porque el original lo desreferencia.
pub fn prepare(
    world_flag: u8,
    owner: u8,
    merchants: &[Merchant],
    merchant_count: u16,
    player: u32,
    large_renown: i16,
) -> Result<Option<FirstDiscovery>, String> {
    if world_flag != 0 {
        return Ok(None);
    }
    if u16::from(owner) >= merchant_count {
        return Err("descubridor fuera de comerciantes activos".into());
    }
    let mut merchant = merchants.get(usize::from(owner)).ok_or("descubridor ausente")?.clone();
    if merchant.bytes.len() != 0x650 {
        return Err("comerciante de descubrimiento truncado".into());
    }
    let env = RepEnv {
        town_count: 0,
        towns: &[],
        renown: [large_renown, 0, 0],
        byte_52c: 0,
    };
    merchant.add_renown_and_social_all_towns_large(&env);
    let home = merchant.bytes[0x19];
    let rank = merchant.bytes[0x39c + usize::from(home)];
    let title = merchant.get_merchant_epithet_index();
    let mut letter = [0u8; 16];
    letter[1] = 0x80;
    letter[2] = 0xff;
    letter[3] = 0xff;
    letter[4] = 0x56;
    letter[5] = home;
    letter[8..10].copy_from_slice(&u16::from(owner).to_le_bytes());
    letter[10..12].copy_from_slice(&u16::from(title).to_le_bytes());
    letter[12..14].copy_from_slice(&u16::from(rank).to_le_bytes());
    Ok(Some(FirstDiscovery {
        merchant,
        world_flag: 1,
        letter,
        notify: [player, 0, 0, u32::from(home), 15, u32::from(owner)],
        event: [7, u32::from(owner), 0],
    }))
}
