//! Tabla de nombres urbanos cargada del CPR, con los bytes de la edición elegida.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct TownNameBank { entries:Vec<Vec<u8>> }
impl TownNameBank {
    /// `0x005125E0` separa nombres con bytes de control; los espacios forman parte del nombre.
    pub fn from_text(data:&[u8])->Result<Self,String>{
        let entries=data.split(|&b|b<0x20).filter(|v|!v.is_empty()).map(<[u8]>::to_vec).collect::<Vec<_>>();
        if entries.len()<80{return Err("banco de nombres urbanos incompleto".into())}Ok(Self{entries})
    }
    pub fn from_archives(archives:&[&[u8]])->Result<Option<Self>,String>{
        for data in archives.iter().rev(){let archive=p3_assets::cpr::Archive::parse(data).map_err(|e|format!("CPR: {e}"))?;if let Some(entry)=archive.find(b"scripts/NamenStaedte_eng.txt"){return Ok(Some(Self::from_text(archive.read(entry).ok_or("nombres urbanos no disponibles")?)?))}}
        Ok(None)
    }
    /// Los 40 nombres de mapa (`0x006DDAA0`), que consulta el comando de guion 0x65.
    pub fn map_names(&self)->Vec<Vec<u8>>{self.entries[..40].to_vec()}
    /// `0x005127E0`: dos tablas, nombre `map_id` y texto `40 + map_id`.
    /// Tercera tabla (`0x006DD8C0`): texto `80 + map_id`, si el archivo la trae.
    pub fn third(&self,map_id:u8)->Option<&[u8]>{let i=usize::from(map_id);if i>=40{return None}self.entries.get(80+i).map(Vec::as_slice)}
    pub fn for_map(&self,map_id:u8)->Option<(&[u8],&[u8])>{let i=usize::from(map_id);if i>=40{return None}Some((&self.entries[i],&self.entries[40+i]))}
}
impl crate::Sim {
    /// Tercer texto de una ciudad (`0x6DD8C0`).
    pub fn town_text3(&self,city:usize)->Option<Vec<u8>>{let id=*self.host.expedition_town_ids.get(city)?;self.host.town_names.as_ref()?.third(id).map(<[u8]>::to_vec)}
    /// Devuelve los dos textos de una ciudad usando su posición de mapa.
    pub fn town_names(&self,city:usize)->Option<(&[u8],&[u8])>{let id=*self.host.expedition_town_ids.get(city)?;self.host.town_names.as_ref()?.for_map(id)}
}
