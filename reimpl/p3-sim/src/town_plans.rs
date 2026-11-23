//! Recursos TOW para la fundación de ciudades (C10).
use p3_core::{buildings::{planlife::PlanFileHost,Site},town::Town};

/// Archivos por índice; el original intenta el número pedido y luego el 00.
#[derive(Clone,Debug)]
pub struct TownPlanBank { plans: Vec<Option<Vec<u8>>> }

/// Datos de la operación de alta para una ciudad nueva durante la partida.
#[derive(Clone, Copy, Debug)]
pub struct FoundTownParams {
    pub map_id: u8, pub plan_id: u8, pub flags: u32, pub location_kind: u8,
    pub full: u32, pub half: u32, pub tick: u32, pub year: u16, pub day_of_year: u16,
    pub day: u8, pub population_seed: u32, pub first_names: u16, pub last_names: u16,
    pub player: u16, pub player_home: u8, pub game_mode: u8,
}

/// Resultado preparado sin modificar el mundo ni el pool de entrada.
pub struct PreparedTown {
    pub town: Town,
    pub site: Site,
    pub captains: p3_core::ai::settlement::TavernCaptainPool,
    pub captains_before: p3_core::ai::settlement::TavernCaptainPool,
    pub entries: Vec<p3_core::economy::world::TickEntry>,
    pub scheduler: p3_core::scheduler::Scheduler,
    pub scheduler_before: p3_core::scheduler::Scheduler,
    pub saved: p3_save::model::TownModel,
    pub siege: [u8; 20],
}

impl PreparedTown {
    /// Prepara conjuntamente mundo, capitanes, tareas y planificador sobre
    /// copias. La operación y los demás estados vivos se publican aparte.
    pub fn stage_save(&self,model:&p3_save::model::SaveModel,locations:&[p3_core::ai::settlement::SettlementLocation;40],planner:&p3_core::route::finder::RoutePlanner,factory:&p3_core::ships::factory::HanseFactory,max_rings:u32)->Result<(p3_save::model::SaveModel,p3_core::route::finder::RoutePlanner,p3_core::ships::factory::HanseFactory),String>{
        if factory.captains!=self.captains_before.captains||factory.free_captain!=self.captains_before.free_head{return Err("pool de capitanes cambió desde la preparación".into())}
        let baseline=p3_save::tail::update_scheduled_task_table(&model.scheduled_tasks,&self.scheduler_before)?;
        if baseline!=model.scheduled_tasks{return Err("planificador de tareas cambió desde la preparación".into())}
        let (world,next_planner)=self.add_port_and_append(&model.world,locations,planner,max_rings)?;
        let mut next_factory=factory.clone();next_factory.captains=self.captains.captains.clone();next_factory.free_captain=self.captains.free_head;
        let mut next=model.clone();next.world=world;next.ships.apply_hanse_factory(factory,&next_factory)?;
        next.scheduled_tasks=p3_save::tail::update_scheduled_task_table(&next.scheduled_tasks,&self.scheduler)?;
        Ok((next,next_planner,next_factory))
    }
    /// Añade el puerto al planificador anterior y prepara la ampliación S05.
    /// El límite de anillos evita que el cálculo original quede bloqueado si
    /// no encuentra suficientes puntos de mar; trabaja siempre sobre copias.
    pub fn add_port_and_append(&self,world:&p3_save::model::WorldModel,locations:&[p3_core::ai::settlement::SettlementLocation;40],planner:&p3_core::route::finder::RoutePlanner,max_rings:u32)->Result<(p3_save::model::WorldModel,p3_core::route::finder::RoutePlanner),String>{
        let n=world.towns.len();let id=self.town.u8(0x2c1) as usize;let registered=usize::from(planner.registered);
        if n>=40||max_rings==0||world.header.len()<0x20+n||planner.town_count as usize!=n||planner.town_order.len()<n||planner.town_table.len()!=n*n||id>=planner.town_slots.len()||registered>n||planner.sea_counts.len()<registered||planner.sea_points.len()<registered{return Err("planificador de puertos incompleto".into())}
        if planner.town_order[..n]!=world.header[0x20..0x20+n] || planner.town_order[..n].iter().any(|&v|usize::from(v)>=planner.town_slots.len()){return Err("orden de puertos incompatible".into())}
        let grid=&planner.nav.grid;if grid.width<=0||grid.height<=0||grid.scale<=0||grid.cells.len()<(grid.width as usize)*(grid.height as usize){return Err("rejilla de navegación incompleta".into())}
        let nodes=planner.nav.nodes.len();if nodes==0||planner.nav.node_count as usize!=nodes||planner.nav.matrix.len()!=nodes||planner.nav.matrix.iter().any(|r|r.len()!=nodes||r.iter().any(|&(_,next)|next<0||next as usize>=nodes)){return Err("grafo de navegación incompleto".into())}
        for target in 0..nodes{for start in 0..nodes{let mut at=start;let mut steps=0;while at!=target{at=planner.nav.matrix[at][target].1 as usize;steps+=1;if steps>nodes{return Err("grafo de navegación cíclico".into())}}}}
        let mut next=planner.clone();if next.town_order.len()==n{next.town_order.push(id as u8)}else{next.town_order[n]=id as u8}next.town_count=(n+1)as u32;
        if !next.add_town_routes(id as u8,max_rings){return Err("no se encontraron puntos de mar para el nuevo puerto".into())}
        self.append_to_world(world,locations,&next)
    }

    /// Amplía una copia del mundo y del planificador ya actualizado con el
    /// puerto nuevo. No publica ninguna de las copias si la matriz falla.
    pub fn append_to_world(&self, world:&p3_save::model::WorldModel, locations:&[p3_core::ai::settlement::SettlementLocation;40], planner:&p3_core::route::finder::RoutePlanner)->Result<(p3_save::model::WorldModel,p3_core::route::finder::RoutePlanner),String>{
        let n=world.towns.len();if n>=40||usize::from(self.town.u8(0x2c0))!=n||world.header.len()<0x48+n{return Err("mundo y ciudad preparada incompatibles".into())}
        let mut ids=world.header[0x20..0x20+n].to_vec();ids.push(self.town.u8(0x2c1));
        let mut next_planner=planner.clone();let matrix=p3_core::ai::settlement::build_town_distance_matrix_with_planner(Some(&world.matrix),&ids,locations,&mut next_planner).map_err(str::to_string)?;
        let mut next_world=world.clone();next_world.append_town(self.saved.clone(),self.town.u8(0x2c1),self.town.u8(0x2c2),matrix,self.siege.to_vec())?;
        Ok((next_world,next_planner))
    }
}

impl TownPlanBank {
    /// Acepta la operación 0x4B y prepara el inicio de misión 0 sin publicar
    /// ningún estado si falla la creación, las tareas o los recursos.
    pub fn prepare_founding_mission(&self,existing:&[Town],memory:&Town,pool:&p3_core::ai::settlement::TavernCaptainPool,scheduler:&p3_core::scheduler::Scheduler,merchants:&[p3_core::ai::reputation::Merchant],merchant_count:u16,op:&p3_core::pump::Operation,_locations:&[p3_core::ai::settlement::SettlementLocation;40],p:FoundTownParams)->Result<Option<PreparedTown>,String>{
        use p3_core::ai::settlement::accept_alderman_mission;
        if op.0[0]!=0x4b{return Err("operación de misión incorrecta".into())}
        let mut next=scheduler.clone();let Some(id)=accept_alderman_mission(&mut next,op,merchants,merchant_count).map_err(str::to_string)? else{return Ok(None)};
        let mut result=self.prepare_assigned_founding_mission(existing,memory,pool,&next,id,merchant_count,p)?;
        result.scheduler_before=scheduler.clone();Ok(Some(result))
    }

    /// Prepara el inicio que el scheduler alcanza tras asignar una oferta.
    /// Mantiene la tarea como cabeza hasta que el llamante termina el despacho.
    pub fn prepare_assigned_founding_mission(&self,existing:&[Town],memory:&Town,pool:&p3_core::ai::settlement::TavernCaptainPool,scheduler:&p3_core::scheduler::Scheduler,id:u16,merchant_count:u16,mut p:FoundTownParams)->Result<PreparedTown,String>{
        use p3_core::ai::settlement::{begin_founding_mission,finish_founding_mission};
        let mut next=scheduler.clone();
        let city=u8::try_from(existing.len()).map_err(|_|"número de ciudades inválido")?;
        let task=next.records.get_mut(usize::from(id)).filter(|_|id<next.capacity).ok_or("misión ausente")?;
        let request=begin_founding_mission(task,city,merchant_count).map_err(str::to_string)?;
        p.map_id=request.map_id;p.plan_id=request.plan_id;p.flags=0;p.full=request.full;p.half=0;
        let mut result=self.prepare_founded_town(existing,memory,pool,&next,p)?;
        finish_founding_mission(&mut result.scheduler,id,&mut result.town,p.tick).map_err(str::to_string)?;
        result.saved=p3_save::model::TownModel::from_live(&result.town,&result.site,&result.entries)?;
        result.scheduler_before=scheduler.clone();Ok(result)
    }

    /// Prepara las partes ya portadas de `0x00531D50` para el caso de una
    /// ciudad nueva en partida. La publicación, nombres y rutas quedan
    /// a cargo de la transacción global; no cambia sus argumentos.
    pub fn prepare_founded_town(&self, existing:&[Town], memory:&Town, pool:&p3_core::ai::settlement::TavernCaptainPool, scheduler:&p3_core::scheduler::Scheduler, p:FoundTownParams)->Result<PreparedTown,String>{
        use p3_core::ai::settlement::{construct_town,init_town_council,add_tavern_captain,init_town_shipyard,init_founded_town,schedule_founded_town};
        if existing.len()>=40 || p.map_id>=40 {return Err("índice de ciudad inválido".into())}
        let city=existing.len();let mut towns=existing.to_vec();let mut town=memory.clone();
        construct_town(&mut town,p.player,p.player_home,p.game_mode);
        town.set_u8(0x2c0,city as u8);town.set_u8(0x2c1,p.map_id);town.set_u8(0x2c2,p.plan_id);
        town.set_u32(0x2c8,p.flags|u32::from(p.location_kind!=0));towns.push(town);
        let factor=u32::from(p.map_id)+1;
        init_town_council(&mut towns,city,p.first_names,p.last_names,p.tick,0,p.full.wrapping_mul(factor),p.half.wrapping_mul(factor)).map_err(str::to_string)?;
        let mut town=towns.pop().expect("ciudad añadida");let mut captains=pool.clone();
        for pirate in [false,true]{add_tavern_captain(&mut town,&mut captains,pirate,p.tick,p.day_of_year,p.day,p.population_seed,p.first_names,p.last_names).map_err(str::to_string)?;}
        init_town_shipyard(&mut town,0);let mut site=Site::default();self.reset_and_load(&mut town,&mut site,p.plan_id)?;
        let entries=init_founded_town(&mut town,&site,(city+1)as u32,p.full,p.half).map_err(str::to_string)?;
        let saved=p3_save::model::TownModel::from_live(&town,&site,&entries)?;
        let scheduler_before=scheduler.clone();let mut scheduler=scheduler_before.clone();schedule_founded_town(&mut scheduler,city as u8,p.flags,0,p.half,p.tick,p.year).map_err(str::to_string)?;
        Ok(PreparedTown{town,site,captains,captains_before:pool.clone(),entries,scheduler,scheduler_before,saved,siege:p3_save::model::new_town_siege(city as u8)})
    }
    pub fn from_archives(archives:&[&[u8]])->Result<Self,String>{
        let parsed=archives.iter().map(|b|p3_assets::cpr::Archive::parse(b).map_err(|e|format!("CPR: {e}"))).collect::<Result<Vec<_>,_>>()?;
        let mut plans=vec![None;256];
        for (i,slot) in plans.iter_mut().enumerate(){let path=format!("iso/towns/stadtplatz{i:02}.tow");for archive in parsed.iter().rev(){if let Some(entry)=archive.find(path.as_bytes()){*slot=Some(archive.read(entry).ok_or_else(||format!("plano no disponible: {path}"))?.to_vec());break}}}
        Ok(Self{plans})
    }

    /// Aplica el reinicio y el cargador original; rechaza datos truncados que
    /// provocarían lecturas fuera del fichero antes de mutar la ciudad.
    pub fn reset_and_load(&self,town:&mut Town,site:&mut Site,index:u8)->Result<(),String>{
        let selected=self.plans[usize::from(index)].as_deref().or(self.plans[0].as_deref());
        if let Some(data)=selected{if data.len()>=0x6c{let rows=u32::from_le_bytes(data[0..4].try_into().unwrap())as usize;let cols=u32::from_le_bytes(data[4..8].try_into().unwrap())as usize;let cells=rows.checked_mul(cols).and_then(|n|n.checked_add(0x64)).ok_or("dimensiones del plano inválidas")?;if rows>i32::MAX as usize||cols>i32::MAX as usize||cells>data.len(){return Err("plano truncado o de dimensiones inválidas".into())}}}
        struct Host<'a>{plan:Option<&'a [u8]>,fallback:Option<&'a [u8]>,selected:Option<&'a [u8]>}
        impl PlanFileHost for Host<'_>{fn open(&mut self,attempt:u32)->bool{self.selected=if attempt==0{self.plan}else{self.fallback};self.selected.is_some()}fn length(&mut self)->u32{self.selected.map_or(0,|v|v.len() as u32)}fn alloc(&mut self,_:u32)->bool{true}fn read(&mut self,_:u32)->Vec<u8>{self.selected.unwrap_or_default().to_vec()}fn close(&mut self){}}
        let mut host=Host{plan:self.plans[usize::from(index)].as_deref(),fallback:self.plans[0].as_deref(),selected:None};
        p3_core::ai::settlement::reset_town_buildings(town,site);site.plan_load_from_file(town,&mut host);Ok(())
    }
}

impl crate::Sim {
    /// Acepta y publica en el estado vivo una misión de fundación con los
    /// recursos disponibles. La operación se prepara en copias antes de mutar.
    pub(crate) fn accept_founding_mission(&mut self,op:&p3_core::pump::Operation)->Result<(),String>{self.run_founding_mission(op,false)}
    pub(crate) fn start_assigned_founding_mission(&mut self,index:u16,expected:p3_core::scheduler::TaskRecord)->Result<(),String>{
        if self.host.council_scheduler.head!=index||self.host.council_scheduler.records.get(usize::from(index))!=Some(&expected){return Err("tarea de fundación cambió durante el despacho".into())}
        let op=p3_core::pump::Operation([0x4b,u32::from(expected.u8_at(0xd)),u32::from(index),0,0]);self.run_founding_mission(&op,true)
    }
    fn run_founding_mission(&mut self,op:&p3_core::pump::Operation,assigned:bool)->Result<(),String>{
        use p3_core::ai::settlement::TavernCaptainPool;
        if assigned {
            // 0x531D50 añade dos tareas. Si la cola crece durante el manejador,
            // el original conserva un puntero al registro anterior; no emular
            // silenciosamente una política de asignador desconocida.
            let scheduler=&self.host.council_scheduler;let mut active=vec![false;usize::from(scheduler.capacity)];let mut cursor=scheduler.head;
            while cursor<scheduler.capacity {let i=usize::from(cursor);if active[i]{return Err("lista activa de tareas cíclica".into())}active[i]=true;cursor=scheduler.records.get(i).ok_or("lista activa de tareas truncada")?.next;}
            let mut at=scheduler.free_head;let mut free=0;let mut first=None;
            while at<scheduler.capacity&&free<2 {if first==Some(at){return Err("lista libre de tareas cíclica".into())}if active[usize::from(at)]{return Err("lista libre de tareas ocupada".into())}let record=scheduler.records.get(usize::from(at)).ok_or("lista libre de tareas truncada")?;if first.is_none(){first=Some(at)}at=record.next;free+=1;}
            if free<2{return Err("fundación programada requiere crecimiento del planificador".into())}
        }
        let n=self.economy.world.towns.len();let world=self.host.founded_world.as_ref().ok_or("mundo de fundación ausente")?;
        if world.towns.len()!=n||self.host.sites.len()!=n||self.economy.world.tick_entries.len()!=n||self.host.expedition_town_ids.len()!=n||world.header.get(0x20..0x20+n)!=Some(self.host.expedition_town_ids.as_slice()){return Err("ciudades de fundación desincronizadas".into())}
        let bank=self.host.settlement_plans.as_ref().ok_or("faltan planos de ciudad")?;
        let locations=self.host.settlement_locations.as_ref().ok_or("faltan posiciones de ciudad")?;
        let inland=self.host.settlement_inland.as_ref().ok_or("faltan datos AmFluss de ciudad")?;
        let map=self.host.council_scheduler.records.get(op.0[2] as usize).ok_or("tarea de fundación ausente")?.u8_at(0x14);if map>=40{return Err("posición de fundación inválida".into())}
        let (first_names,last_names)=self.host.foundation_names.or_else(||self.host.expedition_resources.as_ref().map(|v|(v.first_names,v.last_names))).ok_or("faltan nombres para fundación")?;
        let resources=self.host.expedition_resources.as_ref();
        if resources.is_some_and(|v|v.ports.len()!=n){return Err("puertos de fundación desincronizados".into())}
        let factory=self.host.expedition_factory.as_ref().ok_or("pool de capitanes ausente")?;
        let planner=if let Some(existing)=self.host.navigation_planner.as_ref().filter(|v|v.town_count as usize==n && v.town_table.len()==n*n){existing.clone()}else{self.host.foundation_navigation.as_ref().ok_or("planificador de puertos ausente")?.build_planner(&self.host.expedition_town_ids,100_000)?};
        let date=self.idle.clock.calendar;let population=self.economy.world.towns.get(usize::from(date.day_of_year&15)).ok_or("ciudad de semilla de capitán ausente")?.u32(0x2e0);
        let pool=TavernCaptainPool{captains:factory.captains.clone(),free_head:factory.free_captain};
        let params=FoundTownParams{map_id:0,plan_id:0,flags:0,location_kind:inland[usize::from(map)],full:0,half:0,tick:self.idle.clock.tick,year:date.year,day_of_year:date.day_of_year,day:date.day,population_seed:population,first_names,last_names,player:self.economy.world.merchant_count,player_home:self.economy.world.alderman_town,game_mode:self.economy.world.alderman_merchant};
        if let Some(nav)=&self.host.navigation{if nav.ports.len()!=n||nav.snapshot.manager.len()<0xf8||nav.snapshot.captains!=factory.captains.iter().flatten().copied().collect::<Vec<_>>(){return Err("capitanes o puertos navales desincronizados".into())}}
        let mut prepared=if assigned {bank.prepare_assigned_founding_mission(&self.economy.world.towns,&Town::default(),&pool,&self.host.council_scheduler,op.0[2] as u16,self.economy.world.merchant_count,params)?}else{let Some(prepared)=bank.prepare_founding_mission(&self.economy.world.towns,&Town::default(),&pool,&self.host.council_scheduler,&self.rep_merchants,self.economy.world.merchant_count,op,locations,params)? else{return Ok(())};prepared};
        if assigned {let task=&mut prepared.scheduler.records[op.0[2] as usize];task.due=task.due.wrapping_add(0x100);prepared.scheduler.reschedule_first();prepared.scheduler.dispatching=0;}
        let (next_world,next_planner)=prepared.add_port_and_append(world,locations,&planner,100_000)?;
        let id=prepared.town.u8(0x2c1);let loc=locations[usize::from(id)];let port=(loc.x as i16,loc.y as i16);
        let mut next_factory=factory.clone();next_factory.captains=prepared.captains.captains.clone();next_factory.free_captain=prepared.captains.free_head;
        let mut next_resources=resources.cloned();if let Some(resources)=next_resources.as_mut(){resources.ports.push(Some((port.0 as u16,port.1 as u16)));}
        if let Some(nav)=self.host.navigation.as_mut(){nav.ports.push(port);nav.snapshot.captains=next_factory.captains.iter().flatten().copied().collect();nav.snapshot.manager[0xe0..0xe2].copy_from_slice(&next_factory.free_captain.to_le_bytes());nav.snapshot.manager[0xf2..0xf4].copy_from_slice(&(next_factory.captains.len() as u16).to_le_bytes());}
        self.host.founded_world=Some(next_world);self.host.navigation_planner=Some(next_planner);self.host.council_scheduler=prepared.scheduler;self.host.expedition_factory=Some(next_factory);self.host.expedition_resources=next_resources;self.host.expedition_town_ids.push(id);self.host.sites.push(prepared.site);self.economy.world.tick_entries.push(prepared.entries);self.economy.world.towns.push(prepared.town);self.idle.clock.town_count=(n+1)as u32;
        Ok(())
    }
}
