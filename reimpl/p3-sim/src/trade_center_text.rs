//! Texto comercial de 52C4D2..52C68D para el informe 0x63.
use p3_core::economy::{cformat::{self,FArg},trade_center_convoy::Report};
use sha2::{Digest,Sha256};

#[derive(Clone,Debug)]
pub struct TradeReportTexts { fragments:[Vec<u8>;6], towns:Vec<Vec<u8>>, wares:Vec<Vec<u8>> }
impl TradeReportTexts {
 pub fn from_english_exe(bytes:&[u8])->Result<Self,String>{
  if format!("{:x}",Sha256::digest(bytes))!="fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece"{return Err("ejecutable inglés no reconocido para informes comerciales".into())}
  let image=p3_assets::exe::PeImage::parse(bytes).map_err(str::to_owned)?;let mut fragments:[Vec<u8>;6]=Default::default();
  for(i,slot)in [0x6b95f4u32,0x6b95f8,0x6b95fc,0x6b9600,0x6b9604,0x6b9614].into_iter().enumerate(){fragments[i]=image.pointed_cstr(slot).map_err(str::to_owned)?.to_vec()}
  let towns=(0..30).map(|i|image.pointed_cstr(0x6b1f20+4*i).map(|v|v.to_vec()).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
  let wares=(0..24).map(|i|image.pointed_cstr(0x6c0bb0+4*i).map(|v|v.to_vec()).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
  Ok(Self{fragments,towns,wares})
 }
}

/// Añade el informe al CString ya existente sobre una copia. Conserva la
/// selección original de unidad para compras: se usa la última mercancía
/// considerada para ambas ranuras. Las cantidades que causarían división por
/// cero en el EXE se rechazan sin publicar texto parcial.
pub fn append_report(texts:&TradeReportTexts,units:&[u8;24],center:&[u8],waypoint:u16,report:&Report,before:&[u8])->Result<Vec<u8>,String>{
 if center.len()!=44{return Err("centro de informe truncado".into())}
 let mut out=before.to_vec();
 if !report.sold.iter().chain(&report.bought).any(|&n|n!=0){return Ok(out)}
 if center[5]<2{out.extend_from_slice(&texts.fragments[0]);out.extend_from_slice(texts.towns.get(usize::from(center[4])).ok_or("nombre de centro no disponible")?);out.extend_from_slice(&texts.fragments[1+usize::from(waypoint&1)])}
 else{out.extend_from_slice(&texts.fragments[5]);out.extend_from_slice(&texts.fragments[1])}
 let make=|qty:i32,total:i32,ware:usize,unit:u8,fmt:&[u8]|->Result<Vec<u8>,String>{
  let lots=qty/(if unit==0{2000}else{200});if lots<=0{return Err("informe comercial con cantidad no representable".into())}
  let price=lots.wrapping_shr(1).wrapping_add(total).checked_div(lots).ok_or("división de informe inválida")?;
  let name=texts.wares.get(ware).ok_or("nombre de mercancía ausente")?;
  cformat::format(fmt,&[FArg::Int(lots as u32),FArg::Int(u32::from(if unit==0{b'L'}else{b'B'})),FArg::Str(name.clone()),FArg::Int(price as u32),FArg::Int(total as u32)])
 };
 for slot in 0..2{let qty=report.sold[slot];if qty>0{let ware=usize::from(center[2+slot]);let unit=*units.get(ware).ok_or("mercancía vendida inválida")?;out.extend_from_slice(&make(qty,report.revenue[slot],ware,unit,&texts.fragments[3])?)}}
 if report.bought.iter().any(|&v|v>0){let last=usize::from(report.last_ware.ok_or("última mercancía comprada ausente")?);let unit=*units.get(last).ok_or("unidad de compra inválida")?;
  for slot in 0..2{let qty=report.bought[slot];if qty>0{let lots=qty/(if unit==0{2000}else{200});if lots>0{out.extend_from_slice(&make(qty,report.cost[slot],usize::from(center[slot]),unit,&texts.fragments[4])?)}}}
 }
 Ok(out)
}
