//! Prefijo de 0x5FD510 para un asalto activo y controlado por IA.
//!
//! El original acumula decimosextos de tick en +0x130F. Este tramo no
//! ejecuta movimiento ni colisiones y rechaza la llamada que los activaría.

/// Avanza solamente el prefijo sin ticks de simulación. No modifica nada si
/// hay UI humana, diálogo, cambios de terreno o un tick completo pendiente.
pub fn advance_ai_prefix(scene: &mut [u8], delta: u32) -> Result<(), &'static str> {
    if scene.len() != 0x2f24 || scene[0x5c] != 0 || scene[0x2f1a] != 0 {
        return Err("escena activa incompatible");
    }
    let participants = usize::from(scene[0x558]);
    if participants == 0 || 0x5d + participants > scene.len()
        || scene[0x5d..0x5d + participants].iter().any(|&v| v == 0)
        || scene[0x12e9] != 0 || scene[0x12ea] != 0
    {
        return Err("interfaz o participantes de asalto no conectados");
    }
    let fraction = u32::from(scene[0x130f]);
    if fraction > 15 { return Err("fracción de asalto inválida"); }
    let ticks = u32::from(delta as u16);
    if fraction + ticks >= 16 { return Err("tick completo de asalto pendiente"); }
    if ticks != 0 {
        scene[0xac8] = 1;
        scene[0x130f] = (fraction + ticks) as u8;
    }
    Ok(())
}

/// Frame de la escena de IA: prefijo fraccional o un tick antes del contacto.
/// Para evitar efectos parciales se rechazan los casos aún no contrastados;
/// los efectos devueltos por el tick pertenecen al consumidor de la sesión.
pub fn advance_populated_ai_frame(
    battle:&mut p3_save::tail::RebuiltInitialCommercialBattle,worlds:&mut Vec<u8>,
    env:crate::battle_runtime::Environment<'_>,delta:u32,rng:&mut Option<p3_core::rng::CrtRand>,
) -> Result<Option<crate::battle_runtime::BattleAdvance>,String> {
    if battle.state.object.len()!=0x2f24 {return Err("escena de asalto truncada".into())}
    let fraction=u32::from(battle.state.object[0x130f]);
    if fraction>15 {return Err("fracción de asalto inválida".into())}
    let total=fraction+u32::from(delta as u16);
    if total<16 {
        advance_ai_prefix(&mut battle.state.object,delta).map_err(str::to_owned)?;
        Ok(None)
    } else if total<32 {
        crate::battle_runtime::advance_town_attack_before_contact(battle,worlds,env,delta,rng).map(Some)
    } else {
        Err("más de un tick de asalto aún no contrastado".into())
    }
}

/// Escribe un único asalto activo y ninguna lista auxiliar de batallas.
/// Se conserva el efecto del escritor derivado sobre una copia de la imagen.
/// No valida la recarga ni publica el bloque en una sesión S05.
pub fn serialize_single_active_attack(
    state: &p3_save::tail::SavedInitialCommercialBattle,
) -> Result<Vec<u8>, &'static str> {
    let mut object = state.object.clone();
    let prefix = p3_core::ships::battle::town_attack_save_prefix(&mut object)?;
    let base = p3_core::ships::battle::serialize_basic_battle(
        &p3_core::ships::battle::BattleSaveInput {
            object: &object, nexts: &state.next, prevs: &state.previous,
            bitmap: &state.bitmap, local_ships: &state.local_ships, projectiles: &state.projectiles,
        })?;
    let mut out = Vec::with_capacity(2 + prefix.len() + base.len() + 4);
    out.extend_from_slice(&[1, 1]);
    out.extend_from_slice(&prefix);
    out.extend_from_slice(&base);
    out.extend_from_slice(&0u32.to_le_bytes());
    Ok(out)
}

/// Prepara una actualización aislada del bloque S05 de una escena activa.
/// El caller debe acreditar y conciliar los demás bloques de la sesión; esta
/// función no habilita por sí sola el guardado o la recarga de Sim/Session.
pub fn stage_active_attack_save(model:&p3_save::model::SaveModel, expected:&[u8],
    state:&p3_save::tail::SavedInitialCommercialBattle,world_before:&[u8],world_after:&[u8])
    -> Result<p3_save::model::SaveModel,String> {
    if model.sea_battles!=expected {return Err("gestor S05 modificado por otro sistema".into())}
    if world_before.len()!=world_after.len() || world_before.len()%0x180!=0
        || world_before.len()/0x180!=model.ships.ships.len() {
        return Err("vistas navales S05 incompatibles".into());
    }
    let encoded=serialize_single_active_attack(state).map_err(str::to_owned)?;
    parse_single_active_attack(&encoded,0x54000000,0)?;
    let mut staged=model.clone();
    for (i,(before,after)) in world_before.chunks_exact(0x180).zip(world_after.chunks_exact(0x180)).enumerate() {
        if before==after {continue}
        let stored=staged.ships.ship_image(i)?;
        let mut updated=stored.clone();
        let mut covered=[false;0x180];
        for segment in p3_save::layout::SHIP_CORE.iter().chain(p3_save::layout::SHIP_REST.iter())
            .chain(std::iter::once(&p3_save::layout::SHIP_STATE)) {
            let start=usize::from(segment.obj);let end=start+usize::from(segment.len);
            covered[start..end].fill(true);
            if before[start..end]!=after[start..end] {
                if stored[start..end]!=before[start..end] && stored[start..end]!=after[start..end] {
                    return Err(format!("conflicto barco {i} +0x{start:X}"));
                }
                updated[start..end].copy_from_slice(&after[start..end]);
            }
        }
        if (0..0x180).any(|j|!covered[j] && before[j]!=after[j]) {
            return Err(format!("cambio no persistido en barco {i}"));
        }
        staged.ships.apply_ship_image(i,&stored,&updated)?;
    }
    staged.sea_battles=encoded;
    let save=staged.to_save();
    let roundtrip=p3_save::Save::parse_unpacked(&save.to_unpacked())
        .map_err(|e|format!("separación S05 del asalto: {e}"))?;
    if roundtrip!=save {return Err("bloques S05 ambiguos en asalto activo".into())}
    Ok(staged)
}

/// Lee los campos persistidos de un ataque activo sin listas auxiliares.
/// La escena aún necesita reconstruir máscara, colisiones y recursos antes
/// de avanzar; esta función no la instala ni habilita la recarga de sesión.
pub fn parse_single_active_attack(
    bytes: &[u8], address: u32, slot: u8,
) -> Result<p3_save::tail::SavedInitialCommercialBattle, String> {
    if bytes.len() < 35 || bytes[..2] != [1, 1] || address == 0 || slot == u8::MAX {
        return Err("bloque de asalto activo incompatible".into());
    }
    let mut basic = vec![1, 0];
    basic.extend_from_slice(&bytes[31..]);
    let parsed = p3_save::tail::CurrentSeaBattles::parse(&basic)
        .map_err(|e| format!("cuerpo de asalto: {e:?}"))?;
    if parsed.objects.len() != 1 || !parsed.pairs.is_empty() {
        return Err("listas auxiliares de asalto no admitidas".into());
    }
    let mut state = parsed.initial_convoy_state(0, address, slot)
        .map_err(|e| format!("estado de asalto: {e:?}"))?;
    state.object.resize(0x2f24, 0);
    state.object[..4].copy_from_slice(&0x0067aaa0u32.to_le_bytes());
    p3_core::ships::battle::town_attack_load_prefix(&mut state.object, &bytes[2..31])
        .map_err(str::to_owned)?;
    if serialize_single_active_attack(&state).map_err(str::to_owned)? != bytes {
        return Err("lectura de asalto no reversible".into());
    }
    Ok(state)
}

/// Reconstruye el prefijo de barcos locales y su índice espacial tras cargar
/// un ataque activo sin proyectiles. Falta reconstruir el mapa de distancias
/// de cada TownAttackShip y la máscara terrestre antes de ejecutar la escena.
pub fn rebuild_loaded_active_attack(
    state: p3_save::tail::SavedInitialCommercialBattle,
    world_ships: &[u8], address: u32,
) -> Result<p3_save::tail::RebuiltInitialCommercialBattle, String> {
    if state.object.len() != 0x2f24 || state.object[0x2f1a] != 1 {
        return Err("estado de ataque urbano sin cargar".into());
    }
    let mut rebuilt = state.rebuild_convoy(world_ships, address)
        .map_err(|e| format!("reconstrucción naval del asalto: {e:?}"))?;
    for ship in &mut rebuilt.state.local_ships {
        ship[..4].copy_from_slice(&0x0067aad0u32.to_le_bytes());
        ship[0x127] = 0;
    }
    Ok(rebuilt)
}

/// Reconstruye también proyectiles guardados, consumiendo el RNG CRT explícito.
/// La semilla se confirma solo si toda la reconstrucción tiene éxito.
pub fn rebuild_loaded_active_attack_with_projectiles(state:p3_save::tail::SavedInitialCommercialBattle,
    world_ships:&[u8],address:u32,rng:&mut p3_core::rng::CrtRand)->Result<p3_save::tail::RebuiltInitialCommercialBattle,String>{
    if state.object.len()!=0x2f24 || state.object[0x2f1a]!=1{return Err("estado de ataque urbano sin cargar".into())}
    let mut trial=*rng;let mut rebuilt=state.rebuild_convoy_with_projectiles(world_ships,address,||trial.rand() as u32).map_err(|e|format!("reconstrucción naval y proyectiles: {e:?}"))?;
    for ship in &mut rebuilt.state.local_ships{ship[..4].copy_from_slice(&0x0067aad0u32.to_le_bytes());ship[0x127]=0;}
    *rng=trial;Ok(rebuilt)
}

/// Construye los 0x1004 bytes transitorios de TownAttackShip al cargar.
/// La caché se reconstruye más tarde, al calcular el rumbo. No instala la
/// escena en sesión ni reconstruye la máscara terrestre del gestor.
pub fn expand_loaded_town_ships(ships:&mut [Vec<u8>]) -> Result<(), &'static str> {
    if ships.iter().any(|s|s.len()!=0x164 || s[..4]!=0x0067aad0u32.to_le_bytes()) {
        return Err("barco urbano recargado incompatible");
    }
    for ship in ships {
        ship.resize(0x1168,0);
        ship[0x1164..0x1168].fill(255);
    }
    Ok(())
}

/// Consulta de agua 0x5FFE20 sobre el plano serializado de ciudad o guarida.
/// La conversión de píxel a casilla corresponde al proveedor de geometría.
pub fn town_attack_cell_is_water(scene:&[u8], plan:&[u8], town_count:u32,
    x:u32, y:u32) -> Result<bool,&'static str> {
    if scene.len()<0x2ee8 {return Err("escena de asalto truncada")}
    let read=|at:usize|u32::from_le_bytes(scene[at..at+4].try_into().unwrap());
    let (x,y)=if u32::from(scene[0x1e98])<town_count {
        (x.wrapping_sub(read(0x1ed8)),y.wrapping_sub(read(0x1edc)))
    } else {(x,y)};
    if x>=read(0x2ee0)||y>=read(0x2ee4) {return Ok(true)}
    if plan.len()<0x64 {return Err("plano de asalto truncado")}
    let stride=u32::from_le_bytes(plan[4..8].try_into().unwrap());
    let offset=x.checked_mul(stride).and_then(|n|n.checked_add(y))
        .and_then(|n|usize::try_from(n).ok()).and_then(|n|n.checked_add(0x64))
        .ok_or("índice de plano inválido")?;
    let tile=*plan.get(offset).ok_or("casilla de asalto ausente")?;
    Ok(matches!(tile,6|12|14|30))
}

/// Prepara la lista de dos convoyes de un asalto, con tierra ya reservada.
/// No coloca barcos, inicia viento ni publica la escena en el gestor.
pub fn build_active_roster(
    ships: &[u8], convoys: &[u8], captains: &[u8], captain_count: u16,
    convoy_ids: [u16;2], participants: [u16;2], address: u32,
) -> Result<([p3_core::ships::battle::LocalBattleSide;2], p3_core::ships::battle::BattleCollisionPool), String> {
    use p3_core::ships::{battle::BattleCollisionPool, pirate_engage::build_pirate_sides_growing};
    if address == 0 || participants[0] >= 0xfe || participants[1] != 0xff {
        return Err("participantes de asalto fuera del caso contrastado".into());
    }
    let mut pool = BattleCollisionPool::with_capacity(0).map_err(str::to_owned)?;
    let land=pool.add_growing(7,0,0,0,0x1100,0x0f02).map_err(str::to_owned)?;
    if land != 0 { return Err("índice inicial de tierra incompatible".into()); }
    let mut sides=build_pirate_sides_growing(ships,convoys,captains,captain_count,
        convoy_ids,participants,address,&mut pool).map_err(str::to_owned)?;
    if sides[0].ships.len()!=1 || sides[1].ships.len()!=1 {
        return Err("se requiere un barco por bando".into());
    }
    for side in &mut sides { for ship in &mut side.ships {
        ship[..4].copy_from_slice(&0x0067aad0u32.to_le_bytes());ship[0x127]=0;
    }}
    Ok((sides,pool))
}

/// Ajusta listas y viento después de construir los dos bandos de IA, antes de
/// la colocación 0x600060. No modifica el gestor ni coloca los barcos.
pub fn prepare_active_wind(scene: &mut [u8], manager: &[u8]) -> Result<(), &'static str> {
    if scene.len()!=0x2f24 || manager.len()<0xf8 || scene[0x5d..0x5f]!=[1,1]
        || scene[0x658..0x65c]!=[1,0,1,0] { return Err("bandos de asalto incompatibles"); }
    let r16=|b:&[u8],i:usize|u16::from_le_bytes([b[i],b[i+1]]);
    let mut out=scene.to_vec();
    out[0x12..0x14].fill(255);
    out[0x65c..0x65e].copy_from_slice(&1u16.to_le_bytes());
    let seed=u32::from_le_bytes(out[0xadc..0xae0].try_into().unwrap());
    let first=p3_core::ships::battle::advance_battle_rng(seed);
    let mut wind=(first&3) as i32+5;
    let x=i32::from(r16(&out,0xad6) as i16);let y=i32::from(r16(&out,0xada) as i16);
    for i in 0..16 {
        let at=0x20+i*12;
        if r16(manager,at)<r16(manager,0xf4) {
            let dx=i32::from(r16(manager,at+4) as i16)-x;
            let dy=i32::from(r16(manager,at+6) as i16)-y;
            if dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy))<=0x384 {
                match r16(manager,at+8) { 0=>wind+=3, 1=>wind-=3, _=>() }
            }
        }
    }
    let second=p3_core::ships::battle::advance_battle_rng(first);
    out[0xadc..0xae0].copy_from_slice(&second.to_le_bytes());
    out[0x670]=second as u8;out[0x671]=wind.clamp(2,10) as u8;
    out[0x5c]=0;out[0x2f18..0x2f1a].fill(0);out[0x15c..0x160].fill(0);
    scene.copy_from_slice(&out);Ok(())
}

/// Colocación 0x600060 para un barco por bando.
/// `is_water` recibe píxeles proyectados, antes de la conversión a casilla.
/// El original puede no terminar si no encuentra agua; aquí se limita cada
/// búsqueda a 256 intentos y la escena solo se confirma si ambas terminan.
pub fn place_two_active_ships<F>(
    scene: &mut [u8], ships: &mut [Vec<u8>], world_ships: &[u8], mut is_water: F,
) -> Result<(), &'static str>
where F: FnMut(i16, i16) -> bool {
    if scene.len()!=0x2f24 || scene[0x658..0x65c]!=[1,0,1,0]
        || ships.len()!=2 || ships.iter().any(|s|s.len()<0x164) {
        return Err("colocación fuera del caso de dos barcos");
    }
    let read_i32=|at| i32::from_le_bytes(scene[at..at+4].try_into().unwrap());
    let vx=read_i32(0x1ed8).checked_neg().and_then(|v|v.checked_mul(2)).ok_or("vector inválido")?;
    let vy=read_i32(0x1edc).checked_neg().ok_or("vector inválido")?;
    // El producto y la norma son aritmética de 32 bits en el original.
    if vx.unsigned_abs()>20000 || vy.unsigned_abs()>20000 { return Err("vector fuera del intervalo contrastado"); }
    let (mut ox,mut oy)=(0x880i32,0x781i32);
    if vx.abs()>vy.abs() {
        ox=if vx<0 { ((-vx*11)/2+0x880)/2 } else { (0x1100-(vx*11)/2+0x880)/2 };
    } else {
        oy=if vy<0 { ((-vy*11)/2+0x781)/2 } else { (0xf02-(vy*11)/2+0x781)/2 };
    }
    let norm=((vx*vx+vy*vy) as f64).sqrt() as i32;
    let (ux,uy)=if norm==0 {(181,181)} else {(vx*256/norm,vy*256/norm)};
    let mut seed=u32::from_le_bytes(scene[0xadc..0xae0].try_into().unwrap());
    let mut staged=ships.to_vec();
    let mut first_position=(0i16,0i16);
    for (index,ship) in staged.iter_mut().enumerate() {
        let id=u32::from_le_bytes(ship[4..8].try_into().unwrap()) as usize;
        let facing=*world_ships.get(id.checked_mul(0x180).and_then(|v|v.checked_add(1)).ok_or("índice de barco inválido")?)
            .ok_or("barco mundial ausente")?;
        let direction=if index==0 {1} else {-1};
        let mut accepted=false;
        for attempt in 0..256 {
            let x=ox+direction*((ux*(300+attempt*200))>>8);
            let y=oy+direction*((uy*(300+attempt*200))>>8);
            seed=p3_core::ships::battle::advance_battle_rng(seed);
            let dx=((seed&127) as i32-64)*uy;
            seed=p3_core::ships::battle::advance_battle_rng(seed);
            let dy=((seed&127) as i32-64)*ux;
            let px=x.wrapping_add(((dx as u32)>>8) as i32) as i16;
            let py=y.wrapping_sub(((dy as u32)>>8) as i32) as i16;
            ship[0xa..0xc].copy_from_slice(&px.to_le_bytes());
            ship[0xe..0x10].copy_from_slice(&py.to_le_bytes());
            ship[0x11]=facing.wrapping_sub(5).wrapping_shl(4);
            // La segunda comprobación del original usa el índice del primer barco.
            if index==0 { first_position=(px,py); }
            let (cx,cy)=first_position;
            if is_water(cx,((i32::from(cy)*11)/17) as i16) { accepted=true;break }
        }
        if !accepted { return Err("búsqueda de agua agotada"); }
    }
    scene[0xadc..0xae0].copy_from_slice(&seed.to_le_bytes());
    ships.clone_from_slice(&staged);
    Ok(())
}

/// Colocación con consulta real del plano; la geometría transforma los
/// píxeles proyectados en casillas. No instala ni avanza la escena.
pub fn place_two_ships_in_plan<F>(scene:&mut [u8], ships:&mut [Vec<u8>],
    world_ships:&[u8], plan:&[u8], town_count:u32, mut pixel_to_cell:F) -> Result<(), &'static str>
where F:FnMut(i16,i16)->(u32,u32) {
    if scene.len()!=0x2f24 || plan.len()<0x64 {return Err("escena o plano truncados")}
    let word=|data:&[u8],at:usize|u32::from_le_bytes(data[at..at+4].try_into().unwrap());
    let (width,height,stride)=(word(scene,0x2ee0),word(scene,0x2ee4),word(plan,4));
    if width>0 && height>0 {
        let last=(width-1).checked_mul(stride).and_then(|n|n.checked_add(height-1))
            .and_then(|n|usize::try_from(n).ok()).and_then(|n|n.checked_add(0x64))
            .ok_or("dimensiones de plano inválidas")?;
        if last>=plan.len() {return Err("plano de asalto incompleto")}
    }
    let bounds=scene.to_vec();
    place_two_active_ships(scene,ships,world_ships,|x,y|{
        let (x,y)=pixel_to_cell(x,y);
        town_attack_cell_is_water(&bounds,plan,town_count,x,y).unwrap_or(false)
    })
}

/// Datos del límite posterior a la construcción de listas del asalto.
/// La imagen de escena debe proceder del mismo gestor y de las mismas tablas.
pub struct PopulatedRosterInput<'a> {
    pub object:&'a [u8],
    pub manager:&'a [u8],
    pub world_ships:&'a [u8],
    pub convoys:&'a [u8],
    pub captains:&'a [u8],
    pub plan:&'a [u8],
    pub address:u32,
    pub town_count:u32,
}

/// Entrada previa a 0x5FC55F, con máscara ya construida y un barco por lado.
pub struct PopulatedStartInput<'a> {
    pub object:&'a [u8],pub manager:&'a [u8],pub world_ships:&'a [u8],
    pub convoys:&'a [u8],pub captains:&'a [u8],pub merchants:&'a [u8],
    pub plan:&'a [u8],pub address:u32,pub town_count:u32,
    pub attacker:u16,pub defender:u16,pub tick:u32,
}

/// Inicia la variante de guarida con dos barcos de IA y sin torres activas.
/// La geometría, posición y retrato proceden de recursos del anfitrión.
/// Devuelve una escena preparada; su publicación debe ser transaccional.
pub fn prepare_populated_opening<F>(input:PopulatedStartInput<'_>,band_index:usize,
    band:&p3_save::pirate_bands::PirateBand,position:(u16,u16),portrait:(u8,u8),
    projection:p3_core::ships::town_attack_mask::TownProjection,pixel_to_cell:F)
    -> Result<p3_save::tail::RebuiltInitialCommercialBattle,String>
where F:FnMut(i16,i16)->(u32,u32) {
    if input.object.len()!=0x2f24 || band_index>=5 {return Err("escena o banda inválida".into())}
    band.validate().map_err(str::to_owned)?;
    if crate::pirate_empty_hideout_scene::has_operational_towers(band)? {
        return Err("torres operativas todavía no admitidas".into());
    }
    if band.plan.as_deref()!=Some(input.plan) {return Err("plano de otra banda".into())}
    let defender_cv=u16::from_le_bytes([band.header[8],band.header[9]]);
    let ship_start=usize::from(input.defender).checked_mul(0x180).ok_or("barco inválido")?;
    let defender=input.world_ships.get(ship_start..ship_start+0x180).ok_or("barco defensor ausente")?;
    if u16::from_le_bytes([defender[8],defender[9]])!=defender_cv {
        return Err("convoy defensor de otra banda".into());
    }
    let mut object=input.object.to_vec();
    object[0x1e98]=254-band_index as u8;object[0x2f1a]=0;object[0x12e8]=1;
    object[0xad2]=2;object[0xad6..0xad8].copy_from_slice(&position.0.to_le_bytes());
    object[0xada..0xadc].copy_from_slice(&position.1.to_le_bytes());
    object[0x1e90]=portrait.0;object[0x1e91]=portrait.1;
    object[0x1e94..0x1e96].fill(0);object[0x1e99..0x1e9d].fill(0);
    object[0x1e9e..0x1eae].fill(255);object[0x1ec4..0x1ed4].fill(0);
    object[0x2f08..0x2f18].fill(0);object[0x660..0x662].copy_from_slice(&defender_cv.to_le_bytes());
    prepare_populated_before_mask(PopulatedStartInput {object:&object,..input},projection,pixel_to_cell)
}

/// Construye la máscara y la primera colisión antes de preparar los barcos.
/// El objeto ya debe estar inicializado por el inicio de ataque de la misión.
pub fn prepare_populated_before_mask<F>(input:PopulatedStartInput<'_>,
    projection:p3_core::ships::town_attack_mask::TownProjection,pixel_to_cell:F)
    -> Result<p3_save::tail::RebuiltInitialCommercialBattle,String>
where F:FnMut(i16,i16)->(u32,u32) {
    if input.object.len()!=0x2f24 || input.plan.len()<0x64
        || !(250..=254).contains(&input.object[0x1e98])
        || input.object[0x54..0x5c]!=[0;8] || input.object[0x1e99..0x1e9d]!=[0;4] {
        return Err("escena anterior a la máscara no admitida".into());
    }
    let width=u32::from_le_bytes(input.plan[..4].try_into().unwrap());
    let height=u32::from_le_bytes(input.plan[4..8].try_into().unwrap());
    let mut pool=p3_core::ships::battle::BattleCollisionPool::with_capacity(0).map_err(str::to_owned)?;
    let land=pool.add_growing(7,0,0,0,0x1100,0x0f02).map_err(str::to_owned)?;
    if land!=0 {return Err("primera colisión terrestre incompatible".into())}
    let mut object=input.object.to_vec();
    p3_core::ships::town_attack_mask::hideout_land_mask(&mut object,width,height,
        &input.plan[0x64..],projection,land).map_err(str::to_owned)?;
    for (at,value) in [(0x20,pool.free_head),(0x34,pool.x_head),(0x44,pool.y_head),
        (0x54,pool.high_water),(0x56,pool.kinds.len() as u16),(0x58,(pool.kinds.len()*2) as u16),
        (0x5a,pool.matrices[0].len() as u16)] {object[at..at+2].copy_from_slice(&value.to_le_bytes())}
    prepare_populated_start(PopulatedStartInput {object:&object,..input},pixel_to_cell)
}

/// Prepara participantes y listas desde el límite posterior a la máscara.
/// Solo cubre dos naves armadas de IA; las direcciones de tablas del original
/// no son utilizables y se conservan como opacas. No publica en sesión.
pub fn prepare_populated_start<F>(input:PopulatedStartInput<'_>,pixel_to_cell:F)
    -> Result<p3_save::tail::RebuiltInitialCommercialBattle,String>
where F:FnMut(i16,i16)->(u32,u32) {
    if input.object.len()!=0x2f24 || input.manager.len()<0xf8 || input.world_ships.len()%0x180!=0
        || input.convoys.len()%0x3c!=0 || input.merchants.len()%0x650!=0 {
        return Err("vistas iniciales de asalto truncadas".into());
    }
    let read=|data:&[u8],at:usize|u16::from_le_bytes([data[at],data[at+1]]);
    let n=usize::from(read(input.manager,0xf4));
    if n!=input.world_ships.len()/0x180 || input.attacker==input.defender
        || usize::from(input.attacker)>=n || usize::from(input.defender)>=n {
        return Err("participantes de asalto inválidos".into());
    }
    let mut convoy_ids=[0u16;2];let mut participants=[0u16;2];
    for (side,id) in [input.attacker,input.defender].into_iter().enumerate() {
        let pos=usize::from(id)*0x180;let ship=&input.world_ships[pos..pos+0x180];
        let guns=i32::from_le_bytes(ship[0x120..0x124].try_into().unwrap());
        if guns<=0 {return Err("barco de asalto sin artillería".into())}
        let owner=ship[0];let convoy=read(ship,8);
        if usize::from(convoy)>=input.convoys.len()/0x3c {return Err("convoy de asalto ausente".into())}
        let head=read(input.convoys,usize::from(convoy)*0x3c+0xa);
        if head!=id {return Err("cabeza de convoy no contrastada".into())}
        if owner!=255 {
            let at=usize::from(owner)*0x650;
            if at+10>input.merchants.len() || read(input.merchants,at+8)==0 {
                return Err("participante humano o ausente".into());
            }
        }
        convoy_ids[side]=convoy;participants[side]=u16::from(owner);
    }
    let (_,pool)=build_active_roster(input.world_ships,input.convoys,input.captains,
        read(input.manager,0xf2),convoy_ids,participants,input.address)?;
    let mut object=input.object.to_vec();
    let write=|v:&mut [u8],at:usize,n:u16|v[at..at+2].copy_from_slice(&n.to_le_bytes());
    for (at,value) in [(0x10,0),(0x12,1),(0x14,2),(0x20,pool.free_head),(0x34,pool.x_head),
        (0x44,pool.y_head),(0x54,pool.high_water),(0x56,pool.kinds.len() as u16),
        (0x58,(pool.kinds.len()*2) as u16),(0x5a,pool.matrices[0].len() as u16),
        (0x35a,participants[0]),(0x35c,participants[1]),(0x658,1),(0x65a,1),
        (0x65c,1),(0x65e,convoy_ids[0]),(0x660,convoy_ids[1])] {write(&mut object,at,value)}
    object[0x558]=2;object[0x559]=0;object[0x55a]=1;object[0x5d]=1;object[0x5e]=1;
    object[0xab0]=1;object[0xab1]=1;
    object[0xa84..0xa88].copy_from_slice(&u32::from(input.attacker).to_le_bytes());
    object[0xa88..0xa8c].copy_from_slice(&u32::from(input.defender).to_le_bytes());
    let seed=input.tick.wrapping_add(u32::from(input.object[0x1e98]))
        .wrapping_add(u32::from(input.attacker)).wrapping_add(u32::from(participants[0]))
        .wrapping_add(u32::from(participants[1]));
    object[0xadc..0xae0].copy_from_slice(&seed.to_le_bytes());
    prepare_populated_roster(PopulatedRosterInput {object:&object,manager:input.manager,
        world_ships:input.world_ships,convoys:input.convoys,captains:input.captains,
        plan:input.plan,address:input.address,town_count:input.town_count},pixel_to_cell)
}

/// Ensambla la escena desde ese límite hasta la colocación, sin publicarla.
/// El consumidor conserva la responsabilidad de generar el objeto inicial,
/// proporcionar geometría y conectar la escena al gestor de sesión.
pub fn prepare_populated_roster<F>(input:PopulatedRosterInput<'_>, pixel_to_cell:F)
    -> Result<p3_save::tail::RebuiltInitialCommercialBattle,String>
where F:FnMut(i16,i16)->(u32,u32) {
    if input.object.len()!=0x2f24 || input.manager.len()<0xf8
        || input.object[..4]!=0x0067aaa0u32.to_le_bytes() || input.object[0x2f1a]!=0
        || input.object[0x14..0x16]!=[2,0] || input.object[0xacc..0xad0]!=[0,0,0,0]
        || input.object[0x10..0x12]!=[0,0] || input.object[0x65c..0x65e]!=[1,0]
        || input.object[0x1e99..0x1e9d]!=[0;4]
        || [0x672usize,0x872].into_iter().any(|base|(0..64).any(|i|input.object[base+i*8..base+i*8+2]!=[255,255])) {
        return Err("imagen inicial de asalto no admitida".into());
    }
    let read=|data:&[u8],at:usize|u16::from_le_bytes([data[at],data[at+1]]);
    let (sides,collisions)=build_active_roster(input.world_ships,input.convoys,input.captains,
        read(input.manager,0xf2),[read(input.object,0x65e),read(input.object,0x660)],
        [read(input.object,0x35a),read(input.object,0x35c)],input.address)?;
    let next=sides.iter().flat_map(|s|s.next.iter().copied()).collect();
    let previous=sides.iter().flat_map(|s|s.previous.iter().copied()).collect();
    let spatial_0=sides.iter().flat_map(|s|s.spatial_0.iter().copied()).collect();
    let spatial_1=sides.iter().flat_map(|s|s.spatial_1.iter().copied()).collect();
    let mut local_ships=sides.into_iter().flat_map(|s|s.ships).collect::<Vec<_>>();
    let mut object=input.object.to_vec();
    prepare_active_wind(&mut object,input.manager).map_err(str::to_owned)?;
    place_two_ships_in_plan(&mut object,&mut local_ships,input.world_ships,input.plan,
        input.town_count,pixel_to_cell).map_err(str::to_owned)?;
    Ok(p3_save::tail::RebuiltInitialCommercialBattle {
        state:p3_save::tail::SavedInitialCommercialBattle {object,next,previous,local_ships,
            bitmap:vec![],projectiles:vec![]},collisions,spatial_0,spatial_1,
    })
}

/// 0x625B00 busca en espiral de Manhattan una casilla de distancia no nula.
/// El original no termina si no existe ninguna; aquí se rechaza ese caso.
pub fn nearest_open_cell(map: &[u8], x: u16, y: u16) -> Result<(u16,u16), &'static str> {
    if map.len()!=4096 || x>=64 || y>=64 { return Err("mapa o casilla inválidos"); }
    if !map.iter().any(|&v|v!=0) { return Err("mapa sin casillas abiertas"); }
    let (mut dx,mut dy)=(0i32,0i32);
    for _ in 0..32768 {
        let xx=i32::from(x)+dx;let yy=i32::from(y)+dy;
        if (0..64).contains(&xx) && (0..64).contains(&yy) && map[(xx*64+yy) as usize]!=0 {
            return Ok((xx as u16,yy as u16));
        }
        if dx==0 {
            if dy>0 { dx-=1;dy-=1; } else { dx=1; }
        } else if dx>0 {
            if dy<0 { dx+=1;dy+=1; } else { dx-=1;dy+=1; }
        } else if dy>0 { dx-=1;dy-=1; } else { dx+=1;dy-=1; }
    }
    Err("búsqueda de casilla fuera del límite")
}

/// Campo de distancias 0x6258A0 para un destino y una máscara 64×64.
/// El original usa costes 2/3 y bytes; se rechazan casos cuyo desbordamiento
/// de distancia aún no está contrastado. La entrada se conserva intacta.
pub fn town_distance_field(water: &[u8], pixel_x: u16, pixel_y: u16) -> Result<([u8;4096],(u16,u16)), &'static str> {
    use std::{cmp::Reverse,collections::BinaryHeap};
    if water.len()!=4096 { return Err("máscara de asalto truncada"); }
    let mut map=[0u8;4096];
    for (dest,&src) in map.iter_mut().zip(water) { *dest=if src==0{0}else{255}; }
    let x=((u32::from(pixel_x)*64/4352) as u16).min(63);
    let y=((u32::from(pixel_y)*64/3842) as u16).min(63);
    let (x,y)=nearest_open_cell(&map,x,y)?;
    map[usize::from(x)*64+usize::from(y)]=1;
    let mut heap=BinaryHeap::new();heap.push(Reverse((1u16,x,y)));
    const DIRS:[(i32,i32,u16);8]=[(0,-1,2),(1,0,2),(0,1,2),(-1,0,2),(1,-1,3),(1,1,3),(-1,1,3),(-1,-1,3)];
    while let Some(Reverse((distance,x,y)))=heap.pop() {
        if u16::from(map[usize::from(x)*64+usize::from(y)])!=distance { continue; }
        for (dx,dy,cost) in DIRS {
            let nx=i32::from(x)+dx;let ny=i32::from(y)+dy;
            if !(0..64).contains(&nx)||!(0..64).contains(&ny) {continue}
            let at=(nx*64+ny) as usize;if water[at]==0 {continue}
            let value=distance+cost;
            if value>255 { return Err("desbordamiento de distancia no contrastado"); }
            if value<u16::from(map[at]) {
                map[at]=value as u8;heap.push(Reverse((value,nx as u16,ny as u16)));
            }
        }
    }
    Ok((map,(x,y)))
}

/// Prefijo de 0x625BF0: actualiza el campo de distancias únicamente cuando
/// cambia la casilla de destino. Los destinos negativos no están contrastados.
/// No calcula todavía las penalizaciones de rumbo del resto de la rutina.
pub fn refresh_town_navigation_map(ship: &mut [u8], water: &[u8]) -> Result<bool, &'static str> {
    if ship.len()!=0x1168 || water.len()!=4096 { return Err("barco o máscara de asalto truncados"); }
    let read=|at:usize|u16::from_le_bytes([ship[at],ship[at+1]]);
    let (x,y)=(read(0x20),read(0x22));let old=(read(0x1164),read(0x1166));
    if (ship[0x12a]&1!=0 || x==u16::MAX) && old.0!=u16::MAX { return Ok(false); }
    if x>i16::MAX as u16 || y>i16::MAX as u16 { return Err("destino negativo sin contrastar"); }
    let cell=(((u32::from(x)*64/4352).min(63)) as u16,
              ((u32::from(y)*64/3842).min(63)) as u16);
    if cell==old { return Ok(false); }
    let (map,_)=town_distance_field(water,x,y)?;
    ship[0x164..0x1164].copy_from_slice(&map);
    ship[0x1164..0x1166].copy_from_slice(&cell.0.to_le_bytes());
    ship[0x1166..0x1168].copy_from_slice(&cell.1.to_le_bytes());
    Ok(true)
}

/// Penalizaciones de rumbo de 0x625BF0, incluidas las dos coronas de costa.
/// Solo admite posiciones no negativas y mapas cuyo cálculo es verificable.
/// Se preserva el barco completo si falla la actualización del mapa.
pub fn update_town_navigation(ship: &mut [u8], water: &[u8]) -> Result<(), &'static str> {
    let mut staged=ship.to_vec();
    refresh_town_navigation_map(&mut staged,water)?;
    let read=|at:usize|u16::from_le_bytes([staged[at],staged[at+1]]);
    let (px,py)=(read(0xa),read(0xe));
    if px>i16::MAX as u16 || py>i16::MAX as u16 {return Err("posición negativa sin contrastar")}
    let x=((u32::from(px)*64/4352).min(63)) as i32;
    let y=((u32::from(py)*64/3842).min(63)) as i32;
    let map=staged[0x164..0x1164].to_vec();
    let at=|dx:i32,dy:i32| -> Option<u8> {
        let (a,b)=(x+dx,y+dy);
        if (0..64).contains(&a)&&(0..64).contains(&b) {Some(map[(a*64+b) as usize])} else {None}
    };
    let current=match at(0,0).unwrap(){0=>255,v=>v};
    let navigating=staged[0x12a]&1==0 && read(0x20)!=u16::MAX;
    let first=[(0,-1),(1,-1),(1,0),(1,1),(0,1),(-1,1),(-1,0),(-1,-1)];
    for (i,(dx,dy)) in first.into_iter().enumerate() {
        let value=at(dx,dy).unwrap_or(255);
        let (level,width,replace)=if value==0 {(0xc0,32,true)}
            else if navigating && value>=current {(0x40,17,false)}
            else if navigating && u16::from(value)+1==u16::from(current) {(0x10,17,false)}
            else {continue};
        let angle=(i*32) as i32;
        for j in 0..width {for a in [angle+j,angle-j] {
            let byte=&mut staged[0x27+(a as u8 as usize)];
            if replace {*byte=level} else {*byte=(*byte).max(level)}
        }}
    }
    let second=[(0,-2),(1,-2),(2,-2),(2,-1),(2,0),(2,1),(2,2),(1,2),
        (0,2),(-1,2),(-2,2),(-2,1),(-2,0),(-2,-1),(-2,-2),(-1,-2)];
    for (i,(dx,dy)) in second.into_iter().enumerate() {
        if at(dx,dy)!=Some(0) {continue}
        let angle=(i*16) as i32;
        for j in 0..16 {for a in [angle+j,angle-j] {
            let byte=&mut staged[0x27+(a as u8 as usize)];*byte=(*byte).max(0x40);
        }}
        for j in 0..48 {for a in [angle+16+j,angle-16-j] {
            let byte=&mut staged[0x27+(a as u8 as usize)];*byte=(*byte).max(0x10);
        }}
    }
    ship.copy_from_slice(&staged);Ok(())
}

/// Reconstrucción provisional al entrar en el primer frame tras cargar.
/// Mantiene la llamada transaccional y no habilita el reloj ni el frame.
/// La geometría de proyección debe proceder de los recursos del anfitrión.
pub fn rebuild_loaded_hideout_geometry(
    battle:&mut p3_save::tail::RebuiltInitialCommercialBattle, plan:&[u8],
    projection:p3_core::ships::town_attack_mask::TownProjection,
) -> Result<(),String> {
    if battle.state.object.len()!=0x2f24 || battle.state.object[0x2f1a]!=1 || plan.len()<0x64 {
        return Err("recarga de guarida incompatible".into());
    }
    let mut staged=battle.clone();
    let o=&mut staged.state.object;
    for i in 0..4usize {
        let x=u16::from(o[0x1eae+i]);let y=u16::from(o[0x1eb2+i]);
        let cx=(2*x+(y&1)+1).wrapping_mul(17);let cy=y.wrapping_mul(17).wrapping_sub(34);
        o[0x1e9e+2*i..0x1ea0+2*i].copy_from_slice(&cx.to_le_bytes());
        o[0x1ea6+2*i..0x1ea8+2*i].copy_from_slice(&cy.to_le_bytes());
        let id=staged.collisions.add_growing(8,i as u8,cx.wrapping_sub(34),cy.wrapping_sub(34),
            cx.wrapping_add(34),cy.wrapping_add(34)).map_err(str::to_owned)?;
        o[0x1eb6+2*i..0x1eb8+2*i].copy_from_slice(&id.to_le_bytes());
    }
    let land=staged.collisions.add_growing(7,0,0,0,0x1100,0x0f02).map_err(str::to_owned)?;
    let width=u32::from_le_bytes(plan[..4].try_into().unwrap());
    let height=u32::from_le_bytes(plan[4..8].try_into().unwrap());
    p3_core::ships::town_attack_mask::hideout_land_mask(o,width,height,&plan[0x64..],projection,land)
        .map_err(str::to_owned)?;
    for (at,value) in [(0x20,staged.collisions.free_head),(0x34,staged.collisions.x_head),
        (0x44,staged.collisions.y_head),(0x54,staged.collisions.high_water),
        (0x56,staged.collisions.kinds.len() as u16),(0x58,(staged.collisions.kinds.len()*2) as u16),
        (0x5a,staged.collisions.matrices[0].len() as u16)] {o[at..at+2].copy_from_slice(&value.to_le_bytes())}
    o[0x2f1a]=0;
    *battle=staged;Ok(())
}

/// Cola transitoria no serializada de TownAttackShip, indexada por barco local.
/// El puerto de navegación recibe el prefijo en el instante de la llamada
/// virtual; no adelanta ni ejecuta por sí solo el paso del barco.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct TownNavigationCaches { tails:Vec<Vec<u8>> }
impl TownNavigationCaches {
    pub fn newly_loaded(count:usize)->Result<Self,&'static str>{
        if count>1024 {return Err("demasiados barcos urbanos")}
        let mut tail=vec![0;0x1004];tail[0x1000..].fill(255);
        Ok(Self{tails:vec![tail;count]})
    }
    pub fn update(&mut self,slot:usize,ship:&mut [u8],water:&[u8])->Result<(),&'static str>{
        let tail=self.tails.get(slot).ok_or("caché de barco urbano ausente")?;
        if ship.len()!=0x164 || tail.len()!=0x1004 {return Err("barco urbano truncado")}
        let mut staged=Vec::with_capacity(0x1168);staged.extend_from_slice(ship);staged.extend_from_slice(tail);
        update_town_navigation(&mut staged,water)?;
        ship.copy_from_slice(&staged[..0x164]);self.tails[slot].copy_from_slice(&staged[0x164..]);Ok(())
    }
    pub fn image(&self,slot:usize,ship:&[u8])->Result<Vec<u8>,&'static str>{
        let tail=self.tails.get(slot).ok_or("caché de barco urbano ausente")?;
        if ship.len()!=0x164 || tail.len()!=0x1004{return Err("barco urbano truncado")}
        let mut result=Vec::with_capacity(0x1168);result.extend_from_slice(ship);result.extend_from_slice(tail);Ok(result)
    }
    /// Actualiza la cola transitoria a partir del resultado de un paso validado.
    pub fn replace_image(&mut self,slot:usize,image:&[u8])->Result<(),&'static str>{
        let tail=self.tails.get_mut(slot).ok_or("caché de barco urbano ausente")?;
        if image.len()!=0x1168 || tail.len()!=0x1004{return Err("imagen de barco urbano truncada")}
        tail.copy_from_slice(&image[0x164..]);Ok(())
    }
}

/// Prueba temprana de 0x600770: una vecindad de 3x3 sin tierra evita
/// el cálculo fino de casco. No resuelve los contactos con tierra.
pub fn town_coast_fast_clear(water:&[u8],x:u16,y:u16)->Result<bool,&'static str>{
    if water.len()!=4096 || x>=4352 || y>=3842 {return Err("máscara o posición de costa inválidas")}
    let gx=i32::from(((u32::from(x)<<6)/4352).min(63) as u16);
    let gy=i32::from(((u32::from(y)<<6)/3842).min(63) as u16);
    Ok((-1..=1).all(|dx|(-1..=1).all(|dy|{
        let (xx,yy)=(gx+dx,gy+dy);
        xx<0||yy<0||xx>=64||yy>=64||water[(xx*64+yy) as usize]!=0
    })))
}

/// Acumulación de la ventana de 5×9 de 0x600770 después de proyectar el
/// barco. `land` y `hull` proceden de consultas externas para cada casilla;
/// `weights` debe leerse de la tabla original 0x67AA70. La primera salida es
/// true cuando ningún punto terrestre tocó el casco. No aplica la respuesta
/// física ni sustituye el contraste de las consultas de casco y proyección.
pub fn town_coast_contact_window(land:&[u8;45],hull:&[u8;45],weights:&[u8;45])->(bool,i32,i32){
    let mut clear=true;let mut dx=0i32;let mut dy=0i32;
    for x in 0..5 {for y in 0..9 {let i=x*9+y;
        if land[i]==0 {continue}
        if hull[i]!=0 {clear=false}
        let w=i32::from(weights[i]);
        if x<2 {dx-=w}else if x>2 {dx+=w}
        if y<4 {dy-=w}else if y>4 {dy+=w}
    }}
    (clear,dx,dy)
}

/// 0x601C7B: solapamiento de casco y casilla terrestre. La proyección y las
/// tablas de seno/contorno proceden del anfitrión y del ejecutable verificado.
/// El cálculo usa los desbordamientos y comparaciones de 32 bits del original.
pub fn town_hull_overlaps_land_cell(ship_xy:(i16,i16),heading:u8,cell:(u16,u16),
    dimensions:(i32,i32),pixel:(u32,u32),shape:&[(i16,i16);5],sine:&[i32;65])->bool {
    let (cx,cy)=(i32::from(cell.0),i32::from(cell.1));
    if cx>dimensions.0 || cy>dimensions.1{return false}
    let px=pixel.0 as i32;let py=pixel.1.wrapping_mul(17).wrapping_div(11) as i32;
    let (x,y)=(i32::from(ship_xy.0),i32::from(ship_xy.1));
    let left=px.wrapping_sub(17).wrapping_sub(py) as u32;
    let right=px.wrapping_add(17).wrapping_sub(py) as u32;
    let sum=x.wrapping_add(y) as u32;let diff=x.wrapping_sub(y) as u32;
    if diff>=left && sum>=px.wrapping_add(py).wrapping_sub(17) as u32
        && sum<=px.wrapping_add(py).wrapping_add(17) as u32 && diff<=right{return true}
    let sin=|a:u8|->i32{let a=usize::from(a);if a<128 {sine[if a<=64{a}else{128-a}]}else{-(sine[if a-128<=64{a-128}else{256-a}])}};
    let (sn,cs)=(sin(heading),sin(heading.wrapping_add(64)));
    let rotate=|(vx,vy):(i16,i16)|->(i32,i32){let (vx,vy)=(i32::from(vx),i32::from(vy));
        (cs.wrapping_mul(vx).wrapping_sub(sn.wrapping_mul(vy)).wrapping_shr(16).wrapping_add(x),
         cs.wrapping_mul(vy).wrapping_add(sn.wrapping_mul(vx)).wrapping_shr(16).wrapping_add(y))};
    let diamond=[(px.wrapping_sub(17),py),(px,py.wrapping_sub(17)),(px.wrapping_add(17),py),(px,py.wrapping_add(17))];
    let mut prev=rotate(shape[4]);
    for point in shape {let next=rotate(*point);let vx=next.0.wrapping_sub(prev.0);let vy=next.1.wrapping_sub(prev.1);
        for edge in 0..4 {let a=diamond[edge];let b=diamond[(edge+1)%4];
            let (ax,ay)=(a.0.wrapping_sub(prev.0),a.1.wrapping_sub(prev.1));
            let (bx,by)=(b.0.wrapping_sub(prev.0),b.1.wrapping_sub(prev.1));
            let cross=|a:i32,b:i32,c:i32,d:i32|a.wrapping_mul(b).wrapping_sub(c.wrapping_mul(d));
            let mut t=cross(vx,ay,vy,ax);let mut denom=cross(vx,ay.wrapping_sub(by),vy,ax.wrapping_sub(bx));
            if denom<0{t=t.wrapping_neg();denom=denom.wrapping_neg()}
            if t>denom || t<0{continue}
            let mut u=cross(ax,by.wrapping_sub(ay),ay,bx.wrapping_sub(ax));
            let mut denom=cross(vx,by.wrapping_sub(ay),vy,bx.wrapping_sub(ax));
            if denom<0{u=u.wrapping_neg();denom=denom.wrapping_neg()}
            if u<=denom && u>=0{return true}
        }prev=next;
    }false
}

/// Ventana fina enlazada a la consulta real del casco. La proyección se
/// aporta desde los recursos del anfitrión y `land` corresponde a sus 45
/// consultas de agua, ordenadas por columna y fila. Rechaza centros cuyo
/// vecindario no cabe en u16; no aplica la respuesta física al barco.
pub fn town_coast_window_with_hull<F>(ship_xy:(i16,i16),heading:u8,center:(u16,u16),
    dimensions:(i32,i32),land:&[u8;45],weights:&[u8;45],shape:&[(i16,i16);5],sine:&[i32;65],
    mut cell_to_pixel:F)->Result<(bool,i32,i32),&'static str>
where F:FnMut(u16,u16)->(u32,u32){
    if center.0<2 || center.0>u16::MAX-2 || center.1<4 || center.1>u16::MAX-4{return Err("ventana de casco fuera de rango")}
    let mut hull=[0u8;45];
    for x in 0..5 {for y in 0..9 {let i=x*9+y;if land[i]==0{continue}
        let cell=(center.0-2+x as u16,center.1-4+y as u16);
        if i32::from(cell.0)<=dimensions.0 && i32::from(cell.1)<=dimensions.1 {
            hull[i]=u8::from(town_hull_overlaps_land_cell(ship_xy,heading,cell,dimensions,cell_to_pixel(cell.0,cell.1),shape,sine));
        }
    }}
    Ok(town_coast_contact_window(land,&hull,weights))
}

/// Reúne consultas de agua y casco sobre el plano de la escena, sin aplicar
/// aún movimiento, daño ni efectos. El anfitrión aporta la proyección real.
pub fn town_coast_window_from_plan<F>(scene:&[u8],plan:&[u8],town_count:u32,
    ship_xy:(i16,i16),heading:u8,center:(u16,u16),weights:&[u8;45],shape:&[(i16,i16);5],
    sine:&[i32;65],cell_to_pixel:F)->Result<(bool,i32,i32),&'static str>
where F:FnMut(u16,u16)->(u32,u32){
    if scene.len()<0x2ee8 || center.0<2 || center.0>u16::MAX-2 || center.1<4 || center.1>u16::MAX-4{return Err("escena o ventana de casco inválida")}
    let read=|at:usize|i32::from_le_bytes(scene[at..at+4].try_into().unwrap());
    let mut land=[0u8;45];for x in 0..5 {for y in 0..9 {
        let (cx,cy)=(u32::from(center.0)-2+x as u32,u32::from(center.1)-4+y as u32);
        land[x*9+y]=u8::from(!town_attack_cell_is_water(scene,plan,town_count,cx,cy)?);
    }}
    town_coast_window_with_hull(ship_xy,heading,center,(read(0x2ee0),read(0x2ee4)),&land,weights,shape,sine,cell_to_pixel)
}

/// Escala de 0x600A2E antes de calcular el ángulo de respuesta. El signo y
/// el redondeo siguen la división entera x86 hacia cero; no mueve el barco.
pub fn town_coast_contact_scaled(clear:bool,dx:i32,dy:i32)->Option<(i32,i32)>{
    if clear || (dx==0 && dy==0){return None}
    Some((dx.wrapping_shl(8).wrapping_add(0x70)/0xe0,
          dy.wrapping_shl(8).wrapping_add(0xac)/0x158))
}

/// 0x6029B0: dirección de un vector, con la tabla original de 65 bytes.
/// Conserva la división y el redondeo enteros del x86.
pub fn town_coast_vector_angle(mut x:i32,mut y:i32,angles:&[u8;65])->u8{
    while !(-32767..=32767).contains(&x){x>>=1;y>>=1}
    if y==0{return if x>0{0x40}else{0xc0}}
    let ratio=(x<<16)/y;
    let q:i32=if ratio<0 {
        if ratio.wrapping_neg()>0x10000 {let n=((128-(0x40000000i32/ratio))>>8) as usize;-(64-i32::from(angles[n]))}
        else {let n=((512-ratio)>>10) as usize;-i32::from(angles[n])}
    }else if ratio>0x10000 {let n=((0x40000000i32/ratio+128)>>8)as usize;64-i32::from(angles[n])}
    else {let n=((ratio+512)>>10)as usize;i32::from(angles[n])};
    if y>0{128i32.wrapping_sub(q) as u8}else{q.wrapping_neg() as u8}
}

/// Respuesta de 0x600770 tras detectar contacto, sin efecto de sonido ni
/// cambio de índice espacial. Modifica posición, rumbo y velocidad local.
pub fn apply_town_coast_response(ship:&mut [u8],angle:u8,direct:bool,sine:&[i32;65])->Result<(),&'static str>{
    if ship.len()<0x20{return Err("barco local truncado")}
    let sin=|a:u8|->i32{let a=usize::from(a);if a<128{sine[if a<=64{a}else{128-a}]}else{sine[if a-128<=64{a-128}else{256-a}].wrapping_neg()}};
    let read=|at:usize|i32::from_le_bytes(ship[at..at+4].try_into().unwrap());
    let speed=read(0x1c);let factor=(speed>>4).wrapping_add(0x2000);let shift=if direct{10}else{12};
    let px=read(8).wrapping_sub(factor.wrapping_mul(sin(angle))>>shift);
    let py=read(12).wrapping_add(factor.wrapping_mul(sin(angle.wrapping_add(64)))>>shift);
    ship[8..12].copy_from_slice(&px.to_le_bytes());ship[12..16].copy_from_slice(&py.to_le_bytes());
    let delta=ship[0x11].wrapping_sub(angle);
    let turn=if delta>0 && delta<0x40{1i32}else if delta>0xc0{-1}else{0};
    if direct {ship[0x11]=ship[0x11].wrapping_add((turn*2) as u8)}
    else {let dir=u16::from_le_bytes([ship[0x10],ship[0x11]]).wrapping_add((turn*0x80) as u16);ship[0x10..0x12].copy_from_slice(&dir.to_le_bytes());}
    ship[0x1c..0x20].copy_from_slice(&speed.wrapping_sub(speed>>(if direct{4}else{6})).to_le_bytes());Ok(())
}

/// Prepara y aplica el contacto de costa 0x600770 para una guarida sin
/// sonido. Toda validación sucede antes de modificar el barco. La proyección
/// pertenece al anfitrión; los recursos numéricos provienen del EXE verificado.
pub fn apply_town_coast_contact<F,G>(scene:&[u8],plan:&[u8],town_count:u32,ship:&mut [u8],
    weights:&[u8;45],shape:&[(i16,i16);5],sine:&[i32;65],angles:&[u8;65],sound_possible:bool,
    mut pixel_to_cell:F,cell_to_pixel:G)->Result<bool,&'static str>
where F:FnMut(i32,i32)->Result<(u16,u16),&'static str>,G:FnMut(u16,u16)->(u32,u32){
    if scene.len()<0x2f24 || ship.len()<0x20{return Err("escena o barco de costa truncado")}
    let read=|at:usize|i16::from_le_bytes([ship[at],ship[at+1]]);
    let (x,y)=(read(0xa),read(0xe));
    if x<0 || y<0 || x>=4352 || y>=3842{return Err("posición de costa no contrastada")}
    if town_coast_fast_clear(&scene[0x1ee0..0x2ee0],x as u16,y as u16)?{return Ok(false)}
    let center=pixel_to_cell(i32::from(x),i32::from(y)*11/17)?;
    let (clear,dx,dy)=town_coast_window_from_plan(scene,plan,town_count,(x,y),ship[0x11],center,weights,shape,sine,cell_to_pixel)?;
    let Some((dx,dy))=town_coast_contact_scaled(clear,dx,dy)else{return Ok(false)};
    if sound_possible{return Err("sonido de contacto de costa pendiente")}
    let angle=town_coast_vector_angle(dx,dy,angles);let mut staged=ship.to_vec();
    apply_town_coast_response(&mut staged,angle,scene[0xac8]!=0,sine)?;ship.copy_from_slice(&staged);Ok(true)
}

/// 0x5FE080: recarga de una pieza y retorno con lista atacante vacía.
/// Con recarga cero y atacantes devuelve error sin cambiar la escena: selección
/// de blanco, disparo y efectos aún no están conectados.
pub fn town_defense_piece_cooldown(scene:&mut [u8],piece:usize)->Result<(),&'static str>{
    if scene.len()<0x1ec2 || piece>=4{return Err("pieza de defensa truncada")}
    let at=0x1ebe+piece;let cooldown=scene[at];
    if cooldown==0 {if u16::from_le_bytes([scene[0x10],scene[0x11]])!=u16::MAX{return Err("selección de blanco de torre pendiente")}return Ok(())}
    scene[at]=cooldown.saturating_sub(if scene[0xac8]!=0{16}else{1});Ok(())
}

/// Añade las piezas vivas a las estadísticas del bando defensor, antes de
/// calcular su centro. `count` es el contador de barcos de ese bando. Esta
/// operación aislada no ejecuta un tick ni publica cambios en la sesión.
pub fn town_defense_statistics(scene:&mut [u8],mut count:u8,power:u8,divisor:u8)->Result<u8,&'static str>{
    if scene.len()<0x1ed4 || divisor==0 {return Err("estadísticas de defensa inválidas")}
    fn get(scene:&[u8],at:usize)->u32 {u32::from_le_bytes(scene[at..at+4].try_into().unwrap())}
    fn put(scene:&mut [u8],at:usize,value:u32) {scene[at..at+4].copy_from_slice(&value.to_le_bytes())}
    for piece in 0..4 {
        let health=get(scene,0x1ec4+4*piece);
        if health==0 {continue}
        let strength=u32::from(scene[0x1e99+piece])*u32::from(power)*64/u32::from(divisor);
        put(scene,0xa94,get(scene,0xa94).wrapping_add(health.wrapping_mul(strength)>>10));
        put(scene,0xaa4,get(scene,0xaa4).max(strength));
        let x=u16::from_le_bytes(scene[0x1e9e+2*piece..0x1ea0+2*piece].try_into().unwrap());
        let y=u16::from_le_bytes(scene[0x1ea6+2*piece..0x1ea8+2*piece].try_into().unwrap());
        put(scene,0xab8,get(scene,0xab8).wrapping_add(u32::from(x)));
        put(scene,0xac0,get(scene,0xac0).wrapping_add(u32::from(y)));
        count=count.wrapping_add(1);
    }
    Ok(count)
}

/// Predicción de cuatro iteraciones de 0x5FE080; conserva el truncado y el
/// desbordamiento de posición de 16 bits. Rechaza cuadrados desbordados.
fn predict_tower_target(scene:&[u8],piece:usize,ship:&[u8])->Result<(i16,i16,i32),&'static str>{
    let data=ship.get(..0x1c).ok_or("barco atacante truncado")?;
    let word=|b:&[u8],at:usize|i16::from_le_bytes([b[at],b[at+1]]);
    let dword=|at:usize|i32::from_le_bytes(data[at..at+4].try_into().unwrap());
    let (mut dx,mut dy)=(word(data,0xa).wrapping_sub(word(scene,0x1e9e+piece*2)),word(data,0xe).wrapping_sub(word(scene,0x1ea6+piece*2)));
    let (base_x,base_y)=(dx,dy);let (mut vx,mut vy)=(dword(0x14),dword(0x18));if scene[0xac8]!=0{vx>>=4;vy>>=4}
    for _ in 0..4 {let (x,y)=(i32::from(dx),i32::from(dy));let square=x.checked_mul(x).and_then(|v|y.checked_mul(y).and_then(|w|v.checked_add(w))).ok_or("distancia de torre no contrastada")?;
        let distance=(square as u32).isqrt() as i32;let product=i64::from(distance.wrapping_shl(8))*i64::from(0x3159721fi32);let high=(product>>32)as i32;let q=high>>8;let time=q.wrapping_add(((q as u32)>>31)as i32);
        dx=base_x.wrapping_add((vx.wrapping_mul(time)>>16)as i16);dy=base_y.wrapping_add((vy.wrapping_mul(time)>>16)as i16);
    }
    let (x,y)=(i32::from(dx),i32::from(dy));let distance=x.checked_mul(x).and_then(|v|y.checked_mul(y).and_then(|w|v.checked_add(w))).ok_or("distancia de torre no contrastada")?;
    Ok((dx,dy,distance))
}

/// Selecciona el atacante más cercano sin defensores en la línea de tiro.
/// Esta consulta no crea proyectiles, aplica daño ni modifica la escena.
pub fn town_defense_select_unopposed(scene:&[u8],piece:usize,ships:&[Vec<u8>],next:&[u16])->Result<Option<(u16,u16,u16,i32)>,&'static str>{
    if scene.len()<0x1ec2 || piece>=4{return Err("pieza de defensa truncada")}
    if scene[0x1ebe+piece]!=0{return Err("pieza de defensa en recarga")}
    if u16::from_le_bytes([scene[0x65c],scene[0x65d]])!=u16::MAX{return Err("defensores de torre no contrastados")}
    let mut seen=vec![false;ships.len()];let mut id=u16::from_le_bytes([scene[0x10],scene[0x11]]);let mut selected=None;let mut best=0x7e900;
    while id!=u16::MAX {let i=usize::from(id);if i>=ships.len() || i>=next.len() || seen[i]{return Err("lista atacante inválida")}
        seen[i]=true;let ship=&ships[i];let flags=*ship.get(0x12a).ok_or("barco atacante truncado")?;
        if flags&6==0{let (dx,dy,distance)=predict_tower_target(scene,piece,ship)?;if distance<=best{
            let x=u16::from_le_bytes([scene[0x1e9e+2*piece],scene[0x1e9f+2*piece]]).wrapping_add(dx as u16);
            let y=u16::from_le_bytes([scene[0x1ea6+2*piece],scene[0x1ea7+2*piece]]).wrapping_add(dy as u16);
            selected=Some((id,x,y,distance));best=distance;
        }}id=next[i];
    }
    Ok(selected.filter(|(_,x,_,_)|*x!=u16::MAX))
}

/// Selección 0x5FE080 con defensores, usando la tabla angular original.
/// No produce disparos ni cambia la escena. Rechaza listas corruptas y
/// distancias cuyo cuadrado no cabe en el entero firmado del original.
pub fn town_defense_select_with_defenders(scene:&[u8],piece:usize,ships:&[Vec<u8>],next:&[u16],angles:&[u8;65])
    ->Result<Option<(u16,u16,u16,i32)>,&'static str>{
    if scene.len()<0x1ec2 || piece>=4{return Err("pieza de defensa truncada")}
    if scene[0x1ebe+piece]!=0{return Err("pieza de defensa en recarga")}
    let word=|at:usize|u16::from_le_bytes([scene[at],scene[at+1]]);
    let (tx,ty)=(word(0x1e9e+piece*2),word(0x1ea6+piece*2));
    let mut seen=vec![false;ships.len()];let mut id=word(0x10);let mut selected=None;let mut best=0x7e900;
    while id!=u16::MAX {let i=usize::from(id);if i>=ships.len() || i>=next.len() || seen[i]{return Err("lista atacante inválida")}
        seen[i]=true;let ship=&ships[i];let flags=*ship.get(0x12a).ok_or("barco atacante truncado")?;
        if flags&6==0 {let (dx,dy,distance)=predict_tower_target(scene,piece,ship)?;
            if distance<=best {let mut defense=word(0x65c);let mut defenders=vec![false;ships.len()];let mut blocked=false;
                while defense!=u16::MAX {let j=usize::from(defense);if j>=ships.len() || j>=next.len() || defenders[j]{return Err("lista defensora inválida")}
                    defenders[j]=true;let d=ships[j].get(..0x10).ok_or("barco defensor truncado")?;
                    let x=i32::from(i16::from_le_bytes([d[0xa],d[0xb]]).wrapping_sub(tx as i16));
                    let y=i32::from(i16::from_le_bytes([d[0xe],d[0xf]]).wrapping_sub(ty as i16));
                    let sq=x.checked_mul(x).and_then(|v|y.checked_mul(y).and_then(|w|v.checked_add(w))).ok_or("distancia defensora no contrastada")?;
                    let radius=(sq as u32).isqrt() as i32;let threshold=if radius<32{128}else{4000/radius};
                    // El original calcula la primera dirección con X en ambos
                    // operandos; se conserva y se contrasta, sin atribuir intención.
                    let delta=town_coast_vector_angle(x,x,angles).wrapping_sub(town_coast_vector_angle(i32::from(dx),i32::from(dy),angles));
                    let gap=if delta>128{delta.wrapping_neg()}else{delta};
                    if i32::from(gap)<=threshold && radius>=(distance as u32).isqrt() as i32-80{blocked=true}
                    defense=next[j];
                }
                if !blocked {selected=Some((id,tx.wrapping_add(dx as u16),ty.wrapping_add(dy as u16),distance));best=distance}
            }
        }id=next[i];
    }
    Ok(selected.filter(|(_,x,_,_)|*x!=u16::MAX))
}

/// Prepara el disparo instantáneo de 0x5FE080 sin defensores ni participantes
/// humanos: devuelve blanco y daño y actualiza la recarga solo en la copia.
/// No aplica daño ni publica el resultado en el frame o S05.
pub fn stage_unopposed_tower_instant(scene:&[u8],piece:usize,ships:&[Vec<u8>],next:&[u16],power:u8)
    ->Result<Option<(Vec<u8>,u16,i32)>,&'static str>{
    if scene.len()<0x1ec2{return Err("pieza de defensa truncada")}
    if u16::from_le_bytes([scene[0x35a],scene[0x35b]])!=0xff || u16::from_le_bytes([scene[0x35c],scene[0x35d]])!=0xff{return Err("participantes de torre no proporcionados")}
    stage_tower_instant_with_humans(scene,piece,ships,next,power,[false;2],1)
}

/// Rama instantánea sin defensores, con condición humana y dificultad obtenidas
/// por el anfitrión. Las dos escalas se aplican en orden con truncado x86.
pub fn stage_tower_instant_with_humans(scene:&[u8],piece:usize,ships:&[Vec<u8>],next:&[u16],power:u8,human:[bool;2],difficulty:u8)
    ->Result<Option<(Vec<u8>,u16,i32)>,&'static str>{
    stage_tower_instant_inner(scene,piece,ships,next,power,human,difficulty,None)
}

fn stage_tower_instant_inner(scene:&[u8],piece:usize,ships:&[Vec<u8>],next:&[u16],power:u8,human:[bool;2],difficulty:u8,angles:Option<&[u8;65]>)
    ->Result<Option<(Vec<u8>,u16,i32)>,&'static str>{
    let Some((scene,target,damage,_,_))=stage_tower_shot(scene,piece,ships,next,power,human,difficulty,angles,true)? else{return Ok(None)};
    Ok(Some((scene,target,damage)))
}

fn stage_tower_shot(scene:&[u8],piece:usize,ships:&[Vec<u8>],next:&[u16],power:u8,human:[bool;2],difficulty:u8,angles:Option<&[u8;65]>,instant:bool)
    ->Result<Option<(Vec<u8>,u16,i32,u16,u16)>,&'static str>{
    if scene.len()<0x1ec2 || piece>=4{return Err("pieza de defensa truncada")}
    if (scene[0xac8]!=0)!=instant{return Err("modo de disparo de torre incompatible")}
    for (i,is_human) in human.iter().enumerate(){
        let at=0x35a+2*i;
        if *is_human && u16::from_le_bytes([scene[at],scene[at+1]])==0xff{return Err("participante humano ausente")}
    }
    let selected=if let Some(table)=angles{town_defense_select_with_defenders(scene,piece,ships,next,table)?}else{town_defense_select_unopposed(scene,piece,ships,next)?};
    let Some((target,x,y,square))=selected else{return Ok(None)};
    let strength=i32::from(scene[0x1e99+piece]).wrapping_mul(i32::from(power));let distance=(square as u32).isqrt()as i32;
    let product=strength.wrapping_mul(distance);let high=((i64::from(0x49f49f49i32)*i64::from(product))>>32)as i32;
    let q=high.wrapping_sub(product)>>9;let mut adjusted=strength.wrapping_add(q.wrapping_add(((q as u32)>>31)as i32));
    for (side,is_human) in human.iter().enumerate(){if *is_human {
        adjusted=match (side,difficulty){(0,0)|(1,2)=>adjusted.wrapping_mul(11)/10,(0,2)|(1,0)=>adjusted.wrapping_mul(19)/20,_=>adjusted};
    }}
    let scaled=adjusted.wrapping_mul(2800);let high=((i64::from(0x51eb851fi32)*i64::from(scaled))>>32)as i32;let q=high>>5;let mut damage=q.wrapping_add(((q as u32)>>31)as i32);if damage==0{damage=1}
    let mut staged=scene.to_vec();staged[0x1ebe+piece]=0x96;Ok(Some((staged,target,damage,x,y)))
}

/// Prepara argumentos de 0x602A90 para el disparo visible de torre. El ID,
/// asignador, proyectil, índice y RNG se ejecutan después por el anfitrión.
pub fn stage_tower_projectile(scene:&[u8],piece:usize,ships:&[Vec<u8>],next:&[u16],power:u8,human:[bool;2],difficulty:u8,angles:Option<&[u8;65]>,id:u16)
    ->Result<Option<(Vec<u8>,u16,p3_core::ships::projectile::InitArgs)>,&'static str>{
    let Some((staged,target,damage,x,y))=stage_tower_shot(scene,piece,ships,next,power,human,difficulty,angles,false)? else{return Ok(None)};
    let tx=u16::from_le_bytes([scene[0x1e9e+2*piece],scene[0x1e9f+2*piece]]);let ty=u16::from_le_bytes([scene[0x1ea6+2*piece],scene[0x1ea7+2*piece]]);
    let sx=if i32::from(x as i16)<i32::from(tx){tx.wrapping_sub(17)}else{tx.wrapping_add(17)};
    let sy=if i32::from(y as i16)<i32::from(ty){ty.wrapping_sub(17)}else{ty.wrapping_add(17)};
    let args=p3_core::ships::projectile::InitArgs{id,dest:(x as i16,y as i16),src:(sx as i16,sy as i16),damage,kind:1,shooter:u16::MAX,tag:0x1c};
    Ok(Some((staged,target,args)))
}

/// Construye en copias el proyectil no instantáneo, su índice y RNG CRT.
/// Aún no publica en Session ni despacha efectos visuales.
pub fn stage_tower_projectile_spawn(scene:&[u8],piece:usize,ships:&[Vec<u8>],next:&[u16],power:u8,human:[bool;2],difficulty:u8,angles:Option<&[u8;65]>,
    address:u32,table:&p3_core::ships::projectile::ProjectileTable,index:&p3_core::ships::spatial::CollisionIndex,rng:p3_core::rng::CrtRand)
    ->Result<Option<(Vec<u8>,p3_core::ships::projectile::ProjectileTable,p3_core::ships::spatial::CollisionIndex,p3_core::rng::CrtRand)>,&'static str>{
    if address==0 || scene.len()<0x1ec2{return Err("escena de proyectil inválida")}
    crate::battle_spatial::into_saved(index)?;
    let count=u16::from_le_bytes([scene[0xacc],scene[0xacd]]) as usize;let words=u16::from_le_bytes([scene[0xace],scene[0xacf]]) as usize;
    if count!=table.slots.len() || words!=table.bitmap.len() || words!=(count+31)/32 || count%32!=0 || count>256{return Err("tabla de proyectiles inconsistente")}
    for (i,slot) in table.slots.iter().enumerate(){if slot.is_some()!=((table.bitmap[i/32]>>(i%32))&1!=0){return Err("bitmap de proyectiles inconsistente")}}
    for (at,value) in [(0x20,index.free_head),(0x34,index.x.head),(0x44,index.y.head),(0x54,index.span)]{if u16::from_le_bytes([scene[at],scene[at+1]])!=value{return Err("cabecera espacial desincronizada")}}
    let Some((mut out,_,mut args))=stage_tower_projectile(scene,piece,ships,next,power,human,difficulty,angles,0)? else{return Ok(None)};
    let mut staged=table.clone();let mut spatial=index.clone();let mut random=rng;args.id=staged.allocate_id(&mut out);if args.id>=256{return Err("índice de proyectil fuera de rango")}
    let projectile=p3_core::ships::projectile::init(&mut staged,address,false,&mut spatial,&mut ||random.rand() as u32,args)?;staged.slots[usize::from(args.id)]=Some(projectile);
    for (at,value) in [(0x20,spatial.free_head),(0x34,spatial.x.head),(0x44,spatial.y.head),(0x54,spatial.span)]{out[at..at+2].copy_from_slice(&value.to_le_bytes());}
    Ok(Some((out,staged,spatial,random)))
}

/// Avanza un proyectil de torre en copias, sin vista ni efectos gráficos.
/// Devuelve si alcanzó el destino y fue retirado; no despacha colisiones.
pub fn stage_tower_projectile_advance(scene:&[u8],id:usize,table:&p3_core::ships::projectile::ProjectileTable,index:&p3_core::ships::spatial::CollisionIndex)
    ->Result<(Vec<u8>,p3_core::ships::projectile::ProjectileTable,p3_core::ships::spatial::CollisionIndex,bool),&'static str>{
    if scene.len()<0x1ec2{return Err("escena de proyectil truncada")}
    crate::battle_spatial::into_saved(index)?;
    let count=u16::from_le_bytes([scene[0xacc],scene[0xacd]]) as usize;let words=u16::from_le_bytes([scene[0xace],scene[0xacf]]) as usize;
    if count!=table.slots.len() || words!=table.bitmap.len() || words!=(count+31)/32 || count%32!=0 || id>=count{return Err("tabla de proyectiles inconsistente")}
    for (i,slot) in table.slots.iter().enumerate(){if slot.is_some()!=((table.bitmap[i/32]>>(i%32))&1!=0){return Err("bitmap de proyectiles inconsistente")}}
    let p=table.slots[id].as_ref().ok_or("proyectil ausente")?;if p[0x18]!=1 || u16::from_le_bytes([p[0x24],p[0x25]])!=u16::MAX || !matches!(u16::from_le_bytes([p[0x34],p[0x35]]),0x1c|8) || usize::from(u16::from_le_bytes([p[0x2c],p[0x2d]]))!=id{return Err("proyectil de torre incompatible")}
    let slot=u16::from_le_bytes([p[0x2e],p[0x2f]]);if !crate::battle_runtime::can_remove(index,slot){return Err("índice de proyectil inválido")}
    for (at,value) in [(0x20,index.free_head),(0x34,index.x.head),(0x44,index.y.head),(0x54,index.span)]{if u16::from_le_bytes([scene[at],scene[at+1]])!=value{return Err("cabecera espacial desincronizada")}}
    let mut out=scene.to_vec();let mut staged=table.clone();let mut spatial=index.clone();let ended=match p3_core::ships::projectile::advance(&mut staged,id,scene[0xac8]!=0,false,&mut spatial){p3_core::ships::projectile::Advance::Moved=>false,p3_core::ships::projectile::Advance::Finished{view_effect:false,graphic:0}=>true,_=>return Err("efecto de proyectil pendiente")};
    if ended{for (at,value) in [(0x20,spatial.free_head),(0x34,spatial.x.head),(0x44,spatial.y.head),(0x54,spatial.span)]{out[at..at+2].copy_from_slice(&value.to_le_bytes());}}
    Ok((out,staged,spatial,ended))
}

/// Resultado aislado de una pareja barco–proyectil de torre.
pub struct TowerShipImpact {
    pub scene:Vec<u8>,pub ships:Vec<Vec<u8>>,pub worlds:Vec<u8>,
    pub projectiles:p3_core::ships::projectile::ProjectileTable,
    pub spatial:p3_core::ships::spatial::CollisionIndex,pub notices:Vec<(u16,u16)>,pub mask:u8,
}

/// Comprueba casco e impacto de un proyectil de torre, sin vista. Una pareja
/// que no toca devuelve `None`; las bajas y el impacto se preparan en copias.
pub fn stage_tower_projectile_ship_impact(scene:&[u8],ships:&[Vec<u8>],worlds:&[u8],near:&[u16],far:&[u16],
    table:&p3_core::ships::projectile::ProjectileTable,index:&p3_core::ships::spatial::CollisionIndex,ship_id:u16,projectile_id:u16)
    ->Result<Option<TowerShipImpact>,&'static str>{
    use p3_core::ships::{hull::{hit_mask,HullQuery},projectile::{self,Spatial},weapon_damage::{self,DamageEvent}};
    if scene.len()<0x1ec2{return Err("escena de impacto truncada")}
    crate::battle_spatial::into_saved(index)?;
    let count=u16::from_le_bytes([scene[0xacc],scene[0xacd]]) as usize;let words=u16::from_le_bytes([scene[0xace],scene[0xacf]]) as usize;
    if count!=table.slots.len() || words!=table.bitmap.len() || words!=(count+31)/32 || count%32!=0 || usize::from(projectile_id)>=count{return Err("tabla de proyectiles inconsistente")}
    for (i,slot) in table.slots.iter().enumerate(){if slot.is_some()!=((table.bitmap[i/32]>>(i%32))&1!=0){return Err("bitmap de proyectiles inconsistente")}}
    let p=table.slots[usize::from(projectile_id)].as_ref().ok_or("proyectil ausente")?;if p[0x18]!=1 || u16::from_le_bytes([p[0x24],p[0x25]])!=u16::MAX || !matches!(u16::from_le_bytes([p[0x34],p[0x35]]),0x1c|8) || u16::from_le_bytes([p[0x2c],p[0x2d]])!=projectile_id{return Err("proyectil de torre incompatible")}
    let ship=ships.get(usize::from(ship_id)).ok_or("barco de impacto ausente")?;if ship.len()<0x164 || ship[0x13d]>1{return Err("barco de impacto inválido")}
    if ship[0x12a]&2!=0{return Ok(None)}
    let world_id=u32::from_le_bytes(ship[4..8].try_into().unwrap()) as usize;let offset=world_id.checked_mul(0x180).ok_or("barco mundial inválido")?;let world=worlds.get(offset..offset.checked_add(0x180).ok_or("barco mundial inválido")?).ok_or("barco mundial ausente")?;let kind=usize::from(world[0xe]);if kind>=4{return Err("tipo de casco inválido")}
    let projectile_slot=u16::from_le_bytes([p[0x2e],p[0x2f]]);let ship_slot=*near.get(usize::from(ship_id)).ok_or("índice naval ausente")?;let ship_far=*far.get(usize::from(ship_id)).ok_or("índice naval ausente")?;
    for (slot,kind,reference) in [(ship_slot,0,ship_id),(projectile_slot,1,projectile_id)]{let i=usize::from(slot);if i>=index.capacity || index.types[i]!=kind || index.references[i]!=reference || !crate::battle_runtime::can_remove(index,slot){return Err("pareja espacial inválida")}}
    for (at,value) in [(0x20,index.free_head),(0x34,index.x.head),(0x44,index.y.head),(0x54,index.span)]{if u16::from_le_bytes([scene[at],scene[at+1]])!=value{return Err("cabecera espacial desincronizada")}}
    let read=|b:&[u8],at:usize|u32::from_le_bytes(b[at..at+4].try_into().unwrap());let mask=hit_mask(&HullQuery{hull_type:kind,heading:ship[0x11],ship:(read(ship,8),read(ship,12)),projectile:(read(p,4),read(p,8)),velocity:(read(p,12)as i32,read(p,16)as i32),direct:scene[0xac8]!=0});if mask==0{return Ok(None)}
    struct Deferred(Vec<u16>);impl Spatial for Deferred{fn bounds(&mut self,_:u16,_:i16,_:i16,_:i16,_:i16){unreachable!()}fn remove(&mut self,id:u16){self.0.push(id)}fn add(&mut self,_:u8,_:u8,_:i16,_:i16,_:i16,_:i16)->Option<u16>{unreachable!()}}
    let mut scene=scene.to_vec();let mut ships=ships.to_vec();let mut worlds=worlds.to_vec();let mut projectiles=table.clone();let mut spatial=index.clone();let mut damage_events=Vec::new();let mut error=None;let mut deferred=Deferred(Vec::new());let mut effects=Vec::new();
    let graphic=projectile::impact(&mut projectiles,usize::from(projectile_id),ship_id,&mut scene,&|_|(0,0),&mut deferred,false,|battle,amount|match weapon_damage::apply_weapon_damage(&mut worlds[offset..offset+0x180],&mut ships[usize::from(ship_id)],battle,(ship_slot,ship_far),amount,-1,mask,&mut damage_events){Ok(v)=>v,Err(e)=>{error=Some(e);0}},&mut effects);
    if let Some(e)=error{return Err(e)}if graphic!=0 || !effects.is_empty(){return Err("efecto visual de impacto pendiente")}
    let mut notices=Vec::new();for event in damage_events{match event{DamageEvent::RecalculateEquipment=>{},DamageEvent::RemoveSpatial(slot)=>{if !crate::battle_runtime::can_remove(&spatial,slot){return Err("baja espacial inválida")}spatial.remove(slot)},DamageEvent::SinkNotice(captain)=>{let side=usize::from(ships[usize::from(ship_id)][0x13d]);let at=0x35a+2*(1-side);notices.push((captain,u16::from_le_bytes([scene[at],scene[at+1]])));}}}
    for slot in deferred.0{if !crate::battle_runtime::can_remove(&spatial,slot){return Err("baja de proyectil inválida")}spatial.remove(slot)}
    for (at,value) in [(0x20,spatial.free_head),(0x34,spatial.x.head),(0x44,spatial.y.head),(0x54,spatial.span)]{scene[at..at+2].copy_from_slice(&value.to_le_bytes());}
    Ok(Some(TowerShipImpact{scene,ships,worlds,projectiles,spatial,notices,mask}))
}

/// Preparación transaccional de un disparo instantáneo y su daño sin efectos
/// externos. Hundimientos, recalcular equipo y avisos requieren otra ruta.
pub fn stage_unopposed_tower_damage(scene:&[u8],piece:usize,ships:&[Vec<u8>],worlds:&[u8],next:&[u16],
    near:&[u16],far:&[u16],power:u8)->Result<Option<(Vec<u8>,Vec<Vec<u8>>,Vec<u8>)>,&'static str>{
    if scene.len()<0x1ec2{return Err("pieza de defensa truncada")}
    if u16::from_le_bytes([scene[0x35a],scene[0x35b]])!=0xff || u16::from_le_bytes([scene[0x35c],scene[0x35d]])!=0xff{return Err("participantes de torre no proporcionados")}
    stage_tower_damage_with_humans(scene,piece,ships,worlds,next,near,far,power,[false;2],1)
}

/// Prepara la rama instantánea con dificultad humana, sin publicar efectos externos.
pub fn stage_tower_damage_with_humans(scene:&[u8],piece:usize,ships:&[Vec<u8>],worlds:&[u8],next:&[u16],
    near:&[u16],far:&[u16],power:u8,human:[bool;2],difficulty:u8)->Result<Option<(Vec<u8>,Vec<Vec<u8>>,Vec<u8>)>,&'static str>{
    stage_tower_damage_inner(scene,piece,ships,worlds,next,near,far,power,human,difficulty,None)
}

/// Disparo instantáneo con selección defensora y dificultad obtenidas por el anfitrión.
pub fn stage_tower_damage_with_defenders(scene:&[u8],piece:usize,ships:&[Vec<u8>],worlds:&[u8],next:&[u16],
    near:&[u16],far:&[u16],power:u8,human:[bool;2],difficulty:u8,angles:&[u8;65])->Result<Option<(Vec<u8>,Vec<Vec<u8>>,Vec<u8>)>,&'static str>{
    stage_tower_damage_inner(scene,piece,ships,worlds,next,near,far,power,human,difficulty,Some(angles))
}

fn stage_tower_damage_inner(scene:&[u8],piece:usize,ships:&[Vec<u8>],worlds:&[u8],next:&[u16],
    near:&[u16],far:&[u16],power:u8,human:[bool;2],difficulty:u8,angles:Option<&[u8;65]>)->Result<Option<(Vec<u8>,Vec<Vec<u8>>,Vec<u8>)>,&'static str>{
    let Some((staged,locals,world,events,_))=stage_tower_damage_events(scene,piece,ships,worlds,next,near,far,power,human,difficulty,angles)? else{return Ok(None)};
    if events.iter().any(|e|!matches!(e,p3_core::ships::weapon_damage::DamageEvent::RecalculateEquipment)){return Err("efectos de daño de torre pendientes")}
    Ok(Some((staged,locals,world)))
}

/// Hundimiento sin aviso externo: aplica las bajas al índice espacial clonado.
pub fn stage_tower_damage_with_spatial(scene:&[u8],piece:usize,ships:&[Vec<u8>],worlds:&[u8],next:&[u16],
    near:&[u16],far:&[u16],power:u8,human:[bool;2],difficulty:u8,angles:Option<&[u8;65]>,index:&p3_core::ships::spatial::CollisionIndex)
    ->Result<Option<(Vec<u8>,Vec<Vec<u8>>,Vec<u8>,p3_core::ships::spatial::CollisionIndex)>,&'static str>{
    let Some((scene,ships,world,index,notices))=stage_tower_damage_with_notices(scene,piece,ships,worlds,next,near,far,power,human,difficulty,angles,index)? else{return Ok(None)};
    if !notices.is_empty(){return Err("aviso de hundimiento pendiente")}
    Ok(Some((scene,ships,world,index)))
}

/// Prepara bajas y datos del aviso; el anfitrión debe crear y persistir la noticia.
pub fn stage_tower_damage_with_notices(scene:&[u8],piece:usize,ships:&[Vec<u8>],worlds:&[u8],next:&[u16],
    near:&[u16],far:&[u16],power:u8,human:[bool;2],difficulty:u8,angles:Option<&[u8;65]>,index:&p3_core::ships::spatial::CollisionIndex)
    ->Result<Option<(Vec<u8>,Vec<Vec<u8>>,Vec<u8>,p3_core::ships::spatial::CollisionIndex,Vec<(u16,u16)>)>,&'static str>{
    use p3_core::ships::weapon_damage::DamageEvent;
    let Some((mut staged,locals,world,events,target))=stage_tower_damage_events(scene,piece,ships,worlds,next,near,far,power,human,difficulty,angles)? else{return Ok(None)};
    let mut notices=Vec::new();let mut spatial=index.clone();let mut removed=false;for event in events {match event{
        DamageEvent::RecalculateEquipment=>{},
        DamageEvent::RemoveSpatial(slot)=>{if !crate::battle_runtime::can_remove(&spatial,slot){return Err("índice espacial de hundimiento inválido")}spatial.remove(slot);removed=true},
        DamageEvent::SinkNotice(captain)=>{let side=usize::from(locals[usize::from(target)][0x13d]);let at=0x35a+2*(1-side);notices.push((captain,u16::from_le_bytes([scene[at],scene[at+1]])));},
    }}
    if removed {
        for (at,value) in [(0x20,index.free_head),(0x34,index.x.head),(0x44,index.y.head),(0x54,index.span)]{
            if u16::from_le_bytes([scene[at],scene[at+1]])!=value{return Err("cabecera espacial desincronizada")}
        }
        for (at,value) in [(0x20,spatial.free_head),(0x34,spatial.x.head),(0x44,spatial.y.head),(0x54,spatial.span)]{staged[at..at+2].copy_from_slice(&value.to_le_bytes());}
    }
    Ok(Some((staged,locals,world,spatial,notices)))
}

fn stage_tower_damage_events(scene:&[u8],piece:usize,ships:&[Vec<u8>],worlds:&[u8],next:&[u16],
    near:&[u16],far:&[u16],power:u8,human:[bool;2],difficulty:u8,angles:Option<&[u8;65]>)
    ->Result<Option<(Vec<u8>,Vec<Vec<u8>>,Vec<u8>,Vec<p3_core::ships::weapon_damage::DamageEvent>,u16)>,&'static str>{
    let Some((mut staged,target,damage))=stage_tower_instant_inner(scene,piece,ships,next,power,human,difficulty,angles)? else{return Ok(None)};
    let mut locals=ships.to_vec();let mut world=worlds.to_vec();let ship=locals.get_mut(usize::from(target)).ok_or("blanco de torre ausente")?;
    if ship.len()<0x164{return Err("barco local truncado")}
    if ship[0x13d]>1{return Err("bando local inválido")}
    let id=u32::from_le_bytes(ship[4..8].try_into().unwrap())as usize;let at=id.checked_mul(0x180).ok_or("barco mundial inválido")?;
    let global=world.get_mut(at..at.checked_add(0x180).ok_or("barco mundial inválido")?).ok_or("barco mundial ausente")?;
    let slots=(*near.get(usize::from(target)).ok_or("índice cercano ausente")?,*far.get(usize::from(target)).ok_or("índice lejano ausente")?);
    let mut events=Vec::new();p3_core::ships::weapon_damage::apply_weapon_damage(global,ship,&mut staged,slots,damage,-1,3,&mut events)?;
    Ok(Some((staged,locals,world,events,target)))
}

/// Incluye el recorrido de la lista atacante cuando todos los barcos están
/// hundidos, huidos o predichos fuera del alcance. Rechaza blancos dentro
/// del alcance y listas rotas o cíclicas.
pub fn town_defense_piece_wait(scene:&mut [u8],piece:usize,ships:&[Vec<u8>],next:&[u16])->Result<(),&'static str>{
    if scene.len()<0x1ec2 || piece>=4{return Err("pieza de defensa truncada")}
    if scene[0x1ebe+piece]!=0 || u16::from_le_bytes([scene[0x10],scene[0x11]])==u16::MAX{return town_defense_piece_cooldown(scene,piece)}
    let mut seen=vec![false;ships.len()];let mut id=u16::from_le_bytes([scene[0x10],scene[0x11]]);
    while id!=u16::MAX{let i=usize::from(id);if i>=ships.len() || i>=next.len() || seen[i]{return Err("lista atacante inválida")}
        seen[i]=true;let ship=&ships[i];let flags=*ship.get(0x12a).ok_or("barco atacante truncado")?;
        if flags&6==0{
            let (_,_,distance)=predict_tower_target(scene,piece,ship)?;
            if distance<=0x7e900{return Err("selección de blanco de torre pendiente")}
        }id=next[i];
    }Ok(())
}

/// 0x60117B: comprueba el segmento de un proyectil contra el rombo de una
/// pieza defensiva. No aplica daño, destruye proyectiles ni ejecuta efectos.
pub fn town_tower_projectile_intersects(scene:&[u8],piece:usize,projectile:&[u8])->Result<bool,&'static str>{
    if scene.len()<0x1eae || projectile.len()<0x14 || piece>=4 {return Err("pieza o proyectil truncados")}
    let word=|data:&[u8],at:usize|u16::from_le_bytes([data[at],data[at+1]]);
    let signed=|data:&[u8],at:usize|i32::from(word(data,at) as i16);
    let dword=|data:&[u8],at:usize|i32::from_le_bytes(data[at..at+4].try_into().unwrap());
    let mut vx=signed(projectile,0xe).wrapping_add(dword(projectile,0xc).signum());
    let mut vy=signed(projectile,0x12).wrapping_add(dword(projectile,0x10).signum());
    if scene[0xac8]!=0 {vx=vx.wrapping_shl(4);vy=vy.wrapping_shl(4)}
    let x=i32::from(word(scene,0x1e9e+2*piece)).wrapping_sub(signed(projectile,6));
    let y=i32::from(word(scene,0x1ea6+2*piece)).wrapping_sub(signed(projectile,0xa));
    let v=[(x.wrapping_sub(34),y),(x,y.wrapping_sub(34)),(x.wrapping_add(34),y),
        (x,y.wrapping_add(34)),(x.wrapping_sub(34),y)];
    for edge in v.windows(2) {
        let ((ax,ay),(bx,by))=(edge[0],edge[1]);
        let mut a=ay.wrapping_mul(vx).wrapping_sub(ax.wrapping_mul(vy));
        let mut b=ay.wrapping_sub(by).wrapping_mul(vx).wrapping_sub(ax.wrapping_sub(bx).wrapping_mul(vy));
        if b<0 {a=a.wrapping_neg();b=b.wrapping_neg()}
        if a>b || a<0 {continue}
        let dx=bx.wrapping_sub(ax);let dy=by.wrapping_sub(ay);
        let mut c=dy.wrapping_mul(ax).wrapping_sub(dx.wrapping_mul(ay));
        let mut d=dy.wrapping_mul(vx).wrapping_sub(dx.wrapping_mul(vy));
        if d<0 {c=c.wrapping_neg();d=d.wrapping_neg()}
        if c<=d && c>=0 {return Ok(true)}
    }
    Ok(false)
}

/// Rama no letal de 0x60130B en una guarida pirata. El llamante debe haber
/// validado la banda asociada; el callback de actualización es 0x515D90,
/// que retorna sin modificar estado. Destrucción y ciudades se rechazan.
pub fn town_pirate_tower_nonlethal(scene:&mut [u8],piece:usize,damage:i32,town_count:u32)->Result<(),&'static str>{
    if scene.len()<0x1ed4 || piece>=4 || !(250..=254).contains(&scene[0x1e98])
        || u32::from(scene[0x1e98])<town_count {return Err("pieza pirata incompatible")}
    let at=0x1ec4+piece*4;let hp=i32::from_le_bytes(scene[at..at+4].try_into().unwrap());
    if hp<=damage {return Err("destrucción de pieza pendiente")}
    scene[at..at+4].copy_from_slice(&hp.wrapping_sub(damage).to_le_bytes());Ok(())
}

/// Efectos aún no publicados del derribo de una pieza pirata. El índice y el
/// planificador deben confirmarse en la misma transacción que la escena.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct PirateTowerDestruction {
    pub scene:Vec<u8>,pub collision:u16,pub ruin_task:Option<(u16,u16,u8)>,
}
/// Prepara la rama letal sin interfaz ni hotseat de 0x60130B. No modifica la
/// escena, la banda, el índice ni el planificador del llamante.
pub fn prepare_pirate_tower_destruction(scene:&[u8],piece:usize,damage:i32,town_count:u32,
    band:&p3_save::pirate_bands::PirateBand,viewed:bool,hotseat:bool)
    ->Result<PirateTowerDestruction,&'static str>{
    if scene.len()<0x1ed4 || piece>=4 || !(250..=254).contains(&scene[0x1e98])
        || u32::from(scene[0x1e98])<town_count || viewed || hotseat {
        return Err("destrucción de torre pirata incompatible")
    }
    band.validate()?;
    let at=0x1ec4+piece*4;let hp=i32::from_le_bytes(scene[at..at+4].try_into().unwrap());
    if hp>damage {return Err("la pieza no ha sido destruida")}
    let capacity=band.pieces.len()/8;let mut cursor=0usize;let mut ruin_task=None;
    for _ in 0..4 {
        if cursor>=capacity {break}
        let row=&band.pieces[cursor*8..cursor*8+8];
        if matches!(row[3],0x30|0x31) && row[0]==scene[0x1eae+piece] && row[1]==scene[0x1eb2+piece] {
            ruin_task=Some((u16::from(row[0]),u16::from(row[1]),scene[0x1e98]));break;
        }
        cursor=usize::from(u16::from_le_bytes([row[4],row[5]]));
    }
    let id=0x1eb6+2*piece;let collision=u16::from_le_bytes([scene[id],scene[id+1]]);
    let mut out=scene.to_vec();out[0x1e9e+2*piece..0x1ea0+2*piece].fill(255);
    out[at..at+4].fill(0);out[0xac4]=1;
    Ok(PirateTowerDestruction{scene:out,collision,ruin_task})
}

/// Estado preparado, aún no publicado, del derribo de una pieza pirata.
pub struct StagedPirateTowerDestruction {
    pub scene:Vec<u8>,pub collision:p3_core::ships::spatial::CollisionIndex,
    pub scheduler:p3_core::scheduler::Scheduler,pub task:Option<u16>,
}
/// Ensaya juntos escena, índice y tarea sin tocar las entradas. No prepara
/// otros efectos del impacto ni actualiza el bloque S05. El llamante debe
/// verificar que la banda suministrada es la asociada al destino de la escena.
pub fn stage_pirate_tower_destruction(scene:&[u8],piece:usize,damage:i32,town_count:u32,
    band:&p3_save::pirate_bands::PirateBand,index:&p3_core::ships::spatial::CollisionIndex,
    scheduler:&p3_core::scheduler::Scheduler,tick:u32)
    ->Result<StagedPirateTowerDestruction,String>{
    let plan=prepare_pirate_tower_destruction(scene,piece,damage,town_count,band,false,false).map_err(str::to_owned)?;
    crate::battle_spatial::into_saved(index).map_err(str::to_owned)?;
    let slot=usize::from(plan.collision);
    if slot>=index.capacity || index.types[slot]!=8 || index.references[slot]!=piece as u16
        || !crate::battle_runtime::can_remove(index,plan.collision) {
        return Err("colisión de torre inválida".into());
    }
    let mut staged_index=index.clone();staged_index.remove(plan.collision);
    let mut staged_scheduler=scheduler.clone();let mut task=None;
    if let Some((x,y,target))=plan.ruin_task {
        let n=usize::from(scheduler.capacity);
        if scheduler.records.len()!=n || usize::from(scheduler.free_head)>=n {
            return Err("planificador sin ranura libre contrastada".into());
        }
        let mut seen=vec![false;n];let mut at=scheduler.head;
        while usize::from(at)<n {let i=usize::from(at);if seen[i] || at==scheduler.free_head{return Err("lista de tareas inválida".into())}
            seen[i]=true;at=scheduler.records[i].next;}
        at=scheduler.free_head;
        while usize::from(at)<n {let i=usize::from(at);if seen[i]{return Err("lista libre de tareas inválida".into())}
            seen[i]=true;at=scheduler.records[i].next;}
        let id=staged_scheduler.add_task(tick);let row=&mut staged_scheduler.records[usize::from(id)];
        row.kind=0x25;row.payload[0..2].fill(255);row.payload[2..4].fill(0);
        row.payload[4..6].copy_from_slice(&x.to_le_bytes());row.payload[6..8].copy_from_slice(&y.to_le_bytes());
        row.payload[8]=target;row.payload[9]=1;task=Some(id);
    }
    let mut out=plan.scene;
    for(at,value)in [(0x20,staged_index.free_head),(0x34,staged_index.x.head),(0x44,staged_index.y.head),
        (0x54,staged_index.span)]{out[at..at+2].copy_from_slice(&value.to_le_bytes())}
    Ok(StagedPirateTowerDestruction{scene:out,collision:staged_index,scheduler:staged_scheduler,task})
}

/// Resultado provisional de las armas instantáneas de un barco de asedio.
pub struct StagedSiegeTowerShot {
    pub scene:Vec<u8>,pub ship:Vec<u8>,pub collision:p3_core::ships::spatial::CollisionIndex,
    pub scheduler:p3_core::scheduler::Scheduler,pub shots:usize,
}

/// Paso provisional de un barco urbano, incluidas sus armas acotadas.
pub struct StagedSiegeShipStep {
    pub scene:Vec<u8>,pub ships:Vec<Vec<u8>>,pub worlds:Vec<u8>,
    pub collision:p3_core::ships::spatial::CollisionIndex,
    pub scheduler:p3_core::scheduler::Scheduler,pub shots:usize,
}

/// Resultado provisional de ordenar el índice y despachar parejas urbanas.
pub struct StagedSiegePairs {
    pub scene:Vec<u8>,pub ships:Vec<Vec<u8>>,pub worlds:Vec<u8>,
    pub collision:p3_core::ships::spatial::CollisionIndex,pub candidates:usize,pub coast_contacts:usize,
}

/// Un tick acotado de asalto poblado, sin proyectiles ni fin de combate.
pub struct StagedSiegeFrame {
    pub scene:Vec<u8>,pub ships:Vec<Vec<u8>>,pub worlds:Vec<u8>,
    pub collision:p3_core::ships::spatial::CollisionIndex,pub scheduler:p3_core::scheduler::Scheduler,
    pub shots:usize,pub candidates:usize,pub coast_contacts:usize,
}

/// Compone estadísticas, viento, ambos barcos, recargas y parejas en copias.
pub fn stage_siege_frame<F,G>(scene:&[u8],ships:&[Vec<u8>],worlds:&[u8],next:&[u16],near:&[u16],far:&[u16],
    captains:&[u8],merchants:&[u16],difficulty:u8,band:&p3_save::pirate_bands::PirateBand,
    index:&p3_core::ships::spatial::CollisionIndex,scheduler:&p3_core::scheduler::Scheduler,
    town_count:u32,tick:u32,delta:u32,tower_power:u8,tower_divisor:u8,plan:&[u8],weights:&[u8;45],
    shapes:&[[(i16,i16);5];4],sine:&[i32;65],angles:&[u8;65],sound_possible:bool,
    pixel_to_cell:F,cell_to_pixel:G)->Result<StagedSiegeFrame,String>
where F:FnMut(i32,i32)->Result<(u16,u16),&'static str>,G:Fn(u16,u16)->(u32,u32){
    if scene.len()!=0x2f24 || scene[0x5c]!=0 || scene[0x2f1a]!=0 || scene[0x558]!=2
        || scene[0x5d..0x5f]!=[1,1] || scene[0x12e9..0x12eb]!=[0,0]
        || scene[0x15c..0x160]!=[0,0,0,0] || scene[0xacc..0xace]!=[0,0]
        || scene[0x658..0x65c]!=[1,0,1,0] || ships.len()!=2 || next.len()!=2
        || scene[0x130f]>15 || !(16..32).contains(&(u32::from(scene[0x130f])+u32::from(delta as u16))) {
        return Err("frame de asedio fuera del alcance contrastado".into())
    }
    let first=usize::from(u16::from_le_bytes([scene[0x10],scene[0x11]]));let second=usize::from(u16::from_le_bytes([scene[0x65c],scene[0x65d]]));
    if first>=2 || second>=2 || first==second || next[first]!=u16::MAX || next[second]!=u16::MAX
        || ships.iter().any(|s|s.len()!=0x1168 || s[0x12a]&0x66!=0){return Err("barcos de asedio fuera del alcance contrastado".into())}
    let mut object=scene.to_vec();object[0xac8]=1;object[0x130f]=((u32::from(scene[0x130f])+u32::from(delta as u16))&15)as u8;
    let mut local=ships.to_vec();let mut world=worlds.to_vec();let mut spatial=index.clone();let mut tasks=scheduler.clone();
    town_attack_statistics(&mut object,&local,&world,next,captains,tower_power,tower_divisor)?;
    let counter=object[0xa8c].wrapping_add(1);object[0xa8c]=if u32::from(counter)*40>=1000{0}else{counter};
    p3_core::ships::wind::update_wind(&mut object,0xff,false);
    let mut shots=0;
    for id in [first,second] {
        let staged=stage_siege_ship_step(&object,&local,&world,next,near,far,id,captains,merchants,difficulty,band,&spatial,&tasks,town_count,tick)?;
        object=staged.scene;local=staged.ships;world=staged.worlds;spatial=staged.collision;tasks=staged.scheduler;shots+=staged.shots;
        if local.iter().any(|s|s[0x12a]&0x66!=0){return Err("fin de asedio pendiente".into())}
    }
    for piece in 0..4 {let at=0x1ec4+4*piece;let hp=i32::from_le_bytes(object[at..at+4].try_into().unwrap());
        if hp<0{return Err("vida de torre no contrastada".into())}if hp>0 {
            town_defense_piece_wait(&mut object,piece,&local,next).map_err(|e|format!("disparo automático de torre pendiente: {e}"))?;
        }}
    let paired=stage_siege_pairs(&object,&local,&world,captains,&spatial,plan,town_count,weights,shapes,sine,angles,sound_possible,pixel_to_cell,cell_to_pixel)?;
    Ok(StagedSiegeFrame{scene:paired.scene,ships:paired.ships,worlds:paired.worlds,collision:paired.collision,scheduler:tasks,
        shots,candidates:paired.candidates,coast_contacts:paired.coast_contacts})
}

/// Estadísticas del tick urbano para dos cadenas navales y torres piratas.
fn town_attack_statistics(scene:&mut [u8],ships:&[Vec<u8>],worlds:&[u8],next:&[u16],captains:&[u8],
    tower_power:u8,tower_divisor:u8)->Result<(),String>{
    use p3_core::ships::boarding::{artillery_strength,effective_speed,Captains};
    if scene.len()!=0x2f24 || ships.len()!=next.len() || worlds.len()%0x180!=0 || captains.len()%16!=0
        || captains.len()/16>u16::MAX as usize || tower_divisor==0{return Err("estadísticas urbanas incompatibles".into())}
    fn get(s:&[u8],at:usize)->u32{u32::from_le_bytes(s[at..at+4].try_into().unwrap())}
    fn put(s:&mut [u8],at:usize,v:u32){s[at..at+4].copy_from_slice(&v.to_le_bytes())}
    fn word(s:&[u8],at:usize)->u16{u16::from_le_bytes([s[at],s[at+1]])}
    fn setword(s:&mut [u8],at:usize,v:u16){s[at..at+2].copy_from_slice(&v.to_le_bytes())}
    let original_speed=scene[0xaa8..0xab0].to_vec();
    for at in [0xa90usize,0xa94,0xaa0,0xaa4,0xaa8,0xaac,0xab4,0xab8,0xabc,0xac0]{put(scene,at,0)}
    for at in [0xa98usize,0xa9a,0xa9c,0xa9e]{setword(scene,at,0)}
    let captains=Captains{table:captains,count:(captains.len()/16)as u16};let mut counts=[0u8;2];
    for side in 0..2usize {
        let mut cursor=if side==0{word(scene,0x10)}else{word(scene,0x65c)};let mut seen=vec![false;ships.len()];
        while cursor!=u16::MAX {let id=usize::from(cursor);if id>=ships.len() || seen[id]{return Err("lista de asedio inválida".into())}seen[id]=true;
            let ship=&ships[id];if ship.len()<0x164{return Err("barco urbano truncado".into())}
            if ship[0x12a]&0x26==0 {
                let id=u32::from_le_bytes(ship[4..8].try_into().unwrap()) as usize;
                let world=worlds.get(id.checked_mul(0x180).ok_or("barco mundial inválido")?..id.checked_add(1).and_then(|v|v.checked_mul(0x180)).ok_or("barco mundial inválido")?).ok_or("barco mundial ausente")?;
                put(scene,0xa90+4*side,get(scene,0xa90+4*side).wrapping_add(artillery_strength(world,captains)as u32));
                let crew=word(world,0x40);if crew>word(scene,0xa98+2*side){setword(scene,0xa98+2*side,crew)}
                let extra=word(world,0x154);let boarding=if crew<extra{u32::from(crew)*2}else{u32::from(crew)+u32::from(extra)} as u16;
                if boarding>word(scene,0xa9c+2*side){setword(scene,0xa9c+2*side,boarding)}
                let guns=get(world,0x120);if guns>get(scene,0xaa0+4*side){put(scene,0xaa0+4*side,guns)}
                let speed=effective_speed(world,captains).map_err(str::to_owned)?;if speed>get(scene,0xaa8+4*side){put(scene,0xaa8+4*side,speed)}
                counts[side]=counts[side].wrapping_add(1);
                let x=i32::from(word(ship,0xa)as i16)as u32;let y=i32::from(word(ship,0xe)as i16)as u32;
                put(scene,0xab4+4*side,get(scene,0xab4+4*side).wrapping_add(x));put(scene,0xabc+4*side,get(scene,0xabc+4*side).wrapping_add(y));
            }cursor=next[id];
        }
    }
    counts[1]=town_defense_statistics(scene,counts[1],tower_power,tower_divisor).map_err(str::to_owned)?;
    for side in 0..2usize {let n=u32::from(counts[side]);let (x,y)=if n==0{(0x1100,0xf02)}else{(get(scene,0xab4+4*side)/n,get(scene,0xabc+4*side)/n)};
        put(scene,0xab4+4*side,x);put(scene,0xabc+4*side,y)}
    scene[0xaa8..0xab0].copy_from_slice(&original_speed);Ok(())
}

/// Despacha parejas sin proyectiles ni efectos externos; la costa usa casco,
/// plano y proyección aportados. La entrada no cambia si una rama se rechaza.
pub fn stage_siege_pairs<F,G>(scene:&[u8],ships:&[Vec<u8>],worlds:&[u8],captains:&[u8],
    index:&p3_core::ships::spatial::CollisionIndex,plan:&[u8],town_count:u32,
    weights:&[u8;45],shapes:&[[(i16,i16);5];4],sine:&[i32;65],angles:&[u8;65],sound_possible:bool,
    mut pixel_to_cell:F,cell_to_pixel:G)->Result<StagedSiegePairs,String>
where F:FnMut(i32,i32)->Result<(u16,u16),&'static str>,G:Fn(u16,u16)->(u32,u32){
    use p3_core::ships::{boarding::Captains,collide::{self,BoardCtx,PairHost,Slots}};
    if scene.len()!=0x2f24 || scene[0x2f1a]!=0 || captains.len()%16!=0 || captains.len()/16>u16::MAX as usize
        || worlds.len()%0x180!=0 || scene[0x559]>1 || scene[0x55a]>1 {return Err("parejas de asedio incompatibles".into())}
    for (id,ship) in ships.iter().enumerate(){if ship.len()!=0x1168 || ship[0x13d]>=2
        || usize::from(u16::from_le_bytes([ship[0x138],ship[0x139]]))!=id
        || u32::from_le_bytes(ship[4..8].try_into().unwrap())as usize>=worlds.len()/0x180{return Err("barco urbano incompatible".into())}}
    crate::battle_spatial::into_saved(index).map_err(str::to_owned)?;
    let mut object=scene.to_vec();let mut local=ships.to_vec();for ship in &mut local{ship.resize(0x1170,0)}
    let mut world=worlds.to_vec();let mut spatial=index.clone();let mut failure=None;let mut candidates=0;let mut contacts=0;
    struct Guard{called:bool}impl PairHost for Guard{
        fn hit_mask(&mut self,_:&mut [u8],_:&mut [Vec<u8>],_:&mut [u8],_:u16,_:u16)->u8{self.called=true;0}
        fn impact(&mut self,_:&mut [u8],_:&mut [Vec<u8>],_:&mut [u8],_:u16,_:u16,_:u8){self.called=true}
        fn damage(&mut self,_:&mut [u8],_:&mut [Vec<u8>],_:&mut [u8],_:u16,_:i32){self.called=true}
    }
    let ctx=BoardCtx{captains:Captains{table:captains,count:(captains.len()/16)as u16},virtual_10:true};
    spatial.update(|idx,a,b|{
        if failure.is_some(){return}candidates+=1;
        let (first,second)=(idx.types[usize::from(a)],idx.types[usize::from(b)]);
        if first==8 && second==1 {failure=Some("impacto de proyectil en torre pendiente".to_string());return}
        if first!=0 || second==8{return}
        let id=usize::from(idx.references[usize::from(a)]);
        let Some(ship)=local.get_mut(id) else {failure=Some("barco espacial ausente".to_string());return};
        if ship[0x12a]&2!=0{return}
        if second==7 {
            let world_id=u32::from_le_bytes(ship[4..8].try_into().unwrap())as usize;
            let kind=usize::from(world[world_id*0x180+0xe]);let Some(shape)=shapes.get(kind) else {failure=Some("tipo de casco no admitido".to_string());return};
            match apply_town_coast_contact(&object,plan,town_count,ship,weights,shape,sine,angles,sound_possible,&mut pixel_to_cell,&cell_to_pixel){
                Ok(true)=>contacts+=1,Ok(false)=>{},Err(e)=>failure=Some(e.to_string())}
            return
        }
        let (x,y,kind,reference)=(idx.x.coordinates.clone(),idx.y.coordinates.clone(),idx.types.clone(),idx.references.clone());
        let slots=Slots{x:&x,y:&y,kind:&kind,reference:&reference};let mut guard=Guard{called:false};
        if let Err(e)=collide::dispatch_pair(&mut object,&slots,&mut local,&mut world,&ctx,a,b,false,&mut guard){failure=Some(e.to_string())}
        else if guard.called {failure=Some("efecto de pareja pendiente".to_string())}
    });
    if let Some(error)=failure{return Err(error)}
    for ship in &mut local{ship.truncate(0x1168)}
    for(at,value)in [(0x20,spatial.free_head),(0x34,spatial.x.head),(0x44,spatial.y.head),(0x54,spatial.span)]{object[at..at+2].copy_from_slice(&value.to_le_bytes())}
    Ok(StagedSiegePairs{scene:object,ships:local,worlds:world,collision:spatial,candidates,coast_contacts:contacts})
}

/// Ejecuta un barco de asedio con su imagen de navegación completa y publica
/// únicamente en copias. Rechaza captura, gráficos y armas no conectadas.
pub fn stage_siege_ship_step(scene:&[u8],ships:&[Vec<u8>],worlds:&[u8],next:&[u16],near:&[u16],far:&[u16],
    id:usize,captains:&[u8],merchants:&[u16],difficulty:u8,band:&p3_save::pirate_bands::PirateBand,
    index:&p3_core::ships::spatial::CollisionIndex,scheduler:&p3_core::scheduler::Scheduler,
    town_count:u32,tick:u32)->Result<StagedSiegeShipStep,String>{
    use p3_core::ships::{boarding::{BoardingHost,Captains},siege::siege_ship_step,step::{StepContext,StepEvent,StepOutcome}};
    if scene.len()!=0x2f24 || scene[0x5c]!=0 || scene[0x2f1a]!=0 || ships.len()!=next.len() || ships.len()!=near.len() || ships.len()!=far.len()
        || ships.len()>u16::MAX as usize || captains.len()%16!=0 || captains.len()/16>u16::MAX as usize
        || worlds.len()%0x180!=0 || scene[0x559]>1 || scene[0x55a]>1 {return Err("paso de asedio incompatible".into())}
    for (slot,ship) in ships.iter().enumerate(){
        if ship.len()!=0x1168 || ship[0x13d]>=2 || usize::from(u16::from_le_bytes([ship[0x138],ship[0x139]]))!=slot
            || u32::from_le_bytes(ship[4..8].try_into().unwrap())as usize>=worlds.len()/0x180 {return Err("barco urbano incompatible".into())}
    }
    if id>=ships.len(){return Err("barco urbano ausente".into())}
    for start in [u16::from_le_bytes([scene[0x10],scene[0x11]]),u16::from_le_bytes([scene[0x65c],scene[0x65d]])] {
        let mut seen=vec![false;ships.len()];let mut cursor=start;
        while cursor!=u16::MAX {let i=usize::from(cursor);if i>=ships.len()||seen[i]{return Err("lista de asedio inválida".into())}seen[i]=true;cursor=next[i];}
    }
    let mut object=scene.to_vec();let mut local=ships.to_vec();for ship in &mut local {ship.resize(0x1170,0)}
    let mut world=worlds.to_vec();let mut spatial=index.clone();crate::battle_spatial::into_saved(&spatial).map_err(str::to_owned)?;
    let human=|owner:u8|merchants.get(usize::from(owner)).is_some_and(|v|*v==0);
    let ctx=StepContext{next,slots_near:near,slots_far:far,captains:Captains{table:captains,count:(captains.len()/16)as u16},
        human_owner:&human,difficulty,virtual_10:true,view_active:false};
    struct Host{capture:bool}impl BoardingHost for Host{fn capture(&mut self,_:&mut [u8],_:u16,_:u16){self.capture=true}}
    let mut host=Host{capture:false};let mut events=Vec::new();let param=object[0x670];
    let outcome=siege_ship_step(id,param,town_count,&mut object,&mut local,&mut world,&ctx,&mut host,&mut events).map_err(str::to_owned)?;
    if host.capture{return Err("captura de asedio pendiente".into())}
    for event in events {match event {
        StepEvent::Release(_)=>return Err("recurso gráfico de asedio pendiente".into()),
        StepEvent::SpatialRemove(slot)=>{if !crate::battle_runtime::can_remove(&spatial,slot){return Err("baja espacial inválida".into())}spatial.remove(slot)},
        StepEvent::SpatialBounds{slot,x0,y0,x1,y1}=>{if usize::from(slot)>=spatial.capacity{return Err("hueco espacial inválido".into())}spatial.bounds(slot,x0 as i16,y0 as i16,x1 as i16,y1 as i16)}
    }}
    for ship in &mut local {ship.truncate(0x1168)}
    let mut tasks=scheduler.clone();let mut shots=0;
    if outcome==StepOutcome::Weapons {
        let staged=stage_siege_tower_shots(&object,&local,&world,next,id,captains,merchants,difficulty,band,&spatial,&tasks,town_count,tick,false)?;
        object=staged.scene;local[id]=staged.ship;spatial=staged.collision;tasks=staged.scheduler;shots=staged.shots;
    }
    for(at,value)in [(0x20,spatial.free_head),(0x34,spatial.x.head),(0x44,spatial.y.head),(0x54,spatial.span)]{object[at..at+2].copy_from_slice(&value.to_le_bytes())}
    Ok(StagedSiegeShipStep{scene:object,ships:local,worlds:world,collision:spatial,scheduler:tasks,shots})
}

/// La CRT del EXE devuelve longitud cero para el cuadrado negativo observado
/// en este caso acotado. El plan general sigue rechazando otros desbordamientos.
fn siege_overflow_tower_plan(scene:&[u8],ships:&[Vec<u8>],next:&[u16],id:usize,
    empty_tower_plan:&p3_core::ships::weapons::WeaponPlan,skip:bool)
    ->Result<p3_core::ships::weapons::WeaponPlan,&'static str>{
    use p3_core::ships::weapons::{DirectShot,WeaponPlan};
    const ATAN:[u8;65]=[0,1,1,2,3,3,4,4,5,6,6,7,8,8,9,9,10,11,11,12,12,13,13,14,15,15,16,16,17,17,18,18,19,19,20,20,21,21,22,22,23,23,24,24,25,25,25,26,26,27,27,27,28,28,29,29,29,30,30,30,31,31,31,32,32];
    if !matches!(empty_tower_plan,WeaponPlan::None) || ships.len()!=2 {return Err("objetivos adicionales con desbordamiento")}
    let own=&ships[id];let side=usize::from(own[0x13d]);let map=scene[0x559+side];
    if map!=0 || (!skip && scene[0x5d+side]==0 && u16::from_le_bytes([own[0x128],own[0x129]])>0xfffe) {
        return Err("política de torre con desbordamiento no contrastada")
    }
    let head=usize::from(u16::from_le_bytes([scene[0x10],scene[0x11]]));
    if head!=id || next[id]!=u16::MAX{return Err("convoy con desbordamiento no contrastado")}
    let alive=(0..4).filter(|&p|i32::from_le_bytes(scene[0x1ec4+4*p..0x1ec8+4*p].try_into().unwrap())>0).collect::<Vec<_>>();
    if alive.len()!=1{return Err("torres múltiples con desbordamiento no contrastadas")}
    let piece=alive[0];let selected=u16::from_le_bytes([own[0x128],own[0x129]]);
    if !skip && scene[0x5d+side]==0 && selected!=0xfffe-piece as u16{return Err("torre no seleccionada")}
    let x=i32::from(u16::from_le_bytes(scene[0x1e9e+2*piece..0x1ea0+2*piece].try_into().unwrap()))-i32::from(i16::from_le_bytes([own[0xa],own[0xb]]));
    let y=i32::from(u16::from_le_bytes(scene[0x1ea6+2*piece..0x1ea8+2*piece].try_into().unwrap()))-i32::from(i16::from_le_bytes([own[0xe],own[0xf]]));
    if x.wrapping_mul(x).wrapping_add(y.wrapping_mul(y))>=0{return Err("cuadrado sin desbordamiento")}
    let angle=town_coast_vector_angle(i32::from(x as i16),i32::from(y as i16),&ATAN).wrapping_sub(own[0x11]);
    let band=if angle>0x37 && angle<0x49 {1}else if angle>0xb7 && angle<0xc9 {0}else{return Err("ángulo de torre no elegible")};
    if own[0x130+usize::from(band)]!=0{return Err("banda en recarga")}
    Ok(WeaponPlan::Direct(vec![DirectShot{band,target:0xfffe-piece as u16,length:0}]))
}

/// Selecciona y ejecuta disparos instantáneos solo a torres piratas. Toda
/// entrada permanece intacta si aparece una ruta de barcos, vista o andanada.
pub fn stage_siege_tower_shots(scene:&[u8],ships:&[Vec<u8>],worlds:&[u8],next:&[u16],id:usize,
    captains:&[u8],merchants:&[u16],difficulty:u8,band:&p3_save::pirate_bands::PirateBand,
    index:&p3_core::ships::spatial::CollisionIndex,scheduler:&p3_core::scheduler::Scheduler,
    town_count:u32,tick:u32,skip_policy:bool)->Result<StagedSiegeTowerShot,String>{
    use p3_core::ships::{boarding::{Boarding,Captains},projectile::ShotContext,
        siege_weapons::{self,SiegeEffects,SiegeView},weapons::{self,WeaponPlan}};
    if scene.len()!=0x2f24 || scene[0xac8]==0 || scene[0x559]>1 || scene[0x55a]>1 || ships.len()!=next.len()
        || usize::from(u16::from_le_bytes([scene[0x14],scene[0x15]]))!=ships.len()
        || captains.len()%16!=0 || captains.len()/16>u16::MAX as usize || worlds.len()%0x180!=0
        || !(250..=254).contains(&scene[0x1e98]) || u32::from(scene[0x1e98])<town_count {
        return Err("disparo de asedio incompatible".into())
    }
    band.validate().map_err(str::to_owned)?;crate::battle_spatial::into_saved(index).map_err(str::to_owned)?;
    let own=ships.get(id).ok_or("barco de asedio ausente")?;
    for ship in ships {
        if ship.len()<0x164 || ship[0x13d]>=2 || usize::from(ship[0x13d])+0x559>=scene.len(){return Err("barco de asedio inválido".into())}
        let world=u32::from_le_bytes(ship[4..8].try_into().unwrap()) as usize;
        if world>=worlds.len()/0x180{return Err("barco mundial ausente".into())}
    }
    if usize::from(u16::from_le_bytes([own[0x138],own[0x139]]))!=id{return Err("ranura de barco inconsistente".into())}
    for start in [u16::from_le_bytes([scene[0x10],scene[0x11]]),u16::from_le_bytes([scene[0x65c],scene[0x65d]])] {
        let mut seen=vec![false;ships.len()];let mut cursor=start;
        while cursor!=u16::MAX {let i=usize::from(cursor);if i>=ships.len()||seen[i]{return Err("lista de asedio inválida".into())}seen[i]=true;cursor=next[i];}
    }
    let world_id=u32::from_le_bytes(own[4..8].try_into().unwrap()) as usize;let world=&worlds[world_id*0x180..(world_id+1)*0x180];
    let equipment:[u8;24]=world[0x13c..0x154].try_into().unwrap();let captain=u16::from_le_bytes([world[0x42],world[0x43]])as usize;
    let shot=ShotContext {ship_x:i16::from_le_bytes([own[0xa],own[0xb]]),ship_y:i16::from_le_bytes([own[0xe],own[0xf]]),
        local_id:u16::from_le_bytes([own[0x138],own[0x139]]),battle_pointer:u32::from_le_bytes(own[0x12c..0x130].try_into().unwrap()),
        owner:world[0],maintenance:world[0x3f] as i8,captain_experience:captains.get(captain*16+11).copied(),
        human_difficulty:merchants.get(usize::from(world[0])).filter(|&&m|m==0).map(|_|difficulty)};
    let plan={let local=|n:u16|ships[usize::from(n)].as_slice();let world_of=|n:u32|&worlds[n as usize*0x180..(n as usize+1)*0x180];
        let view=SiegeView{battle:scene,next,local:&local,world:&world_of};
        let board=Boarding{battle:scene,virtual_10:true,world:&world_of,captains:Captains{table:captains,count:(captains.len()/16)as u16}};
        match siege_weapons::plan_siege_weapons(&view,own,skip_policy,&mut |target|board.board_filter(own,local(target),true)) {
            Ok(plan)=>plan,
            Err("raíz de un cuadrado desbordado (ruta matherr de la CRT no modelada)")=>{
                let mut no_towers=scene.to_vec();no_towers[0x1ec4..0x1ed4].fill(0);
                let view=SiegeView{battle:&no_towers,next,local:&local,world:&world_of};
                let board=Boarding{battle:&no_towers,virtual_10:true,world:&world_of,captains:Captains{table:captains,count:(captains.len()/16)as u16}};
                let empty=siege_weapons::plan_siege_weapons(&view,own,skip_policy,&mut |target|board.board_filter(own,local(target),true)).map_err(str::to_owned)?;
                siege_overflow_tower_plan(scene,ships,next,id,&empty,skip_policy).map_err(str::to_owned)?
            },
            Err(e)=>return Err(e.into()),
        }};
    let count=match &plan {WeaponPlan::None=>0,WeaponPlan::Direct(shots)=>shots.len(),WeaponPlan::Volleys(_)=>return Err("andanada de asedio pendiente".into())};
    struct Effects<'a>{equipment:[u8;24],shot:ShotContext,band:&'a p3_save::pirate_bands::PirateBand,
        index:p3_core::ships::spatial::CollisionIndex,scheduler:p3_core::scheduler::Scheduler,town_count:u32,tick:u32,error:Option<String>}
    impl SiegeEffects for Effects<'_>{
        fn estimate(&mut self,battle:&mut [u8],band:u8,length:i32)->Result<i32,&'static str>{Ok(weapons::estimate(&self.equipment,band,length,&self.shot,battle)?.damage)}
        fn ship_damage(&mut self,_:&mut [u8],_:u16,_:i32)->Result<(),&'static str>{Err("disparo de asedio a barco pendiente")}
        fn tower_damage(&mut self,battle:&mut [u8],piece:u8,amount:i32)->Result<(),&'static str>{
            let p=usize::from(piece);if p>=4 || amount<0{return Err("daño a torre inválido")}
            let at=0x1ec4+4*p;let health=i32::from_le_bytes(battle[at..at+4].try_into().unwrap());
            if health<=0{return Err("torre ya destruida")}
            if health>amount {town_pirate_tower_nonlethal(battle,p,amount,self.town_count)?;return Ok(())}
            match stage_pirate_tower_destruction(battle,p,amount,self.town_count,self.band,&self.index,&self.scheduler,self.tick){
                Ok(v)=>{battle.copy_from_slice(&v.scene);self.index=v.collision;self.scheduler=v.scheduler;Ok(())},
                Err(e)=>{self.error=Some(e);Err("derribo de torre rechazado")}}
        }
        fn notify(&mut self,_:u8){self.error=Some("aviso de asedio pendiente".into())}
        fn volley(&mut self,_:&mut [u8],_:u8,_:(i16,i16))->Result<(),&'static str>{Err("andanada de asedio pendiente")}
    }
    let mut out=scene.to_vec();let mut ship=own.clone();let mut fx=Effects{equipment,shot,band,index:index.clone(),scheduler:scheduler.clone(),town_count,tick,error:None};
    let flags=|n:u16|ships.get(usize::from(n)).map_or(0,|s|s[0x12a]);
    let result=siege_weapons::execute_siege_plan(&plan,&mut ship,&mut out,&flags,&mut fx);
    if let Some(error)=fx.error{return Err(error)}result.map_err(str::to_owned)?;
    Ok(StagedSiegeTowerShot{scene:out,ship,collision:fx.index,scheduler:fx.scheduler,shots:count})
}

/// Resultado provisional del contacto torre-proyectil; ninguna entrada se
/// modifica. No incluye persistencia ni publicación en la sesión.
pub struct StagedPirateTowerImpact {
    pub hit:bool,pub scene:Vec<u8>,pub collision:p3_core::ships::spatial::CollisionIndex,
    pub scheduler:p3_core::scheduler::Scheduler,pub projectiles:p3_core::ships::projectile::ProjectileTable,
    pub task:Option<u16>,
}
/// Variedad sin vista ni hotseat de 0x5FF8E0: proyectil sin tirador y sin
/// recurso gráfico, con banda validada. El llamante verifica que la banda
/// corresponde al destino. Los errores dejan intactos todos los argumentos.
pub fn stage_pirate_tower_impact(scene:&[u8],scene_address:u32,town_count:u32,
    band:&p3_save::pirate_bands::PirateBand,index:&p3_core::ships::spatial::CollisionIndex,
    scheduler:&p3_core::scheduler::Scheduler,projectiles:&p3_core::ships::projectile::ProjectileTable,
    tower_slot:u16,projectile_slot:u16,tick:u32,viewed:bool,hotseat:bool)
    ->Result<StagedPirateTowerImpact,String>{
    if viewed || hotseat || scene_address==0 || scene.len()<0x1ed4 || !(250..=254).contains(&scene[0x1e98])
        || u32::from(scene[0x1e98])<town_count{return Err("impacto pirata no admitido".into())}
    band.validate().map_err(str::to_owned)?;crate::battle_spatial::into_saved(index).map_err(str::to_owned)?;
    let a=usize::from(tower_slot);let b=usize::from(projectile_slot);
    if a>=index.capacity||b>=index.capacity||index.types[a]!=8||index.types[b]!=1
        ||index.references[a]>=4||index.references[b]>255||a==b{return Err("objetos de impacto inválidos".into())}
    let piece=usize::from(index.references[a]);let id=usize::from(index.references[b]);
    if projectiles.bitmap.len()!=(projectiles.slots.len()+31)/32 || id>=projectiles.slots.len()
        ||projectiles.bitmap[id/32]&(1u32<<(id%32))==0{return Err("tabla de proyectiles inválida".into())}
    let projectile=projectiles.slots[id].as_ref().ok_or("proyectil ausente")?;
    let w16=|at:usize|u16::from_le_bytes([projectile[at],projectile[at+1]]);
    let w32=|at:usize|u32::from_le_bytes(projectile[at..at+4].try_into().unwrap());
    if w16(0x24)!=u16::MAX || w16(0x2c)!=id as u16 || w16(0x2e)!=projectile_slot
        || w32(0x20)!=0 || w32(0x28)!=scene_address {
        return Err("proyectil o efectos asociados no admitidos".into());
    }
    if !crate::battle_runtime::can_remove(index,tower_slot)||!crate::battle_runtime::can_remove(index,projectile_slot)
        ||u16::from_le_bytes([scene[0x1eb6+piece*2],scene[0x1eb7+piece*2]])!=tower_slot {
        return Err("colisiones de impacto incoherentes".into());
    }
    let hit=town_tower_projectile_intersects(scene,piece,projectile).map_err(str::to_owned)?;
    let mut result=StagedPirateTowerImpact{hit,scene:scene.to_vec(),collision:index.clone(),scheduler:scheduler.clone(),projectiles:projectiles.clone(),task:None};
    if !hit{return Ok(result)}
    let damage=w32(0x14) as i32;let hp=i32::from_le_bytes(scene[0x1ec4+piece*4..0x1ec8+piece*4].try_into().unwrap());
    if hp>damage {town_pirate_tower_nonlethal(&mut result.scene,piece,damage,town_count).map_err(str::to_owned)?}
    else {let dead=stage_pirate_tower_destruction(scene,piece,damage,town_count,band,index,scheduler,tick)?;
        result.scene=dead.scene;result.collision=dead.collision;result.scheduler=dead.scheduler;result.task=dead.task;}
    if !crate::battle_runtime::can_remove(&result.collision,projectile_slot){return Err("baja de proyectil incoherente".into())}
    result.collision.remove(projectile_slot);result.projectiles.bitmap[id/32]&=!(1u32<<(id%32));result.projectiles.slots[id]=None;
    for(at,value)in [(0x20,result.collision.free_head),(0x34,result.collision.x.head),(0x44,result.collision.y.head),
        (0x54,result.collision.span)]{result.scene[at..at+2].copy_from_slice(&value.to_le_bytes())}
    Ok(result)
}
