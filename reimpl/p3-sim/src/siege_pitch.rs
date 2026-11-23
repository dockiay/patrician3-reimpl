//! Demanda de brea de la escena de asedio, consultada por el mercado urbano.

/// `0x00636920`, con la escena y los registros de edificios ya reconstruidos.
pub fn demand(image: &[u8], records: &[u8]) -> Result<i32, &'static str> {
    if image.len() < 0x650 || records.len() % 8 != 0 { return Err("escena o edificios truncados"); }
    let word=|at:usize|u16::from_le_bytes([image[at],image[at+1]]);
    if image[0x78]!=0 || word(0x42)!=0 { return Ok(0); }
    let defenders=u32::from(word(0x236))+u32::from(word(0x23a))+u32::from(word(0x23e));
    let units=defenders.min(u32::from(word(0x21e)))*40;
    let mut building=0;
    if u32::from_le_bytes(image[4..8].try_into().unwrap())!=0 {
        let offset=0x244+4*usize::from(image[0x6d]);
        let raw=image.get(offset..offset+4).ok_or("objetivo de asedio fuera de la escena")?;
        let id=u32::from_le_bytes(raw.try_into().unwrap()) as usize;
        if id<records.len()/8 && matches!(records[id*8+3],0x34|0x35) { building=400; }
    }
    Ok((units+building) as i32)
}
