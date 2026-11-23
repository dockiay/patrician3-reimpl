//! Pantalla de estadísticas del jugador (`StatisticsWindow`, series de `MerchantStatsHistory`). Solo se muestran los campos del
//! registro de 0x6C bytes cuya procedencia está acreditada en `merchant_statistics::capture`; el resto de campos sigue sin nombre.
use crate::Session;
use serde_json::{json, Value};

const REC: usize = 0x6c;
/// (clave, offset en el registro, tipo: 'f' float, 'w' u32, 'h' u16, 'i' i32). Procedencia: `merchant_statistics::capture`.
const FIELDS: [(&str, usize, char); 11] = [
    ("company_value", 0, 'f'),
    ("ship_sales_income", 0x14, 'f'),
    ("ship_purchases", 0x18, 'f'),
    ("stock_quantity", 0x48, 'f'),
    ("stock_value", 0x4c, 'f'),
    ("ships_built", 0x54, 'h'),
    ("buildings", 0x56, 'h'),
    ("houses", 0x58, 'h'),
    ("offices", 0x5a, 'h'),
    ("workers", 0x5c, 'w'),
    ("home_reputation", 0x64, 'f'),
];

fn read(rec: &[u8], at: usize, kind: char) -> f64 {
    match kind {
        'f' => f64::from(f32::from_le_bytes(rec[at..at + 4].try_into().unwrap())),
        'h' => f64::from(u16::from_le_bytes([rec[at], rec[at + 1]])),
        'i' => f64::from(i32::from_le_bytes(rec[at..at + 4].try_into().unwrap())),
        _ => f64::from(u32::from_le_bytes(rec[at..at + 4].try_into().unwrap())),
    }
}

/// Modos de comparación de comerciantes (`StatisticsWindow` 0x4A19C0, `STATISTICS_03.md`): (modo, offset del registro, tipo).
const COMPARE_MODES: [(u32, usize, char); 9] = [
    (0x10, 0, 'f'),
    (0x11, 0x50, 'i'),
    (0x12, 0x68, 'f'),
    (0x16, 0x5c, 'w'),
    (0x17, 0x60, 'w'),
    (0x18, 0x54, 'h'),
    (0x19, 0x56, 'h'),
    (0x1a, 0x58, 'h'),
    (0x1b, 0x64, 'f'),
];

impl Session {
    /// Texto para un byte de titular de cargo (`Town+0x6F1` alcalde, escaños del concejo): el comerciante si el índice es menor que el
    /// número de comerciantes (el original compara con `[0x6DE4AA]`); `0xFF` es vacante y el resto son notables de la ciudad sin registro.
    pub(crate) fn office_holder_label(&self, holder: u8) -> String {
        let h = usize::from(holder);
        if holder == 0xff {
            "vacante".into()
        } else if h == self.owner {
            "tú".into()
        } else if h < self.sim.rep_merchants.len() {
            self.merchant_label(h).unwrap_or_else(|| format!("comerciante {h}"))
        } else {
            "notable de la ciudad".into()
        }
    }

    /// Nombre de pila y apellido de un comerciante como los cachea el original (`0x4F6420`/`0x4F6450`: bit 7 del nombre según el sexo).
    pub(crate) fn merchant_label(&self, owner: usize) -> Option<String> {
        let b = &self.sim.rep_merchants.get(owner)?.bytes;
        let first = if b[0x1a] != 0 { b[0x1c] & 0x7f } else { b[0x1c] | 0x80 };
        let (first, family) = (self.sim.letter_first_name(u32::from(first))?, self.sim.letter_family_name(u32::from(b[0x1d]))?);
        // La leyenda del original usa la inicial del nombre (`+E8`) y el apellido (`+E4`).
        Some(format!("{}. {}", *first.first()? as char, String::from_utf8_lossy(&family)))
    }

    /// Comparación de comerciantes: series de un modo de la tabla verificada para hasta cuatro comerciantes (`owners`), con el periodo
    /// `period` (`daily`, `weekly`, `monthly`). Las medias de todos los nodos y los modos sin productor revisado quedan fuera.
    pub fn statistics_compare(&self, mode: u32, owners: &[usize], period: &str) -> Result<Value, String> {
        let (_, at, kind) = *COMPARE_MODES.iter().find(|m| m.0 == mode).ok_or("modo de comparación no verificado")?;
        if owners.is_empty() || owners.len() > 4 {
            return Err("la comparación admite de uno a cuatro comerciantes".into());
        }
        let stats = self.sim.host.statistics.as_ref().ok_or("el guardado no incluye estadísticas")?;
        let mut series = Vec::new();
        for &owner in owners {
            let item = stats.items.iter().find(|i| usize::from(u16::from_le_bytes([i[0], i[1]])) == owner).ok_or("sin histórico del comerciante")?;
            let (day, week, month) = (usize::from(u16::from_le_bytes([item[2], item[3]])), usize::from(item[4]), usize::from(item[5]));
            let rec = |i: usize| &item[8 + i * REC..8 + (i + 1) * REC];
            let values: Vec<f64> = match period {
                "daily" => (0..32).map(|k| read(rec((day + 1 + k) % 32), at, kind)).collect(),
                "weekly" => (0..24).map(|k| read(rec(32 + (week + 1 + k) % 24), at, kind)).collect(),
                "monthly" => (0..24).map(|k| read(rec(56 + (month + 1 + k) % 24), at, kind)).collect(),
                _ => return Err("periodo desconocido".into()),
            };
            series.push(json!({"owner": owner, "label": self.merchant_label(owner), "values": values}));
        }
        Ok(json!({"mode": mode, "period": period, "series": series}))
    }

    /// Series de ciudad de una mercancía (`GameStatistics` cadena diaria/semanal/mensual/anual, nodos de `0x204` bytes por ciudad,
    /// `town_statistics::capture`). Métricas por mercancía `ware` (24): `consumption` +0 (empresas + ciudadanos + oficinas), `production` +0x60,
    /// `stock` +0xC0, `incoming` +0x120 y `trade_flow` +0x180. La más antigua primero.
    pub fn statistics_town(&self, city: usize, ware: usize, metric: &str, period: &str) -> Result<Value, String> {
        let base = match metric {
            "consumption" => 0,
            "production" => 0x60,
            "stock" => 0xc0,
            "incoming" => 0x120,
            "trade_flow" => 0x180,
            _ => return Err("métrica de ciudad desconocida".into()),
        };
        let chain = match period {
            "daily" => 0,
            "weekly" => 1,
            "monthly" => 2,
            "yearly" => 3,
            _ => return Err("periodo desconocido".into()),
        };
        if ware >= 24 {
            return Err("mercancía fuera de rango".into());
        }
        let stats = self.sim.host.statistics.as_ref().ok_or("el guardado no incluye estadísticas")?;
        let nodes = stats.chains[chain].as_ref().ok_or("cadena de estadísticas ausente")?;
        let mut values = Vec::new();
        for (count, record) in nodes.iter().rev() {
            if city >= *count as usize {
                return Err("ciudad fuera de la muestra".into());
            }
            let at = city * 0x204 + base + 4 * ware;
            values.push(i32::from_le_bytes(record[at..at + 4].try_into().unwrap()));
        }
        Ok(json!({"city": city, "ware": ware, "metric": metric, "period": period, "values": values}))
    }

    pub(crate) fn screen_statistics_action(&mut self, name: &str, action: &str, args: &Value) -> Option<Result<Value, String>> {
        if name != "statistics" {
            return None;
        }
        Some(match action {
            "compare" => {
                let owners: Vec<usize> = args["owners"].as_array().map(|a| a.iter().filter_map(|v| v.as_u64().map(|n| n as usize)).collect()).unwrap_or_default();
                self.statistics_compare(args["mode"].as_u64().unwrap_or(0) as u32, &owners, args["period"].as_str().unwrap_or("daily"))
            }
            "town" => {
                let get = |k: &str| args[k].as_u64().map(|n| n as usize);
                match (get("city"), get("ware")) {
                    (Some(city), Some(ware)) => self.statistics_town(city, ware, args["metric"].as_str().unwrap_or("stock"), args["period"].as_str().unwrap_or("daily")),
                    _ => Err("faltan city y ware".into()),
                }
            }
            _ => Err(format!("acción desconocida: {action}")),
        })
    }

    /// Series diaria (hasta 32, la más antigua primero), semanal y mensual del jugador. `None` si no hay `.pst`.
    pub fn statistics_screen(&self) -> Result<Value, String> {
        let stats = self.sim.host.statistics.as_ref().ok_or("el guardado no incluye estadísticas")?;
        let item = stats
            .items
            .iter()
            .find(|i| usize::from(u16::from_le_bytes([i[0], i[1]])) == self.owner)
            .ok_or("sin histórico del comerciante")?;
        let day = usize::from(u16::from_le_bytes([item[2], item[3]]));
        let (week, month) = (usize::from(item[4]), usize::from(item[5]));
        let years = usize::from(u16::from_le_bytes([item[6], item[7]]));
        let rec = |i: usize| &item[8 + i * REC..8 + (i + 1) * REC];
        let series = |ring: &dyn Fn(usize) -> usize, count: usize| -> Value {
            let mut map = serde_json::Map::new();
            for (name, at, kind) in FIELDS {
                map.insert(name.into(), json!((0..count).map(|k| read(rec(ring(k)), at, kind)).collect::<Vec<_>>()));
            }
            Value::Object(map)
        };
        let daily = series(&|k| (day + 1 + k) % 32, 32);
        let weekly = series(&|k| 32 + (week + 1 + k) % 24, 24);
        let monthly = series(&|k| 56 + (month + 1 + k) % 24, 24);
        Ok(json!({"owner": self.owner, "label": self.merchant_label(self.owner), "merchants": stats.items.iter().map(|i| usize::from(u16::from_le_bytes([i[0], i[1]]))).collect::<Vec<_>>(), "years": years, "daily": daily, "weekly": weekly, "monthly": monthly}))
    }
}
