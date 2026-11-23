//! Normalización acotada de rutas leídas por el cargador de C10.
use p3_core::{route::finder::{RouteOut,RoutePlanner},ships::route::apply_route_plan};
use p3_save::ships::SavedRoute;

/// Prepara el caso de 0x517C10 con movimiento horizontal y coordenada Y
/// desalineada, destino por punto y ruta propia. No ejecuta las otras ramas.
/// El planificador solo se confirma si toda la preparación tiene éxito.
pub fn prepare_horizontal_loaded_replan(image:&[u8],route:&SavedRoute,planner:&mut RoutePlanner,
    tick:u32,town_count:usize,token:u32)->Result<(Vec<u8>,Vec<(i16,i16)>),String>{
    if image.len()!=0x180 || usize::from(image[0x38])<town_count || token==0 {return Err("barco de punto incompatible".into())}
    let SavedRoute::Owned(points)=route else{return Err("ruta propia ausente".into())};
    let word=|at:usize|u16::from_le_bytes([image[at],image[at+1]]);
    let signed=|at:usize|i32::from_le_bytes(image[at..at+4].try_into().unwrap());
    let i=usize::from(word(0x34));let target=points.get(i).ok_or("índice de ruta inválido")?;
    let (x,y)=(word(0x1e) as i16,word(0x22) as i16);
    // La rama exterior de 0x517C10 recoloca barcos fuera del mapa; aún no
    // está conectada. Límites observados del ejecutable inglés 2400×1800.
    if signed(0x28)==0 || signed(0x2c)!=0 || target.1==y || x<0 || y<0 || x>2400 || y>1800 {
        return Err("rama de reparación no contrastada".into());
    }
    let end=*points.last().ok_or("ruta sin destino")?;let mut trial=planner.clone();
    let found=trial.find_route(&[(x,y),end]);
    let RouteOut::Built(rebuilt)=found.route else{return Err("el planificador no devolvió ruta propia".into())};
    if found.status==0 || rebuilt.len()<2{return Err("propiedad de ruta no contrastada".into())}
    let mut out=image.to_vec();apply_route_plan(&mut out,None,end,&rebuilt,true,token,tick)
        .ok_or("no se puede actualizar la ruta")?;
    *planner=trial;Ok((out,rebuilt))
}

/// Decide la rama de reparación de 0x517C10 con un descriptor ya resuelto.
/// Un cociente no representable se rechaza igual que el #DE del original.
pub fn loaded_route_needs_replan(image:&[u8],points:&[(i16,i16)])->Result<bool,&'static str>{
    if image.len()!=0x180 || points.len()>i32::MAX as usize{return Err("ruta cargada inválida")}
    let u16at=|at:usize|u16::from_le_bytes([image[at],image[at+1]]);
    let i32at=|at:usize|i32::from_le_bytes(image[at..at+4].try_into().unwrap());
    let Some(&(tx,ty))=points.get(usize::from(u16at(0x34))) else{return Ok(true)};
    let (x,y)=(u16at(0x1e)as i16,u16at(0x22)as i16);let (vx,vy)=(i32at(0x28),i32at(0x2c));
    if vx==0 {return Ok(if vy==0{tx!=x||ty!=y}else{tx!=x||if vy>0{ty<=y}else{ty>=y}})}
    if vy==0 {return Ok(ty!=y||if vx>0{tx<=x}else{tx>=x})}
    let ax=(i32::from(tx).wrapping_shl(16)).wrapping_sub(i32at(0x1c));
    let ay=(i32::from(ty).wrapping_shl(16)).wrapping_sub(i32at(0x20));
    let a=ax.checked_div(vx).ok_or("división de ruta inválida")?;
    let b=ay.checked_div(vy).ok_or("división de ruta inválida")?;
    let mut low=a.min(b).max(0);let high=a.max(b).wrapping_add(2);
    if i64::from(high)-i64::from(low)>4096{return Err("recorrido de comprobación excesivo")}
    while low<=high {
        let x=i32at(0x1c).wrapping_add(vx.wrapping_mul(low))>>16;
        let y=i32at(0x20).wrapping_add(vy.wrapping_mul(low))>>16;
        if x==i32::from(tx)&&y==i32::from(ty){return Ok(false)}
        if low==i32::MAX {return Err("bucle de ruta excesivo")}
        low+=1;
    }
    Ok(true)
}

/// Revisa rutas de un bloque S05 recién cargado; actualmente solo reconstruye
/// la rama de punto propio horizontal contrastada. Devuelve error para las
/// demás reparaciones pendientes y conserva el modelo de entrada.
pub fn normalize_loaded_ship_routes(ships:&mut p3_save::model::ShipsModel,
    navigation:&crate::foundation_nav::FoundationNavigation,town_order:&[u8],tick:u32)->Result<usize,String>{
    let mut planner=navigation.build_planner(town_order,100_000)?;
    let block=ships.write();let parsed=p3_save::ships::Ships::parse(&block).map_err(|e|format!("barcos: {e:?}"))?;
    let mut input=Vec::new();for i in 0..parsed.ship_slots(){if let Some(image)=parsed.ship_image(i){
        input.push((i,image,parsed.saved_route(i).map_err(str::to_owned)?));
    }}
    let mut staged=ships.clone();let mut changed=0usize;
    for (i,before,route) in input {
        let points=match &route {
            SavedRoute::None=>continue,
            SavedRoute::Owned(points)=>points.clone(),
            SavedRoute::Shared{origin,destination}=>{
                let count=town_order.len();let a=usize::from(*origin);let b=usize::from(*destination);
                if a>=count||b>=count{return Err(format!("ruta compartida inválida: barco {i}"))}
                let id=planner.town_table.get(a*count+b).and_then(|v|*v).ok_or("ruta compartida ausente")?;
                planner.routes.get(id).ok_or("descriptor compartido ausente")?.clone()
            }
        };
        if !loaded_route_needs_replan(&before,&points).map_err(str::to_owned)?{continue}
        let mut loaded=before.clone();p3_core::ships::cargo::recompute_load(&mut loaded);
        let (mut after,points)=prepare_horizontal_loaded_replan(&loaded,&route,&mut planner,tick,town_order.len(),
            u32::try_from(i).map_err(|_|"barco excesivo")?.wrapping_add(1))
            .map_err(|e|format!("reparación de ruta {i}: {e}"))?;
        after[0x118..0x124].copy_from_slice(&before[0x118..0x124]);
        staged.apply_ship_route(i,&before,&after,&points,town_order.len())?;changed+=1;
    }
    *ships=staged;Ok(changed)
}


/// Reconcilia daño y pérdida de equipo tras la carga. Primero repara rutas
/// con la imagen anterior al daño: su duración depende de la vida. El equipo
/// se valida mediante el recálculo original y sus derivados no se serializan.
pub fn normalize_loaded_routes_before_damage(ships:&mut p3_save::model::ShipsModel,
    before:&p3_save::ships::NavigationSnapshot,after:&p3_save::ships::NavigationSnapshot,
    navigation:&crate::foundation_nav::FoundationNavigation,town_order:&[u8],tick:u32)->Result<usize,String>{
    normalize_loaded_routes_before_damage_with_speed(ships,before,after,navigation,town_order,tick,&[])
}

/// Como la anterior, admite cambios de velocidad solo en barcos de la escena.
pub fn normalize_loaded_routes_before_damage_with_speed(ships:&mut p3_save::model::ShipsModel,
    before:&p3_save::ships::NavigationSnapshot,after:&p3_save::ships::NavigationSnapshot,
    navigation:&crate::foundation_nav::FoundationNavigation,town_order:&[u8],tick:u32,
    speed_ships:&[usize])->Result<usize,String>{
    if before.ships.len()!=after.ships.len() || before.ships.len()%0x180!=0{return Err("flota cargada truncada".into())}
    if speed_ships.iter().any(|&i|i>=before.ships.len()/0x180){return Err("barco de velocidad ausente".into())}
    let ranges=[0x18..0x1c,0x3c..0x3e,0x40..0x42,0x54..0xb4,0x114..0x118,0x13c..0x154];
    let mut masked=after.clone();let mut changed=Vec::new();
    for i in 0..before.ships.len()/0x180{let at=i*0x180;let a=&before.ships[at..at+0x180];let b=&after.ships[at..at+0x180];
        let guns=(0x13c..0x154).filter(|&n|a[n]!=b[n]).collect::<Vec<_>>();
        if !guns.is_empty(){
            let removed=guns.iter().copied().filter(|&n|a[n]<6 && b[n]==255).collect::<Vec<_>>();
            if removed.len()!=1 || guns.iter().any(|&n|n!=removed[0] && !(n==removed[0]+1 && (removed[0]-0x13c)%2==0 && a[n]==6 && b[n]==255)) {return Err("pérdida de equipo no conciliada".into())}
            let slot=removed[0];if (slot-0x13c)%2==0 && slot+1<0x154 && a[slot+1]==6 && b[slot+1]!=255{return Err("pareja de equipo no conciliada".into())}
            let mut recomputed=a.to_vec();recomputed[0x13c..0x154].copy_from_slice(&b[0x13c..0x154]);p3_core::ships::cargo::recompute_load(&mut recomputed);
            if recomputed[0x54..0xb4]!=b[0x54..0xb4] || recomputed[0x114..0x124]!=b[0x114..0x124]{return Err("recálculo de equipo no conciliado".into())}
        }else if a[0x54..0xb4]!=b[0x54..0xb4] || a[0x114..0x124]!=b[0x114..0x124]{return Err("carga cambiada sin pérdida de equipo".into())}
        let speed_changed=a[0x30..0x34]!=b[0x30..0x34];
        if speed_changed && speed_ships.contains(&i){masked.ships[at+0x30..at+0x34].copy_from_slice(&a[0x30..0x34]);}
        if ranges.iter().any(|r|a[r.clone()]!=b[r.clone()]) || (speed_changed && speed_ships.contains(&i)){changed.push(i)}
        for range in &ranges{masked.ships[at+range.start..at+range.end].copy_from_slice(&a[range.clone()]);}
        // Los tres derivados no se guardan en S05: el cargador los reconstruye.
        masked.ships[at+0x118..at+0x124].copy_from_slice(&a[0x118..0x124]);
    }
    if masked!=*before{return Err("cambios navales no conciliados con rutas cargadas".into())}
    let mut staged=ships.clone();
    for &i in &changed{let at=i*0x180;let stored=staged.ship_image(i)?;let mut previous=stored.clone();
        for range in &ranges{if stored[range.clone()]!=after.ships[at+range.start..at+range.end]{return Err(format!("daño naval desincronizado: barco {i}"))}previous[range.clone()].copy_from_slice(&before.ships[at+range.start..at+range.end]);}
        if speed_ships.contains(&i){if stored[0x30..0x34]!=after.ships[at+0x30..at+0x34]{return Err(format!("velocidad desincronizada: barco {i}"))}previous[0x30..0x34].copy_from_slice(&before.ships[at+0x30..at+0x34]);}
        staged.apply_ship_image(i,&stored,&previous)?;
    }
    let count=normalize_loaded_ship_routes(&mut staged,navigation,town_order,tick)?;
    for &i in &changed{let at=i*0x180;let stored=staged.ship_image(i)?;let mut damaged=stored.clone();
        for range in &ranges{if stored[range.clone()]!=before.ships[at+range.start..at+range.end]{return Err(format!("base de daño desincronizada: barco {i}"))}damaged[range.clone()].copy_from_slice(&after.ships[at+range.start..at+range.end]);}
        if speed_ships.contains(&i){if stored[0x30..0x34]!=before.ships[at+0x30..at+0x34]{return Err(format!("base de velocidad desincronizada: barco {i}"))}damaged[0x30..0x34].copy_from_slice(&after.ships[at+0x30..at+0x34]);}
        staged.apply_ship_image(i,&stored,&damaged)?;
    }
    *ships=staged;Ok(count)
}
