//! Guardado completo tras avanzar: reúne lo que la simulación modifica y lo escribe sobre el modelo S05 original.

use crate::Sim;
use p3_save::model::SaveModel;
use p3_save::world::World;

/// Acumuladores de dinero del comerciante que tanto la economía como las oficinas incrementan (salarios 0x480..0x48C,
/// ventas 0x4B0/0x4B4, 0x98): si los dos cambian se suman las dos variaciones (aritmética con desbordamiento).
const ACCUMULATORS: [usize; 7] = [0x98, 0x480, 0x484, 0x488, 0x48C, 0x4B0, 0x4B4];

impl Sim {
    /// Aplica altas de oficinas antes de los serializadores que esperan vectores de igual tamaño.
    pub(crate) fn apply_created_offices(&self,model:&mut SaveModel)->Result<(),String>{
        let initial=self.host.office_pool_initial.as_ref().ok_or("pool inicial de oficinas ausente")?;let live=self.current_office_pool()?;
        if live.free==initial.free&&live.records.len()==initial.records.len(){return Ok(())}
        let before=p3_save::office_factory::pool_from_world(&model.world)?;if before.free!=initial.free&&before.free!=live.free{return Err("cabeza libre de oficinas modificada por otro sistema".into())}let raw=model.world.write();let parsed=World::parse(&raw).map_err(|e|format!("mundo: {e:?}"))?;
        let old_towns=(0..parsed.town_count()).map(|i|parsed.town(i)).collect::<Vec<_>>();if old_towns.len()!=self.economy.world.towns.len(){return Err("ciudades de oficina incompatibles".into())}let mut towns=old_towns.clone();for (dst,src) in towns.iter_mut().zip(&self.economy.world.towns){dst.set_u16(0x784,src.u16(0x784));}
        let old_merchants=model.world.merchants.iter().map(|r|p3_core::ai::reputation::Merchant{bytes:p3_save::layout::image(&p3_save::layout::MERCHANT,0x650,r)}).collect::<Vec<_>>();if old_merchants.len()!=self.rep_merchants.len(){return Err("comerciantes de oficina incompatibles".into())}let mut merchants=old_merchants.clone();for (dst,src) in merchants.iter_mut().zip(&self.rep_merchants){dst.set_u16(0xc,src.u16(0xc));}
        if before.records.len()>live.records.len(){return Err("pool guardado mayor que el vivo".into())}let mut after=before.clone();
        for i in 0..live.records.len(){let created=i>=initial.records.len()||u16::from_le_bytes([initial.records[i][0x2c4],initial.records[i][0x2c5]])==u16::MAX;if i>=after.records.len(){after.records.push(live.records[i].clone());}else if created&&u16::from_le_bytes([live.records[i][0x2c4],live.records[i][0x2c5]])!=u16::MAX{let stored=u16::from_le_bytes([after.records[i][0x2c4],after.records[i][0x2c5]]);let owner=u16::from_le_bytes([live.records[i][0x2c4],live.records[i][0x2c5]]);if stored!=u16::MAX&&stored!=owner{return Err("oficina nueva modificada por otro sistema".into())}if stored==u16::MAX{after.records[i]=live.records[i].clone();}}}
        after.free=live.free;p3_save::office_factory::apply(&mut model.world,&before,&after,&old_towns,&towns,&old_merchants,&merchants)
    }
    /// Amplía el mundo serializado con las ciudades que la sesión ha fundado.
    /// Conserva las ciudades existentes y valida su matriz antes de añadir.
    pub(crate) fn apply_founded_cities(&self,model:&mut SaveModel)->Result<(),String>{
        let live=self.economy.world.towns.len();let old=model.world.towns.len();
        if old==live{return Ok(())}if old>live{return Err("el guardado tiene más ciudades que la simulación".into())}
        let source=self.host.founded_world.as_ref().ok_or("mundo fundado ausente")?;
        if source.towns.len()!=live||source.matrix.len()!=live*live||source.sieges.len()!=live||source.header.len()<0x48+live||model.world.header.len()<0x48+old{return Err("mundo fundado incompleto".into())}
        if model.world.header[0x20..0x20+old]!=source.header[0x20..0x20+old]||model.world.header[0x48..0x48+old]!=source.header[0x48..0x48+old]{return Err("índices de ciudad modificados".into())}
        for n in old..live{
            let mut matrix=vec![0;(n+1)*(n+1)];for r in 0..=n{matrix[r*(n+1)..(r+1)*(n+1)].copy_from_slice(&source.matrix[r*live..r*live+n+1]);}
            let town=self.economy.world.towns.get(n).ok_or("ciudad fundada ausente")?;let site=self.host.sites.get(n).ok_or("plano fundado ausente")?;let entries=self.economy.world.tick_entries.get(n).ok_or("suministros fundados ausentes")?;
            let saved=p3_save::model::TownModel::from_live(town,site,entries)?;
            model.world.append_town(saved,source.header[0x20+n],source.header[0x48+n],matrix,source.sieges[n].clone())?;
        }
        Ok(())
    }

    /// Actualiza el historial sin sustituir los centros ni otros datos S04.
    pub fn persist_town_history(&self,model:&mut SaveModel)->Result<(),String>{
        let (initial,current)=match (&self.host.town_history_initial,&self.host.town_history){(Some(a),Some(b))=>(a,b),(None,None)=>return Ok(()),_=>return Err("historial urbano no compatible".into())};
        let rest=p3_save::world_rest::WorldRest::parse(&model.world.rest).map_err(|e|format!("cola de mundo: {e:?}"))?;
        let end=rest.centers.first().ok_or("centros ausentes")?.start;let saved=&model.world.rest[..end];
        if saved!=initial && saved!=current && !self.host.town_history_versions.contains(&crate::save_version(saved)){return Err("historial urbano modificado por otro sistema".into())}
        let mut out=current.clone();out.extend_from_slice(&model.world.rest[end..]);model.world.rest=out;Ok(())
    }

    /// Aplica los centros S04 con detección de cambios concurrentes. Es idempotente
    /// y conserva el historial y el resto de la cola de mundo.
    pub fn persist_trade_centers(&self, model: &mut SaveModel) -> Result<(), String> {
        let (initial,current)=match (&self.host.trade_centers_initial, &self.host.trade_centers) {
            (Some(initial),Some(current)) => (initial,current),
            (None,None) => return Ok(()),
            _ => return Err("centros comerciales no compatibles".into()),
        };
        let rest = p3_save::world_rest::WorldRest::parse(&model.world.rest).map_err(|e| format!("cola de mundo: {e:?}"))?;
        if rest.centers.len()!=initial.len() || current.len()!=initial.len() { return Err("número de centros comerciales modificado".into()); }
        let mut out=Vec::with_capacity(model.world.rest.len());
        let mut offset=0;
        for (i,range) in rest.centers.iter().enumerate() {
            let stored=rest.center(i);
            if stored!=initial[i] && stored!=current[i] && !self.host.trade_center_versions.get(i).is_some_and(|versions|versions.contains(&crate::save_version(stored))) { return Err(format!("centro comercial {i} modificado por otro sistema")); }
            out.extend_from_slice(&model.world.rest[offset..range.start]);
            out.extend_from_slice(&current[i]);
            offset=range.end;
        }
        out.extend_from_slice(&model.world.rest[offset..]);
        model.world.rest=out;
        Ok(())
    }

    /// Guarda juntos los campos de mundo que también cambian fuera del tick
    /// urbano. Rechaza conflictos sin modificar el modelo de entrada.
    pub fn persist_session_world_extras(&self,model:&mut SaveModel)->Result<(),String>{
        let mut staged=model.clone();
        self.persist_first_discovery(&mut staged)?;
        self.persist_weather_marker(&mut staged)?;
        self.persist_town_hall_notices(&mut staged)?;
        *model=staged;Ok(())
    }

    /// GameWorld+0x89: al leer S05 todo byte no nulo se normaliza a 1.
    /// Se comprueba la versión antes de modificar el modelo preparado.
    fn persist_first_discovery(&self,model:&mut SaveModel)->Result<(),String>{
        let current=self.host.first_discovery_flag;
        if current>1{return Err("bandera de primer descubrimiento inválida".into())}
        let saved=model.world.header.get_mut(0x10).ok_or("cabecera de descubrimiento truncada")?;
        if *saved!=self.host.first_discovery_initial&&*saved!=current{return Err("bandera de descubrimiento modificada por otro sistema".into())}
        *saved=current;Ok(())
    }

    /// Actualiza solo GameWorld+0x8F, preservando la cola variable del mundo.
    fn persist_weather_marker(&self,model:&mut SaveModel)->Result<(),String>{
        let (Some(initial),Some(current))=(self.host.weather_marker_initial,self.host.weather_marker)else{return Ok(())};let last=*self.host.weather_marker_saved.borrow();if current==initial&&last.is_none(){return Ok(())}
        let saved=p3_save::world_rest::WorldRest::parse(&model.world.rest).map_err(|e|format!("cola de mundo: {e:?}"))?.field_8f;if saved!=initial&&saved!=current&&Some(saved)!=last{return Err("marcador meteorológico modificado por otro sistema".into())}let slot=model.world.rest.last_mut().ok_or("marcador meteorológico ausente")?;*slot=current;Ok(())
    }

    fn persist_town_hall_notices(&self,model:&mut SaveModel)->Result<(),String>{
        let (Some(initial),Some(current))=(self.host.town_hall_notices_initial,self.host.town_hall_notices)else{return Ok(())};let last=*self.host.town_hall_notices_saved.borrow();if current==initial&&last.is_none(){return Ok(())}
        let saved=model.world.header.get_mut(0x72..0xc6).ok_or("tablón urbano ausente")?;if saved!=initial&&saved!=current&&last.is_none_or(|v|saved!=v){return Err("tablón urbano modificado por otro sistema".into())}saved.copy_from_slice(&current);Ok(())
    }

    /// Persiste las listas variables de asedio sin reemplazar otros datos del mundo.
    pub fn persist_sieges(&self,model:&mut SaveModel)->Result<(),String>{
        let current=&self.host.founded_world.as_ref().ok_or("mundo de asedio ausente")?.sieges;
        if current.len()!=model.world.sieges.len()||self.host.siege_initial.len()>current.len(){return Err("número de asedios incompatible".into())}
        for (i,(saved,live)) in model.world.sieges.iter().zip(current).enumerate(){
            let initial=self.host.siege_initial.get(i).unwrap_or(live);
            if saved!=initial&&saved!=live&&!self.host.siege_versions.get(i).is_some_and(|versions|versions.contains(&crate::save_version(saved))){return Err(format!("asedio {i} modificado por otro sistema"))}
        }
        model.world.sieges.clone_from(current);
        Ok(())
    }

    /// El juego original tiene un solo registro por comerciante; la simulación mantiene dos copias de la imagen
    /// (`economy.merchants` y `rep_merchants`) que modifican sistemas distintos dentro de un mismo paso. Al terminar el
    /// paso se funden de forma tripartita con la imagen de partida `base`: lo que cambió una sola copia se toma de ella,
    /// los acumuladores cambiados por las dos suman las variaciones y el resto de conflictos conserva el de oficinas
    /// (se cuentan en `merchant_conflicts`). Tras la llamada las dos copias son idénticas.
    pub fn reconcile_merchants(&mut self, base: &[p3_core::ai::reputation::Merchant]) {
        for i in 0..self.rep_merchants.len().min(self.economy.merchants.len()).min(base.len()) {
            if self.economy.merchants[i].bytes == base[i].bytes && self.rep_merchants[i].bytes == base[i].bytes {
                continue;
            }
            let b = &base[i].bytes;
            let e = self.economy.merchants[i].bytes.clone();
            let r = &mut self.rep_merchants[i].bytes;
            let mut j = 0;
            while j < r.len().min(e.len()).min(b.len()) {
                if ACCUMULATORS.contains(&j) && j + 4 <= r.len() {
                    let w = |x: &[u8]| i32::from_le_bytes(x[j..j + 4].try_into().unwrap());
                    let (bw, ew, rw) = (w(b), w(&e), w(r));
                    if ew == rw {
                        j += 4;
                        continue;
                    }
                    if ew != bw && rw != bw {
                        self.merchant_double_changes += 1;
                    }
                    let merged = bw.wrapping_add(ew.wrapping_sub(bw)).wrapping_add(rw.wrapping_sub(bw));
                    r[j..j + 4].copy_from_slice(&merged.to_le_bytes());
                    j += 4;
                    continue;
                }
                if e[j] != b[j] {
                    if r[j] == b[j] {
                        r[j] = e[j];
                    } else if r[j] != e[j] {
                        self.merchant_conflicts += 1;
                    }
                }
                j += 1;
            }
            self.economy.merchants[i].bytes.clone_from(&self.rep_merchants[i].bytes);
        }
    }

    /// Publica los campos de carga que el lector original normalizó, con
    /// comprobación transaccional frente a las imágenes de la sesión.
    pub fn persist_loaded_ship_normalization(&self,model:&mut SaveModel)->Result<(),String>{
        let mut staged = model.clone();
        self.persist_first_discovery(&mut staged)?;
        // El lector original normaliza existencias y valor al reconstruir cada
        // barco. Publica esa normalización también si no hubo un tick naval.
        // Acepta un registro ya normalizado o ya actualizado por esta sesión,
        // pero no sustituye cantidades modificadas por un agente externo.
        for (i, original) in self.host.expedition_original_ship_images.iter().enumerate() {
            // Las altas posteriores no pasaron por el lector original. Su
            // carga se persiste con la fábrica, incluso si amplió el pool.
            if self.host.expedition_factory_initial.as_ref().is_some_and(|f|
                f.ships.get(i).is_none_or(|s|u16::from_le_bytes([s[0x134],s[0x135]])>=0x17)) {continue}
            let Some(original)=original else {continue};
            let mut loaded=original.clone();p3_core::ships::cargo::recompute_load(&mut loaded);
            let offsets=(0..24).map(|k|0x54+4*k).chain(std::iter::once(0x114));
            let before=match staged.ships.ship_image(i){Ok(v)=>v,Err(e)=>{if staged.ships.ships.get(i).is_some_and(|r|r.len()==5){continue}else{return Err(e)}}};
            let mut after=before.clone();
            let current=self.host.navigation.as_ref().and_then(|n|n.snapshot.ships.get(i*0x180..(i+1)*0x180)).or_else(||self.host.expedition_ship_images.get(i).and_then(Option::as_deref));
            for at in offsets {
                let old=&original[at..at+4];let normalized=&loaded[at..at+4];if old==normalized{continue}
                let stored=&before[at..at+4];
                if stored==old {after[at..at+4].copy_from_slice(normalized)}
                else if stored!=normalized && current.and_then(|v|v.get(at..at+4))!=Some(stored){return Err(format!("carga del barco {i} modificada por otro sistema en +0x{at:X}"))}
            }
            if before!=after{staged.ships.apply_ship_image(i,&before,&after)?;}
        }
        *model=staged;Ok(())
    }

    /// Escribe en `model` el estado actual: todas las ciudades (campos guardados y suministros), lo que ya persiste
    /// `persist_construction_city` (oficinas, negocios, comerciantes, buzón, tareas, barcos tocados), el reloj
    /// (`GameWorld+0..+5`, tick en `+0x18`) y la fecha de los ajustes. Transaccional: si falla, `model` no cambia.
    pub fn persist_all(&self, model: &mut SaveModel) -> Result<(), String> {
        self.persist_all_inner(model,None,true,false)
    }

    /// Prepara un guardado completo sin confirmar sus puntos de control. El
    /// anfitrión debe verificar y publicar el resultado antes de confirmarlos.
    pub fn stage_all(&self, model: &mut SaveModel) -> Result<(), String> {
        self.persist_all_inner(model,None,false,false)
    }

    /// Prepara una base privada para conciliar el asalto. No es una partida
    /// guardable: el tick sigue detenido antes de puerto y debe reanudarse.
    pub fn stage_pirate_boundary_baseline(&mut self, model:&SaveModel)->Result<(),String>{
        let (index,expected)=self.host.pending_pirate_attack.ok_or("ataque pirata pendiente ausente")?;
        if self.host.pending_pirate_boundary.is_none()
            || self.host.pending_pirate_target.is_none()
            || self.host.pirate_boundary_model.is_some()
            || self.host.council_scheduler.head!=index
            || self.host.council_scheduler.records.get(usize::from(index))!=Some(&expected)
            || self.host.council_scheduler.dispatching!=0
            || self.host.town_attack_active.is_some()
            || self.host.town_attack_save_unreconciled
            || !self.host.town_attack_effects.is_empty()
            || !self.host.town_attack_frame_effects.is_empty()
            || !self.host.town_attack_manager_effects.is_empty() || !self.host.town_attack_global_news.is_empty()
        {return Err("corte de ataque pirata inconsistente".into())}
        let live=self.capture_pirate_live()?;
        let mut staged=model.clone();
        self.persist_all_inner(&mut staged,None,false,true)?;
        self.host.pirate_boundary_live=Some(live);
        self.host.pirate_boundary_model=Some(staged);
        Ok(())
    }

    /// Captura la base de una batalla ya cargada, antes de publicar efectos.
    /// Valida la reconciliación sin cambios y no habilita el guardado general.
    pub fn stage_loaded_pirate_boundary_baseline(&mut self,model:&SaveModel)->Result<(),String>{
        if self.host.pirate_boundary_model.is_some() || self.host.pirate_boundary_live.is_some()
            || self.host.pending_pirate_attack.is_some() || self.host.pending_pirate_boundary.is_some()
            || self.host.clock_before_battle || self.host.town_attack_tick_pending
            || !self.host.council_operations.is_empty() || !self.host.town_attack_effects.is_empty()
            || !self.host.town_attack_frame_effects.is_empty() || !self.host.town_attack_manager_effects.is_empty()
            || !self.host.town_attack_global_news.is_empty() {
            return Err("base cargada de asalto inconsistente".into());
        }
        let active=self.host.town_attack_active.as_ref().ok_or("asalto cargado ausente")?;
        let battle=active.populated.as_ref().ok_or("escena poblada ausente")?;
        if active.original_block!=model.sea_battles || !active.groups.is_empty() || active.object!=battle.state.object
            || crate::pirate_active_frame::serialize_single_active_attack(&battle.state).map_err(str::to_owned)?!=model.sea_battles {
            return Err("gestor cargado de otro S05 o modificado".into());
        }
        let collision=self.host.town_attack_collision.as_ref().ok_or("índice cargado ausente")?;
        if crate::battle_spatial::into_saved(collision).map_err(str::to_owned)?!=battle.collisions {
            return Err("índice cargado desincronizado".into());
        }
        let live=self.capture_pirate_live()?;
        let checked=crate::pirate_empty_scene_publish::prepare_active_save(model,&live,&live)?;
        if checked.write()!=model.write(){return Err("base cargada requiere reconciliación adicional".into())}
        self.host.pirate_boundary_live=Some(live);self.host.pirate_boundary_model=Some(model.clone());Ok(())
    }

    pub fn has_pirate_boundary_baseline(&self)->bool{self.host.pirate_boundary_model.is_some()}

    /// Prepara S05 de un ataque poblado desde la base anterior al asalto.
    /// No confirma el guardado ni habilita la recarga de sesión activa.
    pub fn stage_populated_pirate_attack(&self)->Result<SaveModel,String>{
        if self.host.pending_pirate_attack.is_some() || self.host.pending_pirate_boundary.is_some()
            || self.host.clock_before_battle || self.host.town_attack_tick_pending
            || self.host.clock_partial_error.is_some() || self.host.navigation_error.is_some()
            || !self.host.town_attack_effects.is_empty() || !self.host.town_attack_frame_effects.is_empty()
            || !self.host.town_attack_manager_effects.is_empty() || !self.host.council_operations.is_empty() {
            return Err("asalto, tick u operaciones pendientes de completar".into());
        }
        let active=self.host.town_attack_active.as_ref().ok_or("asalto activo ausente")?;
        if active.groups.iter().any(|g|!g.is_empty()) {return Err("listas auxiliares de asalto no admitidas".into())}
        let battle=active.populated.as_ref().ok_or("asalto poblado ausente")?;
        if active.object!=battle.state.object {return Err("vistas de escena poblada desincronizadas".into())}
        let collision=self.host.town_attack_collision.as_ref().ok_or("índice de asalto ausente")?;
        if crate::battle_spatial::into_saved(collision).map_err(str::to_owned)?!=battle.collisions {
            return Err("índice de escena poblada desincronizado".into());
        }
        let base=self.host.pirate_boundary_model.as_ref().ok_or("base S05 anterior ausente")?;
        let prior=self.host.pirate_boundary_live.as_ref().ok_or("vistas anteriores ausentes")?;
        let mut expected_messages=base.message_pool.clone();for (date,letter) in &self.host.town_attack_global_news{
            p3_state::messages::add_global_inline_letter(&mut expected_messages,date.day,date.month,date.year,letter)?;
        }
        if expected_messages!=self.host.message_pool {return Err("depósito de mensajes cambiado durante el asalto".into())}
        let current=self.capture_pirate_live()?;
        let mut source=prior.clone();source.scene=current.scene.clone();
        let mut model=crate::pirate_empty_scene_publish::prepare_active_save(base,&source,&current)?;model.message_pool=expected_messages;
        // El lector original inserta en cabeza cada suministro urbano: una
        // escritura posterior a la carga invierte los registros completos.
        // No se permiten cambios de suministros aún no conciliados.
        if crate::pirate_active_frame::parse_single_active_attack(&base.sea_battles,active.address,active.slot).is_ok() {
            let original=base.to_save();let parsed=p3_save::world::World::parse(original.block(0)).map_err(|e|format!("mundo S05: {e:?}"))?;
            if parsed.towns.len()!=self.economy.world.tick_entries.len() || model.world.towns.len()!=parsed.towns.len(){return Err("suministros urbanos incompletos".into())}
            for city in 0..parsed.towns.len(){let mut entries=parsed.tick_entries(city);entries.reverse();
                if entries!=self.economy.world.tick_entries[city]{return Err(format!("suministros urbanos modificados: {city}"))}
                let tail=&mut model.world.towns[city].facilities_and_tail;let start=p3_save::world::FACILITIES_LEN+0x55;
                let end=start.checked_add(entries.len().checked_mul(16).ok_or("suministros excesivos")?).ok_or("suministros excesivos")?;
                let records=tail.get_mut(start..end).ok_or("suministros truncados")?;
                let mut full=records.chunks_exact(16).map(|v|v.to_vec()).collect::<Vec<_>>();full.reverse();
                for (dst,src) in records.chunks_exact_mut(16).zip(full){dst.copy_from_slice(&src)}
            }
            // La reparación original ocurre antes del daño posterior: la
            // duración de la ruta depende de la vida anterior del barco.
            let n=parsed.towns.len();let order=base.world.header.get(0x20..0x20+n).ok_or("orden de ciudades truncado")?;
            let raw=base.world.header.get(0x18..0x1c).ok_or("reloj guardado truncado")?;
            let tick=u32::from_le_bytes(raw.try_into().unwrap());
            let resources=self.host.foundation_navigation.as_ref().ok_or("faltan recursos para reparar rutas cargadas")?;
            let mut combat_ships=Vec::with_capacity(battle.state.local_ships.len());
            for ship in &battle.state.local_ships {
                let raw=ship.get(4..8).ok_or("barco local truncado")?;
                let id=u32::from_le_bytes(raw.try_into().unwrap()) as usize;
                if id>=prior.nav.ships.len()/0x180 || combat_ships.contains(&id){return Err("barcos de asalto inconsistentes".into())}
                combat_ships.push(id);
            }
            crate::loaded_route::normalize_loaded_routes_before_damage_with_speed(&mut model.ships,&prior.nav,&current.nav,resources,order,tick,&combat_ships)?;
        }
        let expected=model.sea_battles.clone();
        let world=&self.host.navigation.as_ref().ok_or("flota de asalto ausente")?.snapshot.ships;
        crate::pirate_active_frame::stage_active_attack_save(&model,&expected,&battle.state,world,world)
    }

    /// Reconcilia la escena terminada sobre la instantánea privada anterior.
    /// No confirma puntos de control ni modifica la sesión si falla.
    pub fn stage_completed_pirate_attack(&self)->Result<SaveModel,String>{
        if !self.host.town_attack_save_unreconciled || self.host.town_attack_active.is_some()
            || self.host.pending_pirate_attack.is_some() || self.host.pending_pirate_boundary.is_some()
            || self.host.clock_before_battle || self.host.town_attack_tick_pending
            || self.host.clock_partial_error.is_some() || self.host.navigation_error.is_some()
            || !self.host.town_attack_effects.is_empty() || !self.host.town_attack_frame_effects.is_empty()
            || !self.host.town_attack_manager_effects.is_empty() || !self.host.council_operations.is_empty()
        {return Err("asalto, tick u operaciones pendientes de completar".into())}
        let model=self.host.pirate_boundary_model.as_ref().ok_or("base S05 anterior ausente")?;
        let live=self.host.pirate_boundary_live.as_ref().ok_or("vistas anteriores ausentes")?;
        let active=self.host.pirate_active_live.as_ref().ok_or("imagen del asalto ausente")?;
        let original=p3_save::tail::CurrentSeaBattles::parse(&model.sea_battles).map_err(|e|format!("gestor anterior: {e:?}"))?;
        let groups=crate::battle_manager::groups_from_save(&original).map_err(str::to_owned)?;
        let baseline=crate::pirate_empty_scene_save::Baseline{model:model.clone(),live:live.clone(),groups};
        let finished=self.capture_pirate_live()?;
        let finished_groups=self.idle.battle_manager.as_ref().map(|m|m.groups.as_slice()).unwrap_or(&[]);
        let mut reconciled=crate::pirate_empty_scene_save::prepare_save(model,&baseline,active,&finished,finished_groups,&self.host.message_pool,&self.host.town_attack_effects)?;
        // El tick reanudado también puede cambiar asedios y otros sistemas.
        // Aplicar sus serializadores sobre la escena reconciliada evita perder
        // esos cambios, sin confirmar checkpoints antes de validar la copia.
        self.persist_all_inner(&mut reconciled,None,false,true)?;
        Ok(reconciled)
    }

    /// Solo se llama después de validar y publicar la copia reconciliada.
    pub fn confirm_completed_pirate_attack(&mut self,model:&SaveModel)->Result<(),String>{
        let expected=self.stage_completed_pirate_attack()?;
        if expected.write()!=model.write(){return Err("S05 de asalto no coincide con la transacción preparada".into())}
        self.confirm_full_save(model)?;
        self.host.town_attack_global_news.clear();self.host.pirate_boundary_model=None;
        self.host.pirate_boundary_live=None;
        self.host.pirate_active_live=None;
        self.host.town_attack_save_unreconciled=false;
        Ok(())
    }

    /// Confirma un guardado completo ya validado por el anfitrión.
    pub fn confirm_full_save(&self, model: &SaveModel) -> Result<(), String> {
        let mission=p3_save::tail::script_snapshots(&model.scheduled_tasks)?;
        let factory=self.host.expedition_factory.as_ref().map(|f|{let mut normalized=f.clone();for captain in &mut normalized.captains{captain[14]=u8::from(captain[14]!=0)}(model.ships.clone(),normalized)});
        self.confirm_save_checkpoints(model,true,mission,factory)
    }

    /// Reconcilia el gestor naval con el último guardado completo de esta
    /// sesión. La confirmación se hace solo al terminar toda la transacción.
    pub fn persist_battle_manager(&self,model:&mut SaveModel)->Result<(),String>{
        let Some(manager)=&self.idle.battle_manager else{return Ok(())};
        let current=&model.sea_battles;
        if current!=&manager.original_block&&self.host.battle_manager_saved.borrow().as_ref()!=Some(current){return Err("gestor de batallas modificado por otro sistema".into())}
        let mut staged=model.clone();staged.sea_battles=manager.original_block.clone();manager.persist(&mut staged)?;
        model.sea_battles=staged.sea_battles;Ok(())
    }

    /// Registra la imagen solo después de publicar un guardado completo sin
    /// una batalla activa añadida por el anfitrión.
    pub fn confirm_battle_manager_save(&self,model:&SaveModel){
        if self.idle.battle_manager.is_some(){*self.host.battle_manager_saved.borrow_mut()=Some(model.sea_battles.clone());}
    }

    /// Solo confirma bandas tras publicar la transacción completa de S05.
    pub fn confirm_pirate_band_save(&self,model:&SaveModel)->Result<(),String>{
        let full=if self.host.pirate_band_images_initial.is_some(){Some(p3_save::pirate_bands::read(&model.ships.tail).map_err(str::to_owned)?)}else{None};
        if let Some(bands)=self.host.mission_pirates{*self.host.pirate_bands_saved.borrow_mut()=Some(bands);}
        *self.host.pirate_band_images_saved.borrow_mut()=full;
        Ok(())
    }

    /// Guarda la sesión y una batalla inicial pendiente en la misma transacción.
    /// La simulación de la batalla corresponde al anfitrión naval; las
    /// operaciones, diálogos y peticiones open_for_player deben estar atendidos.
    pub fn persist_initial_encounter(&self,model:&mut SaveModel,slot:u8)->Result<(),String>{
        if self.host.clock_before_battle{return Err("tick de encuentro incompleto".into())}
        let battle=self.host.encounter_battle.as_ref().ok_or("no hay encuentro inicial")?;
        if !battle.operations.is_empty()||battle.dialog.is_some()||battle.open_for_player_requested||battle.object.get(0xac8)!=Some(&1){return Err("efectos de encuentro pendientes de despacho".into())}
        let input=battle.initial_save_input().map_err(str::to_owned)?;
        let nav=self.host.navigation.as_ref().ok_or("flota de encuentro ausente")?;
        if slot==u8::MAX||battle.object.len()<0x664||u16::from_le_bytes([battle.object[0x662],battle.object[0x663]])!=u16::from(slot){return Err("ranura de encuentro incompatible".into())}
        p3_save::model::validate_initial_convoy_battle(&nav.snapshot,&input)?;
        self.persist_all_inner(model,Some((&input,slot)),true,false)
    }

    /// La fábrica puede cambiar el enlace de una ranura libre antes de que
    /// navegación la active. Solo acepta esa versión intermedia si el modelo
    /// de entrada coincide con la base naval o un guardado propio completo.
    fn activate_weather_after_factory(&self,staged:&mut SaveModel,model:&SaveModel,i:usize,old:&[u8],now:&[u8])->Result<(),String>{
        let error=match staged.ships.apply_activated_weather_ship(i,old,now){Ok(())=>return Ok(()),Err(e)=>e};
        let initial=self.host.expedition_factory_initial.as_ref().and_then(|f|f.ships.get(i));
        let current=self.host.expedition_factory.as_ref().and_then(|f|f.ships.get(i));
        let (Some(initial),Some(current))=(initial,current) else{return Err(error)};
        if initial==current||current.len()!=0x180||u16::from_le_bytes([current[0x134],current[0x135]])<0x17{return Err(error)}
        let baseline=[old[0x134],old[0x135],old[6],old[7],old[0x137]];
        let base_matches=model.ships.ships.get(i).is_some_and(|r|r.as_slice()==baseline);
        let saved_matches=self.host.saved_factory_checkpoint.borrow().as_ref().is_some_and(|(saved,_)|saved==&model.ships);
        if !base_matches&&!saved_matches{return Err(error)}
        staged.ships.apply_activated_weather_ship(i,current,now)
    }

    fn persist_all_inner(&self, model:&mut SaveModel, encounter:Option<(&p3_core::ships::battle::BattleSaveInput<'_>,u8)>, confirm:bool, pirate_baseline:bool)->Result<(),String>{
        if !pirate_baseline {self.require_no_active_town_attack_save()?;}
        if self.saved_battle_pending.as_ref().is_some_and(|original|model.sea_battles.as_slice()!=original.as_slice()){return Err("batalla cargada modificada o ausente en el guardado".into())}
        if let Some(error)=&self.host.clock_partial_error{return Err(format!("no se puede guardar un tick incompleto: {error}"))}
        if !self.host.scheduled_office_updates.is_empty(){return Err("oficinas de tareas pendientes de sincronizar".into())}
        if self.host.pending_ai_expansion.is_some()||self.host.pending_office.is_some()||self.host.pending_auction_office.is_some(){return Err("creación de oficina pendiente de completar".into())}
        if self.host.pending_founding.is_some(){return Err("fundación programada pendiente de completar".into())}
        if self.host.pending_pirate_approach.is_some(){return Err("aproximación pirata pendiente de completar".into())}
        if !self.host.council_operations.is_empty() {
            return Err("no se puede guardar con operaciones pendientes de despacho".into());
        }
        if self.host.clock_before_battle || (self.host.encounter_battle.is_some() && encounter.is_none()) {
            return Err("no se puede guardar un encuentro pendiente de integrar".into());
        }
        if let Some(e) = &self.host.navigation_error {
            return Err(format!("no se puede guardar una navegación interrumpida: {e}"));
        }
        let mut staged = model.clone();
        if let Some(pool)=&self.host.route_stop_runtime{pool.persist_into(&mut staged)?;}
        self.persist_loaded_ship_normalization(&mut staged).map_err(|e|format!("normalización naval: {e}"))?;
        self.persist_town_history(&mut staged)?;
        self.persist_trade_centers(&mut staged)?;
        match (&self.host.trade_offer_raw_initial,&self.host.trade_offers){
            (Some(before),Some(after))=>{let parsed=p3_save::world_rest::WorldRest::parse(&staged.world.rest).map_err(|e|format!("cola de mundo: {e:?}"))?;let raw=&staged.world.rest[parsed.offers.clone()];if raw!=before && !self.host.trade_offer_versions.borrow().iter().any(|v|v==raw){return Err("ofertas comerciales modificadas por otro sistema".into())}let saved=parsed.offer_records();p3_save::world_rest::replace_offers(&mut staged.world.rest,&saved,after)?;},
            (None,None)=>(),_=>return Err("ofertas comerciales no compatibles".into()),
        }
        self.persist_weather_marker(&mut staged)?;
        self.persist_town_hall_notices(&mut staged)?;
        // Una escritura repetida del mismo encuentro ya contiene la batalla;
        // se valida íntegramente más abajo antes de aceptarla.
        let battle_already_saved=if encounter.is_some(){!staged.current_basic_sea_battles().map_err(|e|format!("gestor de batallas inválido: {e:?}"))?.objects.is_empty()}else{false};
        if battle_already_saved&&self.idle.battle_manager.as_ref().is_some_and(|m|m.groups.iter().any(|g|!g.is_empty())){return Err("gestor de batallas con listas auxiliares".into())}
        if !battle_already_saved{self.persist_battle_manager(&mut staged)?;}
        self.apply_founded_cities(&mut staged)?;
        self.persist_sieges(&mut staged)?;
        self.apply_created_offices(&mut staged)?;
        let raw = staged.world.write();
        let old = World::parse(&raw).map_err(|e| format!("mundo: {e:?}"))?;
        if old.town_count() != self.economy.world.towns.len() {
            return Err("número de ciudades distinto del guardado".into());
        }
        for city in 0..old.town_count() {
            let before = old.town(city);
            let current = &self.economy.world.towns[city];
            let site = self.host.sites.get(city).ok_or("plano ausente")?;
            staged.world.towns[city].apply_serialized_town(&before, current, site)?;
            staged.world.towns[city]
                .update_tick_entries(self.economy.world.tick_entries.get(city).ok_or("suministros de ciudad ausentes")?)?;
        }
        self.stage_construction_city(&mut staged, 0).map_err(|e|format!("conciliación de ciudad y flota: {e}"))?;
        if let Some(nav)=&self.host.navigation {
            use p3_save::layout::{SHIP_CORE,SHIP_REST,SHIP_STATE,CONVOY,CONVOY_OBJECT_LEN};
            if nav.baseline.ships.len() % 0x180 != 0 || nav.baseline.ships.len() / 0x180 > staged.ships.ships.len() || nav.snapshot.ships.len() != nav.baseline.ships.len()
                || nav.baseline.convoys.len() % CONVOY_OBJECT_LEN != 0 || nav.baseline.convoys.len() / CONVOY_OBJECT_LEN > staged.ships.convoys.len() || nav.snapshot.convoys.len() != nav.baseline.convoys.len()
                || nav.baseline.routes.len() != nav.baseline.ships.len() / 0x180 || nav.snapshot.routes.len() != nav.baseline.routes.len() {
                return Err("navegación incompatible con la flota guardada".into());
            }
            staged.ships.apply_navigation_captains(&nav.baseline.captains, &nav.snapshot.captains)?;
            staged.ships.apply_navigation_heads(&nav.baseline.manager,&nav.snapshot.manager)?;
            let mut weather_before=nav.baseline.manager.clone();
            // 0x00504230 pone a cero el temporizador visual (+0xA de cada registro de 12 bytes) al
            // cargar y el guardado original escribe ese valor de memoria: la base de comparación de esos
            // dos bytes es lo que había en el archivo, no la imagen ya reiniciada.
            if weather_before.len()>=0xe0&&staged.ships.tail.len()>=0xc0{for i in 0..16{let o=i*12+0xa;weather_before[0x20+o..0x22+o].copy_from_slice(&staged.ships.tail[o..o+2]);}}
            if let Some((saved,_))=self.host.saved_factory_checkpoint.borrow().as_ref().filter(|(ships,_)|ships==&model.ships){
                if saved.tail.len()<0xc0||weather_before.len()<0xe0{return Err("meteorología guardada truncada".into())}
                weather_before[0x20..0xe0].copy_from_slice(&saved.tail[..0xc0]);
            }
            staged.ships.apply_navigation_weather(&weather_before,&nav.snapshot.manager)?;
            // El mismo resultado de ruta puede estar ya guardado; una tercera
            // versión sigue siendo un conflicto y revierte toda la transacción.
            for i in 0..nav.baseline.ships.len()/0x180{let at=i*0x180;let old=&nav.baseline.ships[at..at+0x180];let now=&nav.snapshot.ships[at..at+0x180];let route_changed=nav.baseline.routes.get(i)!=nav.snapshot.routes.get(i);if old==now&&!route_changed{continue}let old_state=u16::from_le_bytes([old[0x134],old[0x135]]);let new_state=u16::from_le_bytes([now[0x134],now[0x135]]);if old_state==0x15&&new_state==0x15&&staged.ships.ships.get(i).is_some_and(|r|r.len()==5){let free=self.host.expedition_factory.as_ref().and_then(|f|f.ships.get(i)).ok_or("ranura meteorológica reciclada ausente")?;staged.ships.apply_activated_weather_ship(i,free,now)?;continue}if old_state>=0x17{
                if new_state>=0x17{staged.ships.apply_free_ship(i,old,now)?}
                else if new_state==0x15{self.activate_weather_after_factory(&mut staged,model,i,old,now)?}
                else{
                    // La fábrica ya materializó la ranura libre en S02; exigir
                    // coincidencia de campos persistidos y ruta antes de aceptarla.
                    let image=staged.ships.ship_image(i)?;
                    for seg in SHIP_CORE.iter().chain(SHIP_REST.iter()).chain(std::iter::once(&SHIP_STATE)){for j in usize::from(seg.obj)..usize::from(seg.obj+seg.len){if !(0x24..0x28).contains(&j)&&image[j]!=now[j]{return Err(format!("alta naval {i} desincronizada en +0x{j:X}"))}}}
                    let block=staged.ships.write();let saved=p3_save::ships::Ships::parse(&block).map_err(|e|format!("rutas: {e:?}"))?;
                    if nav.snapshot.routes.get(i).and_then(Option::as_ref)!=Some(&saved.saved_route(i).map_err(str::to_string)?){return Err("ruta de alta naval desincronizada".into())}
                }
                continue
            }if new_state==0xff{if route_changed&&!(matches!(nav.baseline.routes.get(i),Some(Some(_)))&&matches!(nav.snapshot.routes.get(i),Some(None))){return Err("ruta de baja modificada".into())}if old_state==0x15{staged.ships.apply_expired_weather_ship(i,old,now)?}else{staged.ships.apply_removed_commercial_ship_with_route(i,old,now,nav.baseline.routes.get(i).and_then(Option::as_ref))?}continue}if route_changed{let block=staged.ships.write();let saved=p3_save::ships::Ships::parse(&block).map_err(|e|format!("rutas: {e:?}"))?;let stored=saved.saved_route(i).map_err(|e|format!("ruta {i}: {e}"))?;if nav.baseline.routes.get(i).and_then(Option::as_ref)!=Some(&stored) && nav.snapshot.routes.get(i).and_then(Option::as_ref)!=Some(&stored){return Err(format!("ruta {i} modificada por otro sistema"))}}let before=staged.ships.ship_image(i).map_err(|e|format!("navegación barco {i}, estado anterior {old_state:#x}, nuevo {new_state:#x}: {e}"))?;let mut after=before.clone();for seg in SHIP_CORE.iter().chain(SHIP_REST.iter()).chain(std::iter::once(&SHIP_STATE)){for j in usize::from(seg.obj)..usize::from(seg.obj+seg.len){if old[j]!=now[j]{if !(0x24..0x28).contains(&j)&&before[j]!=old[j]&&before[j]!=now[j]{return Err(format!("barco {i} modificado por otro sistema en +0x{j:X}"))}after[j]=now[j];}}}let cleared=route_changed&&matches!(nav.snapshot.routes.get(i),Some(Some(p3_save::ships::SavedRoute::None)));if cleared{after[0x24..0x28].fill(0);after[0x36]=0}let points=u16::try_from(i).ok().and_then(|id|nav.route_for(id)).or_else(||match nav.snapshot.routes.get(i){Some(Some(p3_save::ships::SavedRoute::Owned(p)))=>Some(p.as_slice()),_=>None});if cleared{staged.ships.apply_ship_route(i,&before,&after,&[],self.economy.world.towns.len())?}else if let Some(points)=points{staged.ships.apply_ship_route(i,&before,&after,points,self.economy.world.towns.len())?}else{if route_changed{return Err("ruta modificada no reconstruida".into())}staged.ships.apply_ship_image(i,&before,&after)?}}
            for i in 0..nav.baseline.convoys.len()/CONVOY_OBJECT_LEN{let at=i*CONVOY_OBJECT_LEN;let old=&nav.baseline.convoys[at..at+CONVOY_OBJECT_LEN];let now=&nav.snapshot.convoys[at..at+CONVOY_OBJECT_LEN];if old==now{continue}let record=&mut staged.ships.convoys[i];let before=p3_save::layout::image(&CONVOY,CONVOY_OBJECT_LEN,record);for seg in &CONVOY{let o=usize::from(seg.obj);let p=usize::from(seg.pos);for k in 0..usize::from(seg.len){if old[o+k]!=now[o+k]{if before[o+k]!=old[o+k]&&before[o+k]!=now[o+k]{return Err(format!("convoy {i} modificado por otro sistema"))}record[p+k]=now[o+k];}}}}
        }
        // Las dos copias del comerciante se unifican al terminar cada paso (`reconcile_merchants`); aun así, si el
        // llamante tocó una sola entre pasos, un byte que solo cambió la economía se toma de ella.
        for i in 0..staged.world.merchants.len() {
            let initial = p3_state::office::PersistedMerchant::from_save_v85(&model.world.merchants[i]).ok_or("comerciante truncado")?;
            let mut out = p3_state::office::PersistedMerchant::from_save_v85(&staged.world.merchants[i]).ok_or("comerciante truncado")?;
            let econ = &self.economy.merchants.get(i).ok_or("comerciante de economía ausente")?.bytes;
            if econ.len() != out.merchant.bytes.len() {
                return Err(format!("comerciante {i}: imagen de economía de longitud distinta"));
            }
            for j in 0..econ.len() {
                if econ[j] != initial.merchant.bytes[j] {
                    if out.merchant.bytes[j] == initial.merchant.bytes[j] {
                        out.merchant.bytes[j] = econ[j];
                    }
                }
            }
            staged.world.merchants[i] = out.to_save_v85();
        }
        self.persist_calendar(&mut staged);
        if let Some((battle,slot))=encounter{
            let old=staged.current_basic_sea_battles().map_err(|e|format!("gestor de batallas inválido: {e:?}"))?;
            if !old.pairs.is_empty(){return Err("gestor de batallas con listas auxiliares".into())}let occupied=!old.objects.is_empty();let prior=staged.sea_battles.clone();
            let mut slots=(0..=usize::from(slot)).map(|_|None).collect::<Vec<_>>();slots[usize::from(slot)]=Some(p3_core::ships::battle::BattleSaveInput{object:battle.object,nexts:battle.nexts,prevs:battle.prevs,bitmap:battle.bitmap,local_ships:battle.local_ships,projectiles:battle.projectiles});
            staged.set_current_basic_sea_battles(&slots,&[])?;
            if occupied&&prior!=staged.sea_battles{return Err("gestor de batallas modificado por otro sistema".into())}
            let block=staged.ships.write();let ships=p3_save::ships::Ships::parse(&block).map_err(|e|format!("flota no recargable: {e:?}"))?;let snapshot=ships.navigation_snapshot().map_err(|e|format!("flota no recargable: {e}"))?;
            p3_save::model::validate_initial_convoy_battle(&snapshot,battle)?;
        }
        let mission_checkpoint=p3_save::tail::script_snapshots(&staged.scheduled_tasks)?;
        let factory_checkpoint=self.host.expedition_factory.as_ref().map(|factory|{let mut normalized=factory.clone();for captain in &mut normalized.captains{captain[14]=u8::from(captain[14]!=0)}(staged.ships.clone(),normalized)});
        if confirm {self.confirm_save_checkpoints(&staged,encounter.is_none(),mission_checkpoint,factory_checkpoint)?;}
        *model = staged;
        Ok(())
    }

    fn confirm_save_checkpoints(&self,model:&SaveModel,confirm_battle:bool,mission_checkpoint:Vec<Option<p3_save::tail::ScriptSnapshot>>,factory_checkpoint:Option<(p3_save::model::ShipsModel,p3_core::ships::factory::HanseFactory)>)->Result<(),String>{
        self.confirm_pirate_band_save(model)?;
        if confirm_battle{self.confirm_battle_manager_save(model);}
        *self.host.mission_saved.borrow_mut()=Some(mission_checkpoint);
        *self.host.saved_factory_checkpoint.borrow_mut()=factory_checkpoint;
        if self.host.trade_offers.is_some(){let parsed=p3_save::world_rest::WorldRest::parse(&model.world.rest).map_err(|e|format!("cola de mundo: {e:?}"))?;let raw=model.world.rest[parsed.offers].to_vec();let mut versions=self.host.trade_offer_versions.borrow_mut();if !versions.contains(&raw){versions.push(raw);}}
        *self.host.weather_marker_saved.borrow_mut()=self.host.weather_marker;
        *self.host.town_hall_notices_saved.borrow_mut()=self.host.town_hall_notices;
        Ok(())
    }

    /// Sincroniza la fecha del mundo y la fecha visible de la cabecera S05.
    pub fn persist_calendar(&self,model:&mut SaveModel){
        let c = self.idle.clock.calendar;
        let h = &mut model.world.header;
        h[0] = c.day;
        h[1] = c.month;
        h[2..4].copy_from_slice(&c.year.to_le_bytes());
        h[4..6].copy_from_slice(&c.day_of_year.to_le_bytes());
        h[0x70]=self.economy.world.alderman_merchant;
        h[0x71]=self.economy.world.alderman_town;
        h[0x12]=if self.economy.world.world_8a!=0{0x7f}else{0};
        h[0x13]=self.idle.clock.expedition_countdown;
        h[0x18..0x1C].copy_from_slice(&self.idle.clock.tick.to_le_bytes());
        let s = 4;
        model.header[s + 0x1A] = c.day;
        model.header[s + 0x1B] = c.month + 1;
        model.header[s + 0x1C..s + 0x1E].copy_from_slice(&c.year.to_le_bytes());
        // Velocidad: el nivel elegido en la ventana de opciones (ajustes `+4`) y `OPS+0x8D4/+0x8D8/+0x8DC`
        // (0x00549E0C).
        if let Some(level) = self.host.speed_level {
            model.header[s + 4..s + 8].copy_from_slice(&level.to_le_bytes());
        }
        let at = 4 + p3_save::SETTINGS_LEN;
        for (k, v) in self.host.time_mode.saved().iter().enumerate() {
            model.header[at + 4 * k..at + 4 * k + 4].copy_from_slice(&v.to_le_bytes());
        }
    }
}
