//! Recursos gramaticales del intérprete F7; bytes leídos del ejecutable local.
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct ScriptTextBank {
    pub gender_words:[[Vec<u8>;2];8],
    /// Tablas estáticas de `%R %r %S %w %D %W` (el estado vivo —barcos, comerciantes, jugador local— lo rellena el anfitrión).
    pub format_static:p3_core::mission_format::FormatExtras,
}
impl ScriptTextBank {
    pub fn from_english_exe(bytes:&[u8])->Result<Self,String>{
        let hash=format!("{:x}",Sha256::digest(bytes));
        if hash!="fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece" {
            return Err("ejecutable inglés no reconocido para textos de guion".into());
        }
        let image=p3_assets::exe::PeImage::parse(bytes).map_err(str::to_string)?;
        let mut gender_words: [[Vec<u8>;2];8]=std::array::from_fn(|_|std::array::from_fn(|_|Vec::new()));
        for (digit,pair) in gender_words.iter_mut().enumerate(){
            for (gender,word) in pair.iter_mut().enumerate(){
                let slot=0x6c3040+4*(2*digit+gender) as u32;
                *word=image.pointed_cstr(slot).map_err(|e|format!("texto gramatical {slot:#x}: {e}"))?.to_vec();
            }
        }
        let table=|base:u32,n:u32|->Result<Vec<Vec<u8>>,String>{(0..n).map(|i|image.pointed_cstr(base+4*i).map(|b|b.to_vec()).map_err(|e|format!("texto de formato {:#x}: {e}",base+4*i))).collect()};
        let byte=|a:u32|image.byte_at(a).map_err(|e|format!("byte {a:#x}: {e}"));
        let mut format_static=p3_core::mission_format::FormatExtras::default();
        format_static.titles_a=table(0x6a0608,16)?;
        format_static.titles_b=table(0x6a0718,16)?;
        format_static.ship_types=table(0x6c0cb4,4)?;
        format_static.goods_names=table(0x6c0bb0,24)?;
        format_static.month_names=table(0x6c0d9c,13).or_else(|_|{let mut t=vec![b"(null)".to_vec()];for i in 1..13{t.push(image.pointed_cstr(0x6c0d9c+4*i).map_err(|e|format!("mes {i}: {e}"))?.to_vec())}Ok::<_,String>(t)})?;
        for i in 0..24{format_static.goods_flags.push(byte(0x672c14+i)?)}
        for i in 0..258u32{format_static.calendar.push(u16::from_le_bytes([byte(0x672d76+2*i)?,byte(0x672d77+2*i)?]))}
        Ok(Self{gender_words,format_static})
    }
}

/// Ranuras personalizadas del banco de nombres tal como quedan en memoria tras `name_bank_load_from_buffer`
/// (`0x00511E30`): tres grupos (hombres, mujeres, apellidos) de veinte nombres. El bucle de copia deja de avanzar
/// el destino al llegar a la posición 22 (`cmp ecx, 0x18`), así que un nombre más largo queda en sus 22 primeros
/// bytes. `None` si el bloque no tiene el formato del guardado.
pub fn custom_name_slots(block:&[u8])->Option<[Vec<Vec<u8>>;3]>{
    let bank=p3_save::tail::NameBank::parse(block).ok()?;
    Some(std::array::from_fn(|g|(0..20).map(|i|{let n=bank.name(g,i);n[..n.len().min(22)].to_vec()}).collect()))
}
