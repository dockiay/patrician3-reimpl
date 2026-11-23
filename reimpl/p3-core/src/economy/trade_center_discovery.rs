//! Preparación de 0x0052D0B0 sobre imágenes, con efectos explícitos.
//! El anfitrión confirma el centro, prestigio, cartas y avisos juntos.

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Effect {
    Reputation(u8),
    /// Los bytes 6..7 de Letter no los inicializa el original: se entregan
    /// solo los 14 bytes acreditados (0..6 y 8..16), no basura de pila.
    Letter {
        recipient: u32,
        initialized: [u8; 14],
    },
    Notify([u16; 6]),
    Window {
        kind: u32,
        owner: u8,
    },
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prepared {
    pub center: Vec<u8>,
    pub effects: Vec<Effect>,
}
pub struct Context<'a> {
    pub convoys: &'a [u8],
    pub ships: &'a [u8],
    pub captains: &'a [u8],
    pub merchants: &'a [u8],
    pub town_count: u32,
    pub time: u32,
    pub player: u32,
    pub hotseat: bool,
}
fn word(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes([b[p], b[p + 1]])
}
/// Los índices y tablas se validan antes de producir efectos. `Ok(None)` es
/// el rechazo normal del original; contextos corruptos devuelven error.
pub fn prepare(center: &[u8], convoy: u16, ctx: &Context<'_>) -> Result<Option<Prepared>, &'static str> {
    if center.len() != 44
        || ctx.convoys.len() % 0x3c != 0
        || ctx.ships.len() % 0x180 != 0
        || ctx.captains.len() % 16 != 0
        || ctx.merchants.len() % 0x650 != 0
    {
        return Err("imágenes de descubrimiento incompatibles");
    }
    if center[4] >= 0x7f
        || usize::from(convoy) >= ctx.convoys.len() / 0x3c
        || usize::from(center[6]) < ctx.merchants.len() / 0x650
    {
        return Ok(None);
    }
    let cv = &ctx.convoys[usize::from(convoy) * 0x3c..][..0x3c];
    let leader = usize::from(word(cv, 0x10));
    if leader >= ctx.ships.len() / 0x180 {
        return Ok(None);
    }
    let ship = &ctx.ships[leader * 0x180..][..0x180];
    let cap = usize::from(word(ship, 0x42));
    if cap >= ctx.captains.len() / 16 {
        return Ok(None);
    }
    let owner = cv[0];
    let merchant = ctx
        .merchants
        .get(usize::from(owner) * 0x650..(usize::from(owner) + 1) * 0x650)
        .ok_or("dueño de descubrimiento fuera de comerciantes")?;
    if ctx.town_count == 0 {
        return Err("módulo urbano de descubrimiento nulo");
    }
    let captain = &ctx.captains[cap * 16..cap * 16 + 16];
    let mut after = center.to_vec();
    after[6] = owner;
    after[8..12].copy_from_slice(&ctx.time.to_le_bytes());
    let mut letter = [0u8; 14];
    letter[..6].copy_from_slice(&[0, 0x80, 255, 255, 0x1c, (u32::from(ship[0x13a]) % ctx.town_count) as u8]);
    letter[6..].copy_from_slice(&[
        owner, center[4], captain[2], captain[3], center[0], center[1], center[2], center[3],
    ]);
    let mut effects = vec![
        Effect::Reputation(owner),
        Effect::Letter {
            recipient: u32::MAX,
            initialized: letter,
        },
        Effect::Letter {
            recipient: u32::MAX - 1,
            initialized: letter,
        },
    ];
    if word(merchant, 8) == 0 {
        effects.push(Effect::Notify([
            u16::from(owner),
            0,
            0,
            u16::from(merchant[0x19]),
            0x27,
            leader as u16,
        ]));
        if u32::from(owner) == ctx.player || ctx.hotseat {
            effects.push(Effect::Window {
                kind: if center[5] == 2 { 24 } else { 8 },
                owner,
            })
        }
    }
    Ok(Some(Prepared { center: after, effects }))
}
