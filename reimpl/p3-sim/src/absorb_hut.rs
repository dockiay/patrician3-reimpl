//! `0x00515DA0`, sobre la imagen serializable de la banda. Reutiliza el
//! asignador de piezas C08 y la búsqueda de celdas C05.
//!
//! El original pasa la orientación como *tipo* a `0x004F9C80`: 0x20 busca
//! suelo 0x1B, mientras 0xE0 retorna falso. Después pide colocar tipo 0x2A,
//! cuya máscara no coincide con ese suelo. Se conserva esta conducta.

use p3_core::{buildings::Site, economy::pirate_band::{self, BandHost}, flatmem::{Mem, SparseMem}, town::Town};
use p3_save::pirate_bands::PirateBand;

const OBJECT:u32=0x1000;
const PIECES:u32=0x10000;
const GROWN:u32=0x100000;

struct Allocator;
impl BandHost for Allocator {
    fn alloc(&mut self,m:&mut dyn Mem,size:u32)->u32{for at in 0..size{m.w8_seed(GROWN+at)}GROWN}
    fn free(&mut self,_:u32){}
    fn free_port(&mut self,_:u32)->u8{unreachable!("alloc_slot no busca puertos")}
    fn create_ship(&mut self,_:&mut dyn Mem,_:u32,_:u32,_:u32,_:u32)->u32{unreachable!("alloc_slot no crea barcos")}
}

fn word(bytes:&[u8],at:usize)->u16{u16::from_le_bytes([bytes[at],bytes[at+1]])}

/// Añade como máximo una choza, exactamente en las condiciones de 0x515DA0.
/// Devuelve si se reservó una pieza. Ante datos inválidos no modifica la banda.
/// `convoy_count`, reloj y día son los globales vivos, no estados inferidos.
pub fn build_hut(band:&mut PirateBand,convoy_count:u16,tick:u32,day:u16)->Result<bool,String>{
    band.validate().map_err(str::to_owned)?;
    if word(&band.header,8)<convoy_count{return Ok(false)}
    let capacity=usize::from(word(&band.header,0));
    let mut remaining=(i32::from(word(&band.header,14))/200-2).min(32);
    let mut cursor=0usize;let mut seen=vec![false;capacity];
    while cursor<capacity {
        if seen[cursor]{return Err("cadena de piezas piratas cíclica".into())}seen[cursor]=true;
        if band.pieces[cursor*8+3]==0x1b {remaining-=1;if remaining<=0{return Ok(false)}}
        cursor=usize::from(word(&band.pieces,cursor*8+4));
    }
    let free=usize::from(word(&band.header,2));
    if free>capacity||free<capacity&&seen[free]{return Err("lista libre de piezas piratas inválida".into())}
    if free==capacity&&capacity>usize::from(u16::MAX)-32{return Err("capacidad de piezas piratas desbordada".into())}
    let mut staged=band.clone();
    let mut memory=SparseMem::default();memory.seed(OBJECT,&[0;24]);memory.seed(OBJECT+8,&staged.header);memory.seed(PIECES,&staged.pieces);memory.w32(OBJECT+4,PIECES);
    let piece=pirate_band::alloc_slot(&mut memory,&mut Allocator,OBJECT);
    memory.w8(piece+3,0x1b);let orientation=if tick&1!=0{0x20}else{0xe0};memory.w8(piece+7,orientation);
    if let Some(plan)=staged.plan.as_mut(){
        let mut town=Town::default();town.0[0x7a8..0x80c].copy_from_slice(&plan[..0x64]);
        let rows=town.i32(0x7a8);let columns=town.i32(0x7ac);
        if rows<=0||columns<=0||rows>256||columns>256||i32::from(town.u8(0x800))>=rows||i32::from(town.u8(0x801))>=columns{return Err("dimensiones o centro del plano pirata inválidos".into())}
        let table=memory.r32(OBJECT+4);
        let mut row=memory.r8(table).wrapping_sub((tick&3)as u8).wrapping_add(15);
        let mut column=memory.r8(table+1).wrapping_sub((day&3)as u8).wrapping_add(15);
        let mut site=Site{records:Vec::new(),cells:plan[0x64..].to_vec()};
        if site.find_nearest_cell_for_type(&town,&mut row,&mut column,i32::from(orientation),0x2a,false,0xe0){
            site.place_building(&mut town,i32::from(row),i32::from(column),0x2a,orientation,0xe0);
        }
        memory.w8(piece,row);memory.w8(piece+1,column);
        plan[..0x64].copy_from_slice(&town.0[0x7a8..0x80c]);plan[0x64..].copy_from_slice(&site.cells);
    }else{memory.w8(piece,0xff);memory.w8(piece+1,0xff)}
    staged.header.copy_from_slice(&memory.read(OBJECT+8,16));let table=memory.r32(OBJECT+4);let size=usize::from(memory.r16(OBJECT+8))*8;staged.pieces=memory.read(table,size);
    staged.validate().map_err(str::to_owned)?;*band=staged;Ok(true)
}
