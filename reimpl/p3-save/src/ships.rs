//! Bloque `ships` (S02): gestor de barcos (`0x00504010`/`0x00504230`) reducido a sus registros.
//!
//! Orden observado en los 9 guardados: cabecera de `0x22` bytes (la primera palabra es la versión `0x85` y los contadores de capitanes, barcos y
//! convoyes están en `+0x12`, `+0x14` y `+0x16`: 64, 128 y 32), 64 capitanes de 16 bytes, 128 barcos (en uso: estado menor que `0x17`, núcleo de `0x54`
//! bytes más, si el barco tiene ruta (`+0x36` del núcleo), un contador y 4 bytes por parada, y `0x11E` bytes de resto; libres: estado,
//! siguiente libre y selector, 5 bytes), 32 convoyes de 58 bytes y un resto fijo de 470 bytes (16 registros de 12 bytes, `7F` y las cinco listas
//! de pérdidas de barcos y de convoyes, que no se leen aún).

use crate::layout::{self, Seg};
use std::ops::Range;

pub const HEADER_LEN: usize = 0x22;
pub const CAPTAIN_LEN: usize = 16;
pub const CONVOY_RECORD: usize = 58;
pub const SHIP_FREE_LEN: usize = 5;
pub const SHIP_OBJECT_LEN: usize = 0x180;

#[derive(Debug, PartialEq, Eq)]
pub enum ShipsError {
    Truncated,
    Version(u16),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShipSlot {
    pub range: Range<usize>,
    pub free: bool,
}

/// La dirección guardada es un valor x86 histórico, no un puntero utilizable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SavedRoute {
    None,
    Owned(Vec<(i16, i16)>),
    Shared { origin: u8, destination: u8 },
}

/// Orden de las listas guardadas de barcos en puerto y en el mar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationLists {
    pub docked_head: u16,
    pub sea_head: u16,
    pub docked: Vec<u16>,
    pub sea: Vec<u16>,
}

/// Imagen del gestor para el núcleo nativo. Las rutas deben resolverse antes de
/// ejecutar el tick: los punteros de las imágenes se ponen a cero al cargar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavigationSnapshot {
    pub manager: Vec<u8>,
    pub ships: Vec<u8>,
    pub captains: Vec<u8>,
    pub convoys: Vec<u8>,
    pub routes: Vec<Option<SavedRoute>>,
    pub lists: NavigationLists,
}

impl NavigationSnapshot {
    /// Conecta un descriptor del almacén nativo y devuelve sus puntos. La tabla
    /// compartida la suministra el anfitrión y no se infiere de la dirección x86.
    pub fn bind_route<F>(&mut self, index:usize, token:u32, mut shared:F) -> Result<Vec<(i16,i16)>, &'static str>
    where F:FnMut(u8,u8)->Option<Vec<(i16,i16)>> {
        let route=self.routes.get(index).and_then(Option::as_ref).ok_or("barco libre o inexistente")?;
        let points=match route {
            SavedRoute::None => return Err("barco sin ruta"),
            SavedRoute::Owned(points) => points.clone(),
            SavedRoute::Shared {origin,destination} => shared(*origin,*destination).ok_or("ruta compartida no reconstruida")?,
        };
        if token==0 { return Err("identificador de ruta nulo") }
        if points.is_empty() { return Err("ruta vacía") }
        let start=index.checked_mul(SHIP_OBJECT_LEN).ok_or("índice de barco")?;
        let descriptor=self.ships.get_mut(start+0x24..start+0x28).ok_or("imagen de barco truncada")?;
        if descriptor != [0;4] { return Err("ruta ya vinculada") }
        descriptor.copy_from_slice(&token.to_le_bytes());
        Ok(points)
    }
}

#[derive(Debug)]
pub struct Ships<'a> {
    data: &'a [u8],
    pub captains: Range<usize>,
    pub ships: Vec<ShipSlot>,
    pub convoys: Vec<Range<usize>>,
    /// Lo que sigue a los convoyes: 192 bytes de registros, `7F` y las listas de pérdidas.
    pub tail: Range<usize>,
}

fn u16_at(d: &[u8], o: usize) -> Result<u16, ShipsError> {
    d.get(o..o + 2).map(|s| u16::from_le_bytes([s[0], s[1]])).ok_or(ShipsError::Truncated)
}

/// Aplica cambios de estado de bandas en la cola S02, comprobando la imagen
/// inicial y conservando planos, piezas y campos no relacionados.
pub fn update_pirate_band_statuses(tail:&mut [u8],before:&[Option<p3_core::ai::settlement::MissionPirateBand>;5],after:&[Option<p3_core::ai::settlement::MissionPirateBand>;5])->Result<(),String>{
 let mut staged=tail.to_vec();let mut pos=0xc0usize;
 for i in 0..5{
  let marker=*staged.get(pos).ok_or("banda truncada")?;pos+=1;
  if marker==0{if after[i].is_some(){return Err("banda ausente".into())}continue}
  if marker!=0x7f{return Err("marcador de banda inválido".into())}
  let old=before[i].ok_or("banda inesperada")?;let new=after[i].ok_or("banda eliminada")?;
  if old.target!=new.target{return Err("otros campos de banda modificados".into())}
  let capacity=usize::from(u16_at(&staged,pos).map_err(|_|"banda truncada")?);let first=u16_at(&staged,pos+4).map_err(|_|"banda truncada")?;let target=*staged.get(pos+6).ok_or("banda truncada")?;let status=*staged.get(pos+12).ok_or("banda truncada")?;let reserve=u16_at(&staged,pos+14).map_err(|_|"banda truncada")?;
  if (first!=old.first_ship&&first!=new.first_ship)||target!=old.target||(reserve!=old.reserve_strength&&reserve!=new.reserve_strength)||(status!=old.status&&status!=new.status){return Err("banda desincronizada".into())}
  staged[pos+4..pos+6].copy_from_slice(&new.first_ship.to_le_bytes());staged[pos+12]=new.status;staged[pos+14..pos+16].copy_from_slice(&new.reserve_strength.to_le_bytes());pos=pos.checked_add(16).ok_or("banda truncada")?;let plan=*staged.get(pos).ok_or("plano truncado")?;pos+=1;
  if plan==0x7f{let width=u32::from_le_bytes(staged.get(pos..pos+4).ok_or("plano truncado")?.try_into().unwrap())as usize;let height=u32::from_le_bytes(staged.get(pos+4..pos+8).ok_or("plano truncado")?.try_into().unwrap())as usize;pos=pos.checked_add(100).and_then(|p|width.checked_mul(height).and_then(|n|p.checked_add(n))).ok_or("plano excesivo")?}else if plan!=0{return Err("marcador de plano inválido".into())}
  pos=pos.checked_add(capacity.checked_mul(8).ok_or("banda excesiva")?).ok_or("banda excesiva")?;if pos>staged.len(){return Err("banda truncada".into())}
 }
 tail.copy_from_slice(&staged);Ok(())
}

fn copy(segs: &[Seg], src: &[u8], out: &mut [u8]) {
    for s in segs {
        let (o, n, p) = (usize::from(s.obj), usize::from(s.len), usize::from(s.pos));
        out[o..o + n].copy_from_slice(&src[p..p + n]);
    }
}

impl<'a> Ships<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, ShipsError> {
        let version = u16_at(data, 0)?;
        if version != 0x85 {
            return Err(ShipsError::Version(version));
        }
        let (ncap, nsh, ncv) = (usize::from(u16_at(data, 0x12)?), usize::from(u16_at(data, 0x14)?), usize::from(u16_at(data, 0x16)?));
        let captains = HEADER_LEN..HEADER_LEN + CAPTAIN_LEN * ncap;
        let mut pos = captains.end;
        let mut ships = Vec::with_capacity(nsh);
        for _ in 0..nsh {
            let state = u16_at(data, pos)?;
            let len = if state >= layout::SHIP_FREE_FROM {
                SHIP_FREE_LEN
            } else {
                let core = pos + 2;
                let route = *data.get(core + 0x36).ok_or(ShipsError::Truncated)? != 0;
                let mut n = layout::SHIP_CORE_LEN;
                if route {
                    n += 2 + 4 * usize::from(u16_at(data, core + layout::SHIP_CORE_LEN)?);
                }
                2 + n + layout::SHIP_REST_RECORD_LEN
            };
            if pos + len > data.len() {
                return Err(ShipsError::Truncated);
            }
            ships.push(ShipSlot { range: pos..pos + len, free: state >= layout::SHIP_FREE_FROM });
            pos += len;
        }
        let mut convoys = Vec::with_capacity(ncv);
        for _ in 0..ncv {
            if pos + CONVOY_RECORD > data.len() {
                return Err(ShipsError::Truncated);
            }
            convoys.push(pos..pos + CONVOY_RECORD);
            pos += CONVOY_RECORD;
        }
        Ok(Ships { data, captains, ships, convoys, tail: pos..data.len() })
    }

    pub fn ship_slots(&self) -> usize {
        self.ships.len()
    }

    pub fn ship_count(&self) -> usize {
        self.ships.iter().filter(|s| !s.free).count()
    }

    /// Recorre las dos listas del gestor, comprobando que no hay ciclos,
    /// ranuras libres ni barcos comunes. Un enlace fuera del pool termina la lista,
    /// como en el original (`ships::tick`).
    pub fn navigation_lists(&self) -> Result<NavigationLists, &'static str> {
        let docked_head=u16_at(self.data,8).map_err(|_|"cabecera truncada")?;
        let sea_head=u16_at(self.data,10).map_err(|_|"cabecera truncada")?;
        let mut visited=vec![false;self.ships.len()];
        let mut walk=|head:u16| -> Result<Vec<u16>, &'static str> {
            let mut current=head;let mut result=Vec::new();
            while usize::from(current)<self.ships.len() {
                let index=usize::from(current);
                if visited[index] { return Err("ciclo o barco presente en ambas listas") }
                if self.ships[index].free { return Err("barco libre en lista de navegación") }
                visited[index]=true;result.push(current);
                let image=self.ship_image(index).ok_or("barco no disponible")?;
                current=u16::from_le_bytes([image[6],image[7]]);
            }
            Ok(result)
        };
        let docked=walk(docked_head)?;let sea=walk(sea_head)?;
        Ok(NavigationLists { docked_head,sea_head,docked,sea })
    }

    /// Reconstruye los arrays contiguos y los campos del gestor desde S02.
    /// Reproduce el recálculo de carga del cargador y separa las rutas de sus
    /// direcciones históricas. Ninguna dirección x86 queda activa en el host.
    pub fn navigation_snapshot(&self) -> Result<NavigationSnapshot, &'static str> {
        let lists=self.navigation_lists()?;
        let mut manager=vec![0;0x110];
        const OFFSETS:[usize;16]=[0xe0,0xe2,0xe4,0xe8,0xea,0xec,0xee,0xf0,0xf2,0xf4,0xf6,0xe6,0xfa,0xfc,0xfe,0xf8];
        for (i,&at) in OFFSETS.iter().enumerate() { manager[at..at+2].copy_from_slice(self.data.get(2+2*i..4+2*i).ok_or("cabecera truncada")?); }
        // El cargador original 0x00504230 copia 16 registros de 12 bytes
        // desde la cola y reinicia el temporizador visual de cada uno.
        let weather=self.data.get(self.tail.start..self.tail.start+0xc0).ok_or("meteorología truncada")?;
        manager[0x20..0xe0].copy_from_slice(weather);
        for i in 0..16 {manager[0x2a+i*12..0x2c+i*12].fill(0);}
        let mut ships=vec![0;self.ships.len()*SHIP_OBJECT_LEN];let mut routes=Vec::with_capacity(self.ships.len());
        for i in 0..self.ships.len() {
            let dst=&mut ships[i*SHIP_OBJECT_LEN..(i+1)*SHIP_OBJECT_LEN];
            if let Some(image)=self.ship_image(i) {
                routes.push(Some(self.saved_route(i)?));dst.copy_from_slice(&image);dst[0x24..0x28].fill(0);
                p3_core::ships::cargo::recompute_load(dst);
            } else {
                routes.push(None);let record=self.ship(i);dst[0]=0xff;dst[4..6].fill(0xff);dst[6..8].copy_from_slice(&record[2..4]);dst[0xa..0xe].fill(0xff);dst[0x134..0x136].copy_from_slice(&record[..2]);dst[0x137]=record[4];
            }
        }
        let captains=self.data[self.captains.clone()].chunks_exact(CAPTAIN_LEN).flat_map(|c|crate::captain::decode_v85(c.try_into().expect("capitán completo"))).collect();
        let convoys=(0..self.convoys.len()).flat_map(|i|self.convoy_image(i)).collect();
        Ok(NavigationSnapshot { manager,ships,captains,convoys,routes,lists })
    }

    /// Cabezas de las cinco bandas en el final del gestor. Conserva el plano y
    /// las piezas como bytes opacos, pero valida sus longitudes al recorrerlos.
    pub fn pirate_band_heads(&self)->Result<[Option<u16>;5],ShipsError>{
        Ok(self.pirate_band_metadata()?.0)
    }

    /// Destinos `pirate_band+0xE`, guardados en el séptimo byte del registro.
    pub fn pirate_band_targets(&self)->Result<[Option<u8>;5],ShipsError>{
        Ok(self.pirate_band_metadata()?.1)
    }

    /// Campos de las cinco bandas que usa la preparación de misiones.
    pub fn pirate_band_missions(&self)->Result<[Option<p3_core::ai::settlement::MissionPirateBand>;5],ShipsError>{
        Ok(self.pirate_band_metadata()?.2)
    }

    fn pirate_band_metadata(&self)->Result<([Option<u16>;5],[Option<u8>;5],[Option<p3_core::ai::settlement::MissionPirateBand>;5]),ShipsError>{
        let mut pos=self.tail.start.checked_add(0xc0).ok_or(ShipsError::Truncated)?;
        let mut result=[None;5];let mut targets=[None;5];let mut missions=[None;5];
        for i in 0..5 {
            let marker=*self.data.get(pos).ok_or(ShipsError::Truncated)?;pos+=1;
            if marker==0 {continue}if marker!=0x7f{return Err(ShipsError::Truncated)}
            let capacity=usize::from(u16_at(self.data,pos)?);let first=u16_at(self.data,pos+4)?;let target=*self.data.get(pos+6).ok_or(ShipsError::Truncated)?;let status=*self.data.get(pos+12).ok_or(ShipsError::Truncated)?;let reserve=u16_at(self.data,pos+14)?;
            result[i]=Some(first);targets[i]=Some(target);missions[i]=Some(p3_core::ai::settlement::MissionPirateBand{first_ship:first,target,reserve_strength:reserve,status});pos=pos.checked_add(16).ok_or(ShipsError::Truncated)?;
            let plan=*self.data.get(pos).ok_or(ShipsError::Truncated)?;pos+=1;
            if plan==0x7f {
                let width=u32::from_le_bytes(self.data.get(pos..pos+4).ok_or(ShipsError::Truncated)?.try_into().unwrap()) as usize;
                let height=u32::from_le_bytes(self.data.get(pos+4..pos+8).ok_or(ShipsError::Truncated)?.try_into().unwrap()) as usize;
                pos=pos.checked_add(100).and_then(|p|width.checked_mul(height).and_then(|n|p.checked_add(n))).ok_or(ShipsError::Truncated)?;
            }else if plan!=0{return Err(ShipsError::Truncated)}
            pos=pos.checked_add(capacity.checked_mul(8).ok_or(ShipsError::Truncated)?).ok_or(ShipsError::Truncated)?;
            if pos>self.data.len(){return Err(ShipsError::Truncated)}
        }
        Ok((result,targets,missions))
    }

    pub fn ship(&self, k: usize) -> &[u8] {
        &self.data[self.ships[k].range.clone()]
    }

    /// Paradas de la ruta de un barco en uso (dos `u16` cada una); vacío si no tiene ruta.
    pub fn route(&self, k: usize) -> Vec<[u16; 2]> {
        let r = self.ship(k);
        if self.ships[k].free || r[2 + 0x36] == 0 {
            return Vec::new();
        }
        let n = usize::from(u16::from_le_bytes([r[2 + layout::SHIP_CORE_LEN], r[2 + layout::SHIP_CORE_LEN + 1]]));
        (0..n)
            .map(|i| {
                let o = 2 + layout::SHIP_CORE_LEN + 2 + 4 * i;
                [u16::from_le_bytes([r[o], r[o + 1]]), u16::from_le_bytes([r[o + 2], r[o + 3]])]
            })
            .collect()
    }

    /// Imagen del objeto `ship` (`0x180` bytes) de un barco en uso. `+0x24` conserva
    /// la dirección x86 guardada, que no se puede desreferenciar; la ruta va aparte.
    pub fn ship_image(&self, k: usize) -> Option<Vec<u8>> {
        if self.ships[k].free {
            return None;
        }
        let r = self.ship(k);
        let mut out = vec![0u8; SHIP_OBJECT_LEN];
        copy(&[layout::SHIP_STATE], r, &mut out);
        let route_len = if r[2 + 0x36] != 0 { 2 + 4 * self.route(k).len() } else { 0 };
        copy(&layout::SHIP_CORE, &r[2..], &mut out);
        copy(&layout::SHIP_REST, &r[2 + layout::SHIP_CORE_LEN + route_len..], &mut out);
        Some(out)
    }

    /// Describe la ruta serializada sin interpretar la antigua dirección x86.
    /// Las compartidas requieren reconstruir la tabla de rutas del juego.
    pub fn saved_route(&self, k: usize) -> Result<SavedRoute, &'static str> {
        let slot = self.ships.get(k).ok_or("índice de barco")?;
        if slot.free { return Err("barco libre") }
        let image = self.ship_image(k).ok_or("barco libre")?;
        let pointer = u32::from_le_bytes(image[0x24..0x28].try_into().unwrap());
        if pointer == 0 {
            if image[0x36] != 0 { return Err("ruta propia sin descriptor") }
            return Ok(SavedRoute::None)
        }
        if image[0x36] != 0 {
            return Ok(SavedRoute::Owned(self.route(k).into_iter().map(|[x,y]|(x as i16,y as i16)).collect()))
        }
        Ok(SavedRoute::Shared { origin: image[0x37], destination: image[0x38] })
    }

    /// Reconstruye una imagen con un identificador nativo, nunca la dirección x86.
    /// Las rutas compartidas se resuelven contra la tabla del anfitrión; si esta
    /// no está disponible se rechaza la recarga en lugar de inventar una ruta.
    pub fn rehydrate_route<F>(&self, k: usize, token: u32, mut shared: F) -> Result<(Vec<u8>, Vec<(i16,i16)>), &'static str>
    where F: FnMut(u8,u8)->Option<Vec<(i16,i16)>> {
        let mut image=self.ship_image(k).ok_or("barco libre o índice inexistente")?;
        let points=match self.saved_route(k)? {
            SavedRoute::None => { image[0x24..0x28].fill(0); return Ok((image,Vec::new())); }
            SavedRoute::Owned(points) => points,
            SavedRoute::Shared {origin,destination} => shared(origin,destination).ok_or("ruta compartida no reconstruida")?,
        };
        if token==0 { return Err("identificador de ruta nulo") }
        if points.is_empty() { return Err("ruta vacía") }
        image[0x24..0x28].copy_from_slice(&token.to_le_bytes());
        Ok((image,points))
    }

    pub fn convoy(&self, k: usize) -> &[u8] {
        &self.data[self.convoys[k].clone()]
    }

    /// Imagen del objeto `Convoy` (`0x3C` bytes).
    pub fn convoy_image(&self, k: usize) -> Vec<u8> {
        layout::image(&layout::CONVOY, layout::CONVOY_OBJECT_LEN, self.convoy(k))
    }
}
