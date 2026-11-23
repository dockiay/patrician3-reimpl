//! Puente entre la reconstrucción de S02 y el índice espacial vivo del combate.
use p3_core::ships::{battle::BattleCollisionPool,spatial::{Axis,CollisionIndex}};

fn rows(matrix:&[Vec<u32>],capacity:usize)->Result<Vec<Vec<u32>>,&'static str>{
 let pages=capacity/32;if matrix.len()!=pages||matrix.iter().any(|p|p.len()!=capacity){return Err("matriz espacial truncada")}
 Ok((0..capacity).map(|r|(0..pages).map(|p|matrix[p][r]).collect()).collect())
}
fn pages(matrix:&[Vec<u32>],capacity:usize)->Result<Vec<Vec<u32>>,&'static str>{
 let n=capacity/32;if matrix.len()!=capacity||matrix.iter().any(|r|r.len()!=n){return Err("matriz espacial truncada")}
 Ok((0..n).map(|p|(0..capacity).map(|r|matrix[r][p]).collect()).collect())
}

pub fn into_live(pool:&BattleCollisionPool)->Result<CollisionIndex,&'static str>{
 let n=pool.kinds.len();if n==0||n%32!=0||n>32736||usize::from(pool.high_water)>n||pool.owners.len()!=n||pool.free_next.len()!=n{return Err("capacidad espacial inválida")}
 for a in [&pool.x,&pool.x_next,&pool.x_previous,&pool.y,&pool.y_next,&pool.y_previous]{if a.len()!=n*2{return Err("eje espacial truncado")}}
 let axis=|coordinates:&[u16],next:&[u16],previous:&[u16],head,mat:&[Vec<u32>]|->Result<Axis,&'static str>{Ok(Axis{head,coordinates:coordinates.iter().map(|v|*v as i16).collect(),next:next.to_vec(),previous:previous.to_vec(),overlap:rows(mat,n)?})};
 Ok(CollisionIndex{capacity:n,span:pool.high_water,free_head:pool.free_head,free_next:pool.free_next.clone(),types:pool.kinds.clone(),references:pool.owners.clone(),x:axis(&pool.x,&pool.x_next,&pool.x_previous,pool.x_head,&pool.matrices[0])?,y:axis(&pool.y,&pool.y_next,&pool.y_previous,pool.y_head,&pool.matrices[1])?,pairs:rows(&pool.matrices[2],n)?})
}

pub fn into_saved(index:&CollisionIndex)->Result<BattleCollisionPool,&'static str>{
 let n=index.capacity;if n==0||n%32!=0||n>32736||usize::from(index.span)>n||index.types.len()!=n||index.references.len()!=n||index.free_next.len()!=n{return Err("capacidad espacial inválida")}
 for a in [&index.x,&index.y]{if a.coordinates.len()!=n*2||a.next.len()!=n*2||a.previous.len()!=n*2{return Err("eje espacial truncado")}}
 Ok(BattleCollisionPool{kinds:index.types.clone(),owners:index.references.clone(),free_next:index.free_next.clone(),x:index.x.coordinates.iter().map(|v|*v as u16).collect(),x_next:index.x.next.clone(),x_previous:index.x.previous.clone(),y:index.y.coordinates.iter().map(|v|*v as u16).collect(),y_next:index.y.next.clone(),y_previous:index.y.previous.clone(),matrices:[pages(&index.x.overlap,n)?,pages(&index.y.overlap,n)?,pages(&index.pairs,n)?],free_head:index.free_head,x_head:index.x.head,y_head:index.y.head,high_water:index.span})
}
