//! Datos del ejecutable necesarios para la fase final de expediciones.
use crate::expedition_return_letter::ExpeditionLetterTexts;

#[derive(Clone, Debug)]
pub struct Resources {
    pub texts: ExpeditionLetterTexts,
    pub first_discovery_renown: i16,
    pub finish_renown: i16,
    pub weapon_strength: u8,
    pub no_contract_text: Vec<u8>,
    pub contract_header: Vec<u8>,
    pub no_contract_suffix: Vec<u8>,
    pub contract_texts: crate::trade_contract_text::ContractTexts,
    pub trade: crate::trade_center_data::TradeCenterData,
    pub trade_texts: crate::trade_center_text::TradeReportTexts,
}
impl Resources {
    /// Comprueba el SHA-256 del EXE inglés antes de extraer sus tablas.
    pub fn from_english_exe(exe: &[u8]) -> Result<Self, String> {
        let texts = ExpeditionLetterTexts::from_english_exe(exe)?;
        let image = p3_assets::exe::PeImage::parse(exe).map_err(str::to_owned)?;
        let word = |address| -> Result<i16, String> {
            Ok(i16::from_le_bytes([
                image.byte_at(address).map_err(str::to_owned)?,
                image.byte_at(address + 1).map_err(str::to_owned)?,
            ]))
        };
        let contract_header = image.pointed_cstr(0x6b1774).map_err(str::to_owned)?.to_vec();
        let no_contract_suffix = image.pointed_cstr(0x6b1770).map_err(str::to_owned)?.to_vec();
        let mut no_contract_text = contract_header.clone();
        no_contract_text.extend_from_slice(&no_contract_suffix);
        Ok(Self {
            texts,
            no_contract_text,
            contract_header,
            no_contract_suffix,
            contract_texts: crate::trade_contract_text::ContractTexts::from_english_exe(exe)?,
            trade: crate::trade_center_data::TradeCenterData::from_english_exe(exe)?,
            trade_texts: crate::trade_center_text::TradeReportTexts::from_english_exe(exe)?,
            first_discovery_renown: word(0x672dd2)?,
            finish_renown: word(0x672dd0)?,
            weapon_strength: image.byte_at(0x672cc8).map_err(str::to_owned)?,
        })
    }
}

/// Banco de nombres cargado del CPR y del S05. Fuera de rango el original
/// devuelve el comienzo de la tabla correspondiente (primera entrada).
pub struct CaptainNameBank<'a> {
    pub given: &'a [Vec<Vec<u8>>; 2],
    pub family: &'a [Vec<u8>],
}
impl super::expedition_return_stage::CaptainNames for CaptainNameBank<'_> {
    fn names(&self, given: u8, family: u8) -> Result<(&[u8], &[u8]), String> {
        let table = &self.given[usize::from(given >> 7)];
        let first = table
            .get(usize::from(given & 0x7f))
            .or_else(|| table.first())
            .ok_or("banco de nombres de pila vacío")?;
        let last = self
            .family
            .get(usize::from(family))
            .or_else(|| self.family.first())
            .ok_or("banco de apellidos vacío")?;
        Ok((first, last))
    }
}

/// Frontera estricta para viajes que no visitan centros: nunca inventa ventas.
#[derive(Clone)]
pub struct NoCenters(pub Vec<u8>);
impl crate::expedition_visit::CenterHost for NoCenters {
    fn visit(
        &mut self,
        _: &mut crate::expedition_visit::VisitState,
        _: usize,
        _: usize,
        _: u16,
        _: bool,
    ) -> Result<crate::expedition_visit::CenterOutcome, String> {
        Err("comercio de centro todavía no conectado".into())
    }
}
impl crate::expedition_return_stage::ReturnHost for NoCenters {
    fn navigation_after_visits(
        &self,
        before: &crate::navigation::LiveVoyage,
    ) -> Result<crate::navigation::LiveVoyage, String> {
        Ok(before.clone())
    }
    fn sales_text(&self) -> &[u8] {
        &[]
    }
    fn contracts_text(&self) -> &[u8] {
        &self.0
    }
}

/// Puente entre la sesión naval y el anfitrión de centros independiente.
/// Las ramas comerciales no contrastadas se rechazan desde NativeCenterHost.
#[derive(Clone)]
pub enum CenterAdapter {
    NoCenters(NoCenters),
    Native {
        host: crate::expedition_center_host::NativeCenterHost,
        no_contract_suffix: Vec<u8>,
    },
}
impl crate::expedition_visit::CenterHost for CenterAdapter {
    fn visit(
        &mut self,
        state: &mut crate::expedition_visit::VisitState,
        center: usize,
        waypoint: usize,
        mode: u16,
        allowed: bool,
    ) -> Result<crate::expedition_visit::CenterOutcome, String> {
        match self {
            Self::NoCenters(host) => host.visit(state, center, waypoint, mode, allowed),
            Self::Native { host, .. } => host.visit(state, center, waypoint, mode, allowed),
        }
    }
}
impl crate::expedition_return_stage::ReturnHost for CenterAdapter {
    fn navigation_after_visits(
        &self,
        before: &crate::navigation::LiveVoyage,
    ) -> Result<crate::navigation::LiveVoyage, String> {
        match self {
            Self::NoCenters(_) => Ok(before.clone()),
            Self::Native { host, .. } => {
                let mut after = before.clone();
                let ships = host.ships.concat();
                if ships.len() != before.snapshot.ships.len()
                    || host.convoys.len() != before.snapshot.convoys.len()
                    || host.captains != before.snapshot.captains
                {
                    return Err("flota comercial incompatible con navegación".into());
                }
                after.snapshot.ships = ships;
                after.snapshot.convoys = host.convoys.clone();
                Ok(after)
            }
        }
    }
    fn sales_text(&self) -> &[u8] {
        match self {
            Self::NoCenters(_) => &[],
            Self::Native { host, .. } => &host.trade_text,
        }
    }
    fn contracts_text(&self) -> &[u8] {
        match self {
            Self::NoCenters(host) => &host.0,
            Self::Native { host, .. } => &host.contract_text,
        }
    }
    fn no_contract_suffix(&self) -> &[u8] {
        match self {
            Self::NoCenters(_) => &[],
            Self::Native { no_contract_suffix, .. } => no_contract_suffix,
        }
    }
}
