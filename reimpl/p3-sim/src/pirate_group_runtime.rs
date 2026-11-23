//! Puente transaccional para la memoria x86 del tick diario 514000.
//! El anfitrión aporta callbacks nativos clonables; un callback no conectado
//! debe devolver Err. Ningún efecto se publica cuando cualquiera de ellos falla.
use p3_core::{economy::pirate_group_tick::{self,PirateGroupTickHost},flatmem::{Mem,SparseMem},ships::tick::TickWorld};
use crate::convoy_absorption::AbsorptionState;

pub const MANAGER:u32=0x006dd7a0;
pub const BAND:u32=0x56000000;
pub const PIECES:u32=0x57000000;
pub const SHIPS:u32=0x58000000;
pub const CONVOYS:u32=0x59000000;
pub const CAPTAINS:u32=0x5a000000;
pub const MERCHANTS:u32=0x5b000000;
pub const TOWNS:u32=0x5c000000;
pub const PLAN:u32=0x5d000000;

/// Ventana de 256 bytes desde `0x00673658` del EXE inglés que lee el tick de grupos (`0x00514000`) como tabla de
/// límites de actividad. Es la misma que la sesión copia del ejecutable; se usa cuando no se aporta.
pub const LEVEL_THRESHOLDS: [u8; 256] = [
    4, 8, 16, 18, 0, 0, 0, 75, 5, 8, 10, 12, 10, 16, 30, 24,
    6, 6, 5, 5, 123, 2, 181, 2, 8, 2, 66, 2, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 96, 192, 0, 0, 0, 75, 16, 9, 9, 9,
    16, 9, 9, 9, 9, 9, 9, 11, 11, 9, 11, 8, 8, 9, 9, 8,
    0, 0, 0, 0, 0, 0, 0, 75, 249, 244, 236, 0, 243, 238, 229, 0,
    236, 231, 224, 0, 3, 1, 2, 1, 1, 2, 1, 1, 255, 255, 255, 255,
    254, 255, 255, 255, 0, 232, 125, 71, 0, 0, 112, 65, 0, 128, 59, 69,
    0, 0, 150, 67, 0, 0, 0, 75, 0, 0, 0, 75, 0, 0, 0, 0,
    0, 1, 1, 2, 3, 3, 4, 4, 5, 6, 6, 7, 8, 8, 9, 9,
    10, 11, 11, 12, 12, 13, 13, 14, 15, 15, 16, 16, 17, 17, 18, 18,
    19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 25, 26,
    26, 27, 27, 27, 28, 28, 29, 29, 29, 30, 30, 30, 31, 31, 31, 32,
    32, 0, 0, 0, 16, 135, 17, 0, 19, 0, 0, 0, 11, 0, 0, 0,
    12, 0, 0, 0, 17, 0, 0, 0, 4, 3, 12, 10, 0, 1, 11, 11,
    0, 2, 18, 12, 0, 3, 0, 13, 0, 4, 7, 14, 1, 0, 20, 15,
    1, 2, 13, 16, 1, 3, 4, 17, 1, 4, 23, 18, 2, 0, 12, 19,
];

/// Los valores proceden del reloj, flags de sesión y tabla del EXE verificado.
#[derive(Clone)]
pub struct GroupEnv {
 pub calendar_byte:u8,
 pub suppress_events:u8,
 pub operation_host:u32,
 pub level_thresholds:[u8;256],
 /// Recursos extra para callbacks, como tablas de datos del ejecutable.
 pub extra_memory:Vec<(u32,Vec<u8>)>,
}

/// Sólo publica después del tick completo: flota, banda, cola y host se
/// confirman juntos. El host puede conservar CRT y operaciones en su clon.
/// Los punteros fabricados son internos y nunca se guardan en las imágenes.
pub fn tick<H:PirateGroupTickHost+Clone>(w:&mut TickWorld,state:&mut AbsorptionState,index:usize,env:&GroupEnv,host:&mut H)->Result<(),String>{
 if index>=5||state.bands.len()!=5{return Err("índice o número de cuadrillas inválido".into())}
 state.bands.get(index).and_then(Option::as_ref).ok_or("cuadrilla ausente")?;
 tick_inner(w,state,env,host,Some(index),None)
}

fn tick_inner<H:PirateGroupTickHost+Clone>(w:&mut TickWorld,state:&mut AbsorptionState,env:&GroupEnv,host:&mut H,index:Option<usize>,replenish:Option<(u16,u8)>)->Result<(),String>{
 if state.bands.len()!=5{return Err("número de cuadrillas inválido".into())}
 for old in state.bands.iter().flatten(){old.validate().map_err(str::to_owned)?;if old.pieces.len()>0x100000||old.plan.as_ref().is_some_and(|p|p.len()>0x100000){return Err("región de cuadrilla excesiva".into())}}
 if [w.remove.ships.len(),w.remove.captains.len(),w.remove.convoys.len(),w.remove.merchants.len(),w.remove.towns.len()].into_iter().any(|n|n>0x1000000){return Err("región de flota excesiva".into())}
 if w.remove.manager.len()<0x110{return Err("gestor naval truncado".into())}
 let mut m=SparseMem::default();
 for (base,data) in [(MANAGER,&*w.remove.manager),(SHIPS,&*w.remove.ships),(CAPTAINS,&*w.remove.captains),(CONVOYS,&*w.remove.convoys),(MERCHANTS,&*w.remove.merchants),(TOWNS,&*w.remove.towns)]{m.seed(base,data)}
 m.w32(MANAGER,CAPTAINS);m.w32(MANAGER+4,SHIPS);m.w32(MANAGER+8,CONVOYS);
 for (i,old) in state.bands.iter().enumerate(){
  let b=BAND+i as u32*0x10000;let p=PIECES+i as u32*0x100000;let q=PLAN+i as u32*0x100000;
  if let Some(old)=old{m.seed(b,&[0;24]);m.seed(b+8,&old.header);if !old.pieces.is_empty(){m.seed(p,&old.pieces);m.w32(b+4,p)}if let Some(plan)=&old.plan{m.seed(q,plan);m.w32(b,q)}m.w32(MANAGER+0xc+4*i as u32,b)}else{m.w32(MANAGER+0xc+4*i as u32,0)}
 }
 m.seed(0x006de508,&TOWNS.to_le_bytes());m.seed(0x006de518,&MERCHANTS.to_le_bytes());m.seed(0x006de4b0,&w.remove.town_count.to_le_bytes());m.seed(0x006de4a4,&[env.calendar_byte]);m.seed(0x006de4aa,&w.remove.merchant_count.to_le_bytes());m.seed(0x006de52a,&[env.suppress_events]);m.seed(0x006df31c,&env.operation_host.to_le_bytes());m.seed(0x00673658,&env.level_thresholds);
 for (at,data) in &env.extra_memory {if at.checked_add(u32::try_from(data.len()).map_err(|_|"semilla excesiva")?).is_none()||m.contains_any(*at,data.len()){return Err("semilla solapada o fuera de rango".into())}m.seed(*at,data)}
 let mut staged_host=host.clone();
 let outcome=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
  if let Some(index)=index{return pirate_group_tick::tick(&mut m,&mut staged_host,BAND+index as u32*0x10000,index as u32)}
  let (target,difficulty)=replenish.ok_or("modo de cuadrilla ausente")?;
  for i in 0..5u32 {
   let band=m.r32(MANAGER+0xc+4*i);if band==0||u16::from(m.r8(band+0xe))!=target||m.r32(band)!=0{continue}
   let count=m.r16(MANAGER+0xf4);let base=m.r32(MANAGER+4);let mut ship=m.r16(band+0x12);let mut seen=vec![false;usize::from(count)];let mut missing=i32::from(env.level_thresholds[usize::from(difficulty)]);
   while ship<count {let at=usize::from(ship);if seen[at]{return Err("reserva pirata cíclica".into())}seen[at]=true;missing-=1;ship=m.r16(base+u32::from(ship)*0x180+6)}
   while missing>0 {let kind=env.calendar_byte.wrapping_add(missing as u8)&3;let power=u32::from(m.r8(band+0x14))*40+100;staged_host.create_ship(&mut m,band,u32::from(kind),1,power)?;missing-=1;}
  }
  Ok(())
 }));
 match outcome{Ok(result)=>result?,Err(_)=>return Err("memoria o cadena de cuadrilla fuera de los recursos aportados".into())}
 let ship_count=usize::from(m.r16(MANAGER+0xf4));let cap_count=usize::from(m.r16(MANAGER+0xf2));let convoy_count=usize::from(m.r16(MANAGER+0xf6));
 if ship_count*0x180>w.remove.ships.len()||cap_count*16>w.remove.captains.len()||convoy_count*0x3c>w.remove.convoys.len(){return Err("pool ampliado fuera de la capacidad transaccional".into())}
 let mut manager=m.read(MANAGER,w.remove.manager.len());manager[..0x20].copy_from_slice(&w.remove.manager[..0x20]);
 let ships=m.read(SHIPS,w.remove.ships.len());let captains=m.read(CAPTAINS,w.remove.captains.len());let convoys=m.read(CONVOYS,w.remove.convoys.len());let merchants=m.read(MERCHANTS,w.remove.merchants.len());let towns=m.read(TOWNS,w.remove.towns.len());
 let mut after=state.bands.clone();
 for (i,old) in state.bands.iter().enumerate(){
  let ptr=m.r32(MANAGER+0xc+4*i as u32);let b=BAND+i as u32*0x10000;let Some(old)=old else{if ptr!=0{return Err("alta de cuadrilla no soportada".into())}continue};
  if ptr==0{after[i]=None;continue}if ptr!=b{return Err("reubicación de cuadrilla no soportada".into())}
  let mut band=old.clone();band.header.copy_from_slice(&m.read(b+8,16));
  let capacity=usize::from(u16::from_le_bytes([band.header[0],band.header[1]]));
  fn available(m:&SparseMem,base:u32,len:usize)->bool{base!=0&&base.checked_add(len as u32).is_some()&&m.contains_range(base,len)}
  let piece_ptr=m.r32(b+4);
  band.pieces=if capacity==0{Vec::new()}else{if !available(&m,piece_ptr,capacity*8){return Err("piezas reubicadas fuera de memoria".into())}m.read(piece_ptr,capacity*8)};
  let plan_ptr=m.r32(b);
  band.plan=if plan_ptr==0{None}else{
   if !available(&m,plan_ptr,0x64){return Err("plano reubicado sin cabecera serializable".into())}
   let rows=m.r32(plan_ptr);let columns=m.r32(plan_ptr+4);
   let len=usize::try_from(rows).ok().and_then(|r|usize::try_from(columns).ok().and_then(|c|r.checked_mul(c))).and_then(|n|n.checked_add(0x64)).filter(|n|*n<=0x100000).ok_or("dimensiones de plano excesivas")?;
   if !available(&m,plan_ptr,len){return Err("plano reubicado truncado".into())}Some(m.read(plan_ptr,len))
  };
  band.validate().map_err(str::to_owned)?;after[i]=Some(band);
 }
 w.remove.manager.copy_from_slice(&manager);w.remove.ships.copy_from_slice(&ships);w.remove.captains.copy_from_slice(&captains);w.remove.convoys.copy_from_slice(&convoys);w.remove.merchants.copy_from_slice(&merchants);w.remove.towns.copy_from_slice(&towns);
 state.bands=after;*host=staged_host;Ok(())
}

/// Resultado preparado que puede contener pools ampliados. El integrador
/// confirma los seis buffers, bandas y host juntos al finalizar la fase naval.
#[derive(Clone)]
pub struct Prepared<H>{
 pub manager:Vec<u8>,pub ships:Vec<u8>,pub captains:Vec<u8>,pub convoys:Vec<u8>,
 pub merchants:Vec<u8>,pub towns:Vec<u8>,pub state:AbsorptionState,pub host:H,
}

/// Prepara únicamente el tramo de 0x514530 que crea el plano y coloca las
/// piezas. No crea todavía el convoy ni publica cambios sobre la banda viva.
pub fn prepare_missing_hideout_plan(
    original: &p3_save::pirate_bands::PirateBand,
    plans: &crate::town_plans::TownPlanBank,
    tick: u32,
    piece_bytes: (u8, u8),
) -> Result<p3_save::pirate_bands::PirateBand, String> {
    use p3_core::{buildings::Site, town::Town};
    original.validate().map_err(str::to_owned)?;
    if original.plan.is_some() { return Err("la banda ya tiene plano".into()); }
    let mut band = original.clone();
    let mut town = Town::default();
    let mut site = Site::default();
    plans.reset_and_load(&mut town, &mut site, 0xfe - (band.header[7] & 3))?;
    let rows = town.i32(0x7a8);
    let cols = town.i32(0x7ac);
    if rows <= 0 || cols <= 0 || rows > 256 || cols > 256
        || usize::try_from(rows * cols).ok() != Some(site.cells.len())
        || i32::from(town.u8(0x800)) >= rows || i32::from(town.u8(0x801)) >= cols
    { return Err("plano de guarida inválido o ausente".into()); }
    let count = usize::from(u16::from_le_bytes([band.header[0], band.header[1]]));
    let mut visited = vec![false; count];
    let mut current = 0usize;
    let special = piece_bytes.1;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<(), String> {
        while current < count {
            if visited[current] { return Err("cadena de piezas piratas cíclica".into()); }
            visited[current] = true;
            let at = current * 8;
            band.pieces[at + 6] = 0;
            let mut kind = band.pieces[at + 3];
            if !matches!(kind, 0x24 | 0x30 | 0x31 | 0x1b) {
                return Err("tipo de pieza sin avance en el original".into());
            }
            if kind == 0x24 || kind == 0x30 || kind == 0x31 {
                if kind == 0x24 { band.pieces[at + 7] = 0; }
                let mut row = band.pieces[at];
                let mut col = band.pieces[at + 1];
                let search = if kind == 0x24 { 0x24 } else { 0x30 };
                let found = site.find_nearest_cell_for_type(&town, &mut row, &mut col, search, 0, false, special);
                band.pieces[at] = row;
                band.pieces[at + 1] = col;
                if found {
                    if kind == 0x24 { site.place_building(&mut town, i32::from(row), i32::from(col), 0x24, 0, special); }
                    else { site.mark_2x2_occupied(&mut town, i32::from(row), i32::from(col)); }
                } else if kind != 0x24 {
                    kind = 0x1b;
                    band.pieces[at + 3] = kind;
                }
            }
            let next = u16::from_le_bytes([band.pieces[at + 4], band.pieces[at + 5]]);
            if kind == 0x1b {
                let orientation = if current & 1 != 0 { piece_bytes.1 } else { piece_bytes.0 };
                band.pieces[at + 7] = orientation;
                let mut row = town.u8(0x800).wrapping_sub((tick as u8).wrapping_add(current as u8) & 7).wrapping_add(4);
                let mut col = town.u8(0x801).wrapping_sub(((tick as i32 >> 4) as u8).wrapping_add(current as u8) & 7).wrapping_add(4);
                let found = site.find_nearest_cell_for_type(&town, &mut row, &mut col, 0x1b, orientation, false, special);
                band.pieces[at] = row;
                band.pieces[at + 1] = col;
                if found {
                    site.place_building(&mut town, i32::from(row), i32::from(col), 0x1b, orientation, special);
                } else {
                    let mut previous = 4usize;
                    let mut link = usize::from(u16::from_le_bytes([band.pieces[4], band.pieces[5]]));
                    let mut scanned = vec![false; count];
                    while link < count && link != current {
                        if scanned[link] { return Err("cadena de piezas piratas cíclica".into()); }
                        scanned[link] = true;
                        previous = link * 8 + 4;
                        link = usize::from(u16::from_le_bytes([band.pieces[previous], band.pieces[previous + 1]]));
                    }
                    if link == current {
                        band.pieces[previous..previous + 2].copy_from_slice(&next.to_le_bytes());
                        band.pieces[at + 4..at + 6].copy_from_slice(&band.header[2..4]);
                        band.header[2..4].copy_from_slice(&(current as u16).to_le_bytes());
                    }
                }
            }
            current = usize::from(next);
        }
        Ok(())
    }));
    match result { Ok(result) => result?, Err(_) => return Err("acceso inválido al colocar piezas de guarida".into()) }
    let mut plan = town.0[0x7a8..0x80c].to_vec();
    plan.extend_from_slice(&site.cells);
    band.plan = Some(plan);
    band.validate().map_err(str::to_owned)?;
    Ok(band)
}
/// Variante con buffers propios para permitir los crecimientos de 64 ranuras
/// de barcos y capitanes. No modifica ninguna entrada si falla el tick.
pub fn prepare<H:PirateGroupTickHost+Clone>(w:&TickWorld,state:&AbsorptionState,index:usize,env:&GroupEnv,host:&H)->Result<Prepared<H>,String>{
 if w.remove.manager.len()<0x110{return Err("gestor naval truncado".into())}
 let r16=|at:usize|u16::from_le_bytes([w.remove.manager[at],w.remove.manager[at+1]]);
 let mut out=Prepared{manager:w.remove.manager.to_vec(),ships:w.remove.ships.to_vec(),captains:w.remove.captains.to_vec(),convoys:w.remove.convoys.to_vec(),merchants:w.remove.merchants.to_vec(),towns:w.remove.towns.to_vec(),state:state.clone(),host:host.clone()};
 let old_ship=out.ships.len();let old_cap=out.captains.len();
 for (free,count,unit,buffer) in [(r16(0xe2),r16(0xf4),0x180usize,&mut out.ships),(r16(0xe0),r16(0xf2),16usize,&mut out.captains)]{
  if buffer.len()<usize::from(count)*unit{return Err("pool de cuadrilla truncado".into())}
  if free>=count&&count<=u16::MAX-64{buffer.resize(buffer.len().max((usize::from(count)+64)*unit),0)}
 }
 {
  let mut remove=p3_core::ships::remove::RemoveWorld{manager:&mut out.manager,ships:&mut out.ships,captains:&mut out.captains,convoys:&mut out.convoys,merchants:&mut out.merchants,towns:&mut out.towns,pool_limit:w.remove.pool_limit,merchant_count:w.remove.merchant_count,town_count:w.remove.town_count,ui_a:w.remove.ui_a,ui_b:w.remove.ui_b};
  let mut staged=TickWorld{remove:&mut remove,time:w.time,player:w.player};tick(&mut staged,&mut out.state,index,env,&mut out.host)?;
 }
 let ship_count=usize::from(u16::from_le_bytes([out.manager[0xf4],out.manager[0xf5]]));let cap_count=usize::from(u16::from_le_bytes([out.manager[0xf2],out.manager[0xf3]]));
 out.ships.truncate(old_ship.max(ship_count*0x180));out.captains.truncate(old_cap.max(cap_count*16));Ok(out)
}

/// 0x5144A0 sobre las cinco bandas, en orden; devuelve todas las imágenes
/// preparadas sin modificar el mundo ni el host de entrada. La dificultad y
/// la tabla proceden del estado de juego y del EXE, respectivamente.
pub fn prepare_replenish<H:PirateGroupTickHost+Clone>(w:&TickWorld,state:&AbsorptionState,target:u16,difficulty:u8,env:&GroupEnv,host:&H)->Result<Prepared<H>,String>{
 if w.remove.manager.len()<0x110{return Err("gestor naval truncado".into())}
 let r16=|at:usize|u16::from_le_bytes([w.remove.manager[at],w.remove.manager[at+1]]);
 let mut out=Prepared{manager:w.remove.manager.to_vec(),ships:w.remove.ships.to_vec(),captains:w.remove.captains.to_vec(),convoys:w.remove.convoys.to_vec(),merchants:w.remove.merchants.to_vec(),towns:w.remove.towns.to_vec(),state:state.clone(),host:host.clone()};
 let old_ship=out.ships.len();let old_cap=out.captains.len();let limit=usize::from(env.level_thresholds[usize::from(difficulty)]);let extra=(limit*5+63)/64*64;
 for (count,unit,buffer) in [(r16(0xf4),0x180usize,&mut out.ships),(r16(0xf2),16usize,&mut out.captains)]{
  if buffer.len()<usize::from(count)*unit{return Err("pool de cuadrilla truncado".into())}
  let reserve=(usize::from(count)+extra).min(usize::from(u16::MAX));buffer.resize(buffer.len().max(reserve*unit),0);
 }
 {
  let mut remove=p3_core::ships::remove::RemoveWorld{manager:&mut out.manager,ships:&mut out.ships,captains:&mut out.captains,convoys:&mut out.convoys,merchants:&mut out.merchants,towns:&mut out.towns,pool_limit:w.remove.pool_limit,merchant_count:w.remove.merchant_count,town_count:w.remove.town_count,ui_a:w.remove.ui_a,ui_b:w.remove.ui_b};
  let mut staged=TickWorld{remove:&mut remove,time:w.time,player:w.player};tick_inner(&mut staged,&mut out.state,env,&mut out.host,None,Some((target,difficulty)))?;
 }
 let ship_count=usize::from(u16::from_le_bytes([out.manager[0xf4],out.manager[0xf5]]));let cap_count=usize::from(u16::from_le_bytes([out.manager[0xf2],out.manager[0xf3]]));
 out.ships.truncate(old_ship.max(ship_count*0x180));out.captains.truncate(old_cap.max(cap_count*16));Ok(out)
}

/// 0x514530 cuando la banda ya tiene plano y convoy válido. Transfiere
/// sus reservas a ese convoy y equipa cada barco. Crear un plano, barco o
/// convoy requiere dependencias adicionales y se rechaza explícitamente.
/// El resultado es una copia; las entradas no se modifican ante ningún error.
fn create_reserve_convoy(w:&mut TickWorld,ship:u16)->Result<u16,String>{
 let r16=|v:&[u8],at:usize|u16::from_le_bytes([v[at],v[at+1]]);
 let r32=|v:&[u8],at:usize|u32::from_le_bytes(v[at..at+4].try_into().unwrap());
 let count=r16(w.remove.manager,0xf4);let capacity=r16(w.remove.manager,0xf6);let cv=r16(w.remove.manager,0xe4);
 if ship>=count{return Err("barco inválido".into())}if cv>=capacity{return Err("ampliación del pool de convoyes no implementada".into())}
 let p=usize::from(ship)*0x180;let b=usize::from(cv)*0x3c;
 if p+0x180>w.remove.ships.len()||b+0x3c>w.remove.convoys.len(){return Err("flota truncada".into())}
 let next=r16(w.remove.convoys,b+8);w.remove.manager[0xe4..0xe6].copy_from_slice(&next.to_le_bytes());
 w.remove.convoys[b+8..b+10].copy_from_slice(&u16::MAX.to_le_bytes());w.remove.convoys[b+10..b+12].copy_from_slice(&ship.to_le_bytes());
 w.remove.convoys[b]=w.remove.ships[p];w.remove.convoys[b+1]=w.remove.ships[p+0xe];
 w.remove.convoys[b+0x1c..b+0x1e].copy_from_slice(&u16::from(w.remove.ships[p+0x39]).to_le_bytes());
 w.remove.convoys[b+0x1e]=w.remove.ships[p+0x37];w.remove.convoys[b+0x1f]=w.remove.ships[p+0x38];
 let capacity_value=r32(w.remove.ships,p+0x10);w.remove.convoys[b+4..b+8].copy_from_slice(&capacity_value.to_le_bytes());
 let flags=r32(w.remove.convoys,b+0x18)|0x400;w.remove.convoys[b+0x18..b+0x1c].copy_from_slice(&flags.to_le_bytes());
 w.remove.ships[p+8..p+10].copy_from_slice(&cv.to_le_bytes());
 w.remove.convoys[b+0x10..b+0x12].copy_from_slice(&ship.to_le_bytes());w.remove.convoys[b+0x14..b+0x16].copy_from_slice(&2u16.to_le_bytes());
 p3_core::ships::convoy::refresh_from_ships(w,cv).map_err(|e|format!("inicialización de convoy: {e:?}"))?;
 w.remove.convoys[b+0x20..b+0x24].fill(0);w.remove.convoys[b+0x24..b+0x38].fill(255);
 w.remove.convoys[b+0x38..b+0x3a].copy_from_slice(&u16::MAX.to_le_bytes());w.remove.convoys[b+2..b+4].copy_from_slice(&u16::MAX.to_le_bytes());
 let flags=r32(w.remove.convoys,b+0x18)|0x4401;w.remove.convoys[b+0x18..b+0x1c].copy_from_slice(&flags.to_le_bytes());
 w.remove.convoys[b+0x16..b+0x18].copy_from_slice(&0x100u16.to_le_bytes());let state=r16(w.remove.ships,p+0x134);w.remove.convoys[b+0x12..b+0x14].copy_from_slice(&state.to_le_bytes());
 Ok(cv)
}

pub fn prepare_hideout_existing<H:Clone>(w:&TickWorld,state:&AbsorptionState,index:usize,alderman_town:u8,host:&H)->Result<Prepared<H>,String>{
 prepare_hideout_with_ports(w,state,index,alderman_town,host,None)
}

/// Rama de 0x514530 sin convoy ni barco de reserva. El original crea un
/// barco con capitán, crea un convoy y después devuelve el barco a la reserva
/// con un cuarto de casco. Todas las imágenes y el host se preparan en copias.
/// La publicación de los pools ampliados y de sus rutas corresponde al llamante.
pub fn prepare_hideout_new_reserve<H:PirateGroupTickHost+Clone>(
 w:&TickWorld,state:&AbsorptionState,index:usize,alderman_town:u8,host:&H,
 locations:&[p3_core::ai::settlement::SettlementLocation;40],
)->Result<Prepared<H>,String>{
 if state.bands.len()!=5||index>=5||w.remove.manager.len()<0x110{return Err("mundo de guarida incompleto".into())}
 let band=state.bands[index].as_ref().ok_or("cuadrilla ausente")?;
 band.validate().map_err(str::to_owned)?;
 if band.plan.is_none(){return Err("creación de plano de guarida pendiente".into())}
 let r16=|v:&[u8],p:usize|u16::from_le_bytes([v[p],v[p+1]]);
 let ship_count=r16(w.remove.manager,0xf4);let cap_count=r16(w.remove.manager,0xf2);let cv_count=r16(w.remove.manager,0xf6);
 if r16(&band.header,8)<cv_count||r16(&band.header,10)<ship_count{return Err("la banda ya tiene convoy o reserva".into())}
 if w.remove.ships.len()!=usize::from(ship_count)*0x180||w.remove.captains.len()!=usize::from(cap_count)*16||w.remove.convoys.len()!=usize::from(cv_count)*0x3c{return Err("pools piratas desincronizados".into())}
 let mut mem=SparseMem::default();
 for (base,data) in [(MANAGER,&*w.remove.manager),(SHIPS,&*w.remove.ships),(CAPTAINS,&*w.remove.captains),(CONVOYS,&*w.remove.convoys)]{mem.seed(base,data)}
 mem.seed(BAND,&[0;24]);mem.seed(BAND+8,&band.header);
 mem.w32(MANAGER,CAPTAINS);mem.w32(MANAGER+4,SHIPS);mem.w32(MANAGER+8,CONVOYS);
 let mut next_host=host.clone();
 let power=u32::from(band.header[12])*40+100;
 let outcome=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||next_host.create_ship(&mut mem,BAND,0,1,power)));
 let address=match outcome{Ok(result)=>result?,Err(_)=>return Err("alta de reserva fuera de memoria".into())};
 let offset=address.checked_sub(SHIPS).filter(|n|n%0x180==0).ok_or("barco creado fuera del pool")?;
 let id=u16::try_from(offset/0x180).map_err(|_|"índice de barco excesivo")?;
 let new_ship_count=mem.r16(MANAGER+0xf4);let new_cap_count=mem.r16(MANAGER+0xf2);
 if id>=new_ship_count||mem.r16(BAND+0x12)!=id{return Err("alta de reserva inconsistente".into())}
 let mut manager=mem.read(MANAGER,w.remove.manager.len());manager[..0x20].copy_from_slice(&w.remove.manager[..0x20]);
 let mut ships=mem.read(SHIPS,usize::from(new_ship_count)*0x180);
 let mut captains=mem.read(CAPTAINS,usize::from(new_cap_count)*16);
 let mut convoys=w.remove.convoys.to_vec();let mut merchants=w.remove.merchants.to_vec();let mut towns=w.remove.towns.to_vec();
 let mut staged_state=state.clone();staged_state.bands[index].as_mut().unwrap().header.copy_from_slice(&mem.read(BAND+8,16));
 let mut out={
  let mut remove=p3_core::ships::remove::RemoveWorld{manager:&mut manager,ships:&mut ships,captains:&mut captains,convoys:&mut convoys,merchants:&mut merchants,towns:&mut towns,pool_limit:w.remove.pool_limit,merchant_count:w.remove.merchant_count,town_count:w.remove.town_count,ui_a:w.remove.ui_a,ui_b:w.remove.ui_b};
  let staged=TickWorld{remove:&mut remove,time:w.time,player:w.player};
  prepare_hideout_with_ports(&staged,&staged_state,index,alderman_town,&next_host,Some(locations))?
 };
 let convoy=r16(&out.state.bands[index].as_ref().unwrap().header,8);
 let ship=usize::from(id)*0x180;let cv=usize::from(convoy)*0x3c;
 let health=i32::from_le_bytes(out.ships[ship+0x14..ship+0x18].try_into().unwrap())>>2;
 out.ships[ship+0x18..ship+0x1c].copy_from_slice(&health.to_le_bytes());
 out.ships[ship+8..ship+10].fill(255);out.convoys[cv+0xa..cv+0xc].fill(255);
 out.state.bands[index].as_mut().unwrap().header[10..12].copy_from_slice(&id.to_le_bytes());
 Ok(out)
}

/// Admite crear el convoy a partir de una reserva existente. Si se agotan las
/// ranuras, prepara el crecimiento del pool en la misma copia transaccional.
/// Crear un barco cuando no hay reserva sigue siendo una operación pendiente.
pub fn prepare_hideout_with_ports<H:Clone>(w:&TickWorld,state:&AbsorptionState,index:usize,alderman_town:u8,host:&H,locations:Option<&[p3_core::ai::settlement::SettlementLocation;40]>)->Result<Prepared<H>,String>{
 if state.bands.len()!=5||index>=5||w.remove.manager.len()<0x110{return Err("mundo de guarida incompleto".into())}
 let r16=|b:&[u8],at:usize|u16::from_le_bytes([b[at],b[at+1]]);
 let ship_count=usize::from(r16(w.remove.manager,0xf4));let convoy_count=usize::from(r16(w.remove.manager,0xf6));
 if w.remove.ships.len()!=ship_count*0x180||w.remove.convoys.len()!=convoy_count*0x3c{return Err("flota de guarida desincronizada".into())}
 let band=state.bands[index].as_ref().ok_or("cuadrilla ausente")?;band.validate().map_err(str::to_owned)?;
 if band.plan.is_none(){return Err("creación de plano de guarida pendiente".into())}
 let mut cv=usize::from(r16(&band.header,8));
 let mut out=Prepared{manager:w.remove.manager.to_vec(),ships:w.remove.ships.to_vec(),captains:w.remove.captains.to_vec(),convoys:w.remove.convoys.to_vec(),merchants:w.remove.merchants.to_vec(),towns:w.remove.towns.to_vec(),state:state.clone(),host:host.clone()};
 if cv>=convoy_count {
  let port=locations.and_then(|v|v.get(usize::from(band.header[6]))).ok_or("puerto para convoy pirata ausente")?;
  let ship=r16(&band.header,10);if usize::from(ship)>=ship_count{return Err("creación de barco pirata pendiente".into())}
  let free=usize::from(r16(&out.manager,0xe4));
  if free>=convoy_count {
   if free!=convoy_count{return Err("cabeza libre de convoy fuera del pool".into())}
   let grown=convoy_count.checked_add(32).filter(|n|*n<=usize::from(u16::MAX)).ok_or("desbordamiento de pool de convoyes")?;
   out.convoys.resize(grown*0x3c,0);
   out.manager[0xf6..0xf8].copy_from_slice(&(grown as u16).to_le_bytes());
   for id in convoy_count..grown {
    let offset=id*0x3c;
    out.convoys[offset+8..offset+10].copy_from_slice(&((id+1) as u16).to_le_bytes());
    out.convoys[offset+0x12..offset+0x14].fill(0xff);
   }
  }
  let at=usize::from(ship)*0x180;let next=r16(&out.ships,at+6);
  out.ships[at+6..at+8].copy_from_slice(&u16::MAX.to_le_bytes());
  out.ships[at+0x1e..at+0x20].copy_from_slice(&(port.x as u16).to_le_bytes());
  out.ships[at+0x22..at+0x24].copy_from_slice(&(port.y as u16).to_le_bytes());
  let new_cv={let mut remove=p3_core::ships::remove::RemoveWorld{manager:&mut out.manager,ships:&mut out.ships,captains:&mut out.captains,convoys:&mut out.convoys,merchants:&mut out.merchants,towns:&mut out.towns,pool_limit:w.remove.pool_limit,merchant_count:w.remove.merchant_count,town_count:w.remove.town_count,ui_a:w.remove.ui_a,ui_b:w.remove.ui_b};
   let mut staged=TickWorld{remove:&mut remove,time:w.time,player:w.player};
   create_reserve_convoy(&mut staged,ship)?};
  cv=usize::from(new_cv);
  let band=out.state.bands[index].as_mut().expect("banda validada");
  band.header[8..10].copy_from_slice(&new_cv.to_le_bytes());
  band.header[10..12].copy_from_slice(&next.to_le_bytes());
 }
 let mut current=r16(&out.state.bands[index].as_ref().expect("banda validada").header,10);if usize::from(current)>=ship_count{return Ok(out)}
 let town=usize::from(alderman_town);if town>=w.remove.town_count as usize{return Err("ciudad del Ältermann ausente".into())}let offset=town.checked_mul(0x9f8).and_then(|v|v.checked_add(0x810)).ok_or("ciudad excesiva")?;let bytes=w.remove.towns.get(offset..offset+4).ok_or("ciudad del Ältermann truncada")?;let level=p3_core::economy::pirate_group_callbacks::town_weapon_level(u32::from_le_bytes(bytes.try_into().unwrap()));
 let mut seen=vec![false;ship_count];let convoy_offset=cv*0x3c;let band_out=out.state.bands[index].as_mut().expect("banda validada");
 while usize::from(current)<ship_count{
  let i=usize::from(current);if seen[i]{return Err("reserva pirata cíclica".into())}seen[i]=true;let at=i*0x180;let next=r16(&out.ships,at+6);band_out.header[10..12].copy_from_slice(&next.to_le_bytes());
  let head=r16(&out.convoys,convoy_offset+0x0a);out.ships[at+6..at+8].copy_from_slice(&head.to_le_bytes());out.convoys[convoy_offset+0x0a..convoy_offset+0x0c].copy_from_slice(&current.to_le_bytes());out.ships[at+8..at+10].copy_from_slice(&(cv as u16).to_le_bytes());
  let kind=u32::from(out.ships[at+0xe]);let power=(u32::from(band_out.header[12])+1)*kind*20+0x32;p3_core::economy::pirate_group_callbacks::arm_ship(&mut out.ships[at..at+0x180],power,level)?;current=next;
 }
 Ok(out)
}

#[cfg(test)]
mod tests {
 use super::*;
 use p3_core::ships::remove::RemoveWorld;
 #[derive(Clone,Default)] struct Host{calls:usize}
 impl PirateGroupTickHost for Host{
  fn remove_ship(&mut self,_:&mut dyn Mem,_:u16)->Result<(),String>{self.calls+=1;Err("remove test".into())}
  fn dissolve(&mut self,_:&mut dyn Mem,_:u32)->Result<(),String>{Err("dissolve test".into())}
  fn destroy_band(&mut self,_:&mut dyn Mem,_:u32)->Result<(),String>{Err("destroy test".into())}
  fn create_ship(&mut self,_:&mut dyn Mem,_:u32,_:u32,_:u32,_:u32)->Result<u32,String>{self.calls+=1;Err("create test".into())}
  fn upgrade(&mut self,_:&mut dyn Mem,_:u32,_:u32)->Result<(),String>{Err("upgrade test".into())}
  fn crew(&mut self,_:&mut dyn Mem,_:u32)->Result<u16,String>{Err("crew test".into())}
  fn create_captain(&mut self,_:&mut dyn Mem)->Result<u16,String>{Err("captain test".into())}
  fn enqueue(&mut self,_:&mut dyn Mem,_:[u32;5])->Result<(),String>{Err("enqueue test".into())}
 }
 fn env()->GroupEnv{GroupEnv{calendar_byte:0,suppress_events:0,operation_host:0,level_thresholds:[0;256],extra_memory:vec![]}}
 #[test] fn active_convoy_updates_ship_and_preserves_other_bands(){
  let mut manager=vec![0;0x110];manager[0xf4..0xf6].copy_from_slice(&1u16.to_le_bytes());manager[0xf6..0xf8].copy_from_slice(&1u16.to_le_bytes());let before=manager.clone();
  let mut ships=vec![0;0x180];ships[4..6].copy_from_slice(&u16::MAX.to_le_bytes());ships[0x42..0x44].copy_from_slice(&u16::MAX.to_le_bytes());ships[0x158]=99;
  let mut caps=vec![];let mut cvs=vec![0;0x3c];let mut merchants=vec![];let mut towns=vec![];
  let mut rem=RemoveWorld{manager:&mut manager,ships:&mut ships,captains:&mut caps,convoys:&mut cvs,merchants:&mut merchants,towns:&mut towns,pool_limit:0,merchant_count:1,town_count:0,ui_a:false,ui_b:false};let mut w=TickWorld{remove:&mut rem,time:0,player:0};
  let mut header=[0;16];header[4..6].copy_from_slice(&0u16.to_le_bytes());header[8..10].copy_from_slice(&0u16.to_le_bytes());let band=p3_save::pirate_bands::PirateBand{header,plan:None,pieces:vec![]};
  let mut state=AbsorptionState{bands:vec![None,Some(band.clone()),None,Some(band.clone()),None],day:1,network_client:0,operations:vec![]};let mut host=Host::default();
  tick(&mut w,&mut state,1,&env(),&mut host).unwrap();assert_eq!(w.remove.ships[0x158],1);assert_eq!(state.bands[3],Some(band));assert_eq!(w.remove.manager,&before);assert_eq!(host.calls,0);
 }
 #[test] fn active_convoy_bridge_matches_original_x86(){
  use std::io::Write;
  let (Ok(exe),Ok(python))=(std::env::var("P3_EXE"),std::env::var("P3_ORACLE_PY")) else{return};
  let mut manager=vec![0;0x110];manager[0xf4..0xf6].copy_from_slice(&1u16.to_le_bytes());manager[0xf6..0xf8].copy_from_slice(&1u16.to_le_bytes());
  let mut ships=vec![0;0x180];ships[4..6].copy_from_slice(&u16::MAX.to_le_bytes());ships[0x42..0x44].copy_from_slice(&u16::MAX.to_le_bytes());ships[0x158]=99;
  let mut header=[0;16];header[..2].copy_from_slice(&1u16.to_le_bytes());let pieces=vec![255;8];let band=p3_save::pirate_bands::PirateBand{header,plan:None,pieces:pieces.clone()};
  let mut raw_band=vec![0u8;0x20];raw_band[8..24].copy_from_slice(&header);let convoy=vec![0u8;0x3c];
  let hex=|data:&[u8]|data.iter().map(|b|format!("{b:02x}")).collect::<String>();
  let case=serde_json::json!({"band":hex(&raw_band),"ships":hex(&ships),"pieces":hex(&pieces),"convoys":hex(&convoy),"index":1,"globals":[[0x6dd892u32,0,2],[0x6dd894u32,1,2],[0x6dd896u32,1,2],[0x6dd88cu32,0,2],[0x6de4aau32,1,2]]});
  let script=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_pirate_group_tick.py");let mut child=std::process::Command::new(python).arg(script).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"binary":exe,"cases":[case]}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let expected:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();
  let mut caps=vec![];let mut cvs=convoy;let mut merchants=vec![];let mut towns=vec![];let mut rem=RemoveWorld{manager:&mut manager,ships:&mut ships,captains:&mut caps,convoys:&mut cvs,merchants:&mut merchants,towns:&mut towns,pool_limit:0,merchant_count:1,town_count:0,ui_a:false,ui_b:false};let mut w=TickWorld{remove:&mut rem,time:0,player:0};let mut state=AbsorptionState{bands:vec![None,Some(band),None,None,None],day:1,network_client:0,operations:vec![]};let mut host=Host::default();
  tick(&mut w,&mut state,1,&env(),&mut host).unwrap();assert_eq!(hex(w.remove.ships),expected[0]["ships"]);assert_eq!(hex(&state.bands[1].as_ref().unwrap().header),&expected[0]["band"].as_str().unwrap()[16..48]);assert_eq!(hex(&state.bands[1].as_ref().unwrap().pieces),expected[0]["pieces"]);assert_eq!(expected[0]["events"],serde_json::json!([]));
 }
 #[derive(Clone,Default)] struct GrowHost;
 impl PirateGroupTickHost for GrowHost{
  fn remove_ship(&mut self,_:&mut dyn Mem,_:u16)->Result<(),String>{Err("remove".into())}
  fn dissolve(&mut self,_:&mut dyn Mem,_:u32)->Result<(),String>{Err("dissolve".into())}
  fn destroy_band(&mut self,_:&mut dyn Mem,_:u32)->Result<(),String>{Err("destroy".into())}
  fn create_ship(&mut self,m:&mut dyn Mem,band:u32,_:u32,_:u32,_:u32)->Result<u32,String>{
   m.w16(MANAGER+0xf4,64);m.w16(MANAGER+0xe2,1);m.w16(band+0x12,0);m.w16(SHIPS+6,u16::MAX);m.w16(SHIPS+0x42,u16::MAX);m.w32(SHIPS+0x14,10000);
   let pieces=0x5e000000;for i in 0..8{m.w8_seed(pieces+i);m.w8(pieces+i,0xff)}m.w32(band+4,pieces);m.w16(band+8,1);
   let plan=0x5f000000;for i in 0..0x65{m.w8_seed(plan+i)}m.w32(plan,1);m.w32(plan+4,1);m.w8(plan+0x64,42);m.w32(band,plan);Ok(SHIPS)
  }
  fn upgrade(&mut self,_:&mut dyn Mem,_:u32,_:u32)->Result<(),String>{Ok(())}
  fn crew(&mut self,_:&mut dyn Mem,_:u32)->Result<u16,String>{Ok(20)}
  fn create_captain(&mut self,m:&mut dyn Mem)->Result<u16,String>{m.w16(MANAGER+0xf2,64);m.w16(MANAGER+0xe0,1);m.w8(CAPTAINS+8,42);Ok(0)}
  fn enqueue(&mut self,_:&mut dyn Mem,_:[u32;5])->Result<(),String>{Ok(())}
 }
 #[test] fn prepared_growth_and_relocated_band_are_owned_until_commit(){
  let mut manager=vec![0;0x110];let before=manager.clone();let mut ships=vec![];let mut caps=vec![];let mut cvs=vec![];let mut merchants=vec![];let mut towns=vec![];
  let mut rem=RemoveWorld{manager:&mut manager,ships:&mut ships,captains:&mut caps,convoys:&mut cvs,merchants:&mut merchants,towns:&mut towns,pool_limit:0,merchant_count:1,town_count:0,ui_a:false,ui_b:false};let w=TickWorld{remove:&mut rem,time:0,player:0};
  let mut header=[0u8;16];header[4..6].fill(255);header[8..12].fill(255);let band=p3_save::pirate_bands::PirateBand{header,plan:None,pieces:vec![]};let state=AbsorptionState{bands:vec![Some(band),None,None,None,None],day:1,network_client:0,operations:vec![]};
  let result=prepare(&w,&state,0,&env(),&GrowHost).unwrap();assert_eq!(w.remove.manager,&before);assert!(w.remove.ships.is_empty());assert!(w.remove.captains.is_empty());assert_eq!(result.ships.len(),64*0x180);assert_eq!(result.captains.len(),64*16);assert_eq!(result.captains[8],42);assert_eq!(result.state.bands[0].as_ref().unwrap().pieces,vec![255;8]);assert_eq!(result.state.bands[0].as_ref().unwrap().plan.as_ref().unwrap()[0x64],42);
 }
 #[test] fn failing_callback_rolls_back_all(){
  let mut manager=vec![0;0x110];manager[0xf4..0xf6].copy_from_slice(&1u16.to_le_bytes());let before=manager.clone();let mut ships=vec![0;0x180];ships[4..6].copy_from_slice(&u16::MAX.to_le_bytes());ships[0x42..0x44].copy_from_slice(&u16::MAX.to_le_bytes());ships[0x158]=99;let ship_before=ships.clone();
  let mut caps=vec![];let mut cvs=vec![];let mut merchants=vec![];let mut towns=vec![];let mut rem=RemoveWorld{manager:&mut manager,ships:&mut ships,captains:&mut caps,convoys:&mut cvs,merchants:&mut merchants,towns:&mut towns,pool_limit:0,merchant_count:1,town_count:0,ui_a:false,ui_b:false};let mut w=TickWorld{remove:&mut rem,time:0,player:0};
  let mut header=[0;16];header[4..6].copy_from_slice(&0u16.to_le_bytes());header[8..12].fill(255);let band=p3_save::pirate_bands::PirateBand{header,plan:None,pieces:vec![]};let mut state=AbsorptionState{bands:vec![Some(band),None,None,None,None],day:1,network_client:0,operations:vec![]};let original=state.bands.clone();let mut host=Host::default();
  assert_eq!(tick(&mut w,&mut state,0,&env(),&mut host).unwrap_err(),"create test");assert_eq!(w.remove.manager,&before);assert_eq!(w.remove.ships,&ship_before);assert_eq!(state.bands,original);assert_eq!(host.calls,0);
  // Un índice válido según el contador, pero fuera del slice físico, debe
  // devolverse como error transaccional, no desbordar ni propagar panic.
  w.remove.manager[0xf4]=2;state.bands[0].as_mut().unwrap().header[4]=1;let before=w.remove.manager.to_vec();let bad=state.bands.clone();
  assert!(tick(&mut w,&mut state,0,&env(),&mut host).unwrap_err().contains("memoria"));assert_eq!(w.remove.manager,&before);assert_eq!(w.remove.ships,&ship_before);assert_eq!(state.bands,bad);assert_eq!(host.calls,0);
 }
}
