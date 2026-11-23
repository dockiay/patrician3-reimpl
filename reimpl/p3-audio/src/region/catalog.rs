//! Parte de audio del constructor GameScreen, 0x004274FC..0x00427902.
//! La memoria CString se sustituye por datos Rust; no se construye la interfaz.
use super::{Catalog, ScreenAudio, Track};
use crate::banks::BankResources;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnedTrack {
    pub file: String,
    pub volume: u8,
}

impl OwnedTrack {
    fn borrow(&self) -> Track<'_> {
        Track {
            file: &self.file,
            volume: self.volume,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegionalBank {
    pub region: Vec<OwnedTrack>,
    pub sea: Vec<OwnedTrack>,
    pub town_atmosphere: [OwnedTrack; 3],
    pub sea_atmosphere: [OwnedTrack; 2],
    pub battle: OwnedTrack,
}

/// Crear una vez después de cargar el banco, conservar mientras se selecciona.
pub struct RegionalRefs<'a> {
    region: Vec<Track<'a>>,
    sea: Vec<Track<'a>>,
    town_atmosphere: [Track<'a>; 3],
    sea_atmosphere: [Track<'a>; 2],
    battle: Track<'a>,
}

impl RegionalBank {
    pub fn references(&self) -> RegionalRefs<'_> {
        RegionalRefs {
            region: self.region.iter().map(OwnedTrack::borrow).collect(),
            sea: self.sea.iter().map(OwnedTrack::borrow).collect(),
            town_atmosphere: self.town_atmosphere.each_ref().map(OwnedTrack::borrow),
            sea_atmosphere: self.sea_atmosphere.each_ref().map(OwnedTrack::borrow),
            battle: self.battle.borrow(),
        }
    }
}

impl<'a> RegionalRefs<'a> {
    /// La tabla de música por ciudad pertenece al mundo, no a sound.ini.
    pub fn catalog<'b>(&'b self, town: &'b [Track<'b>]) -> Catalog<'b>
    where
        'a: 'b,
    {
        Catalog {
            region: &self.region,
            sea: &self.sea,
            town_atmosphere: self.town_atmosphere,
            sea_atmosphere: self.sea_atmosphere,
            battle: self.battle,
            town,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CatalogError {
    NegativeCount(i32),
    TitleLimit { count: usize, limit: usize },
    Capacity,
}

const FILE: &str = "./scripts/sound.ini";

fn track(resources: &mut impl BankResources, section: &str, index: usize) -> OwnedTrack {
    let file = resources
        .text(FILE, section, &format!("Titel{index}"), 4096)
        .unwrap_or_default();
    let volume = resources.integer(FILE, section, &format!("Loudness{index}"), 100) as u8;
    OwnedTrack { file, volume }
}

fn reserve(count: usize) -> Result<Vec<OwnedTrack>, CatalogError> {
    let mut tracks = Vec::new();
    tracks.try_reserve_exact(count).map_err(|_| CatalogError::Capacity)?;
    Ok(tracks)
}

impl ScreenAudio {
    /// Tramo acotado del constructor, no otra función completa del catálogo.
    /// region_limit acota datos malformados antes de reservar o iterar. Un fallo
    /// conserva los siete recuentos ya leídos. No se exige suma(recuentos)==N.
    /// Los punteros a datos trasladados a RegionalBank se normalizan a cero.
    pub fn load_regional_catalog(
        &mut self,
        resources: &mut impl BankResources,
        region_limit: usize,
    ) -> Result<RegionalBank, CatalogError> {
        let count = resources.integer(FILE, "Region", "Titel", 0);
        for i in 0..7 {
            let value = resources.integer(FILE, "Region", &format!("Region{i}"), 0);
            self.set_byte(0x2889 + i, value as u8);
        }
        let count = usize::try_from(count).map_err(|_| CatalogError::NegativeCount(count))?;
        if count > region_limit {
            return Err(CatalogError::TitleLimit {
                count,
                limit: region_limit,
            });
        }
        let mut region = reserve(count)?;
        self.put(0x287c, 0);
        self.put(0x2880, 0);
        for i in 0..count {
            region.push(track(resources, "Region", i));
        }
        let town_atmosphere = std::array::from_fn(|i| {
            let t = track(resources, "TownMapAtmo", i);
            self.put(0x2890 + 4 * i, 0);
            self.set_byte(0x289c + i, t.volume);
            t
        });
        let sea_count = resources.integer(FILE, "SeaMap", "Titel", 0) as u8;
        self.set_byte(0x28c8, sea_count);
        let mut sea = reserve(sea_count as usize)?;
        self.put(0x28a0, 0);
        self.put(0x28a4, 0);
        for i in 0..sea_count as usize {
            sea.push(track(resources, "SeaMap", i));
        }
        let sea_atmosphere = std::array::from_fn(|i| {
            let t = track(resources, "SeaMapAtmo", i);
            self.put(0x28a8 + 4 * i, 0);
            self.set_byte(0x28b0 + i, t.volume);
            t
        });
        let battle = track(resources, "SeeschlachtAtmo", 0);
        self.put(0x28bc, 0);
        self.set_byte(0x28c0, battle.volume);
        Ok(RegionalBank {
            region,
            sea,
            town_atmosphere,
            sea_atmosphere,
            battle,
        })
    }
}
