//! Recursos vivos de apertura de guarida. Las entradas ausentes se rechazan:
//! no se sustituyen con dimensiones, nombres ni textos inventados.
use p3_core::ai::settlement::SettlementLocation;
use p3_core::ships::town_attack_mask::TownProjection;
use p3_save::pirate_bands::PirateBand;

/// `0x587ED0` carga DownVector en Class22+0x4d0 y TileSize en +0x510.
pub fn projection_from_iso(data: &[u8]) -> Result<TownProjection, String> {
    let mut section = false;
    let (mut size, mut down) = (None, None);
    for raw in data.split(|&b| b == b'\n' || b == b'\r') {
        let text = std::str::from_utf8(raw).map_err(|_| "iso.ini no es ASCII")?;
        let line = text.split(';').next().unwrap_or("").trim();
        if line.starts_with('[') && line.ends_with(']') {
            section = line[1..line.len()-1].trim().eq_ignore_ascii_case("LIGHTNINGVIEW0");
        } else if section {
            if let Some((key, value)) = line.split_once('=') {
                let target = if key.trim().eq_ignore_ascii_case("TileSize") { &mut size }
                    else if key.trim().eq_ignore_ascii_case("DownVector") { &mut down }
                    else { continue };
                let v = value.split_whitespace().map(str::parse::<i32>)
                    .collect::<Result<Vec<_>, _>>().map_err(|_| "vector de iso.ini inválido")?;
                if v.len()!=3 || target.replace([v[0],v[1],v[2]]).is_some() {
                    return Err("vector duplicado o sin tres componentes".into());
                }
            }
        }
    }
    let size = size.ok_or("falta TileSize")?;
    let down = down.ok_or("falta DownVector")?;
    if size[0]<=0 || size[1]<=0 { return Err("tamaño de baldosa no positivo".into()); }
    Ok(TownProjection { width:size[0], height:size[1], x_offset:down[0], y_offset:down[1] })
}

/// El último CPR prevalece; un recurso inválido no cae a otra versión.
pub fn projection_from_archives(archives:&[&[u8]]) -> Result<Option<TownProjection>,String> {
    for bytes in archives.iter().rev() {
        let archive=p3_assets::cpr::Archive::parse(bytes).map_err(|e| format!("CPR: {e}"))?;
        if let Some(entry)=archive.find(b"scripts/iso.ini") {
            return projection_from_iso(archive.read(entry).ok_or("iso.ini comprimido o truncado")?).map(Some);
        }
    }
    Ok(None)
}

/// Coordenadas reales en la tabla de 40 ciudades, indexada por band+0xE.
pub fn position(band:&PirateBand, locations:&[SettlementLocation;40]) -> Result<(u16,u16),String> {
    let row=locations.get(usize::from(band.header[6])).ok_or("destino de banda fuera de la tabla")?;
    Ok((u16::try_from(row.x).map_err(|_| "coordenada x fuera de rango")?,
        u16::try_from(row.y).map_err(|_| "coordenada y fuera de rango")?))
}

/// `0x515AE0`: recorre la cadena, toma el primer capitán de rango > 0x20 y,
/// si no existe, usa 45 y el byte bajo del recuento real de apellidos comunes.
pub fn portrait(band:&PirateBand, ships:&[u8], captains:&[u8], ordinary_family_count:u16) -> Result<(bool,u8,u8),String> {
    if ships.len()%0x180!=0 || captains.len()%16!=0 || ships.len()/0x180>u16::MAX as usize || captains.len()/16>u16::MAX as usize {
        return Err("pools de barcos o capitanes incompatibles".into());
    }
    let mut seen=vec![false;ships.len()/0x180];
    let mut current=u16::from_le_bytes([band.header[4],band.header[5]]) as usize;
    while current < seen.len() {
        if seen[current] { return Err("cadena de barcos piratas cíclica".into()); }
        seen[current]=true;
        let ship=&ships[current*0x180..][..0x180];
        let captain=u16::from_le_bytes([ship[0x42],ship[0x43]]) as usize;
        if captain < captains.len()/16 {
            let c=&captains[captain*16..][..16];
            if c[8]>0x20 { return Ok((true,c[2],c[3])); }
        }
        current=u16::from_le_bytes([ship[4],ship[5]]) as usize;
    }
    Ok((false,0x2d,ordinary_family_count as u8))
}

/// Los índices fuera de rango usan la primera entrada, igual que 0x512A60/AE0.
pub fn names<'a>(given:u8, family:u8, given_bank:&'a [Vec<Vec<u8>>;2], family_bank:&'a [Vec<u8>]) -> Result<(&'a [u8], &'a [u8]),String> {
    let bank=&given_bank[usize::from(given>>7)];
    let first=bank.get(usize::from(given&0x7f)).or_else(||bank.first()).ok_or("banco de nombres vacío")?;
    let last=family_bank.get(usize::from(family)).or_else(||family_bank.first()).ok_or("banco de apellidos vacío")?;
    Ok((first,last))
}

#[derive(Debug,PartialEq,Eq)]
pub struct HideoutTexts { pub victory:Vec<u8>, pub human:Vec<u8>, pub reward:Vec<u8>, pub own_separator:Vec<u8>, pub enemy_separator:Vec<u8> }

/// Banco de la imagen ejecutable validada por el anfitrión; no usa cadenas fijas.
pub fn texts(mut get:impl FnMut(u32)->Option<Vec<u8>>) -> Result<HideoutTexts,String> {
    let mut require=|slot| get(slot).ok_or_else(||format!("texto ausente en {slot:08X}"));
    Ok(HideoutTexts { victory:require(0x6aa1a4)?, human:require(0x6aa1e4)?, reward:require(0x6a1dac)?, own_separator:require(0x6a2260)?, enemy_separator:require(0x6a2268)? })
}
