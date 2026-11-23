//! Publicación en la flota viva (`LiveVoyage`) de lo que crea la fábrica hanseática
//! (`create_pirate_band_convoy` 0x0050AC40, fase 1 de las expediciones 0x23/0x24/0x34).
//!
//! En el original la fábrica y la navegación comparten los mismos objetos del gestor
//! (`0x6DD7A0`): un barco o convoy creado por la fase 1 existe en la flota en ese mismo
//! instante. p3-sim mantiene dos vistas; `publish_navigation_delta` lleva los cambios de la
//! navegación a la fábrica y esta función hace el camino inverso para la creación.
//! Como en la subasta (`publish_auction_ship`), la base de la navegación recibe también la
//! imagen creada: la persistencia de la fábrica escribe la creación y la de la navegación
//! solo los cambios posteriores.
use p3_core::ships::factory::HanseFactory;

fn put(dst:&mut Vec<u8>,i:usize,len:usize,image:&[u8]){let at=i*len;if dst.len()<at+len{dst.resize(at+len,0)}dst[at..at+len].copy_from_slice(image)}

/// Copia barcos, convoyes, capitanes y palabras del gestor que cambian entre `before` y
/// `after`. Falla sin tocar nada si las vistas no tienen el mismo tamaño de partida.
pub fn publish_factory(nav:&mut crate::navigation::LiveVoyage,before:&HanseFactory,after:&HanseFactory)->Result<(),String>{
    let snap=&nav.snapshot;
    if snap.ships.len()!=before.ships.len()*0x180||snap.convoys.len()<before.convoys.len()*0x3c||snap.captains.len()<before.captains.len()*16||snap.routes.len()!=before.ships.len()||nav.baseline.ships.len()!=snap.ships.len()||snap.manager.len()<0xf8{
        return Err("flota viva incompatible con la fábrica".into());
    }
    if after.ships.len()<before.ships.len()||after.convoys.len()<before.convoys.len()||after.captains.len()<before.captains.len(){return Err("la fábrica encogió".into())}
    let mut staged=nav.snapshot.clone();let mut base=nav.baseline.clone();
    for (i,image) in after.ships.iter().enumerate(){
        if before.ships.get(i)==Some(image){continue}
        if image.len()!=0x180{return Err("barco de fábrica truncado".into())}
        let live=u16::from_le_bytes([image[0x134],image[0x135]])<0x17;
        put(&mut staged.ships,i,0x180,image);put(&mut base.ships,i,0x180,image);
        let route=live.then_some(p3_save::ships::SavedRoute::None);
        if i<staged.routes.len(){staged.routes[i]=route.clone()}else{staged.routes.push(route.clone())}
        if i<base.routes.len(){base.routes[i]=route}else{base.routes.push(route)}
    }
    for (i,image) in after.convoys.iter().enumerate(){
        if before.convoys.get(i)==Some(image){continue}
        if image.len()!=0x3c{return Err("convoy de fábrica truncado".into())}
        put(&mut staged.convoys,i,0x3c,image);put(&mut base.convoys,i,0x3c,image);
    }
    for (i,image) in after.captains.iter().enumerate(){
        if before.captains.get(i)==Some(image){continue}
        put(&mut staged.captains,i,16,image);put(&mut base.captains,i,16,image);
    }
    let w=|m:&mut Vec<u8>,at:usize,v:u16|m[at..at+2].copy_from_slice(&v.to_le_bytes());
    for (at,old,new) in [(0xe0,before.free_captain,after.free_captain),(0xe2,before.free_ship,after.free_ship),(0xe4,before.free_convoy,after.free_convoy),(0xec,before.convoy_head,after.convoy_head),(0xee,before.name_cursor,after.name_cursor),(0xf0,before.name_step,after.name_step)]{
        if old!=new{w(&mut staged.manager,at,new)}
    }
    let counts=[(0xf2,after.captains.len()),(0xf4,after.ships.len()),(0xf6,after.convoys.len())];
    for (at,n) in counts{let n=u16::try_from(n).map_err(|_|"fábrica demasiado grande")?;w(&mut staged.manager,at,n);w(&mut base.manager,at,n)}
    crate::navigation::refresh_lists(&mut staged)?;
    nav.snapshot=staged;nav.baseline=base;Ok(())
}

/// Fases de las expediciones (0x23/0x24/0x34) que cambian barcos o convoyes ya creados
/// (salida 7, búsqueda 8/9, vuelta 10, fin 5) trabajan sobre las imágenes de la sesión; con
/// la flota viva se llevan a la navegación los bytes que cambian en las ranuras que la tarea
/// declara cambiadas. Solo la vista viva: la base sigue siendo la del guardado, de modo que la
/// persistencia de la navegación ve el mismo cambio que la de la expedición.
pub fn publish_images(nav:&mut crate::navigation::LiveVoyage,ships_before:&[Option<Vec<u8>>],ships_after:&[Option<Vec<u8>>],ships:&[usize],convoys_before:&[Vec<u8>],convoys_after:&[Vec<u8>],convoys:&[usize],docked_before:u16,docked_after:u16)->Result<(),String>{
    publish_images_from(nav,|i|ships_before.get(i).and_then(|v|v.as_deref()),ships_after,ships,|i|convoys_before.get(i).map(Vec::as_slice),convoys_after,convoys,docked_before,docked_after)
}
/// `publish_images` con las imágenes anteriores servidas por índice (el despacho de cada tick las guarda en bloques
/// seguidos). Escribe sobre la flota viva sin clonarla y anota cada byte, ruta y cabecera que cambia: ante un error
/// lo deshace, de modo que la flota queda intacta igual que cuando se trabajaba sobre una copia.
pub(crate) fn publish_images_from<'a>(nav:&mut crate::navigation::LiveVoyage,ship_before:impl Fn(usize)->Option<&'a [u8]>,ships_after:&[Option<Vec<u8>>],ships:&[usize],convoy_before:impl Fn(usize)->Option<&'a [u8]>,convoys_after:&[Vec<u8>],convoys:&[usize],docked_before:u16,docked_after:u16)->Result<(),String>{
    let staged=&mut nav.snapshot;
    let head=[staged.manager[0xe8],staged.manager[0xe9]];
    let mut undo_ships:Vec<(usize,u8)>=Vec::new();let mut undo_convoys:Vec<(usize,u8)>=Vec::new();let mut undo_routes:Vec<(usize,Option<p3_save::ships::SavedRoute>)>=Vec::new();
    let mut apply=|staged:&mut p3_save::ships::NavigationSnapshot|->Result<(),String>{
        // La fase 5 apila los barcos desembarcados en la lista de atracados (`gestor+0xE8`).
        if docked_before!=docked_after&&u16::from_le_bytes(head)==docked_before{staged.manager[0xe8..0xea].copy_from_slice(&docked_after.to_le_bytes())}
        for &i in ships{
            let (Some(old),Some(Some(new)))=(ship_before(i),ships_after.get(i)) else{continue};
            if old==new.as_slice(){continue}
            let live=staged.ships.get_mut(i*0x180..(i+1)*0x180).ok_or("barco de expedición fuera de la flota viva")?;
            for j in 0..0x180{if old[j]!=new[j]{undo_ships.push((i*0x180+j,live[j]));live[j]=new[j]}}
            if u16::from_le_bytes([new[0x134],new[0x135]])>=0x17{if let Some(r)=staged.routes.get_mut(i){undo_routes.push((i,r.take()))}}
        }
        for &i in convoys{
            let (Some(old),Some(new))=(convoy_before(i),convoys_after.get(i)) else{continue};
            if old==new.as_slice()||old.len()<0x3c||new.len()<0x3c{continue}
            let live=staged.convoys.get_mut(i*0x3c..(i+1)*0x3c).ok_or("convoy de expedición fuera de la flota viva")?;
            for j in 0..0x3c{if old[j]!=new[j]{undo_convoys.push((i*0x3c+j,live[j]));live[j]=new[j]}}
        }
        crate::navigation::refresh_lists(staged)
    };
    let result=apply(staged);
    if result.is_err(){
        for (at,v) in undo_ships.into_iter().rev(){staged.ships[at]=v}
        for (at,v) in undo_convoys.into_iter().rev(){staged.convoys[at]=v}
        for (i,r) in undo_routes.into_iter().rev(){staged.routes[i]=r}
        staged.manager[0xe8..0xea].copy_from_slice(&head);
    }
    result
}
