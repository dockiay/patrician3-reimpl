//! Puertos seguros del recuento de calles y del edificio municipal 0x2F.
use p3_core::{buildings::{Site,type2f::Type2fHost},town::{off,Town}};

fn dimensions(t:&Town,s:&Site)->Result<(usize,usize),&'static str>{
 let rows=t.i32(off::PLAN_ROWS);let cols=t.i32(off::PLAN_STRIDE);
 if rows<0||cols<0||rows>255||cols>255{return Err("dimensiones inválidas")}
 let count=(rows as usize).checked_mul(cols as usize).ok_or("dimensiones inválidas")?;if count>s.cells.len(){return Err("rejilla truncada")}Ok((rows as usize,cols as usize))
}
fn chain(s:&Site,head:u16,count:usize)->Result<Vec<usize>,&'static str>{
 if s.records.len()<count*8{return Err("registros truncados")};let mut seen=vec![false;count];let mut out=Vec::new();let mut i=usize::from(head);
 while i<count{if seen[i]{return Err("lista de edificios cíclica")};seen[i]=true;out.push(i);i=usize::from(s.next(i))}Ok(out)
}
pub fn count_streets(t:&mut Town,s:&Site)->Result<bool,&'static str>{
 dimensions(t,s)?;let (total,built)=s.count_street_cells(t);let changed=t.u16(off::STREETS_TOTAL)!=total||t.u16(off::STREETS_BUILT)!=built;t.set_u16(off::STREETS_TOTAL,total);t.set_u16(off::STREETS_BUILT,built);Ok(changed)
}
pub fn try_add_type2f(t:&mut Town,s:&mut Site,employees:u32,host:&mut dyn Type2fHost)->Result<bool,&'static str>{
 if t.flags()&0x100_0000!=0{return Ok(false)}
 let count=usize::from(t.u16(off::BUILDING_RECORD_COUNT));let public=chain(s,t.u16(off::PUBLIC_BUILDING_HEAD),count)?;let found=public.iter().filter(|&&i|s.rec(i,3)==0x2f).count() as u32;let weighted=found.wrapping_mul(((found+1)>>1).wrapping_mul(30));
 if weighted>=employees||t.i32(0x30)<20000||t.i32(0x34)<2000{return Ok(false)}
 let (rows,cols)=dimensions(t,s)?;let mut first=None;
 for (at,&cell) in s.cells.iter().take(rows*cols).enumerate(){if cell==0x0f {let neighbor=at.checked_add(cols).and_then(|v|v.checked_sub(2)).ok_or("marcador sin vecino")?;if neighbor>=rows*cols{return Err("marcador sin vecino")}if first.is_none(){first=Some(at)}}}
 if let Some(at)=first {let row=at/cols;let col=at%cols;let other=if col&1!=0{row.checked_add(1)}else{row.checked_sub(1)}.ok_or("marcador fuera de rejilla")?;if col<2||other>=rows{return Err("huella de marcador fuera de rejilla")};chain(s,t.u16(off::CONSTRUCTION_HEAD),count)?;let free=t.u16(off::FREE_BUILDING_HEAD);if usize::from(free)>count||(usize::from(free)==count&&count>u16::MAX as usize-128){return Err("pool de edificios inválido")}}
 let mut town=t.clone();let mut site=s.clone();site.try_add_type_2f_building_by_employees(&mut town,employees,None,host);let changed=town.0!=t.0||site!=*s;*t=town;*s=site;Ok(changed)
}
