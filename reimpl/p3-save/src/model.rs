//! Modelo editable de la partida (S05): los bloques del `.pat` y el `.pst` descompuestos en registros propios y escritos de nuevo.
//!
//! El modelo parte de los lectores de S02/S04 (`world`, `ships`, `tail`, `world_rest`, `statistics`) y escribe en el mismo orden que los serializadores
//! del juego. Los contadores que el original deriva del contenido (ciudades, negocios, oficinas, comerciantes, barcos, capitanes y convoyes) se recalculan
//! al escribir, de modo que añadir o quitar registros no deja la cabecera incoherente. El resto de campos se conserva por posición, como en los lectores;
//! cambiar el tamaño de un registro que arrastra listas (comerciante, barco con ruta) exige editar también el contador interno de esa lista.
//! Lo que el modelo no demuestra: que el deserializador original acepte un archivo editado (ver `research/systems/borde-motor/save-blocks/README.md`).

use crate::ships::{self, Ships};
use crate::statistics::{self, Statistics};
use crate::world::{self, World};
use crate::{Save, BLOCK_NAMES};

const SEP: u8 = 0x7F;

/// Imagen serializada de un asedio recién construido para la ciudad indicada
/// (constructor 0x006334B0 y escritor 0x006335A0).
pub fn new_town_siege(city:u8)->[u8;20]{
 let mut record=[0u8;20];record[4]=city;record[5..7].copy_from_slice(&0x3fffu16.to_le_bytes());record[7]=2;for i in [9,11,13,15]{record[i]=0xff}record
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModelError {
    Container(crate::SaveError),
    World(world::WorldError),
    Ships(ships::ShipsError),
    Statistics(statistics::StatisticsError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TownModel {
    pub economy: Vec<u8>,
    pub tavern: Vec<u8>,
    pub buildings: Vec<u8>,
    pub shipyard: Vec<u8>,
    pub council: Vec<u8>,
    /// Instalaciones, resto fijo y nodos de suministro (hasta el separador final, que se reescribe).
    pub facilities_and_tail: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldModel {
    /// Cabecera de `0xC7` bytes; los contadores `+6`, `+0xA`, `+0xC` y `+0xE` se recalculan.
    pub header: Vec<u8>,
    pub towns: Vec<TownModel>,
    pub matrix: Vec<u8>,
    /// Los cuatro bytes `F1 F2 F3 F4` entre la matriz y los asedios.
    pub marker: [u8; 4],
    pub sieges: Vec<Vec<u8>>,
    pub businesses: Vec<Vec<u8>>,
    pub offices: Vec<Vec<u8>>,
    pub merchants: Vec<Vec<u8>>,
    pub rest: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShipsModel {
    /// Cabecera de `0x22` bytes; los contadores `+0x12`, `+0x14` y `+0x16` se recalculan.
    pub header: Vec<u8>,
    pub captains: Vec<Vec<u8>>,
    pub ships: Vec<Vec<u8>>,
    pub convoys: Vec<Vec<u8>>,
    pub tail: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatisticsModel {
    pub header: [u16; 6],
    pub chains: [Option<Vec<(u32, Vec<u8>)>>; statistics::SERIES_CHAINS],
    pub items: Vec<Vec<u8>>,
    pub slack: u8,
}

/// Partida completa: el resto de bloques pequeños se guardan como bytes y se validan con sus lectores al escribir.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveModel {
    pub header: [u8; crate::HEADER_LEN],
    pub world: WorldModel,
    pub ships: ShipsModel,
    pub message_pool: Vec<u8>,
    pub route_stop_pool: Vec<u8>,
    pub scheduled_tasks: Vec<u8>,
    pub sea_battles: Vec<u8>,
    pub name_bank: Vec<u8>,
}

/// Escribe los campos de `Town` guardados por S02 y los bloques del plano y registros.
/// Rechaza cambios en campos no guardados para no perder estado silenciosamente.
impl TownModel {
    /// Crea las secciones serializadas de una ciudad viva, con sus nodos de suministro.
    /// Los dos bytes reservados de cada nodo nuevo se inicializan a cero, como su constructor.
    pub fn from_live(town:&p3_core::town::Town,site:&p3_core::buildings::Site,entries:&[p3_core::economy::world::TickEntry])->Result<Self,String>{
        let count=u16::try_from(entries.len()).map_err(|_|"demasiadas entradas de suministro")?;
        let tail=world::FACILITIES_LEN.checked_add(world::TAIL_FIXED).and_then(|n|entries.len().checked_mul(world::NODE_LEN).and_then(|m|n.checked_add(m))).ok_or("tamaño de ciudad inválido")?;
        let mut model=Self{economy:vec![0;world::ECONOMY_LEN],tavern:vec![0;world::TAVERN_LEN],buildings:Vec::new(),shipyard:vec![0;world::SHIPYARD_LEN],council:vec![0;world::COUNCIL_LEN],facilities_and_tail:vec![0;tail]};
        let at=world::FACILITIES_LEN+0x53;model.facilities_and_tail[at..at+2].copy_from_slice(&count.to_le_bytes());model.apply_serialized_town(town,town,site)?;model.update_tick_entries(entries)?;Ok(model)
    }

    /// Actualiza las entradas de suministro existentes conservando sus bytes reservados.
    pub fn update_tick_entries(&mut self, entries:&[p3_core::economy::world::TickEntry])->Result<(),String>{
        let tail=self.facilities_and_tail.get_mut(world::FACILITIES_LEN..).ok_or("cola de ciudad truncada")?;
        let header=tail.get(0x53..0x55).ok_or("contador de suministros truncado")?;
        let count=u16::from_le_bytes([header[0],header[1]]) as usize;
        if count!=entries.len(){return Err("cantidad de entradas de suministro distinta".into())}
        let end=0x55usize.checked_add(count.checked_mul(16).ok_or("cantidad de suministros inválida")?).ok_or("cantidad de suministros inválida")?;
        let records=tail.get_mut(0x55..end).ok_or("entradas de suministro truncadas")?;
        for (record,entry) in records.chunks_exact_mut(16).zip(entries){
            record[0..4].copy_from_slice(&entry.amount.to_le_bytes());
            record[4..8].copy_from_slice(&entry.sum.to_le_bytes());
            record[8..10].copy_from_slice(&entry.max.to_be_bytes());
            record[10..12].copy_from_slice(&entry.factor.to_be_bytes());
            record[12]=entry.facility_type;record[13]=entry.town;
        }
        Ok(())
    }
    /// Copia únicamente los campos serializados de la ciudad y el plano suministrado.
    pub fn apply_serialized_town(&mut self,before:&p3_core::town::Town,current:&p3_core::town::Town,site:&p3_core::buildings::Site)->Result<(),String>{
        let mut after=before.clone();let mut copy=|at:usize,n:usize|after.0[at..at+n].copy_from_slice(&current.0[at..at+n]);
        copy(0,world::ECONOMY_LEN);copy(0x6d4,world::COUNCIL_LEN);copy(0x82c,world::TAVERN_LEN);copy(0x840,world::FACILITIES_LEN);copy(0x794,0x10);copy(0x7a8,0x64);copy(0x998,8);
        for &(_,at,n) in world::SHIPYARD_FIELDS.iter().chain(world::BUILDING_FIELDS).chain(world::TAIL_FIELDS){copy(at,n);}
        self.apply_town(before,&after,site)
    }
    pub fn apply_town(&mut self,before:&p3_core::town::Town,after:&p3_core::town::Town,
                      site:&p3_core::buildings::Site)->Result<(),String>{
        use crate::world::{SHIPYARD_FIELDS,BUILDING_FIELDS,TAIL_FIELDS};
        let mut covered=[false;p3_core::town::TOWN_SIZE];
        let mut cover=|at:usize,len:usize|covered[at..at+len].fill(true);
        cover(0,world::ECONOMY_LEN);cover(0x6d4,world::COUNCIL_LEN);cover(0x82c,world::TAVERN_LEN);
        cover(0x840,world::FACILITIES_LEN);cover(0x794,0x10);cover(0x7a8,0x64);
        for &(_,at,n) in SHIPYARD_FIELDS.iter().chain(BUILDING_FIELDS).chain(TAIL_FIELDS){cover(at,n);}
        cover(0x998,8);
        for i in 0..covered.len(){if !covered[i]&&before.0[i]!=after.0[i]{return Err(format!("campo Town+0x{i:X} no guardado"));}}
        let count=after.u16(0x776) as usize;
        if site.records.len()!=count*8{return Err("longitud de registros de edificios".into());}
        let cells=(after.u32(0x7a8) as usize).checked_mul(after.u32(0x7ac) as usize).ok_or("dimensiones del plano")?;
        if site.cells.len()!=cells{return Err("longitud de celdas del plano".into());}
        if self.economy.len()!=world::ECONOMY_LEN || self.council.len()!=world::COUNCIL_LEN ||
           self.tavern.len()!=world::TAVERN_LEN || self.shipyard.len()!=world::SHIPYARD_LEN ||
           self.facilities_and_tail.len()<world::FACILITIES_LEN+world::TAIL_FIXED {return Err("sección de ciudad truncada".into());}
        self.economy.copy_from_slice(&after.0[..world::ECONOMY_LEN]);
        self.council.copy_from_slice(&after.0[0x6d4..0x6d4+world::COUNCIL_LEN]);
        self.tavern.copy_from_slice(&after.0[0x82c..0x82c+world::TAVERN_LEN]);
        for &(pos,at,n) in SHIPYARD_FIELDS{self.shipyard[pos..pos+n].copy_from_slice(&after.0[at..at+n]);}
        self.facilities_and_tail[..world::FACILITIES_LEN].copy_from_slice(&after.0[0x840..0x840+world::FACILITIES_LEN]);
        let tail=&mut self.facilities_and_tail[world::FACILITIES_LEN..];
        for &(pos,at,n) in TAIL_FIELDS{tail[pos..pos+n].copy_from_slice(&after.0[at..at+n]);}
        for k in 0..4 {tail[0x18+2*k]=after.0[0x998+k];tail[0x19+2*k]=after.0[0x99c+k];}
        self.buildings=site.serialize_building_data(after);Ok(())
    }
}

fn bytes(d: &[u8], r: &std::ops::Range<usize>) -> Vec<u8> {
    d[r.clone()].to_vec()
}

fn chunks(d: &[u8], r: &std::ops::Range<usize>, n: usize) -> Vec<Vec<u8>> {
    d[r.clone()].chunks(n).map(<[u8]>::to_vec).collect()
}

fn put_u16(d: &mut [u8], at: usize, v: usize) {
    d[at..at + 2].copy_from_slice(&u16::try_from(v).expect("contador de 16 bits").to_le_bytes());
}

impl WorldModel {
    /// Añade una ciudad con la matriz y el registro de asedio ya calculados.
    /// La tabla anterior debe conservarse dentro de la nueva matriz; la fila y
    /// columna nuevas han de coincidir y la diagonal debe ser cero.
    pub fn append_town(&mut self,town:TownModel,map_id:u8,plan_id:u8,matrix:Vec<u8>,siege:Vec<u8>)->Result<(),String>{
        let n=self.towns.len();if n>=40||usize::from(map_id)>=40||self.header.len()!=world::HEADER_LEN{return Err("índice de ciudad o cabecera inválidos".into())}if town.economy.get(0x2c0).copied()!=Some(n as u8){return Err("índice de ciudad incoherente".into())}if self.matrix.len()!=n*n||matrix.len()!=(n+1)*(n+1){return Err("tamaño de matriz inválido".into())}if self.sieges.len()!=n||siege.len()!=20||siege[4]!=n as u8{return Err("registro de asedio inválido".into())}
        for row in 0..n{if self.matrix[row*n..(row+1)*n]!=matrix[row*(n+1)..row*(n+1)+n]{return Err("la matriz anterior cambió".into())}if matrix[row*(n+1)+n]!=matrix[n*(n+1)+row]{return Err("distancias nuevas asimétricas".into())}}if matrix[n*(n+1)+n]!=0{return Err("diagonal de ciudad no nula".into())}
        self.header[0x20+n]=map_id;self.header[0x48+n]=plan_id;self.header[6..10].copy_from_slice(&((n+1)as u32).to_le_bytes());self.towns.push(town);self.matrix=matrix;self.sieges.push(siege);Ok(())
    }

    pub fn parse(d: &[u8]) -> Result<WorldModel, world::WorldError> {
        let w = World::parse(d)?;
        let towns = w
            .towns
            .iter()
            .map(|t| TownModel {
                economy: bytes(d, &t.economy),
                tavern: bytes(d, &t.tavern),
                buildings: bytes(d, &t.buildings),
                shipyard: bytes(d, &t.shipyard),
                council: bytes(d, &t.council),
                facilities_and_tail: d[t.facilities.start..t.tail.end].to_vec(),
            })
            .collect();
        Ok(WorldModel {
            header: d[..world::HEADER_LEN].to_vec(),
            towns,
            matrix: bytes(d, &w.matrix),
            marker: d[w.matrix.end..w.matrix.end + 4].try_into().unwrap(),
            sieges: w.siege_records.iter().map(|r|bytes(d,r)).collect(),
            businesses: chunks(d, &w.businesses, 20),
            offices: w.offices.iter().map(|o| bytes(d, &o.range)).collect(),
            merchants: w.merchants.iter().map(|m| bytes(d, m)).collect(),
            rest: bytes(d, &w.rest),
        })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut h = self.header.clone();
        put_u16(&mut h, 6, self.towns.len());
        put_u16(&mut h, 0xA, self.businesses.len());
        put_u16(&mut h, 0xC, self.offices.len());
        put_u16(&mut h, 0xE, self.merchants.len());
        let mut out = h;
        for (i, t) in self.towns.iter().enumerate() {
            out.extend_from_slice(&t.economy);
            for part in [&t.tavern, &t.buildings, &t.shipyard, &t.council] {
                out.push(SEP);
                out.extend_from_slice(part);
            }
            out.push(SEP);
            out.extend_from_slice(&t.facilities_and_tail);
            if i + 1 < self.towns.len() {
                out.push(SEP);
            }
        }
        out.extend_from_slice(&self.matrix);
        out.extend_from_slice(&self.marker);
        for r in self.sieges.iter().chain(&self.businesses).chain(&self.offices).chain(&self.merchants) {
            out.extend_from_slice(r);
        }
        out.extend_from_slice(&self.rest);
        out
    }
}

impl ShipsModel {
    /// Integra los cambios de capitanes de una instantánea naval. Las imágenes
    /// están en disposición de memoria; el guardado usa la disposición 0x85.
    /// Acepta cambios iguales de otro sistema y rechaza valores divergentes.
    pub fn apply_navigation_captains(&mut self, before: &[u8], after: &[u8]) -> Result<(), String> {
        if before.len() % crate::captain::CAPTAIN_LEN != 0 || after.len() % crate::captain::CAPTAIN_LEN != 0
            || before.len() > after.len() || after.len() / crate::captain::CAPTAIN_LEN > self.captains.len() {
            return Err("capitanes de navegación incompatibles con el guardado".into());
        }
        let mut staged = self.captains.clone();
        for (i, (old, new)) in before.chunks_exact(16).zip(after.chunks_exact(16)).enumerate() {
            if old == new { continue; }
            let record: &[u8; 16] = staged[i].as_slice().try_into().map_err(|_| "capitán truncado")?;
            let mut current = crate::captain::decode_v85(record);
            for j in 0..16 {
                if old[j] == new[j] { continue; }
                let wanted = if j == 14 { u8::from(new[j] != 0) } else { new[j] };
                let prior = if j == 14 { u8::from(old[j] != 0) } else { old[j] };
                if current[j] != prior && current[j] != wanted {
                    return Err(format!("capitán {i} modificado por otro sistema en +0x{j:X}"));
                }
                current[j] = wanted;
            }
            staged[i] = crate::captain::encode_v85(&current).to_vec();
        }
        for (i,new) in after.chunks_exact(16).enumerate().skip(before.len()/16){
            let record: &[u8;16]=staged[i].as_slice().try_into().map_err(|_|"capitán ampliado truncado")?;
            let mut expected:[u8;16]=new.try_into().expect("capitán completo");expected[14]=u8::from(expected[14]!=0);
            if crate::captain::decode_v85(record)!=expected{return Err("capitán ampliado desincronizado".into())}
        }
        self.captains = staged;
        Ok(())
    }
    /// Persiste las listas y entradas del gestor. El llamante debe guardar también
    /// las cartas y contadores emitidos durante la operación en la misma transacción.
    pub fn apply_shipyard_lists(&mut self,before:&p3_core::shipyard::ShipManager,after:&p3_core::shipyard::ShipManager)->Result<(),String>{
        use crate::layout::{CONVOY,CONVOY_OBJECT_LEN,CONVOY_RECORD_LEN};
        if before.ship_count!=after.ship_count || before.entry_count!=after.entry_count ||
            usize::from(before.ship_count)!=self.ships.len() || before.ships.len()!=self.ships.len()*0x180 || after.ships.len()!=before.ships.len() ||
            usize::from(before.entry_count)!=self.convoys.len() || before.entries.len()!=self.convoys.len()*CONVOY_OBJECT_LEN || after.entries.len()!=before.entries.len() {
            return Err("capacidad o imágenes de astillero inválidas".into());
        }
        if self.header.len()<14{return Err("cabecera de barcos truncada".into());}
        if u16::from_le_bytes([self.header[8],self.header[9]])!=before.docked_head || u16::from_le_bytes([self.header[12],self.header[13]])!=before.free_entry_head{return Err("cabeceras anteriores de astillero no coinciden".into());}
        let mut staged=self.clone();
        for i in 0..self.ships.len(){let a=&before.ships[i*0x180..(i+1)*0x180];let b=&after.ships[i*0x180..(i+1)*0x180];if a!=b{staged.apply_ship_image(i,a,b)?;}}
        for (i,record) in self.convoys.iter().enumerate(){
            if record.len()!=CONVOY_RECORD_LEN{return Err("entrada de astillero truncada".into());}
            let a=&before.entries[i*CONVOY_OBJECT_LEN..(i+1)*CONVOY_OBJECT_LEN];let b=&after.entries[i*CONVOY_OBJECT_LEN..(i+1)*CONVOY_OBJECT_LEN];
            let stored=crate::layout::image(&CONVOY,CONVOY_OBJECT_LEN,record);let mut covered=[false;CONVOY_OBJECT_LEN];
            for seg in &CONVOY{let o=usize::from(seg.obj);let n=usize::from(seg.len);let p=usize::from(seg.pos);covered[o..o+n].fill(true);staged.convoys[i][p..p+n].copy_from_slice(&b[o..o+n]);}
            for j in 0..CONVOY_OBJECT_LEN {if covered[j] && a[j]!=stored[j]{return Err(format!("entrada {i}+0x{j:X} anterior no coincide"));}if !covered[j] && a[j]!=b[j]{return Err(format!("entrada {i}+0x{j:X} no guardada"));}}
        }
        staged.header[8..10].copy_from_slice(&after.docked_head.to_le_bytes());staged.header[12..14].copy_from_slice(&after.free_entry_head.to_le_bytes());*self=staged;Ok(())
    }
    pub fn parse(d: &[u8]) -> Result<ShipsModel, ships::ShipsError> {
        let s = Ships::parse(d)?;
        Ok(ShipsModel {
            header: d[..ships::HEADER_LEN].to_vec(),
            captains: chunks(d, &s.captains, ships::CAPTAIN_LEN),
            ships: s.ships.iter().map(|x| bytes(d, &x.range)).collect(),
            convoys: s.convoys.iter().map(|c| bytes(d, c)).collect(),
            tail: bytes(d, &s.tail),
        })
    }

    pub fn write(&self) -> Vec<u8> {
        let mut out = self.header.clone();
        put_u16(&mut out, 0x12, self.captains.len());
        put_u16(&mut out, 0x14, self.ships.len());
        put_u16(&mut out, 0x16, self.convoys.len());
        for r in self.captains.iter().chain(&self.ships).chain(&self.convoys) {
            out.extend_from_slice(r);
        }
        out.extend_from_slice(&self.tail);
        out
    }
}

impl StatisticsModel {
    pub fn parse(d: &[u8]) -> Result<StatisticsModel, statistics::StatisticsError> {
        let s = Statistics::parse(d)?;
        let chains = std::array::from_fn(|i| {
            s.chains[i].as_ref().map(|links| links.iter().map(|l| (l.series, d[l.data.clone()].to_vec())).collect())
        });
        Ok(StatisticsModel {
            header: s.header,
            chains,
            items: s.items.iter().map(|i| bytes(d, &i.range)).collect(),
            slack: s.slack,
        })
    }

    /// Escribe el objeto con el número de elementos recalculado en `header[5]`.
    pub fn write(&self) -> Vec<u8> {
        let mut out = statistics::MARKER.to_le_bytes().to_vec();
        for (i, h) in self.header.iter().enumerate() {
            let v = if i == 5 { u16::try_from(self.items.len()).expect("contador de 16 bits") } else { *h };
            out.extend_from_slice(&v.to_le_bytes());
        }
        for chain in &self.chains {
            match chain {
                None => out.push(0),
                Some(links) => {
                    out.push(1);
                    for (k, (n, data)) in links.iter().enumerate() {
                        out.extend_from_slice(&n.to_le_bytes());
                        out.extend_from_slice(data);
                        out.push(u8::from(k + 1 < links.len()));
                    }
                }
            }
            out.extend_from_slice(&statistics::SEPARATOR.to_le_bytes());
        }
        for item in &self.items {
            out.extend_from_slice(item);
        }
        out.extend_from_slice(&statistics::SEPARATOR.to_le_bytes());
        out.push(self.slack);
        out
    }
}

impl SaveModel {
    pub fn from_save(save: &Save) -> Result<SaveModel, ModelError> {
        let block = |n: &str| save.block_named(n).unwrap();
        Ok(SaveModel {
            header: *save.header(),
            world: WorldModel::parse(block("game_world")).map_err(ModelError::World)?,
            ships: ShipsModel::parse(block("ships")).map_err(ModelError::Ships)?,
            message_pool: block("message_pool").to_vec(),
            route_stop_pool: block("route_stop_pool").to_vec(),
            scheduled_tasks: block("scheduled_tasks").to_vec(),
            sea_battles: block("sea_battles").to_vec(),
            name_bank: block("name_bank").to_vec(),
        })
    }

    pub fn parse(file: &[u8]) -> Result<SaveModel, ModelError> {
        SaveModel::from_save(&Save::parse(file).map_err(ModelError::Container)?)
    }

    pub fn to_save(&self) -> Save {
        let blocks: [Vec<u8>; 7] = [
            self.world.write(),
            self.ships.write(),
            self.message_pool.clone(),
            self.route_stop_pool.clone(),
            self.scheduled_tasks.clone(),
            self.sea_battles.clone(),
            self.name_bank.clone(),
        ];
        debug_assert_eq!(BLOCK_NAMES.len(), blocks.len());
        Save::from_parts(self.header, blocks)
    }

    /// Fichero `.pat` con el compresor original.
    pub fn write(&self) -> Vec<u8> {
        self.to_save().to_file()
    }
}
impl ShipsModel {
    /// Persiste las cabezas de listas navales modificadas, incluida la lista
    /// libre de convoyes. Comprueba todas antes de escribir cualquiera.
    pub fn apply_navigation_heads(&mut self, before: &[u8], after: &[u8]) -> Result<(), String> {
        const HEADS: [(usize, usize); 7] = [(0xe0, 2), (0xe2, 4), (0xe4, 6), (0xe6, 24), (0xe8, 8), (0xea, 10), (0xec, 12)];
        if before.len() < 0xee || after.len() < 0xee || self.header.len() < 26 {
            return Err("cabecera de navegación truncada".into());
        }
        for (source, destination) in HEADS {
            if before[source..source+2] != after[source..source+2]
                && self.header[destination..destination+2] != before[source..source+2]
                && self.header[destination..destination+2] != after[source..source+2] {
                return Err(format!("lista naval modificada por otro sistema en +0x{source:X}"));
            }
        }
        for (source, destination) in HEADS {
            if before[source..source+2] != after[source..source+2] {
                self.header[destination..destination+2].copy_from_slice(&after[source..source+2]);
            }
        }
        Ok(())
    }
    /// Persiste los registros meteorológicos modificados, validando primero
    /// todas las diferencias frente a la cola S02.
    pub fn apply_navigation_weather(&mut self,before:&[u8],after:&[u8])->Result<(),String>{
        if before.len()<0xe0 || after.len()<0xe0 || self.tail.len()<0xc0{return Err("meteorología truncada".into())}
        for i in 0..0xc0 {if before[0x20+i]!=after[0x20+i] && self.tail[i]!=before[0x20+i] && self.tail[i]!=after[0x20+i]{return Err(format!("meteorología modificada por otro sistema en +0x{i:X}"))}}
        for i in 0..0xc0 {if before[0x20+i]!=after[0x20+i]{self.tail[i]=after[0x20+i]}}
        Ok(())
    }
    /// Imagen de un barco ocupado reconstruida con las tablas S02.
    pub fn ship_image(&self,index:usize)->Result<Vec<u8>,String>{
        if index>=self.ships.len(){return Err("índice de barco".into());}
        let raw=self.write();let ships=crate::ships::Ships::parse(&raw).map_err(|e|format!("barcos: {e:?}"))?;
        ships.ship_image(index).ok_or_else(||"barco libre o índice inexistente".into())
    }
    /// Libera un barco de fenómeno meteorológico caducado. Conserva el
    /// enlace libre y el selector que persiste el formato S02.
    pub fn apply_expired_weather_ship(&mut self,index:usize,before:&[u8],after:&[u8])->Result<(),String>{
        use crate::layout::{SHIP_CORE,SHIP_REST,SHIP_STATE};
        if before.len()!=0x180||after.len()!=0x180||u16::from_le_bytes([before[0x134],before[0x135]])!=0x15||u16::from_le_bytes([after[0x134],after[0x135]])!=0xff||before[0x36]!=0{return Err("caducidad de barco meteorológico no admitida".into())}
        let released=vec![after[0x134],after[0x135],after[6],after[7],after[0x137]];
        if self.ships.get(index)==Some(&released){return Ok(())}
        let stored=self.ship_image(index)?;
        for seg in SHIP_CORE.iter().chain(SHIP_REST.iter()).chain(std::iter::once(&SHIP_STATE)){for j in usize::from(seg.obj)..usize::from(seg.obj+seg.len){if !(0x24..0x28).contains(&j)&&before[j]!=stored[j]{return Err(format!("barco meteorológico {index} modificado en +0x{j:X}"))}}}
        self.ships[index]=released;Ok(())
    }
    /// Guarda una ranura liberada por el destructor naval. La ruta debe
    /// estar ausente y todos los campos anteriores persistidos deben coincidir.
    pub fn apply_removed_commercial_ship(&mut self,index:usize,before:&[u8],after:&[u8])->Result<(),String>{self.apply_removed_commercial_ship_with_route(index,before,after,None)}
    pub fn apply_removed_commercial_ship_with_route(&mut self,index:usize,before:&[u8],after:&[u8],expected_route:Option<&crate::ships::SavedRoute>)->Result<(),String>{
        use crate::layout::{SHIP_CORE,SHIP_REST,SHIP_STATE};
        if before.len()!=0x180||after.len()!=0x180||u16::from_le_bytes([before[0x134],before[0x135]])>=0x17||u16::from_le_bytes([after[0x134],after[0x135]])!=0xff||after[0]!=0xff{return Err("baja naval no admitida".into())}
        let encoded=vec![after[0x134],after[0x135],after[6],after[7],after[0x137]];if self.ships.get(index)==Some(&encoded){return Ok(())}let block=self.write();let parsed=crate::ships::Ships::parse(&block).map_err(|e|format!("rutas: {e:?}"))?;let route=parsed.saved_route(index).map_err(str::to_string)?;if expected_route.map_or(!matches!(route,crate::ships::SavedRoute::None),|expected|expected!=&route){return Err("ruta de baja modificada por otro sistema".into())}let stored=self.ship_image(index)?;
        for seg in SHIP_CORE.iter().chain(SHIP_REST.iter()).chain(std::iter::once(&SHIP_STATE)){for j in usize::from(seg.obj)..usize::from(seg.obj+seg.len){if !(0x24..0x28).contains(&j)&&before[j]!=stored[j]{return Err(format!("barco {index} modificado en +0x{j:X}"))}}}
        self.ships[index]=vec![after[0x134],after[0x135],after[6],after[7],after[0x137]];Ok(())
    }
    /// Actualiza el enlace +4 de un barco ocupado sin alterar su ruta.
    pub fn apply_ship_owner_link(&mut self,index:usize,before:u16,after:u16)->Result<(),String>{
        let record=self.ships.get_mut(index).ok_or("barco ausente")?;if record.len()<2+crate::layout::SHIP_CORE_LEN{return Err("barco libre o truncado".into())}let stored=u16::from_le_bytes([record[2],record[3]]);if stored!=before&&stored!=after{return Err("enlace de barco modificado por otro sistema".into())}record[2..4].copy_from_slice(&after.to_le_bytes());Ok(())
    }
    /// Actualiza un barco ocupado preservando ruta y bytes no interpretados.
    /// Rechaza cambios de campos no guardados, de disponibilidad o del indicador de ruta.
    pub fn apply_ship_image(&mut self,index:usize,before:&[u8],after:&[u8])->Result<(),String>{
        use crate::layout::{SHIP_CORE,SHIP_REST,SHIP_STATE,SHIP_CORE_LEN,SHIP_FREE_FROM};
        if before.len()!=0x180||after.len()!=0x180{return Err("tamaño del barco".into());}
        let record=self.ships.get(index).ok_or("índice de barco")?;
        if record.len()<2+SHIP_CORE_LEN{return Err("barco libre o truncado".into());}
        let old_state=u16::from_le_bytes([before[0x134],before[0x135]]);let new_state=u16::from_le_bytes([after[0x134],after[0x135]]);
        if old_state>=SHIP_FREE_FROM||new_state>=SHIP_FREE_FROM{return Err("cambio de estado ocupado/libre no soportado".into());}
        if before[0x36]!=after[0x36]{return Err("cambio de ruta no soportado".into());}
        let route=if record[2+0x36]!=0 {
            let p=2+SHIP_CORE_LEN;if p+2>record.len(){return Err("ruta truncada".into());}
            2+4*usize::from(u16::from_le_bytes([record[p],record[p+1]]))
        }else{0};
        let rest=2+SHIP_CORE_LEN+route;if rest+crate::layout::SHIP_REST_RECORD_LEN>record.len(){return Err("barco truncado".into());}
        let mut covered=[false;0x180];for seg in SHIP_CORE.iter().chain(SHIP_REST.iter()).chain(std::iter::once(&SHIP_STATE)){
            let o=seg.obj as usize;let n=seg.len as usize;covered[o..o+n].fill(true);
        }
        let stored=self.ship_image(index)?;
        for i in 0..0x180{
            if covered[i] && before[i]!=stored[i]{return Err(format!("estado anterior ship+0x{i:X} no coincide"));}
            if !covered[i] && before[i]!=after[i]{return Err(format!("campo ship+0x{i:X} no guardado"));}
        }
        let mut result=record.clone();result[..2].copy_from_slice(&after[0x134..0x136]);
        for (segs,base) in [(&SHIP_CORE[..],2),(&SHIP_REST[..],rest)]{
            for seg in segs {let o=seg.obj as usize;let p=base+seg.pos as usize;let n=seg.len as usize;result[p..p+n].copy_from_slice(&after[o..o+n]);}
        }
        self.ships[index]=result;Ok(())
    }

    /// Sustituye también la ruta, validando primero los campos persistidos y
    /// conservando intacto el modelo si la validación falla.
    pub fn apply_ship_route(&mut self,index:usize,before:&[u8],after:&[u8],points:&[(i16,i16)],town_count:usize)->Result<(),String>{
        if before.len()!=0x180||after.len()!=0x180{return Err("tamaño del barco".into())}
        let mut staged=self.clone();
        let mut intermediate=after.to_vec();intermediate[0x36]=before[0x36];
        staged.apply_ship_image(index,before,&intermediate)?;
        staged.ships[index]=serialize_ship_route(after,points,town_count)?;
        *self=staged;Ok(())
    }
}
impl ShipsModel {
    /// Persiste trabajo naval sin cambios de listas, estados ni entradas de obra/convoy.
    /// Rechaza ampliaciones y campos de barco que el registro no conserva.
    pub fn apply_shipyard_manager(&mut self,before:&p3_core::shipyard::ShipManager,after:&p3_core::shipyard::ShipManager)->Result<(),String>{
        if before.ship_count!=after.ship_count || before.entry_count!=after.entry_count || before.entries!=after.entries ||
           before.ship_count as usize!=self.ships.len() || before.ships.len()!=self.ships.len()*0x180 || after.ships.len()!=before.ships.len(){
            return Err("cambio de entradas de astillero o capacidad no soportado".into());
        }
        if before.docked_head!=after.docked_head || before.free_entry_head!=after.free_entry_head {
            return Err("botaduras y cambios de listas de astillero aún no se persisten con sus avisos".into());
        }
        if self.header.len()<0xe{return Err("cabecera de barcos truncada".into());}
        if u16::from_le_bytes([self.header[8],self.header[9]])!=before.docked_head ||
           u16::from_le_bytes([self.header[12],self.header[13]])!=before.free_entry_head{return Err("cabeceras anteriores de astillero no coinciden".into());}
        let mut staged=self.clone();
        for i in 0..self.ships.len(){let a=&before.ships[i*0x180..(i+1)*0x180];let b=&after.ships[i*0x180..(i+1)*0x180];
            if a[6..8]!=b[6..8] || a[0x134..0x136]!=b[0x134..0x136]{return Err("cambio de lista o estado de barco no soportado".into());}
            if a!=b{staged.apply_ship_image(i,a,b)?;}
        }
        staged.header[8..10].copy_from_slice(&after.docked_head.to_le_bytes());staged.header[12..14].copy_from_slice(&after.free_entry_head.to_le_bytes());
        *self=staged;Ok(())
    }
}

/// Serializa una imagen de barco activo sin ruta, como `ship_save` (0x005177d0).
/// Solo se usa al convertir una ranura libre en barco; no altera el modelo.
pub fn serialize_new_ship(image:&[u8])->Result<Vec<u8>,String>{
    use crate::layout::{SHIP_CORE,SHIP_CORE_LEN,SHIP_REST,SHIP_REST_RECORD_LEN};
    if image.len()!=0x180{return Err("imagen de barco inválida".into())}let state=u16::from_le_bytes([image[0x134],image[0x135]]);if state>0x16||image[0x24..0x28]!=[0;4]||image[0x36]!=0{return Err("barco libre o con ruta no soportada".into())}
    let mut out=vec![0u8;2+SHIP_CORE_LEN+SHIP_REST_RECORD_LEN];out[..2].copy_from_slice(&state.to_le_bytes());for (segs,base) in [(&SHIP_CORE[..],2),(&SHIP_REST[..],2+SHIP_CORE_LEN)]{for seg in segs{let o=usize::from(seg.obj);let n=usize::from(seg.len);let p=base+usize::from(seg.pos);out[p..p+n].copy_from_slice(&image[o..o+n])}}Ok(out)
}

/// Serializa un barco ocupado con el contenido de su descriptor de ruta.
/// Los puntos solo se escriben para rutas propias; las compartidas se
/// reconstruyen a partir de las ciudades al cargar la partida.
pub fn serialize_ship_route(image:&[u8],points:&[(i16,i16)],town_count:usize)->Result<Vec<u8>,String>{
    use crate::layout::{SHIP_CORE,SHIP_CORE_LEN,SHIP_REST,SHIP_REST_RECORD_LEN};
    if image.len()!=0x180{return Err("imagen de barco inválida".into())}
    let state=u16::from_le_bytes([image[0x134],image[0x135]]);
    if state>=0x17{return Err("barco libre".into())}
    let has_route=image[0x24..0x28]!=[0;4];
    let own=if !has_route {0} else if usize::from(image[0x37])<town_count&&usize::from(image[0x38])<town_count {image[0x36]} else {1};
    if own!=0&&points.len()>u16::MAX as usize{return Err("ruta demasiado larga".into())}
    let extra=if own!=0{2+points.len()*4}else{0};
    let mut out=vec![0;2+SHIP_CORE_LEN+extra+SHIP_REST_RECORD_LEN];
    out[..2].copy_from_slice(&state.to_le_bytes());
    for (segs,base) in [(&SHIP_CORE[..],2),(&SHIP_REST[..],2+SHIP_CORE_LEN+extra)]{
        for seg in segs{let o=usize::from(seg.obj);let n=usize::from(seg.len);let p=base+usize::from(seg.pos);out[p..p+n].copy_from_slice(&image[o..o+n]);}
    }
    out[2+0x36]=own;
    if own!=0{let p=2+SHIP_CORE_LEN;out[p..p+2].copy_from_slice(&(points.len() as u16).to_le_bytes());for (i,(x,y)) in points.iter().enumerate(){let p=p+2+i*4;out[p..p+2].copy_from_slice(&x.to_le_bytes());out[p+2..p+4].copy_from_slice(&y.to_le_bytes());}}
    Ok(out)
}

impl ShipsModel {
    /// Activa una ranura libre con una entidad meteorológica creada por 0x78.
    /// La ranura anterior y el resultado ya guardado son las únicas versiones aceptadas.
    pub fn apply_activated_weather_ship(&mut self,index:usize,before:&[u8],after:&[u8])->Result<(),String>{
        if before.len()!=0x180||after.len()!=0x180||u16::from_le_bytes([before[0x134],before[0x135]])<0x17||u16::from_le_bytes([after[0x134],after[0x135]])!=0x15||after[0x36]!=0{return Err("alta meteorológica inválida".into())}
        let encoded=serialize_new_ship(after)?;let record=self.ships.get_mut(index).ok_or("ranura meteorológica ausente")?;
        if *record==encoded{return Ok(())}
        if record.len()!=5||record[..2]!=before[0x134..0x136]||record[2..4]!=before[6..8]||record[4]!=before[0x137]{return Err("ranura meteorológica modificada por otro sistema".into())}
        *record=encoded;Ok(())
    }
    /// Actualiza el enlace de una ranura libre sin aceptar una tercera versión.
    pub fn apply_free_ship(&mut self,index:usize,before:&[u8],after:&[u8])->Result<(),String>{
        if before.len()!=0x180||after.len()!=0x180||u16::from_le_bytes([before[0x134],before[0x135]])<0x17||u16::from_le_bytes([after[0x134],after[0x135]])<0x17{return Err("ranura libre inválida".into())}
        let old=vec![before[0x134],before[0x135],before[6],before[7],before[0x137]];let new=vec![after[0x134],after[0x135],after[6],after[7],after[0x137]];let record=self.ships.get_mut(index).ok_or("ranura libre ausente")?;
        if *record!=old&&*record!=new{return Err("ranura libre modificada por otro sistema".into())}*record=new;Ok(())
    }
    /// Persiste atómicamente la fábrica hanseática y sus ampliaciones.
    /// Valida los registros y cabezas anteriores.
    pub fn apply_hanse_factory(&mut self,before:&p3_core::ships::factory::HanseFactory,after:&p3_core::ships::factory::HanseFactory)->Result<(),String>{self.apply_hanse_factory_with_routes(before,after,None)}
    /// Transacción de fábrica sincronizada con la flota viva. Fusiona únicamente
    /// los bytes que conserva S02: acepta la versión anterior o la ya aplicada y
    /// conserva los cambios independientes. Las rutas se comparan por contenido;
    /// sus tokens nativos nunca se escriben. Un descriptor nuevo usa el marcador
    /// de presencia 1; al recargar, NavigationSnapshot lo elimina y vuelve a enlazar.
    /// Los campos exclusivamente de ejecución no forman parte de esta persistencia.
    pub fn apply_hanse_factory_navigation(
        &mut self,
        before: &p3_core::ships::factory::HanseFactory,
        after: &p3_core::ships::factory::HanseFactory,
        before_routes: &[Option<crate::ships::SavedRoute>],
        after_routes: &[Option<crate::ships::SavedRoute>],
        town_count: usize,
    ) -> Result<(), String> {
        use crate::layout::{CONVOY, CONVOY_OBJECT_LEN, CONVOY_RECORD_LEN, SHIP_CORE, SHIP_REST, SHIP_STATE};
        use crate::ships::SavedRoute;
        fn merge(saved: &mut [u8], old: &[u8], new: &[u8], context: &str) -> Result<(), String> {
            if saved.len()!=old.len() || old.len()!=new.len(){return Err(format!("{context}: tamaño incompatible"))}
            for i in 0..saved.len(){if old[i]!=new[i]{if saved[i]!=old[i]&&saved[i]!=new[i]{return Err(format!("{context}: conflicto en +0x{i:X}"))}saved[i]=new[i]}}
            Ok(())
        }
        fn routed(image:&[u8],route:&SavedRoute,previous_pointer:&[u8],town_count:usize)->Result<Vec<u8>,String>{
            let mut image=image.to_vec();
            let points=match route {
                SavedRoute::None=>{if image[0x36]!=0{return Err("ruta ausente con indicador propio".into())}image[0x24..0x28].fill(0);&[][..]},
                SavedRoute::Owned(points)=>{
                    if points.is_empty(){return Err("ruta propia vacía".into())}
                    if image[0x36]==0&&usize::from(image[0x37])<town_count&&usize::from(image[0x38])<town_count{return Err("ruta propia con indicador compartido".into())}
                    if previous_pointer==[0;4]{image[0x24..0x28].copy_from_slice(&1u32.to_le_bytes())}else{image[0x24..0x28].copy_from_slice(previous_pointer)}
                    points.as_slice()
                },
                SavedRoute::Shared{origin,destination}=>{
                    if image[0x36]!=0||image[0x37]!=*origin||image[0x38]!=*destination||usize::from(*origin)>=town_count||usize::from(*destination)>=town_count{return Err(format!("ruta compartida incompatible con ciudades o indicador (ruta {origin}->{destination}, imagen +0x36={:#x} +0x37={} +0x38={}, estado {:#x})",image[0x36],image[0x37],image[0x38],u16::from_le_bytes([image[0x134],image[0x135]])))}
                    if previous_pointer==[0;4]{image[0x24..0x28].copy_from_slice(&1u32.to_le_bytes())}else{image[0x24..0x28].copy_from_slice(previous_pointer)}
                    &[][..]
                }
            };
            serialize_ship_route(&image,points,town_count)
        }
        if after.ships.len()<before.ships.len()||after.captains.len()<before.captains.len()||after.convoys.len()<before.convoys.len()
            ||!(self.ships.len()==before.ships.len()||self.ships.len()==after.ships.len())
            ||!(self.captains.len()==before.captains.len()||self.captains.len()==after.captains.len())
            ||!(self.convoys.len()==before.convoys.len()||self.convoys.len()==after.convoys.len())
            ||before_routes.len()!=before.ships.len()||after_routes.len()!=after.ships.len()||self.header.len()<0x12{
            return Err("capacidad de fábrica o rutas incompatible".into())
        }
        if before.ships.iter().chain(&after.ships).any(|s|s.len()!=0x180)
            ||before.convoys.iter().chain(&after.convoys).any(|c|c.len()!=CONVOY_OBJECT_LEN){return Err("imagen de fábrica truncada".into())}
        let raw=self.write();let saved=crate::ships::Ships::parse(&raw).map_err(|e|format!("flota guardada: {e:?}"))?;
        let mut staged=self.clone();
        let old=[before.free_captain,before.free_ship,before.free_convoy,before.convoy_head,before.name_cursor,before.name_step];
        let new=[after.free_captain,after.free_ship,after.free_convoy,after.convoy_head,after.name_cursor,after.name_step];
        for (j,k) in [2,4,6,12,14,16].into_iter().enumerate(){
            if old[j]!=new[j]{let v=u16::from_le_bytes([staged.header[k],staged.header[k+1]]);if v!=old[j]&&v!=new[j]{return Err("cabeza de fábrica desincronizada".into())}staged.header[k..k+2].copy_from_slice(&new[j].to_le_bytes())}
        }
        for i in 0..after.ships.len(){
            let b=&after.ships[i];let state=u16::from_le_bytes([b[0x134],b[0x135]]);
            if (state<0x17)!=after_routes[i].is_some(){return Err(format!("disponibilidad y ruta incompatibles: barco {i}"))}
            if i>=before.ships.len(){
                let record=if state<0x17{routed(b,after_routes[i].as_ref().unwrap(),&[0;4],town_count)?}else{vec![b[0x134],b[0x135],b[6],b[7],b[0x137]]};
                if i<staged.ships.len(){if staged.ships[i]!=record{return Err("barco ampliado desincronizado".into())}}else{staged.ships.push(record)}
                continue
            }
            let a=&before.ships[i];let old_state=u16::from_le_bytes([a[0x134],a[0x135]]);
            if (old_state<0x17)!=before_routes[i].is_some(){return Err(format!("ruta anterior incompatible: barco {i}"))}
            if a==b&&before_routes[i]==after_routes[i]{continue}
            if old_state>=0x17&&state>=0x17{staged.apply_free_ship(i,a,b)?;continue}
            if state>=0x17{
                staged.apply_removed_commercial_ship_with_route(i,a,b,before_routes[i].as_ref())?;continue
            }
            let route=after_routes[i].as_ref().unwrap();
            if old_state>=0x17{
                let record=routed(b,route,&[0;4],town_count)?;
                if staged.ships[i]==record{continue}
                let free=vec![a[0x134],a[0x135],a[6],a[7],a[0x137]];
                if staged.ships[i]!=free{return Err("barco libre desincronizado".into())}
                staged.ships[i]=record;continue
            }
            let stored=saved.ship_image(i).ok_or("barco activo desaparecido")?;
            let stored_route=saved.saved_route(i).map_err(str::to_string)?;
            if Some(&stored_route)!=before_routes[i].as_ref()&&&stored_route!=route{return Err(format!("ruta {i} modificada por otro sistema"))}
            let mut updated=stored.clone();
            for seg in SHIP_CORE.iter().chain(SHIP_REST.iter()).chain(std::iter::once(&SHIP_STATE)){
                let o=usize::from(seg.obj);let n=usize::from(seg.len);
                // La dirección pertenece al proceso original/nativo; solo la ruta lógica se persiste.
                if o==0x24&&n==4{continue}
                merge(&mut updated[o..o+n],&a[o..o+n],&b[o..o+n],&format!("barco {i} campo 0x{o:X}"))?;
            }
            // La tabla lógica es la evidencia autoritativa. El indicador y las ciudades
            // se conservan/validan conjuntamente al serializar el registro resultante.
            staged.ships[i]=routed(&updated,route,&stored[0x24..0x28],town_count).map_err(|e|format!("barco {i}: {e}; antes {:?} ruta {:?}, después {:?}, guardado {:?} ruta {:?}",&a[0x36..0x39],before_routes[i],&b[0x36..0x39],&stored[0x36..0x39],stored_route))?;
        }
        for i in 0..after.captains.len(){
            let new=crate::captain::encode_v85(&after.captains[i]);
            if i<before.captains.len(){let old=crate::captain::encode_v85(&before.captains[i]);merge(&mut staged.captains[i],&old,&new,&format!("capitán {i}"))?}
            else if i<staged.captains.len(){if staged.captains[i]!=new{return Err("capitán ampliado desincronizado".into())}}else{staged.captains.push(new.to_vec())}
        }
        for i in 0..after.convoys.len(){
            let b=&after.convoys[i];
            if i<before.convoys.len(){
                let a=&before.convoys[i];if staged.convoys[i].len()!=CONVOY_RECORD_LEN{return Err("convoy truncado".into())}
                for seg in &CONVOY{let o=usize::from(seg.obj);let p=usize::from(seg.pos);let n=usize::from(seg.len);merge(&mut staged.convoys[i][p..p+n],&a[o..o+n],&b[o..o+n],&format!("convoy {i} campo 0x{o:X}"))?}
            }else{
                let mut record=vec![0;CONVOY_RECORD_LEN];for seg in &CONVOY{let o=usize::from(seg.obj);let p=usize::from(seg.pos);let n=usize::from(seg.len);record[p..p+n].copy_from_slice(&b[o..o+n])}
                if i<staged.convoys.len(){if staged.convoys[i]!=record{return Err("convoy ampliado desincronizado".into())}}else{staged.convoys.push(record)}
            }
        }
        let written=staged.write();let check=crate::ships::Ships::parse(&written).map_err(|e|format!("fábrica no recargable: {e:?}"))?;
        for i in 0..after.ships.len(){
            let changed=i>=before.ships.len()||before.ships[i]!=after.ships[i]||before_routes[i]!=after_routes[i];
            if changed{if let Some(expected)=&after_routes[i]{if &check.saved_route(i).map_err(str::to_string)?!=expected{return Err(format!("ruta {i} no se conserva al recargar"))}}}
        }
        *self=staged;Ok(())
    }
    pub fn apply_hanse_factory_with_routes(&mut self,before:&p3_core::ships::factory::HanseFactory,after:&p3_core::ships::factory::HanseFactory,routes:Option<&[Option<crate::ships::SavedRoute>]>)->Result<(),String>{
        use crate::layout::{CONVOY,CONVOY_OBJECT_LEN,CONVOY_RECORD_LEN};
        if after.ships.len()<before.ships.len()||after.captains.len()<before.captains.len()||after.convoys.len()<before.convoys.len()||!(self.ships.len()==before.ships.len()||self.ships.len()==after.ships.len())||!(self.captains.len()==before.captains.len()||self.captains.len()==after.captains.len())||!(self.convoys.len()==before.convoys.len()||self.convoys.len()==after.convoys.len())||self.header.len()<0x12{return Err("capacidad de fábrica incompatible".into())}
        let old=[before.free_captain,before.free_ship,before.free_convoy,before.convoy_head,before.name_cursor,before.name_step];let new=[after.free_captain,after.free_ship,after.free_convoy,after.convoy_head,after.name_cursor,after.name_step];let offsets=[2,4,6,12,14,16];for j in 0..offsets.len(){let k=offsets[j];let v=u16::from_le_bytes([self.header[k],self.header[k+1]]);if v!=old[j]&&v!=new[j]{return Err("cabeza de fábrica desincronizada".into())}}
        let mut staged=self.clone();for i in 0..before.ships.len(){let a=&before.ships[i];let b=&after.ships[i];if a==b{continue}if a.len()!=0x180||b.len()!=0x180{return Err("imagen de barco inválida".into())}let old_state=u16::from_le_bytes([a[0x134],a[0x135]]);let new_state=u16::from_le_bytes([b[0x134],b[0x135]]);if old_state>=0x17&&new_state>=0x17{staged.apply_free_ship(i,a,b)?;continue}if new_state==0xff{staged.apply_removed_commercial_ship_with_route(i,a,b,routes.and_then(|v|v.get(i)).and_then(Option::as_ref))?;continue}if old_state<0x17&&new_state<0x17&&a.iter().zip(b.iter()).enumerate().all(|(j,(x,y))|(4..6).contains(&j)||x==y){staged.apply_ship_owner_link(i,u16::from_le_bytes([a[4],a[5]]),u16::from_le_bytes([b[4],b[5]]))?;continue}let record=staged.ships.get(i).ok_or("barco ausente")?;let encoded=serialize_new_ship(b)?;if *record==encoded{continue}if record.len()!=5&&old_state<0x17&&new_state<0x17{staged.apply_ship_image(i,a,b)?;continue}if record.len()!=5{return Err("la fábrica solo puede activar barcos libres".into())}if old_state<0x17||record[..2]!=a[0x134..0x136]||record[2..4]!=a[6..8]||record[4]!=a[0x137]{return Err("barco libre desincronizado".into())}staged.ships[i]=encoded;}
        for i in 0..before.captains.len(){let a=&before.captains[i];let b=&after.captains[i];if a==b{continue}let record: [u8;16]=staged.captains[i].as_slice().try_into().map_err(|_|"capitán truncado")?;let saved=crate::captain::decode_v85(&record);if saved==*b{continue}if saved!=*a{return Err("capitán anterior no coincide".into())}staged.captains[i]=crate::captain::encode_v85(b).to_vec();}
        for i in 0..before.convoys.len(){let a=&before.convoys[i];let b=&after.convoys[i];if a==b{continue}if a.len()!=CONVOY_OBJECT_LEN||b.len()!=CONVOY_OBJECT_LEN||staged.convoys[i].len()!=CONVOY_RECORD_LEN{return Err("convoy truncado".into())}let stored=crate::layout::image(&CONVOY,CONVOY_OBJECT_LEN,&staged.convoys[i]);let already=CONVOY.iter().all(|seg|{let o=usize::from(seg.obj);let n=usize::from(seg.len);stored[o..o+n]==b[o..o+n]});if already{continue}let mut covered=[false;CONVOY_OBJECT_LEN];for seg in &CONVOY{let o=usize::from(seg.obj);let n=usize::from(seg.len);let p=usize::from(seg.pos);covered[o..o+n].fill(true);if stored[o..o+n]!=a[o..o+n]{return Err("convoy anterior no coincide".into())}staged.convoys[i][p..p+n].copy_from_slice(&b[o..o+n]);}for j in 0..CONVOY_OBJECT_LEN{if !covered[j]&&a[j]!=b[j]{return Err("campo de convoy no serializado".into())}}}
        for i in before.ships.len()..after.ships.len(){let b=&after.ships[i];if b.len()!=0x180{return Err("barco nuevo truncado".into())}let state=u16::from_le_bytes([b[0x134],b[0x135]]);let record=if state<0x17{serialize_new_ship(b)?}else{vec![b[0x134],b[0x135],b[6],b[7],b[0x137]]};if i<staged.ships.len(){if staged.ships[i]!=record{return Err("barco ampliado desincronizado".into())}}else{staged.ships.push(record)}}
        for i in before.captains.len()..after.captains.len(){let record=crate::captain::encode_v85(&after.captains[i]).to_vec();if i<staged.captains.len(){if staged.captains[i]!=record{return Err("capitán ampliado desincronizado".into())}}else{staged.captains.push(record)}}
        for i in before.convoys.len()..after.convoys.len(){let b=&after.convoys[i];if b.len()!=CONVOY_OBJECT_LEN{return Err("convoy ampliado truncado".into())}let mut record=vec![0u8;CONVOY_RECORD_LEN];for seg in &CONVOY{let o=usize::from(seg.obj);let n=usize::from(seg.len);let p=usize::from(seg.pos);record[p..p+n].copy_from_slice(&b[o..o+n])}if i<staged.convoys.len(){if staged.convoys[i]!=record{return Err("convoy ampliado desincronizado".into())}}else{staged.convoys.push(record)}}
        for j in 0..offsets.len(){let k=offsets[j];staged.header[k..k+2].copy_from_slice(&new[j].to_le_bytes())}*self=staged;Ok(())
    }
}
impl SaveModel {
    /// Encuadra batallas base del formato actual sin reconstruir su simulación.
    pub fn current_basic_sea_battles(&self)->Result<crate::tail::CurrentSeaBattles<'_>,crate::tail::TailError>{crate::tail::CurrentSeaBattles::parse(&self.sea_battles)}
    /// Sustituye el bloque con batallas base válidas y comprueba el contenedor.
    /// No acepta variantes de ciudad ni estados que el lector actual no pueda encuadrar.
    pub fn set_current_basic_sea_battles(&mut self,battles:&[Option<p3_core::ships::battle::BattleSaveInput<'_>>],pairs:&[Vec<(u32,u32)>])->Result<(),String>{
        let block=crate::tail::serialize_basic_sea_battles(battles,pairs).map_err(|e|format!("batallas: {e:?}"))?;crate::tail::CurrentSeaBattles::parse(&block).map_err(|e|format!("batallas no recargables: {e:?}"))?;let mut staged=self.clone();staged.sea_battles=block.clone();let save=staged.to_save();let parsed=crate::Save::parse_unpacked(&save.to_unpacked()).map_err(|e|format!("contenedor: {e}"))?;if parsed!=save{return Err("separador ambiguo en el bloque de batallas".into())}self.sea_battles=block;Ok(())
    }
    /// Sustituye una batalla básica única si su imagen anterior no cambió.
    /// Rechaza listas auxiliares y variantes que aún no se pueden reconstruir.
    pub fn replace_paused_convoy_battle(&mut self,before:&[u8],after:&p3_core::ships::battle::BattleSaveInput<'_>)->Result<(),String>{
        self.replace_paused_convoy_battle_checked(before,after,None)
    }
    /// Permite cambios de bando solo si la flota transaccional acredita las
    /// nuevas listas de ambos convoyes. La llamada no publica la flota por sí sola.
    pub fn replace_paused_convoy_battle_with_fleet(&mut self,before:&[u8],after:&p3_core::ships::battle::BattleSaveInput<'_>,fleet:&crate::ships::NavigationSnapshot)->Result<(),String>{
        self.replace_paused_convoy_battle_checked(before,after,Some(fleet))
    }
    fn replace_paused_convoy_battle_checked(&mut self,before:&[u8],after:&p3_core::ships::battle::BattleSaveInput<'_>,fleet:Option<&crate::ships::NavigationSnapshot>)->Result<(),String>{
        let current=self.current_basic_sea_battles().map_err(|e|format!("batallas: {e:?}"))?;
        if current.objects.len()!=1||current.body(0)!=Some(before){return Err("batalla anterior modificada o gestor incompatible".into())}
        let mut pairs:Vec<Vec<(u32,u32)>>=Vec::new();let mut seen=Vec::new();for (i,pair) in current.pairs.iter().enumerate(){let index=usize::try_from(pair.index).map_err(|_|"índice de lista excesivo")?;if index>=100_000{return Err("índice de lista excesivo".into())}if index>=pairs.len(){pairs.resize_with(index+1,Vec::new);seen.resize(index+1,false)}if seen[index]{return Err("lista de batalla duplicada".into())}seen[index]=true;pairs[index]=current.entries(i).ok_or("lista ausente")?.collect();}
        let old=current.initial_convoy_state(0,0x59000000,0).map_err(|e|format!("batalla anterior no reconstruible: {e:?}"))?;
        let mut staged=self.clone();staged.set_current_basic_sea_battles(&[Some(p3_core::ships::battle::BattleSaveInput{object:after.object,nexts:after.nexts,prevs:after.prevs,bitmap:after.bitmap,local_ships:after.local_ships,projectiles:after.projectiles})],&pairs)?;
        let frame=staged.current_basic_sea_battles().map_err(|e|format!("batalla: {e:?}"))?;let new=frame.initial_convoy_state(0,0x59000000,0).map_err(|e|format!("batalla no reconstruible: {e:?}"))?;
        if old.local_ships.len()!=new.local_ships.len()||old.object[0x35a..0x35e]!=new.object[0x35a..0x35e]||old.object[0x65e..0x662]!=new.object[0x65e..0x662]||old.local_ships.iter().zip(&new.local_ships).any(|(a,b)|a[4..8]!=b[4..8]){return Err("identidad de batalla modificada".into())}
        if old.local_ships.iter().zip(&new.local_ships).any(|(a,b)|a[0x13d]!=b[0x13d]){let fleet=fleet.ok_or("participantes de batalla modificados sin transacción de flota")?;validate_initial_convoy_battle(fleet,after)?;}
        *self=staged;Ok(())
    }
}

impl SaveModel {
    /// Publica conjuntamente un cambio naval y el bloque de una batalla
    /// comercial inicial. Admite desvíos propios y rechaza ampliaciones, batallas
    /// preexistentes y conflictos en registros persistidos; es atómico.
    pub fn apply_initial_battle_transition(
        &mut self,
        before: &crate::ships::NavigationSnapshot,
        after: &crate::ships::NavigationSnapshot,
        battle: &p3_core::ships::battle::BattleSaveInput<'_>,
        slot: u8,
    ) -> Result<(), String> {
        self.apply_initial_battle_transition_inner(before,after,battle,slot,false)
    }
    /// Publica la flota y una batalla inicial de dos convoyes, incluidos
    /// participantes especiales. No reconstruye ni ejecuta combate vivo.
    pub fn apply_initial_convoy_battle_transition(
        &mut self,before:&crate::ships::NavigationSnapshot,after:&crate::ships::NavigationSnapshot,
        battle:&p3_core::ships::battle::BattleSaveInput<'_>,slot:u8,
    )->Result<(),String>{self.apply_initial_battle_transition_inner(before,after,battle,slot,true)}
    /// Variante que conserva las listas temporales ya actualizadas por el
    /// gestor, validando su bloque anterior antes de publicar la batalla.
    pub fn apply_initial_convoy_battle_with_pairs(
        &mut self,before:&crate::ships::NavigationSnapshot,after:&crate::ships::NavigationSnapshot,
        battle:&p3_core::ships::battle::BattleSaveInput<'_>,slot:u8,pairs:&[Vec<(u32,u32)>],
    )->Result<(),String>{
        let expected=crate::tail::serialize_basic_sea_battles(&[],pairs).map_err(|e|format!("gestor: {e:?}"))?;
        if self.sea_battles!=expected{return Err("listas del gestor de batalla modificadas".into())}
        let mut staged=self.clone();staged.sea_battles=vec![0;5];staged.apply_initial_convoy_battle_transition(before,after,battle,slot)?;
        staged.set_current_basic_sea_battles(&[Some(p3_core::ships::battle::BattleSaveInput{object:battle.object,nexts:battle.nexts,prevs:battle.prevs,bitmap:battle.bitmap,local_ships:battle.local_ships,projectiles:battle.projectiles})],pairs)?;
        *self=staged;Ok(())
    }
    fn apply_initial_battle_transition_inner(
        &mut self,before:&crate::ships::NavigationSnapshot,after:&crate::ships::NavigationSnapshot,
        battle:&p3_core::ships::battle::BattleSaveInput<'_>,slot:u8,allow_convoys:bool,
    )->Result<(),String>{
        use crate::layout::{SHIP_CORE,SHIP_REST,SHIP_STATE,CONVOY,CONVOY_OBJECT_LEN};
        if slot == u8::MAX { return Err("ranura de batalla inválida".into()); }
        let existing=self.current_basic_sea_battles().map_err(|e|format!("gestor de batallas inválido: {e:?}"))?;
        if !existing.objects.is_empty() || !existing.pairs.is_empty() {
            return Err("gestor de batallas ocupado o con listas auxiliares".into());
        }
        if before.manager.len()<0xf8 || after.manager.len()!=before.manager.len()
            || before.manager[0xf2..0xf8]!=after.manager[0xf2..0xf8]
            || before.ships.len()!=self.ships.ships.len()*0x180 || after.ships.len()!=before.ships.len()
            || before.convoys.len()!=self.ships.convoys.len()*CONVOY_OBJECT_LEN || after.convoys.len()!=before.convoys.len()
            || before.captains.len()!=after.captains.len() || before.routes.len()!=self.ships.ships.len() || after.routes.len()!=before.routes.len() {
            return Err("ampliación naval o capitanes no admitidos".into());
        }
        if battle.object.len()<0xad0 || !battle.projectiles.is_empty() || !battle.bitmap.is_empty(){return Err("batalla inicial inválida".into())}
        let convoy_count=after.convoys.len()/CONVOY_OBJECT_LEN;
        let allocated=if allow_convoys { validate_initial_convoy_battle(after,battle)? } else {
        if battle.local_ships.len()!=2
            || u16::from_le_bytes([battle.object[0x14],battle.object[0x15]])!=2 {
            return Err("batalla comercial inicial inválida".into());
        }
        let mut allocated=Vec::new();
        for local in battle.local_ships {
            if local.len()<0x164 {return Err("barco local truncado".into())}
            let id=u32::from_le_bytes(local[4..8].try_into().expect("barco local")) as usize;
            let start=id.checked_mul(0x180).ok_or("barco mundial inválido")?;
            let ship=after.ships.get(start..start+0x180).ok_or("barco mundial inexistente")?;
            let side=usize::from(local[0x13d]);
            if side>1 || u16::from_le_bytes([ship[0x134],ship[0x135]])!=0x14
                || u16::from(ship[0])!=u16::from_le_bytes([battle.object[0x35a+2*side],battle.object[0x35b+2*side]]) {
                return Err("barco mundial no corresponde al bando".into());
            }
            let cv=usize::from(u16::from_le_bytes([ship[8],ship[9]]));
            let convoy=after.convoys.get(cv*CONVOY_OBJECT_LEN..(cv+1)*CONVOY_OBJECT_LEN).ok_or("convoy inexistente")?;
            if u16::from_le_bytes([convoy[0xa],convoy[0xb]]) as usize!=id
                || u16::from_le_bytes([convoy[0x10],convoy[0x11]]) as usize!=id || allocated.contains(&cv) {
                return Err("convoy de batalla incoherente".into());
            }
            allocated.push(cv);
        }
        allocated
        };
        let mut cursor=usize::from(u16::from_le_bytes([after.manager[0xe4],after.manager[0xe5]]));
        let mut seen=vec![false;convoy_count];
        while cursor<convoy_count {
            if seen[cursor] || allocated.contains(&cursor) {return Err("lista libre de convoyes incoherente".into())}
            seen[cursor]=true;let at=cursor*CONVOY_OBJECT_LEN+8;
            cursor=usize::from(u16::from_le_bytes([after.convoys[at],after.convoys[at+1]]));
        }
        let mut staged=self.clone();
        staged.ships.apply_navigation_captains(&before.captains,&after.captains)?;
        staged.ships.apply_navigation_heads(&before.manager,&after.manager)?;
        staged.ships.apply_navigation_weather(&before.manager,&after.manager)?;
        for i in 0..staged.ships.ships.len() {
            let at=i*0x180;let old=&before.ships[at..at+0x180];let now=&after.ships[at..at+0x180];
            let route_changed=before.routes[i]!=after.routes[i];
            if old==now && !route_changed {continue}
            if u16::from_le_bytes([now[0x134],now[0x135]])==0xff {if route_changed&&!(matches!(before.routes.get(i),Some(Some(_)))&&matches!(after.routes.get(i),Some(None))){return Err("ruta de baja modificada".into())}if u16::from_le_bytes([old[0x134],old[0x135]])==0x15{staged.ships.apply_expired_weather_ship(i,old,now)?}else{staged.ships.apply_removed_commercial_ship_with_route(i,old,now,before.routes.get(i).and_then(Option::as_ref))?}continue}
            let pointer_changed=old[0x24..0x28]!=now[0x24..0x28];
            let logical_binding=allow_convoys&&matches!(after.routes[i],Some(crate::ships::SavedRoute::Owned(_)|crate::ships::SavedRoute::Shared{..}));
            if !route_changed && (old[0x36]!=now[0x36] || (pointer_changed&&!logical_binding)) {
                return Err(format!("cambio de ruta no admitido para barco {i}"));
            }
            if route_changed {
                let block=staged.ships.write();
                let saved=crate::ships::Ships::parse(&block).map_err(|e|format!("rutas: {e:?}"))?;
                let stored_route=saved.saved_route(i).map_err(|e|format!("ruta {i}: {e}"))?;
                if before.routes[i].as_ref()!=Some(&stored_route) && after.routes[i].as_ref()!=Some(&stored_route) {return Err(format!("ruta {i} modificada por otro sistema"))}
            }
            let stored=staged.ships.ship_image(i)?;let mut updated=stored.clone();
            for seg in SHIP_CORE.iter().chain(SHIP_REST.iter()).chain(std::iter::once(&SHIP_STATE)) {
                for j in usize::from(seg.obj)..usize::from(seg.obj+seg.len) {
                    if old[j]!=now[j] {
                        if !(0x24..0x28).contains(&j) && stored[j]!=old[j] && stored[j]!=now[j] {
                            return Err(format!("barco {i} modificado por otro sistema en +0x{j:X}"));
                        }
                        updated[j]=now[j];
                    }
                }
            }
            if route_changed {updated[0x24..0x28].copy_from_slice(&now[0x24..0x28]);updated[0x36]=now[0x36];match &after.routes[i]{
                Some(crate::ships::SavedRoute::Owned(points)) if updated[0x24..0x28]!=[0;4]&&!points.is_empty()=>staged.ships.apply_ship_route(i,&stored,&updated,points,staged.world.towns.len())?,
                Some(crate::ships::SavedRoute::None) if allow_convoys && updated[0x36]==0 && updated[0x24..0x28]==[0;4]=>staged.ships.apply_ship_route(i,&stored,&updated,&[],staged.world.towns.len())?,
                Some(crate::ships::SavedRoute::Shared{origin,destination}) if allow_convoys && updated[0x36]==0 && updated[0x24..0x28]!=[0;4] && updated[0x37]==*origin && updated[0x38]==*destination && usize::from(*origin)<staged.world.towns.len() && usize::from(*destination)<staged.world.towns.len()=>staged.ships.apply_ship_route(i,&stored,&updated,&[],staged.world.towns.len())?,
                _=>return Err(format!("cambio de ruta no admitido para barco {i}: {:?}, origen {}, destino {}, indicador {}",after.routes[i],updated[0x37],updated[0x38],updated[0x36]))
            }
            let written=staged.ships.write();let parsed=crate::ships::Ships::parse(&written).map_err(|e|format!("rutas: {e:?}"))?;let actual=parsed.saved_route(i).map_err(|e|format!("ruta {i}: {e}"))?;
            if after.routes[i].as_ref()!=Some(&actual){return Err(format!("ruta {i} no se puede reconstruir tras guardar"))}
            }else{staged.ships.apply_ship_image(i,&stored,&updated)?}
        }
        for i in 0..staged.ships.convoys.len() {
            let at=i*CONVOY_OBJECT_LEN;let old=&before.convoys[at..at+CONVOY_OBJECT_LEN];let now=&after.convoys[at..at+CONVOY_OBJECT_LEN];
            if old==now {continue}
            let record=&mut staged.ships.convoys[i];let stored=crate::layout::image(&CONVOY,CONVOY_OBJECT_LEN,record);
            for seg in &CONVOY {let o=usize::from(seg.obj);let p=usize::from(seg.pos);for k in 0..usize::from(seg.len) {
                if old[o+k]!=now[o+k] {
                    if stored[o+k]!=old[o+k] && stored[o+k]!=now[o+k] {return Err(format!("convoy {i} modificado por otro sistema"))}
                    record[p+k]=now[o+k];
                }
            }}
        }
        let mut slots=(0..=usize::from(slot)).map(|_|None).collect::<Vec<_>>();
        slots[usize::from(slot)]=Some(p3_core::ships::battle::BattleSaveInput{
            object:battle.object,nexts:battle.nexts,prevs:battle.prevs,bitmap:battle.bitmap,
            local_ships:battle.local_ships,projectiles:battle.projectiles,
        });
        staged.set_current_basic_sea_battles(&slots,&[])?;
        let block=staged.ships.write();
        let parsed=crate::ships::Ships::parse(&block).map_err(|e|format!("flota no recargable: {e:?}"))?;
        parsed.navigation_snapshot().map_err(|e|format!("listas navales no recargables: {e}"))?;
        let battles=staged.current_basic_sea_battles().map_err(|e|format!("batalla no recargable: {e:?}"))?;
        if allow_convoys {battles.initial_convoy_state(0,0x59000000,0)} else {battles.initial_commercial_state(0,0x59000000,0)}.map_err(|e|format!("batalla no admitida: {e:?}"))?;
        *self=staged;Ok(())
    }
}

/// Valida que los barcos seleccionados correspondan exactamente a las listas
/// de los dos convoyes y que estos no estén en la lista naval activa.
pub fn validate_initial_convoy_battle(after:&crate::ships::NavigationSnapshot,battle:&p3_core::ships::battle::BattleSaveInput<'_>)->Result<Vec<usize>,String>{
 const SHIP:usize=0x180;const CV:usize=0x3c;
 let o=battle.object;let u16at=|b:&[u8],at:usize|u16::from_le_bytes([b[at],b[at+1]]);
 if after.manager.len()<0xee||after.ships.len()%SHIP!=0||after.convoys.len()%CV!=0{return Err("flota truncada".into())}
 if o.len()<0xad0||o[0x558]!=2{return Err("participantes de batalla inválidos".into())}
 let ship_count=after.ships.len()/SHIP;let cv_count=after.convoys.len()/CV;
 let n0=usize::from(u16at(o,0x658));let n1=usize::from(u16at(o,0x65a));
 if n0+n1==0||n0.checked_add(n1)!=Some(battle.local_ships.len())||usize::from(u16at(o,0x14))!=battle.local_ships.len(){return Err("conteo de batalla inválido".into())}
 let ids=[usize::from(u16at(o,0x65e)),usize::from(u16at(o,0x660))];if ids[0]>=cv_count||ids[1]>=cv_count||ids[0]==ids[1]{return Err("convoyes de batalla inválidos".into())}
 let mut local_ids=vec![false;ship_count];
 for side in 0..2{
  let base=ids[side]*CV;let leader=usize::from(u16at(&after.convoys,base+0x10));let status=u16at(&after.convoys,base+0x12);if status!=0x12&&status!=0x14{return Err("estado de convoy inválido".into())}
  let participant=u16at(o,0x35a+2*side);let mut expected=vec![false;ship_count];let mut seen=vec![false;ship_count];let mut current=usize::from(u16at(&after.convoys,base+0xa));
  while current<ship_count{if seen[current]{return Err("lista de convoy cíclica".into())}seen[current]=true;let at=current*SHIP;if usize::from(u16at(&after.ships,at+8))!=ids[side]{return Err("miembro de convoy incoherente".into())}if participant==0xfe||u16::from(after.ships[at])==participant{expected[current]=true}current=usize::from(u16at(&after.ships,at+6));}
  let count=if side==0{n0}else{n1};if (leader>=ship_count||!seen[leader])&&!(count==0&&!seen.iter().any(|&v|v)&&leader>=ship_count){return Err("líder de convoy inválido".into())}
  let mut local_id=u16at(o,if side==0{0x10}else{0x65c});let mut previous=u16::MAX;let mut visits=0usize;while local_id!=u16::MAX{let index=usize::from(local_id);let local=battle.local_ships.get(index).ok_or("barco local ausente")?;if local.len()<0x164||usize::from(local[0x13d])!=side||battle.prevs.get(index)!=Some(&previous){return Err("barco local inválido".into())}let id=u32::from_le_bytes(local[4..8].try_into().expect("barco local")) as usize;if id>=ship_count||local_ids[id]||!expected[id]{return Err("selección de convoy incoherente".into())}local_ids[id]=true;expected[id]=false;visits+=1;if visits>battle.local_ships.len(){return Err("lista local cíclica".into())}previous=local_id;local_id=*battle.nexts.get(index).ok_or("enlace local ausente")?;}
  if visits!=if side==0{n0}else{n1}{return Err("conteo de bando incoherente".into())}if expected.iter().any(|x|*x){return Err("selección de convoy incompleta".into())}
 }
 for (head,label) in [(0xec,"convoy de batalla en lista activa"),(0xe4,"convoy de batalla en lista libre")]{let mut current=usize::from(u16at(&after.manager,head));let mut seen=vec![false;cv_count];while current<cv_count{if seen[current]||ids.contains(&current){return Err(label.into())}seen[current]=true;current=usize::from(u16at(&after.convoys,current*CV+8));}}
 Ok(ids.to_vec())
}
