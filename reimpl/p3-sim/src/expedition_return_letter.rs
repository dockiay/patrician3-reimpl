//! Carta extensa 0x63 al regresar de una expedición (4E70E0..4E79A3).
//! El original no suma las comas de las enumeraciones al tamaño serializado;
//! se conserva ese comportamiento, incluida la posible truncación del final.
use std::collections::BTreeMap;
use sha2::{Digest, Sha256};

const SLOTS: &[u32] = &[
    0x6b95e0,0x6b95e4,0x6b95e8,0x6b95ec,0x6b95f0,0x6b9608,0x6b960c,0x6b9610,
    0x6b9618,0x6b961c,0x6b9620,0x6b9624,0x6b9628,0x6b962c,0x6b9630,
    0x6b1750,0x6b1754,0x6b1758,0x6b175c,0x6b1760,0x6b1784,0x6b1788,0x6b178c,
];

#[derive(Clone, Debug)]
pub struct ExpeditionLetterTexts { cells:BTreeMap<u32,Vec<u8>>, names:Vec<Vec<u8>>, indigenous_suffix:Option<Vec<u8>> }
impl ExpeditionLetterTexts {
    pub fn from_english_exe(bytes:&[u8])->Result<Self,String>{
        if format!("{:x}",Sha256::digest(bytes))!="fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece" {return Err("ejecutable inglés no reconocido para carta de expedición".into())}
        let image=p3_assets::exe::PeImage::parse(bytes).map_err(str::to_owned)?;
        let mut cells=BTreeMap::new();
        for &slot in SLOTS {cells.insert(slot,image.pointed_cstr(slot).map_err(|e|format!("texto {slot:#x}: {e}"))?.to_vec());}
        let names=(0..30).map(|i|image.pointed_cstr(0x6b1f20+4*i).map(|s|s.to_vec()).map_err(str::to_owned)).collect::<Result<Vec<_>,_>>()?;
        // [6B9634] apunta a una zona virtual inicializada durante la sesión,
        // no a texto del archivo PE. Debe suministrarla el contexto vivo.
        Ok(Self{cells,names,indigenous_suffix:None})
    }
    pub fn set_indigenous_suffix(&mut self,text:Vec<u8>){self.indigenous_suffix=Some(text)}
    fn text(&self,slot:u32)->&[u8]{&self.cells[&slot]}
}

#[derive(Clone,Copy)]
pub enum Incident { None, Storm, Attack{stolen:u32} }

/// Entradas calculadas por las etapas anteriores y por la finalización del
/// convoy; `None` en discovered representa una aldea sin nombre europeo.
pub struct ExpeditionLetter<'a> {
    pub ship:&'a [u8], pub town:&'a [u8], pub discovered:&'a [Option<u8>],
    pub unmatched:u8, pub sales:&'a [u8], pub contracts:&'a [u8], pub incident:Incident,
    pub captain:Option<(&'a [u8],&'a [u8])>, pub initial_money:u32, pub returned_money:u32,
}

/// Devuelve exactamente los bytes declarados por la carta, sin normalizar las
/// longitudes erróneas del original. No modifica el pool de mensajes.
pub fn build(texts:&ExpeditionLetterTexts,input:&ExpeditionLetter<'_>)->Result<Vec<u8>,String>{
    if input.discovered.len()>4{return Err("más de cuatro descubrimientos".into())}
    let mut out=Vec::new();let mut declared=0usize;
    fn append(out:&mut Vec<u8>,size:&mut usize,s:&[u8]){out.extend_from_slice(s);*size+=s.len()}
    fn cstr(s:&[u8])->&[u8]{&s[..s.iter().position(|&v|v==0).unwrap_or(s.len())]}
    macro_rules! add {($s:expr)=>{append(&mut out,&mut declared,$s)}}
    macro_rules! text {($slot:expr)=>{add!(texts.text($slot))}}
    text!(0x6b95e0);add!(cstr(input.ship));text!(0x6b95e4);add!(cstr(input.town));text!(0x6b95e8);
    let european=input.discovered.iter().flatten().map(|&i|texts.names.get(usize::from(i)).ok_or("centro europeo fuera de rango")).collect::<Result<Vec<_>,_>>()?;
    if input.discovered.is_empty(){if input.unmatched!=0{text!(0x6b9618)}}else{
        match european.len(){
            4=>{text!(0x6b961c);add!(&european[0]);out.extend_from_slice(b", ");add!(&european[1]);out.extend_from_slice(b", ");add!(&european[2]);text!(0x6b9620);add!(&european[3]);text!(0x6b1760);text!(0x6b1750)},
            3=>{text!(0x6b961c);add!(&european[0]);out.extend_from_slice(b", ");add!(&european[1]);text!(0x6b9620);add!(&european[2]);text!(0x6b175c);text!(0x6b1750)},
            2=>{text!(0x6b961c);add!(&european[0]);text!(0x6b9620);add!(&european[1]);text!(0x6b9624)},
            1=>{text!(0x6b95ec);add!(&european[0]);text!(0x6b95f0)},
            0=>{},_=>unreachable!(),
        }
        let indigenous=input.discovered.len()-european.len();
        if indigenous>0{
            text!(0x6b9628);
            match indigenous {1=>text!(0x6b962c),2=>text!(0x6b9630),3|4=>{text!(0x6b1754);text!(0x6b1750+4*indigenous as u32);text!(0x6b1758)},_=>unreachable!()}
            add!(cstr(texts.indigenous_suffix.as_deref().ok_or("texto dinámico de aldeas ausente")?));
        }
    }
    add!(input.sales);add!(input.contracts);
    match input.incident{
        Incident::None=>{},Incident::Storm=>text!(0x6b1784),Incident::Attack{stolen:0}=>text!(0x6b1788),
        Incident::Attack{stolen}=>{let formatted=p3_core::economy::cformat::format(texts.text(0x6b178c),&[p3_core::economy::cformat::FArg::Int(stolen)])?;add!(&formatted)},
    }
    if input.initial_money!=0||input.returned_money!=0{
        text!(0x6b9608);
        if let Some((given,family))=input.captain{add!(cstr(given));add!(b" ");add!(cstr(family));add!(b" ")}
        add!(format!("{}\\C",input.initial_money).as_bytes());text!(0x6b960c);
        add!(format!("{}\\C",input.returned_money).as_bytes());text!(0x6b9610);
    }
    if out.len()>=4000{return Err("carta de expedición supera el búfer original".into())}
    out.push(0);declared+=1;out.truncate(declared);Ok(out)
}

/// Publica la carta y la cabeza del comerciante como una sola transacción.
/// `town` es la ciudad de origen del destinatario, ya resuelta por el llamante.
pub fn deliver(texts:&ExpeditionLetterTexts,input:&ExpeditionLetter<'_>,pool:&mut Vec<u8>,
    merchant:&mut p3_core::ai::reputation::Merchant,date:(u8,u8,u16),town:u8)->Result<bool,String>{
    let payload=build(texts,input)?;let mut staged_pool=pool.clone();let mut staged_merchant=merchant.clone();
    let mut record=[0u8;16];record[1]=0x80;record[2..4].fill(0xff);record[4]=0x63;record[5]=town;record[6..8].fill(0xff);
    let delivered=p3_state::messages::post_to_merchant(&mut staged_pool,&mut staged_merchant,date.0,date.1,date.2,
        &record,p3_state::messages::Payload::Text(&payload),&p3_state::messages::RETENTION)?;
    *pool=staged_pool;*merchant=staged_merchant;Ok(delivered)
}
