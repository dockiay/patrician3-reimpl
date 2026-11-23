//! Publicación transaccional de cartas de una escena de guarida vacía.
use crate::pirate_empty_hideout_scene::Prepared;
use p3_core::{
    ai::reputation::Merchant,
    ships::{
        town_attack_finish::{FinishMessage, HumanFinishEffect},
        town_attack_start::EmptyHideoutEffect,
    },
};
use p3_state::messages::{post_message, Payload};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Effect {
    Start(EmptyHideoutEffect),
    /// Llamada a 0x004D6530, incluso si la preferencia descarta la carta.
    Post {
        recipient: u32,
        kind: u8,
        delivered: usize,
    },
    /// 0x004D7B10 para una carta personal efectivamente entregada.
    MailNotice {
        recipient: u32,
        descriptor: [u8; 16],
        text: Option<Vec<u8>>,
    },
    OpenDialog {
        address: u32,
    },
}
#[derive(Clone, Debug)]
pub struct Published {
    pub scene: Prepared,
    pub pool: Vec<u8>,
    pub effects: Vec<Effect>,
}

/// Publica sobre copias: el llamante confirma escena, comerciantes, pool y UI juntos.
/// `retention` contiene la tabla del original; las preferencias están en cada comerciante.
pub fn prepare(
    mut scene: Prepared,
    pool: &[u8],
    merchant_count: u16,
    date: (u8, u8, u16),
    retention: &[u32],
) -> Result<Published, String> {
    if scene.merchants.len() % 0x650 != 0 || scene.merchants.len() / 0x650 < usize::from(merchant_count) {
        return Err("comerciantes truncados".into());
    }
    if !scene.letters.is_empty() && !scene.human_finish_effects.is_empty() {
        return Err("efectos de victoria incompatibles".into());
    }
    let mut merchants = scene
        .merchants
        .chunks_exact(0x650)
        .map(|b| Merchant { bytes: b.to_vec() })
        .collect::<Vec<_>>();
    let mut staged_pool = pool.to_vec();
    let mut effects = scene
        .start_effects
        .iter()
        .cloned()
        .map(Effect::Start)
        .collect::<Vec<_>>();
    fn deliver(
        m: &FinishMessage,
        pool: &mut Vec<u8>,
        merchants: &mut [Merchant],
        count: u16,
        date: (u8, u8, u16),
        retention: &[u32],
        effects: &mut Vec<Effect>,
    ) -> Result<(), String> {
        let recipient = if m.recipient == u32::MAX - 1 {
            -2
        } else {
            i32::try_from(m.recipient).map_err(|_| "destinatario no admitido")?
        };
        let payload = match &m.text {
            Some(text) => Payload::Text(text),
            None => Payload::None,
        };
        let delivered = post_message(
            pool,
            merchants,
            count,
            date.0,
            date.1,
            date.2,
            recipient,
            &m.descriptor,
            payload,
            retention,
        )?;
        effects.push(Effect::Post {
            recipient: m.recipient,
            kind: m.descriptor[4],
            delivered,
        });
        if recipient >= 0 && delivered != 0 {
            let mut descriptor = m.descriptor;
            descriptor[0] = date.0;
            descriptor[1] = date.1 | 0x80;
            descriptor[2..4].copy_from_slice(&date.2.to_le_bytes());
            // El puntero de texto de x86 es transitorio; se entrega por separado.
            if descriptor[4] == 0x29 {
                descriptor[8..12].fill(0)
            }
            effects.push(Effect::MailNotice {
                recipient: m.recipient,
                descriptor,
                text: m.text.clone(),
            });
        }
        Ok(())
    }
    for msg in &scene.letters {
        deliver(
            msg,
            &mut staged_pool,
            &mut merchants,
            merchant_count,
            date,
            retention,
            &mut effects,
        )?
    }
    for effect in &scene.human_finish_effects {
        match effect {
            HumanFinishEffect::Letter(msg) => deliver(
                msg,
                &mut staged_pool,
                &mut merchants,
                merchant_count,
                date,
                retention,
                &mut effects,
            )?,
            HumanFinishEffect::OpenDialog { address } => effects.push(Effect::OpenDialog { address: *address }),
        }
    }
    scene.merchants = merchants.into_iter().flat_map(|m| m.bytes).collect();
    Ok(Published {
        scene,
        pool: staged_pool,
        effects,
    })
}
