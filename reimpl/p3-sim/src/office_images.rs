//! Imágenes vivas de oficina (`0x44C` bytes por oficina) para las pantallas y el puente C08 fuera del despacho de tareas.
//!
//! El puente de tareas (`OfficeTownHost::dispatch_scheduled_tasks_fleet`) deja `scheduled_office_images` vacío cuando ninguna tarea
//! vencida las necesita. Eso ocurre al avanzar un tick o al pedir presupuesto de un barco nuevo en el astillero, y las consultas de
//! oficina y armería dejaban de funcionar (WIN-01 de PRUEBAS.md). Aquí se reconstruyen igual que lo hace el puente.

use crate::Sim;
use p3_save::office::decode_office;

impl Sim {
    /// Si las imágenes faltan o no cubren todas las oficinas, las rehace desde las oficinas persistidas con el enlace `+0x2CA` vivo.
    pub fn ensure_office_images(&mut self) {
        let complete = self.host.scheduled_office_images.as_ref().is_some_and(|v| !v.is_empty() && v.len() == self.persisted.len());
        if complete {
            return;
        }
        let images = self
            .persisted
            .iter()
            .enumerate()
            .map(|(i, office)| {
                let mut image = decode_office(&office.to_save_v85()).to_vec();
                if let Some(live) = self.economy.world.offices.get(i) {
                    image[0x2ca..0x2cc].copy_from_slice(&live.next.to_le_bytes());
                }
                image
            })
            .collect();
        self.host.scheduled_office_images = Some(images);
    }
}
