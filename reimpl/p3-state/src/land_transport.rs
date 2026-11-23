//! Cartas reales y efectos preparados de la tarea 0x0C.
use crate::messages::{self, Payload, RETENTION};
use p3_core::{
    calendar::Calendar,
    economy::land_transport::{self, Env, Event},
    scheduler::TaskRecord,
};
#[derive(Clone)]
pub struct Prepared {
    pub core: land_transport::Prepared,
    pub message_pool: Vec<u8>,
    pub notifications: Vec<[u32; 6]>,
}
/// Los cambios de oficina, comerciante, tarea, cartas y avisos se confirman
/// juntos. La preferencia y el tipo de comerciante los filtra 0x4D6680.
pub fn prepare(task: &TaskRecord, env: &Env<'_>, pool: &[u8], calendar: Calendar) -> Result<Prepared, String> {
    let mut core = land_transport::prepare(task, env)?;
    let mut next = pool.to_vec();
    let mut notifications = Vec::new();
    for event in &core.events {
        match *event {
            Event::Letter {
                owner,
                kind,
                town,
                origin,
                destination,
            } => {
                let merchant = core
                    .merchants
                    .get_mut(usize::from(owner))
                    .ok_or("destinatario de transporte ausente")?;
                let mut record = [0u8; 16];
                record[1] = 0x80;
                record[2..4].fill(0xff);
                record[4] = kind;
                record[5] = town;
                record[8..10].copy_from_slice(&u16::from(origin).to_le_bytes());
                record[10..12].copy_from_slice(&u16::from(destination).to_le_bytes());
                messages::post_to_merchant(
                    &mut next,
                    merchant,
                    calendar.day,
                    calendar.month,
                    calendar.year,
                    &record,
                    Payload::None,
                    &RETENTION,
                )?;
            }
            Event::Notify {
                owner,
                town,
                origin,
                destination,
            } => notifications.push([
                u32::from(owner),
                0,
                0,
                u32::from(town),
                0x29,
                u32::from(origin) | (u32::from(destination) << 8),
            ]),
        }
    }
    Ok(Prepared {
        core,
        message_pool: next,
        notifications,
    })
}
