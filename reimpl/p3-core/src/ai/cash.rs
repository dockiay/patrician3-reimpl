//! IA con deuda (C06): `merchant_ai_raise_cash_or_go_bankrupt` (0x004F4E50).

/// Operación que registra la IA al reunir dinero y al quebrar.
const RAISE_CASH: u32 = 0x29;
const BANKRUPTCY: u32 = 0x65;
const DEBT_LIMIT: i32 = -50000;
/// Subvención que recibe la IA con el indicador `0x006DE52A`.
const GRANT: i32 = 0x3A98;

/// Lo que la función pide fuera del comerciante.
pub trait CashHost {
    /// `merchant_ai_queue_building_work` (0x004F5A20): ¿pudo encolar la venta de un edificio?
    fn queue_building_work(&mut self, task: u32) -> bool;
    /// `scheduled_tasks_merchant_has_building_in_auction` (0x004DB220).
    fn has_building_in_auction(&mut self, task: u8) -> bool;
    /// `_rand` del CRT (0..32767).
    fn rand(&mut self) -> i32;
    /// `enqueue_operation` (0x0054AA70): los cinco dwords del registro.
    fn enqueue_operation(&mut self, record: &[u32; 5]);
}

/// Globales de las que lee: `0x006DE52A` (¿modo sin quiebras?) y `0x006DF31C` (¿red?).
pub struct CashEnv {
    pub flag_2a: bool,
    pub flag_31c: bool,
}

/// `gold` es `Merchant+0`, `home` su ciudad natal (`+0x19`) y `task` el argumento de la IA (el índice de comerciante).
pub fn raise_cash_or_go_bankrupt(gold: &mut i32, home: u8, task: u32, env: &CashEnv, host: &mut dyn CashHost) {
    if env.flag_2a {
        *gold = gold.wrapping_add(GRANT);
        return;
    }
    if env.flag_31c {
        return;
    }
    let amount: u32;
    let wanted: u16;
    if *gold < DEBT_LIMIT {
        if !host.queue_building_work(task) && !host.has_building_in_auction(task as u8) {
            host.enqueue_operation(&[BANKRUPTCY, task & 0xFF, 0, 0, 0]);
            return;
        }
        amount = 50000;
        wanted = 50000;
    } else {
        let r = host.rand() % 30000;
        if r < gold.wrapping_neg() {
            host.queue_building_work(task);
        }
        wanted = 10000u16.wrapping_sub(*gold as u16);
        amount = (wanted as u32 >> 3) * 7;
    }
    let mut record = [RAISE_CASH, amount, 0, 0, 0];
    // +0xC: palabra con lo pedido y los bytes 0xC, 0; +0x10: 0xFF, tarea, ciudad natal, 0xF0.
    record[3] = wanted as u32 | 0x000C_0000;
    record[4] = 0xFF | (task & 0xFF) << 8 | (home as u32) << 16 | 0xF0 << 24;
    host.enqueue_operation(&record);
}
