//! Estado canónico, primera versión (S01).
//!
//! Identidad por **identificador de dominio** (`TownId`, `MerchantId`, `OfficeId`, `ShipId`, `FacilityId`), nunca por dirección de memoria
//! ni por posición en una lista enlazada: las cadenas del original (`Town+0x784` → oficinas → negocios) se resuelven a listas ordenadas
//! de identificadores y cada referencia se valida. Alcance de esta versión (lo que necesitan los primeros escenarios de compraventa y
//! paso del tiempo): reloj, ciudades (existencias, umbrales de precio, población, dinero, impuestos, instalaciones), comerciantes,
//! oficinas, negocios de comerciante y barcos. Fuera: capitanes, convoyes, edificios, eventos y RNG (ver `research/corpus/README.md`).
//!
//! La extracción sale de [`EconomyWorld`], el modelo con paridad x86 de C04; la extracción desde guardados (`p3-save`) llegará con S02/S04.

use p3_core::economy::world::EconomyWorld;
use p3_core::town::{fac, off, Town, FACILITIES, WARES};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub mod siege;

/// Fin de cadena en el original.
const END: u16 = 0xFFFF;

macro_rules! id {
    ($n:ident, $t:ty) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $n(pub $t);
    };
}
id!(TownId, u8);
id!(MerchantId, u16);
id!(OfficeId, u16);
id!(ShipId, u16);
id!(FacilityId, u16);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Clock {
    pub tick: u32,
    pub month: u8,
    pub day_of_year: u16,
}

/// Instalación de una ciudad (`Town+0x840`, ranuras con tipo distinto de cero).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TownFacility {
    pub slot: u8,
    pub kind: u8,
    pub employees: u16,
    pub efficiency: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TownState {
    /// Existencias por mercancía.
    pub stock: Vec<i32>,
    /// Umbrales de precio: cuatro por mercancía.
    pub price_thresholds: Vec<[i32; 4]>,
    /// Ricos, acomodados, pobres y mendigos.
    pub population: [i32; 4],
    pub money: i32,
    pub flags: u32,
    pub facilities: Vec<TownFacility>,
    /// Oficinas de la ciudad en el orden de su cadena.
    pub offices: Vec<OfficeId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerchantState {
    pub gold: i32,
    /// 0 = jugador humano.
    pub kind: u16,
    pub home_town: TownId,
    pub income: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfficeState {
    pub town: TownId,
    /// Existencias de armas (mercancías 20–23).
    pub weapons: [i32; 4],
    /// Negocios en el orden de su cadena.
    pub businesses: Vec<FacilityId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessState {
    pub employees: u16,
    pub town: TownId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShipState {
    pub owner: MerchantId,
    pub state: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    pub clock: Clock,
    pub towns: BTreeMap<TownId, TownState>,
    pub merchants: BTreeMap<MerchantId, MerchantState>,
    pub offices: BTreeMap<OfficeId, OfficeState>,
    pub businesses: BTreeMap<FacilityId, BusinessState>,
    pub ships: BTreeMap<ShipId, ShipState>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StateError {
    /// Una cadena vuelve sobre un elemento ya visitado.
    Cycle(&'static str),
    /// Referencia a un elemento que no existe: `(origen, destino)`.
    Dangling(String),
    /// Un mismo elemento aparece en dos cadenas.
    Shared(String),
}

impl Snapshot {
    /// Extrae el estado canónico. Las cadenas rotas se informan, no se corrigen.
    pub fn extract(w: &EconomyWorld) -> Result<Snapshot, StateError> {
        let mut towns = BTreeMap::new();
        let mut offices = BTreeMap::new();
        let mut businesses = BTreeMap::new();
        let mut owned_office: BTreeMap<OfficeId, TownId> = BTreeMap::new();
        let mut owned_business: BTreeMap<FacilityId, OfficeId> = BTreeMap::new();

        for (i, t) in w.towns.iter().enumerate() {
            let id = TownId(i as u8);
            let mut chain = Vec::new();
            let mut o = t.u16(off::FIRST_OFFICE);
            while o != END {
                let oid = OfficeId(o);
                if chain.contains(&oid) {
                    return Err(StateError::Cycle("cadena de oficinas"));
                }
                let rec = w
                    .offices
                    .get(o as usize)
                    .ok_or_else(|| StateError::Dangling(format!("ciudad {i} → oficina {o}")))?;
                if let Some(prev) = owned_office.insert(oid, id) {
                    return Err(StateError::Shared(format!("oficina {o} en las ciudades {} y {i}", prev.0)));
                }
                chain.push(oid);
                let mut bs = Vec::new();
                let mut m = rec.first_facility;
                while m != END {
                    let bid = FacilityId(m);
                    if bs.contains(&bid) {
                        return Err(StateError::Cycle("cadena de negocios"));
                    }
                    let b = w
                        .merchant_facilities
                        .get(m as usize)
                        .ok_or_else(|| StateError::Dangling(format!("oficina {o} → negocio {m}")))?;
                    if owned_business.insert(bid, oid).is_some() {
                        return Err(StateError::Shared(format!("negocio {m} en dos oficinas")));
                    }
                    businesses.insert(
                        bid,
                        BusinessState {
                            employees: b.employees,
                            town: TownId(b.town),
                        },
                    );
                    bs.push(bid);
                    m = b.next;
                }
                offices.insert(
                    oid,
                    OfficeState {
                        town: id,
                        weapons: rec.weapons,
                        businesses: bs,
                    },
                );
                o = rec.next;
            }
            towns.insert(id, town_state(t, chain));
        }

        let merchants: BTreeMap<_, _> = w
            .merchants
            .iter()
            .enumerate()
            .map(|(i, m)| {
                (
                    MerchantId(i as u16),
                    MerchantState {
                        gold: m.gold,
                        kind: m.kind,
                        home_town: TownId(m.home_town),
                        income: m.income,
                    },
                )
            })
            .collect();
        let ships = w
            .ships
            .iter()
            .enumerate()
            .map(|(i, s)| {
                (
                    ShipId(i as u16),
                    ShipState {
                        owner: MerchantId(u16::from(s.owner)),
                        state: s.state,
                    },
                )
            })
            .collect();

        let snap = Snapshot {
            clock: Clock {
                tick: w.tick,
                month: w.month,
                day_of_year: w.day_of_year,
            },
            towns,
            merchants,
            offices,
            businesses,
            ships,
        };
        snap.validate()?;
        Ok(snap)
    }

    /// Comprueba que cada referencia apunta a un elemento existente.
    pub fn validate(&self) -> Result<(), StateError> {
        for (id, m) in &self.merchants {
            if !self.towns.contains_key(&m.home_town) {
                return Err(StateError::Dangling(format!("comerciante {} → ciudad {}", id.0, m.home_town.0)));
            }
        }
        for (id, b) in &self.businesses {
            if !self.towns.contains_key(&b.town) {
                return Err(StateError::Dangling(format!("negocio {} → ciudad {}", id.0, b.town.0)));
            }
        }
        for (id, s) in &self.ships {
            if !self.merchants.contains_key(&s.owner) {
                return Err(StateError::Dangling(format!("barco {} → comerciante {}", id.0, s.owner.0)));
            }
        }
        Ok(())
    }

    /// Texto canónico (JSON con las claves ordenadas): dos estados iguales dan el mismo texto.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("el estado es serializable")
    }

    /// Diferencias entre dos estados como `ruta: antes → después` (vacío si son iguales).
    pub fn diff(&self, other: &Snapshot) -> Vec<String> {
        let (a, b) = (serde_json::to_value(self).unwrap(), serde_json::to_value(other).unwrap());
        let mut out = Vec::new();
        walk("", &a, &b, &mut out);
        out
    }
}

/// Ciudades de un guardado (`game_world`). Las oficinas quedan vacías: su bloque aún no se lee (S02), así que las cadenas no se resuelven.
pub fn towns_from_save(save: &p3_save::Save) -> Result<BTreeMap<TownId, TownState>, StateError> {
    let data = save
        .block_named("game_world")
        .ok_or_else(|| StateError::Dangling("falta game_world".into()))?;
    let world = p3_save::world::World::parse(data).map_err(|e| StateError::Dangling(format!("game_world: {e:?}")))?;
    Ok((0..world.town_count())
        .map(|i| (TownId(i as u8), town_state(&world.town(i), Vec::new())))
        .collect())
}

fn town_state(t: &Town, offices: Vec<OfficeId>) -> TownState {
    TownState {
        stock: (0..WARES).map(|w| t.stock(w)).collect(),
        price_thresholds: (0..WARES).map(|w| [0, 1, 2, 3].map(|j| t.threshold(w, j))).collect(),
        population: [0, 1, 2, 3].map(|g| t.pop(g)),
        money: t.i32(off::MONEY),
        flags: t.flags(),
        facilities: (0..FACILITIES)
            .filter(|&f| t.facility_type(f) != 0)
            .map(|f| TownFacility {
                slot: f as u8,
                kind: t.u8(Town::fac(f, fac::TYPE)),
                employees: t.employees(f),
                efficiency: t.efficiency(f),
            })
            .collect(),
        offices,
    }
}

fn walk(path: &str, a: &serde_json::Value, b: &serde_json::Value, out: &mut Vec<String>) {
    use serde_json::Value::*;
    match (a, b) {
        (Object(x), Object(y)) => {
            for k in x.keys().chain(y.keys().filter(|k| !x.contains_key(*k))) {
                let p = format!("{path}/{k}");
                match (x.get(k), y.get(k)) {
                    (Some(l), Some(r)) => walk(&p, l, r, out),
                    (Some(l), None) => out.push(format!("{p}: {l} → (ausente)")),
                    (None, Some(r)) => out.push(format!("{p}: (ausente) → {r}")),
                    (None, None) => {}
                }
            }
        }
        (Array(x), Array(y)) if x.len() == y.len() => {
            for (i, (l, r)) in x.iter().zip(y).enumerate() {
                walk(&format!("{path}/{i}"), l, r, out);
            }
        }
        _ if a != b => out.push(format!("{path}: {a} → {b}")),
        _ => {}
    }
}
pub mod office;
pub mod office_factory;
pub mod messages;
pub mod letter_ops;
pub mod trade_center_discovery;
pub mod land_transport;

pub mod trade_offer_watch;
