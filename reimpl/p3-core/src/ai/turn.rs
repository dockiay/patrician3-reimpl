//! Turno de la IA de un comerciante (C06): `merchant_run_ai_turn` (0x004F4940).

/// Lo que lee y escribe del comerciante.
#[derive(Clone, Copy, Debug)]
pub struct TurnMerchant {
    /// `+0`: dinero.
    pub gold: i32,
    /// `+8`: marcas (2|4: jugador o IA especial; sin ellas es una IA corriente).
    pub flags: u8,
    /// `+0x547`: se devuelve como parte del resultado.
    pub b547: u8,
}

/// Contadores y datos de la tarea programada del comerciante (`0x006DD73C`).
#[derive(Clone, Copy, Debug)]
pub struct TurnTask {
    /// `+8`: identificador del comerciante en la tarea (el argumento de las acciones).
    pub id: u16,
    /// `+0x10`: cuenta de turnos para los objetivos de existencias.
    pub stock_counter: u8,
    /// `+0x11`: cuenta de turnos para barcos.
    pub fleet_counter: u8,
    /// `+0xE`: cuenta de turnos para la vivienda.
    pub housing_counter: u8,
}

/// Las acciones que lanza el turno.
pub trait TurnHost {
    /// `update_office_trade_stock_targets` (0x004F4F80).
    fn update_stock_targets(&mut self);
    /// `mark_ship_for_arming` (0x004F5080).
    fn mark_ship_for_arming(&mut self, task: u16);
    /// `assign_ship_home_towns` (0x004F5130).
    fn assign_ship_home_towns(&mut self);
    /// `raise_cash_or_go_bankrupt` (0x004F4E50).
    fn raise_cash(&mut self, task: u8);
    /// `choose_housing` (0x004F4B40).
    fn choose_housing(&mut self, task: u16);
    /// `try_join_guild` (0x004F4A20).
    fn try_join_guild(&mut self, task: u16);
}

/// Un turno. `network_game` es el dword `0x006DF31C`. Devuelve `0x100 + Merchant+0x547`.
pub fn run_ai_turn(m: &TurnMerchant, task: &mut TurnTask, network_game: u32, host: &mut dyn TurnHost) -> u32 {
    task.stock_counter = task.stock_counter.wrapping_add(1);
    if task.stock_counter > 3 {
        task.stock_counter = 0;
        host.update_stock_targets();
    }
    task.fleet_counter = task.fleet_counter.wrapping_add(1);
    if task.fleet_counter > 0x1E {
        task.fleet_counter = 0;
        if m.flags & 6 != 0 {
            host.mark_ship_for_arming(task.id);
        } else {
            host.assign_ship_home_towns();
        }
    }
    if m.gold < 0x1388 {
        host.raise_cash(task.id as u8);
    }
    if m.flags & 6 == 0 && network_game == 0 {
        task.housing_counter = task.housing_counter.wrapping_add(1);
        if task.housing_counter != 0 {
            host.choose_housing(task.id);
            task.housing_counter = 0;
        }
        host.try_join_guild(task.id);
    }
    0x100 + m.b547 as u32
}
