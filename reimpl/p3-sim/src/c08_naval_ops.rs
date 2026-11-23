//! C08: operaciones de taberna que tocan la tabla de barcos (`0x006DD7A0`): `0x53` aceptar un pasaje
//! (`0x0053C840`) y `0x54` asignar un barco a la ranura de una tarea de guion (`0x0053CA30`).
//!
//! Se siembra el puente plano de C08 más la tabla de barcos (`[0x006DD7A4]`, recuento `0x006DD894`) y las
//! banderas de convoy (`[0x006DD7A8]`, recuento `0x006DD896`), con la imagen viva del barco de la operación
//! (flota de navegación o imágenes de expedición). Al publicar se copian `+0x157` y la marca `+0x3C` a las
//! vistas vivas y a la lista `tavern_changed_ships`, que el guardado escribe; las banderas de convoy siguen
//! el camino de `debt_changed_convoys`. Contraste: `tests/c08_tavern_ships_live_x86.rs`.
use crate::c08_flat::{guarded, SharedMem, OP};
use crate::Sim;
use p3_core::flatmem::Mem;
use p3_core::pump::Operation;

const SHIPS: u32 = 0x6000_0000;
const CAPTAINS: u32 = 0x6200_0000;
const CONVOYS: u32 = 0x6100_0000;
const SHIP_SIZE: usize = 0x180;
const CONVOY_SIZE: usize = 0x3c;

struct Passage<'a> {
    calls: &'a mut crate::c08_flat::Calls,
    mem: SharedMem,
}

impl p3_core::economy::tavern_passage::PassageHost for Passage<'_> {
    fn add_task(&mut self, due: u32) -> u32 {
        let mut mem = self.mem.clone();
        self.calls.call(&mut mem, 0x4d8cf0, 0x006d_d73c, &[due])
    }
    fn send_letter(&mut self, owner: u8, record: &[u8; 16]) {
        // 0x004D6530(0x006DD730, propietario & 0xFF, carta).
        self.calls.letters.push((i32::from(owner), *record));
    }
    fn mark_changed(&mut self, ship: u32, bits: u32) {
        // 0x00517770: `ship+0x3C |= bits` (palabra) y, en convoy válido, `convoy+0x18 |= bits`.
        let mut m = self.mem.clone();
        let v = m.r16(ship + 0x3c) | bits as u16;
        m.w16(ship + 0x3c, v);
        let convoy = m.r16(ship + 8);
        if convoy < m.r16(0x006d_d896) {
            let at = m.r32(0x006d_d7a8) + CONVOY_SIZE as u32 * u32::from(convoy) + 0x18;
            let f = m.r32(at) | (bits & 0xffff);
            m.w32(at, f);
        }
    }
}

impl Sim {
    /// Imagen viva del barco `i`: flota de navegación si existe, si no la imagen de expedición.
    fn c08_live_ship(&self, i: usize) -> Result<Vec<u8>, String> {
        let image = if let Some(nav) = &self.host.navigation {
            nav.snapshot.ships.get(i * SHIP_SIZE..(i + 1) * SHIP_SIZE).map(<[u8]>::to_vec)
        } else {
            self.host.expedition_ship_images.get(i).and_then(Option::clone)
        };
        let image = image.ok_or("barco de taberna sin imagen viva")?;
        if image.len() < SHIP_SIZE {
            return Err("imagen de barco de taberna truncada".into());
        }
        let cargo = self.economy.cargo_ships.get(i).ok_or("carga naval de taberna ausente")?;
        if cargo.dirty != u16::from_le_bytes([image[0x3c], image[0x3d]]) {
            return Err("marca de cambios del barco desincronizada".into());
        }
        Ok(image[..SHIP_SIZE].to_vec())
    }

    /// Siembra tabla de barcos y convoyes; devuelve el índice del barco sembrado (si está en la tabla).
    fn c08_seed_ships(&self, mem: &mut p3_core::flatmem::SparseMem, ship: u16) -> Result<Option<usize>, String> {
        self.c08_seed_ships_with(mem, ship, u16::MAX)
    }

    /// Como [`Self::c08_seed_ships`], con un segundo barco sembrado (órdenes entre dos barcos, como `0x10`).
    fn c08_seed_ships_with(&self, mem: &mut p3_core::flatmem::SparseMem, ship: u16, other: u16) -> Result<Option<usize>, String> {
        let count = usize::from(self.economy.world.ship_count);
        if self.economy.cargo_ships.len() != count {
            return Err("flota de taberna incompleta".into());
        }
        let mut table = vec![0u8; count.max(1) * SHIP_SIZE];
        let seeded = if usize::from(ship) < count {
            let i = usize::from(ship);
            table[i * SHIP_SIZE..(i + 1) * SHIP_SIZE].copy_from_slice(&self.c08_live_ship(i)?);
            Some(i)
        } else {
            None
        };
        if usize::from(other) < count && other != ship {
            let j = usize::from(other);
            table[j * SHIP_SIZE..(j + 1) * SHIP_SIZE].copy_from_slice(&self.c08_live_ship(j)?);
        }
        mem.seed(SHIPS, &table);
        mem.seed(0x006d_d7a4, &SHIPS.to_le_bytes());
        mem.seed(0x006d_d894, &(count as u16).to_le_bytes());
        let flags = &self.host.debt_convoy_flags;
        if let Some(nav) = &self.host.navigation {
            if nav.snapshot.convoys.len() != flags.len() * CONVOY_SIZE {
                return Err("convoyes de taberna incompletos".into());
            }
            for (i, f) in flags.iter().enumerate() {
                if crate::u32_at(&nav.snapshot.convoys, i * CONVOY_SIZE + 0x18) != *f {
                    return Err("convoy de taberna desincronizado".into());
                }
            }
        }
        let mut convoys = vec![0u8; flags.len().max(1) * CONVOY_SIZE];
        // Entradas completas (primer barco `+0xA`, etc.) cuando la vista viva las tiene; las órdenes de carga recorren el convoy del barco.
        if let Some(nav) = &self.host.navigation {
            convoys[..nav.snapshot.convoys.len()].copy_from_slice(&nav.snapshot.convoys);
        } else if self.host.expedition_convoy_images.len() == flags.len() && self.host.expedition_convoy_images.iter().all(|c| c.len() == CONVOY_SIZE) {
            convoys = self.host.expedition_convoy_images.concat();
            convoys.resize(flags.len().max(1) * CONVOY_SIZE, 0);
        }
        for (i, f) in flags.iter().enumerate() {
            convoys[i * CONVOY_SIZE + 0x18..i * CONVOY_SIZE + 0x1c].copy_from_slice(&f.to_le_bytes());
        }
        mem.seed(CONVOYS, &convoys);
        mem.seed(0x006d_d7a8, &CONVOYS.to_le_bytes());
        mem.seed(0x006d_d896, &(flags.len() as u16).to_le_bytes());
        Ok(seeded)
    }

    /// Publica `+0x157` y `+0x3C` del barco y las banderas de convoy que cambiaron.
    fn c08_publish_ship(&mut self, mem: &mut p3_core::flatmem::SparseMem, ship: Option<usize>, crew: bool) -> Result<(), String> {
        let flags = (0..self.host.debt_convoy_flags.len()).map(|i| mem.r32(CONVOYS + (i * CONVOY_SIZE + 0x18) as u32)).collect::<Vec<_>>();
        if let Some(i) = ship {
            let after = mem.read(SHIPS + (i * SHIP_SIZE) as u32, SHIP_SIZE);
            let before = self.c08_live_ship(i)?;
            if let Some(k) = (0..SHIP_SIZE).find(|&k| before[k] != after[k] && !(matches!(k, 0x3c | 0x3d | 0x157) || (crew && matches!(k, 0x3e..=0x43 | 0x114..=0x123 | 0x154 | 0x155)))) {
                return Err(format!("operación de taberna cambia el barco en +{k:#x}"));
            }
            if before != after {
                let write = |b: &mut [u8]| {
                    b[0x3c..0x44].copy_from_slice(&after[0x3c..0x44]);
                    b[0x114..0x124].copy_from_slice(&after[0x114..0x124]);
                    b[0x157] = after[0x157];
                    b[0x154..0x156].copy_from_slice(&after[0x154..0x156]);
                };
                if let Some(b) = self.host.expedition_ship_images.get_mut(i).and_then(Option::as_mut) {
                    write(b)
                }
                if let Some(nav) = self.host.navigation.as_mut() {
                    write(&mut nav.snapshot.ships[i * SHIP_SIZE..(i + 1) * SHIP_SIZE])
                }
                self.economy.cargo_ships[i].dirty = u16::from_le_bytes([after[0x3c], after[0x3d]]);
                if !self.host.tavern_changed_ships.contains(&i) {
                    self.host.tavern_changed_ships.push(i)
                }
            }
        }
        for (i, f) in flags.into_iter().enumerate() {
            if self.host.debt_convoy_flags[i] != f {
                self.host.debt_convoy_flags[i] = f;
                if let Some(nav) = self.host.navigation.as_mut() {
                    nav.snapshot.convoys[i * CONVOY_SIZE + 0x18..i * CONVOY_SIZE + 0x1c].copy_from_slice(&f.to_le_bytes());
                }
                if !self.host.debt_changed_convoys.contains(&i) {
                    self.host.debt_changed_convoys.push(i)
                }
            }
        }
        Ok(())
    }

    /// 0x51: entregar mercancía a un contrato de suministro (`0x0053C050`) con todos los barcos del comerciante sembrados; publica la carga de
    /// los barcos servidos en todas sus vistas, las existencias de la oficina, el dinero del comerciante y la ranura del contrato.
    pub(crate) fn c08_supply_contract(&mut self, op: Operation) -> Result<(), String> {
        self.c08_all_ships_operation(op)
    }

    /// Órdenes que cargan o descargan barcos del comerciante (`0x51` contrato de suministro, `0x4D` oferta del comerciante de la taberna).
    fn c08_all_ships_operation(&mut self, op: Operation) -> Result<(), String> {
        self.c08_merchant_views_synced()?;
        if matches!(op.0[0], 0x08 | 0x1b) {
            // Las existencias vivas están en las oficinas persistidas (compras y ventas de la plaza); la imagen de oficina del puente debe verlas.
            if let Some(images) = self.host.scheduled_office_images.as_mut() {
                for (image, office) in images.iter_mut().zip(self.persisted.iter()) {
                    for w in 0..24 {
                        image[4 + 4 * w..8 + 4 * w].copy_from_slice(&office.stock.stock[w].to_le_bytes());
                        image[0x3b8 + 4 * w..0x3bc + 4 * w].copy_from_slice(&office.stock.average_price[w].to_le_bytes());
                    }
                }
            }
        }
        let mut flat = self.flat_seed(op.0)?;
        self.c08_seed_ships(&mut flat.mem, u16::MAX)?;
        let count = usize::from(self.economy.world.ship_count);
        let mut table = Vec::with_capacity(count * SHIP_SIZE);
        for i in 0..count {
            if self.host.navigation.is_none() && self.economy.world.ships.get(i).is_some_and(|ship|ship.state >= 0x17) {
                // The original array includes free slots. They have no live
                // expedition image, but their loaded factory image still exists.
                // A free slot outside the merchant chain must not reject delivery.
                let image = self.host.expedition_factory.as_ref().and_then(|factory|factory.ships.get(i))
                    .ok_or("imagen de ranura libre de la flota ausente")?;
                if image.len() != SHIP_SIZE || u16::from_le_bytes([image[0x134],image[0x135]]) < 0x17 {
                    return Err("imagen de ranura libre de la flota incompatible".into());
                }
                table.extend_from_slice(image);
            } else {
                table.extend(self.c08_live_ship(i)?);
            }
        }
        flat.mem.seed(SHIPS, &table);
        {
            let (calls, m) = (&mut flat.calls, &mut flat.mem);
            match op.0[0] {
                0x51 => guarded(|| p3_core::economy::ops_misc::deliver_supply_contract(m, calls, OP))?,
                0x50 => guarded(|| p3_core::economy::ops_misc::deal_weapons(m, calls, OP))?,
                0x1d => guarded(|| p3_core::economy::ops_misc::convoy_repair_pay(m, calls, OP))?,
                0x08 => guarded(|| p3_core::economy::small_fns::op_transfer_ware_ship_office(m, calls, OP))?,
                0x1b => guarded(|| p3_core::economy::small_fns::op_move_wares_convoy_office(m, calls, OP))?,
                _ => guarded(|| p3_core::economy::ops_misc::accept_trader_offer(m, calls, OP))?,
            }
        }
        if let Some(e) = flat.calls.error.take() {
            return Err(e);
        }
        let mut mem = std::mem::take(&mut flat.mem);
        let changed: Vec<(usize, Vec<u8>)> = (0..count).filter_map(|i| {
            let after = mem.read(SHIPS + (i * SHIP_SIZE) as u32, SHIP_SIZE);
            (after != table[i * SHIP_SIZE..(i + 1) * SHIP_SIZE]).then_some((i, after))
        }).collect();
        let flags = (0..self.host.debt_convoy_flags.len()).map(|i| mem.r32(CONVOYS + (i * CONVOY_SIZE + 0x18) as u32)).collect::<Vec<_>>();
        let states = (0..self.host.debt_convoy_flags.len()).map(|i| mem.r16(CONVOYS + (i * CONVOY_SIZE + 0x12) as u32)).collect::<Vec<_>>();
        flat.mem = mem;
        let snapshot = (self.host.expedition_ship_images.clone(), self.host.navigation.as_ref().map(|n| (n.snapshot.ships.clone(), n.snapshot.convoys.clone())), self.economy.cargo_ships.clone(), self.host.debt_convoy_flags.clone(), self.host.tavern_changed_ships.clone(), self.host.debt_changed_convoys.clone());
        for (i, after) in &changed {
            if self.economy.world.ships.get(*i).is_some_and(|ship|ship.state >= 0x17) {
                return Err(format!("la operación cambia la ranura libre del barco {i}"));
            }
            let before = &table[i * SHIP_SIZE..(i + 1) * SHIP_SIZE];
            // Solo cambian la carga (+0x54..0xB4), lo que `recompute_load` deriva de ella y las marcas de cambios.
            if let Some(k) = (0..SHIP_SIZE).find(|&k| before[k] != after[k] && !matches!(k, 0x3c | 0x3d | 0x54..=0x113 | 0x114..=0x12f)) {
                return Err(format!("la entrega al contrato cambia el barco {i} en +{k:#x}"));
            }
            if let Some(b) = self.host.expedition_ship_images.get_mut(*i).and_then(Option::as_mut) {
                b.copy_from_slice(after);
            }
            if let Some(nav) = self.host.navigation.as_mut() {
                nav.snapshot.ships[i * SHIP_SIZE..(i + 1) * SHIP_SIZE].copy_from_slice(after);
            }
            let cargo = &mut self.economy.cargo_ships[*i];
            for w in 0..24 {
                cargo.stock[w] = i32::from_le_bytes(after[0x54 + 4 * w..0x58 + 4 * w].try_into().unwrap());
                cargo.average_price[w] = f32::from_le_bytes(after[0xb4 + 4 * w..0xb8 + 4 * w].try_into().unwrap());
            }
            cargo.recompute_load();
            cargo.dirty = u16::from_le_bytes([after[0x3c], after[0x3d]]);
            if !self.host.tavern_changed_ships.contains(i) {
                self.host.tavern_changed_ships.push(*i);
            }
        }
        for (i, f) in flags.into_iter().enumerate() {
            if self.host.debt_convoy_flags[i] != f {
                self.host.debt_convoy_flags[i] = f;
                if let Some(nav) = self.host.navigation.as_mut() {
                    nav.snapshot.convoys[i * CONVOY_SIZE + 0x18..i * CONVOY_SIZE + 0x1c].copy_from_slice(&f.to_le_bytes());
                }
                if !self.host.debt_changed_convoys.contains(&i) {
                    self.host.debt_changed_convoys.push(i)
                }
            }
        }
        // Estado de entrada naval (`+0x12`) que la orden 0x1D pone en reparación.
        for (i, st) in states.into_iter().enumerate() {
            let at = i * CONVOY_SIZE + 0x12;
            if let Some(nav) = self.host.navigation.as_mut() {
                if nav.snapshot.convoys.len() >= at + 2 && nav.snapshot.convoys[at..at + 2] != st.to_le_bytes() {
                    nav.snapshot.convoys[at..at + 2].copy_from_slice(&st.to_le_bytes());
                    if !self.host.debt_changed_convoys.contains(&i) {
                        self.host.debt_changed_convoys.push(i)
                    }
                }
            } else if let Some(f) = self.host.expedition_factory.as_mut() {
                if let Some(e) = f.convoys.get_mut(i).filter(|e| e.len() >= 0x14) {
                    e[0x12..0x14].copy_from_slice(&st.to_le_bytes());
                }
            }
        }
        let offices_before = self.host.scheduled_office_images.clone();
        if let Err(e) = self.flat_publish(flat) {
            let (images, nav, cargo, flags, changed, convoys) = snapshot;
            self.host.expedition_ship_images = images;
            if let (Some(n), Some((ships, c))) = (self.host.navigation.as_mut(), nav) {
                n.snapshot.ships = ships;
                n.snapshot.convoys = c;
            }
            self.economy.cargo_ships = cargo;
            self.host.debt_convoy_flags = flags;
            self.host.tavern_changed_ships = changed;
            self.host.debt_changed_convoys = convoys;
            return Err(e);
        }
        // Las existencias de oficina que cambió la entrega, a las demás vistas de oficina.
        if let (Some(before), Some(after)) = (offices_before.as_ref(), self.host.scheduled_office_images.clone()) {
            for (i, (b, a)) in before.iter().zip(after.iter()).enumerate() {
                for w in 0..24 {
                    let at = 4 + 4 * w;
                    if b[at..at + 4] != a[at..at + 4] {
                        let v = i32::from_le_bytes(a[at..at + 4].try_into().unwrap());
                        self.persisted[i].stock.stock[w] = v;
                        self.economy.offices[i].stock[w] = v;
                        // `OfficeInfo::add_stock` recalcula también el precio medio (`+0x3B8`).
                        if let Ok(avg) = <[u8; 4]>::try_from(&a[0x3b8 + 4 * w..0x3bc + 4 * w]) {
                            let v = f32::from_le_bytes(avg);
                            self.persisted[i].stock.average_price[w] = v;
                            self.economy.offices[i].average_price[w] = v;
                        }
                        if let Some(pool) = self.host.office_pool.as_mut() {
                            pool.records[i][at..at + 4].copy_from_slice(&a[at..at + 4]);
                            pool.records[i][0x3b8 + 4 * w..0x3bc + 4 * w].copy_from_slice(&a[0x3b8 + 4 * w..0x3bc + 4 * w]);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// 0x53 y 0x54 sobre el puente plano con la tabla de barcos.
    pub(crate) fn c08_tavern_ship_operation(&mut self, op: Operation) -> Result<(), String> {
        self.c08_merchant_views_synced()?;
        let mut flat = self.flat_seed(op.0)?;
        // `0x10` (marineros entre dos barcos) siembra también el destino `op+8`.
        let other = if matches!(op.0[0], 0x10 | 0x11) { op.0[2] as u16 } else { u16::MAX };
        let ship = self.c08_seed_ships_with(&mut flat.mem, if op.0[0] == 0x06 { op.0[2] } else { op.0[1] } as u16, other)?;
        let other_index = (usize::from(other) < usize::from(self.economy.world.ship_count)).then_some(usize::from(other));
        let mut captains_seeded = None;
        if op.0[0] == 0x06 {
            // Tabla de capitanes en `[0x006DD7A0]`; el recuento `0x006DD892` ya viene del puente.
            let factory = self.host.expedition_factory.as_ref().ok_or("pool de capitanes ausente")?;
            let table = factory.captains.concat();
            flat.mem.seed(CAPTAINS, &table);
            flat.mem.seed(0x006d_d7a0, &CAPTAINS.to_le_bytes());
            captains_seeded = Some(table);
        }
        if op.0[0] == 0x06 {
            // `assign_captain_to_ship` lee barco `op+8`, carta `op+4` y ciudad `op+0xC`: los argumentos de la pantalla ya vienen en ese orden.
            let calls = &mut flat.calls;
            let m = &mut flat.mem;
            guarded(|| p3_core::economy::ops_misc::assign_captain_to_ship(m, calls, OP))?;
        } else if op.0[0] == 0x04 {
            let calls = &mut flat.calls;
            let m = &mut flat.mem;
            guarded(|| p3_core::economy::ops_ship::hire_sailors(m, calls, OP))?;
        } else if op.0[0] == 0x05 {
            let calls = &mut flat.calls;
            let m = &mut flat.mem;
            guarded(|| p3_core::economy::ops_ship::dismiss_sailors(m, calls, OP))?;
        } else if op.0[0] == 0x11 {
            let calls = &mut flat.calls;
            let m = &mut flat.mem;
            guarded(|| p3_core::economy::ops_ship::transfer_cutlasses(m, calls, OP))?;
        } else if op.0[0] == 0x10 {
            let calls = &mut flat.calls;
            let m = &mut flat.mem;
            guarded(|| p3_core::economy::ops_ship::transfer_sailors(m, calls, OP))?;
        } else if op.0[0] == 0x54 {
            let cleanups = {
                let m = &mut flat.mem;
                let mut out = 0;
                guarded(|| out = p3_core::economy::letters::op_tavern_assign_ship_to_task_slot(m, OP))?;
                out
            };
            if cleanups != 0 {
                // 0x0051AAB0 sobre la carta tomada, que queda a la cabeza de la lista libre del buzón.
                let entry = flat.mem.r32(0x006d_d730) + 16 * u32::from(flat.mem.r16(0x006d_d734));
                crate::c08_flat::reset_letter(&mut flat.mem, entry);
            }
        } else {
            let shared = SharedMem(std::rc::Rc::new(std::cell::RefCell::new(std::mem::take(&mut flat.mem))));
            let result = {
                let mut host = Passage { calls: &mut flat.calls, mem: shared.clone() };
                let mut mem = shared.clone();
                guarded(|| p3_core::economy::tavern_passage::accept_request(&mut mem, &mut host, OP))
            };
            flat.mem = std::rc::Rc::try_unwrap(shared.0).map_err(|_| "memoria del puente aún compartida")?.into_inner();
            result?;
        }
        if let Some(e) = flat.calls.error.take() {
            return Err(e);
        }
        // Se valida el barco antes de publicar nada del puente: un fallo deja la operación sin efectos.
        let mut mem = std::mem::take(&mut flat.mem);
        let snapshot = (self.host.expedition_ship_images.clone(), self.host.navigation.as_ref().map(|n| (n.snapshot.ships.clone(), n.snapshot.convoys.clone())), self.economy.cargo_ships.clone(), self.host.debt_convoy_flags.clone(), self.host.tavern_changed_ships.clone(), self.host.debt_changed_convoys.clone());
        let crew_op = matches!(op.0[0], 0x04 | 0x05 | 0x06 | 0x10 | 0x11);
        self.c08_publish_ship(&mut mem, ship, crew_op)?;
        if let Some(second) = other_index {
            self.c08_publish_ship(&mut mem, Some(second), crew_op)?;
        }
        let captains_after = captains_seeded.as_ref().map(|old| (old.clone(), mem.read(CAPTAINS, old.len())));
        flat.mem = mem;
        if let Err(e) = self.flat_publish(flat) {
            let (images, nav, cargo, flags, changed, convoys) = snapshot;
            self.host.expedition_ship_images = images;
            if let (Some(n), Some((ships, c))) = (self.host.navigation.as_mut(), nav) {
                n.snapshot.ships = ships;
                n.snapshot.convoys = c;
            }
            self.economy.cargo_ships = cargo;
            self.host.debt_convoy_flags = flags;
            self.host.tavern_changed_ships = changed;
            self.host.debt_changed_convoys = convoys;
            return Err(e);
        }
        if let Some((before, after)) = captains_after {
            self.c08_publish_captains(&before, &after)?;
        }
        Ok(())
    }

    /// Publica la tabla de capitanes que cambió una orden de taberna en las vistas vivas (fábrica, navegación y piratas), con la misma
    /// comprobación de sincronía que los demás caminos de capitanes.
    pub(crate) fn c08_publish_captains(&mut self, before: &[u8], after: &[u8]) -> Result<(), String> {
        if before == after {
            return Ok(());
        }
        let factory = self.host.expedition_factory.as_mut().ok_or("pool de capitanes ausente")?;
        if factory.captains.len() * 16 != after.len() {
            return Err("tabla de capitanes con otro tamaño".into());
        }
        let mut pirates = self.host.hanse_pirates.clone();
        if let Some(p) = pirates.as_mut() {
            if p.captains.len() != factory.captains.len() {
                return Err("vistas piratas de capitanes truncadas".into());
            }
            for (i, old) in factory.captains.iter().enumerate() {
                for j in 0..16 {
                    let new = after[i * 16 + j];
                    if old[j] != new {
                        if p.captains[i][j] != old[j] && p.captains[i][j] != new {
                            return Err("capitán pirata desincronizado".into());
                        }
                        p.captains[i][j] = new;
                    }
                }
            }
        }
        for (i, c) in factory.captains.iter_mut().enumerate() {
            c.copy_from_slice(&after[i * 16..(i + 1) * 16]);
        }
        let flat_table = factory.captains.concat();
        if let Some(nav) = self.host.navigation.as_mut() {
            nav.snapshot.captains = flat_table;
        }
        self.host.hanse_pirates = pirates;
        Ok(())
    }
}

impl Sim {
    /// 0x2F (`Town::cancel_ship_order`, `0x0052B110`): retira un encargo de la cola del astillero, devuelve el 90 % si la obra no empezó y envía la carta 0x14.
    /// El puente plano ejecuta la función real con la tabla de barcos sembrada; el barco retirado (`0x00507FB0`) se aplica después sobre la vista naval.
    pub(crate) fn c08_cancel_ship_order(&mut self, op: Operation) -> Result<(), String> {
        use p3_core::ships::remove::{RemoveEvent, RemoveWorld, MERCHANT_SIZE, TOWN_SIZE};
        self.c08_merchant_views_synced()?;
        let mut flat = self.flat_seed(op.0)?;
        self.c08_seed_ships(&mut flat.mem, u16::MAX)?;
        let count = usize::from(self.economy.world.ship_count);
        let mut table = Vec::with_capacity(count * SHIP_SIZE);
        for i in 0..count {
            // Los barcos libres no tienen imagen viva; la cadena de la ciudad solo pasa por barcos vivos.
            table.extend(self.c08_live_ship(i).unwrap_or_else(|_| vec![0u8; SHIP_SIZE]));
        }
        flat.mem.seed(SHIPS, &table);
        {
            let (calls, m) = (&mut flat.calls, &mut flat.mem);
            guarded(|| p3_core::economy::ops_misc::cancel_ship_order_op(m, calls, OP))?;
        }
        let removed = std::mem::take(&mut flat.calls.removed_ships);
        self.flat_publish(flat)?;
        if removed.is_empty() {
            return Ok(());
        }
        let nav = self.host.navigation.as_ref().ok_or("navegación ausente para retirar el barco")?;
        let before = nav.snapshot.clone();
        let mut staged = nav.clone();
        let merchant_count = usize::from(self.economy.world.merchant_count);
        let mut merchants = self.rep_merchants.clone();
        let mut town_bytes = self.economy.world.towns.iter().flat_map(|t| t.0.iter().copied()).collect::<Vec<_>>();
        let mut merchant_bytes = merchants.iter().flat_map(|m| m.bytes.iter().copied()).collect::<Vec<_>>();
        struct NoRoute;
        impl p3_core::ships::route::RouteHost for NoRoute {
            fn points_of(&mut self, _: u32) -> u32 {
                0
            }
            fn free(&mut self, _: u32) {}
            fn ui_route_cleared(&mut self, _: u16) {}
        }
        for ship in removed {
            let snapshot = &mut staged.snapshot;
            let mut remove = RemoveWorld {
                manager: &mut snapshot.manager,
                ships: &mut snapshot.ships,
                captains: &mut snapshot.captains,
                convoys: &mut snapshot.convoys,
                merchants: &mut merchant_bytes,
                towns: &mut town_bytes,
                pool_limit: 0,
                merchant_count: merchant_count as u16,
                town_count: self.economy.world.towns.len() as u32,
                ui_a: false,
                ui_b: false,
            };
            let mut world = p3_core::ships::tick::TickWorld { remove: &mut remove, time: self.idle.clock.tick, player: self.economy.world.local_merchant };
            for event in p3_core::ships::remove::remove_ship(world.remove, ship) {
                match event {
                    RemoveEvent::MerchantChanged(_) | RemoveEvent::UiA(_) | RemoveEvent::UiB(_) => {}
                    // Un barco en cola no tiene ruta que liberar; la función original igualmente la consulta.
                    RemoveEvent::ClearRoute(i) => {
                        let at = i as usize * SHIP_SIZE;
                        if u32::from_le_bytes(world.remove.ships[at + 0x24..at + 0x28].try_into().unwrap()) != 0 {
                            return Err("el encargo retirado tiene ruta".into());
                        }
                        p3_core::ships::route::clear_route(&mut world.remove.ships[at..at + SHIP_SIZE], i as u16, world.player, false, &mut NoRoute);
                    }
                    RemoveEvent::OrderRemove(i) => p3_core::ships::order::remove_from_tick(&mut world, i as u16),
                    other => return Err(format!("el encargo retirado exige un efecto no conectado: {other:?}")),
                }
            }
        }
        for (dst, src) in merchants.iter_mut().zip(merchant_bytes.chunks_exact(MERCHANT_SIZE)) {
            dst.bytes.copy_from_slice(src);
        }
        for (dst, src) in self.economy.world.towns.iter_mut().zip(town_bytes.chunks_exact(TOWN_SIZE)) {
            dst.0.copy_from_slice(src);
        }
        self.c08_publish_merchants(merchants);
        self.host.navigation = Some(staged);
        self.host.publish_navigation_delta(&before, &mut self.economy.world, &mut self.economy.cargo_ships);
        Ok(())
    }
}

impl Sim {
    /// 0x4E (`op_hire_captain`, `0x0053B850`): el jugador contrata al capitán pirata de la taberna para su barco; el barco pasa a la banda pirata viva más cercana.
    /// La función real corre sobre el puente plano con la tabla de barcos, capitanes y bandas sembradas; después se publican el barco (con su ruta, calculada con el
    /// planificador hacia el puerto de su ciudad), la lista de capitanes, la banda, la poda de los grupos del comerciante y la reputación.
    /// Límites: sin banda viva el original crea una nueva (`0x513D80`) y esta rama devuelve un error; el destino de la ruta es el puerto de la ciudad (`ports`), no
    /// la proyección de las celdas del mapa `[0x6CBDC8]` que usa el original, que la sesión no conserva.
    pub(crate) fn c08_hire_pirate_captain(&mut self, op: Operation) -> Result<(), String> {
        use p3_core::route::finder::RouteOut;
        const BANDS: u32 = 0x6300_0000;
        const MAP: u32 = 0x6400_0000;
        self.c08_merchant_views_synced()?;
        let bands = self.c08_event_bands()?;
        let locations = *self.host.settlement_locations.as_ref().ok_or("ubicaciones de banda ausentes")?;
        let town_ids = self.host.expedition_town_ids.clone();
        let ports = self.host.expedition_resources.as_ref().ok_or("puertos de expedición ausentes")?.ports.clone();
        let mut planner = self.host.navigation_planner.clone().ok_or("planificador naval ausente")?;
        let caps_before = self.host.expedition_factory.as_ref().ok_or("pool de capitanes ausente")?.captains.concat();
        let mut flat = self.flat_seed(op.0)?;
        self.c08_seed_ships(&mut flat.mem, u16::MAX)?;
        let count = usize::from(self.economy.world.ship_count);
        let mut table = Vec::with_capacity(count * SHIP_SIZE);
        for i in 0..count {
            table.extend(self.c08_live_ship(i).unwrap_or_else(|_| vec![0u8; SHIP_SIZE]));
        }
        flat.mem.seed(SHIPS, &table);
        flat.mem.seed(CAPTAINS, &caps_before);
        flat.mem.seed(0x006d_d7a0, &CAPTAINS.to_le_bytes());
        for k in 0..5u32 {
            let band = bands.get(k as usize).and_then(Option::as_ref);
            flat.mem.seed(0x006d_d7ac + 4 * k, &(if band.is_some() { BANDS + k * 0x40 } else { 0 }).to_le_bytes());
            if let Some(b) = band {
                flat.mem.seed(BANDS + k * 0x40, &[0u8; 0x40]);
                flat.mem.seed(BANDS + k * 0x40 + 8, &b.header);
            }
        }
        flat.mem.seed(0x006c_e6d0, &0u32.to_le_bytes());
        flat.mem.seed(0x006c_c950, &0u32.to_le_bytes());
        flat.mem.seed(0x006d_e4b8, &town_ids);
        for (i, l) in locations.iter().enumerate() {
            flat.mem.seed(0x006d_dbb0 + 0x34 * i as u32, &[0u8; 8]);
            flat.mem.w32(0x006d_dbb0 + 0x34 * i as u32, l.x as u32);
            flat.mem.w32(0x006d_dbb0 + 0x34 * i as u32 + 4, l.y as u32);
        }
        // Celdas del mapa: la ruta usa el puerto de la ciudad, así que solo hacen falta lecturas válidas.
        flat.mem.seed(0x006c_bdc8, &MAP.to_le_bytes());
        flat.mem.seed(MAP, &[0u8; 0x400]);
        flat.mem.w32(MAP + 0x24, MAP + 0x100);
        for t in 0..40u32 {
            flat.mem.w32(MAP + 0x100 + 4 * t, MAP + 0x200 + 8 * t);
        }
        flat.mem.seed(0x0066_e0d0, &3536u32.to_le_bytes());
        flat.mem.seed(0x0067_2dd8, &256u16.to_le_bytes());
        {
            let (calls, m) = (&mut flat.calls, &mut flat.mem);
            guarded(|| p3_core::economy::ops_misc::hire_pirate_captain(m, calls, OP))?;
        }
        if let Some(e) = flat.calls.error.take() {
            return Err(e);
        }
        let hire = std::mem::take(&mut flat.calls.hire);
        let mut mem = std::mem::take(&mut flat.mem);
        let ships_after = mem.read(SHIPS, count * SHIP_SIZE);
        let caps_after = mem.read(CAPTAINS, caps_before.len());
        let headers: Vec<Option<[u8; 16]>> = (0..5u32).map(|k| bands.get(k as usize).and_then(Option::as_ref).map(|_| <[u8; 16]>::try_from(mem.read(BANDS + k * 0x40 + 8, 16)).unwrap())).collect();
        flat.mem = mem;
        let nav = self.host.navigation.as_ref().ok_or("flota naval ausente")?;
        let before = nav.snapshot.clone();
        let mut staged = nav.clone();
        let changed: Vec<usize> = (0..count).filter(|&i| ships_after[i * SHIP_SIZE..(i + 1) * SHIP_SIZE] != table[i * SHIP_SIZE..(i + 1) * SHIP_SIZE]).collect();
        for &i in &changed {
            staged.snapshot.ships[i * SHIP_SIZE..(i + 1) * SHIP_SIZE].copy_from_slice(&ships_after[i * SHIP_SIZE..(i + 1) * SHIP_SIZE]);
        }
        if let Some((ship, at_call)) = hire.route {
            let i = ship as usize;
            let town = usize::from(at_call[0x39]);
            let end = ports.get(town).copied().flatten().ok_or("puerto de la ciudad del barco ausente")?;
            let end = (end.0 as i16, end.1 as i16);
            let start = (i16::from_le_bytes([at_call[0x1e], at_call[0x1f]]), i16::from_le_bytes([at_call[0x22], at_call[0x23]]));
            let points = match planner.find_route(&[start, end]).route {
                RouteOut::Built(v) => v,
                _ => return Err("ruta del barco contratado no disponible".into()),
            };
            let mut routed = at_call.clone();
            p3_core::ships::route::apply_route_plan(&mut routed, None, end, &points, true, 0x10000 + ship * 16, self.idle.clock.tick).ok_or("ruta del barco contratado inválida")?;
            let image = &mut staged.snapshot.ships[i * SHIP_SIZE..(i + 1) * SHIP_SIZE];
            for range in [2..4, 0x1c..0x1e, 0x20..0x22, 0x24..0x30, 0x34..0x37, 0x38..0x39, 0x48..0x4c] {
                image[range.clone()].copy_from_slice(&routed[range]);
            }
            *staged.snapshot.routes.get_mut(i).ok_or("tabla de rutas incompleta")? = Some(p3_save::ships::SavedRoute::Owned(points));
        }
        self.flat_publish(flat)?;
        let count_m = usize::from(self.economy.world.merchant_count);
        for o in hire.pruned {
            let o = o as usize;
            if o >= count_m {
                continue;
            }
            let mut lists = self.host.merchant_ship_groups.get(o).ok_or("grupos navales del comerciante ausentes")?.clone();
            let owners: Vec<u8> = (0..count).map(|i| staged.snapshot.ships[i * SHIP_SIZE]).collect();
            let merchant = &mut self.rep_merchants[o];
            let mut m = merchant.clone();
            m.prune_foreign_ships_from_groups(o as u8, &mut lists, &|id| owners.get(usize::from(id)).copied().unwrap_or(0xff));
            let mut all = self.rep_merchants.clone();
            all[o] = m;
            self.c08_publish_merchants(all);
            if self.host.merchant_ship_groups[o] != lists {
                self.host.merchant_ship_groups[o] = lists;
                if !self.host.merchant_ship_groups_changed.contains(&o) {
                    self.host.merchant_ship_groups_changed.push(o);
                }
            }
        }
        self.host.navigation = Some(staged);
        self.host.publish_navigation_delta(&before, &mut self.economy.world, &mut self.economy.cargo_ships);
        self.c08_publish_captains(&caps_before, &caps_after)?;
        if let Some(images) = self.host.pirate_band_images.as_mut() {
            for (k, h) in headers.into_iter().enumerate() {
                if let (Some(h), Some(Some(band))) = (h, images.get_mut(k)) {
                    band.header = h;
                    if let Some(v) = self.host.mission_pirates.as_mut() {
                        v[k] = Some(p3_core::ai::settlement::MissionPirateBand { first_ship: u16::from_le_bytes([h[4], h[5]]), target: h[6], status: h[12], reserve_strength: u16::from_le_bytes([h[14], h[15]]) });
                    }
                    if let Some(v) = self.host.pirate_targets.as_mut() {
                        v[k] = Some(h[6]);
                    }
                }
            }
        }
        Ok(())
    }
}
