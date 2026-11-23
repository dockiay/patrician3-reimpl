//! Lectura acotada de datos PE32. No ejecuta el archivo ni carga bibliotecas.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// Traducciones del idioma elegido por dirección del literal del EXE inglés (paquete `scripts/gen_language_pack.py`). Los textos que el
/// simulador lee del EXE inglés (cartas, batallas, misiones, informes de comercio) consultan aquí antes de devolver el literal inglés.
static TRANSLATIONS: OnceLock<RwLock<HashMap<u32, &'static [u8]>>> = OnceLock::new();

/// Fija las traducciones (las cadenas se conservan toda la ejecución).
pub fn set_translations(map: HashMap<u32, Vec<u8>>) {
    let map: HashMap<u32, &'static [u8]> = map.into_iter().map(|(k, v)| (k, &*Box::leak(v.into_boxed_slice()))).collect();
    *TRANSLATIONS.get_or_init(Default::default).write().unwrap() = map;
}

/// Traducción del literal inglés que empieza en `address`, si la hay.
pub fn translation(address: u32) -> Option<&'static [u8]> {
    TRANSLATIONS.get()?.read().ok()?.get(&address).copied()
}

#[derive(Clone,Copy)]
struct Section { virtual_address:u32, raw_start:usize, raw_size:usize }
pub struct PeImage<'a> { data:&'a [u8], base:u32, sections:Vec<Section> }

fn u16_at(data:&[u8],at:usize)->Result<u16,&'static str>{Ok(u16::from_le_bytes(data.get(at..at.checked_add(2).ok_or("desbordamiento PE")?).ok_or("PE truncado")?.try_into().unwrap()))}
fn u32_at(data:&[u8],at:usize)->Result<u32,&'static str>{Ok(u32::from_le_bytes(data.get(at..at.checked_add(4).ok_or("desbordamiento PE")?).ok_or("PE truncado")?.try_into().unwrap()))}

impl<'a> PeImage<'a>{
 pub fn parse(data:&'a [u8])->Result<Self,&'static str>{
  if data.get(..2)!=Some(b"MZ"){return Err("cabecera DOS inválida")}
  let pe=u32_at(data,0x3c)? as usize;if data.get(pe..pe.checked_add(4).ok_or("desbordamiento PE")?)!=Some(b"PE\0\0"){return Err("cabecera PE inválida")}
  let file=pe.checked_add(4).ok_or("desbordamiento PE")?;if u16_at(data,file)?!=0x14c{return Err("arquitectura PE distinta de x86")}
  let n=usize::from(u16_at(data,file+2)?);let opt_len=usize::from(u16_at(data,file+16)?);let opt=file.checked_add(20).ok_or("desbordamiento PE")?;if opt_len<32||u16_at(data,opt)?!=0x10b{return Err("PE32 inválido")}
  let base=u32_at(data,opt+28)?;let table=opt.checked_add(opt_len).ok_or("desbordamiento PE")?;let end=table.checked_add(n.checked_mul(40).ok_or("demasiadas secciones")?).ok_or("desbordamiento PE")?;if end>data.len(){return Err("secciones truncadas")}
  let mut sections=Vec::with_capacity(n);for i in 0..n{let at=table+i*40;let va=u32_at(data,at+12)?;let size=u32_at(data,at+16)? as usize;let raw=u32_at(data,at+20)? as usize;if raw.checked_add(size).is_none_or(|end|end>data.len()){return Err("sección fuera del archivo")};if u64::from(va)+size as u64>u64::from(u32::MAX)+1{return Err("sección fuera de memoria")};sections.push(Section{virtual_address:va,raw_start:raw,raw_size:size})}
  Ok(Self{data,base,sections})
 }
 fn tail(&self,address:u32)->Result<&'a [u8],&'static str>{let rva=address.checked_sub(self.base).ok_or("dirección anterior a la imagen")?;let section=self.sections.iter().find(|s|rva>=s.virtual_address&&u64::from(rva-s.virtual_address)<s.raw_size as u64).ok_or("dirección sin datos de archivo")?;let off=(rva-section.virtual_address)as usize;Ok(&self.data[section.raw_start+off..section.raw_start+section.raw_size])}
 /// Copia acotada de `len` bytes desde una dirección virtual (dentro de una sola sección).
 pub fn bytes_at(&self,address:u32,len:usize)->Result<&'a [u8],&'static str>{self.tail(address)?.get(..len).ok_or("rango fuera de la sección")}
 /// Byte de una dirección virtual de la imagen.
 pub fn byte_at(&self,address:u32)->Result<u8,&'static str>{self.tail(address)?.first().copied().ok_or("dirección fuera de la imagen")}
 /// Interpreta el dword de una dirección virtual como puntero a cadena NUL.
 pub fn pointed_cstr(&self,slot:u32)->Result<&'a [u8],&'static str>{let bytes=self.tail(slot)?;let ptr=u32_at(bytes,0)?;if let Some(t)=translation(ptr){return Ok(t)}let text=self.tail(ptr)?;let end=text.iter().take(65536).position(|&b|b==0).ok_or("cadena sin terminador")?;Ok(&text[..end])}
}
