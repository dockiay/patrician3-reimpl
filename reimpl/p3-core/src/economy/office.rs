//! Gastos y contadores de los negocios de una oficina (C04).
use super::taxes::OfficeAccounts;
use crate::ai::reputation::Merchant;

// Tablas del EXE inglés en 0x6729E8 y 0x672A00; entradas firmadas.
const AI: [i8; 21] = [
    30, 0, 36, 26, 56, 30, 16, 36, 16, 6, 39, 6, 33, 9, 36, 36, 36, 26, 3, 0, 6,
];
const HUMAN: [i8; 21] = [
    30, 0, 36, 26, 68, 32, 22, 48, 17, 7, 45, 8, 40, 9, 44, 44, 38, 32, 3, 3, 7,
];

/// Contadores `Office+0x448` y `Office+0x44A`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OfficeWorkers {
    pub total: u16,
    pub auxiliary: u16,
}

/// `facility::pay_wages` 0x004D3560. `kind` debe corresponder a una instalación 0..21.
/// `whale_oil` corresponde al dword de la ciudad `+0x2CC` (comparación sin signo).
pub fn pay_wages(
    kind: usize,
    employees: u16,
    auxiliary: u16,
    whale_oil: u32,
    merchant: &mut Merchant,
    office: &mut OfficeAccounts,
    workers: &mut OfficeWorkers,
) {
    let human = merchant.i32(8) as u16 == 0;
    let table = if human { &HUMAN } else { &AI };
    let mut wage = (i32::from(table[kind]) * i32::from(employees)) / 3;
    if kind == 5 && whale_oil != 0 {
        wage = wage.wrapping_add(if human {
            i32::from(employees) * 24 / 3
        } else {
            i32::from(employees) * 7
        });
    }
    for offset in [0x480, 0x484] {
        merchant.set_i32(offset, merchant.i32(offset).wrapping_add(wage));
    }
    office.current[2] = office.current[2].wrapping_add(wage);
    workers.total = workers.total.wrapping_add(employees);
    workers.auxiliary = workers.auxiliary.wrapping_add(auxiliary);
}

/// Argumentos de valoración que produce el despacho 0x004D3780.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductionValuation {
    pub first: i32,
    pub second: i32,
}
/// Produce en el almacén de la oficina y actualiza la capacidad de la ciudad.
/// Solo cubre los productores de mercancías 4..=20; `None` para instalaciones especiales.
/// El llamante debe aplicar la valoración devuelta antes del siguiente negocio.
pub fn produce_business(
    town: &mut crate::town::Town,
    storage: &mut [u8; 0x2c0],
    kind: u8,
    efficiency: i32,
    employees: u16,
    capacity: u16,
    tick: u32,
    mask: &mut u32,
) -> Option<ProductionValuation> {
    use super::production as p;
    use crate::town::{fac, off, Town};
    let k = kind as usize;
    if !(4..=20).contains(&k) {
        return None;
    }
    let mut t = town.clone();
    t.0[..0x2c0].copy_from_slice(storage);
    let f = 0;
    let base = Town::fac(f, 0);
    t.set_i32(base + fac::EFFICIENCY, efficiency);
    t.set_u16(base + fac::EMPLOYEES, employees);
    let products: [usize; 21] = [0, 0, 0, 0, 9, 2, 3, 12, 5, 0, 1, 11, 8, 4, 16, 14, 7, 18, 19, 15, 17];
    let w = products[k];
    let before = t.stock(w);
    let before_extra = if k == 5 {
        t.stock(10)
    } else if k == 10 {
        t.stock(13)
    } else {
        0
    };
    let cap = capacity as i32;
    match k {
        4 => p::hunting_lodge(&mut t, f, tick, cap),
        5 => p::fishermans_hut(&mut t, f, tick, cap as u32, false),
        6 => p::brewery(&mut t, f, tick, 0, cap as u32),
        7 => p::workshop(&mut t, f, tick, cap),
        8 => p::apiary(&mut t, f, tick, cap),
        9 => p::farm_grain(&mut t, f, tick, cap),
        10 => p::farm_cattle(&mut t, f, tick, cap),
        11 => p::sawmill(&mut t, f, tick, cap),
        12 => p::weaving_mill(&mut t, f, tick, cap),
        13 => p::saltworks(&mut t, f, tick, 0, cap as u32),
        14 => p::iron_smelter(&mut t, f, tick, cap),
        15 => p::farm_sheep(&mut t, f, tick, cap),
        16 => p::vineyard(&mut t, f, tick, cap as u32),
        17 => p::pottery(&mut t, f, tick, cap as u32),
        18 => p::brickworks(&mut t, f, tick, cap),
        19 => p::pitchmaker(&mut t, f, tick, 0, cap as u32),
        20 => p::farm_hemp(&mut t, f, tick, cap),
        _ => unreachable!(),
    }
    let delta = t.stock(w).wrapping_sub(before);
    let second = if k == 5 {
        t.stock(10).wrapping_sub(before_extra)
    } else if k == 10 {
        t.stock(13).wrapping_sub(before_extra)
    } else {
        0
    };
    let always = matches!(k, 8 | 9 | 11 | 14 | 15 | 16 | 20);
    let first = if always {
        t.i32(off::DAILY_PRODUCTION + 4 * w)
    } else {
        delta
    };
    let active = if always {
        employees != 0
    } else if k == 5 {
        delta > 0 || second > 0
    } else {
        delta > 0
    };
    if active {
        *mask |= 1u32 << k;
    }
    storage.copy_from_slice(&t.0[..0x2c0]);
    town.0[off::PRODUCTION_VALUES..off::PRODUCTION_VALUES + 96]
        .copy_from_slice(&t.0[off::PRODUCTION_VALUES..off::PRODUCTION_VALUES + 96]);
    Some(ProductionValuation { first, second })
}

/// Valoración de producción 0x004D4400, incluidas las dos salidas de pesca y ganadería.
pub fn value_business(
    kind: u8,
    employees: u16,
    auxiliary: u16,
    tax: u16,
    qty: ProductionValuation,
    storage: &[u8; 0x2c0],
    average: &mut [f32; 24],
    whaling: bool,
) -> bool {
    let k = kind as usize;
    if !(4..=20).contains(&k) {
        return false;
    }
    if qty.first == 0 && qty.second == 0 {
        return true;
    }
    let divisors: [u16; 21] = [
        50, 40, 30, 15, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 15, 15, 30,
    ];
    let rent = ((auxiliary / divisors[k]) as i32).wrapping_mul(tax as i32) as f32;
    let wages = (i32::from(HUMAN[k]) * i32::from(employees)) as f32;
    let mut cost = (f64::from(wages) / 3.0 + f64::from(rent)) as f32;
    let stock_at = |w: usize| {
        let p = 4 + 4 * w;
        i32::from_le_bytes(storage[p..p + 4].try_into().unwrap())
    };
    if k == 5 && whaling {
        cost = (f64::from(cost) + (24.0 / 3.0) * f64::from(employees as f32)) as f32;
        let oil_cost = (f64::from(cost) * f64::from(0.4f32)) as f32;
        cost = (f64::from(cost) - f64::from(oil_cost)) as f32;
        let oil = stock_at(10);
        average[10] = if oil > 0 {
            ((f64::from(oil.wrapping_sub(qty.second)) * f64::from(average[10]) + f64::from(oil_cost)) / f64::from(oil))
                as f32
        } else {
            0.0
        };
        let fish = stock_at(2);
        let q = f64::from(qty.first as f32);
        let mut sum = f64::from(fish.wrapping_sub(qty.first)) * f64::from(average[2]) + f64::from(cost);
        sum += q * f64::from(average[4]) / 5.0;
        sum += q * f64::from(average[17]) / 10.0;
        average[2] = if fish > 0 { (sum / f64::from(fish)) as f32 } else { 0.0 };
        return true;
    }
    if k == 10 {
        let leather_cost = (f64::from(cost) * f64::from(0.49f32)) as f32;
        cost = (f64::from(cost) - f64::from(leather_cost)) as f32;
        let leather = stock_at(13);
        let q = f64::from(qty.second as f32);
        let mut sum = f64::from(leather.wrapping_sub(qty.second)) * f64::from(average[13]);
        sum += q * f64::from(average[11]) / 5.0;
        sum += q * f64::from(average[4]) / 10.0;
        sum += f64::from(leather_cost);
        average[13] = if leather > 0 {
            (sum / f64::from(leather)) as f32
        } else {
            0.0
        };
        let meat = stock_at(1);
        let q = f64::from(qty.first as f32);
        let sum = f64::from(meat.wrapping_sub(qty.first)) * f64::from(average[1])
            + q * f64::from(average[4]) / 4.0
            + f64::from(cost);
        average[1] = if meat > 0 { (sum / f64::from(meat)) as f32 } else { 0.0 };
        return true;
    }
    let produced = qty.first as f32;
    let ingredients: &[(usize, f64)] = match k {
        4 => &[(12, 20.0), (17, 20.0)],
        5 => &[(4, 5.0), (17, 10.0)],
        6 => &[(0, 4.0), (11, 10.0)],
        7 => &[(16, 1.0), (11, 1.0)],
        12 => &[(14, 1.0)],
        13 | 17 | 19 => &[(11, 1.0)],
        18 => &[(11, 2.0)],
        _ => &[],
    };
    for &(ware, divisor) in ingredients {
        cost = (f64::from(produced) * f64::from(average[ware]) / divisor + f64::from(cost)) as f32;
    }
    if qty.first > 0 {
        let product = [0, 0, 0, 0, 9, 2, 3, 12, 5, 0, 1, 11, 8, 4, 16, 14, 7, 18, 19, 15, 17][k];
        let at = 4 + product * 4;
        let stock = i32::from_le_bytes(storage[at..at + 4].try_into().unwrap());
        let old = stock.wrapping_sub(qty.first);
        average[product] = ((f64::from(old) * f64::from(average[product]) + f64::from(cost)) / f64::from(stock)) as f32;
    }
    true
}

/// Fronteras del astillero y la construcción llamadas por negocios especiales.
pub trait SpecialBusinessHost {
    fn shipyard_construction(&mut self, town: u8, work: u32);
    fn shipyard_repairs(&mut self, town: u8, work: u32);
    fn shipyard_construction_with_town(&mut self, town: &mut crate::town::Town, work: u32) {
        self.shipyard_construction(town.index(), work)
    }
    fn shipyard_repairs_with_town(&mut self, town: &mut crate::town::Town, work: u32) {
        self.shipyard_repairs(town.index(), work)
    }
    fn shipyard_advance_with_town(&mut self, town: &mut crate::town::Town, work: u32) {
        self.shipyard_construction_with_town(town, work);
        self.shipyard_repairs_with_town(town, work)
    }
    fn sync_active_merchant(&mut self, _merchant: &crate::ai::reputation::Merchant) {}
    fn apply_active_merchant(&mut self, _merchant: &mut crate::ai::reputation::Merchant) -> Result<(), String> {
        Ok(())
    }
    fn advance_construction(&mut self, town: &mut crate::town::Town, workers: u16);
    /// Estado de registros y plano del anfitrión tras las obras; necesario para persistir en S05.
    fn city_site(&self, _town: u8) -> Option<&crate::buildings::Site> {
        None
    }
    /// Gestor naval actualizado, si el anfitrión lo conecta a S05.
    fn shipyard_manager(&self) -> Option<&crate::shipyard::ShipManager> {
        None
    }
    /// Acceso opcional a los registros vivos de negocios, para seguir sus enlaces tras callbacks.
    fn live_business_count(&self) -> Option<usize> {
        None
    }
    fn live_business(&self, _index: u16) -> Option<[u8; 20]> {
        None
    }
    fn take_relocated_business(&mut self, _index: u16) -> Option<[u8; 20]> {
        None
    }
    fn put_live_business(&mut self, _index: u16, _record: [u8; 20]) -> Result<(), String> {
        Err("sin gestor de negocios vivos".into())
    }
    fn sync_active_office(&mut self, _image: &[u8]) {}
    fn apply_active_office(&mut self, _image: &mut [u8]) -> Result<(), String> {
        Ok(())
    }
}
/// Casos 0–3 de `produce_for_storage` (0x004D3780).
/// El armero puede inicializar a 1024 la eficiencia del propio negocio.
pub fn produce_special_business(
    town: &mut crate::town::Town,
    storage: &mut [u8; 0x2c0],
    kind: u8,
    efficiency: &mut i32,
    employees: u16,
    tick: u32,
    activity: &mut u32,
    host: &mut dyn SpecialBusinessHost,
) -> bool {
    use crate::town::{fac, off, Town};
    if kind > 3 {
        return false;
    }
    match kind {
        0 => {
            if town.flags() & 0x10 == 0 {
                let beer = i32::from(employees).wrapping_mul(*efficiency).wrapping_mul(60) / 100 >> 10;
                let pos = off::BUSINESS_CONSUMPTION + 3 * 4;
                let current = i32::from_le_bytes(storage[pos..pos + 4].try_into().unwrap());
                storage[pos..pos + 4].copy_from_slice(&current.wrapping_add(beer).to_le_bytes());
                let pos = off::STOCK + 3 * 4;
                let stock = i32::from_le_bytes(storage[pos..pos + 4].try_into().unwrap());
                let result = if stock < beer { 0 } else { stock.wrapping_sub(beer) };
                storage[pos..pos + 4].copy_from_slice(&result.to_le_bytes());
            }
        }
        1 => {
            let mut pitch = (i32::from(employees) * 71 >> 5) + 0x43;
            if town.flags() & 0x10 != 0 {
                pitch += pitch;
            }
            let pos = off::BUSINESS_CONSUMPTION + 15 * 4;
            let current = i32::from_le_bytes(storage[pos..pos + 4].try_into().unwrap());
            storage[pos..pos + 4].copy_from_slice(&current.wrapping_add(pitch).to_le_bytes());
            let pos = off::STOCK + 15 * 4;
            let stock = i32::from_le_bytes(storage[pos..pos + 4].try_into().unwrap());
            let result = if stock > pitch { stock - pitch } else { 0 };
            storage[pos..pos + 4].copy_from_slice(&result.to_le_bytes());
            host.shipyard_advance_with_town(town, u32::from(employees) * 100);
        }
        2 => host.advance_construction(town, employees),
        3 => {
            let mut proxy = town.clone();
            proxy.0[..0x2c0].copy_from_slice(storage);
            let base = Town::fac(0, 0);
            proxy.set_i32(base + fac::EFFICIENCY, *efficiency);
            proxy.set_u16(base + fac::EMPLOYEES, employees);
            super::production::weaponsmith(&mut proxy, 0, tick, 20);
            *efficiency = proxy.i32(base + fac::EFFICIENCY);
            storage.copy_from_slice(&proxy.0[..0x2c0]);
            town.set_u8(off::WEAPONSMITH_SHORTAGE, proxy.u8(off::WEAPONSMITH_SHORTAGE));
        }
        _ => unreachable!(),
    }
    if employees != 0 {
        *activity |= 1u32 << kind;
    }
    true
}
/// Trabajo del astillero, que el integrador conecta al gestor de barcos.
pub trait BusinessShipyardHost {
    fn construction(&mut self, town: u8, work: u32);
    fn repairs(&mut self, town: u8, work: u32);
    fn construction_with_town(&mut self, town: &mut crate::town::Town, work: u32) {
        self.construction(town.index(), work)
    }
    fn repairs_with_town(&mut self, town: &mut crate::town::Town, work: u32) {
        self.repairs(town.index(), work)
    }
    fn manager(&self) -> Option<&crate::shipyard::ShipManager> {
        None
    }
}
/// Puente a `Site::advance_construction` con el estado de obra de una ciudad.
pub struct SiteBusinessHost<'a> {
    pub town: u8,
    pub site: crate::buildings::Site,
    pub merchant_count: u16,
    pub visual_owner: Option<u32>,
    pub advance_host: &'a mut dyn crate::buildings::advance::AdvanceHost,
    pub shipyard: &'a mut dyn BusinessShipyardHost,
}
impl SpecialBusinessHost for SiteBusinessHost<'_> {
    fn shipyard_construction(&mut self, t: u8, w: u32) {
        self.shipyard.construction(t, w)
    }
    fn shipyard_repairs(&mut self, t: u8, w: u32) {
        self.shipyard.repairs(t, w)
    }
    fn shipyard_construction_with_town(&mut self, t: &mut crate::town::Town, w: u32) {
        self.shipyard.construction_with_town(t, w)
    }
    fn shipyard_repairs_with_town(&mut self, t: &mut crate::town::Town, w: u32) {
        self.shipyard.repairs_with_town(t, w)
    }
    fn advance_construction(&mut self, t: &mut crate::town::Town, n: u16) {
        assert_eq!(t.index(), self.town, "ciudad de obra incorrecta");
        let mut cx = crate::buildings::advance::AdvanceCtx {
            merchant_count: self.merchant_count,
            visual_owner: self.visual_owner,
            host: self.advance_host,
        };
        self.site.advance_construction(t, &mut cx, u32::from(n));
    }
    fn city_site(&self, t: u8) -> Option<&crate::buildings::Site> {
        if t == self.town {
            Some(&self.site)
        } else {
            None
        }
    }
    fn shipyard_manager(&self) -> Option<&crate::shipyard::ShipManager> {
        self.shipyard.manager()
    }
    fn live_business_count(&self) -> Option<usize> {
        self.advance_host.live_business_count()
    }
    fn live_business(&self, i: u16) -> Option<[u8; 20]> {
        self.advance_host.live_business(i)
    }
    fn take_relocated_business(&mut self, i: u16) -> Option<[u8; 20]> {
        self.advance_host.take_relocated_business(i)
    }
    fn put_live_business(&mut self, i: u16, r: [u8; 20]) -> Result<(), String> {
        self.advance_host.put_live_business(i, r)
    }
    fn sync_active_office(&mut self, image: &[u8]) {
        self.advance_host.sync_active_office(image)
    }
    fn apply_active_office(&mut self, image: &mut [u8]) -> Result<(), String> {
        self.advance_host.apply_active_office(image)
    }
}

/// Puente de trabajo a las rutinas nativas de construcción y reparación naval (C05).
/// El anfitrión conserva y persiste el gestor de barcos y los avisos que esas rutinas emiten.
pub struct LiveBusinessShipyard<'a> {
    pub manager: &'a mut crate::shipyard::ShipManager,
    pub launch_host: &'a mut dyn crate::shipyard::LaunchHost,
    pub repair_host: &'a mut dyn crate::shipyard::ShipyardHost,
    pub merchant_count: u16,
    pub tick: u32,
    pub player: u32,
}
impl BusinessShipyardHost for LiveBusinessShipyard<'_> {
    fn manager(&self) -> Option<&crate::shipyard::ShipManager> {
        Some(self.manager)
    }
    fn construction(&mut self, _: u8, _: u32) {
        panic!("requiere estado de ciudad")
    }
    fn repairs(&mut self, _: u8, _: u32) {
        panic!("requiere estado de ciudad")
    }
    fn construction_with_town(&mut self, t: &mut crate::town::Town, w: u32) {
        let index = u32::from(t.index());
        self.manager.advance_ship_construction(
            t,
            index,
            w as i32,
            self.tick,
            self.player,
            self.merchant_count,
            self.launch_host,
        );
    }
    fn repairs_with_town(&mut self, t: &mut crate::town::Town, w: u32) {
        let index = u32::from(t.index());
        let mut cx = crate::shipyard::ShipyardCtx {
            merchant_count: self.merchant_count,
            host: self.repair_host,
        };
        self.manager.advance_repairs(t, index, w as i32, &mut cx);
    }
}
