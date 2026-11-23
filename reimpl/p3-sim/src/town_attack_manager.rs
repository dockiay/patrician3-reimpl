//! Gestor vivo acotado de un asalto urbano vacío ya resuelto.
use crate::battle_manager::{groups_from_save, groups_to_save};
use p3_core::ships::battle::{self, BattleAllocator, BattleSaveInput};

#[derive(Clone)]
pub struct Runtime {
    pub manager: Vec<u8>,
    pub object: Vec<u8>,
    pub groups: Vec<Vec<[u8; 8]>>,
    pub slot: u8,
    pub address: u32,
    pub original_block: Vec<u8>,
    /// Escena poblada aún no ejecutable por el despachador de frames.
    pub populated: Option<p3_save::tail::RebuiltInitialCommercialBattle>,
    /// Solo están contrastados los dos primeros ticks de esta apertura.
    pub populated_ticks: u8,
    /// Colas transitorias reconstruidas tras cargar; aún no despachadas al paso naval.
    pub town_navigation: Option<crate::pirate_active_frame::TownNavigationCaches>,
}

struct Allocator {
    object: Vec<u8>,
    address: u32,
    table: Vec<u8>,
}
impl BattleAllocator for Allocator {
    fn allocate(&mut self, size: u32) -> u32 {
        if size == 0x2f24 {
            self.address
        } else {
            0
        }
    }
    fn construct(&mut self, pointer: u32, slot: u8) -> u32 {
        battle::construct_town_attack_battle(&mut self.object, &mut self.table, pointer, u32::from(slot)).unwrap_or(0)
    }
}
impl Runtime {
    /// La imagen es el contenido real aportado por el asignador antes de la
    /// construcción; el original no inicializa todos sus bytes.
    pub fn allocate(original_block: &[u8], initial_image: &[u8], address: u32) -> Result<Self, String> {
        if initial_image.len() != 0x2f24 || address == 0 || address.checked_add(0x2f24).is_none() {
            return Err("imagen o dirección de asignador inválida".into());
        }
        let frame =
            p3_save::tail::CurrentSeaBattles::parse(original_block).map_err(|e| format!("gestor anterior: {e:?}"))?;
        if !frame.objects.is_empty() {
            return Err("gestor anterior con batalla activa".into());
        }
        let groups = groups_from_save(&frame).map_err(str::to_owned)?;
        let mut manager = vec![0; 0xc40];
        if !groups.is_empty() {
            manager[0xc30..0xc34].copy_from_slice(&1u32.to_le_bytes());
            manager[0xc34..0xc38].copy_from_slice(&(groups.len() as u32).to_le_bytes())
        }
        let mut host = Allocator {
            object: initial_image.to_vec(),
            address,
            table: vec![0; 0x400],
        };
        let slot = battle::allocate_town_attack_battle(&mut manager, &mut host).ok_or("gestor truncado")?;
        if slot == u8::MAX
            || u32::from_le_bytes(
                manager[usize::from(slot) * 4..usize::from(slot) * 4 + 4]
                    .try_into()
                    .unwrap(),
            ) != address
        {
            return Err("reserva de escena fallida".into());
        }
        Ok(Self {
            manager,
            object: host.object,
            groups,
            slot,
            address,
            original_block: original_block.to_vec(),
            populated: None,
            populated_ticks: 0,
            town_navigation: None,
        })
    }
    /// La escena del caller se publica solo si procede de esta misma ranura.
    pub fn replace_object(&mut self, object: &[u8]) -> Result<(), String> {
        if object.len() != 0x2f24
            || u32::from_le_bytes(object[..4].try_into().unwrap()) != 0x67aaa0
            || u16::from_le_bytes(object[0x662..0x664].try_into().unwrap()) != u16::from(self.slot)
        {
            return Err("escena de otro gestor o tipo".into());
        }
        self.object.copy_from_slice(object);
        Ok(())
    }
    /// Bytes que escribiría 0x00611950; no se publican como S05 aquí.
    pub fn serialize_original(&self) -> Result<Vec<u8>, String> {
        if self.manager.len() < 0xc38 || self.slot == u8::MAX || self.address == 0 {
            return Err("gestor de escena truncado".into());
        }
        for i in 0..255usize {
            let p = u32::from_le_bytes(self.manager[i * 4..i * 4 + 4].try_into().unwrap());
            if p != if i == usize::from(self.slot) { self.address } else { 0 } {
                return Err("ranuras de escena desincronizadas".into());
            }
        }
        if u32::from_le_bytes(self.manager[0xc34..0xc38].try_into().unwrap()) as usize != self.groups.len() {
            return Err("listas de escena desincronizadas".into());
        }
        let mut object = self.object.clone();
        if object.len() != 0x2f24 {
            return Err("objeto de escena truncado".into());
        }
        if u32::from_le_bytes(object[..4].try_into().unwrap()) != 0x67aaa0
            || u16::from_le_bytes(object[0x662..0x664].try_into().unwrap()) != u16::from(self.slot)
        {
            return Err("objeto de otra escena".into());
        }
        let word = |at: usize| u16::from_le_bytes([object[at], object[at + 1]]);
        if word(0x14) != 0 || word(0xacc) != 0 || word(0xace) != 0 {
            return Err("listas locales o proyectiles aún no admitidos".into());
        }
        let prefix = battle::town_attack_save_prefix(&mut object).map_err(str::to_owned)?;
        let input = BattleSaveInput {
            object: &object,
            nexts: &[],
            prevs: &[],
            bitmap: &[],
            local_ships: &[],
            projectiles: &[],
        };
        let base = battle::serialize_basic_battle(&input).map_err(str::to_owned)?;
        let mut out = vec![1u8, 1];
        out.extend_from_slice(&prefix);
        out.extend_from_slice(&base);
        let groups = groups_to_save(&self.groups);
        let count = groups.iter().filter(|g| !g.is_empty()).count();
        out.extend_from_slice(&(count as u32).to_le_bytes());
        for (i, g) in groups.iter().enumerate() {
            if g.is_empty() {
                continue;
            }
            out.extend_from_slice(&(i as u32).to_le_bytes());
            out.extend_from_slice(&(g.len() as u32).to_le_bytes());
            for &(a, b) in g {
                out.extend_from_slice(&a.to_le_bytes());
                out.extend_from_slice(&b.to_le_bytes())
            }
        }
        Ok(out)
    }
    /// Hasta reconstruir y resolver el asalto, el lector actual no admite
    /// kind 1 y el lector original puede consumir el bloque siguiente.
    pub fn persist_checked(&self, model: &mut p3_save::model::SaveModel) -> Result<(), String> {
        if model.sea_battles != self.original_block {
            return Err("gestor de batallas modificado por otro sistema".into());
        }
        let _ = self.serialize_original()?;
        Err("guardado de asalto urbano activo no recargable: resolución pendiente".into())
    }
}
