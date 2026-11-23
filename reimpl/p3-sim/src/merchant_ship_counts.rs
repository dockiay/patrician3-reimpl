//! Reconstrucción de `merchant+0x47C` durante `ships_deserialize` (0x00504645).
use p3_core::ai::reputation::Merchant;

/// Cuenta los nodos de cada cadena `merchant+0xE` enlazada por `ship+4`.
/// Un ciclo se rechaza: el bucle original no termina para ese dato corrupto.
pub fn rebuild(merchants:&mut [Merchant],next:&[u16])->Result<(),&'static str>{
 let mut counts=Vec::with_capacity(merchants.len());
 for merchant in merchants.iter(){if merchant.bytes.len()<0x480{return Err("comerciante truncado")}let mut id=usize::from(merchant.u16(0xe));let mut seen=vec![false;next.len()];let mut count=0u32;while id<next.len(){if seen[id]{return Err("flota comercial cíclica")}seen[id]=true;count=count.wrapping_add(1);id=usize::from(next[id]);}counts.push(count);}
 for (merchant,count) in merchants.iter_mut().zip(counts){merchant.set_i32(0x47c,count as i32)}Ok(())
}
