//! Conexión de las pujas de C10 con las valoraciones y oficinas existentes.
use p3_core::{
    ai::{
        asset_values::ship_current_value,
        bids::{self, BidMerchant, ShipBidEnv, ShipValueHost},
        guild::GuildOffice,
        reputation::Merchant,
    },
    economy::auction::{self, BidLimit},
    scheduler::TaskRecord,
    town::Town,
};

pub fn ship_bids(
    task: &mut TaskRecord,
    towns: &[Town],
    merchants: &[Merchant],
    offices: &[GuildOffice],
    ships: &[Option<Vec<u8>>],
) -> Result<bool, String> {
    if task.kind != 0x0a {
        return Err("pujas de edificios aún no conectadas".into());
    }
    let count = u16::try_from(merchants.len()).map_err(|_| "demasiados comerciantes")?;
    let office_count = u16::try_from(offices.len()).map_err(|_| "demasiadas oficinas")?;
    let id = usize::from(u16::from_le_bytes([task.u8_at(8), task.u8_at(9)]));
    let image = ships
        .get(id)
        .and_then(Option::as_deref)
        .ok_or("barco de subasta ausente")?;
    if image.len() < 0x180 {
        return Err("barco de subasta truncado".into());
    }
    let kind = image[0xe];
    let maximum = i32::from_le_bytes(image[0x14..0x18].try_into().unwrap());
    let current = i32::from_le_bytes(image[0x18..0x1c].try_into().unwrap());
    let value = ship_current_value(kind, maximum, current).map_err(str::to_owned)? as i32;
    let mut kinds = vec![0u8; ships.len()];
    kinds[id] = kind;
    let flags = towns.iter().map(|t| t.u32(0x2c8)).collect::<Vec<_>>();
    struct Value(i32);
    impl ShipValueHost for Value {
        fn ship_value(&mut self, _: usize, _: usize) -> i32 {
            self.0
        }
    }
    struct Adapter<'a> {
        merchants: &'a [Merchant],
        env: ShipBidEnv<'a>,
        value: i32,
    }
    impl BidLimit for Adapter<'_> {
        fn limit(&mut self, index: u16, _: bool, ship: u16, city: u8) -> Result<i32, String> {
            let m = self
                .merchants
                .get(usize::from(index))
                .ok_or("comerciante de subasta ausente")?;
            if m.bytes.len() < 0x548 {
                return Err("comerciante de subasta truncado".into());
            }
            let control = m.u16(8);
            let home = m.bytes[0x19];
            if control != 0 && usize::from(home) >= self.env.town_flags.len() {
                return Err("ciudad natal de subasta ausente".into());
            }
            let mut at = usize::from(m.u16(0xc));
            let mut seen = vec![false; usize::from(self.env.office_count)];
            while at < seen.len() {
                if seen[at] {
                    return Err("cadena de oficinas de subasta cíclica".into());
                }
                seen[at] = true;
                at = usize::from(self.env.offices[at].next)
            }
            let person = BidMerchant {
                control,
                home,
                first_office: m.u16(0xc),
                assets: m.i32(0x53c),
                limit: m.i32(0x470),
                b546: m.bytes[0x546],
                b547: m.bytes[0x547],
            };
            Ok(bids::ship_auction_bid_limit(
                &person,
                &self.env,
                u32::from(ship),
                u32::from(city),
                &mut Value(self.value),
            ))
        }
    }
    let env = ShipBidEnv {
        town_flags: &flags,
        ship_kinds: &kinds,
        offices,
        office_count,
    };
    auction::run_bids(task, count, &mut Adapter { merchants, env, value })
}

pub fn building_bids(
    task: &mut TaskRecord,
    town: &Town,
    site: &p3_core::buildings::Site,
    merchants: &[Merchant],
    offices: &[bids::BidOffice],
    businesses: &[bids::BidTask],
    store_flag: i32,
    siege_active: bool,
) -> Result<bool, String> {
    use bids::{BidBuilding, BidTown, BuildingBidEnv, BuildingBidHost, BuildingBidMerchant};
    let count = usize::from(town.u16(0x776));
    if site.records.len() < count * 8 {
        return Err("registros de subasta truncados".into());
    }
    let object = usize::from(u16::from_le_bytes([task.u8_at(8), task.u8_at(9)]) & 0x7fff);
    if object >= count || site.records[object * 8 + 3] >= 30 {
        return Err("edificio de subasta inválido".into());
    }
    let mut at = usize::from(town.u16(0x762));
    let mut seen = vec![false; count];
    while at < count {
        if seen[at] {
            return Err("lista de obras de subasta cíclica".into());
        }
        seen[at] = true;
        at = usize::from(u16::from_le_bytes([site.records[at * 8 + 4], site.records[at * 8 + 5]]));
    }
    let office_count = u16::try_from(offices.len()).map_err(|_| "demasiadas oficinas")?;
    let business_count = u16::try_from(businesses.len()).map_err(|_| "demasiados negocios")?;
    let mut at = usize::from(town.u16(0x784));
    let mut visited = vec![false; offices.len()];
    while at < offices.len() {
        if visited[at] {
            return Err("oficinas urbanas de subasta cíclicas".into());
        }
        visited[at] = true;
        let office = offices[at];
        if usize::from(office.merchant) >= merchants.len() {
            return Err("propietario de oficina de subasta ausente".into());
        }
        let mut business = usize::from(office.task);
        let mut seen_business = vec![false; businesses.len()];
        while business < businesses.len() {
            if seen_business[business] {
                return Err("negocios de subasta cíclicos".into());
            }
            seen_business[business] = true;
            business = usize::from(businesses[business].next)
        }
        at = usize::from(office.next_in_town)
    }
    let controls = merchants
        .iter()
        .map(|m| {
            if m.bytes.len() >= 10 {
                Ok(m.u16(8))
            } else {
                Err("comerciante truncado")
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let stocks = (0..21).map(|i| town.u16(0x848 + i * 16)).collect::<Vec<_>>();
    let city = BidTown {
        width: town.i32(0x7ac),
        cells: &site.cells,
        demand: [town.i32(0x2d8), town.i32(0x2dc), town.i32(0x2e0)],
        divisor: [town.u16(0x2f8), town.u16(0x2fa), town.u16(0x2fc)],
        first: town.u16(0x762),
        count: town.u16(0x776),
        first_office: town.u16(0x784),
        reserve: town.u32(0x2cc),
        stock: &stocks,
    };
    let env = BuildingBidEnv {
        town: city,
        offices,
        office_count,
        tasks: businesses,
        task_count: business_count,
        merchants: &controls,
    };
    struct Values<'a> {
        site: &'a p3_core::buildings::Site,
        town: &'a Town,
        store: i32,
        siege: bool,
    }
    impl BuildingBidHost for Values<'_> {
        fn building_record(&mut self, index: u16) -> BidBuilding {
            let p = usize::from(index) * 8;
            let r = &self.site.records[p..p + 8];
            BidBuilding {
                x: r[0],
                y: r[1],
                owner: r[2],
                kind: r[3],
                next: u16::from_le_bytes([r[4], r[5]]),
                flag: r[6],
            }
        }
        fn sale_value(&mut self, _: u32, index: u32, inside: u8) -> i32 {
            let base = self.site.building_base_value(self.town, index, self.store);
            auction::building_sale_price(base, inside != 0, self.siege)
        }
    }
    struct Adapter<'a> {
        merchants: &'a [Merchant],
        env: BuildingBidEnv<'a>,
        values: Values<'a>,
    }
    impl BidLimit for Adapter<'_> {
        fn limit(&mut self, index: u16, _: bool, building: u16, city: u8) -> Result<i32, String> {
            let m = self.merchants.get(usize::from(index)).ok_or("comerciante ausente")?;
            if m.bytes.len() < 0x1a {
                return Err("comerciante truncado".into());
            }
            let mut at = usize::from(m.u16(0xc));
            let mut seen = vec![false; usize::from(self.env.office_count)];
            while at < seen.len() {
                if seen[at] {
                    return Err("oficinas de comerciante cíclicas".into());
                }
                seen[at] = true;
                at = usize::from(self.env.offices[at].next)
            }
            let person = BuildingBidMerchant {
                assets: m.i32(0),
                control: m.u16(8),
                first_office: m.u16(0xc),
                home: m.bytes[0x19],
            };
            if person.assets < 0 && person.control != 0 {
                return Err("puja con logaritmo no acreditado".into());
            }
            Ok(bids::building_auction_bid_limit(
                &person,
                &self.env,
                u32::from(building),
                u32::from(city),
                index as u8,
                &mut self.values,
            ) as i32)
        }
    }
    auction::run_bids(
        task,
        u16::try_from(merchants.len()).map_err(|_| "demasiados comerciantes")?,
        &mut Adapter {
            merchants,
            env,
            values: Values {
                site,
                town,
                store: store_flag,
                siege: siege_active,
            },
        },
    )
}
