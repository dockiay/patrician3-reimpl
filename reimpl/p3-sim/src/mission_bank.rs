//! Carga privada de los guiones `.p2m` de los CPR proporcionados por el anfitrión.
use std::collections::BTreeMap;

#[derive(Clone,Debug,Default)]
pub struct MissionBank { scripts:BTreeMap<u16,Vec<u8>> }

impl MissionBank {
    /// Resuelve `[Missions]` del INI indicado. Los recursos comerciales solo
    /// residen en memoria; el último CPR suministrado tiene prioridad.
    pub fn from_archives(archives:&[&[u8]],config:&str)->Result<Self,String>{
        let parsed=archives.iter().map(|b|p3_assets::cpr::Archive::parse(b).map_err(|e|format!("CPR de misiones: {e}"))).collect::<Result<Vec<_>,_>>()?;
        let read=|path:&str|parsed.iter().rev().find_map(|a|a.find(path.as_bytes()).and_then(|e|a.read(e)));
        let ini=read(config).ok_or_else(||format!("configuración de misiones ausente: {config}"))?;
        let text=String::from_utf8_lossy(ini);let mut section=false;let mut names=BTreeMap::new();
        for raw in text.lines(){let line=raw.trim();if line.starts_with('[')&&line.ends_with(']'){section=line[1..line.len()-1].eq_ignore_ascii_case("missions");continue}if !section||line.starts_with(';'){continue}if let Some((key,value))=line.split_once('='){if let Ok(id)=key.trim().parse::<u16>(){let name=value.trim();if !name.is_empty(){names.insert(id,name.to_owned());}}}}
        let mut scripts=BTreeMap::new();for(id,name)in names{let path=format!("missions_addon/{name}");if let Some(blob)=read(&path){scripts.insert(id,blob.to_vec());}}
        Ok(Self{scripts})
    }
    pub fn get(&self,id:u16)->Option<&[u8]>{self.scripts.get(&id).map(Vec::as_slice)}
    pub fn len(&self)->usize{self.scripts.len()}
    pub fn is_empty(&self)->bool{self.scripts.is_empty()}
}
