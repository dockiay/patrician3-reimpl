//! Conexión de la operación meteorológica 0x78 con la fábrica y el orden C07.
use p3_core::{
    pump::Operation,
    ships::{
        factory::{init_ship_at_town, reset_ship},
        order::XOrder,
    },
};

/// Aplica `op_spawn_weather_entity` a copias del gestor y el vector navales.
/// Devuelve el índice creado; si las 16 ranuras están ocupadas no cambia nada.
/// Los buffers se publican juntos después de validar el estado.
pub(crate) fn spawn(
    manager: &mut Vec<u8>,
    ships: &mut Vec<u8>,
    op: Operation,
    tick: u32,
) -> Result<Option<u16>, String> {
    if op.0[0] != 0x78 {
        return Err("operación meteorológica inválida".into());
    }
    if manager.len() < 0xf6 {
        return Err("gestor meteorológico truncado".into());
    }
    let get = |buf: &[u8], at: usize| u16::from_le_bytes([buf[at], buf[at + 1]]);
    let mut m = manager.clone();
    let mut s = ships.clone();
    let old_count = get(&m, 0xf4);
    let n = usize::from(old_count);
    if s.len() != n * 0x180 {
        return Err("vector meteorológico de barcos truncado".into());
    }
    let Some(slot) = (0..16).find(|&i| get(&m, 0x20 + i * 12) >= old_count) else {
        return Ok(None);
    };
    let mut free = get(&m, 0xe2);
    if free >= old_count {
        if n + 64 > u16::MAX as usize {
            return Err("ampliación naval fuera de rango".into());
        }
        free = old_count;
        s.resize((n + 64) * 0x180, 0);
        for i in n..n + 64 {
            let at = i * 0x180;
            s[at + 4..at + 6].fill(0xff);
            s[at + 6..at + 8].copy_from_slice(&((i + 1) as u16).to_le_bytes());
            s[at + 8..at + 10].fill(0xff);
            s[at + 0x134..at + 0x136].copy_from_slice(&0xffu16.to_le_bytes());
        }
        m[0xf4..0xf6].copy_from_slice(&((n + 64) as u16).to_le_bytes());
    }
    let at = usize::from(free) * 0x180;
    let ship = &mut s[at..at + 0x180];
    let next = get(ship, 6);
    m[0xe2..0xe4].copy_from_slice(&next.to_le_bytes());
    reset_ship(ship, tick).ok_or("barco meteorológico truncado")?;
    ship[0x137] = 0;
    ship[0x134..0x136].copy_from_slice(&0x15u16.to_le_bytes());
    init_ship_at_town(ship, slot as u8, 0, 0, tick).ok_or("barco meteorológico truncado")?;
    ship[0x13b] = slot as u8;
    let x = op.0[3] as u16;
    let y = op.0[4] as u16;
    ship[0x1e..0x20].copy_from_slice(&x.to_le_bytes());
    ship[0x22..0x24].copy_from_slice(&y.to_le_bytes());
    let pos = 0x20 + slot * 12;
    m[pos..pos + 2].copy_from_slice(&free.to_le_bytes());
    m[pos + 2..pos + 4].copy_from_slice(&(op.0[2] as u16).to_le_bytes());
    m[pos + 4..pos + 6].copy_from_slice(&x.to_le_bytes());
    m[pos + 6..pos + 8].copy_from_slice(&y.to_le_bytes());
    m[pos + 8..pos + 10].copy_from_slice(&u16::from(op.0[1] as u8).to_le_bytes());
    m[pos + 10..pos + 12].fill(0);
    let count = get(&m, 0xf4);
    let head = get(&m, 0xea);
    // El original sigue la lista espacial; rechazar ciclos evita bloquear el reloj.
    if head < count {
        for link in [10usize, 12] {
            let mut id = head;
            let mut seen = vec![false; usize::from(count)];
            while id < count {
                let i = usize::from(id);
                if seen[i] {
                    return Err("orden espacial meteorológico cíclico".into());
                }
                seen[i] = true;
                id = get(&s, i * 0x180 + link)
            }
        }
    }
    XOrder {
        ships: &mut s,
        count,
        sea_head: head,
    }
    .insert(free, u32::from(u16::MAX));
    *manager = m;
    *ships = s;
    Ok(Some(free))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn hex(b: &[u8]) -> String {
        b.iter().map(|v| format!("{v:02x}")).collect()
    }
    #[test]
    fn weather_operation_matches_original() {
        let (Ok(exe), Ok(python)) = (std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) else {
            return;
        };
        let (mut cases, mut actual) = (Vec::new(), Vec::new());
        for i in 0..96u32 {
            let mut m = vec![0u8; 0x100];
            let mut ships = vec![0u8; 4 * 0x180];
            m[0xf4..0xf6].copy_from_slice(&4u16.to_le_bytes());
            m[0xe2..0xe4].copy_from_slice(&(if i % 3 == 0 { 4u16 } else { 1 }).to_le_bytes());
            m[0xea..0xec].copy_from_slice(&(if i % 2 == 0 { 0u16 } else { u16::MAX }).to_le_bytes());
            for k in 0..16 {
                let id = if k < (i % 17) as usize { 0u16 } else { u16::MAX };
                m[0x20 + k * 12..0x22 + k * 12].copy_from_slice(&id.to_le_bytes())
            }
            for k in 0..4 {
                let at = k * 0x180;
                ships[at + 4..at + 6].fill(255);
                ships[at + 6..at + 8].copy_from_slice(&((k + 1) as u16).to_le_bytes());
                ships[at + 8..at + 14].fill(255);
                ships[at + 0x134..at + 0x136].copy_from_slice(&(if k == 0 { 0u16 } else { 255u16 }).to_le_bytes())
            }
            ships[0x1e..0x20].copy_from_slice(&(200i16).to_le_bytes());
            let payload = [i % 5, 1000 + i * 31, 50 + i * 7, 400 - i * 3];
            let tick = 0x123400 + i;
            cases.push(serde_json::json!({"manager":hex(&m),"ships":hex(&ships),"payload":payload,"tick":tick}));
            let result = spawn(
                &mut m,
                &mut ships,
                Operation([0x78, payload[0], payload[1], payload[2], payload[3]]),
                tick,
            )
            .unwrap();
            assert_eq!(result.is_none(), i % 17 == 16);
            actual.push(serde_json::json!({"manager":hex(&m),"ships":hex(&ships)}));
        }
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_weather_spawn.py");
        let mut child = std::process::Command::new(python)
            .arg(script)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(serde_json::json!({"binary":exe,"cases":cases}).to_string().as_bytes())
            .unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let expected: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(actual.len(), expected.len());
        for (i, (a, b)) in actual.iter().zip(expected.iter()).enumerate() {
            for name in ["manager", "ships"] {
                let left = a[name].as_str().unwrap();
                let right = b[name].as_str().unwrap();
                if left != right {
                    let j = left
                        .as_bytes()
                        .iter()
                        .zip(right.as_bytes())
                        .position(|(x, y)| x != y)
                        .unwrap_or(0)
                        / 2;
                    panic!(
                        "caso {i} {name} byte {j:#x}: {} frente a {}",
                        &left[j * 2..(j * 2 + 2).min(left.len())],
                        &right[j * 2..(j * 2 + 2).min(right.len())]
                    )
                }
            }
        }
    }
}

impl crate::Sim {
    /// Consume la operación una vez, sincronizando las imágenes que comparte C10.
    pub(crate) fn spawn_weather_operation(&mut self, op: Operation) -> Result<(), String> {
        let mut nav = self
            .host
            .navigation
            .clone()
            .ok_or("flota meteorológica no reconstruida")?;
        let mut factory = self.host.expedition_factory.clone().ok_or("fábrica naval ausente")?;
        let mut before = nav.snapshot.clone();
        let old = before.ships.len() / 0x180;
        if before.ships.len() != old * 0x180
            || before.manager.len() < 0xf6
            || usize::from(u16::from_le_bytes([before.manager[0xf4], before.manager[0xf5]])) != old
            || factory.ships.len() != old
            || factory.free_ship != u16::from_le_bytes([before.manager[0xe2], before.manager[0xe3]])
            || self.economy.world.ships.len() != old
            || self.economy.cargo_ships.len() != old
            || self.host.debt_ship_metadata.len() != old
            || self.host.expedition_ship_images.len() != old
            || self.host.expedition_original_ship_images.len() != old
        {
            return Err("vistas meteorológicas de flota incompatibles".into());
        }
        if self
            .host
            .hanse_pirates
            .as_ref()
            .is_some_and(|p| p.next.len() != old || p.ship_captains.len() != old)
        {
            return Err("vista pirata de flota incompatible".into());
        }
        let Some(id) = spawn(
            &mut nav.snapshot.manager,
            &mut nav.snapshot.ships,
            op,
            self.idle.clock.tick,
        )?
        else {
            return Ok(());
        };
        let count = nav.snapshot.ships.len() / 0x180;
        let index = usize::from(id);
        if index < old {
            let previous = &before.ships[index * 0x180..(index + 1) * 0x180];
            let f = &factory.ships[index];
            if f.len() != 0x180
                || f[6..8] != previous[6..8]
                || f[0x134..0x136] != previous[0x134..0x136]
                || f[0x137] != previous[0x137]
            {
                return Err("ranura libre de fábrica desincronizada".into());
            }
        }
        for i in old..count {
            let mut free = vec![0u8; 0x180];
            free[0] = 0xff;
            free[4..6].fill(0xff);
            free[6..8].copy_from_slice(&((i + 1) as u16).to_le_bytes());
            free[8..14].fill(0xff);
            free[0x134..0x136].copy_from_slice(&255u16.to_le_bytes());
            before.ships.extend_from_slice(&free);
            before.routes.push(None);
            nav.baseline.ships.extend_from_slice(&free);
            nav.baseline.routes.push(None);
            nav.snapshot.routes.push(None);
            factory.ships.push(free);
        }
        nav.snapshot.routes[index] = Some(p3_save::ships::SavedRoute::None);
        factory.free_ship = u16::from_le_bytes([nav.snapshot.manager[0xe2], nav.snapshot.manager[0xe3]]);
        crate::navigation::refresh_lists(&mut nav.snapshot)?;
        let image = nav.snapshot.ships[index * 0x180..(index + 1) * 0x180].to_vec();
        let added = count - old;
        self.economy
            .world
            .ships
            .extend((0..added).map(|_| p3_core::economy::world::Ship {
                owner: 0xff,
                state: 0xff,
            }));
        self.economy.world.ship_count = count as u16;
        self.economy
            .cargo_ships
            .extend((0..added).map(|_| p3_core::economy::ship_trade::ShipCargo {
                owner: 0xff,
                ..Default::default()
            }));
        self.host
            .debt_ship_metadata
            .extend((0..added).map(|_| (u16::MAX, 0xff, 0, u16::MAX)));
        self.host.expedition_ship_images.extend((0..added).map(|_| None));
        self.host
            .expedition_original_ship_images
            .extend((0..added).map(|_| None));
        if let Some(pirates) = self.host.hanse_pirates.as_mut() {
            pirates.next.extend((0..added).map(|_| u16::MAX));
            pirates.ship_captains.extend((0..added).map(|_| u16::MAX));
        }
        self.host.expedition_ship_images[index] = Some(image);
        self.host.expedition_factory = Some(factory);
        nav.events.push("clima:redraw".into());
        self.host.navigation = Some(nav);
        self.host
            .publish_navigation_delta(&before, &mut self.economy.world, &mut self.economy.cargo_ships);
        Ok(())
    }
}
