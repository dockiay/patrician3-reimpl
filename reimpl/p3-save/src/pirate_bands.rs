//! Plan y piezas de las cinco bandas de la cola S02, conservando el resto.
//!
//! Los 16 bytes de cabecera, el plano opcional y `capacity * 8` bytes de
//! piezas siguen el serializador `pirate_band_serialize` (0x005137E0).

use std::ops::Range;

const PREFIX: usize = 0xc0;
const BAND_COUNT: usize = 5;
const PLAN_HEADER: usize = 0x64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PirateBand {
    /// Campos guardados de `band+8..+0x17`.
    pub header: [u8; 16],
    /// Cabecera serializada del plano y sus celdas, sin el marcador.
    pub plan: Option<Vec<u8>>,
    /// Todas las piezas, incluidas las libres, de ocho bytes cada una.
    pub pieces: Vec<u8>,
}

impl PirateBand {
    /// Comprueba que cabecera, plano y piezas se pueden serializar juntos.
    pub fn validate(&self)->Result<(),&'static str>{self.encode().map(|_|())}

    fn encode(&self) -> Result<Vec<u8>, &'static str> {
        let capacity=usize::from(u16::from_le_bytes([self.header[0],self.header[1]]));
        if self.pieces.len()!=capacity*8{return Err("capacidad de piezas incompatible")}
        let mut bytes=Vec::new();bytes.push(0x7f);bytes.extend_from_slice(&self.header);
        match &self.plan {
            None=>bytes.push(0),
            Some(plan)=>{if plan_size(plan)?!=plan.len(){return Err("plano de banda incompatible")};bytes.push(0x7f);bytes.extend_from_slice(plan)}
        }
        bytes.extend_from_slice(&self.pieces);Ok(bytes)
    }
}

fn plan_size(data:&[u8])->Result<usize,&'static str>{
    let header=data.get(..PLAN_HEADER).ok_or("plano de banda truncado")?;
    let rows=u32::from_le_bytes(header[0..4].try_into().unwrap());
    let columns=u32::from_le_bytes(header[4..8].try_into().unwrap());
    if rows>i32::MAX as u32||columns>i32::MAX as u32{return Err("dimensiones de plano inválidas")}
    let len=(rows as usize).checked_mul(columns as usize).and_then(|n|n.checked_add(PLAN_HEADER)).ok_or("plano de banda excesivo")?;
    if len>data.len(){return Err("celdas de banda truncadas")}Ok(len)
}

fn bands(tail:&[u8])->Result<Vec<(Range<usize>,Option<PirateBand>)>,&'static str>{
    if tail.len()<PREFIX{return Err("cola de barcos truncada")}
    let mut at=PREFIX;let mut result=Vec::with_capacity(BAND_COUNT);
    for _ in 0..BAND_COUNT {
        let start=at;let marker=*tail.get(at).ok_or("marcador de banda ausente")?;at+=1;
        if marker==0 {result.push((start..at,None));continue}
        if marker!=0x7f{return Err("marcador de banda inválido")}
        let header:[u8;16]=tail.get(at..at+16).ok_or("cabecera de banda truncada")?.try_into().unwrap();at+=16;
        let flag=*tail.get(at).ok_or("marcador de plano ausente")?;at+=1;
        let plan=match flag {
            0=>None,
            0x7f=>{let len=plan_size(tail.get(at..).ok_or("plano de banda truncado")?)?;let end=at.checked_add(len).ok_or("plano de banda excesivo")?;let plan=tail[at..end].to_vec();at=end;Some(plan)},
            _=>return Err("marcador de plano inválido"),
        };
        let capacity=usize::from(u16::from_le_bytes([header[0],header[1]]));
        let end=at.checked_add(capacity*8).ok_or("piezas de banda excesivas")?;
        let pieces=tail.get(at..end).ok_or("piezas de banda truncadas")?.to_vec();at=end;
        result.push((start..at,Some(PirateBand{header,plan,pieces})));
    }
    Ok(result)
}

/// Lee las cinco bandas, sin interpretar datos posteriores de la cola.
pub fn read(tail:&[u8])->Result<Vec<Option<PirateBand>>,&'static str>{
    Ok(bands(tail)?.into_iter().map(|(_,band)|band).collect())
}

/// Sustituye una banda entera, rechaza conflictos y conserva las demás y la
/// cola posterior. El cambio es atómico incluso si la nueva imagen es inválida.
pub fn replace(tail:&mut Vec<u8>,index:usize,before:&PirateBand,after:&PirateBand)->Result<(),&'static str>{
    let records=bands(tail)?;let (range,current)=records.get(index).ok_or("índice de banda inválido")?;
    let current=current.as_ref().ok_or("banda ausente")?;
    if current!=before&&current!=after{return Err("banda modificada por otro sistema")}
    let replacement=after.encode()?;
    tail.splice(range.clone(),replacement);
    Ok(())
}

/// Sustituye también presencia de una banda, comparando la imagen previa y
/// aceptando una segunda publicación idéntica. La cola ajena se conserva.
pub fn replace_optional(tail:&mut Vec<u8>,index:usize,before:Option<&PirateBand>,after:Option<&PirateBand>)->Result<(),&'static str>{
 let records=bands(tail)?;let (range,current)=records.get(index).ok_or("índice de banda inválido")?;
 if current.as_ref()!=before&&current.as_ref()!=after{return Err("banda modificada por otro sistema")}
 let replacement=match after{Some(b)=>b.encode()?,None=>vec![0]};tail.splice(range.clone(),replacement);Ok(())
}
