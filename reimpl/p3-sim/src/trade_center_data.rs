//! Tablas del ejecutable inglés usadas por el comercio de expediciones.
use p3_core::economy::trade_center_prices::Tables;
use sha2::{Digest,Sha256};

#[derive(Clone,Debug)]
pub struct TradeCenterData {
    pub base:[f32;24],pub units:[u8;24],pub sell_caps:[i32;4],pub buy_caps:[i32;4],
    pub locations:[(u16,u16);127],pub renown:i16,
}
impl TradeCenterData {
    /// Los bytes se leen de direcciones virtuales PE, sin cargar DLL ni ejecutar
    /// el archivo. Se conserva incluso el solapamiento de las tablas originales;
    /// los cálculos aceptan únicamente los tipos contrastados 0..=2.
    pub fn from_english_exe(bytes:&[u8])->Result<Self,String>{
        if format!("{:x}",Sha256::digest(bytes))!="fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece"{return Err("ejecutable inglés no reconocido para centros comerciales".into())}
        let image=p3_assets::exe::PeImage::parse(bytes).map_err(str::to_owned)?;
        let read=|at:u32|->Result<u32,String>{let mut v=[0u8;4];for(i,b)in v.iter_mut().enumerate(){*b=image.byte_at(at+i as u32).map_err(str::to_owned)?}Ok(u32::from_le_bytes(v))};
        let mut base=[0f32;24];let mut units=[0u8;24];for i in 0..24{base[i]=f32::from_bits(read(0x673a18+4*i as u32)?);units[i]=image.byte_at(0x672c14+i as u32).map_err(str::to_owned)?;}
        let mut sell_caps=[0i32;4];let mut buy_caps=[0i32;4];for i in 0..4{sell_caps[i]=read(0x673984+4*i as u32)? as i32;buy_caps[i]=read(0x673990+4*i as u32)? as i32;}
        let mut locations=[(0,0);127];for(i,p)in locations.iter_mut().enumerate(){let v=read(0x673858+4*i as u32)?;*p=(v as u16,(v>>16)as u16)}
        let renown=read(0x672dd4)? as u16 as i16;
        Ok(Self{base,units,sell_caps,buy_caps,locations,renown})
    }
    pub fn price_tables(&self)->Tables<'_>{Tables{base:&self.base,units:&self.units,sell_caps:&self.sell_caps,buy_caps:&self.buy_caps}}
}
