//! Registro de capitán de 16 bytes del formato 0x85.
//! El salario de memoria +0xC se guarda en +8 y la experiencia +0xA en +0xC.
pub const CAPTAIN_LEN:usize=16;
/// `captain_deserialize` 0x004FDEB0 para versión 0x85.
pub fn decode_v85(saved:&[u8;CAPTAIN_LEN])->[u8;CAPTAIN_LEN]{
 let mut mem=[0;CAPTAIN_LEN];mem[..8].copy_from_slice(&saved[..8]);mem[12..14].copy_from_slice(&saved[8..10]);
 mem[8..12].copy_from_slice(&saved[10..14]);mem[14]=u8::from(saved[14]!=0);mem[15]=saved[15];mem
}
/// `captain_serialize` 0x004FDE30: normaliza el booleano en +0xE.
pub fn encode_v85(mem:&[u8;CAPTAIN_LEN])->[u8;CAPTAIN_LEN]{
 let mut saved=[0;CAPTAIN_LEN];saved[..8].copy_from_slice(&mem[..8]);saved[8..10].copy_from_slice(&mem[12..14]);
 saved[10..14].copy_from_slice(&mem[8..12]);saved[14]=u8::from(mem[14]!=0);saved[15]=mem[15];saved
}
