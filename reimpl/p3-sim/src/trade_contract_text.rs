//! Formato de contratos de 0x52C6C0; recursos tomados del EXE inglés verificado.
use p3_core::economy::{
    cformat::{self, FArg},
    tavern_passage::date_from_tick,
    trade_center_contract::Report,
};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug)]
pub struct ContractTexts {
    fmt_create: Vec<u8>,
    directions: [Vec<u8>; 2],
    fmt_done: Vec<u8>,
    actions: [Vec<u8>; 2],
    fmt_remaining: Vec<u8>,
    complete: Vec<u8>,
    towns: Vec<Vec<u8>>,
    wares: Vec<Vec<u8>>,
}
impl ContractTexts {
    pub fn from_english_exe(bytes: &[u8]) -> Result<Self, String> {
        if format!("{:x}", Sha256::digest(bytes)) != "fe436dc5f8addc4d3aa437eb48a8bc6415d9ba5577c1027d06db9bb915eddece"
        {
            return Err("ejecutable inglés no reconocido para contratos".into());
        }
        let pe = p3_assets::exe::PeImage::parse(bytes).map_err(str::to_owned)?;
        let get = |addr| pe.pointed_cstr(addr).map(|v| v.to_vec()).map_err(str::to_owned);
        Ok(Self {
            fmt_create: get(0x6b1764)?,
            directions: [get(0x6b176c)?, get(0x6b1768)?],
            fmt_done: get(0x6b1778)?,
            actions: [get(0x6b177c)?, get(0x6b1780)?],
            fmt_remaining: get(0x6b1790)?,
            complete: get(0x6b1794)?,
            towns: (0..30).map(|i| get(0x6b1f20 + 4 * i)).collect::<Result<_, _>>()?,
            wares: (0..24).map(|i| get(0x6c0bb0 + 4 * i)).collect::<Result<_, _>>()?,
        })
    }
}
fn date(tick: u32) -> Result<(u8, u8, u16), String> {
    if tick > i32::MAX as u32 {
        return Err("fecha de contrato fuera de rango acreditado".into());
    }
    Ok(date_from_tick(tick as i32))
}
fn positional(fmt: &[u8], args: &[FArg]) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < fmt.len() {
        if fmt[i..].starts_with(b"%$") {
            let mut n = 0usize;
            i += 2;
            while i < fmt.len() && fmt[i].is_ascii_digit() {
                n = n * 10 + usize::from(fmt[i] - b'0');
                i += 1
            }
            if n == 0 || fmt.get(i) != Some(&b'$') {
                return Err("formato posicional inválido".into());
            }
            i += 1;
            let ty = *fmt.get(i).ok_or("formato posicional truncado")?;
            i += 1;
            let spec = [b'%', ty];
            out.extend(cformat::format(
                &spec,
                &[args.get(n - 1).ok_or("argumento posicional ausente")?.clone()],
            )?)
        } else {
            out.push(fmt[i]);
            i += 1
        }
    }
    Ok(out)
}
pub fn append_existing(
    t: &ContractTexts,
    center: &[u8],
    report: &Report,
    price: i32,
    remaining: i32,
    before: &[u8],
) -> Result<Vec<u8>, String> {
    if center.len() < 6 || report.slot >= 2 || report.direction >= 2 {
        return Err("contrato de informe inválido".into());
    }
    let slot = usize::from(report.slot);
    let ware = usize::from(center[slot + if report.direction == 0 { 0 } else { 2 }]);
    let town = t
        .towns
        .get(usize::from(center[4]))
        .ok_or("nombre de centro ausente")?
        .clone();
    let ware = t.wares.get(ware).ok_or("nombre de mercancía ausente")?.clone();
    let args = [
        FArg::Int((report.quantity / 200) as u32),
        FArg::Int(b'B' as u32),
        FArg::Str(ware),
        FArg::Int(price as u32),
        FArg::Str(town),
        FArg::Str(t.actions[usize::from(report.direction)].clone()),
    ];
    let mut out = before.to_vec();
    out.extend(positional(&t.fmt_done, &args)?);
    if report.completed {
        out.extend_from_slice(&t.complete)
    } else {
        let (d, m, y) = date(report.expiry)?;
        out.extend(cformat::format(
            &t.fmt_remaining,
            &[
                FArg::Int(remaining as u32),
                FArg::Int(b'B' as u32),
                FArg::Int(d as u32),
                FArg::Int(m as u32),
                FArg::Int(y as u32),
            ],
        )?)
    }
    Ok(out)
}
pub fn append_created(t: &ContractTexts, center: &[u8], contract: &[u8; 16], before: &[u8]) -> Result<Vec<u8>, String> {
    if center.len() < 6 || contract[2] >= 2 || contract[3] >= 2 {
        return Err("contrato nuevo inválido".into());
    }
    let slot = usize::from(contract[2]);
    let dir = usize::from(contract[3]);
    let ware = usize::from(center[slot + if dir == 0 { 0 } else { 2 }]);
    let n = i32::from_le_bytes(contract[4..8].try_into().unwrap());
    let price = i32::from_le_bytes(contract[8..12].try_into().unwrap());
    let (d, m, y) = date(u32::from_le_bytes(contract[12..16].try_into().unwrap()))?;
    let args = [
        FArg::Str(
            t.towns
                .get(usize::from(center[4]))
                .ok_or("nombre de centro ausente")?
                .clone(),
        ),
        FArg::Str(t.directions[dir].clone()),
        FArg::Int(n as u32),
        FArg::Int(b'B' as u32),
        FArg::Str(t.wares.get(ware).ok_or("nombre de mercancía ausente")?.clone()),
        FArg::Int(d as u32),
        FArg::Int(m as u32),
        FArg::Int(y as u32),
        FArg::Int(price as u32),
    ];
    let mut out = before.to_vec();
    out.extend(cformat::format(&t.fmt_create, &args)?);
    Ok(out)
}
