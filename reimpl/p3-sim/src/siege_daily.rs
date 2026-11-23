//! Puente entre el asedio sin escena y el mundo vivo del reloj C10.
use crate::Sim;
use p3_state::siege::DailyInput;

impl Sim {
    pub(crate) fn create_siege_request(&mut self, op: [u32; 5]) -> Result<(), String> {
        let tail = *self
            .host
            .siege_request_heap_bytes
            .front()
            .ok_or("faltan dos bytes indeterminados del heap")?;
        let fields = op[2].to_le_bytes();
        let city = usize::from(fields[1]);
        let owner = usize::from(fields[2]);
        let merchant = self
            .rep_merchants
            .get(owner)
            .ok_or("destinatario de solicitud ausente")?;
        let enabled = match merchant.bytes[0x38] {
            0..=4 => true,
            5 => false,
            _ => return Err("preferencia de carta oficial fuera de tabla".into()),
        };
        if self.economy.merchants.len() != self.rep_merchants.len() {
            return Err("vistas de comerciante incompatibles".into());
        }
        let record = self
            .host
            .founded_world
            .as_ref()
            .ok_or("mundo de asedio ausente")?
            .sieges
            .get(city)
            .ok_or("asedio de solicitud ausente")?;
        let result = p3_state::siege::create_request(
            record,
            &self.rep_merchants,
            &self.host.message_pool,
            op,
            self.idle.clock.calendar,
            enabled,
            tail,
        )?;
        self.host.founded_world.as_mut().unwrap().sieges[city] = result.record;
        self.economy.merchants.clone_from(&result.merchants);
        self.rep_merchants = result.merchants;
        self.host.message_pool = result.message_pool;
        self.host.siege_request_heap_bytes.pop_front();
        Ok(())
    }
    /// Prepara el inicio sin presentación de ventana, con o sin murallas.
    pub(crate) fn start_siege(
        &mut self,
        city: usize,
        variant: u8,
        strength: u8,
        mode: u8,
    ) -> Result<(), String> {
        let player = usize::from(
            self.host
                .expedition_resources
                .as_ref()
                .ok_or("jugador local no configurado")?
                .player,
        );
        let merchant = self.rep_merchants.get(player).ok_or("jugador local ausente")?;
        let town = self.economy.world.towns.get(city).ok_or("ciudad ausente")?;
        let home = usize::from(merchant.bytes[0x19]);
        // 0x00629CC0..0x00629D36: si la ciudad es la natal del jugador o tiene `Town+0x6F1 == jugador`, el original muestra
        // el aviso 0x15 (0x469380, 0x64F2C1, 0x469AF0): solo presentación, sin cambios al mundo, y sigue con el mismo
        // inicio. La ventana de la escena (+0xC324) no existe sin pantalla, así que solo queda el bit 0x10 de `Town+0x2C8`.
        let mut offer = None;
        let mut next_rng = self.host.investigation_rng;
        if mode != 0 && home != city {
            if home >= self.economy.world.towns.len() {
                return Err("ciudad natal de jugador inválida".into());
            }
            if merchant.bytes[0x39c + home] > 4 {
                let locations = self
                    .host
                    .settlement_locations
                    .as_ref()
                    .ok_or("regiones de asedio ausentes")?;
                let ids = &self.host.expedition_town_ids;
                let a = usize::from(*ids.get(home).ok_or("ciudad natal sin región")?);
                let b = usize::from(*ids.get(city).ok_or("ciudad asediada sin región")?);
                if locations.get(a).ok_or("región natal inválida")?.kind
                    == locations.get(b).ok_or("región asediada inválida")?.kind
                {
                    let rec = self
                        .host
                        .founded_world
                        .as_ref()
                        .ok_or("mundo ausente")?
                        .sieges
                        .get(home)
                        .ok_or("asedio natal ausente")?;
                    p3_save::world::siege_record_len(rec, home).map_err(|e| format!("asedio natal: {e:?}"))?;
                    let at = 18 + usize::from(rec[17]) * 8;
                    let count = usize::from(rec[at]);
                    let rows = &rec[at + 1..at + 1 + count * 16];
                    let rows = rows
                        .chunks_exact(16)
                        .map(|r| r.try_into().unwrap())
                        .collect::<Vec<[u8; 16]>>();
                    offer = p3_core::economy::siege::supply_request_offer(
                        home as u8,
                        player as u8,
                        city as u8,
                        merchant.bytes[0x39c + home],
                        mode,
                        true,
                        &rows,
                        self.idle.clock.tick,
                        &mut next_rng,
                    )
                    .map_err(str::to_string)?;
                }
            }
        }
        let record = self
            .host
            .founded_world
            .as_ref()
            .ok_or("mundo ausente")?
            .sieges
            .get(city)
            .ok_or("registro ausente")?;
        let site = self.host.sites.get(city).ok_or("plano de asedio ausente")?;
        let level = site.wall_expansion_level(town);
        let (record, town, message_pool, walled) = if level == 0xff {
            let result = p3_state::siege::start_unwalled(record, town, &self.host.message_pool, self.idle.clock.calendar,
                self.idle.clock.tick, self.host.siege_difficulty, variant, strength, mode)?;
            (result.record, result.town, result.message_pool, None)
        } else {
            let synthetic;
            let memory = match self.host.siege_walled_heap.front() {
                Some(memory) => memory,
                None => {
                    // Sin memoria explícita: el reloj sale del inicio medido en el original (mismo tick o el siguiente,
                    // misma ciudad) y la memoria dinámica es el montón a cero del oráculo.
                    let tick = self.idle.clock.tick;
                    let clock = self.host.siege_start_clocks.iter().find(|(t, c, _)| usize::from(*c) == city && t.wrapping_sub(tick) <= 1 || usize::from(*c) == city && tick.wrapping_sub(*t) <= 1)
                        .map(|(_, _, clock)| *clock).ok_or("faltan memoria y reloj del asedio amurallado")?;
                    let rows = site.records.len() / 8;
                    synthetic = crate::WalledSiegeHeap { tower_bytes: vec![0; rows * 12 + 12], list_bytes: vec![0; (rows + 0x400) * 2 + 2], tower_address: 0x2330_0000, wall_clock: clock };
                    &synthetic
                }
            };
            let offices = self.current_office_pool()?;
            let input = p3_core::economy::siege::WalledStartInput {
                city: city as u8, records: &site.records, offices: &offices.records,
                defense_head: town.u16(0x768), construction_head: town.u16(0x762), office_head: town.u16(0x784),
                level, grid: &site.cells, stride: town.u32(0x7ac) as usize,
                tower_heap: &memory.tower_bytes, list_heap: &memory.list_bytes,
                tower_address: memory.tower_address, wall_clock: memory.wall_clock,
                militia: std::array::from_fn(|i| town.u8(0x998 + i)), population: town.i32(0x2d4),
                population_0: town.i32(0x2d8), population_2: town.i32(0x2e0), tick: self.idle.clock.tick,
                variant, strength, difficulty: self.host.siege_difficulty, mode, flags: town.u32(0x2c8),
            };
            let result = p3_state::siege::start_walled(record, town, &self.host.message_pool, self.idle.clock.calendar, &input)?;
            let live = crate::LiveWalledSiege { image: result.image, tower_bytes: result.tower_heap, list_bytes: result.list_heap };
            (result.record, result.town, result.message_pool, Some(live))
        };
        self.host.founded_world.as_mut().unwrap().sieges[city] = record;
        if self.host.siege_retreat_flags.len() <= city { self.host.siege_retreat_flags.resize(city + 1, None); }
        self.host.siege_retreat_flags[city] = Some(walled.is_none());
        if self.host.siege_walled_live.len() <= city { self.host.siege_walled_live.resize(city + 1, None); }
        if walled.is_some() { self.host.siege_walled_heap.pop_front(); }
        self.host.siege_walled_live[city] = walled;
        self.economy.world.towns[city] = town;
        self.host.message_pool = message_pool;
        self.host.investigation_rng = next_rng;
        if let Some(op) = offer {
            self.host.council_operations.push(p3_core::pump::Operation(op));
        }
        Ok(())
    }
    pub(crate) fn reject_unsimulated_siege_scenes(&self) -> Result<(), String> {
        // Rechaza antes de su vencimiento las variantes de 0x26 todavía no portadas.
        let scheduler = &self.host.council_scheduler;
        let mut at = scheduler.head; let mut seen = vec![false; usize::from(scheduler.capacity)];
        while at < scheduler.capacity {
            let index = usize::from(at);
            let task = scheduler.records.get(index).ok_or("planificador truncado")?;
            if seen[index] { return Err("lista de tareas cíclica".into()); }
            seen[index] = true;
            if task.kind == 0x25 && task.due <= self.idle.clock.tick.wrapping_add(1) {
                if !self.host.siege_headless_march{return Err("derrumbe requiere sesión sin vista".into())}
                prepare_ruin(*task,&self.economy.world.towns,&self.host.sites).map_err(|e|format!("tarea de derrumbe 0x25: {e}"))?;
            }
            if task.kind == 0x26 && task.due <= self.idle.clock.tick.wrapping_add(1) {
                if !self.host.siege_headless_march { return Err("incendio requiere sesión sin vista".into()); }
                prepare_burn(scheduler, at, &self.economy.world.towns, &self.host.sites, self.economy.world.merchant_count, &self.economy.offices, &self.host.lists, &self.host.housing)
                    .map_err(|e| format!("tarea de incendio 0x26: {e}"))?;
            }
            at = task.next;
        }
        let records = &self
            .host
            .founded_world
            .as_ref()
            .ok_or("mundo de asedio ausente")?
            .sieges;
        for (city, record) in records.iter().enumerate() {
            if let Some(scene) =
                p3_save::world::siege_scene(record, city).map_err(|e| format!("asedio {city}: {e:?}"))?
            {
                if scene[0x1c..0x1e] != [0, 0] || scene[0x29..0x2b] != [0, 0] {
                    let can_march = self.host.siege_headless_march
                        && self
                            .host
                            .sites
                            .get(city)
                            .zip(self.economy.world.towns.get(city))
                            .is_some_and(|(site, town)| {
                                site.wall_expansion_level(town) == 0xff
                                    && (p3_state::siege::march_unwalled_scene(&mut record.clone(), town).is_ok()
                                        || self.host.siege_retreat_flags.get(city).copied().flatten().is_some_and(|flag|
                                            p3_state::siege::retreat_unwalled_scene(&mut record.clone(), &mut town.clone(), flag).is_ok())
                                        || self.prepare_siege_plunder(city, self.idle.clock.tick.wrapping_add(1)).is_ok())
                            });
                    let can_approach = self.host.siege_headless_march
                        && self.host.siege_walled_live.get(city).and_then(Option::as_ref)
                            .zip(self.economy.world.towns.get(city)).zip(self.host.sites.get(city))
                            .is_some_and(|((live, town), site)| site.wall_expansion_level(town) != 0xff
                                && p3_state::siege::approach_walled_scene(
                                    &mut record.clone(), &mut live.image.clone(), town).is_ok());
                    let can_recover = !can_approach && self.host.siege_headless_march
                        && self.prepare_recovered_walled(city).is_ok_and(|(mut record,mut image,towers)| {
                            let town=&self.economy.world.towns[city];let site=&self.host.sites[city];
                            p3_state::siege::approach_walled_scene(&mut record,&mut image,town).is_ok()
                                || p3_state::siege::assault_walled(&mut record,&mut image,&mut town.clone(),site,&towers).is_ok()
                        });
                    let can_assault = self.host.siege_headless_march
                        && self.prepare_walled_assault(city,self.idle.clock.tick.wrapping_add(1)).is_ok();
                    let can_wait = self.host.siege_headless_march && self.prepare_walled_wait(city).is_ok();
                    let can_walled_march = self.host.siege_headless_march && self.prepare_walled_march(city).is_ok();
                    let can_walled_plunder = self.host.siege_headless_march && self.prepare_walled_plunder(city,self.idle.clock.tick.wrapping_add(1)).is_ok();
                    let can_walled_restart = self.host.siege_headless_march && self.prepare_walled_restart(city).is_ok();
                    let can_milestone = !can_march && !can_approach && !can_recover && !can_assault && self.host.siege_headless_march
                        && self.prepare_walled_milestone(city, self.idle.clock.tick.wrapping_add(1)).is_ok();
                    if !can_march && !can_approach && !can_recover && !can_assault && !can_wait && !can_walled_march && !can_walled_plunder && !can_walled_restart && !can_milestone {
                        return Err(format!(
                            "asedio {city}: escena guardada todavía no simulada o en un umbral; espera={:?}; marcha={:?}; recuperación={:?}",self.prepare_walled_wait(city).err(),self.prepare_walled_march(city).err(),self.prepare_recovered_walled(city).err()
                        ));
                    }
                }
            }
        }
        Ok(())
    }
    fn prepare_recovered_walled(&self,city:usize)->Result<(Vec<u8>,Vec<u8>,Vec<u8>),String>{
        if self.host.siege_walled_live.get(city).and_then(Option::as_ref).is_some(){return Err("escena viva ya disponible".into())}
        let town=self.economy.world.towns.get(city).ok_or("ciudad ausente")?;
        let site=self.host.sites.get(city).ok_or("plano ausente")?;
        let level=site.wall_expansion_level(town);if level==0xff{return Err("ciudad sin muralla".into())}
        let mut record=self.host.founded_world.as_ref().ok_or("mundo ausente")?.sieges.get(city).ok_or("asedio ausente")?.clone();
        let recovered=p3_state::siege::recover_walled_headless(&mut record,town,site,level)?;Ok((record,recovered.image,recovered.tower_bytes))
    }
    fn prepare_walled_restart(&self,city:usize)->Result<(Vec<u8>,Vec<u8>),String>{
        let live=self.host.siege_walled_live.get(city).and_then(Option::as_ref).ok_or("escena viva ausente")?;let mut record=self.host.founded_world.as_ref().ok_or("mundo ausente")?.sieges.get(city).ok_or("asedio ausente")?.clone();let mut image=live.image.clone();
        let town=self.economy.world.towns.get(city).ok_or("ciudad ausente")?;let site=self.host.sites.get(city).ok_or("plano ausente")?;p3_state::siege::restart_walled_march(&mut record,&mut image,town,site,&live.tower_bytes)?;Ok((record,image))
    }
    fn prepare_walled_march(&self,city:usize)->Result<(Vec<u8>,Vec<u8>,Vec<u8>),String>{
        let mut record=self.host.founded_world.as_ref().ok_or("mundo ausente")?.sieges.get(city).ok_or("asedio ausente")?.clone();let town=self.economy.world.towns.get(city).ok_or("ciudad ausente")?;
        let(mut image,towers)=if let Some(live)=self.host.siege_walled_live.get(city).and_then(Option::as_ref){(live.image.clone(),live.tower_bytes.clone())}else{
            let site=self.host.sites.get(city).ok_or("plano ausente")?;let level=site.wall_expansion_level(town);if level==0xff{return Err("ciudad sin murallas".into())}
            let recovered=p3_state::siege::recover_walled_march(&mut record,town,site,level)?;(recovered.image,recovered.tower_bytes)
        };
        if p3_state::siege::march_walled_scene(&mut record,&mut image,town).is_err(){p3_state::siege::finish_walled_march(&mut record,&mut image,town)?}Ok((record,image,towers))
    }
    fn prepare_walled_wait(&self,city:usize)->Result<(Vec<u8>,Vec<u8>,p3_core::town::Town,Vec<u8>,bool),String>{
        let mut record=self.host.founded_world.as_ref().ok_or("mundo ausente")?.sieges.get(city).ok_or("asedio ausente")?.clone();
        let mut town=self.economy.world.towns.get(city).ok_or("ciudad ausente")?.clone();let site=self.host.sites.get(city).ok_or("plano ausente")?;
        let(mut image,towers)=if let Some(live)=self.host.siege_walled_live.get(city).and_then(Option::as_ref){(live.image.clone(),live.tower_bytes.clone())}else{
            let level=site.wall_expansion_level(&town);if level==0xff{return Err("ciudad sin murallas".into())}
            let recovered=p3_state::siege::recover_walled_wait(&mut record,&town,site,level,self.idle.clock.tick)?;let mut image=recovered.image;
            if let Some(flag)=self.host.siege_retreat_flags.get(city).copied().flatten(){image[0x79]=u8::from(flag)}(image,recovered.tower_bytes)
        };
        let removed=p3_state::siege::wait_defensive_victory(&mut record,&mut image,&mut town,site,&towers)?;Ok((record,image,town,towers,removed))
    }
    fn prepare_walled_assault(&self,city:usize,tick:u32)->Result<(Vec<Vec<u8>>,Vec<u8>,p3_core::town::Town,Vec<u8>,p3_core::scheduler::Scheduler,Vec<u8>),String>{
        let town=self.economy.world.towns.get(city).ok_or("ciudad ausente")?;let site=self.host.sites.get(city).ok_or("plano ausente")?;
        if site.wall_expansion_level(town)==0xff{return Err("ciudad sin muralla".into())}
        let (mut record,mut image,mut towers)=if let Some(live)=self.host.siege_walled_live.get(city).and_then(Option::as_ref){
            (self.host.founded_world.as_ref().ok_or("mundo ausente")?.sieges.get(city).ok_or("asedio ausente")?.clone(),live.image.clone(),live.tower_bytes.clone())
        }else{self.prepare_recovered_walled(city)?};
        let mut town=town.clone();let mut scheduler=self.host.council_scheduler.clone();let mut pool=self.host.message_pool.clone();
        let mut records=self.host.founded_world.as_ref().ok_or("mundo ausente")?.sieges.clone();records[city]=std::mem::take(&mut record);
        p3_state::siege::assault_walled_transaction(&mut records,city,&mut image,&mut town,site,&mut towers,&mut scheduler,&mut pool,self.idle.clock.calendar,tick)?;
        Ok((records,image,town,towers,scheduler,pool))
    }
    fn prepare_walled_milestone(&self, city: usize, tick: u32) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, p3_core::scheduler::Scheduler), String> {
        let town = self.economy.world.towns.get(city).ok_or("ciudad ausente")?;
        let site = self.host.sites.get(city).ok_or("plano ausente")?;
        if site.wall_expansion_level(town) == 0xff { return Err("ciudad sin muralla".into()); }
        let (mut record,mut image,towers)=if let Some(live)=self.host.siege_walled_live.get(city).and_then(Option::as_ref){
            (self.host.founded_world.as_ref().ok_or("mundo ausente")?.sieges.get(city).ok_or("asedio ausente")?.clone(),live.image.clone(),live.tower_bytes.clone())
        }else{self.prepare_recovered_walled(city)?};
        if image.len() != 0x650 || image[0x6f] == 0xff || image[0x6f] >= image[0x6e] { return Err("sin hito".into()); }
        let pos = 0x254 + usize::from(image[0x6f]) * 8;
        if pos + 2 > image.len() { return Err("objetivo truncado".into()); }
        let timer = u16::from_le_bytes([image[0x42], image[0x43]]);
        let milestone = u16::from_le_bytes([image[pos], image[pos + 1]]);
        if timer == 0 || timer - 1 != milestone { return Err("sin hito".into()); }
        let mut scheduler = self.host.council_scheduler.clone();
        p3_state::siege::schedule_walled_milestone(&mut record, &mut image, town, &mut scheduler, tick)?;
        Ok((record, image, towers, scheduler))
    }
    /// Comprueba la transición antes de avanzar el reloj; no modifica el mundo vivo.
    fn prepare_siege_plunder(&self, city: usize, tick: u32) -> Result<p3_state::siege::PlunderTransition, String> {
        let records = &self.host.founded_world.as_ref().ok_or("mundo de asedio ausente")?.sieges;
        let town = self.economy.world.towns.get(city).ok_or("ciudad de asedio ausente")?;
        let offices = self.current_office_pool()?;
        p3_state::siege::plunder_unwalled_scene(records, city, town, &offices.records, &self.rep_merchants,
            &self.host.message_pool, self.idle.clock.calendar, tick)
    }
    fn prepare_walled_plunder(&self,city:usize,tick:u32)->Result<(p3_state::siege::PlunderTransition,Vec<u8>,Vec<u8>),String>{
        let mut records=self.host.founded_world.as_ref().ok_or("mundo de asedio ausente")?.sieges.clone();let town=self.economy.world.towns.get(city).ok_or("ciudad ausente")?;
        let(image,towers)=if let Some(live)=self.host.siege_walled_live.get(city).and_then(Option::as_ref){(live.image.clone(),live.tower_bytes.clone())}else{
            let site=self.host.sites.get(city).ok_or("plano ausente")?;let level=site.wall_expansion_level(town);if level==0xff{return Err("ciudad sin murallas".into())}
            let record=records.get_mut(city).ok_or("asedio ausente")?;let recovered=p3_state::siege::recover_walled_march(record,town,site,level)?;(recovered.image,recovered.tower_bytes)
        };
        let offices=self.current_office_pool()?;let(out,image)=p3_state::siege::plunder_walled_scene(&records,city,&image,town,&offices.records,&self.rep_merchants,&self.host.message_pool,self.idle.clock.calendar,tick)?;
        if out.offices.len()!=self.persisted.len() || self.economy.offices.len()<out.offices.len() || self.economy.world.offices.len()<out.offices.len() || out.offices.iter().any(|raw|raw.len()!=0x44c){return Err("oficinas de asedio incompatibles".into())}Ok((out,image,towers))
    }
    fn apply_siege_plunder(&mut self,city:usize,next:p3_state::siege::PlunderTransition)->Result<(),String>{
        if next.offices.len()!=self.persisted.len() || self.economy.offices.len()<next.offices.len() || self.economy.world.offices.len()<next.offices.len() || next.offices.iter().any(|raw|raw.len()!=0x44c){return Err("oficinas de asedio incompatibles".into())}
        for (i,raw) in next.offices.iter().enumerate(){if self.economy.offices[i].owner==u16::MAX{continue}for k in 0..24{let at=4+4*k;self.persisted[i].stock.stock[k]=i32::from_le_bytes(raw[at..at+4].try_into().unwrap());}self.economy.offices[i]=self.persisted[i].stock.clone();self.economy.world.offices[i].weapons.copy_from_slice(&self.persisted[i].stock.stock[20..24]);}
        self.economy.world.towns[city]=next.town;self.host.message_pool=next.message_pool;self.host.founded_world.as_mut().unwrap().sieges=next.records;Ok(())
    }
    /// Ranura sin nada que simular en `siege_scene_step`: el registro existe, está bien formado, no guarda
    /// escena y tampoco hay imagen viva amurallada. En ese caso todas las preparaciones (`prepare_walled_*`,
    /// `prepare_recovered_walled`) fallan por «escena ausente»/«escena viva ausente» sin tocar nada y
    /// `cleanup_finished_scene` devuelve `Ok(false)`: no se escribe estado ni se consume el generador
    /// aleatorio, así que saltar la llamada da exactamente el mismo resultado. Cualquier otro caso
    /// (registro ausente o mal formado, ciudad ausente, imagen viva) devuelve `false` y sigue el camino completo.
    pub(crate) fn siege_scene_idle(&self, city: usize) -> bool {
        let Some(record) = self.host.founded_world.as_ref().and_then(|world| world.sieges.get(city)) else { return false };
        self.economy.world.towns.get(city).is_some()
            && self.host.siege_walled_live.get(city).and_then(Option::as_ref).is_none()
            && matches!(p3_save::world::siege_has_scene(record, city), Ok(false))
    }
    pub(crate) fn siege_scene_step(&mut self, city: usize) -> Result<(), String> {
        if self.host.siege_headless_march {
            if let Ok((record,image))=self.prepare_walled_restart(city){self.host.founded_world.as_mut().unwrap().sieges[city]=record;self.host.siege_walled_live[city].as_mut().unwrap().image=image;return Ok(())}
            if let Ok((next,image,towers))=self.prepare_walled_plunder(city,self.idle.clock.tick){
                self.apply_siege_plunder(city,next)?;if self.host.siege_walled_live.len()<=city{self.host.siege_walled_live.resize(city+1,None)}
                if let Some(live)=self.host.siege_walled_live[city].as_mut(){live.image=image}else{self.host.siege_walled_live[city]=Some(crate::LiveWalledSiege{image,tower_bytes:towers,list_bytes:Vec::new()})}return Ok(())
            }
            if let Ok((record,image,towers))=self.prepare_walled_march(city){
                self.host.founded_world.as_mut().unwrap().sieges[city]=record;if self.host.siege_walled_live.len()<=city{self.host.siege_walled_live.resize(city+1,None)}
                if let Some(live)=self.host.siege_walled_live[city].as_mut(){live.image=image}else{self.host.siege_walled_live[city]=Some(crate::LiveWalledSiege{image,tower_bytes:towers,list_bytes:Vec::new()})}return Ok(())
            }
            if let Ok((record,image,town,towers,removed))=self.prepare_walled_wait(city){
                self.host.founded_world.as_mut().unwrap().sieges[city]=record;self.economy.world.towns[city]=town;
                if self.host.siege_walled_live.len()<=city{self.host.siege_walled_live.resize(city+1,None)}
                if removed{self.host.siege_walled_live[city]=None;if let Some(flag)=self.host.siege_retreat_flags.get_mut(city){*flag=None}}
                else if let Some(live)=self.host.siege_walled_live[city].as_mut(){live.image=image}else{self.host.siege_walled_live[city]=Some(crate::LiveWalledSiege{image,tower_bytes:towers,list_bytes:Vec::new()})}return Ok(())
            }
            if let Ok((record, image, towers, scheduler)) = self.prepare_walled_milestone(city, self.idle.clock.tick) {
                self.host.founded_world.as_mut().unwrap().sieges[city] = record;
                if self.host.siege_walled_live.len()<=city{self.host.siege_walled_live.resize(city+1,None)}
                if let Some(live)=self.host.siege_walled_live[city].as_mut(){live.image=image}else{self.host.siege_walled_live[city]=Some(crate::LiveWalledSiege{image,tower_bytes:towers,list_bytes:Vec::new()})}
                self.host.council_scheduler = scheduler;
                return Ok(());
            }
            if let Ok((records,image,town,towers,scheduler,pool))=self.prepare_walled_assault(city,self.idle.clock.tick){
                self.host.founded_world.as_mut().unwrap().sieges=records;self.economy.world.towns[city]=town;self.host.council_scheduler=scheduler;self.host.message_pool=pool;
                if let Some(flag)=self.host.siege_retreat_flags.get_mut(city){*flag=Some(image[0x79]!=0)}
                if self.host.siege_walled_live.len()<=city{self.host.siege_walled_live.resize(city+1,None)}
                if let Some(live)=self.host.siege_walled_live[city].as_mut(){live.image=image;live.tower_bytes=towers}else{self.host.siege_walled_live[city]=Some(crate::LiveWalledSiege{image,tower_bytes:towers,list_bytes:Vec::new()})}
                return Ok(())
            }
            if let Ok((mut record,mut image,towers))=self.prepare_recovered_walled(city){
                let town=self.economy.world.towns.get_mut(city).ok_or("ciudad ausente")?;
                if p3_state::siege::approach_walled_scene(&mut record,&mut image,town).is_err(){
                    let site=self.host.sites.get(city).ok_or("plano ausente")?;
                    p3_state::siege::assault_walled(&mut record,&mut image,town,site,&towers)?;
                }
                if let Some(flag)=self.host.siege_retreat_flags.get_mut(city){*flag=Some(image[0x79]!=0)}
                self.host.founded_world.as_mut().unwrap().sieges[city]=record;
                if self.host.siege_walled_live.len()<=city{self.host.siege_walled_live.resize(city+1,None)}
                self.host.siege_walled_live[city]=Some(crate::LiveWalledSiege{image,tower_bytes:towers,list_bytes:Vec::new()});
                return Ok(())
            }
        }
        let retreat_flag = self.host.siege_retreat_flags.get(city).copied().flatten();
        let record = self.host.founded_world.as_mut().ok_or("mundo de asedio ausente")?.sieges.get_mut(city).ok_or("registro de asedio ausente")?;
        let town = self.economy.world.towns.get_mut(city).ok_or("ciudad de asedio ausente")?;
        let error = match p3_state::siege::cleanup_finished_scene(record, town) {
            Ok(removed) => { if removed { if let Some(flag) = self.host.siege_retreat_flags.get_mut(city) { *flag = None; } } return Ok(()); }
            Err(error) => error,
        };
        if !self.host.siege_headless_march { return Err(error); }
        let site = self.host.sites.get(city).ok_or("plano de asedio ausente")?;
        if site.wall_expansion_level(town) != 0xff {
            let live = self.host.siege_walled_live.get_mut(city).and_then(Option::as_mut)
                .ok_or("imagen viva del asedio amurallado ausente")?;
            if p3_state::siege::approach_walled_scene(record, &mut live.image, town).is_ok(){return Ok(())}
            p3_state::siege::assault_walled(record,&mut live.image,town,site,&live.tower_bytes)?;
            if let Some(flag)=self.host.siege_retreat_flags.get_mut(city){*flag=Some(live.image[0x79]!=0)}
            return Ok(())
        }
        if p3_state::siege::march_unwalled_scene(record, town).is_ok() { return Ok(()); }
        if let Some(flag) = retreat_flag {
            if p3_state::siege::retreat_unwalled_scene(record, town, flag).is_ok() {
                if p3_save::world::siege_scene(record, city).map_err(|e| format!("asedio: {e:?}"))?.is_none() {
                    self.host.siege_retreat_flags[city] = None;
                }
                return Ok(());
            }
        }
        let next = self.prepare_siege_plunder(city, self.idle.clock.tick)?;
        self.apply_siege_plunder(city,next)
    }
    pub(crate) fn siege_step(&mut self, city: usize, network_client: bool) -> Result<(), String> {
        if self.economy.world.world_8a != 0 {
            return Ok(());
        }
        let record = self
            .host
            .founded_world
            .as_ref()
            .ok_or("mundo de asedio ausente")?
            .sieges
            .get(city)
            .ok_or("registro de asedio ausente")?;
        let regions = self
            .host
            .settlement_locations
            .as_ref()
            .ok_or("regiones de asedio ausentes")?
            .iter()
            .map(|v| v.kind)
            .collect::<Vec<_>>();
        let pool = self.current_office_pool()?;
        if self.economy.world.merchants.len() != self.rep_merchants.len()
            || self.economy.merchants.len() != self.rep_merchants.len()
        {
            return Err("vistas de comerciantes incompatibles".into());
        }
        let mut merchants = self.rep_merchants.clone();
        for (i, m) in merchants.iter_mut().enumerate() {
            m.set_i32(0, self.economy.world.merchants[i].gold);
            m.set_i32(0x4c8, self.economy.world.merchants[i].income);
        }
        let enabled = merchants
            .iter()
            .map(|m| match m.bytes[0x38] {
                0..=4 => Ok(true),
                5 => Ok(false),
                _ => Err("preferencia de carta oficial fuera de tabla"),
            })
            .collect::<Result<Vec<_>, _>>()?;
        // El original llama a 0x635220 en el tick de cada ciudad; se toman los bytes anotados en ese tick.
        let tick = self.idle.clock.tick;
        let stack_tail = match self.host.siege_stack_tails.iter().position(|(t, _)| *t == tick) {
            Some(i) if self.idle.clock.calendar.day == 1 => self.host.siege_stack_tails.remove(i).map(|(_, b)| b),
            _ => self.host.siege_stack_tail,
        };
        let env = DailyInput {
            tick: self.idle.clock.tick,
            calendar: self.idle.clock.calendar,
            difficulty: self.host.siege_difficulty,
            disabled: false,
            network_client: u32::from(network_client),
            regions: &regions,
            stack_tail,
            message_enabled: &enabled,
        };
        let flag=self.host.siege_walled_live.get(city).and_then(Option::as_ref).and_then(|v|v.image.get(0x78)).copied();
        let next = p3_state::siege::daily_with_live_scene(
            record,
            &self.economy.world.towns,
            &pool.records,
            &merchants,
            &self.host.message_pool,
            self.host.investigation_rng,
            &env,
            flag,
        )?;
        if next.offices.len() != self.persisted.len() {
            return Err("oficinas de asedio incompatibles".into());
        }
        for (i, raw) in next.offices.iter().enumerate() {
            if self.economy.offices[i].owner == u16::MAX { continue; }
            if raw.len() != 0x44c {
                return Err("oficina de asedio truncada".into());
            }
            for k in 0..24 {
                let at = 4 + 4 * k;
                self.persisted[i].stock.stock[k] = i32::from_le_bytes(raw[at..at + 4].try_into().unwrap());
            }
            self.economy.offices[i] = self.persisted[i].stock.clone();
            self.economy.world.offices[i]
                .weapons
                .copy_from_slice(&self.persisted[i].stock.stock[20..24]);
        }
        self.economy.world.towns = next.towns;
        for (i, m) in next.merchants.into_iter().enumerate() {
            self.economy.world.merchants[i].gold = m.i32(0);
            self.economy.world.merchants[i].income = m.i32(0x4c8);
            self.economy.merchants[i] = m.clone();
            self.rep_merchants[i] = m;
        }
        self.host.message_pool = next.message_pool;
        self.host.investigation_rng = next.rng;
        if let Some(live) = self.host.siege_walled_live.get_mut(city).and_then(Option::as_mut) {
            let scene = p3_save::world::siege_scene(&next.record, city)
                .map_err(|e| format!("escena: {e:?}"))?.ok_or("escena viva ausente")?;
            p3_save::siege_scene::decode_fields(scene, &mut live.image).map_err(str::to_string)?;
        }
        self.host.founded_world.as_mut().unwrap().sieges[city] = next.record;
        if let Some(op) = next.operation {
            self.host.council_operations.push(p3_core::pump::Operation(op));
        }
        Ok(())
    }
}

/// Prepara el derrumbe de una defensa urbana sin vista.
pub(crate) fn prepare_ruin(task:p3_core::scheduler::TaskRecord,towns:&[p3_core::town::Town],sites:&[p3_core::buildings::Site])->Result<(usize,p3_core::scheduler::TaskRecord,p3_core::town::Town,p3_core::buildings::Site,bool),String>{
    let city=usize::from(task.u8_at(0x10));let mut town=towns.get(city).ok_or("ciudad o guarida no admitida")?.clone();let mut site=sites.get(city).ok_or("plano ausente")?.clone();let mut task=task;
    let keep=p3_core::buildings::burn::tick_ruin(&mut task,&mut town,&mut site,0xe0).map_err(str::to_string)?;Ok((city,task,town,site,keep))
}

/// Prepara en copias el incendio, la subasta y las listas privadas, sin vista.
pub(crate) fn prepare_burn(
    scheduler: &p3_core::scheduler::Scheduler, task_index: u16,
    towns: &[p3_core::town::Town], sites: &[p3_core::buildings::Site], merchants: u16,
    stocks: &[p3_core::economy::trade::OfficeStock], lists: &[p3_core::buildings::counts::OfficeLists], housing: &[p3_core::buildings::housing::OfficeHousing],
) -> Result<(p3_core::scheduler::Scheduler, usize, p3_core::scheduler::TaskRecord, p3_core::town::Town, p3_core::buildings::Site, Option<(usize,u16,u16)>, bool), String> {
    let task = *scheduler.records.get(usize::from(task_index)).ok_or("tarea ausente")?;
    let city = usize::from(task.u8_at(0x10));
    let mut town = towns.get(city).ok_or("ciudad de incendio ausente")?.clone();
    let mut site = sites.get(city).ok_or("plano de incendio ausente")?.clone();
    let target = u16::from_le_bytes([task.u8_at(8), task.u8_at(9)]);
    let mut next_scheduler=scheduler.clone();
    let mut office = None;
    if task.u8_at(0x11)!=0 {
        let base=usize::from(target).checked_mul(8).ok_or("edificio inválido")?;
        let row=site.records.get(base..base+8).ok_or("edificio ausente")?;
        if !matches!(row[3],0x28|0x32|0x33) {
            p3_core::buildings::burn::remove_from_auction(&mut next_scheduler,towns,city as u8,target).map_err(str::to_string)?;
        }
        if u16::from(row[2])<merchants && !matches!(row[3],0x28|0x29|0x32|0x33) {
            // Las casillas libres de la tabla (dueño y ciudad 0, todo a cero) coinciden con el
            // comerciante 0 en la ciudad 0: no son oficinas vivas.
            let pristine=|s:&p3_core::economy::trade::OfficeStock|s.stock==[0;24]&&s.average_price==[0.0;24]&&s.business_consumption==[0;24]&&s.ai_target==[0;24]&&s.administrator_limit==[0;24]&&s.administrator_experience.is_none()&&s.automation_flags==0;
            let mut matches=stocks.iter().enumerate().filter(|(_,s)|s.owner==u16::from(row[2]) && usize::from(s.town)==city);
            let (index,_)=matches.next().ok_or("oficina privada ausente")?;
            if matches.any(|(_,s)|!(pristine(s)&&s.owner==0&&s.town==0)){return Err("oficina privada ambigua".into())}
            let list=lists.get(index).ok_or("lista de oficina ausente")?;let house=housing.get(index).ok_or("viviendas de oficina ausentes")?;
            let mut image=vec![0u8;0x44c];image[0x2ce..0x2d0].copy_from_slice(&list.building_head.to_le_bytes());image[0x2d0..0x2d2].copy_from_slice(&house.houses_head.to_le_bytes());office=Some((index,image));
        }
    }
    let mut task = task;
    let keep = p3_core::buildings::burn::tick(&mut task, &mut town, &mut site, merchants, 0xe0, office.as_mut().map(|(_,v)|v.as_mut_slice())).map_err(str::to_string)?;
    task.next=next_scheduler.records[usize::from(task_index)].next;
    let update=office.map(|(i,v)|(i,u16::from_le_bytes([v[0x2ce],v[0x2cf]]),u16::from_le_bytes([v[0x2d0],v[0x2d1]])));
    Ok((next_scheduler, city, task, town, site, update, keep))
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_state::siege::daily;
    use p3_core::{ai::settlement::SettlementLocation, calendar::Calendar, rng::CrtRand};
    use std::{
        io::Write,
        process::{Command, Stdio},
    };

    // Las escenas sintéticas sustituyen el almacén de la ciudad del guardado.
    // Si su antigua cabeza libre ya no cabe, todos los registros del escenario
    // se consideran ocupados; el siguiente índice libre es el tamaño actual.
    fn apply_fixture_town(model: &mut p3_save::model::TownModel, old: &p3_core::town::Town, town: &p3_core::town::Town, site: &p3_core::buildings::Site) {
        let mut town = town.clone();
        let count = town.u16(0x776);
        assert_eq!(usize::from(count) * 8, site.records.len());
        if town.u16(0x760) > count { town.set_u16(0x760, count); }
        // El escenario no incluye el edificio público 0x2d del guardado base;
        // no debe heredar la bandera que obliga al cargador a recrearlo.
        if !site.records.chunks_exact(8).any(|record| record[3] == 0x2d) {
            town.set_u32(0x76c, town.u32(0x76c) & !1);
        }
        model.apply_town(old, &town, site).unwrap();
    }

    fn hex(v: &[u8]) -> String {
        v.iter().map(|b| format!("{b:02x}")).collect()
    }
    fn slots(pool: &[u8]) -> Vec<u8> {
        let parsed = p3_save::tail::MessagePool::parse(pool).unwrap();
        parsed
            .letters
            .iter()
            .flat_map(|letter| pool[letter.range.start..letter.range.start + 16].iter().copied())
            .collect()
    }

    #[test]
    fn live_daily_transaction_persists_and_preserves_conflicts() {
        let Ok(root) = std::env::var("P3_SAVES") else { return };
        let raw = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let mut sim = Sim::from_save_bytes(&raw).unwrap();
        let city = sim
            .economy
            .world
            .towns
            .iter()
            .position(|t| usize::from(t.u16(0x784)) < sim.persisted.len())
            .unwrap();
        let office = usize::from(sim.economy.world.towns[city].u16(0x784));
        let owner = usize::from(sim.persisted[office].stock.owner);
        assert!(owner < sim.rep_merchants.len());
        sim.economy.world.world_8a = 0;
        sim.host.siege_difficulty = 1;
        sim.host.investigation_rng = Some(CrtRand { state: 3 });
        sim.host.settlement_locations = Some([SettlementLocation { x: 0, y: 0, kind: 0 }; 40]);
        sim.idle.clock.tick = 0x9000;
        sim.idle.clock.calendar = Calendar {
            day: 8,
            month: 1,
            year: 1,
            day_of_year: 38,
        };
        sim.persisted[office].stock.stock.fill(100_000);
        sim.economy.offices[office] = sim.persisted[office].stock.clone();
        sim.rep_merchants[owner].bytes[0x38] = 0;
        sim.economy.merchants[owner].bytes[0x38] = 0;
        let mut record = sim.host.founded_world.as_ref().unwrap().sieges[city][..17].to_vec();
        record[4] = city as u8;
        record[5..7].copy_from_slice(&0x4000u16.to_le_bytes());
        for i in 0..4 {
            record[9 + i * 2] = i as u8;
        }
        let agreement = [0, 1, 0, owner as u8, 1, 0, 1, 0];
        let mut request = [1, 2, 3, 4, 5, owner as u8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];
        request[12..16].copy_from_slice(&(sim.idle.clock.tick - 1).to_le_bytes());
        record.push(1);
        record.extend(agreement);
        record.push(1);
        record.extend(request);
        record.push(0);
        sim.host.siege_initial[city] = record.clone();
        sim.host.founded_world.as_mut().unwrap().sieges[city] = record.clone();
        let offices = sim.current_office_pool().unwrap().records;
        let mut merchants = sim.rep_merchants.clone();
        for (i, m) in merchants.iter_mut().enumerate() {
            m.set_i32(0, sim.economy.world.merchants[i].gold);
            m.set_i32(0x4c8, sim.economy.world.merchants[i].income);
        }
        let regions = vec![0; 40];
        let enabled = merchants.iter().map(|m| m.bytes[0x38] != 5).collect::<Vec<_>>();
        let env = DailyInput {
            tick: sim.idle.clock.tick,
            calendar: sim.idle.clock.calendar,
            difficulty: 1,
            disabled: false,
            network_client: 0,
            regions: &regions,
            stack_tail: None,
            message_enabled: &enabled,
        };
        let expected = daily(
            &record,
            &sim.economy.world.towns,
            &offices,
            &merchants,
            &sim.host.message_pool,
            sim.host.investigation_rng,
            &env,
        )
        .unwrap();
        if let (Ok(binary), Ok(python)) = (std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) {
            let c = sim.idle.clock.calendar;
            let request = serde_json::json!({"binary":binary,"record":hex(&record),"towns":hex(&sim.economy.world.towns.iter().flat_map(|t|t.0.iter().copied()).collect::<Vec<_>>()),"offices":hex(&offices.concat()),"merchants":hex(&merchants.iter().flat_map(|m|m.bytes.iter().copied()).collect::<Vec<_>>()),"pool":hex(&sim.host.message_pool),"slots":hex(&slots(&sim.host.message_pool)),"day":c.day,"month":c.month,"year":c.year,"doy":c.day_of_year,"tick":sim.idle.clock.tick,"seed":3,"difficulty":1,"regions":regions});
            let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_siege_daily_live.py");
            let mut child = Command::new(python)
                .arg(script)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap();
            child
                .stdin
                .take()
                .unwrap()
                .write_all(request.to_string().as_bytes())
                .unwrap();
            let output = child.wait_with_output().unwrap();
            assert!(
                output.status.success(),
                "oráculo: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            let original: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(
                hex(&expected.record),
                original["record"].as_str().unwrap(),
                "registro x86"
            );
            assert_eq!(
                hex(&expected
                    .towns
                    .iter()
                    .flat_map(|t| t.0.iter().copied())
                    .collect::<Vec<_>>()),
                original["towns"].as_str().unwrap(),
                "ciudades x86"
            );
            assert_eq!(
                hex(&expected.offices.concat()),
                original["offices"].as_str().unwrap(),
                "oficinas x86"
            );
            assert_eq!(
                hex(&expected
                    .merchants
                    .iter()
                    .flat_map(|m| m.bytes.iter().copied())
                    .collect::<Vec<_>>()),
                original["merchants"].as_str().unwrap(),
                "comerciantes x86"
            );
            assert_eq!(
                hex(&slots(&expected.message_pool)),
                original["slots"].as_str().unwrap(),
                "ranuras x86"
            );
            assert_eq!(
                u16::from_le_bytes(expected.message_pool[6..8].try_into().unwrap()) as u64,
                original["free"].as_u64().unwrap()
            );
            assert_eq!(
                u16::from_le_bytes(expected.message_pool[8..10].try_into().unwrap()) as u64,
                original["head"].as_u64().unwrap()
            );
            assert_eq!(expected.rng.unwrap().state as u64, original["seed"].as_u64().unwrap());
            assert!(original["operation"].is_null());
        }
        sim.siege_step(city, false).unwrap();
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[city], expected.record);
        assert_eq!(sim.host.message_pool, expected.message_pool);
        assert_eq!(
            sim.economy.world.towns.iter().map(|t| &t.0[..]).collect::<Vec<_>>(),
            expected.towns.iter().map(|t| &t.0[..]).collect::<Vec<_>>()
        );
        assert_eq!(
            sim.rep_merchants.iter().map(|m| &m.bytes).collect::<Vec<_>>(),
            expected.merchants.iter().map(|m| &m.bytes).collect::<Vec<_>>()
        );
        let actual = sim.current_office_pool().unwrap();
        for (i, office) in actual.records.iter().enumerate() {
            assert_eq!(&office[4..100], &expected.offices[i][4..100]);
        }
        let mut model = p3_save::model::SaveModel::parse(&raw).unwrap();
        model.world.sieges[city] = record;
        let mut conflict = model.clone();
        conflict.world.sieges[city][0] ^= 1;
        let before = conflict.clone();
        assert!(sim.persist_sieges(&mut conflict).is_err());
        assert_eq!(conflict, before);
        sim.persist_all(&mut model).unwrap();
        let reloaded = Sim::from_save_bytes(&model.write()).unwrap();
        assert_eq!(reloaded.host.founded_world.unwrap().sieges[city], expected.record);
        assert_eq!(reloaded.host.message_pool, expected.message_pool);
        assert_eq!(
            reloaded.persisted[office].stock.stock,
            sim.persisted[office].stock.stock
        );
        assert_eq!(reloaded.rep_merchants[owner].i32(0), sim.rep_merchants[owner].i32(0));
    }

    #[test]
    fn live_breach_survives_daily_siege_tick(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();let mut sim=Sim::from_save_bytes(&raw).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_wall_breached.json")).unwrap();let raw=fixture["cases"][0]["image"].as_str().unwrap();let image=raw.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
        let mut record=vec![0;20];record[19]=1;record.extend(p3_save::siege_scene::encode(&image).unwrap());sim.host.founded_world.as_mut().unwrap().sieges[0]=record;
        sim.host.siege_walled_live.resize(1,None);sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image,tower_bytes:vec![],list_bytes:vec![]});
        sim.economy.world.world_8a=0;sim.host.investigation_rng=Some(CrtRand{state:3});sim.host.settlement_locations=Some([SettlementLocation{x:0,y:0,kind:0};40]);sim.idle.clock.tick=0x9000;sim.idle.clock.calendar=Calendar{day:8,month:1,year:1,day_of_year:38};
        sim.siege_step(0,false).unwrap();let scene=p3_save::world::siege_scene(&sim.host.founded_world.as_ref().unwrap().sieges[0],0).unwrap().unwrap();assert_eq!(scene[0x22],1);assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image[0x78],1);
    }

    #[test]
    fn long_defensive_wait_survives_daily_siege_tick(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_wall_breached.json")).unwrap();let source=fixture["cases"][0]["image"].as_str().unwrap();
        for wait in [1000u16,500]{let mut sim=Sim::from_save_bytes(&raw).unwrap();let mut image=source.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();image[0x3a..0x3c].copy_from_slice(&wait.to_le_bytes());image[0x6a..0x6c].copy_from_slice(&500u16.to_le_bytes());
            let mut record=vec![0;20];record[19]=1;record.extend(p3_save::siege_scene::encode(&image).unwrap());sim.host.founded_world.as_mut().unwrap().sieges[0]=record;sim.host.siege_walled_live.resize(1,None);sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image,tower_bytes:vec![],list_bytes:vec![]});
            sim.economy.world.world_8a=0;sim.host.investigation_rng=Some(CrtRand{state:3});sim.host.settlement_locations=Some([SettlementLocation{x:0,y:0,kind:0};40]);sim.idle.clock.tick=0x9000;sim.idle.clock.calendar=Calendar{day:8,month:1,year:1,day_of_year:38};
            sim.siege_step(0,false).unwrap();let scene=p3_save::world::siege_scene(&sim.host.founded_world.as_ref().unwrap().sieges[0],0).unwrap().unwrap();assert_eq!(u16::from_le_bytes([scene[0x1c],scene[0x1d]]),wait);assert_eq!(scene[0x22],1);
        }
    }

    #[test]
    fn siege_start_stays_queued_until_its_handler_exists() {
        let Ok(root) = std::env::var("P3_SAVES") else { return };
        let raw = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let mut sim = Sim::from_save_bytes(&raw).unwrap();
        sim.economy.world.world_8a = 0;
        sim.host.siege_difficulty = 1;
        sim.host.investigation_rng = Some(CrtRand { state: 3 });
        sim.host.settlement_locations = Some([SettlementLocation { x: 0, y: 0, kind: 0 }; 40]);
        sim.idle.clock.tick = 0x9000;
        sim.idle.clock.calendar = Calendar {
            day: 3,
            month: 0,
            year: 1,
            day_of_year: 3,
        };
        let mut record = vec![0; 20];
        record[4] = 0;
        record[5] = 1;
        record[8] = 0xff;
        for i in 0..4 {
            record[9 + 2 * i] = 0xff;
        }
        sim.host.founded_world.as_mut().unwrap().sieges[0] = record;
        sim.siege_step(0, false).unwrap();
        assert_eq!(sim.host.council_operations.len(), 1);
        assert_eq!(sim.host.council_operations[0].0[0], 0x8e);
        let town = &mut sim.economy.world.towns[0];
        town.set_u32(0x76c, 0x100);
        town.set_u16(0x776, 3);
        town.set_u16(0x768, 0);
        town.set_u16(0x762, u16::MAX);
        let mut rows = vec![0; 24];
        for i in 0..3 {
            rows[8 * i + 3] = 0x34;
            rows[8 * i + 4..8 * i + 6].copy_from_slice(&(if i == 2 { u16::MAX } else { (i + 1) as u16 }).to_le_bytes());
        }
        sim.host.sites[0].records = rows;
        let op = sim.host.council_operations[0].0;
        let errors = sim.dispatch_pending_council_operations();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("0x8E"));
        assert_eq!(sim.host.council_operations[0].0, op);
        let mut model = p3_save::model::SaveModel::parse(&raw).unwrap();
        let before = model.clone();
        assert!(sim.persist_all(&mut model).is_err());
        assert_eq!(model, before);
    }

    #[test]
    fn siege_start_is_discarded_when_gate_rejects_it() {
        let Ok(root) = std::env::var("P3_SAVES") else { return };
        let raw = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let mut sim = Sim::from_save_bytes(&raw).unwrap();
        sim.economy.world.towns[0].set_u32(0x76c, 0);
        let before = sim.host.founded_world.as_ref().unwrap().sieges[0].clone();
        sim.host
            .council_operations
            .push(p3_core::pump::Operation([0x8e, 0, 0, 0, 1]));
        assert!(sim.dispatch_pending_council_operations().is_empty());
        assert!(sim.host.council_operations.is_empty());
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0], before);
    }

    #[test]
    fn saved_scene_is_preserved_but_cannot_advance() {
        let Ok(root) = std::env::var("P3_SAVES") else { return };
        let raw = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_scene_save.json")).unwrap();
        let scene = data["cases"][0]["record"]
            .as_str()
            .unwrap()
            .as_bytes()
            .chunks_exact(2)
            .map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap())
            .collect::<Vec<_>>();
        let mut model = p3_save::model::SaveModel::parse(&raw).unwrap();
        model.world.sieges[0] = scene.clone();
        let raw = model.write();
        let mut sim = Sim::from_save_bytes(&raw).unwrap();
        let before = sim.idle.clock.tick;
        let error = sim.advance_pumped_until(before + 1, 1, false).unwrap_err();
        assert!(error.contains("escena guardada"));
        assert_eq!(sim.idle.clock.tick, before);
        sim.persist_all(&mut model).unwrap();
        // La carga original (0x627AD0) recalcula `+0x78` (byte 0x22 de la escena) como `+0x3C != 0 || +0x3A != 0`.
        let mut expected = scene.clone();
        let at = expected.len() - p3_save::world::siege_scene(&scene, 0).unwrap().unwrap().len();
        expected[at + 0x22] = u8::from(expected[at + 0x18..at + 0x1a] != [0, 0] || expected[at + 0x1c..at + 0x1e] != [0, 0]);
        assert_eq!(model.world.sieges[0], expected);
    }

    #[test]
    fn finished_saved_scene_is_cleaned_by_clock_and_persisted() {
        let Ok(root) = std::env::var("P3_SAVES") else { return };
        let raw = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let mut model = p3_save::model::SaveModel::parse(&raw).unwrap();
        let record = &mut model.world.sieges[0];
        let end = record.len() - 1;
        assert_eq!(record[end], 0);
        record[end] = 1;
        let mut scene = vec![0; 0x51];
        scene[0x50] = 0xff;
        record.extend(scene);
        let mut sim = Sim::from_save_bytes(&model.write()).unwrap();
        let old = sim.economy.world.towns[0].u32(0x2c8);
        sim.economy.world.towns[0].set_u32(0x2c8, old | 0x10);
        let tick = sim.idle.clock.tick;
        sim.advance_with_merchant_daily(tick + 1, false).unwrap();
        assert_eq!(sim.idle.clock.tick, tick + 1);
        assert_eq!(sim.economy.world.towns[0].u32(0x2c8) & 0x10, 0);
        assert!(!p3_save::world::siege_has_scene(&sim.host.founded_world.as_ref().unwrap().sieges[0], 0).unwrap());
        sim.persist_all(&mut model).unwrap();
        let reloaded = Sim::from_save_bytes(&model.write()).unwrap();
        assert!(!p3_save::world::siege_has_scene(&reloaded.host.founded_world.unwrap().sieges[0], 0).unwrap());
    }

    #[test]
    fn unwalled_start_is_dispatched_and_persisted() {
        let Ok(root) = std::env::var("P3_SAVES") else { return };
        let raw = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let mut model = p3_save::model::SaveModel::parse(&raw).unwrap();
        let mut sim = Sim::from_save_bytes(&raw).unwrap();
        sim.host.expedition_resources = Some(crate::HanseFleetResources {
            player: 0,
            ..Default::default()
        });
        sim.rep_merchants[0].bytes[0x19] = 1;
        sim.host.siege_difficulty = 1;
        let town = &mut sim.economy.world.towns[0];
        town.set_u8(0x6f1, 0xff);
        town.set_u32(0x76c, 0x100);
        town.set_u16(0x768, 0);
        town.set_u16(0x762, u16::MAX);
        town.set_u16(0x766, u16::MAX);
        for i in 0..6 {
            town.set_u8(0x7b0 + i, 0)
        }
        let site = &mut sim.host.sites[0];
        assert!(site.records.len() >= 24);
        for i in 0..3 {
            site.records[8 * i + 3] = 0x34;
            site.records[8 * i + 4..8 * i + 6]
                .copy_from_slice(&(if i == 2 { u16::MAX } else { (i + 1) as u16 }).to_le_bytes())
        }
        assert_eq!(site.wall_expansion_level(town), 0xff);
        let expected = p3_state::siege::start_unwalled(
            &sim.host.founded_world.as_ref().unwrap().sieges[0],
            &sim.economy.world.towns[0],
            &sim.host.message_pool,
            sim.idle.clock.calendar,
            sim.idle.clock.tick,
            1,
            1,
            70,
            1,
        )
        .unwrap();
        sim.host.settlement_locations = Some([SettlementLocation { x: 0, y: 0, kind: 0 }; 40]);
        sim.rep_merchants[0].bytes[0x39d] = 5;
        let mut home = sim.host.founded_world.as_ref().unwrap().sieges[1][..17].to_vec();
        home.extend([0, 0, 0]);
        sim.host.siege_initial[1] = home.clone();
        sim.host.founded_world.as_mut().unwrap().sieges[1] = home.clone();
        model.world.sieges[1] = home;
        sim.host
            .council_operations
            .push(p3_core::pump::Operation([0x8e, 0, 1, 70, 1]));
        let old = sim.host.founded_world.as_ref().unwrap().sieges[0].clone();
        let errors = sim.dispatch_pending_council_operations();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("CRT"));
        assert_eq!(sim.host.council_operations.len(), 1);
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0], old);
        sim.rep_merchants[0].bytes[0x39d] = 0;
        assert!(sim.dispatch_pending_council_operations().is_empty());
        assert!(sim.host.council_operations.is_empty());
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0], expected.record);
        assert_eq!(sim.host.message_pool, expected.message_pool);
        sim.persist_all(&mut model).unwrap();
        let saved = p3_save::model::SaveModel::parse(&model.write()).unwrap();
        assert_eq!(saved.world.sieges[0], expected.record);
        assert_eq!(saved.message_pool, expected.message_pool);
        let loaded = Sim::from_save_bytes(&model.write()).unwrap();
        assert_eq!(loaded.economy.world.towns[0].u32(0x2c8), expected.town.u32(0x2c8));
        let tick = sim.idle.clock.tick;
        assert!(sim
            .advance_with_merchant_daily(tick + 1, false)
            .unwrap_err()
            .contains("escena guardada"));
        assert_eq!(sim.idle.clock.tick, tick);
        sim.host.founded_world.as_mut().unwrap().sieges[0] = old;
        sim.rep_merchants[0].bytes[0x39d] = 5;
        sim.host.investigation_rng = Some(CrtRand { state: 123 });
        let mut expected_rng = Some(CrtRand { state: 123 });
        let expected_offer =
            p3_core::economy::siege::supply_request_offer(1, 0, 0, 5, 1, true, &[], tick, &mut expected_rng)
                .unwrap()
                .unwrap();
        sim.host
            .council_operations
            .push(p3_core::pump::Operation([0x8e, 0, 1, 70, 1]));
        assert!(sim.dispatch_pending_council_operations().is_empty());
        assert_eq!(sim.host.council_operations.len(), 1);
        assert_eq!(sim.host.council_operations[0].0, expected_offer);
        assert_eq!(sim.host.investigation_rng, expected_rng);
        let errors = sim.dispatch_pending_council_operations();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("heap"));
        assert_eq!(sim.host.council_operations[0].0, expected_offer);
        let tail = [0x31, 0x42];
        let enabled = sim.rep_merchants[0].bytes[0x38] != 5;
        let expected_request = p3_state::siege::create_request(
            &sim.host.founded_world.as_ref().unwrap().sieges[1],
            &sim.rep_merchants,
            &sim.host.message_pool,
            expected_offer,
            sim.idle.clock.calendar,
            enabled,
            tail,
        )
        .unwrap();
        sim.host.siege_request_heap_bytes.push_back(tail);
        assert!(sim.dispatch_pending_council_operations().is_empty());
        assert!(sim.host.council_operations.is_empty());
        assert_eq!(
            sim.host.founded_world.as_ref().unwrap().sieges[1],
            expected_request.record
        );
        assert_eq!(sim.host.message_pool, expected_request.message_pool);
        assert!(sim.host.siege_request_heap_bytes.is_empty());
        sim.persist_all(&mut model).unwrap();
        let loaded = Sim::from_save_bytes(&model.write()).unwrap();
        assert_eq!(loaded.host.founded_world.unwrap().sieges[1], expected_request.record);
        assert_eq!(loaded.host.message_pool, expected_request.message_pool);
    }

    #[test]
    fn headless_plunder_runs_at_clock_and_persists_before_next_boundary() {
        let Ok(root) = std::env::var("P3_SAVES") else { return };
        let raw = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let mut model = p3_save::model::SaveModel::parse(&raw).unwrap();
        let data: serde_json::Value =
            serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_scene_march.json")).unwrap();
        let text = data["cases"][0]["before"].as_str().unwrap();
        let mut record = text
            .as_bytes()
            .chunks_exact(2)
            .map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap())
            .collect::<Vec<_>>();
        record[20 + 0x18..20 + 0x1a].copy_from_slice(&11u16.to_le_bytes());
        record[20 + 0x1a..20 + 0x1c].copy_from_slice(&20u16.to_le_bytes());
        model.world.sieges[0] = record.clone();
        let mut sim = Sim::from_save_bytes(&model.write()).unwrap();
        let town = &mut sim.economy.world.towns[0];
        town.set_u16(0x766, u16::MAX);
        for i in 0..6 {
            town.set_u8(0x7b0 + i, 0)
        }
        assert_eq!(sim.host.sites[0].wall_expansion_level(town), 0xff);
        sim.host.siege_headless_march = true;
        sim.idle.clock.tick = 100;
        let expected = sim.prepare_siege_plunder(0, 101).unwrap();
        sim.advance_with_merchant_daily(101, false).unwrap();
        assert_eq!(sim.idle.clock.tick, 101);
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges, expected.records);
        assert_eq!(sim.economy.world.towns[0], expected.town);
        for (i, (actual, want)) in sim.current_office_pool().unwrap().records.iter().zip(&expected.offices).enumerate() {
            if let Some((at, (a, b))) = actual.iter().zip(want).enumerate().find(|(_, (a, b))| a != b) { panic!("oficina {i} +{at:x}: {a:x} != {b:x}"); }
        }
        assert_eq!(sim.host.message_pool, expected.message_pool);
        sim.persist_all(&mut model).unwrap();
        let loaded = Sim::from_save_bytes(&model.write()).unwrap();
        assert_eq!(loaded.host.founded_world.as_ref().unwrap().sieges, expected.records);
        assert_eq!(loaded.host.message_pool, expected.message_pool);
        assert_eq!(loaded.economy.world.towns[0], expected.town);
        let mut sim = loaded;
        sim.host.siege_headless_march = true;
        sim.idle.clock.tick = 101;
        let record = &mut sim.host.founded_world.as_mut().unwrap().sieges[0];
        record[20 + 0x18..20 + 0x1a].copy_from_slice(&1u16.to_le_bytes());
        record[20 + 0x1e..20 + 0x20].copy_from_slice(&1u16.to_le_bytes());
        sim.advance_with_merchant_daily(102, false).unwrap();
        let record = &sim.host.founded_world.as_ref().unwrap().sieges[0];
        assert_eq!(&record[20 + 0x1c..20 + 0x1e], &2u16.to_le_bytes());
        sim.persist_all(&mut model).unwrap();
        let mut reloaded = Sim::from_save_bytes(&model.write()).unwrap();
        reloaded.host.siege_headless_march = true;
        reloaded.siege_scene_step(0).unwrap();
        let record = &reloaded.host.founded_world.as_ref().unwrap().sieges[0];
        assert_eq!(&record[20 + 0x1c..20 + 0x1e], &1u16.to_le_bytes());
        reloaded.siege_scene_step(0).unwrap();
        assert!(!p3_save::world::siege_has_scene(&reloaded.host.founded_world.as_ref().unwrap().sieges[0], 0).unwrap());
        let record = &mut sim.host.founded_world.as_mut().unwrap().sieges[0];
        record[..4].copy_from_slice(&0u32.to_le_bytes());
        record[20 + 0x18..20 + 0x1a].copy_from_slice(&0u16.to_le_bytes());
        record[20 + 0x1c..20 + 0x1e].copy_from_slice(&2u16.to_le_bytes());
        record[20 + 0x1e..20 + 0x20].copy_from_slice(&1u16.to_le_bytes());
        sim.idle.clock.tick = 0x2000;
        sim.host.siege_retreat_flags[0] = Some(false);
        let mut expected_record = sim.host.founded_world.as_ref().unwrap().sieges[0].clone();
        let mut expected_town = sim.economy.world.towns[0].clone();
        p3_state::siege::retreat_unwalled_scene(&mut expected_record, &mut expected_town, false).unwrap();
        sim.advance_with_merchant_daily(0x2001, false).unwrap();
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0], expected_record);
        assert_eq!(sim.economy.world.towns[0], expected_town);
    }
    #[test]
    fn walled_start_dispatches_with_explicit_heap_and_roundtrips() {
        let Ok(root) = std::env::var("P3_SAVES") else { return };
        let raw = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_wall_begin_chain.json")).unwrap();
        let case = &fixture["cases"][26];
        let n = |key: &str| case[key].as_u64().unwrap();
        let decode = |key: &str| case[key].as_str().unwrap().as_bytes().chunks_exact(2).map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap()).collect::<Vec<_>>();
        assert_eq!(n("level"), 0);
        let mut model = p3_save::model::SaveModel::parse(&raw).unwrap();
        model.world.sieges[0] = vec![0; 20];
        let data = model.world.write(); let world = p3_save::world::World::parse(&data).unwrap();
        let old = world.town(0); let mut town = old.clone();
        town.set_u32(0x2c8, n("flags") as u32); town.set_i32(0x2d4, n("population") as i32);
        town.set_i32(0x2d8, n("pop0") as i32); town.set_i32(0x2e0, n("pop2") as i32);
        town.set_u32(0x76c, 0x100); town.set_u16(0x768, n("defence_head") as u16);
        town.set_u16(0x762, n("construction_head") as u16); town.set_u16(0x766, u16::MAX); town.set_u16(0x784, u16::MAX);
        town.set_u32(0x7a8, 128); town.set_u32(0x7ac, 256); town.set_u8(0x7b0, 255); town.set_u8(0x7b2, 0); town.set_u8(0x7b4, 0);
        town.set_u8(0x6f1, 0xff); for (i, b) in decode("militia").into_iter().enumerate() { town.set_u8(0x998 + i, b); }
        let mut grid = vec![0u8; 128 * 256]; for pair in case["grid"].as_array().unwrap() { let at=pair[0].as_u64().unwrap() as usize; if at < grid.len() { grid[at]=pair[1].as_u64().unwrap() as u8; } }
        let site = p3_core::buildings::Site { records: decode("records"), cells: grid };
        town.set_u16(0x776, (site.records.len() / 8) as u16);
        assert_eq!(site.wall_expansion_level(&town), 0);
        assert!(p3_core::economy::siege::start_gate(&town, &site, false).unwrap());
        apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim = Sim::from_save_bytes(&model.write()).unwrap();
        sim.idle.clock.tick = n("tick") as u32; sim.idle.clock.calendar = p3_core::calendar::Calendar::for_tick(sim.idle.clock.tick);
        sim.host.siege_difficulty = n("difficulty") as u8;
        sim.host.expedition_resources = Some(crate::HanseFleetResources { player: 0, ..Default::default() });
        sim.rep_merchants[0].bytes[0x19] = 1; sim.rep_merchants[0].bytes[0x39d] = 0;
        let op = p3_core::pump::Operation([0x8e, 0, n("variant") as u32, n("strength") as u32, n("mode") as u32]);
        let before = sim.host.founded_world.as_ref().unwrap().sieges[0].clone();
        sim.host.council_operations.push(op);
        let errors = sim.dispatch_pending_council_operations();
        assert_eq!(errors.len(), 1); assert!(errors[0].contains("memoria"));
        assert_eq!(sim.host.council_operations, vec![op]); assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0], before);
        sim.host.siege_walled_heap.push_back(crate::WalledSiegeHeap { tower_bytes: decode("heap_towers"), list_bytes: decode("heap_list"), tower_address: 0x23300000, wall_clock: n("clock") as u32 });
        assert!(sim.dispatch_pending_council_operations().is_empty()); assert!(sim.host.siege_walled_heap.is_empty());
        let live = sim.host.siege_walled_live[0].as_ref().unwrap(); assert_eq!(live.image, decode("after_image"));
        assert_eq!(live.tower_bytes, decode("after_towers")); assert_eq!(live.list_bytes, decode("after_list"));
        let saved = sim.host.founded_world.as_ref().unwrap().sieges[0].clone(); assert_eq!(&saved[20..], decode("after_serial"));
        let tick = sim.idle.clock.tick; assert!(sim.advance_with_merchant_daily(tick + 1, false).is_err()); assert_eq!(sim.idle.clock.tick, tick);
        sim.host.siege_headless_march = true;
        let mut expected = saved; let mut image = sim.host.siege_walled_live[0].as_ref().unwrap().image.clone();
        p3_state::siege::approach_walled_scene(&mut expected, &mut image, &sim.economy.world.towns[0]).unwrap();
        sim.idle.clock.tick = 4;
        sim.idle.clock.calendar = p3_core::calendar::Calendar::for_tick(4);
        sim.advance_with_merchant_daily(5, false).unwrap();
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0], expected);
        assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image, image);
        let mut milestone_model = model.clone();
        sim.persist_all(&mut model).unwrap(); let mut reloaded = Sim::from_save_bytes(&model.write()).unwrap();
        assert_eq!(reloaded.host.founded_world.as_ref().unwrap().sieges[0], expected);
        reloaded.host.siege_headless_march = true;
        reloaded.reject_unsimulated_siege_scenes().unwrap();
        let mut recovered=expected.clone();let town=&reloaded.economy.world.towns[0];let site=&reloaded.host.sites[0];
        let mut minimal=p3_state::siege::recover_walled_headless(&mut recovered,town,site,site.wall_expansion_level(town)).unwrap().image;
        p3_state::siege::approach_walled_scene(&mut recovered,&mut minimal,town).unwrap();
        reloaded.idle.clock.tick=4;reloaded.idle.clock.calendar=Calendar::for_tick(4);reloaded.advance_with_merchant_daily(5,false).unwrap();
        assert_eq!(reloaded.host.founded_world.as_ref().unwrap().sieges[0],recovered);

        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_walled_milestone.json")).unwrap();
        let row = &fixture["cases"][0];
        let data = |key: &str| row[key].as_str().unwrap().as_bytes().chunks_exact(2).map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(), 16).unwrap()).collect::<Vec<_>>();
        for i in 0..4 { sim.economy.world.towns[0].set_u8(0x998 + i, row["militia"][i].as_u64().unwrap() as u8); }
        sim.host.founded_world.as_mut().unwrap().sieges[0] = data("before");
        sim.host.siege_walled_live[0].as_mut().unwrap().image = data("image");
        sim.idle.clock.tick = 4; sim.idle.clock.calendar = p3_core::calendar::Calendar::for_tick(4);
        sim.advance_with_merchant_daily(5, false).unwrap();
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0], data("after"));
        assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image, data("after_image"));
        let task = sim.host.council_scheduler.records.iter().find(|task| task.kind == 0x26 && task.due == 6).unwrap();
        assert_eq!(task.kind, 0x26); assert_eq!(task.due, 6);
        assert_eq!(&task.payload[..2], &data("task_after")[8..10]);
        assert!(sim.advance_with_merchant_daily(6, false).is_err()); assert_eq!(sim.idle.clock.tick, 5);
        sim.persist_all(&mut milestone_model).unwrap();
        let reloaded = Sim::from_save_bytes(&milestone_model.write()).unwrap();
        let task = reloaded.host.council_scheduler.records.iter().find(|task| task.kind == 0x26 && task.due == 6).unwrap();
        assert_eq!(task.kind, 0x26); assert_eq!(task.due, 6);
    }

    #[test]
    fn public_burn_runs_from_clock_and_persists() {
        let Ok(root) = std::env::var("P3_SAVES") else { return };
        let raw = std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_burn_public.json")).unwrap();
        let decode = |value: &str| value.as_bytes().chunks_exact(2).map(|b| u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
        let row = fixture["cases"].as_array().unwrap().iter().find(|row| {
            let t=decode(row["task"].as_str().unwrap());let r=decode(row["records"].as_str().unwrap());
            t[0x11]==1 && r[11]==0 && row["result"].as_u64().unwrap()==1
        }).unwrap();
        let source_town=decode(row["town"].as_str().unwrap());let source_task=decode(row["task"].as_str().unwrap());
        let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();
        for (city,record) in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();
        let old=world.town(0);let mut town=old.clone();
        for offset in [0x760,0x764,0x76a,0x776] {town.set_u16(offset,u16::from_le_bytes([source_town[offset],source_town[offset+1]]));}
        town.set_u32(0x2c8,u32::from_le_bytes(source_town[0x2c8..0x2cc].try_into().unwrap()));
        town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);
        let site=p3_core::buildings::Site{records:decode(row["records"].as_str().unwrap()),cells:vec![0;128*256]};
        apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;
        let id=sim.host.council_scheduler.add_task(5);let task=&mut sim.host.council_scheduler.records[usize::from(id)];
        task.kind=0x26;task.payload.copy_from_slice(&source_task[8..24]);
        sim.idle.clock.tick=4;sim.idle.clock.calendar=Calendar::for_tick(4);
        sim.advance_with_merchant_daily(5,false).unwrap();
        assert_eq!(sim.host.sites[0].records,decode(row["after_records"].as_str().unwrap()));
        let task=&sim.host.council_scheduler.records[usize::from(id)];assert_eq!(task.kind,0x26);assert_eq!(task.due,10);
        assert_eq!(&task.payload,&decode(row["after_task"].as_str().unwrap())[8..24]);
        sim.persist_all(&mut model).unwrap();let reloaded=Sim::from_save_bytes(&model.write()).unwrap();
        assert_eq!(reloaded.host.sites[0].records,sim.host.sites[0].records);
        assert_eq!(reloaded.host.council_scheduler.records[usize::from(id)].due,10);
    }

    #[test]
    fn private_burn_updates_office_and_survives_reload() {
        let Ok(root)=std::env::var("P3_SAVES") else{return};
        let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_burn_private.json")).unwrap();
        let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
        let row=fixture["cases"].as_array().unwrap().iter().find(|r|{let t=decode(r["task"].as_str().unwrap());let b=decode(r["records"].as_str().unwrap());let o=decode(r["office"].as_str().unwrap());t[0x11]==1&&b[11]==0&&o[0x2ce..0x2d0]==1u16.to_le_bytes()&&r["result"].as_u64().unwrap()==1}).unwrap();
        let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();
        for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();
        let office_index=(0..model.world.offices.len()).find(|&i|world.office_image(i).is_some_and(|o|o[0x2c6]==0&&u16::from_le_bytes([o[0x2c4],o[0x2c5]])<world.merchant_count())).unwrap();
        let mut office=world.office_image(office_index).unwrap();let owner=u16::from_le_bytes([office[0x2c4],office[0x2c5]]) as u8;
        let source_office=decode(row["office"].as_str().unwrap());office[0x2ce..0x2d2].copy_from_slice(&source_office[0x2ce..0x2d2]);
        model.world.offices[office_index]=p3_save::office::encode_office(office.as_slice().try_into().unwrap()).to_vec();
        let old=world.town(0);let mut town=old.clone();let source_town=decode(row["town"].as_str().unwrap());
        for offset in [0x760,0x764,0x76a,0x776]{town.set_u16(offset,u16::from_le_bytes([source_town[offset],source_town[offset+1]]));}
        town.set_u32(0x2c8,u32::from_le_bytes(source_town[0x2c8..0x2cc].try_into().unwrap()));town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);
        let mut records=decode(row["records"].as_str().unwrap());records[10]=owner;
        let site=p3_core::buildings::Site{records,cells:vec![0;128*256]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;
        let auction=sim.host.council_scheduler.add_task(0x70000000);let a=&mut sim.host.council_scheduler.records[usize::from(auction)];a.kind=0x36;a.payload[..6].copy_from_slice(&[1,0,0xff,0xff,0xff,0xff]);a.set_u8_at(0x15,0);
        let id=sim.host.council_scheduler.add_task(5);let task=&mut sim.host.council_scheduler.records[usize::from(id)];task.kind=0x26;task.payload.copy_from_slice(&decode(row["task"].as_str().unwrap())[8..24]);
        sim.idle.clock.tick=4;sim.idle.clock.calendar=Calendar::for_tick(4);sim.advance_with_merchant_daily(5,false).unwrap();
        let mut expected=decode(row["after_records"].as_str().unwrap());expected[10]=owner;assert_eq!(sim.host.sites[0].records,expected);
        let expected_office=decode(row["after_office"].as_str().unwrap());
        assert_eq!(sim.host.council_scheduler.records[usize::from(auction)].kind,u16::MAX);
        let live=p3_save::office::decode_office(&sim.persisted[office_index].to_save_v85());assert_eq!(&live[0x2ce..0x2d2],&expected_office[0x2ce..0x2d2]);
        sim.persist_all(&mut model).unwrap();let reloaded=Sim::from_save_bytes(&model.write()).unwrap();
        let saved=p3_save::office::decode_office(&reloaded.persisted[office_index].to_save_v85());assert_eq!(&saved[0x2ce..0x2d2],&expected_office[0x2ce..0x2d2]);
        assert_eq!(reloaded.host.council_scheduler.records[usize::from(auction)].kind,u16::MAX);
        assert_eq!(reloaded.host.sites[0].records,expected);
    }

    #[test]
    fn private_burn_and_auction_match_one_x86_execution() {
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_burn_auction.json")).unwrap();
        let rows=fixture["cases"].as_array().unwrap();assert_eq!(rows.len(),500);
        let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
        let parse=|v:&[u8]|p3_core::scheduler::TaskRecord{due:u32::from_le_bytes(v[0..4].try_into().unwrap()),next:u16::from_le_bytes(v[4..6].try_into().unwrap()),kind:u16::from_le_bytes(v[6..8].try_into().unwrap()),payload:v[8..24].try_into().unwrap()};
        let encode=|t:&p3_core::scheduler::TaskRecord|{let mut v=Vec::new();v.extend(t.due.to_le_bytes());v.extend(t.next.to_le_bytes());v.extend(t.kind.to_le_bytes());v.extend(t.payload);v};
        for(i,row)in rows.iter().enumerate(){
            let task=parse(&decode(row["task"].as_str().unwrap()));let auction=parse(&decode(row["auction"].as_str().unwrap()));
            let mut scheduler=p3_core::scheduler::Scheduler::new();scheduler.capacity=2;scheduler.head=0;scheduler.free_head=2;scheduler.records=vec![task,auction];
            let town=p3_core::town::Town(Box::new(<[u8;p3_core::town::TOWN_SIZE]>::try_from(decode(row["town"].as_str().unwrap())).unwrap()));
            let site=p3_core::buildings::Site{records:decode(row["records"].as_str().unwrap()),cells:vec![0;128*256]};
            let office=decode(row["office"].as_str().unwrap());let lists=[p3_core::buildings::counts::OfficeLists{next:u16::MAX,building_head:u16::from_le_bytes([office[0x2ce],office[0x2cf]])}];
            let housing=[p3_core::buildings::housing::OfficeHousing{houses_head:u16::from_le_bytes([office[0x2d0],office[0x2d1]]),..Default::default()}];
            let stocks=[p3_core::economy::trade::OfficeStock::default()];
            let (mut scheduler,_,task,town,site,office,keep)=super::prepare_burn(&scheduler,0,&[town],&[site],2,&stocks,&lists,&housing).unwrap_or_else(|e|panic!("caso {i}: {e}"));
            scheduler.records[0]=task;
            assert_eq!(encode(&scheduler.records[0]),decode(row["after_task"].as_str().unwrap()),"tarea {i}");
            assert_eq!(encode(&scheduler.records[1]),decode(row["after_auction"].as_str().unwrap()),"subasta {i}");
            assert_eq!(u64::from(scheduler.head),row["head"].as_u64().unwrap());assert_eq!(u64::from(scheduler.free_head),row["free"].as_u64().unwrap());
            assert_eq!(town.0.to_vec(),decode(row["after_town"].as_str().unwrap()),"ciudad {i}");assert_eq!(site.records,decode(row["after_records"].as_str().unwrap()),"edificios {i}");
            if let Some((_,businesses,houses))=office {let expected=decode(row["after_office"].as_str().unwrap());assert_eq!(businesses,u16::from_le_bytes([expected[0x2ce],expected[0x2cf]]));assert_eq!(houses,u16::from_le_bytes([expected[0x2d0],expected[0x2d1]]));}
            assert_eq!(u64::from(keep),row["result"].as_u64().unwrap());
        }
    }

    #[test]
    fn simple_wall_assault_runs_from_clock_and_persists() {
        let Ok(root)=std::env::var("P3_SAVES") else{return};
        let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_wall_assault_troops.json")).unwrap();
        let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
        let row=fixture["cases"].as_array().unwrap().iter().find(|r|{let a=decode(r["image"].as_str().unwrap());let b=decode(r["after_image"].as_str().unwrap());let t=decode(r["town"].as_str().unwrap());let u=decode(r["after_town"].as_str().unwrap());a[0..4]!=b[0..4]&&b[0x78]==0&&t[0x40..0x44]!=u[0x40..0x44]}).unwrap();
        let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}
        model.world.sieges[0]=decode(row["before"].as_str().unwrap());
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();let from=decode(row["town"].as_str().unwrap());
        town.set_u32(0x2c8,u32::from_le_bytes(from[0x2c8..0x2cc].try_into().unwrap()));town.set_u32(0x40,u32::from_le_bytes(from[0x40..0x44].try_into().unwrap()));town.set_u16(0x776,1);town.set_u16(0x766,0);town.set_u16(0x768,0);town.set_u16(0x762,u16::MAX);town.set_i32(0x2e0,0);town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);
        for i in 0..4{town.set_u8(0x998+i,from[0x998+i]);}
        let site=p3_core::buildings::Site{records:decode(row["records"].as_str().unwrap()),cells:vec![0;128*256]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;sim.host.siege_walled_live.resize(1,None);
        sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image:decode(row["image"].as_str().unwrap()),tower_bytes:vec![],list_bytes:vec![]});
        sim.idle.clock.tick=4;sim.idle.clock.calendar=Calendar::for_tick(4);sim.advance_with_merchant_daily(5,false).unwrap();
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image,decode(row["after_image"].as_str().unwrap()));
        assert_eq!(sim.economy.world.towns[0].u32(0x40),u32::from_le_bytes(decode(row["after_town"].as_str().unwrap())[0x40..0x44].try_into().unwrap()));
        sim.persist_all(&mut model).unwrap();let mut reloaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(reloaded.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));
        reloaded.host.siege_headless_march=true;let mut expected=reloaded.host.founded_world.as_ref().unwrap().sieges[0].clone();let t=&reloaded.economy.world.towns[0];let site=&reloaded.host.sites[0];
        let mut image=p3_state::siege::recover_walled_headless(&mut expected,t,site,site.wall_expansion_level(t)).unwrap().image;let mut test_town=t.clone();p3_state::siege::assault_walled_simple(&mut expected,&mut image,&mut test_town,site).unwrap();
        reloaded.idle.clock.tick=4;reloaded.idle.clock.calendar=Calendar::for_tick(4);reloaded.advance_with_merchant_daily(5,false).unwrap();
        assert_eq!(reloaded.host.founded_world.as_ref().unwrap().sieges[0],expected);
        assert_eq!(reloaded.economy.world.towns[0].u32(0x40),test_town.u32(0x40));
    }

    #[test]
    fn defensive_victory_from_clock_persists_letter_and_city(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_defense_victory.json")).unwrap();
        let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();let row=&fixture["cases"][0];
        let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}model.world.sieges[0]=decode(row["before"].as_str().unwrap());
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();let from=decode(row["town"].as_str().unwrap());let after=decode(row["after_town"].as_str().unwrap());
        town.set_u32(0x2c8,u32::from_le_bytes(from[0x2c8..0x2cc].try_into().unwrap()));town.set_u32(0x40,0);town.set_u16(0x776,1);town.set_u16(0x766,0);town.set_u16(0x768,0);town.set_u16(0x762,u16::MAX);town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);
        for i in 0..4{town.set_u8(0x998+i,from[0x998+i]);town.set_u16(0x300+2*i,u16::from_le_bytes(from[0x300+2*i..0x302+2*i].try_into().unwrap()));}
        let site=p3_core::buildings::Site{records:decode(row["records"].as_str().unwrap()),cells:vec![0;128*256]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;sim.host.siege_walled_live.resize(1,None);sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image:decode(row["image"].as_str().unwrap()),tower_bytes:vec![],list_bytes:vec![]});
        let mut pool=vec![0u8;12+4*16];pool[..4].copy_from_slice(&0xaffe0005u32.to_le_bytes());pool[4..6].copy_from_slice(&4u16.to_le_bytes());pool[8..10].fill(0xff);for j in 0..4{pool[12+j*16+4]=0xff;pool[12+j*16+6..12+j*16+8].copy_from_slice(&(if j<3{j as u16+1}else{u16::MAX}).to_le_bytes());}sim.host.message_pool=pool;
        sim.idle.clock.tick=4;sim.idle.clock.calendar=Calendar{day:15,month:4,year:1400,day_of_year:0};sim.advance_with_merchant_daily(5,false).unwrap();
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image,decode(row["after_image"].as_str().unwrap()));
        assert_eq!(&sim.host.message_pool[12..],&decode(row["slots"].as_str().unwrap()));for i in 0..4{assert_eq!(sim.economy.world.towns[0].u16(0x300+2*i),u16::from_le_bytes(after[0x300+2*i..0x302+2*i].try_into().unwrap()));}
        sim.persist_all(&mut model).unwrap();let loaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(loaded.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));assert_eq!(loaded.host.message_pool,sim.host.message_pool);for i in 0..4{assert_eq!(loaded.economy.world.towns[0].u16(0x300+2*i),sim.economy.world.towns[0].u16(0x300+2*i));}
    }

    #[test]
    fn defensive_wait_chain_runs_from_clock_and_cleans_scene(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_defense_wait.json")).unwrap();let row=&fixture["cases"][0];
        let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
        let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}model.world.sieges[0]=decode(row["before"].as_str().unwrap());
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();let from=decode(row["town"].as_str().unwrap());
        town.set_u32(0x2c8,u32::from_le_bytes(from[0x2c8..0x2cc].try_into().unwrap()));town.set_u16(0x776,1);town.set_u16(0x766,0);town.set_u16(0x768,0);town.set_u16(0x762,u16::MAX);town.set_i32(0x7a8,128);town.set_i32(0x7ac,255);for i in 0..4{town.set_u8(0x998+i,from[0x998+i]);}
        let site=p3_core::buildings::Site{records:decode(row["records"].as_str().unwrap()),cells:vec![0;128*255]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;sim.economy.world.world_8a=1;sim.host.siege_walled_live.resize(1,None);sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image:decode(row["image"].as_str().unwrap()),tower_bytes:vec![],list_bytes:vec![]});sim.idle.clock.tick=4;sim.idle.clock.calendar=Calendar::for_tick(4);
        for (i,step) in row["chain"].as_array().unwrap().iter().enumerate(){sim.advance_with_merchant_daily(5+i as u32,false).unwrap_or_else(|e|panic!("tick {i}: {e}"));assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0],decode(step["record"].as_str().unwrap()),"registro {i}");
            if step["image"].is_null(){assert!(sim.host.siege_walled_live[0].is_none())}else{assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image,decode(step["image"].as_str().unwrap()),"imagen {i}")}
            let expected=decode(step["town"].as_str().unwrap());assert_eq!(sim.economy.world.towns[0].u32(0x2c8)&0x10,u32::from_le_bytes(expected[0x2c8..0x2cc].try_into().unwrap())&0x10);}
        sim.persist_all(&mut model).unwrap();let loaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(loaded.host.founded_world.as_ref().unwrap().sieges[0],sim.host.founded_world.as_ref().unwrap().sieges[0]);
    }

    #[test]
    fn recovered_defensive_wait_and_march_run_from_clock_and_persist(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let source:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_walled_operation.json")).unwrap();
        for (is_march,raw_fixture) in [(false,include_str!("../../p3-core/tests/fixtures/c10_siege_walled_reload_wait.json")),(true,include_str!("../../p3-core/tests/fixtures/c10_siege_walled_reload_march.json"))]{let fixture:serde_json::Value=serde_json::from_str(raw_fixture).unwrap();
        let row=fixture["cases"].as_array().unwrap().iter().find(|r|r["retreat"].as_u64()==Some(1)&&r["removed"].as_bool()==Some(false)&&!r["towers"].as_str().unwrap().is_empty()).unwrap();let base=&source["cases"][row["source"].as_u64().unwrap()as usize]["input"];
        let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();
        for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}model.world.sieges[0]=decode(row["saved"].as_str().unwrap());let tick=row["tick"].as_u64().unwrap()as u32;model.world.header[0x18..0x1c].copy_from_slice(&tick.to_le_bytes());
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();let from=decode(row["town"].as_str().unwrap());
        town.set_u32(0x2c8,u32::from_le_bytes(from[0x2c8..0x2cc].try_into().unwrap()));town.set_u16(0x776,u16::from_le_bytes(from[0x776..0x778].try_into().unwrap()));town.set_u16(0x766,1);town.set_u16(0x768,u16::from_le_bytes(from[0x768..0x76a].try_into().unwrap()));town.set_u16(0x762,u16::from_le_bytes(from[0x762..0x764].try_into().unwrap()));town.set_i32(0x2e0,i32::from_le_bytes(from[0x2e0..0x2e4].try_into().unwrap()));town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);for i in 0..4{town.set_u8(0x998+i,from[0x998+i]);}
        let mut grid=vec![0u8;128*256];for p in base["grid"].as_array().unwrap(){grid[p[0].as_u64().unwrap()as usize]=p[1].as_u64().unwrap()as u8;}let site=p3_core::buildings::Site{records:decode(base["records"].as_str().unwrap()),cells:grid};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(sim.host.siege_retreat_flags[0],Some(true));sim.host.siege_headless_march=true;sim.economy.world.world_8a=1;if is_march{assert!(sim.prepare_walled_march(0).is_ok(),"{:?}",sim.prepare_walled_march(0).err())}else{assert!(sim.prepare_walled_wait(0).is_ok(),"{:?}",sim.prepare_walled_wait(0).err())}sim.advance_with_merchant_daily(tick+1,false).unwrap();
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));assert!(sim.host.siege_walled_live[0].as_ref().unwrap().tower_bytes.len()>0);
        sim.persist_all(&mut model).unwrap();let loaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(loaded.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));}
    }

    #[test]
    fn walled_departure_chain_runs_from_clock_and_persists(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_walled_departure.json")).unwrap();
        let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();for index in [1,254]{let row=&fixture["cases"][index];let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}model.world.sieges[0]=decode(row["before"].as_str().unwrap());
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();let from=decode(row["town"].as_str().unwrap());town.set_u32(0x2c8,u32::from_le_bytes(from[0x2c8..0x2cc].try_into().unwrap()));town.set_u16(0x776,u16::from_le_bytes(from[0x776..0x778].try_into().unwrap()));town.set_u16(0x766,0);town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);for i in 0..4{town.set_u8(0x998+i,from[0x998+i]);}
        let site=p3_core::buildings::Site{records:decode(row["records"].as_str().unwrap()),cells:vec![0;128*256]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;sim.economy.world.world_8a=1;sim.host.siege_walled_live.resize(1,None);sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image:decode(row["image"].as_str().unwrap()),tower_bytes:decode(row["towers"].as_str().unwrap()),list_bytes:vec![]});sim.idle.clock.tick=4;sim.idle.clock.calendar=Calendar::for_tick(4);
        for (i,step) in row["steps"].as_array().unwrap().iter().enumerate(){sim.advance_with_merchant_daily(5+i as u32,false).unwrap();assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0],decode(step["record"].as_str().unwrap()),"registro {i}");if step["image"].is_null(){assert!(sim.host.siege_walled_live[0].is_none())}else{assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image,decode(step["image"].as_str().unwrap()),"imagen {i}")}}
        sim.persist_all(&mut model).unwrap();let loaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(loaded.host.founded_world.as_ref().unwrap().sieges[0],sim.host.founded_world.as_ref().unwrap().sieges[0]);}
    }

    #[test]
    fn walled_plunder_runs_from_clock_and_persists(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_scene_march.json")).unwrap();let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
        for (timer,duration) in [(11u16,20u16),(1,1)]{let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();let mut record=decode(fixture["cases"][0]["before"].as_str().unwrap());record[20+0x18..20+0x1a].copy_from_slice(&timer.to_le_bytes());record[20+0x1a..20+0x1c].copy_from_slice(&duration.to_le_bytes());record[20+0x1e..20+0x20].copy_from_slice(&3u16.to_le_bytes());model.world.sieges[0]=record.clone();
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();town.set_u16(0x776,1);town.set_u16(0x766,0);town.set_u16(0x768,0);town.set_u16(0x762,u16::MAX);town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);
        let site=p3_core::buildings::Site{records:vec![20,20,0xfe,0x34,0xff,0xff,0,0],cells:vec![0;128*256]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut image=vec![0u8;0x650];p3_save::siege_scene::decode_fields(p3_save::world::siege_scene(&record,0).unwrap().unwrap(),&mut image).unwrap();image[0x78]=1;image[0x79]=1;image[0x578]=0xff;
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;sim.host.siege_walled_live.resize(1,None);sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image,tower_bytes:vec![],list_bytes:vec![]});sim.idle.clock.tick=100;sim.idle.clock.calendar=Calendar::for_tick(100);
        let(expected,image,_)=sim.prepare_walled_plunder(0,101).unwrap();sim.advance_with_merchant_daily(101,false).unwrap();assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges,expected.records);assert_eq!(sim.host.message_pool,expected.message_pool);assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image,image);
        sim.persist_all(&mut model).unwrap();let loaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(loaded.host.founded_world.as_ref().unwrap().sieges,expected.records);assert_eq!(loaded.host.message_pool,expected.message_pool);let mut wanted=expected.town.clone();let mut staged=wanted.clone();staged.0[0x840..0x990].fill(0);let active=staged.threshold(0,0)>1;let world=p3_core::economy::world::EconomyWorld{month:loaded.saved_calendar.1,..Default::default()};p3_core::economy::market::update_town_price_thresholds(&world,&mut staged,active,&mut crate::NullHost::default());wanted.0[0x4f0..0x670].copy_from_slice(&staged.0[0x4f0..0x670]);let actual=&loaded.economy.world.towns[0].0;let differences:Vec<_>=actual.iter().zip(wanted.0.iter()).enumerate().filter_map(|(i,(a,b))|(a!=b).then_some((i,*a,*b))).collect();assert!(differences.is_empty(),"ciudad recargada: {differences:?}");}
    }

    #[test]
    fn walled_march_runs_from_clock_and_saves(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_walled_march.json")).unwrap();let row=&fixture["cases"][1];
        let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}model.world.sieges[0]=decode(row["before"].as_str().unwrap());
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();let from=decode(row["town"].as_str().unwrap());town.set_u16(0x776,1);town.set_u16(0x766,0);town.set_u16(0x768,0);town.set_u16(0x762,u16::MAX);town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);for i in 0..4{town.set_u8(0x998+i,from[0x998+i]);}
        let site=p3_core::buildings::Site{records:vec![20,20,0xfe,0x34,0xff,0xff,0,0],cells:vec![0;128*256]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;sim.host.siege_walled_live.resize(1,None);sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image:decode(row["image"].as_str().unwrap()),tower_bytes:vec![0;12],list_bytes:vec![]});sim.idle.clock.tick=4;sim.idle.clock.calendar=Calendar::for_tick(4);sim.advance_with_merchant_daily(5,false).unwrap();
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image,decode(row["after_image"].as_str().unwrap()));
        sim.persist_all(&mut model).unwrap();let loaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(loaded.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));
    }

    #[test]
    fn mixed_assault_persists_population_and_militia(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_wall_assault_mixed.json")).unwrap();
        let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
        let row=fixture["cases"].as_array().unwrap().iter().find(|r|{let a=decode(r["town"].as_str().unwrap());let b=decode(r["after_town"].as_str().unwrap());(1..4).any(|i|b[0x998+i]<a[0x998+i])}).unwrap();
        let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}model.world.sieges[0]=decode(row["before"].as_str().unwrap());
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();let from=decode(row["town"].as_str().unwrap());let after=decode(row["after_town"].as_str().unwrap());
        for at in [0x40,0x2c8,0x2d4,0x2d8,0x2dc,0x2e0,0x2e4]{town.set_u32(at,u32::from_le_bytes(from[at..at+4].try_into().unwrap()))}
        town.set_u16(0x844,u16::from_le_bytes(from[0x844..0x846].try_into().unwrap()));town.set_u16(0x776,1);town.set_u16(0x766,0);town.set_u16(0x768,0);town.set_u16(0x762,u16::MAX);town.set_i32(0x2e0,u32::from_le_bytes(from[0x2e0..0x2e4].try_into().unwrap())as i32);town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);
        for i in 0..4{town.set_u8(0x998+i,from[0x998+i]);}
        let site=p3_core::buildings::Site{records:decode(row["records"].as_str().unwrap()),cells:vec![0;128*256]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;sim.host.siege_walled_live.resize(1,None);sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image:decode(row["image"].as_str().unwrap()),tower_bytes:vec![],list_bytes:vec![]});
        sim.idle.clock.tick=4;sim.idle.clock.calendar=Calendar::for_tick(4);sim.advance_with_merchant_daily(5,false).unwrap();
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));
        for at in [0x2d4,0x2d8,0x2dc,0x2e0,0x2e4]{assert_eq!(sim.economy.world.towns[0].u32(at),u32::from_le_bytes(after[at..at+4].try_into().unwrap()));}
        for i in 0..4{assert_eq!(sim.economy.world.towns[0].u8(0x998+i),after[0x998+i]);}assert_eq!(sim.economy.world.towns[0].u16(0x844),u16::from_le_bytes(after[0x844..0x846].try_into().unwrap()));
        sim.persist_all(&mut model).unwrap();let reloaded=Sim::from_save_bytes(&model.write()).unwrap();let town=&reloaded.economy.world.towns[0];
        for at in [0x2d4,0x2d8,0x2dc,0x2e0,0x2e4]{assert_eq!(town.u32(at),u32::from_le_bytes(after[at..at+4].try_into().unwrap()));}
        for i in 0..4{assert_eq!(town.u8(0x998+i),after[0x998+i]);}assert_eq!(town.u16(0x844),u16::from_le_bytes(after[0x844..0x846].try_into().unwrap()));
    }

    #[test]
    fn wall_tower_assault_runs_from_clock_and_saves(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_wall_assault_towers.json")).unwrap();
        let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
        let row=fixture["cases"].as_array().unwrap().iter().find(|r|{let a=decode(r["image"].as_str().unwrap());let b=decode(r["after_image"].as_str().unwrap());a[0x240..0x242]!=b[0x240..0x242]&&b[0x78]==0&&u16::from_le_bytes([b[0x240],b[0x241]])>4000}).unwrap();
        let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}model.world.sieges[0]=decode(row["before"].as_str().unwrap());
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();let from=decode(row["town"].as_str().unwrap());
        for at in [0x40,0x2c8,0x2d4,0x2d8,0x2dc,0x2e0,0x2e4]{town.set_u32(at,u32::from_le_bytes(from[at..at+4].try_into().unwrap()))}
        town.set_u16(0x844,u16::from_le_bytes(from[0x844..0x846].try_into().unwrap()));town.set_u16(0x776,u16::from_le_bytes(from[0x776..0x778].try_into().unwrap()));town.set_u16(0x766,0);town.set_u16(0x768,0);town.set_u16(0x762,u16::MAX);town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);
        for i in 0..4{town.set_u8(0x998+i,from[0x998+i]);}
        let mut records=decode(row["records"].as_str().unwrap());let count=records.len()/8;for i in 0..count{let next=if i+1<count{(i+1)as u16}else{u16::MAX};records[i*8+4..i*8+6].copy_from_slice(&next.to_le_bytes());}
        let site=p3_core::buildings::Site{records,cells:vec![0;128*256]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;sim.host.siege_walled_live.resize(1,None);sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image:decode(row["image"].as_str().unwrap()),tower_bytes:decode(row["towers"].as_str().unwrap()),list_bytes:vec![]});
        sim.idle.clock.tick=4;sim.idle.clock.calendar=Calendar::for_tick(4);sim.advance_with_merchant_daily(5,false).unwrap();
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image,decode(row["after_image"].as_str().unwrap()));
        sim.persist_all(&mut model).unwrap();let mut reloaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(reloaded.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));
        reloaded.host.siege_headless_march=true;let mut expected=reloaded.host.founded_world.as_ref().unwrap().sieges[0].clone();let town=&reloaded.economy.world.towns[0];let site=&reloaded.host.sites[0];
        let recovered=p3_state::siege::recover_walled_headless(&mut expected,town,site,site.wall_expansion_level(town)).unwrap();let mut image=recovered.image;let mut next_town=town.clone();p3_state::siege::assault_walled(&mut expected,&mut image,&mut next_town,site,&recovered.tower_bytes).unwrap();
        reloaded.idle.clock.tick=4;reloaded.idle.clock.calendar=Calendar::for_tick(4);reloaded.advance_with_merchant_daily(5,false).unwrap();assert_eq!(reloaded.host.founded_world.as_ref().unwrap().sieges[0],expected);assert_eq!(reloaded.economy.world.towns[0],next_town);
    }

    #[test]
    fn tower_loss_schedules_task_and_persists(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_wall_tower_loss.json")).unwrap();let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
        let row=fixture["cases"].as_array().unwrap().iter().find(|r|r["due"].as_array().unwrap().len()==1).unwrap();
        let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}model.world.sieges[0]=decode(row["before"].as_str().unwrap());
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();let from=decode(row["town"].as_str().unwrap());
        for at in [0x40,0x2c8,0x2d4,0x2d8,0x2dc,0x2e0,0x2e4]{town.set_u32(at,u32::from_le_bytes(from[at..at+4].try_into().unwrap()))}
        town.set_u16(0x844,u16::from_le_bytes(from[0x844..0x846].try_into().unwrap()));town.set_u16(0x776,u16::from_le_bytes(from[0x776..0x778].try_into().unwrap()));town.set_u16(0x766,0);town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);for i in 0..4{town.set_u8(0x998+i,from[0x998+i]);}
        let site=p3_core::buildings::Site{records:decode(row["records"].as_str().unwrap()),cells:vec![0;128*256]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;sim.host.siege_walled_live.resize(1,None);sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image:decode(row["image"].as_str().unwrap()),tower_bytes:decode(row["towers"].as_str().unwrap()),list_bytes:vec![]});
        sim.idle.clock.tick=4;sim.idle.clock.calendar=Calendar::for_tick(4);sim.advance_with_merchant_daily(5,false).unwrap();
        assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0],decode(row["after"].as_str().unwrap()));assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image,decode(row["after_image"].as_str().unwrap()));assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().tower_bytes,decode(row["after_towers"].as_str().unwrap()));
        let task=sim.host.council_scheduler.records.iter().find(|t|t.kind==0x25&&t.due==6).unwrap();assert_eq!(&task.payload[..10],&decode(row["task_after"].as_str().unwrap())[8..18]);
        sim.persist_all(&mut model).unwrap();let reloaded=Sim::from_save_bytes(&model.write()).unwrap();assert!(reloaded.host.council_scheduler.records.iter().any(|t|t.kind==0x25&&t.due==6));
        sim.advance_with_merchant_daily(6,false).unwrap();assert_eq!(sim.idle.clock.tick,6);
    }

    #[test]
    fn tower_ruin_task_runs_and_persists(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_tower_ruin.json")).unwrap();let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();
        let row=fixture["cases"].as_array().unwrap().iter().find(|r|{let task=decode(r["task"].as_str().unwrap());let records=decode(r["records"].as_str().unwrap());task[0x11]==1&&records[11]==0x32&&r["result"].as_u64().unwrap()==1}).unwrap();
        let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();let from=decode(row["town"].as_str().unwrap());
        for at in [0x760,0x768,0x76a,0x776]{town.set_u16(at,u16::from_le_bytes(from[at..at+2].try_into().unwrap()))}town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);
        let site=p3_core::buildings::Site{records:decode(row["records"].as_str().unwrap()),cells:vec![0;128*256]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;let id=sim.host.council_scheduler.add_task(5);let task=&mut sim.host.council_scheduler.records[usize::from(id)];task.kind=0x25;task.payload.copy_from_slice(&decode(row["task"].as_str().unwrap())[8..24]);
        sim.idle.clock.tick=4;sim.idle.clock.calendar=Calendar::for_tick(4);sim.advance_with_merchant_daily(5,false).unwrap();
        assert_eq!(sim.host.sites[0].records,decode(row["after_records"].as_str().unwrap()));let task=&sim.host.council_scheduler.records[usize::from(id)];assert_eq!(task.kind,0x25);assert_eq!(task.due,10);assert_eq!(&task.payload,&decode(row["after_task"].as_str().unwrap())[8..24]);
        sim.persist_all(&mut model).unwrap();let reloaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(reloaded.host.sites[0].records,sim.host.sites[0].records);assert_eq!(reloaded.host.council_scheduler.records[usize::from(id)].due,10);
    }

    #[test]
    fn tower_loss_ruin_and_next_tick_run_from_clock(){
        let Ok(root)=std::env::var("P3_SAVES") else{return};let raw=std::fs::read(format!("{root}/visibility/save/Ein/AUTO.pat")).unwrap();
        let fixture:serde_json::Value=serde_json::from_str(include_str!("../../p3-core/tests/fixtures/c10_siege_tower_ruin_chain.json")).unwrap();let decode=|s:&str|s.as_bytes().chunks_exact(2).map(|b|u8::from_str_radix(std::str::from_utf8(b).unwrap(),16).unwrap()).collect::<Vec<_>>();let row=&fixture["cases"][0];
        let mut model=p3_save::model::SaveModel::parse(&raw).unwrap();for(city,record)in model.world.sieges.iter_mut().enumerate(){*record=vec![0;20];record[4]=city as u8;}model.world.sieges[0]=decode(row["before"].as_str().unwrap());
        let data=model.world.write();let world=p3_save::world::World::parse(&data).unwrap();let old=world.town(0);let mut town=old.clone();let from=decode(row["town"].as_str().unwrap());
        for at in [0x40,0x2c8,0x2d4,0x2d8,0x2dc,0x2e0,0x2e4]{town.set_u32(at,u32::from_le_bytes(from[at..at+4].try_into().unwrap()))}for at in [0x760,0x768,0x76a,0x776,0x844]{town.set_u16(at,u16::from_le_bytes(from[at..at+2].try_into().unwrap()))}
        town.set_u16(0x766,0);town.set_i32(0x7a8,128);town.set_i32(0x7ac,256);for i in 0..4{town.set_u8(0x998+i,from[0x998+i]);}
        let site=p3_core::buildings::Site{records:decode(row["records"].as_str().unwrap()),cells:vec![0;128*256]};apply_fixture_town(&mut model.world.towns[0],&old,&town,&site);
        let mut sim=Sim::from_save_bytes(&model.write()).unwrap();sim.host.siege_headless_march=true;sim.host.siege_walled_live.resize(1,None);sim.host.siege_walled_live[0]=Some(crate::LiveWalledSiege{image:decode(row["image"].as_str().unwrap()),tower_bytes:decode(row["towers"].as_str().unwrap()),list_bytes:vec![]});
        sim.idle.clock.tick=0xf4;sim.idle.clock.calendar=Calendar::for_tick(0xf4);sim.advance_with_merchant_daily(0xf5,false).unwrap();assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0],decode(row["first"].as_str().unwrap()));
        sim.advance_with_merchant_daily(0xf6,false).unwrap();assert_eq!(sim.host.founded_world.as_ref().unwrap().sieges[0],decode(row["second"].as_str().unwrap()));assert_eq!(sim.host.siege_walled_live[0].as_ref().unwrap().image,decode(row["image_second"].as_str().unwrap()));assert_eq!(sim.host.sites[0].records,decode(row["records_ruin"].as_str().unwrap()));
        let task=sim.host.council_scheduler.records.iter().find(|t|t.kind==0x25&&t.due==0xfb).unwrap();assert_eq!(&task.payload[..10],&decode(row["task_after"].as_str().unwrap())[8..18]);
        sim.persist_all(&mut model).unwrap();let reloaded=Sim::from_save_bytes(&model.write()).unwrap();assert_eq!(reloaded.host.founded_world.as_ref().unwrap().sieges[0],decode(row["second"].as_str().unwrap()));assert_eq!(reloaded.host.sites[0].records,decode(row["records_ruin"].as_str().unwrap()));
    }

}
