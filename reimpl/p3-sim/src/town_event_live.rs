//! Publicación preparada de una creación pirata en las vistas de navegación.
//!
//! Las copias devueltas solo deben publicarse junto con la tarea, las cartas y
//! las bandas. Ninguna entrada se modifica si falla una validación.
use crate::{navigation::{self, LiveVoyage}, town_pirate_spawn};
use p3_core::{economy::hanse_petition::PiratePortraits, ships::factory::HanseFactory};
use p3_save::ships::SavedRoute;

pub struct Prepared {
    pub ship: u16,
    pub navigation: LiveVoyage,
    pub factory: HanseFactory,
    pub portraits: PiratePortraits,
}

fn word(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
}

/// Integra una salida ya contrastada de `5063C0` en copias de las vistas.
/// El barco asignado puede proceder de una ranura libre o de un pool ampliado.
pub fn prepare(
    navigation: &LiveVoyage,
    factory: &HanseFactory,
    portraits: &PiratePortraits,
    fleet: &town_pirate_spawn::Prepared,
) -> Result<Prepared, String> {
    let old = &navigation.snapshot;
    if old.manager.len() < 0xf8 || fleet.manager.len() != old.manager.len()
        || old.ships.len() % 0x180 != 0 || old.captains.len() % 16 != 0
        || old.convoys.len() % 0x3c != 0 || fleet.ships.len() % 0x180 != 0
        || fleet.captains.len() % 16 != 0 || fleet.convoys.len() % 0x3c != 0 {
        return Err("imágenes navales truncadas".into());
    }
    let (ships, caps, convoys) = (old.ships.len() / 0x180, old.captains.len() / 16, old.convoys.len() / 0x3c);
    let (new_ships, new_caps, new_convoys) = (fleet.ships.len() / 0x180, fleet.captains.len() / 16, fleet.convoys.len() / 0x3c);
    if factory.ships.len() != ships || factory.captains.len() != caps || factory.convoys.len() != convoys
        || portraits.next.len() != ships || portraits.ship_captains.len() != ships || portraits.captains.len() != caps
        || old.routes.len() != ships || navigation.baseline.ships.len() != old.ships.len()
        || navigation.baseline.routes.len() != ships || navigation.baseline.manager.len() < 0xf8
        || navigation.baseline.convoys.len() != old.convoys.len()
        || fleet.bands.len() != 5 || new_ships < ships || new_caps < caps || new_convoys < convoys
        || usize::from(word(&old.manager, 0xf4)) != ships || usize::from(word(&old.manager, 0xf2)) != caps
        || usize::from(word(&old.manager, 0xf6)) != convoys || usize::from(word(&fleet.manager, 0xf4)) != new_ships
        || usize::from(word(&fleet.manager, 0xf2)) != new_caps || usize::from(word(&fleet.manager, 0xf6)) != new_convoys
        || word(&old.manager, 0xe0) != factory.free_captain || word(&old.manager, 0xe2) != factory.free_ship
        || word(&old.manager, 0xe4) != factory.free_convoy || word(&old.manager, 0xec) != factory.convoy_head
        || word(&old.manager, 0xee) != factory.name_cursor || word(&old.manager, 0xf0) != factory.name_step {
        return Err("vistas o capacidades navales desincronizadas".into());
    }
    let id = usize::from(fleet.ship);
    if id >= new_ships || (id < ships && word(&old.ships, id * 0x180 + 0x134) < 0x17)
        || word(&fleet.ships, id * 0x180 + 0x134) >= 0x17 {
        return Err("ranura pirata asignada inválida".into());
    }
    for i in 0..ships {
        let p = i * 0x180;
        if factory.ships[i].len() != 0x180 || (word(&old.ships, p + 0x134) < 0x17
            && (portraits.next[i] != word(&old.ships, p + 4)
                || portraits.ship_captains[i] != word(&old.ships, p + 0x42))) {
            return Err("vistas de barco pirata desincronizadas".into());
        }
    }
    if factory.convoys.iter().any(|v| v.len() != 0x3c)
        || factory.captains.iter().flatten().copied().ne(old.captains.iter().copied())
        || portraits.captains != factory.captains {
        return Err("vistas de capitán o convoy desincronizadas".into());
    }
    let mut nav = navigation.clone();
    let mut out_factory = factory.clone();
    let mut out_portraits = portraits.clone();
    // La fábrica persiste los slots nuevos; navegación debe reconocer su
    // imagen inicial para no intentar aplicar una transición desde un slot
    // S02 libre que solo tiene cinco bytes.
    for i in 0..ships {
        let at=i*0x180;
        if word(&old.ships,at+0x134)>=0x17 && word(&fleet.ships,at+0x134)<0x17 {
            nav.baseline.ships[at..at+0x180].copy_from_slice(&fleet.ships[at..at+0x180]);
            nav.baseline.routes[i]=Some(SavedRoute::None);
        }
    }
    for image in fleet.ships.chunks_exact(0x180).skip(ships) {
        nav.baseline.ships.extend_from_slice(image);
        nav.baseline.routes.push(if word(image, 0x134) < 0x17 { Some(SavedRoute::None) } else { None });
    }
    nav.baseline.manager[0xf4..0xf6].copy_from_slice(&fleet.manager[0xf4..0xf6]);
    nav.baseline.convoys.extend_from_slice(&fleet.convoys[old.convoys.len()..]);
    nav.baseline.manager[0xf6..0xf8].copy_from_slice(&fleet.manager[0xf6..0xf8]);
    nav.snapshot.manager = fleet.manager.clone();
    nav.snapshot.ships = fleet.ships.clone();
    nav.snapshot.captains = fleet.captains.clone();
    nav.snapshot.convoys = fleet.convoys.clone();
    nav.snapshot.routes.resize(new_ships, None);
    for (i,image) in fleet.ships.chunks_exact(0x180).enumerate(){
        if word(image,0x134)<0x17 && (i>=ships || word(&old.ships,i*0x180+0x134)>=0x17){nav.snapshot.routes[i]=Some(SavedRoute::None)}
    }
    navigation::refresh_lists(&mut nav.snapshot)?;
    // Preserva modificaciones independientes de las imágenes de fábrica y
    // aplica únicamente los bytes que realmente cambió el alta original.
    out_factory.ships.resize_with(new_ships, || vec![0; 0x180]);
    for i in 0..new_ships {
        let image = &fleet.ships[i * 0x180..(i + 1) * 0x180];
        if i >= ships || (word(&old.ships,i*0x180+0x134)>=0x17 && word(image,0x134)<0x17) { out_factory.ships[i].copy_from_slice(image); }
        else { for j in 0..0x180 { if old.ships[i * 0x180 + j] != image[j] { out_factory.ships[i][j] = image[j]; } } }
    }
    out_factory.captains = fleet.captains.chunks_exact(16).map(|v| v.try_into().unwrap()).collect();
    out_factory.convoys.resize_with(new_convoys, || vec![0; 0x3c]);
    for i in 0..new_convoys {
        let image = &fleet.convoys[i * 0x3c..(i + 1) * 0x3c];
        if i >= convoys { out_factory.convoys[i].copy_from_slice(image); }
        else { for j in 0..0x3c { if old.convoys[i * 0x3c + j] != image[j] { out_factory.convoys[i][j] = image[j]; } } }
    }
    out_factory.free_captain = word(&fleet.manager, 0xe0);
    out_factory.free_ship = word(&fleet.manager, 0xe2);
    out_factory.free_convoy = word(&fleet.manager, 0xe4);
    out_factory.convoy_head = word(&fleet.manager, 0xec);
    out_factory.name_cursor = word(&fleet.manager, 0xee);
    out_factory.name_step = word(&fleet.manager, 0xf0);
    out_portraits.next.resize(new_ships, u16::MAX);
    out_portraits.ship_captains.resize(new_ships, u16::MAX);
    for (i, image) in fleet.ships.chunks_exact(0x180).enumerate() {
        if word(image, 0x134) < 0x17 {
            out_portraits.next[i] = word(image, 4);
            out_portraits.ship_captains[i] = word(image, 0x42);
        }
    }
    out_portraits.captains = out_factory.captains.clone();
    for (i, band) in fleet.bands.iter().enumerate() {
        if i >= out_portraits.heads.len() { return Err("demasiadas bandas".into()); }
        out_portraits.heads[i] = band.as_ref().map(|v| word(&v.header, 4));
    }
    Ok(Prepared { ship: fleet.ship, navigation: nav, factory: out_factory, portraits: out_portraits })
}
