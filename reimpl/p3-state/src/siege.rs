//! Transacción diaria de un registro S05 de asedio sin escena activa.
use p3_core::{
    ai::reputation::Merchant,
    calendar::Calendar,
    economy::siege::{self, GoodwillInput},
    rng::CrtRand,
    town::Town,
};

#[derive(Clone, Debug)]
pub struct DailyInput<'a> {
    pub tick: u32,
    pub calendar: Calendar,
    pub difficulty: u8,
    pub disabled: bool,
    pub network_client: u32,
    pub regions: &'a [u8],
    pub stack_tail: Option<[u8; 3]>,
    pub message_enabled: &'a [bool],
}
#[derive(Clone, Debug)]
pub struct DailySnapshot {
    pub record: Vec<u8>,
    pub towns: Vec<Town>,
    pub offices: Vec<Vec<u8>>,
    pub merchants: Vec<Merchant>,
    pub message_pool: Vec<u8>,
    pub rng: Option<CrtRand>,
    pub operation: Option<[u32; 5]>,
}

#[derive(Clone, Debug)]
pub struct UnwalledOperation {
    pub record: Vec<u8>,
    pub town: Town,
    pub message_pool: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct SupplyRequestOperation {
    pub record: Vec<u8>,
    pub merchants: Vec<Merchant>,
    pub message_pool: Vec<u8>,
}
/// Ejecuta 0x8B de forma transaccional; el llamante aporta los dos bytes indeterminados del heap.
pub fn create_request(
    record: &[u8],
    merchants: &[Merchant],
    pool: &[u8],
    op: [u32; 5],
    calendar: Calendar,
    enabled: bool,
    heap_tail: [u8; 2],
) -> Result<SupplyRequestOperation, String> {
    if op[0] != 0x8b {
        return Err("operación distinta de 0x8B".into());
    }
    let a = op[1].to_le_bytes();
    let b = op[2].to_le_bytes();
    let city = usize::from(b[1]);
    if p3_save::world::siege_record_len(record, city).map_err(|e| format!("registro: {e:?}"))? != record.len() {
        return Err("longitud de asedio incompatible".into());
    }
    let at = 18 + usize::from(record[17]) * 8;
    let count = usize::from(record[at]);
    if count == 255 {
        return Err("demasiadas solicitudes de asedio".into());
    }
    let end = at + 1 + count * 16;
    let mut rows = record[at + 1..end]
        .chunks_exact(16)
        .map(|x| x.try_into().unwrap())
        .collect::<Vec<[u8; 16]>>();
    let letter = siege::create_supply_request(
        &mut rows,
        city as u8,
        b[2],
        b[3],
        [a[0], a[1], a[2], a[3], b[0]],
        op[3],
        heap_tail,
    );
    let mut updated = record[..at].to_vec();
    updated.push(rows.len() as u8);
    for row in &rows {
        updated.extend_from_slice(row)
    }
    updated.extend_from_slice(&record[end..]);
    let mut merchants = merchants.to_vec();
    let target = merchants.get_mut(usize::from(b[2])).ok_or("destinatario ausente")?;
    let mut message_pool = pool.to_vec();
    crate::messages::add_siege_supply_request(
        &mut message_pool,
        target,
        calendar.day,
        calendar.month,
        calendar.year,
        &letter,
        enabled,
    )?;
    Ok(SupplyRequestOperation {
        record: updated,
        merchants,
        message_pool,
    })
}
/// Inicia una ciudad sin murallas en modo sin operación adicional ni presentación.
pub fn start_unwalled(
    record: &[u8],
    town: &Town,
    pool: &[u8],
    calendar: Calendar,
    tick: u32,
    difficulty: u8,
    variant: u8,
    strength: u8,
    mode: u8,
) -> Result<UnwalledOperation, String> {
    let city = *record.get(4).ok_or("registro truncado")?;
    if p3_save::world::siege_has_scene(record, usize::from(city)).map_err(|e| format!("registro: {e:?}"))? {
        return Err("la ciudad ya tiene escena de asedio".into());
    }
    let militia = std::array::from_fn(|i| town.u8(0x998 + i));
    let start = siege::begin_unwalled(
        town.i32(0x2d4),
        difficulty,
        variant,
        strength,
        militia,
        tick,
        town.u32(0x2c8),
        mode,
    )
    .map_err(str::to_string)?;
    let mut record = record.to_vec();
    let flag_at = record.len() - 1;
    record[7] = 2;
    record[flag_at] = 1;
    record.extend(p3_save::siege_scene::encode(&start.image).map_err(str::to_string)?);
    let mut town = town.clone();
    town.set_u32(0x2c8, start.flags);
    let mut message_pool = pool.to_vec();
    let mut notice = [0u8; 16];
    notice[4] = 0x53;
    notice[5] = city;
    notice[8..10].copy_from_slice(&start.notice_troops.to_le_bytes());
    crate::messages::add_global_inline_letter(&mut message_pool, calendar.day, calendar.month, calendar.year, &notice)?;
    Ok(UnwalledOperation {
        record,
        town,
        message_pool,
    })
}

/// Cierra la escena que ya no tiene cuenta atrás ni atacantes; devuelve si fue eliminada.
/// El original también libera sus estructuras; aquí solo se admite una escena sin vista reconstruida.
pub fn cleanup_finished_scene(record: &mut Vec<u8>, town: &mut Town) -> Result<bool, String> {
    let city = usize::from(*record.get(4).ok_or("registro truncado")?);
    let Some(scene) = p3_save::world::siege_scene(record, city).map_err(|e| format!("escena: {e:?}"))? else {
        return Ok(false);
    };
    if scene[0x1c..0x1e] != [0, 0] || scene[0x29..0x2b] != [0, 0] {
        return Err("escena de asedio aún activa o con espera".into());
    }
    let flag_at = record.len() - scene.len() - 1;
    record.truncate(flag_at + 1);
    record[flag_at] = 0;
    town.set_u32(0x2c8, town.u32(0x2c8) & !0x10);
    Ok(true)
}

/// Avanza un tick de marcha sin vista ni murallas, lejos de los umbrales de saqueo y salida.
/// El llamante verifica que la ciudad no tenga muralla y que no haya una vista activa.
pub fn march_unwalled_scene(record: &mut Vec<u8>, town: &Town) -> Result<(), String> {
    march_scene(record,town,false)
}

fn march_scene(record:&mut Vec<u8>,town:&Town,allow_towers:bool)->Result<(),String>{
    let city = usize::from(*record.get(4).ok_or("registro truncado")?);
    let scene = p3_save::world::siege_scene(record, city)
        .map_err(|e| format!("escena: {e:?}"))?
        .ok_or("escena ausente")?;
    let word = |at| u16::from_le_bytes([scene[at], scene[at + 1]]);
    let wait = word(0x1c);
    let attack = word(0x29);
    let approach = word(0x14);
    let timer = word(0x18);
    let half = word(0x1a) / 2;
    if wait != 0 || attack == 0 || approach != 0 || timer <= 1 || timer - 1 == half || (!allow_towers && word(0x4d) != 0) {
        return Err("tick de asedio fuera de marcha sencilla".into());
    }
    let at = record.len() - scene.len();
    record[at + 0x18..at + 0x1a].copy_from_slice(&(timer - 1).to_le_bytes());
    record[at + 0x22] = 1;
    for i in 0..4 {
        let offset = at + 0x2d + 8 * i;
        record[offset..offset + 2].copy_from_slice(&u16::from(town.u8(0x998 + i)).to_le_bytes());
    }
    Ok(())
}

/// Marcha amurallada sin vista, conservando la imagen y el puntero residual de torres.
pub fn march_walled_scene(record:&mut Vec<u8>,image:&mut Vec<u8>,town:&Town)->Result<(),String>{
    let city=usize::from(*record.get(4).ok_or("registro truncado")?);
    let saved=p3_save::world::siege_scene(record,city).map_err(|e|format!("escena: {e:?}"))?.ok_or("escena ausente")?;
    if image.len()!=0x650 || usize::from(image[0x5d3])!=city || image[0x77]!=0 || image[0x78]!=1 || image[0x628..0x62c]!=[0;4]{return Err("marcha con vista o imagen incompatible".into())}
    if p3_save::siege_scene::encode(image).map_err(str::to_string)?!=saved{return Err("escena viva y guardada divergentes".into())}
    let mut next=record.clone();march_scene(&mut next,town,true)?;let mut next_image=image.clone();
    let saved=p3_save::world::siege_scene(&next,city).map_err(|e|format!("escena: {e:?}"))?.ok_or("escena ausente")?;
    p3_save::siege_scene::decode_fields(saved,&mut next_image).map_err(str::to_string)?;*record=next;*image=next_image;Ok(())
}

/// Último paso de marcha amurallada, antes de la espera de retirada.
pub fn finish_walled_march(record:&mut Vec<u8>,image:&mut Vec<u8>,town:&Town)->Result<(),String>{
    let city=usize::from(*record.get(4).ok_or("registro truncado")?);let saved=p3_save::world::siege_scene(record,city).map_err(|e|format!("escena: {e:?}"))?.ok_or("escena ausente")?;
    if image.len()!=0x650 || usize::from(image[0x5d3])!=city || image[0x77]!=0 || image[0x78]!=1 || image[0x628..0x62c]!=[0;4]{return Err("salida con vista o imagen incompatible".into())}
    if p3_save::siege_scene::encode(image).map_err(str::to_string)?!=saved{return Err("escena viva y guardada divergentes".into())}
    let word=|at|u16::from_le_bytes([image[at],image[at+1]]);let duration=word(0x6a);
    if word(0x42)!=0 || word(0x3a)!=0 || word(0x3c)!=1 || word(0x68)/2==0 || word(0x21e)==0{return Err("salida de marcha fuera de fase contrastada".into())}
    let mut next=image.clone();next[0x3c..0x3e].fill(0);next[0x3a..0x3c].copy_from_slice(&duration.wrapping_mul(2).to_le_bytes());
    for i in 0..4{let at=0x232+4*i;next[at..at+2].copy_from_slice(&u16::from(town.u8(0x998+i)).to_le_bytes())}
    let encoded=p3_save::siege_scene::encode(&next).map_err(str::to_string)?;let at=record.len()-saved.len();record.truncate(at);record.extend(encoded);*image=next;Ok(())
}

/// La espera que queda a cero por desbordamiento reinicia la marcha si +0x79 sigue activo.
pub fn restart_walled_march(record:&mut Vec<u8>,image:&mut Vec<u8>,town:&Town,site:&p3_core::buildings::Site,towers:&[u8])->Result<(),String>{
    let city=usize::from(*record.get(4).ok_or("registro truncado")?);let saved=p3_save::world::siege_scene(record,city).map_err(|e|format!("escena: {e:?}"))?.ok_or("escena ausente")?;
    if image.len()!=0x650 || usize::from(image[0x5d3])!=city || image[0x77]!=0 || image[0x78]!=1 || image[0x79]!=1 || image[0x628..0x62c]!=[0;4]{return Err("reinicio de marcha fuera del estado contrastado".into())}
    if p3_save::siege_scene::encode(image).map_err(str::to_string)?!=saved{return Err("escena viva y guardada divergentes".into())}
    let word=|at|u16::from_le_bytes([image[at],image[at+1]]);if word(0x42)!=0 || word(0x3a)!=0 || word(0x3c)!=0 || word(0x21e)==0{return Err("reinicio de marcha fuera de fase".into())}
    let count=usize::from(word(0x242));if count!=0 && (image[0x250..0x254]==[0;4] || towers.len()<count*12){return Err("lista viva de torres truncada".into())}
    for slot in towers.chunks_exact(12).take(count){let id=u32::from_le_bytes(slot[8..12].try_into().unwrap())as usize;if site.records.get(id*8..id*8+8).is_none(){return Err("torre ausente".into())}}
    let mut next=image.clone();next[0x3c..0x3e].copy_from_slice(&word(0x68).to_le_bytes());next[0x242..0x244].fill(0);
    for i in 0..4{let at=0x232+4*i;next[at..at+2].copy_from_slice(&u16::from(town.u8(0x998+i)).to_le_bytes())}
    let encoded=p3_save::siege_scene::encode(&next).map_err(str::to_string)?;let at=record.len()-saved.len();record.truncate(at);record.extend(encoded);*image=next;Ok(())
}

/// Avanza la aproximación amurallada sin vista y fuera del siguiente golpe a edificio.
/// La imagen viva acredita que no hay campo de flujo; S05 por sí solo no lo guarda.
pub fn approach_walled_scene(record: &mut Vec<u8>, image: &mut Vec<u8>, town: &Town) -> Result<(), String> {
    approach_walled_inner(record, image, town, false).map(|_| ())
}

/// En el hito devuelve el edificio cuya tarea 0x26 debe insertarse para el siguiente tick.
pub fn approach_walled_milestone(record: &mut Vec<u8>, image: &mut Vec<u8>, town: &Town) -> Result<u16, String> {
    approach_walled_inner(record, image, town, true)?.ok_or_else(|| "tick sin hito de edificio".into())
}

fn approach_walled_inner(record: &mut Vec<u8>, image: &mut Vec<u8>, town: &Town, expect_milestone: bool) -> Result<Option<u16>, String> {
    let city = usize::from(*record.get(4).ok_or("registro truncado")?);
    let saved = p3_save::world::siege_scene(record, city)
        .map_err(|e| format!("escena: {e:?}"))?.ok_or("escena ausente")?;
    if image.len() != 0x650 || usize::from(image[0x5d3]) != city || image[0x77] != 0 {
        return Err("imagen de aproximación no acreditada o con vista".into());
    }
    if p3_save::siege_scene::encode(image).map_err(str::to_string)? != saved {
        return Err("imagen viva y escena guardada divergentes".into());
    }
    let word = |at| u16::from_le_bytes([image[at], image[at + 1]]);
    let timer = word(0x42);
    if timer == 0 || word(0x3a) != 0 || word(0x21e) == 0 {
        return Err("aproximación fuera de los ticks admitidos".into());
    }
    let first = image[0x6f];
    let milestone = if first != 0xff {
        if first >= image[0x6e] { return Err("índice de objetivo inválido".into()); }
        let at = 0x254 + usize::from(first) * 8;
        if at + 8 > image.len() { return Err("lista de objetivos truncada".into()); }
        (timer - 1 == word(at)).then(|| word(at + 4))
    } else { None };
    if milestone.is_some() != expect_milestone {
        return Err("aproximación en un umbral no admitido".into());
    }
    let mut next = image.clone();
    next[0x42..0x44].copy_from_slice(&(timer - 1).to_le_bytes());
    if milestone.is_some() {
        let next_target = first + 1;
        next[0x6f] = if next_target == next[0x6e] { 0xff } else { next_target };
    }
    for i in 0..4 {
        let at = 0x232 + i * 4;
        next[at..at + 2].copy_from_slice(&u16::from(town.u8(0x998 + i)).to_le_bytes());
    }
    let encoded = p3_save::siege_scene::encode(&next).map_err(str::to_string)?;
    let at = record.len() - saved.len();
    record.truncate(at);
    record.extend(encoded);
    *image = next;
    Ok(milestone)
}

/// Ataque sin vista ni torres. Las resoluciones de derrota siguen excluidas.
pub fn assault_walled_simple(record:&mut Vec<u8>,image:&mut Vec<u8>,town:&mut Town,site:&p3_core::buildings::Site)->Result<(),String>{
    assault_walled(record,image,town,site,&[])
}

/// Ataque sin vista con torres; se detiene antes de destruir una torre.
pub fn assault_walled(record:&mut Vec<u8>,image:&mut Vec<u8>,town:&mut Town,site:&p3_core::buildings::Site,towers:&[u8])->Result<(),String>{
    let mut towers=towers.to_vec();assault_walled_inner(record,image,town,site,&mut towers,false,false).map(|_|())
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct TowerLoss {pub x:u8,pub y:u8,pub city:u8}

/// Avanza el ataque y devuelve la torre destruida; el llamante debe programar la tarea 0x25.
pub fn assault_walled_tower_loss(record:&mut Vec<u8>,image:&mut Vec<u8>,town:&mut Town,site:&p3_core::buildings::Site,towers:&mut Vec<u8>)->Result<Option<TowerLoss>,String>{
    assault_walled_inner(record,image,town,site,towers,true,false).map(|v|v.0)
}

fn assault_walled_inner(record:&mut Vec<u8>,image:&mut Vec<u8>,town:&mut Town,site:&p3_core::buildings::Site,towers:&mut Vec<u8>,allow_loss:bool,allow_victory:bool)->Result<(Option<TowerLoss>,bool),String>{
    let city=usize::from(*record.get(4).ok_or("registro truncado")?);
    let saved=p3_save::world::siege_scene(record,city).map_err(|e|format!("escena: {e:?}"))?.ok_or("escena ausente")?;
    if image.len()!=0x650 || usize::from(image[0x5d3])!=city || image[0x77]!=0 || image[0x79]!=0 {return Err("asalto fuera del estado admitido".into())}
    if p3_save::siege_scene::encode(image).map_err(str::to_string)?!=saved {return Err("escena viva y guardada divergentes".into())}
    let w=|at|u16::from_le_bytes([image[at],image[at+1]]);
    let d=|at|u32::from_le_bytes(image[at..at+4].try_into().unwrap());
    let attacker=w(0x21e);let defender=u16::from(town.u8(0x998));
    if attacker==0 || defender==0 || w(0x42)!=0 || w(0x3a)!=0 || w(0x3c)!=0 {return Err("fuerzas o fase de asalto no admitidas".into())}
    let tower_count=usize::from(w(0x242));
    let has_tower_list=d(0x250)!=0;let active=usize::from(image[0x6c]);
    if tower_count!=0 && !has_tower_list{return Err("lista viva de torres ausente".into())}
    let slots=if has_tower_list{tower_count.max(active+1)}else{0};
    if (tower_count!=0 && active>=tower_count)||towers.len()<slots*12{return Err("torre activa o memoria inválida".into())}
    let mut next_towers=towers.clone();let mut tower_types=Vec::with_capacity(slots);
    for slot in towers.chunks_exact(12).take(slots){
        let id=u32::from_le_bytes(slot[8..12].try_into().unwrap()) as usize;
        let row=site.records.get(id*8..id*8+8).ok_or("torre ausente")?;
        // Con recuento cero el original no consulta el tipo de la ranura de reserva.
        if tower_count!=0 && !matches!(row[3],0x32|0x33){return Err("tipo de torre inválido".into())}
        tower_types.push((row[3],row[6]==0));
    }
    let mut completed=0usize;for _ in 0..tower_count{if tower_types[completed].1{completed+=1}}
    let attackers=std::array::from_fn::<u32,5,_>(|i|u32::from(w(0x21e+4*i)));
    let defenders=std::array::from_fn::<u32,4,_>(|i|u32::from(town.u8(0x998+i)));
    let attacker_other=attackers[1]+attackers[2]+attackers[3];let defender_other=defenders[1]+defenders[2]+defenders[3];
    if attacker_other>u32::from(u16::MAX){return Err("suma de tropas fuera del rango acreditado".into())}
    let breached=image[0x78]!=0;let mut pitch=0u32;
    if !breached && d(4)!=0 {
        let slot=usize::from(image[0x6d]);if slot>=3{return Err("puerta inválida".into())}
        let id=d(0x244+4*slot) as usize;let row=site.records.get(id*8..id*8+8).ok_or("puerta ausente")?;
        if matches!(row[3],0x34|0x35){pitch=400;}
    }
    let mut next=image.clone();let mut next_town=town.clone();
    for i in 0..4{let at=0x232+i*4;next[at..at+2].copy_from_slice(&u16::from(town.u8(0x998+i)).to_le_bytes())}
    let unit_pitch=if breached{0}else{attackers[0].min(defender_other)*40};
    let accumulated=u32::from(image[0x62c]);let stock=town.u32(0x40);let required=(accumulated+pitch+unit_pitch)>>8;
    let (effective,effective_unit)=if stock<required {
        let available=stock.wrapping_mul(256).wrapping_sub(accumulated).wrapping_add(255);
        let remainder=available%40;next[0x62c]=(255-remainder) as u8;next_town.set_u32(0x40,0);
        let available=available-remainder;if available<pitch{(0,0)}else{(pitch,available-pitch)}
    } else {
        next[0x62c]=accumulated.wrapping_add(pitch).wrapping_add(unit_pitch) as u8;next_town.set_u32(0x40,stock-required);(pitch,unit_pitch)
    };
    if effective!=0 {let health=d(4);let new=if health<28{u32::from(health!=0)}else{health-27};next[4..8].copy_from_slice(&new.to_le_bytes());}
    let coefficients=[(24u32,34u32),(32,46),(36,51)];
    let mut attacker_damage=[0u32;5];let mut defender_damage=[0u32;5];let mut attacker_pool=0u32;let mut defender_pool=0u32;
    if breached {attacker_damage[0]=defenders[0]*68;defender_damage[0]=attackers[0]*68;}
    let mut remaining_defenders=defender_other;
    for i in 1..4 {
        let units=defenders[i];let product=units.wrapping_mul(effective_unit/40);
        let selected=if product==0{0}else{(remaining_defenders/2).wrapping_add(product)/remaining_defenders};
        remaining_defenders=remaining_defenders.wrapping_sub(units);
        attacker_damage[0]=attacker_damage[0].wrapping_add(selected.wrapping_mul(68));
        let unselected=units.wrapping_sub(selected);
        if attacker_other==0{attacker_damage[0]=attacker_damage[0].wrapping_add(unselected.wrapping_mul(coefficients[i-1].0));}
        else{attacker_pool=attacker_pool.wrapping_add(unselected.wrapping_mul(coefficients[i-1].1));}
    }
    if attackers[4]==0 {
        if attacker_other==0{attacker_damage[0]=attacker_damage[0].wrapping_add(completed as u32*82)}else{attacker_pool=attacker_pool.wrapping_add(completed as u32*116)}
    }else{attacker_damage[4]=attacker_damage[4].wrapping_add(completed as u32*23)}
    let mut barrier=d(0);
    if defender_other==0 {
        for i in 1..4{
            if next[0x78]==0 && has_tower_list {let coefficient=if tower_types[active].0==0x32{[5u32,6,7][i-1]}else{[2u32,3,3][i-1]};defender_damage[4]=defender_damage[4].wrapping_add(coefficient*attackers[i]);}
            else if next[0x78]==0{let damage=2*attackers[i];if damage<barrier{barrier-=damage}else{barrier=0;next[0x78]=1}}
            else{defender_damage[0]=defender_damage[0].wrapping_add(coefficients[i-1].0*attackers[i]);}
        }
    }else{
        if !breached{defender_pool=attackers[0].min(defender_other)*68;}
        for i in 1..4{defender_pool=defender_pool.wrapping_add(coefficients[i-1].1*attackers[i]);}
    }
    if tower_count!=0 {defender_damage[4]=defender_damage[4].wrapping_add((if tower_types[active].0==0x32{12}else{5})*attackers[4]);}
    else if defender_other!=0{defender_pool=defender_pool.wrapping_add(87*attackers[4]);}
    else if next[0x78]==0{let damage=4*attackers[4];if damage<barrier{barrier-=damage}else{barrier=0;next[0x78]=1}}
    else{defender_damage[0]=defender_damage[0].wrapping_add(61*attackers[4]);}
    next[0..4].copy_from_slice(&barrier.to_le_bytes());
    if u32::from_le_bytes(next[4..8].try_into().unwrap())!=0 && next[0x78]==0 {
        let new=barrier.saturating_sub(23);next[0..4].copy_from_slice(&new.to_le_bytes());
        if barrier<24{next[0x78]=1;}
    }
    let mut remaining_attackers=attacker_other;let mut remaining_defenders=defender_other;
    for i in 1..4 {
        if remaining_attackers!=0{let part=attackers[i].wrapping_mul(attacker_pool)/remaining_attackers;attacker_pool=attacker_pool.wrapping_sub(part);attacker_damage[i]=attacker_damage[i].wrapping_add(part);remaining_attackers-=attackers[i];}
        if remaining_defenders!=0{let part=defenders[i].wrapping_mul(defender_pool)/remaining_defenders;defender_pool=defender_pool.wrapping_sub(part);defender_damage[i]=defender_damage[i].wrapping_add(part);remaining_defenders-=defenders[i];}
    }
    for i in 0..5 {
        let health=d(0x21c+4*i).saturating_sub(attacker_damage[i]);next[0x21c+4*i..0x220+4*i].copy_from_slice(&health.to_le_bytes());
        if i<4 {
            let health=((defenders[i]<<16)|u32::from(w(0x230+4*i))).saturating_sub(defender_damage[i]);next[0x230+4*i..0x234+4*i].copy_from_slice(&health.to_le_bytes());
            if (health>>16)<defenders[i]{siege::remove_militia_unit(&mut next_town,i).map_err(str::to_string)?;}
        }
    }
    let mut tower_loss=None;
    if has_tower_list {
        let health=d(0x240).saturating_sub(defender_damage[4]);
        next[0x240..0x244].copy_from_slice(&health.to_le_bytes());
        let remaining=(health>>16) as usize;
        if remaining<tower_count {
            if !allow_loss{return Err("destrucción de torre aún no admitida".into())}
            if remaining+1!=tower_count || d(0x628)!=0{return Err("destrucción de varias torres o vista activa".into())}
            let id=u32::from_le_bytes(next_towers[active*12+8..active*12+12].try_into().unwrap()) as usize;
            let row=site.records.get(id*8..id*8+8).ok_or("torre destruida ausente")?;
            tower_loss=Some(TowerLoss{x:row[0],y:row[1],city:city as u8});
            let last=(tower_count-1)*12;let dst=active*12;let moved:[u8;8]=next_towers[last+4..last+12].try_into().unwrap();next_towers[dst+4..dst+12].copy_from_slice(&moved);
            let id=u32::from_le_bytes(next_towers[dst+8..dst+12].try_into().unwrap()) as usize;let row=site.records.get(id*8..id*8+8).ok_or("torre trasladada ausente")?;tower_types[active]=(row[3],row[6]==0);
            next[0x6c]=siege::select_active_wall_tower(&site.records,&next_towers,remaining as u16,next_town.i32(0x2e0),next[0x6c]).map_err(str::to_string)?;
            next[0x620..0x624].fill(0xff);next[0x624..0x628].fill(0);
        }
    }
    let read_word=|data:&[u8],at|u16::from_le_bytes([data[at],data[at+1]]);
    if read_word(&next,0x21e)==0 && read_word(&next,0x232)==0{
        add_first_militia(&mut next,&mut next_town);
    }
    let nw=|at|u32::from(read_word(&next,at));
    let strength=nw(0x21e)*72+nw(0x222)*18+nw(0x226)*24+nw(0x22a)*27+nw(0x22e)*280;
    let defense=nw(0x232)*72+nw(0x236)*18+nw(0x23a)*24+nw(0x23e)*27+tower_types.iter().take(completed).map(|(ty,_)|if *ty==0x32{432}else{1080}).sum::<u32>();
    let victory=nw(0x21e)==0 || strength*3<defense;
    if victory && !allow_victory{return Err("resolución de derrota requiere transacción completa".into())}
    if victory {
        next[0x78]=1;next_town.set_u32(0x2c8,next_town.u32(0x2c8)&!0x10);
        let wait=w(0x6a).wrapping_mul(2);next[0x3a..0x3c].copy_from_slice(&wait.to_le_bytes());next[0x242..0x244].fill(0);
        for i in 0..4{let at=0x300+2*i;next_town.set_u16(at,next_town.u16(at).wrapping_add(20));}
    } else if next[0x78]==0 && defense==nw(0x232)*72 && strength<defense {next[0x78]=1;}
    else if next[0x78]!=0 && (nw(0x232)==0 || strength>2*defense){
        next[0x79]=1;let duration=w(0x68);next[0x3c..0x3e].copy_from_slice(&duration.to_le_bytes());next[0x242..0x244].fill(0);
    }
    let encoded=p3_save::siege_scene::encode(&next).map_err(str::to_string)?;let at=record.len()-saved.len();record.truncate(at);record.extend(encoded);*image=next;*town=next_town;*towers=next_towers;Ok((tower_loss,victory))
}

fn add_first_militia(image:&mut [u8],town:&mut Town){
    image[0x232..0x234].copy_from_slice(&1u16.to_le_bytes());
    let poor=town.u32(0x2e0).wrapping_add(20);let beggars=town.i32(0x2e4).saturating_sub(15).max(0) as u32;
    town.set_u32(0x2e0,poor);town.set_u32(0x2e4,beggars);
    let total=town.u32(0x2d8).wrapping_add(town.u32(0x2dc)).wrapping_add(poor).wrapping_add(beggars);town.set_u32(0x2d4,total);
    town.set_u8(0x998,town.u8(0x998).wrapping_add(1));town.set_u16(0x844,town.u16(0x844).wrapping_add(5));
}

/// Avanza un hito y añade la tarea 0x26 sobre copias, conservando los bytes libres del registro.
pub fn schedule_walled_milestone(
    record: &mut Vec<u8>, image: &mut Vec<u8>, town: &Town,
    scheduler: &mut p3_core::scheduler::Scheduler, tick: u32,
) -> Result<(), String> {
    let mut next_record = record.clone(); let mut next_image = image.clone(); let mut next_scheduler = scheduler.clone();
    let building = approach_walled_milestone(&mut next_record, &mut next_image, town)?;
    let slot = add_siege_task(&mut next_scheduler,tick.wrapping_add(1))?;
    let task = &mut next_scheduler.records[usize::from(slot)];
    task.kind = 0x26;
    task.payload[..2].copy_from_slice(&building.to_le_bytes());
    task.set_u8_at(0x10, record[4]); task.set_u8_at(0x11, 1);
    *record = next_record; *image = next_image; *scheduler = next_scheduler;
    Ok(())
}

fn add_siege_task(next_scheduler:&mut p3_core::scheduler::Scheduler,due:u32)->Result<u16,String>{
    let capacity = usize::from(next_scheduler.capacity);
    if next_scheduler.records.len() != capacity { return Err("planificador truncado".into()); }
    if next_scheduler.free_head >= next_scheduler.capacity
        && (next_scheduler.free_head != next_scheduler.capacity || next_scheduler.capacity > u16::MAX - 128) {
        return Err("crecimiento de planificador no admitido".into());
    }
    let mut seen = vec![false; capacity]; let mut index = next_scheduler.head;
    while index < next_scheduler.capacity {
        let at = usize::from(index);
        if seen[at] || index == next_scheduler.free_head { return Err("lista de tareas inválida".into()); }
        seen[at] = true; index = next_scheduler.records[at].next;
    }
    Ok(next_scheduler.add_task(due))
}

/// Simula el asalto y, si cae una torre, inserta su tarea 0x25 de forma transaccional.
pub fn assault_walled_schedule_tower(record:&mut Vec<u8>,image:&mut Vec<u8>,town:&mut Town,site:&p3_core::buildings::Site,towers:&mut Vec<u8>,scheduler:&mut p3_core::scheduler::Scheduler,tick:u32)->Result<(),String>{
    let(mut rec,mut img,mut city,mut heap,mut tasks)=(record.clone(),image.clone(),town.clone(),towers.clone(),scheduler.clone());
    if let Some(loss)=assault_walled_tower_loss(&mut rec,&mut img,&mut city,site,&mut heap)?{
        let id=add_siege_task(&mut tasks,tick.wrapping_add(1))?;let task=&mut tasks.records[usize::from(id)];task.kind=0x25;
        task.payload[..2].copy_from_slice(&u16::MAX.to_le_bytes());task.payload[2..4].fill(0);
        task.payload[4..6].copy_from_slice(&u16::from(loss.x).to_le_bytes());task.payload[6..8].copy_from_slice(&u16::from(loss.y).to_le_bytes());task.set_u8_at(0x10,loss.city);task.set_u8_at(0x11,1);
    }
    *record=rec;*image=img;*town=city;*towers=heap;*scheduler=tasks;Ok(())
}

/// Asalto y victoria defensiva sin vista, con carta y tarea de torre atómicas.
/// El modo de suministros repone la paciencia sin marcar solicitudes al vencer la ciudad.
pub fn assault_walled_transaction(records:&mut Vec<Vec<u8>>,city:usize,image:&mut Vec<u8>,town:&mut Town,site:&p3_core::buildings::Site,towers:&mut Vec<u8>,scheduler:&mut p3_core::scheduler::Scheduler,pool:&mut Vec<u8>,calendar:Calendar,tick:u32)->Result<(),String>{
    let(mut rec,mut img,mut next_town,mut heap,mut tasks,mut letters)=(records.get(city).ok_or("asedio ausente")?.clone(),image.clone(),town.clone(),towers.clone(),scheduler.clone(),pool.clone());
    if usize::from(*rec.get(4).ok_or("registro truncado")?)!=city{return Err("ciudad de asedio incompatible".into())}
    let(loss,victory)=assault_walled_inner(&mut rec,&mut img,&mut next_town,site,&mut heap,true,true)?;
    if let Some(loss)=loss{
        let id=add_siege_task(&mut tasks,tick.wrapping_add(1))?;let task=&mut tasks.records[usize::from(id)];task.kind=0x25;
        task.payload[..2].copy_from_slice(&u16::MAX.to_le_bytes());task.payload[2..4].fill(0);
        task.payload[4..6].copy_from_slice(&u16::from(loss.x).to_le_bytes());task.payload[6..8].copy_from_slice(&u16::from(loss.y).to_le_bytes());task.set_u8_at(0x10,loss.city);task.set_u8_at(0x11,1);
    }
    if victory {
        let mut notice=[0u8;16];notice[1]=0x80;notice[2..4].fill(0xff);notice[4]=0x38;notice[5]=city as u8;
        let start=u16::from_le_bytes([img[0x584],img[0x585]]);let duration=((tick>>8)as u16).wrapping_sub(start).wrapping_add(1);
        notice[8..10].copy_from_slice(&duration.to_le_bytes());notice[10..14].fill(0xff);notice[14]=img[0x6e];
        crate::messages::add_global_inline_letter(&mut letters,calendar.day,calendar.month,calendar.year,&notice)?;
        if img[0x61c]!=0{
            let mut goodwill=u16::from_le_bytes([rec[5],rec[6]]);
            siege::refill_goodwill(city as u8,&mut goodwill,rec[8],next_town.i32(0x2d4),tick,false,&mut [],0);
            rec[5..7].copy_from_slice(&goodwill.to_le_bytes());
        }
    }
    records[city]=rec;*image=img;*town=next_town;*towers=heap;*scheduler=tasks;*pool=letters;Ok(())
}

/// Espera posterior a victoria defensiva sin vista; valida la lista viva de torres.
/// Devuelve true cuando el destructor elimina la escena del registro.
pub fn wait_defensive_victory(record:&mut Vec<u8>,image:&mut Vec<u8>,town:&mut Town,site:&p3_core::buildings::Site,towers:&[u8])->Result<bool,String>{
    let city=usize::from(*record.get(4).ok_or("registro truncado")?);
    let saved=p3_save::world::siege_scene(record,city).map_err(|e|format!("escena: {e:?}"))?.ok_or("escena ausente")?;
    if image.len()!=0x650 || usize::from(image[0x5d3])!=city || image[0x77]!=0 || image[0x78]==0 || image[0x628..0x62c]!=[0;4] {return Err("espera defensiva fuera del estado contrastado".into())}
    if p3_save::siege_scene::encode(image).map_err(str::to_string)?!=saved{return Err("escena viva y guardada divergentes".into())}
    let word=|at|u16::from_le_bytes([image[at],image[at+1]]);let wait=word(0x3a);let duration=word(0x6a);
    if wait==0 || duration==0 || word(0x3c)!=0 || word(0x42)!=0{return Err("fase de espera no admitida".into())}
    let count=usize::from(word(0x242));let has_list=image[0x250..0x254]!=[0;4];
    if count!=0 && (!has_list || towers.len()<count*12){return Err("lista viva de torres truncada".into())}
    if wait>duration{for slot in towers.chunks_exact(12).take(count){let id=u32::from_le_bytes(slot[8..12].try_into().unwrap())as usize;if site.records.get(id*8..id*8+8).is_none(){return Err("torre ausente".into())}}}
    let mut next=image.clone();let mut city_state=town.clone();
    for i in 0..4{let at=0x232+4*i;next[at..at+2].copy_from_slice(&u16::from(town.u8(0x998+i)).to_le_bytes())}
    next[0x3a..0x3c].copy_from_slice(&(wait-1).to_le_bytes());
    if wait<=duration {
        city_state.set_u32(0x2c8,city_state.u32(0x2c8)&!0x10);
        if wait==1{
            let flag_at=record.len()-saved.len()-1;record.truncate(flag_at+1);record[flag_at]=0;*town=city_state;return Ok(true)
        }
    }else{
        if image[0x79]==0{
            let defender=u32::from(town.u8(0x998));
            if word(0x21e)!=0{
                let health=u32::from_le_bytes(next[0x21c..0x220].try_into().unwrap());next[0x21c..0x220].copy_from_slice(&health.saturating_sub(defender*68).to_le_bytes());
            }else{
                let mut remaining=(1..4).map(|i|u32::from(word(0x21e+4*i))).sum::<u32>();
                if remaining>u32::from(u16::MAX){return Err("suma de tropas fuera del rango acreditado".into())}
                if remaining==0{
                    let health=u32::from_le_bytes(next[0x22c..0x230].try_into().unwrap());next[0x22c..0x230].copy_from_slice(&health.saturating_sub(defender*20).to_le_bytes());
                    if word(0x22e)==0 && defender!=0{let gate=u32::from_le_bytes(next[4..8].try_into().unwrap());if gate!=0{next[4..8].copy_from_slice(&gate.saturating_sub(defender*14).max(1).to_le_bytes());}}
                }else{
                    let mut damage=defender*97;
                    for i in 1..4{let units=u32::from(word(0x21e+4*i));if remaining!=0{let part=units*damage/remaining;damage-=part;remaining-=units;let at=0x21c+4*i;let health=u32::from_le_bytes(next[at..at+4].try_into().unwrap());next[at..at+4].copy_from_slice(&health.saturating_sub(part).to_le_bytes());}}
                }
            }
        }
        if next[0x21e..0x220]==[0,0] && town.u8(0x998)==0{add_first_militia(&mut next,&mut city_state)}
    }
    let encoded=p3_save::siege_scene::encode(&next).map_err(str::to_string)?;let at=record.len()-saved.len();record.truncate(at);record.extend(encoded);*image=next;*town=city_state;Ok(false)
}

/// Indicadores vivos que `0x627AD0` reconstruye al leer una escena S05.
pub fn scene_flags_after_load(record: &[u8], city: usize, tick: u32) -> Result<Option<(bool, bool)>, String> {
    let Some(scene) = p3_save::world::siege_scene(record, city).map_err(|e| format!("escena: {e:?}"))? else { return Ok(None); };
    let timer = u16::from_le_bytes([scene[0x18], scene[0x19]]);
    let wait = u16::from_le_bytes([scene[0x1c], scene[0x1d]]);
    let visible = timer != 0 || wait != 0;
    let last = u32::from_le_bytes(record[..4].try_into().unwrap());
    let retreat = visible && !(timer == 0 && last.wrapping_add(0x1e00) <= tick);
    Ok(Some((visible, retreat)))
}

/// Byte `+0x78` de la escena guardada tras la carga: `0x627AD0` lo recalcula (`visible`) en lugar de leerlo.
pub fn set_loaded_scene_visible(record: &mut [u8], city: usize, visible: bool) -> Result<(), String> {
    let len = p3_save::world::siege_scene(record, city).map_err(|e| format!("escena: {e:?}"))?.ok_or("escena ausente")?.len();
    let at = record.len() - len;
    record[at + 0x22] = u8::from(visible);
    Ok(())
}

/// Imagen y lista mínima necesarias para el asedio sin vista tras cargar S05.
pub struct RecoveredWalled {pub image:Vec<u8>,pub tower_bytes:Vec<u8>}

/// No reconstruye la vista ni acredita los tiempos de disparo de las torres.
pub fn recover_walled_headless(record: &mut Vec<u8>, town: &Town, site: &p3_core::buildings::Site, level: u8) -> Result<RecoveredWalled, String> {
    recover_walled(record,town,site,level,None,false)
}

/// Reconstruye la escena en espera, incluida su bandera viva según el reloj.
pub fn recover_walled_wait(record:&mut Vec<u8>,town:&Town,site:&p3_core::buildings::Site,level:u8,tick:u32)->Result<RecoveredWalled,String>{
    recover_walled(record,town,site,level,Some(tick),false)
}

/// Reconstruye las torres y banderas de una marcha amurallada guardada.
pub fn recover_walled_march(record:&mut Vec<u8>,town:&Town,site:&p3_core::buildings::Site,level:u8)->Result<RecoveredWalled,String>{
    recover_walled(record,town,site,level,None,true)
}

fn recover_walled(record:&mut Vec<u8>,town:&Town,site:&p3_core::buildings::Site,level:u8,wait_tick:Option<u32>,march:bool)->Result<RecoveredWalled,String>{
    let city=usize::from(*record.get(4).ok_or("registro truncado")?);
    if usize::from(town.index())!=city || level>2 {return Err("ciudad o muralla incompatibles".into())}
    let saved=p3_save::world::siege_scene(record,city).map_err(|e|format!("escena: {e:?}"))?.ok_or("escena ausente")?;
    let word=|at|u16::from_le_bytes([saved[at],saved[at+1]]);
    if (word(0x29)==0 && wait_tick.is_none()) || (wait_tick.is_some() != (word(0x1c)!=0)) || (march && (word(0x14)!=0 || word(0x18)==0)) || (!march && word(0x14)==0 && word(0x18)!=0) {return Err("escena fuera de fase recuperable sin vista".into())}
    if site.records.len()!=usize::from(town.u16(0x776))*8 {return Err("registros de ciudad incompatibles".into())}
    let mut image=vec![0u8;0x650];image[0x5d3]=city as u8;
    p3_save::siege_scene::decode_fields(saved,&mut image).map_err(str::to_string)?;
    let gates=siege::choose_wall_gate(&site.records,town.u16(0x768),town.u16(0x762),level,0).map_err(str::to_string)?;
    for (i,gate) in gates.gates.iter().enumerate(){image[0x244+4*i..0x248+4*i].copy_from_slice(&gate.to_le_bytes())}
    let stride=usize::try_from(town.i32(0x7ac)).map_err(|_|"paso de plano inválido")?;
    if !siege::compute_wall_gate_cells(&mut image,&site.records,&site.cells,stride).map_err(str::to_string)? {return Err("puerta seleccionada ausente".into())}
    let capacity=u16::from_le_bytes([image[0x242],image[0x243]]);let selected=usize::from(image[0x6d]);
    let heap=vec![0u8;usize::from(capacity)*12];
    let towers=siege::list_wall_towers(&site.records,town.u16(0x768),gates.gates[selected],capacity,0,&heap).map_err(str::to_string)?;
    image[0x242..0x244].copy_from_slice(&towers.count.to_le_bytes());
    if towers.allocated{image[0x250..0x254].copy_from_slice(&0x23100000u32.to_le_bytes())}
    image[0x6c]=siege::select_active_wall_tower(&site.records,&towers.bytes,towers.count,town.i32(0x2e0),image[0x6c]).map_err(str::to_string)?;
    image[0x78]=u8::from(wait_tick.is_some() || march);image[0x79]=if march{1}else{wait_tick.map_or(0,|tick|u8::from(u32::from_le_bytes(record[..4].try_into().unwrap()).wrapping_add(0x1e00)>tick))};image[0x578]=0xff;
    let encoded=p3_save::siege_scene::encode(&image).map_err(str::to_string)?;
    let at=record.len()-saved.len();record.truncate(at);record.extend(encoded);
    Ok(RecoveredWalled{image,tower_bytes:towers.bytes})
}

/// Último tick de marcha y dos ticks de retirada sin vista ni murallas.
/// `retreat_active` representa el campo vivo +0x79, ausente de S05; debe
/// acreditarse mediante la creación o reconstrucción de la escena original.
pub fn retreat_unwalled_scene(record: &mut Vec<u8>, town: &mut Town, retreat_active: bool) -> Result<(), String> {
    let city = usize::from(*record.get(4).ok_or("registro truncado")?);
    let scene = p3_save::world::siege_scene(record, city).map_err(|e| format!("escena: {e:?}"))?.ok_or("escena ausente")?;
    let word = |at| u16::from_le_bytes([scene[at], scene[at + 1]]);
    let (timer, wait, duration) = (word(0x18), word(0x1c), word(0x1e));
    if word(0x14) != 0 || word(0x29) == 0 || word(0x4d) != 0 || duration != 1 {
        return Err("retirada fuera de escena sin vista admitida".into());
    }
    let at = record.len() - scene.len();
    let next_wait = match (timer, wait) {
        (1, 0) if word(0x1a) / 2 != 0 => 2u16,
        (0, 2) => 1u16,
        (0, 1) => {
            let flag_at = at - 1;
            record.truncate(flag_at + 1);
            record[flag_at] = 0;
            town.set_u32(0x2c8, town.u32(0x2c8) & !0x10);
            return Ok(());
        }
        _ => return Err("retirada de asedio no simulada o estado interno desconocido".into()),
    };
    if timer == 0 && wait == 2 && !retreat_active {
        let offset = at + 0x27;
        let attacker = u32::from_le_bytes(record[offset..offset + 4].try_into().unwrap());
        let damage = u32::from(town.u8(0x998)) * 68;
        record[offset..offset + 4].copy_from_slice(&attacker.saturating_sub(damage).to_le_bytes());
    }
    record[at + 0x18..at + 0x1a].copy_from_slice(&0u16.to_le_bytes());
    record[at + 0x1c..at + 0x1e].copy_from_slice(&next_wait.to_le_bytes());
    record[at + 0x22] = 1;
    for i in 0..4 {
        let offset = at + 0x2d + 8 * i;
        record[offset..offset + 2].copy_from_slice(&u16::from(town.u8(0x998 + i)).to_le_bytes());
    }
    Ok(())
}

/// Ejecuta las ayudas y el desgaste en su orden real; no despacha 0x8E.
pub fn daily(
    record: &[u8],
    towns: &[Town],
    offices: &[Vec<u8>],
    merchants: &[Merchant],
    pool: &[u8],
    rng: Option<CrtRand>,
    env: &DailyInput<'_>,
) -> Result<DailySnapshot, String> {
    daily_with_live_scene(record,towns,offices,merchants,pool,rng,env,None)
}

/// `live_flag` conserva +0x78 cuando no hubo lectura de S05 entre dos ticks.
pub fn daily_with_live_scene(
    record:&[u8],towns:&[Town],offices:&[Vec<u8>],merchants:&[Merchant],pool:&[u8],rng:Option<CrtRand>,env:&DailyInput<'_>,live_flag:Option<u8>,
) -> Result<DailySnapshot,String> {
    let city = usize::from(*record.get(4).ok_or("registro de asedio truncado")?);
    let len = p3_save::world::siege_record_len(record, city).map_err(|e| format!("registro de asedio: {e:?}"))?;
    if len != record.len() {
        return Err("longitud de asedio incompatible".into());
    }
    if city >= towns.len() {
        return Err("ciudad de asedio ausente".into());
    }
    let (mut record, mut towns, mut offices, mut merchants, mut message_pool, mut rng) = (
        record.to_vec(),
        towns.to_vec(),
        offices.to_vec(),
        merchants.to_vec(),
        pool.to_vec(),
        rng,
    );
    if env.disabled {
        return Ok(DailySnapshot {
            record,
            towns,
            offices,
            merchants,
            message_pool,
            rng,
            operation: None,
        });
    }
    let mut goodwill = u16::from_le_bytes([record[5], record[6]]);
    let mut cooldown = record[8];
    let mut wares = [record[9], record[11], record[13], record[15]];
    let levels = [record[10], record[12], record[14], record[16]];
    let count = usize::from(record[17]);
    let request_at = 18 + 8 * count;
    let request_count = usize::from(record[request_at]);
    let scene_at = request_at + 1 + 16 * request_count;
    let scene_present = record[scene_at] != 0;
    let mut scene_active = false;
    if scene_present {
        let scene = &record[scene_at + 1..];
        let word = |at| u16::from_le_bytes([scene[at], scene[at + 1]]);
        let wait = word(0x1c);
        let attack = word(0x29);
        scene_active = if wait == 0 { attack != 0 } else { wait > word(0x1e) };
        let timer = word(0x18);
        // La lectura lo reconstruye; una escena viva conserva su indicador.
        record[scene_at + 1 + 0x22] = live_flag.unwrap_or(u8::from(timer != 0 || wait != 0));
    }
    let mut agreements = record[18..request_at]
        .chunks_exact(8)
        .map(|v| v.try_into().unwrap())
        .collect::<Vec<[u8; 8]>>();
    let mut requests = record[request_at + 1..request_at + 1 + 16 * request_count]
        .chunks_exact(16)
        .map(|v| v.try_into().unwrap())
        .collect::<Vec<[u8; 16]>>();
    if env.calendar.day == 1 {
        let (stock, consumption) = siege::world_stock_consumption(&towns, &offices).map_err(str::to_string)?;
        let (selected, new_levels) = siege::choose_wares(
            stock,
            consumption,
            towns[city].u32(0x2d4),
            wares,
            levels,
            env.stack_tail.unwrap_or([0; 3]),
        );
        if env.stack_tail.is_none() && selected[1..].contains(&0xff) {
            return Err("niveles de demanda dependen de bytes de pila no conocidos".into());
        }
        wares = selected;
        for i in 0..4 {
            record[9 + 2 * i] = wares[i];
            record[10 + 2 * i] = new_levels[i];
        }
    } else if matches!(env.calendar.day, 8 | 22) {
        siege::process_agreements(
            &mut goodwill,
            &mut agreements,
            &towns[city],
            &mut offices,
            &mut merchants,
        )
        .map_err(str::to_string)?;
    }
    let payments = siege::expire_requests(
        &mut goodwill,
        &mut requests,
        towns[city].u32(0x2d4),
        env.tick,
        &mut merchants,
    )
    .map_err(str::to_string)?;
    for payment in payments {
        let i = usize::from(payment.merchant);
        let merchant = merchants.get_mut(i).ok_or("destinatario de pago ausente")?;
        crate::messages::add_siege_supply_payment(
            &mut message_pool,
            merchant,
            env.calendar.day,
            env.calendar.month,
            env.calendar.year,
            &payment,
            env.message_enabled.get(i).copied().unwrap_or(true),
        )?;
    }
    if env.calendar.day_of_year % 7 == 0 && !scene_active {
        siege::ship_surplus(&mut towns[city], wares, &mut goodwill, &mut cooldown).map_err(str::to_string)?;
    }
    let input = GoodwillInput {
        disabled: false,
        day: env.calendar.day,
        day_of_year: env.calendar.day_of_year,
        year: env.calendar.year,
        difficulty: env.difficulty,
        town: city as u8,
        town_count: towns.len() as u32,
        regions: env.regions,
        population: towns[city].i32(0x2d4),
        cooldown,
        goodwill,
        tick: env.tick,
        last_plunder: u32::from_le_bytes(record[..4].try_into().unwrap()),
        scene: scene_present,
        scene_active,
        network_client: env.network_client,
    };
    let outcome = siege::goodwill(&input, &mut rng).map_err(str::to_string)?;
    record[5..7].copy_from_slice(&outcome.goodwill.to_le_bytes());
    record[8] = cooldown;
    let mut serialized = record[..17].to_vec();
    serialized.push(u8::try_from(agreements.len()).map_err(|_| "demasiados acuerdos")?);
    for row in &agreements {
        serialized.extend_from_slice(row)
    }
    serialized.push(u8::try_from(requests.len()).map_err(|_| "demasiadas solicitudes")?);
    for row in &requests {
        serialized.extend_from_slice(row)
    }
    serialized.extend_from_slice(&record[scene_at..]);
    Ok(DailySnapshot {
        record: serialized,
        towns,
        offices,
        merchants,
        message_pool,
        rng,
        operation: outcome.operation,
    })
}
/// Resultado transaccional del saqueo territorial de una escena sin vista ni murallas.
#[derive(Clone, Debug)]
pub struct PlunderTransition {
    pub records: Vec<Vec<u8>>,
    pub town: Town,
    pub offices: Vec<Vec<u8>>,
    pub message_pool: Vec<u8>,
}

/// Ejecuta el umbral medio de la marcha; el llamante comprueba que no haya murallas ni vista.
pub fn plunder_unwalled_scene(
    records: &[Vec<u8>], city: usize, town: &Town, offices: &[Vec<u8>],
    merchants: &[Merchant], pool: &[u8], calendar: Calendar, tick: u32,
) -> Result<PlunderTransition, String> {
    plunder_scene(records,city,town,offices,merchants,pool,calendar,tick,false)
}

fn plunder_scene(
    records:&[Vec<u8>],city:usize,town:&Town,offices:&[Vec<u8>],merchants:&[Merchant],pool:&[u8],calendar:Calendar,tick:u32,allow_towers:bool,
)->Result<PlunderTransition,String>{
    let mut records = records.to_vec();
    let mut requests = Vec::with_capacity(records.len());
    let mut request_ranges = Vec::with_capacity(records.len());
    for (i, record) in records.iter().enumerate() {
        if p3_save::world::siege_record_len(record, i).map_err(|e| format!("registro: {e:?}"))? != record.len() {
            return Err("longitud de asedio incompatible".into());
        }
        let start = 19 + usize::from(record[17]) * 8;
        let end = start + usize::from(record[start - 1]) * 16;
        requests.push(record[start..end].chunks_exact(16).map(|v| v.try_into().unwrap()).collect::<Vec<[u8; 16]>>());
        request_ranges.push((start, end));
    }
    let record = records.get(city).ok_or("asedio de ciudad ausente")?;
    let scene = p3_save::world::siege_scene(record, city).map_err(|e| format!("escena: {e:?}"))?.ok_or("escena ausente")?;
    let word = |at| u16::from_le_bytes([scene[at], scene[at + 1]]);
    let timer = word(0x18);
    if word(0x1c) != 0 || word(0x29) == 0 || word(0x14) != 0 || (!allow_towers && word(0x4d) != 0) || timer == 0 || timer - 1 != word(0x1a) / 2 {
        return Err("umbral de saqueo fuera de escena admitida".into());
    }
    let retreat_duration=word(0x1e);
    let scene_at = record.len() - scene.len();
    let start_day = u16::from_le_bytes([scene[0x10], scene[0x11]]);
    let mode = scene[0x25] != 0;
    let targets = scene[0x4f];
    let last = u32::from_le_bytes(record[..4].try_into().unwrap());
    let mut town = town.clone();
    town.set_u32(0x2c8, town.u32(0x2c8) & !0x10);
    let mut offices = offices.to_vec();
    let result = siege::plunder_without_convoy(&mut town, &mut offices, merchants, tick, last, 0).map_err(str::to_string)?;
    let record = &mut records[city];
    record[..4].copy_from_slice(&result.last_tick.to_le_bytes());
    record[scene_at + 0x18..scene_at + 0x1a].copy_from_slice(&(timer - 1).to_le_bytes());
    if timer==1{record[scene_at+0x1c..scene_at+0x1e].copy_from_slice(&retreat_duration.wrapping_mul(2).to_le_bytes());}
    record[scene_at + 0x22] = 1;
    for i in 0..4 {
        let offset = scene_at + 0x2d + 8 * i;
        record[offset..offset + 2].copy_from_slice(&u16::from(town.u8(0x998 + i)).to_le_bytes());
    }
    let mut notice = [0u8; 16];
    notice[1] = 0x80; notice[2..4].fill(0xff); notice[4] = 0x38; notice[5] = city as u8;
    notice[8..10].copy_from_slice(&((tick >> 8) as u16).wrapping_sub(start_day).wrapping_add(1).to_le_bytes());
    notice[10..14].copy_from_slice(&result.cash.to_le_bytes());
    notice[14..16].copy_from_slice(&u16::from(targets).to_le_bytes());
    let mut message_pool = pool.to_vec();
    crate::messages::add_global_inline_letter(&mut message_pool, calendar.day, calendar.month, calendar.year, &notice)?;
    if mode {
        let mut goodwill = u16::from_le_bytes([record[5], record[6]]);
        siege::refill_goodwill(city as u8, &mut goodwill, record[8], town.i32(0x2d4), tick, true, &mut requests, 0);
        record[5..7].copy_from_slice(&goodwill.to_le_bytes());
        for (record, (rows, (start, end))) in records.iter_mut().zip(requests.iter().zip(request_ranges)) {
            for (out, row) in record[start..end].chunks_exact_mut(16).zip(rows) { out.copy_from_slice(row); }
        }
    }
    Ok(PlunderTransition { records, town, offices, message_pool })
}

/// Saqueo amurallado sin vista, con imagen viva y todas sus consecuencias transaccionales.
pub fn plunder_walled_scene(records:&[Vec<u8>],city:usize,image:&[u8],town:&Town,offices:&[Vec<u8>],merchants:&[Merchant],pool:&[u8],calendar:Calendar,tick:u32)->Result<(PlunderTransition,Vec<u8>),String>{
    let record=records.get(city).ok_or("asedio ausente")?;let saved=p3_save::world::siege_scene(record,city).map_err(|e|format!("escena: {e:?}"))?.ok_or("escena ausente")?;
    if image.len()!=0x650 || usize::from(image[0x5d3])!=city || image[0x77]!=0 || image[0x78]!=1 || image[0x628..0x62c]!=[0;4]{return Err("saqueo con vista o imagen incompatible".into())}
    if p3_save::siege_scene::encode(image).map_err(str::to_string)?!=saved{return Err("escena viva y guardada divergentes".into())}
    let out=plunder_scene(records,city,town,offices,merchants,pool,calendar,tick,true)?;let mut next_image=image.to_vec();
    let next=p3_save::world::siege_scene(&out.records[city],city).map_err(|e|format!("escena: {e:?}"))?.ok_or("escena ausente")?;p3_save::siege_scene::decode_fields(next,&mut next_image).map_err(str::to_string)?;Ok((out,next_image))
}

/// Escena amurallada preparada, junto con los bloques vivos del asedio.
#[derive(Clone, Debug)]
pub struct WalledOperation {
    pub record: Vec<u8>,
    pub town: Town,
    pub message_pool: Vec<u8>,
    pub image: Vec<u8>,
    pub tower_heap: Vec<u8>,
    pub list_heap: Vec<u8>,
}

/// Prepara el inicio y la carta de forma transaccional; sin ventana de ciudad.
pub fn start_walled(
    record: &[u8], town: &Town, pool: &[u8], calendar: Calendar,
    input: &siege::WalledStartInput<'_>,
) -> Result<WalledOperation, String> {
    let city = *record.get(4).ok_or("registro truncado")?;
    if city != input.city || input.flags != town.u32(0x2c8) || input.population != town.i32(0x2d4)
        || input.population_0 != town.i32(0x2d8) || input.population_2 != town.i32(0x2e0)
        || input.militia != std::array::from_fn(|i| town.u8(0x998 + i)) {
        return Err("ciudad de escena incompatible".into());
    }
    if p3_save::world::siege_has_scene(record, usize::from(city)).map_err(|e| format!("registro: {e:?}"))? {
        return Err("la ciudad ya tiene escena de asedio".into());
    }
    let start = siege::begin_walled(input).map_err(str::to_string)?;
    if start.outside_overflow { return Err("escritura original fuera de la reserva de negocios; faltan efectos del montón".into()); }
    let mut record = record.to_vec(); let flag = record.len() - 1;
    record[7] = 2; record[flag] = 1;
    record.extend(p3_save::siege_scene::encode(&start.image).map_err(str::to_string)?);
    let mut town = town.clone(); town.set_u32(0x2c8, start.flags);
    let mut message_pool = pool.to_vec(); let mut notice = [0u8; 16];
    notice[4] = 0x53; notice[5] = city; notice[8..10].copy_from_slice(&start.notice_troops.to_le_bytes());
    crate::messages::add_global_inline_letter(&mut message_pool, calendar.day, calendar.month, calendar.year, &notice)?;
    Ok(WalledOperation { record, town, message_pool, image: start.image, tower_heap: start.tower_heap, list_heap: start.list_heap })
}
