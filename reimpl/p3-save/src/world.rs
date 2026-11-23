//! Bloque `game_world` (S02): cabecera de mundo y reparto de las ciudades en sus secciones.
//!
//! Orden observado en los 7 guardados del corpus (24 ciudades): cabecera de `0xC7` bytes y, por ciudad, bloque económico (`0x6D4`) `7F`,
//! taberna (`0x14`) `7F`, datos de edificios (variable) `7F`, astillero (`0x1A`) `7F`, concejo (`0x88`) `7F`, 21 instalaciones (`0x150`) y
//! un resto de `0x55` bytes más `0x10` por nodo de suministro, cerrado por `7F` salvo en la última ciudad. El tamaño de los datos de edificios sale de
//! `town_get_building_data_serialized_size` (`0x0051E9F0`): `0x44 + 8·registros + ancho·alto + 0x64`, con el número de registros en el
//! primer `u16` y el ancho y alto como los dos primeros `u32` del plano. Detrás de las ciudades empieza la matriz de distancias.

use crate::layout;
use p3_core::economy::world::TickEntry;
use p3_core::town::{off, Town, FACILITIES};
use std::ops::Range;

pub const HEADER_LEN: usize = 0xC7;
pub const ECONOMY_LEN: usize = 0x6D4;
pub const TAVERN_LEN: usize = 0x14;
pub const SHIPYARD_LEN: usize = 0x1A;
pub const COUNCIL_LEN: usize = 0x88;
pub const FACILITIES_LEN: usize = FACILITIES * 16;
/// Resto fijo tras las instalaciones, sin nodos ni el separador final.
pub const TAIL_FIXED: usize = 0x55;
pub const NODE_LEN: usize = 0x10;
const SEP: u8 = 0x7F;
/// Posición de la ciudad dentro del bloque económico (`Town+0x2C0`).
const INDEX_AT: usize = off::TOWN_INDEX;

#[derive(Debug, PartialEq, Eq)]
pub enum WorldError {
    Truncated,
    /// Un separador `7F` o el índice de ciudad no está donde toca: `(ciudad, qué)`.
    Layout(usize, &'static str),
}

/// Secciones de una ciudad como rangos dentro del bloque `game_world`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TownSections {
    pub economy: Range<usize>,
    pub tavern: Range<usize>,
    pub buildings: Range<usize>,
    pub shipyard: Range<usize>,
    pub council: Range<usize>,
    pub facilities: Range<usize>,
    /// Resto con los nodos de suministro (sin el separador final).
    pub tail: Range<usize>,
}

#[derive(Debug)]
pub struct World<'a> {
    data: &'a [u8],
    pub towns: Vec<TownSections>,
    /// Primer byte tras la última ciudad (matriz de distancias).
    pub after_towns: usize,
    /// Matriz de distancias (`ciudades²` bytes).
    pub matrix: Range<usize>,
    /// Conjunto de registros de asedio tras el marcador; su longitud es variable.
    pub sieges: Range<usize>,
    /// Un registro por ciudad, incluidos acuerdos y solicitudes. Las escenas activas aún no se admiten.
    pub siege_records: Vec<Range<usize>>,
    /// Registros de 20 bytes (`merchant_business_serialize`, `0x004D3250`): su número es el `u16` de la cabecera en `+0xA`.
    pub businesses: Range<usize>,
    /// Entradas de oficina: tantas como el `u16` de la cabecera en `+0xC`; libres (6 bytes, primer `u16` = `FFFF`) o completas (`0x44C`).
    pub offices: Vec<OfficeSlot>,
    /// Comerciantes: tantos como el `u16` de la cabecera en `+0xE`; cada uno `1552` bytes más sus listas de barcos.
    pub merchants: Vec<Range<usize>>,
    /// Lo que sigue a los comerciantes (historial de información, centros comerciales y ofertas): `world_rest::WorldRest`.
    pub rest: Range<usize>,
}

/// Entrada de la tabla de oficinas del guardado.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OfficeSlot {
    pub range: Range<usize>,
    pub free: bool,
}

pub const OFFICE_FULL_LEN: usize = 0x44C;
pub const OFFICE_FREE_LEN: usize = 6;
pub const MERCHANT_FIXED_LEN: usize = 1552;
/// Posición, dentro del registro de comerciante, del contador de la primera de las diez listas de barcos (`u16` cada uno).
const MERCHANT_LIST_COUNTS: usize = 1532;

/// Diez grupos guardados y un undécimo grupo transitorio del comerciante.
pub type MerchantShipGroups = [Vec<u16>; 11];

/// Reconstruye las listas variables sin interpretar los punteros x86.
pub fn merchant_ship_groups(record:&[u8])->Result<MerchantShipGroups,String>{
 if record.len()<MERCHANT_FIXED_LEN{return Err("comerciante truncado".into())}
 let mut groups:MerchantShipGroups=std::array::from_fn(|_|Vec::new());let mut at=MERCHANT_FIXED_LEN;
 for (i,group) in groups.iter_mut().take(10).enumerate(){let pos=MERCHANT_LIST_COUNTS+i*2;let count=usize::from(u16::from_le_bytes([record[pos],record[pos+1]])&0x7fff);let end=at.checked_add(count*2).ok_or("grupo demasiado grande")?;let data=record.get(at..end).ok_or("grupo de comerciante truncado")?;*group=data.chunks_exact(2).map(|v|u16::from_le_bytes([v[0],v[1]])).collect();at=end;}
 if at!=record.len(){return Err("longitud de grupos de comerciante inválida".into())}Ok(groups)
}

/// Reescribe las listas desde la parte activa y detecta cambios ajenos a la transacción.
pub fn apply_merchant_ship_groups(record:&mut Vec<u8>,merchant:&[u8],before:&MerchantShipGroups,groups:&MerchantShipGroups)->Result<(),String>{
 if merchant.len()<0x622{return Err("comerciante vivo truncado".into())}let mut active:MerchantShipGroups=std::array::from_fn(|_|Vec::new());
 for i in 0..11{let count=usize::from(u16::from_le_bytes([merchant[0x60c+i*2],merchant[0x60d+i*2]])&0x7fff);if count>groups[i].len(){return Err("grupo vivo truncado".into())}active[i].extend_from_slice(&groups[i][..count]);}
 let stored=merchant_ship_groups(record)?;if stored[..10]!=before[..10]&&stored[..10]!=active[..10]{return Err("grupos de comerciante modificados por otro sistema".into())}
 let mut next=record[..MERCHANT_FIXED_LEN].to_vec();for i in 0..10{next[MERCHANT_LIST_COUNTS+i*2..MERCHANT_LIST_COUNTS+i*2+2].copy_from_slice(&merchant[0x60c+i*2..0x60e+i*2]);for id in &active[i]{next.extend_from_slice(&id.to_le_bytes());}}
 *record=next;Ok(())
}

fn u16_at(d: &[u8], o: usize) -> Result<usize, WorldError> {
    d.get(o..o + 2).map(|s| usize::from(u16::from_le_bytes([s[0], s[1]]))).ok_or(WorldError::Truncated)
}
fn u32_at(d: &[u8], o: usize) -> Result<usize, WorldError> {
    d.get(o..o + 4)
        .map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]) as usize)
        .ok_or(WorldError::Truncated)
}

/// Longitud de un registro de asedio con listas y escena opaca de versión actual.
pub fn siege_record_len(data:&[u8],city:usize)->Result<usize,WorldError>{
    if !data.get(4).is_some_and(|&b|usize::from(b)==city){return Err(WorldError::Layout(city,"índice de asedio"))}
    let agreements=usize::from(*data.get(17).ok_or(WorldError::Truncated)?);
    let requests_at=18usize.checked_add(agreements*8).ok_or(WorldError::Truncated)?;
    let requests=usize::from(*data.get(requests_at).ok_or(WorldError::Truncated)?);
    let scene_at=requests_at.checked_add(1).and_then(|x|x.checked_add(requests*16)).ok_or(WorldError::Truncated)?;
    let scene=scene_at.checked_add(1).ok_or(WorldError::Truncated)?;
    if *data.get(scene_at).ok_or(WorldError::Truncated)?==0{return Ok(scene)}
    let count=usize::from(*data.get(scene+0x4f).ok_or(WorldError::Truncated)?);
    let first=usize::from(*data.get(scene+0x50).ok_or(WorldError::Truncated)?);
    let rows=if first==0xff{0}else{count.saturating_sub(first)};
    let end=scene.checked_add(0x51).and_then(|n|n.checked_add(rows*8)).ok_or(WorldError::Truncated)?;
    data.get(..end).ok_or(WorldError::Truncated)?;Ok(end)
}

/// Una escena guardada necesita un simulador propio antes de avanzar el reloj.
pub fn siege_has_scene(data:&[u8],city:usize)->Result<bool,WorldError>{
    let len=siege_record_len(data,city)?;if len!=data.len(){return Err(WorldError::Layout(city,"longitud de asedio"))}
    let requests_at=18+usize::from(data[17])*8;
    let scene_at=requests_at+1+usize::from(data[requests_at])*16;
    Ok(data[scene_at]!=0)
}

/// Campos serializados de la escena, si el registro contiene una.
pub fn siege_scene(data:&[u8],city:usize)->Result<Option<&[u8]>,WorldError>{
    if !siege_has_scene(data,city)?{return Ok(None)}
    let requests_at=18+usize::from(data[17])*8;let scene_at=requests_at+1+usize::from(data[requests_at])*16;
    Ok(Some(&data[scene_at+1..]))
}

impl<'a> World<'a> {
    pub fn parse(data: &'a [u8]) -> Result<World<'a>, WorldError> {
        let count = u16_at(data, 6)?;
        let mut towns = Vec::with_capacity(count);
        let mut pos = HEADER_LEN;
        let mut after = HEADER_LEN;
        for t in 0..count {
            let eco = pos..pos + ECONOMY_LEN;
            let at = |o: usize| data.get(o).copied().ok_or(WorldError::Truncated);
            if at(eco.start + INDEX_AT)? as usize != t {
                return Err(WorldError::Layout(t, "índice de ciudad"));
            }
            if at(eco.end)? != SEP {
                return Err(WorldError::Layout(t, "separador tras el bloque económico"));
            }
            let tavern = eco.end + 1..eco.end + 1 + TAVERN_LEN;
            if at(tavern.end)? != SEP {
                return Err(WorldError::Layout(t, "separador tras la taberna"));
            }
            let b = tavern.end + 1;
            let plan = b + 0x44 + 8 * u16_at(data, b)?;
            let (w, h) = (u32_at(data, plan)?, u32_at(data, plan + 4)?);
            let cells = w.checked_mul(h).filter(|&n| n < 1 << 24).ok_or(WorldError::Layout(t, "plano"))?;
            let buildings = b..plan + cells + 0x64;
            let shipyard = buildings.end + 1..buildings.end + 1 + SHIPYARD_LEN;
            let council = shipyard.end + 1..shipyard.end + 1 + COUNCIL_LEN;
            if at(buildings.end)? != SEP || at(shipyard.end)? != SEP || at(council.end)? != SEP {
                return Err(WorldError::Layout(t, "separadores de edificios, astillero o concejo"));
            }
            let facilities = council.end + 1..council.end + 1 + FACILITIES_LEN;
            // El resto es de 0x55 + 0x10·nodos bytes y acaba en 0x7F; se localiza la siguiente ciudad (o la matriz) con esa regla.
            let mut end = None;
            for nodes in 0..64 {
                let tail_end = facilities.end + TAIL_FIXED + NODE_LEN * nodes;
                let ok = if t + 1 < count {
                    let next = tail_end + 1;
                    data.get(tail_end) == Some(&SEP)
                        && data.get(next + INDEX_AT).is_some_and(|&i| i as usize == t + 1)
                        && data.get(next + ECONOMY_LEN) == Some(&SEP)
                } else {
                    // La última ciudad no lleva separador final: sigue la matriz count×count y el marcador F1 F2 F3 F4 (desde la versión 0x5A).
                    data.get(tail_end + count * count..tail_end + count * count + 4) == Some(&[0xF1, 0xF2, 0xF3, 0xF4][..])
                };
                if ok {
                    end = Some(tail_end);
                    break;
                }
            }
            let tail_end = end.ok_or(WorldError::Layout(t, "resto de la ciudad"))?;
            towns.push(TownSections {
                economy: eco,
                tavern,
                buildings,
                shipyard,
                council,
                tail: facilities.end..tail_end,
                facilities,
            });
            pos = tail_end + 1;
            after = tail_end;
        }
        let matrix = after..after + count * count;
        // `0x006335A0`: prefijo de 17, número y nodos de acuerdos (8),
        // número y nodos de solicitudes (16), y bandera de escena.
        let start=matrix.end+4;let mut end=start;let mut siege_records=Vec::with_capacity(count);
        for k in 0..count{
            let len=siege_record_len(data.get(end..).ok_or(WorldError::Truncated)?,k)?;
            let next=end.checked_add(len).ok_or(WorldError::Truncated)?;siege_records.push(end..next);end=next;
        }
        let sieges=start..end;
        let businesses = sieges.end..sieges.end + 20 * u16_at(data, 0xA)?;
        if businesses.end > data.len() {
            return Err(WorldError::Truncated);
        }
        let mut offices = Vec::with_capacity(u16_at(data, 0xC)?);
        let mut pos = businesses.end;
        for _ in 0..u16_at(data, 0xC)? {
            let free = data.get(pos..pos + 2).ok_or(WorldError::Truncated)? == [0xFF, 0xFF];
            let len = if free { OFFICE_FREE_LEN } else { OFFICE_FULL_LEN };
            if pos + len > data.len() {
                return Err(WorldError::Truncated);
            }
            offices.push(OfficeSlot { range: pos..pos + len, free });
            pos += len;
        }
        let mut merchants = Vec::with_capacity(u16_at(data, 0xE)?);
        for _ in 0..u16_at(data, 0xE)? {
            let mut len = MERCHANT_FIXED_LEN;
            for k in 0..10 {
                len += 2 * (u16_at(data, pos + MERCHANT_LIST_COUNTS + 2 * k)? & 0x7FFF);
            }
            if pos + len > data.len() {
                return Err(WorldError::Truncated);
            }
            merchants.push(pos..pos + len);
            pos += len;
        }
        let rest = pos..data.len();
        Ok(World { data, towns, after_towns: after, matrix, sieges, siege_records, businesses, offices, merchants, rest })
    }

    /// Registro `k` de 20 bytes de `merchant_business_serialize`.
    pub fn business(&self, k: usize) -> &[u8] {
        &self.data[self.businesses.start + 20 * k..self.businesses.start + 20 * k + 20]
    }

    /// Entrada `k` de la tabla de oficinas: `0x44C` bytes si está en uso o 6 si es una entrada libre.
    pub fn office(&self, k: usize) -> &[u8] {
        &self.data[self.offices[k].range.clone()]
    }

    /// Imagen del objeto `Office` (`0x44C` bytes) de una entrada en uso; `None` si la entrada está libre.
    pub fn office_image(&self, k: usize) -> Option<Vec<u8>> {
        (!self.offices[k].free).then(|| layout::image(&layout::OFFICE, layout::OFFICE_OBJECT_LEN, self.office(k)))
    }

    /// Registro `k` de comerciante (1.552 bytes más listas de barcos).
    pub fn merchant(&self, k: usize) -> &[u8] {
        &self.data[self.merchants[k].clone()]
    }

    /// Imagen del objeto `merchant` (`0x650` bytes); las listas de barcos y los punteros quedan a cero.
    pub fn merchant_image(&self, k: usize) -> Vec<u8> {
        layout::image(&layout::MERCHANT, layout::MERCHANT_OBJECT_LEN, self.merchant(k))
    }

    pub fn merchant_slots(&self) -> usize {
        self.merchants.len()
    }

    pub fn business_count(&self) -> usize {
        self.businesses.len() / 20
    }

    /// Oficinas en uso (entradas completas).
    pub fn office_count(&self) -> usize {
        self.offices.iter().filter(|o| !o.free).count()
    }

    /// Entradas de la tabla de oficinas, libres incluidas.
    pub fn office_slots(&self) -> usize {
        self.offices.len()
    }

    pub fn header(&self) -> &[u8] {
        &self.data[..HEADER_LEN]
    }

    /// Día del año (`GameWorld+4`) y mes (`+1`) de la cabecera.
    pub fn clock(&self) -> (u8, u16) {
        (self.data[1], u16::from_le_bytes([self.data[4], self.data[5]]))
    }

    pub fn town_count(&self) -> usize {
        self.towns.len()
    }

    /// Comerciantes según la cabecera (`+0xA`).
    pub fn merchant_count(&self) -> u16 {
        u16::from_le_bytes([self.data[0xA], self.data[0xB]])
    }

    /// Reconstruye la ciudad con todos los campos que el guardado contiene (versión `0x85`), en su desplazamiento de `Town`.
    ///
    /// Copias contiguas: bloque económico (`+0`), concejo (`+0x6D4`), iglesia (`+0x794`), cabecera del plano (`+0x7A8`), taberna (`+0x82C`) e instalaciones (`+0x840`); campos sueltos de edificios y del resto según `BUILDING_FIELDS` y
    /// `TAIL_FIELDS`. Quedan a cero lo que el original no guarda: puntero de registros (`+0x75C`), celdas del plano (`+0x80C`) y cabeza
    /// de la lista de entradas (`+0x9B4`); los registros, las celdas y las entradas se leen aparte (`building_records`, `plan_cells`,
    /// `tick_entries`). El indicador del plano `+0x7A4` se reconstruye a 1, como en `town_plan_deserialize`.
    pub fn town(&self, i: usize) -> Town {
        let s = &self.towns[i];
        let mut t = Town::default();
        let d = self.data;
        let mut put = |at: usize, src: &[u8]| t.0[at..at + src.len()].copy_from_slice(src);
        put(0, &d[s.economy.clone()]);
        put(ECONOMY_LEN, &d[s.council.clone()]);
        put(0x82C, &d[s.tavern.clone()]);
        let y = &d[s.shipyard.clone()];
        for &(rel, obj, len) in SHIPYARD_FIELDS {
            put(obj, &y[rel..rel + len]);
        }
        put(off::FACILITY, &d[s.facilities.clone()]);
        let b = &d[s.buildings.clone()];
        for &(rel, obj, len) in BUILDING_FIELDS {
            put(obj, &b[rel..rel + len]);
        }
        put(0x794, &b[0x34..0x44]);
        let plan = &d[self.plan_range(s)];
        // town_plan_deserialize (0x004F9770) marca el plano como actualizado.
        put(0x7A4, &1u32.to_le_bytes());
        put(0x7A8, &plan[..0x64]);
        let tail = &d[s.tail.clone()];
        for &(rel, obj, len) in TAIL_FIELDS {
            put(obj, &tail[rel..rel + len]);
        }
        // Reparto intercalado de milicias: `[unidades, reclutas]` por cada una de las cuatro entradas.
        for k in 0..4 {
            put(0x998 + k, &tail[0x18 + 2 * k..0x19 + 2 * k]);
            put(0x99C + k, &tail[0x19 + 2 * k..0x1A + 2 * k]);
        }
        t
    }

    fn plan_range(&self, s: &TownSections) -> Range<usize> {
        let b = s.buildings.start;
        let plan = b + 0x44 + 8 * usize::from(u16::from_le_bytes([self.data[b], self.data[b + 1]]));
        plan..s.buildings.end
    }

    /// Registros de edificios de la ciudad (8 bytes cada uno), sin interpretar.
    pub fn building_records(&self, i: usize) -> &[u8] {
        let s = &self.towns[i];
        &self.data[s.buildings.start + 0x44..self.plan_range(s).start]
    }

    /// Celdas del plano urbano (`ancho · alto` bytes).
    pub fn plan_cells(&self, i: usize) -> &[u8] {
        let r = self.plan_range(&self.towns[i]);
        &self.data[r.start + 0x64..r.end]
    }

    /// Entradas de suministro diario (`town_tick_entry`, 16 bytes en el fichero); su número es el `u16` en `+0x53` del resto.
    pub fn tick_entries(&self, i: usize) -> Vec<TickEntry> {
        let tail = &self.data[self.towns[i].tail.clone()];
        let n = usize::from(u16::from_le_bytes([tail[0x53], tail[0x54]]));
        (0..n)
            .filter_map(|k| tail.get(0x55 + 16 * k..0x55 + 16 * k + 16))
            .map(|e| {
                let u32at = |o: usize| u32::from_le_bytes(e[o..o + 4].try_into().unwrap());
                TickEntry {
                    amount: u32at(0) as i32,
                    sum: u32at(4) as i32,
                    // El escritor guarda los pares `u16` e instalación/ciudad con el orden de cada pareja intercambiado.
                    max: u16::from(e[9]) | (u16::from(e[8]) << 8),
                    factor: u16::from(e[11]) | (u16::from(e[10]) << 8),
                    facility_type: e[12],
                    town: e[13],
                }
            })
            .collect()
    }
}

/// Astillero (`town_shipyard.tsv`): `(posición, desplazamiento en Town, bytes)`; `+0x822..+0x823` no se guardan.
pub(crate) const SHIPYARD_FIELDS: &[(usize, usize, usize)] =
    &[(0x00, 0x810, 12), (0x0C, 0x81C, 2), (0x0E, 0x81E, 2), (0x10, 0x820, 2), (0x12, 0x824, 4), (0x16, 0x828, 4)];

/// Campos sueltos de los datos de edificios: `(posición en el bloque, desplazamiento en Town, bytes)`, de `town_building_data.tsv`.
pub(crate) const BUILDING_FIELDS: &[(usize, usize, usize)] = &[
    (0x00, 0x776, 2),
    (0x02, 0x760, 2),
    (0x04, 0x762, 2),
    (0x06, 0x764, 2),
    (0x08, 0x76A, 2),
    (0x0A, 0x768, 2),
    (0x0C, 0x766, 2),
    (0x0E, 0x774, 1),
    (0x0F, 0x771, 1),
    (0x10, 0x770, 1),
    (0x11, 0x76C, 4),
    (0x15, 0x784, 2),
    (0x17, 0x773, 1),
    (0x18, 0x772, 1),
    (0x19, 0x789, 1),
    (0x1A, 0x788, 1),
    (0x1B, 0x78A, 1),
    (0x1C, 0x78C, 2),
    (0x1E, 0x78E, 2),
    (0x20, 0x786, 2),
    (0x22, 0x790, 2),
    (0x24, 0x792, 2),
    (0x26, 0x775, 1),
    (0x27, 0x78B, 1),
    (0x28, 0x778, 6),
    (0x2E, 0x77E, 6),
];

/// Campos sueltos del resto de la ciudad (`town.tsv`): `(posición desde las instalaciones, desplazamiento en Town, bytes)`.
/// Las cuatro milicias van intercaladas y se copian aparte.
pub(crate) const TAIL_FIELDS: &[(usize, usize, usize)] = &[
    (0x00, 0x9A4, 16),
    (0x10, 0x990, 4),
    (0x14, 0x994, 2),
    (0x16, 0x996, 2),
    (0x20, 0x9A0, 2),
    (0x22, 0x9A2, 2),
    (0x24, 0x9B8, 4),
    (0x28, 0x9BC, 1),
    (0x29, 0x9BD, 1),
    (0x2A, 0x9C0, 4),
    (0x2E, 0x9C4, 1),
    (0x2F, 0x9C6, 18),
    (0x41, 0x9D8, 18),
];
