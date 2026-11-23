//! Traspaso transaccional de inmuebles entre oficinas existentes.
//!
//! Usa la transferencia y el recuento C05 sobre copias; aún no crea oficinas.
use p3_core::{
    buildings::{transfer::{TransferCtx, TransferHost, TransferMerchant}, Site},
    economy::{facility_pool::FacilityPool, office_recount::{self, NativeFacilities}},
    town::Town,
};

fn word(image: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([image[at], image[at + 1]])
}

#[derive(Clone, Debug)]
pub(crate) struct TransferSnapshot {
    pub town: Town,
    pub site: Site,
    pub offices: Vec<Vec<u8>>,
    pub merchants: Vec<TransferMerchant>,
    pub facilities: FacilityPool,
    pub changed_offices: Vec<usize>,
}

struct Host<'a> {
    merchants: &'a mut [TransferMerchant],
    offices: &'a mut [Vec<u8>],
    owners: [u8; 2],
    indices: [usize; 2],
    recount: Vec<usize>,
}
impl TransferHost for Host<'_> {
    fn merchant(&mut self, owner: u32) -> &mut TransferMerchant { &mut self.merchants[owner as usize] }
    fn find_office(&mut self, owner: u32, _: u32, _: u32) -> Option<u32> {
        self.owners.iter().position(|&v| u32::from(v) == owner).map(|i| self.indices[i] as u32)
    }
    fn office_head(&mut self, office: u32, second: bool) -> u16 {
        word(&self.offices[office as usize], 0x2ce + 2 * usize::from(second))
    }
    fn set_office_head(&mut self, office: u32, second: bool, value: u16) {
        let at = 0x2ce + 2 * usize::from(second);
        self.offices[office as usize][at..at + 2].copy_from_slice(&value.to_le_bytes());
    }
    fn recount_office(&mut self, office: u32) { self.recount.push(office as usize); }
    fn draw_chain(&mut self, _: u32, _: u32) { unreachable!("sin vista urbana") }
    fn set_progress(&mut self, _: u32, _: u32, _: u32, _: u32, _: u32) { unreachable!("sin vista urbana") }
}

/// Devuelve una copia coherente o rechaza la operación sin modificar su entrada.
/// Comprador con oficina existente; vendedor público o con oficina; sin vista.
pub(crate) fn transfer_existing_offices(
    town: &Town, site: &Site, offices: &[Vec<u8>], merchants: &[TransferMerchant],
    facilities: &FacilityPool, object: u16, buyer: u8, price: u16, source_relocated: bool,
) -> Result<TransferSnapshot, String> {
    let count = usize::from(town.u16(0x776));
    if site.records.len() != count * 8 || usize::from(object) >= count { return Err("edificio de subasta ausente".into()) }
    if offices.iter().any(|o| o.len() != 0x44c) { return Err("oficina de subasta truncada".into()) }
    let old = site.rec(usize::from(object), 2);
    let kind = site.rec(usize::from(object), 3);
    if old == buyer || usize::from(buyer) >= merchants.len() {
        return Err("propietarios de subasta incompatibles".into())
    }
    if !(4..=29).contains(&kind) { return Err("tipo de edificio de subasta no acreditado".into()) }
    let mut found = [None, None];
    let mut seen = vec![false; offices.len()];
    let mut at = usize::from(town.u16(0x784));
    while at < offices.len() {
        if seen[at] { return Err("ciclo en oficinas urbanas".into()) }
        seen[at] = true;
        let office = &offices[at];
        if office[0x2c6] != town.index() { return Err("oficina de otra ciudad".into()) }
        for (i, owner) in [old, buyer].iter().enumerate() {
            if word(office, 0x2c4) == u16::from(*owner) {
                if found[i].replace(at).is_some() { return Err("oficinas duplicadas".into()) }
            }
        }
        at = usize::from(word(office, 0x2ca));
    }
    let public = usize::from(old) >= merchants.len();
    let source = if public { None } else { Some(found[0].ok_or("vendedor sin oficina")?) };
    let indices = [source.unwrap_or(usize::MAX), found[1].ok_or("comprador sin oficina")?];
    let second = kind >= 21;
    let mut seen = vec![false; count];
    let mut at = usize::from(if let Some(source)=source {word(&offices[source],0x2ce+2*usize::from(second))}else{town.u16(0x764)});
    let mut linked = false;
    while at < count {
        if seen[at] { return Err("ciclo en edificios del vendedor".into()) }
        seen[at] = true;
        linked |= at == usize::from(object);
        at = usize::from(site.next(at));
    }
    if !linked { return Err("edificio fuera de la lista del vendedor".into()) }
    let mut result = TransferSnapshot { town: town.clone(), site: site.clone(), offices: offices.to_vec(),
        merchants: merchants.to_vec(), facilities: facilities.clone(), changed_offices: source.into_iter().chain(std::iter::once(indices[1])).collect() };
    let calls = {
        let mut host = Host { merchants: &mut result.merchants, offices: &mut result.offices,
            owners: [old, buyer], indices, recount: Vec::new() };
        let mut cx = TransferCtx { merchant_count: u16::try_from(merchants.len()).map_err(|_| "demasiados comerciantes")?, visual_owner: u32::MAX, host: &mut host };
        let paid = i32::from(price);
        result.site.transfer_building_to_merchant(&mut result.town, &mut cx, u32::from(object), buyer, paid, paid * 9 / 10);
        host.recount
    };
    for index in calls {
        // Si la fábrica amplió el array, el original recuenta el vendedor
        // mediante el puntero anterior. Sus efectos en ciudad y pool sobreviven,
        // pero los campos de aquella oficina quedan en el bloque viejo.
        let mut old_office = if source_relocated && index == indices[0] { Some(result.offices[index].clone()) } else { None };
        let office: &mut [u8; 0x44c] = match old_office.as_mut(){Some(old)=>old.as_mut_slice(),None=>result.offices[index].as_mut_slice()}.try_into().map_err(|_| "oficina truncada")?;
        let mut pool = NativeFacilities { pool: &mut result.facilities, relocated: None };
        office_recount::recount(office, &mut result.town, &mut result.site, &mut pool)?;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    fn put(image: &mut [u8], at: usize, value: u16) { image[at..at + 2].copy_from_slice(&value.to_le_bytes()); }
    fn hex(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }
    fn unhex(s: &str) -> Vec<u8> { s.as_bytes().chunks_exact(2).map(|v| u8::from_str_radix(std::str::from_utf8(v).unwrap(), 16).unwrap()).collect() }

    #[test]
    fn transfer_and_real_recounts_match_original() {
        let (Ok(binary), Ok(python)) = (std::env::var("P3_EXE"), std::env::var("P3_ORACLE_PY")) else { return };
        for (kind, amount) in [(21, 1500u16), (27, 551), (29, 0), (4, 1500)] {
            let mut town = Town::default(); town.set_u8(0x2c0, 0); town.set_u16(0x776, 2);
            town.set_u16(0x762, u16::MAX); town.set_u16(0x784, 0);
            let site = Site { records: vec![10,10,0,kind,255,255,0,0, 11,11,1,if kind < 21 {4} else {22},255,255,0,0], cells: vec![] };
            let mut offices = vec![vec![0u8; 0x44c]; 2];
            for i in 0..2 { put(&mut offices[i], 0x2c4, i as u16); put(&mut offices[i], 0x2ca, if i == 0 { 1 } else { u16::MAX });
                put(&mut offices[i], 0x2cc, u16::MAX); put(&mut offices[i], 0x2ce, if kind < 21 {i as u16} else {u16::MAX}); put(&mut offices[i], 0x2d0, if kind < 21 {u16::MAX} else {i as u16}); }
            let merchants = vec![TransferMerchant {gold: 1200, sold: 90, bought: 31}, TransferMerchant {gold: 3000, sold: 20, bought: 40}];
            let mut raw_merchants = vec![0u8; 2 * 0x650];
            for (i, m) in merchants.iter().enumerate() { for (at, value) in [(0, m.gold), (0x4b8, m.sold), (0x4bc, m.bought)] {
                raw_merchants[i * 0x650 + at..i * 0x650 + at + 4].copy_from_slice(&value.to_le_bytes()); }}
            let mut facilities = FacilityPool { records: vec![[0xff; 20]; 16], free: 0 };
            for i in 0..16 { put(&mut facilities.records[i], 8, if i == 15 { u16::MAX } else { i as u16 + 1 }); }
            let input = serde_json::json!({"binary":binary,"town":hex(&town.0[..]),"records":hex(&site.records),
                "offices":hex(&offices.concat()),"merchants":hex(&raw_merchants),"merchant_count":2,
                "facilities":hex(&facilities.records.concat()),"facility_count":16,"free":0,"object":0,"buyer":1,"price":amount});
            let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../oracle/x86_c10_auction_transfer.py");
            let mut child = std::process::Command::new(&python).arg(script).stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();
            child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();
            let output = child.wait_with_output().unwrap();
            assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
            let expected: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            let out = transfer_existing_offices(&town, &site, &offices, &merchants, &facilities, 0, 1, amount, false).unwrap();
            assert_eq!(out.town.0.as_slice(), unhex(expected["town"].as_str().unwrap()));
            assert_eq!(out.site.records, unhex(expected["records"].as_str().unwrap()));
            assert_eq!(out.offices.concat(), unhex(expected["offices"].as_str().unwrap()));
            assert_eq!(out.facilities.records.concat(), unhex(expected["facilities"].as_str().unwrap()));
            assert_eq!(u64::from(out.facilities.free), expected["free"].as_u64().unwrap());
            for (i, m) in out.merchants.iter().enumerate() { for (at, value) in [(0, m.gold), (0x4b8, m.sold), (0x4bc, m.bought)] {
                raw_merchants[i * 0x650 + at..i * 0x650 + at + 4].copy_from_slice(&value.to_le_bytes()); }}
            assert_eq!(raw_merchants, unhex(expected["merchants"].as_str().unwrap()));
        }
    }
}
