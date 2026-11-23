//! Funciones pequeñas del EXE sobre la imagen de memoria (accesores, constructores, tamaños de guardado, listas enlazadas y limpiezas). Cada una sigue la
//! convención del original: `this` es ECX y `a` los argumentos de pila en orden; devuelve EAX (en las que el original solo escribe AX o AL, el
//! resto de bits es ruido y la prueba compara solo los bits definidos). Las llamadas a otras funciones (liberar `0x0064F7E2`, reservar `0x0064F7B9`,
//! recálculos de barco…) van por [`OpsHost`] con la dirección del original.
use crate::economy::ops_misc::OpsHost;
use crate::flatmem::Mem;

const FREE: u32 = 0x0064_f7e2;
const ALLOC: u32 = 0x0064_f7b9;
const SHIPS: u32 = 0x006d_d7a4;
const SHIP_COUNT: u32 = 0x006d_d894;
const LETTER_TABLE: u32 = 0x006d_d7a0;
const LETTER_COUNT: u32 = 0x006d_d892;

fn bool32(v: bool) -> u32 {
    u32::from(v)
}

pub fn town_has_flag_0x10(m: &mut dyn Mem, this: u32) -> u32 {
    bool32(m.r32(this + 0x2c8) & 0x10 == 0x10)
}
pub fn facility_get_employees_max(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r16(this + 0xc))
}
pub fn town_tick_entry_get_type(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r8(this + 0x10))
}
pub fn game_world_get_town(m: &mut dyn Mem, this: u32, index: u32) -> u32 {
    m.r32(this + 0x68).wrapping_add(index.wrapping_mul(0x9f8))
}
pub fn game_world_get_towns_count(m: &mut dyn Mem, this: u32) -> u32 {
    m.r32(this + 0x10)
}
pub fn operations_get_field_2c(m: &mut dyn Mem, this: u32) -> u32 {
    m.r32(this + 0x2c)
}
pub fn town_get_chapels(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r8(this + 0x771))
}
pub fn town_get_hospitals(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r8(this + 0x773))
}
/// `0x004F3790`: suma `v` (byte) al contador `+0x157` del barco (a 0 si llega a 0x7F) y encadena con `0x00517770(4)`.
pub fn ship_add_counter_0x157(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, v: u32) -> u32 {
    let sum = m.r8(this + 0x157).wrapping_add(v as u8);
    m.w8(this + 0x157, sum);
    if sum >= 0x7f {
        m.w8(this + 0x157, 0);
    }
    host.call(m, 0x517770, this, &[4])
}
pub fn convoy_get_leader_ship_index(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r16(this + 0x10))
}
pub fn town_has_assigned_slot(m: &mut dyn Mem, this: u32) -> u32 {
    bool32(m.r8(this + 0x82c) < 0x14)
}
pub fn game_world_get_start_day(m: &mut dyn Mem, this: u32) -> u32 {
    m.r32(this + 0xc)
}
pub fn game_world_get_game_time(m: &mut dyn Mem, this: u32) -> u32 {
    m.r32(this + 0x14)
}
pub fn ships_get_convoy(m: &mut dyn Mem, this: u32, index: u32) -> u32 {
    m.r32(this + 8).wrapping_add(index.wrapping_mul(60))
}
pub fn ships_get_docked_list_head(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r16(this + 0xe8))
}
pub fn ships_get_at_sea_list_head(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r16(this + 0xea))
}
pub fn message_pool_get_message(m: &mut dyn Mem, this: u32, index: u32) -> u32 {
    if index > u32::from(m.r16(this + 6)) {
        0
    } else {
        m.r32(this).wrapping_add(index << 4)
    }
}
pub fn name_bank_get_given_names_count_table1(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r16(this + 0xd0))
}
pub fn name_bank_get_given_names_count_table2(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r16(this + 0xd2))
}
pub fn name_bank_get_surnames_count(m: &mut dyn Mem, this: u32) -> u32 {
    u32::from(m.r16(this + 0xd4))
}
pub fn popup_text_descriptor_init(m: &mut dyn Mem, this: u32) -> u32 {
    m.w32(this, 0);
    m.w8(this + 4, 0);
    m.w16(this + 6, 0);
    m.w32(this + 8, 0);
    m.w8(this + 0xc, 0xff);
    m.w32(this + 0x10, 0x3fb0_0000);
    m.w8(this + 0x14, 1);
    m.w16(this + 0x16, 0);
    m.w32(this + 0x18, 0);
    m.w32(this + 0x1c, 0);
    m.w8(this + 0xf, 0xff);
    m.w8(this + 0xd, 0);
    m.w8(this + 0xe, 0);
    0
}
pub fn sea_battle_is_idle(m: &mut dyn Mem, this: u32) -> u32 {
    bool32(m.r8(this + 0x3fd) == 0 && m.r8(this + 0x3fc) == 0xff)
}
pub fn auto_trader_get_record_size() -> u32 {
    0x10
}
/// `0x004FE160`: jornal del comerciante automático a partir de su habilidad comercial (`+0xA`): `20 * (habilidad * 0x2FA0BE83 >> 35) + 10`.
pub fn auto_trader_set_wage_from_trade_skill(m: &mut dyn Mem, this: u32) -> u32 {
    let skill = i64::from(m.r8(this + 0xa));
    let hi = ((i64::from(0x2fa0_be83i32) * skill) >> 32) as i32;
    let mut q = hi >> 3;
    q = q.wrapping_add(((q as u32) >> 31) as i32);
    let v = q.wrapping_mul(5).wrapping_mul(4).wrapping_add(0xa) as u32;
    m.w16(this + 0xc, v as u16);
    v
}
pub fn church_init(m: &mut dyn Mem, this: u32) -> u32 {
    m.w32(this + 8, 0xff);
    m.w32(this, 0);
    m.w32(this + 4, 0);
    for k in 0xc..0x10 {
        m.w8(this + k, 0);
    }
    0
}
pub fn office_construct(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    host.call(m, 0x52de70, this, &[]);
    host.call(m, 0x4febd0, this, &[0]);
    this
}
pub fn office_get_save_size(m: &mut dyn Mem, this: u32) -> u32 {
    if m.r16(this + 0x2c4).wrapping_add(1) != 0 {
        0x44c
    } else {
        6
    }
}
pub fn office_get_administrator_daily_cost(m: &mut dyn Mem, this: u32) -> u32 {
    let letter = m.r16(this + 0x2f2);
    if letter >= m.r16(LETTER_COUNT) {
        return 0;
    }
    let table = m.r32(LETTER_TABLE);
    let base = u32::from(m.r16(table.wrapping_add(u32::from(letter) << 4).wrapping_add(0xc)));
    base + u32::from(m.r16(this + 0x2d2))
}
pub fn convoy_get_save_size() -> u32 {
    0x3a
}
/// `0x005038F0`: número de barcos de la cadena del convoy (empieza en `+0xA`, sigue por `barco+6` mientras el índice sea menor que el recuento).
pub fn convoy_count_ships(m: &mut dyn Mem, this: u32) -> u32 {
    let mut index = u32::from(m.r16(this + 0xa));
    let count = u32::from(m.r16(SHIP_COUNT));
    let mut n = 0;
    if index >= count {
        return 0;
    }
    let table = m.r32(SHIPS);
    loop {
        n += 1;
        index = u32::from(m.r16(table.wrapping_add(index.wrapping_mul(0x180)).wrapping_add(6)));
        if index >= count {
            break;
        }
    }
    n
}
pub fn dword_array_init(m: &mut dyn Mem, this: u32) -> u32 {
    m.w32(this + 8, 0);
    m.w32(this, 0);
    m.w32(this + 4, 0);
    this
}
/// `0x00503D50` / `0x00503D80` (mismo cuerpo): libera el búfer (`+8`) y pone a 0 recuento, capacidad y puntero.
pub fn dword_array_free(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let buffer = m.r32(this + 8);
    m.w32(this, 0);
    m.w32(this + 4, 0);
    if buffer != 0 {
        host.call(m, FREE, 0, &[buffer]);
        m.w32(this + 8, 0);
    }
}
pub fn ships_validate_all_ship_routes(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let mut n = i32::from(m.r16(this + 0xf4));
    while n > 0 {
        n -= 1;
        let ship = m.r32(this + 4).wrapping_add((n as u32).wrapping_mul(0x180));
        host.call(m, 0x517c10, ship, &[n as u32 & 0xffff]);
    }
}
pub fn town_tick_entry_init(m: &mut dyn Mem, this: u32) -> u32 {
    m.w32(this, 0);
    m.w32(this + 4, 0);
    m.w32(this + 8, 0);
    m.w16(this + 0xc, 0);
    m.w16(this + 0xe, 0);
    m.w8(this + 0x10, 0xff);
    m.w8(this + 0x11, 0xff);
    m.w16(this + 0x12, 0);
    this
}
pub fn town_tick_entry_get_serialized_size() -> u32 {
    0x10
}
/// `0x0050FE00(tasa, a, b)`: tasa limitada a 0x400.
pub fn town_tick_entry_init_with_rate(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    m.w8(this + 0x10, a[2] as u8);
    m.w8(this + 0x11, a[1] as u8);
    let rate = a[0] as u16;
    m.w16(this + 0xc, if rate > 0x400 { 0x400 } else { rate });
    let v = m.r16(this + 0xc);
    m.w16(this + 0xe, v);
    m.w32(this + 8, 0);
    m.w32(this + 4, 0);
    m.w16(this + 0x12, 0);
    0
}
/// `0x0050FFB0`: suministro efectivo = `+8 * tasa >> 10` (tasa `+0xE` < 0x400); 0 sin tasa y `+8` tal cual con la tasa llena.
pub fn town_tick_entry_get_effective_supply(m: &mut dyn Mem, this: u32) -> u32 {
    let rate = m.r16(this + 0xe);
    if rate < 1 {
        return 0;
    }
    let supply = m.r32(this + 8);
    if rate >= 0x400 {
        return supply;
    }
    ((i64::from(supply as i32) * i64::from(rate)) >> 10) as u32
}
pub fn facility_get_serialized_size() -> u32 {
    0x10
}
pub fn facility_init(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    m.w8(this + 6, a[1] as u8);
    m.w32(this, 0);
    m.w16(this + 4, 0);
    m.w8(this + 7, a[0] as u8);
    m.w16(this + 8, 0);
    m.w16(this + 0xa, 0);
    m.w32(this + 0xc, 0);
    0
}
pub fn script_blob_pool_init(m: &mut dyn Mem, this: u32) -> u32 {
    m.w32(this, 0x0067_3608);
    m.w16(this + 8, 0);
    m.w32(this + 4, 0);
    this
}
pub fn pirate_band_get_pseudo_town_id(m: &mut dyn Mem, this: u32) -> u32 {
    0xfe - u32::from(m.r8(this + 0xf) & 3)
}
/// `0x005177B0`: tamaño de guardado del barco: 5 si el tipo (`+0x134`) es ≥ 0x17; si no, el núcleo (`0x00516280`) más 0x120.
pub fn ship_get_save_size(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    if m.r16(this + 0x134) >= 0x17 {
        return 5;
    }
    host.call(m, 0x516280, this, &[]).wrapping_add(0x120)
}
pub fn ship_get_index(m: &mut dyn Mem, this: u32) -> u32 {
    let diff = this.wrapping_sub(m.r32(SHIPS)) as i32;
    let hi = ((i64::from(0x2aaa_aaabi32) * i64::from(diff)) >> 32) as i32;
    let q = hi >> 6;
    q.wrapping_add(((q as u32) >> 31) as i32) as u32
}
pub fn ship_clear_ware_placeholders(m: &mut dyn Mem, this: u32) {
    for k in 0..24 {
        if m.r32(this + 0x54 + 4 * k) == 1 {
            m.w32(this + 0x54 + 4 * k, 0);
        }
    }
}
pub fn letter_init(m: &mut dyn Mem, this: u32) -> u32 {
    m.w8(this + 4, 0xff);
    m.w8(this + 5, 0xff);
    m.w8(this, 0);
    m.w8(this + 1, 0x80);
    m.w16(this + 2, 0xffff);
    for k in 0..8 {
        m.w8(this + 8 + k, 0);
    }
    this
}
/// `0x0051A890`: tamaño de guardado de la carta según su tipo (`+4`).
pub fn letter_get_serialized_size(m: &mut dyn Mem, this: u32) -> u32 {
    let kind = m.r8(this + 4);
    if matches!(kind, 0x29 | 0x47 | 0x63 | 0x83) {
        return m.r32(this + 0xc).wrapping_add(0x10);
    }
    if (0x3c..=0x40).contains(&kind) || kind == 0x71 {
        let p = m.r32(this + 8);
        return m.r32(p).wrapping_add(0x18);
    }
    0x10
}
pub fn town_get_economy_block_serialized_size() -> u32 {
    0x6d4
}
pub fn town_get_shipyard_block_serialized_size() -> u32 {
    0x1a
}
pub fn town_set_head_tax_rate(m: &mut dyn Mem, this: u32, a: &[u32]) {
    let v = a[0] as u8;
    m.w8(this + 0x6f8, if v > 0x14 { 0xa } else { v });
}
pub fn list_iterator_post_increment(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let node = m.r32(this);
    let next = m.r32(node);
    m.w32(this, next);
    m.w32(a[0], node);
    a[0]
}
pub fn list_alloc_node(m: &mut dyn Mem, host: &mut dyn OpsHost, a: &[u32]) -> u32 {
    let node = host.call(m, ALLOC, 0, &[0x18]);
    let head = if a[0] == 0 { node } else { a[0] };
    m.w32(node, head);
    m.w32(node + 4, if a[1] != 0 { a[1] } else { node });
    node
}
pub fn trade_center_contract_copy_construct(m: &mut dyn Mem, a: &[u32]) -> u32 {
    if a[0] != 0 {
        for k in 0..4 {
            let v = m.r32(a[1] + 4 * k);
            m.w32(a[0] + 4 * k, v);
        }
    }
    a[0]
}
pub fn storage_clear(m: &mut dyn Mem, this: u32) -> u32 {
    for k in 0..0x2c0 {
        m.w8(this + k, 0);
    }
    this
}
/// `0x0052DFF0(franja, días)`: `días - 1` si el día del año (byte bajo de `0x006DE4B4`) ya pasó la franja.
pub fn town_adjust_days_for_daily_slot(m: &mut dyn Mem, a: &[u32]) -> u32 {
    let mut limit = a[0] << 3;
    limit = if limit >= 0x100 { limit.wrapping_sub(0xfd) } else { limit.wrapping_add(7) };
    if limit >= (m.r32(0x006d_e4b4) & 0xff) {
        a[1].wrapping_sub(1)
    } else {
        a[1]
    }
}
pub fn storage_get_weapon_piece_count(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    if a[0] >= 6 {
        return m.r32(this + 0x2bc);
    }
    let divisor = i64::from(m.r16(0x0067_2cb4 + a[0] * 2));
    let value = i64::from(m.r32(this + a[0] * 4 + 0x124) as i32);
    (value / divisor) as u32
}
pub fn set_trade_difficulty_price_factor(m: &mut dyn Mem, this: u32, a: &[u32]) {
    let v = if (a[0] as u8) < 2 { m.r32(0x0067_39fc + (a[0] & 0xff) * 4) } else { m.r32(0x0067_3a04) };
    m.w32(this + 0x64, v);
}
pub fn get_merchant(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    m.r32(this + 0x78).wrapping_add(a[0].wrapping_mul(0x650))
}
pub fn get_office(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    if a[0] >= u32::from(m.r16(this + 8)) {
        0
    } else {
        m.r32(this + 0x74).wrapping_add(a[0].wrapping_mul(0x44c))
    }
}
pub fn get_office_index(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let diff = a[0].wrapping_sub(m.r32(this + 0x74));
    if (diff as i32) < 0 {
        return 0xffff;
    }
    let hi = ((u64::from(0xdca0_1dcbu32) * u64::from(diff)) >> 32) as u32;
    let q = ((diff.wrapping_sub(hi)) >> 1).wrapping_add(hi) >> 10;
    if (q as i32) < i32::from(m.r16(this + 8)) {
        q
    } else {
        0xffff
    }
}
pub fn trade_offer_construct(m: &mut dyn Mem, this: u32) -> u32 {
    m.w32(this, 0);
    this
}
pub fn trade_offer_list_serialized_size(m: &mut dyn Mem, this: u32) -> u32 {
    let mut size = 0x13;
    let mut node = m.r32(this);
    while node != 0 {
        size += 0x13;
        node = m.r32(node);
    }
    size
}
/// `0x00533E80`: libera los nodos siguientes (el más lejano primero) y después el propio nodo.
pub fn trade_offer_free_following_nodes(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let next = m.r32(this);
    if next != 0 {
        trade_offer_free_following_nodes(m, host, next);
    }
    m.w32(this, 0);
    host.call(m, FREE, 0, &[this]);
}
/// `0x00533EE0(clave, destino)`: busca en la cadena la oferta con la clave (`+4`, `+0xC`, `+0xD`) y la describe con `0x00533F20`.
pub fn trade_offer_describe_by_key(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let key = a[0];
    let mut node = this;
    loop {
        if m.r32(node + 4) == m.r32(key) && u16::from(m.r8(node + 0xc)) == m.r16(key + 4) && u16::from(m.r8(node + 0xd)) == m.r16(key + 6) {
            return host.call(m, 0x533f20, node, &[a[1]]);
        }
        node = m.r32(node);
        if node == 0 {
            return 0;
        }
    }
}
pub fn map_discovery_clear(m: &mut dyn Mem, this: u32) {
    for k in 0..0x60 {
        m.w8(this + k, 0);
    }
}
pub fn town_info_history_construct(m: &mut dyn Mem, this: u32) -> u32 {
    for k in 0..8 {
        m.w8(this + 0xa00 + k, 0xff);
        m.w32(this + 0xa08 + 4 * k, 0);
    }
    this
}
pub fn town_info_history_free_slots(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    for k in 0..8 {
        let p = m.r32(this + 0xa08 + 4 * k);
        if p != 0 {
            host.call(m, FREE, 0, &[p]);
            m.w32(this + 0xa08 + 4 * k, 0);
        }
        m.w8(this + 0xa00 + k, 0xff);
    }
}
pub fn town_info_history_get_snapshot(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    m.r32(this.wrapping_add(((a[0] << 4).wrapping_add(a[1] & 0xf)).wrapping_mul(4)))
}
pub fn operation_record_init(m: &mut dyn Mem, this: u32) -> u32 {
    m.w32(this, 0xffff_ffff);
    for k in [4, 8, 0xc, 0x10] {
        m.w32(this + k, 0);
    }
    this
}
pub fn operation_queue_zero_init(m: &mut dyn Mem, this: u32) -> u32 {
    for k in 0..0x1c {
        m.w8(this + k, 0);
    }
    this
}
pub fn operation_packet_release_next_chain(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let next = m.r32(this + 0x420);
    if next != 0 {
        operation_packet_release_next_chain(m, host, next);
        host.call(m, FREE, 0, &[next]);
        m.w32(this + 0x420, 0);
    }
}
/// `0x005440A0(borrar)`: destructor de paquete: destruye (y libera) la cadena siguiente y, con el bit 0, libera el propio paquete.
pub fn operation_packet_destructor(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let next = m.r32(this + 0x420);
    if next != 0 {
        operation_packet_destructor(m, host, next, &[1]);
        m.w32(this + 0x420, 0);
    }
    if a[0] & 1 != 0 {
        host.call(m, FREE, 0, &[this]);
    }
    this
}
pub fn net_connection_init(m: &mut dyn Mem, this: u32) -> u32 {
    m.w32(this + 0x10, 0);
    m.w32(this + 0x14, 0);
    m.w32(this + 0x18, 1);
    m.w32(this + 0x1c, 0);
    this
}
pub fn custom_map_get_town_name_slot(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let index = if a[0] >= m.r32(this) { 0 } else { a[0] };
    let slot = u32::from(m.r8(this.wrapping_add(index).wrapping_add(0x16c)));
    this.wrapping_add(slot * 4).wrapping_add(0x1bc)
}
pub fn custom_map_next_town_with_flag_40(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let count = m.r32(this);
    let mut index = a[0];
    loop {
        index = index.wrapping_add(1);
        if index >= count {
            index = 0;
        }
        if m.r8(this.wrapping_add(index).wrapping_add(0x144)) & 0x40 == 0x40 {
            return index;
        }
    }
}
pub fn custom_map_prev_town_with_flag_40(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let mut index = a[0];
    loop {
        if index == 0 {
            index = m.r32(this);
        }
        let flags = m.r8(this.wrapping_add(index).wrapping_add(0x143));
        index = index.wrapping_sub(1);
        if flags & 0x40 == 0x40 {
            return index;
        }
    }
}
/// `0x00545890(con_año)`: reinicia astillero y edificios de la ciudad con el año de la partida (o 0) y el valor de `0x006DE4E0[tipo]`.
pub fn town_init_shipyard_and_buildings(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) {
    let year = if a[0] != 0 { u32::from(m.r16(0x006d_e4a2)) } else { 0 };
    host.call(m, 0x545be0, this, &[year]);
    let kind = u32::from(m.r8(this + 0x2c0));
    let v = u32::from(m.r8(0x006d_e4e0 + kind));
    host.call(m, 0x51e8d0, this, &[v]);
}
pub fn town_move_to_and_clear(m: &mut dyn Mem, this: u32, a: &[u32]) {
    for k in 0..0x9f8 {
        let v = m.r8(this + k);
        m.w8(a[0] + k, v);
    }
    for k in 0..0x9f8 {
        m.w8(this + k, 0);
    }
}
pub fn count_connected_clients(m: &mut dyn Mem, this: u32) -> u32 {
    let base = m.r32(this + 0x44);
    let mut n = 0;
    if base != 0 {
        for k in 0..8 {
            if m.r32(base + 0x18 + 0x24 * k) > 1 {
                n += 1;
            }
        }
    }
    n
}
pub fn reset_town_distance_matrix(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let p = m.r32(this + 0x6c);
    if p != 0 {
        host.call(m, FREE, 0, &[p]);
        m.w32(this + 0x6c, 0);
    }
    host.call(m, 0x531bd0, this, &[]);
}
pub fn message_scalar_deleting_destructor(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    host.call(m, 0x51a880, this, &[]);
    if a[0] & 1 != 0 {
        host.call(m, FREE, 0, &[this]);
    }
    this
}

/// 0x00533EA0: recorre la cadena desde `this`; devuelve 0 si algún nodo coincide con (clave, byte0xC, byte0xD), 1 si ninguno (solo AL).
pub fn trade_offer_chain_key_is_free(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let mut node = this;
    loop {
        if m.r32(node + 4) == a[0] && u16::from(m.r8(node + 0xc)) == (a[1] as u16) && u16::from(m.r8(node + 0xd)) == (a[2] as u16) {
            return 0;
        }
        node = m.r32(node);
        if node == 0 {
            return 1;
        }
    }
}
/// 0x00534090 (cdecl, cabeza como primer argumento): 1 si algún nodo coincide, 0 si no (solo AL).
pub fn trade_offer_chain_contains_key(m: &mut dyn Mem, a: &[u32]) -> u32 {
    let mut node = a[0];
    while node != 0 {
        if m.r32(node + 4) == a[1] && u32::from(m.r8(node + 0xc)) == a[2] && u32::from(m.r8(node + 0xd)) == a[3] {
            return 1;
        }
        node = m.r32(node);
    }
    0
}
/// 0x00500EC0: existencias de la ranura `i` descontando lo reservado (solo si el bit está en la máscara +0x3B4 y `i` < contador global).
pub fn office_get_stock_available(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let i = a[0];
    let base = this.wrapping_add(i.wrapping_mul(4));
    let cur = m.r32(base.wrapping_add(4));
    if m.r16(this + 0x2f2) < m.r16(0x006d_d892) && (1u32 << (i & 31)) & m.r32(this + 0x3b4) != 0 {
        let reserved = m.r32(base.wrapping_add(0x354)) as i32;
        if reserved > 0 {
            let d = cur.wrapping_sub(reserved as u32) as i32;
            return if d <= 0 { 0 } else { d as u32 };
        }
    }
    cur
}
/// 0x004FE190: salario diario del comerciante automático a partir de sus cuatro habilidades (bytes 8..0xB).
pub fn auto_trader_set_daily_wage_from_skills(m: &mut dyn Mem, this: u32) {
    let sum = i32::from(m.r8(this + 0xb)) + i32::from(m.r8(this + 0xa)) + i32::from(m.r8(this + 9));
    let q = sum / 50;
    let r = i32::from(m.r8(this + 8)) % 11;
    m.w16(this + 0xc, (q + r + 10) as u16);
}
/// 0x00504900: pone a cero el campo +0x3C de cada barco y el +0x18 de cada registro de 0x3C bytes, de atrás adelante.
pub fn ships_clear_route_cache_flags(m: &mut dyn Mem, this: u32) {
    let n = u32::from(m.r16(this + 0xf4));
    if n != 0 {
        let base = m.r32(this + 4);
        let mut p = base.wrapping_add(n.wrapping_mul(0x180));
        if p > base {
            loop {
                p = p.wrapping_sub(0x180);
                m.w16(p + 0x3c, 0);
                if p <= m.r32(this + 4) {
                    break;
                }
            }
        }
        let n2 = u32::from(m.r16(this + 0xf6));
        if n2 != 0 {
            let base = m.r32(this + 8);
            let mut p = base.wrapping_add(n2.wrapping_mul(0x3c));
            if p > base {
                loop {
                    p = p.wrapping_sub(0x3c);
                    m.w32(p + 0x18, 0);
                    if p <= m.r32(this + 8) {
                        break;
                    }
                }
            }
        }
    }
}
/// 0x00526300: resta de la ranura `0x14+(i&3)` lo pedido (`*ptr`); si no alcanza, pide el existente redondeado hacia cero a decenas.
pub fn town_take_from_slot_clamped(m: &mut dyn Mem, this: u32, a: &[u32]) {
    let slot = ((a[0] & 3) + 0x14).wrapping_mul(4).wrapping_add(this).wrapping_add(4);
    let mut have = m.r32(slot) as i32;
    let mut want = m.r32(a[1]) as i32;
    if have < want {
        want = (have / 10).wrapping_mul(10);
        m.w32(a[1], want as u32);
        have = m.r32(slot) as i32;
    }
    m.w32(slot, have.wrapping_sub(want) as u32);
}
/// 0x005263D0: calcula en `*ptr` la cantidad ofrecida a partir de la reserva de la ranura (`this+0x54+4i`) y las tablas 0x673794/0x67378C/0x67379C.
pub fn town_compute_slot_amount(m: &mut dyn Mem, this: u32, a: &[u32]) {
    let i = a[0] & 3;
    let stock = m.r32(this + 0x54 + i * 4) as i32;
    if stock < 10 {
        m.w32(a[1], 0);
        return;
    }
    let want = m.r32(a[1]) as i32;
    let edx = if want > stock { stock } else { want };
    let eax = stock.wrapping_sub(edx >> 1);
    let prod = eax.wrapping_mul(i32::from(m.r16(0x0067_3794 + i * 2)));
    let div = m.r32(this + 0x2d4) as i32;
    let q = if div < 0x1f4 { prod / 50 } else { prod.wrapping_mul(10) / div };
    let mut r = i32::from(m.r16(0x0067_378c + i * 2)).wrapping_sub(q);
    let floor = i32::from(m.r8(0x0067_379c + i));
    if r < floor {
        r = floor;
    }
    m.w32(a[1], r as u32);
}
/// 0x005285B0: bonificación del ciclo para el tipo `a[0]` (tabla 2,3,5,8,0xB,0xE); fuera de rango devuelve el byte con signo de 0x673A78.
pub fn town_get_cycle_bonus(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    const T: [u32; 6] = [2, 3, 5, 8, 0xb, 0xe];
    if a[0] >= 6 {
        return m.r8(0x0067_3a78) as i8 as i32 as u32;
    }
    let s = u32::from(m.r8(this + 0x2c0)) + u32::from(m.r16(0x006d_e4a4));
    (s % 5) * 3 + T[a[0] as usize] * 163
}
/// 0x0052B0D0: 1 si el tipo `a[0]` (0..3) está permitido: no si la ciudad tiene el bit 1 de +0x2C8 y el tipo el bit 2; si no, byte con signo +0x824+t >= 0 (solo AL).
pub fn town_slot_allowed(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let t = a[0] & 0xff;
    if t > 3 || (m.r8(this + 0x2c8) & 1 != 0 && t & 2 == 2) {
        return 0;
    }
    bool32((m.r8(this + 0x824 + t) as i8) >= 0)
}
/// 0x00526880: 1 si alguna de las dos ranuras de +0x830 es válida y su registro (por 0x4D8DB0) pertenece al jugador actual (byte +0xE, global 0x6DFC14).
pub fn town_has_slot_owned_by_current_player(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    for k in 0..2u32 {
        let slot = m.r16(this + 0x830 + k * 2);
        if slot < m.r16(0x006d_d748) {
            let rec = host.call(m, 0x4d8db0, 0x006d_d73c, &[u32::from(slot)]).wrapping_add(8);
            if u32::from(m.r8(rec + 0xe)) == m.r32(0x006d_fc14) {
                return 1;
            }
        }
    }
    0
}

/// 0x0053E9C0: actualiza las banderas (+8) de la parada de ruta `this` según su propietario y guarda su byte bajo en el registro de 0xDC bytes de la ruta (+3).
pub fn route_op_update_flags(m: &mut dyn Mem, this: u32) {
    let idx = m.r32(this + 4);
    if idx >= u32::from(m.r16(0x006d_d72a)) {
        return;
    }
    let mut flags = m.r32(this + 8);
    if flags & 1 == 1 {
        let tbl = m.r32(0x006d_d72c);
        let owner = u32::from(m.r8(tbl.wrapping_add(idx.wrapping_mul(0xdc)).wrapping_add(2)));
        if m.r32(0x006d_e4b0) > owner {
            let rec = m.r32(0x006d_e508).wrapping_add(owner.wrapping_mul(2552));
            if m.r32(rec + 0x76c) & 0x40 == 0 {
                flags = flags.wrapping_sub(1) | 8;
                m.w32(this + 8, flags);
            }
        } else {
            flags = flags.wrapping_sub(1);
            m.w32(this + 8, flags);
        }
    }
    let low = m.r8(this + 8);
    let tbl = m.r32(0x006d_d72c);
    m.w8(tbl.wrapping_add(idx.wrapping_mul(0xdc)).wrapping_add(3), low);
}

/// 0x0053E480: operación «fijar parada de ruta»: guarda cantidad/precio de la parada `slot` de la ruta y la mueve a la posición `pos` del orden de paradas
/// (+4..+0x1B del registro de 0xDC bytes), reparando después permutaciones inválidas.
pub fn route_op_set_stop(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let route = u32::from(m.r16(this + 4));
    if m.r16(this + 4) > m.r16(0x006d_d72a) {
        return;
    }
    let slot = u32::from(m.r16(this + 8));
    if slot >= 0x18 {
        return;
    }
    let tbl = m.r32(0x006d_d72c);
    let rec = tbl.wrapping_add(route.wrapping_mul(0xdc));
    let v = m.r32(this + 0xc);
    m.w32(rec + slot * 4 + 0x7c, v);
    let v = m.r32(this + 0x10);
    m.w32(rec + slot * 4 + 0x1c, v);
    let owner = u32::from(m.r16(this + 6));
    let merchant = host.call(m, 0x5303c0, 0x006d_e4a0, &[owner]);
    let v = m.r32(this + 0x10) as i32;
    if v != 0 {
        if v < 0 {
            m.w16(merchant.wrapping_add(slot * 4).wrapping_add(0x548), (v as u16).wrapping_neg());
        } else {
            m.w16(merchant.wrapping_add(slot * 4).wrapping_add(0x54a), v as u16);
        }
    }
    let pos = u32::from(m.r16(this + 0xa));
    if pos >= 0x18 {
        return;
    }
    // posición actual de `slot` en el orden
    let mut i: u32 = 0;
    if u32::from(m.r8(rec + 4)) != slot {
        loop {
            if i >= 0x18 {
                for k in 0..0x18 {
                    m.w8(rec + 4 + k, k as u8);
                }
                return;
            }
            let d = u32::from(m.r8(rec + i + 5));
            i += 1;
            if d == slot {
                break;
            }
        }
    }
    if i > pos {
        let mut e = i as i32 - 1;
        while e >= pos as i32 {
            let b = m.r8(rec + e as u32 + 4);
            m.w8(rec + e as u32 + 5, b);
            e -= 1;
        }
    } else if i < pos {
        let mut e = i + 1;
        while e <= pos {
            let b = m.r8(rec + e + 4);
            m.w8(rec + e + 3, b);
            e += 1;
        }
    }
    let b = m.r8(this + 8);
    m.w8(rec + pos + 4, b);
    let mut tmp = [0xffu8; 0x18];
    for k in 0..0x18u32 {
        let a = m.r8(rec + k + 4);
        if a < 0x18 {
            tmp[a as usize] = k as u8;
        }
    }
    for k in 0..0x18u32 {
        let a = m.r8(rec + k + 4);
        if a < 0x18 && u32::from(tmp[a as usize]) == k {
            continue;
        }
        if let Some(j) = tmp.iter().position(|&t| t == 0xff) {
            m.w8(rec + k + 4, j as u8);
            tmp[j] = k as u8;
        }
    }
}

/// 0x0053E7C0: operación sobre el plan de ruta de un barco (o de su convoy): llama a 0x4D4F60 con (puntero a ruta, +8, +0xC, +0x10==1), marca el barco si la ruta
/// queda inválida y termina con la llamada 0x48FD90 sobre el global 0x6CE6D0.
pub fn route_op_assign_plan(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let idx = m.r32(this + 4);
    let n = u32::from(m.r16(0x006d_d894));
    if idx >= n {
        return;
    }
    let ships = m.r32(0x006d_d7a4);
    let mut ship = idx.wrapping_mul(0x180).wrapping_add(ships);
    let convoy = m.r16(ship + 8);
    if convoy < m.r16(0x006d_d896) {
        let c = m.r32(0x006d_d7a8).wrapping_add(u32::from(convoy) * 60);
        let leader = m.r16(c + 0x10);
        if leader > (n as u16) {
            return;
        }
        ship = u32::from(leader).wrapping_mul(0x180).wrapping_add(ships);
    }
    let routes = ship + 0x132;
    if m.r16(routes) >= m.r16(0x006d_d72a) {
        return;
    }
    let a1 = m.r32(this + 8);
    let a2 = m.r32(this + 0xc);
    let a3 = bool32(m.r32(this + 0x10) == 1);
    host.call(m, 0x4d4f60, 0x006d_d728, &[routes, a1, a2, a3]);
    if m.r16(routes) >= m.r16(0x006d_d72a) {
        let b = m.r8(ship + 0x136) & 0xfc;
        let f = m.r8(ship + 0x3d) | 8;
        m.w8(ship + 0x3d, f);
        m.w8(ship + 0x136, b);
    }
    let g = m.r32(0x006c_e6d0);
    host.call(m, 0x48fd90, g, &[]);
}

/// 0x0053E610: operación sobre registros de ruta (0xDC bytes): crear/duplicar (+8 != 0, 0x4D4C90) o cambiar el propietario (+0xC) y borrar la ruta (0x509030,
/// 0x4D4E80). Los 16 bits altos de ciertos argumentos son los del global 0x6DE4B0, como deja el registro EDX en el original.
pub fn route_op_edit(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let route = m.r32(this + 4);
    if route >= u32::from(m.r16(0x006d_d72a)) {
        return;
    }
    let tbl = m.r32(0x006d_d72c);
    let rec = tbl.wrapping_add(route.wrapping_mul(0xdc));
    let limit = m.r32(0x006d_e4b0);
    let target = m.r32(this + 0xc);
    if m.r32(this + 8) != 0 {
        if target >= limit {
            return;
        }
        let new = host.call(m, 0x4d4c90, 0x006d_d728, &[]);
        let tbl = m.r32(0x006d_d72c);
        let me = tbl.wrapping_add(m.r32(this + 4).wrapping_mul(0xdc));
        let nr = tbl.wrapping_add((new & 0xffff).wrapping_mul(0xdc));
        let di = m.r16(me);
        m.w16(nr, di);
        m.w16(me, new as u16);
        let owner = m.r8(this + 0xc);
        m.w8(nr + 2, owner);
        m.w8(nr + 3, 0);
        return;
    }
    if target >= limit {
        let dx = u32::from(m.r16(rec));
        let arg1;
        if dx == route {
            arg1 = 0xffff;
        } else {
            if m.r8(rec + 3) & 4 == 4 {
                let o = tbl.wrapping_add(dx.wrapping_mul(0xdc)) + 3;
                let b = m.r8(o) | 4;
                m.w8(o, b);
            }
            arg1 = (limit & 0xffff_0000) | dx;
        }
        let r = u32::from(m.r16(this + 4));
        host.call(m, 0x509030, 0x006d_d7a0, &[r, arg1]);
        let r2 = m.r32(this + 4);
        host.call(m, 0x4d4e80, 0x006d_d728, &[r2]);
        return;
    }
    if target == u32::from(m.r8(rec + 2)) {
        return;
    }
    let keep = m.r8(rec + 3) & 4;
    let owner = m.r8(this + 0xc);
    m.w8(rec + 2, owner);
    m.w8(rec + 3, keep);
    let ship_idx = m.r32(this + 0x10);
    if ship_idx < u32::from(m.r16(0x006d_d894)) {
        let ship = m.r32(0x006d_d7a4).wrapping_add(ship_idx.wrapping_mul(0x180));
        let r = m.r32(this + 4);
        if u32::from(m.r16(ship + 0x132)) == r {
            let f = m.r8(ship + 0x3c) | 0x20;
            m.w8(ship + 0x3c, f);
            let dl = m.r8(ship + 0x136) & 0xfc;
            m.w8(ship + 0x136, dl);
            if m.r16(ship + 0x134) >= 0xc {
                let arg = (limit & 0xffff_0000) | u32::from(m.r16(this + 0x10));
                host.call(m, 0x5171e0, ship, &[arg]);
            }
        }
        let kind = u32::from(m.r8(ship));
        if kind < u32::from(m.r16(0x006d_e4aa)) {
            let o = u32::from(m.r8(rec + 2));
            if host.call(m, 0x5308a0, 0x006d_e4a0, &[kind, o, 0]) != 0 {
                return;
            }
        }
    }
    for k in 0..0x18u32 {
        if m.r32(rec + 0x1c + k * 4) == 0 {
            m.w32(rec + 0x7c + k * 4, 0);
        }
    }
}

/// 0x005352B0: rellena las 40 filas de 16 dwords con las banderas actuales de cada ciudad (+0x2C8 | 0x7C0000; 0 si la ciudad no existe).
pub fn town_info_history_fill_with_current_flags(m: &mut dyn Mem, this: u32) {
    for k in 0..40u32 {
        let v = if k < m.r32(0x006d_e4b0) {
            let towns = m.r32(0x006d_e508);
            m.r32(towns.wrapping_add(k * 0x9f8).wrapping_add(0x2c8)) | 0x007c_0000
        } else {
            0
        };
        for j in 0..16u32 {
            m.w32(this + k * 0x40 + j * 4, v);
        }
    }
}
/// 0x004D7720: sigue la cadena de mensajes desde `a[0]` y devuelve 1 si alguno es de tipo 0x45 o 0x46 (solo AL).
pub fn has_type_45_or_46_message(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let count = u32::from(m.r16(this + 6));
    let mut i = a[0];
    if i >= count {
        return 0;
    }
    let mut rem = count;
    loop {
        let before = rem;
        rem = rem.wrapping_sub(1);
        if before == 0 {
            return 0;
        }
        let node = m.r32(this).wrapping_add(i << 4);
        let t = m.r8(node + 4);
        if t == 0x45 || t == 0x46 {
            return 1;
        }
        i = u32::from(m.r16(node + 6));
        if i >= count {
            return 0;
        }
    }
}
/// 0x004D6FD0: recorre la cadena de mensajes desde `a[0]` y pone a cero el campo +0xE (tipo 0x21) o el byte +0xF (tipo 0x22).
pub fn clear_type_21_22_message_fields(m: &mut dyn Mem, this: u32, a: &[u32]) {
    let count = u32::from(m.r16(this + 6));
    let mut i = a[0];
    if i >= count {
        return;
    }
    let mut rem = count;
    loop {
        let before = rem;
        rem = rem.wrapping_sub(1);
        if (before as i32) <= 0 {
            return;
        }
        let off = i << 4;
        let node = m.r32(this).wrapping_add(off);
        let t = m.r8(node + 4);
        if t == 0x21 {
            m.w16(node + 0xe, 0);
        } else if t == 0x22 {
            m.w8(node + 0xf, 0);
        }
        let base = m.r32(this);
        i = u32::from(m.r16(base.wrapping_add(off).wrapping_add(6)));
        if i >= u32::from(m.r16(this + 6)) {
            return;
        }
    }
}
/// 0x00511080: libera cada bloque de la tabla +4 (entradas de 8 bytes, contador u16 en +8), la tabla, y deja el contador a cero.
pub fn script_blob_pool_clear(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    if m.r32(this + 4) != 0 {
        let mut i = 0u32;
        if m.r16(this + 8) != 0 {
            loop {
                let table = m.r32(this + 4);
                let p = m.r32(table.wrapping_add(i * 8));
                if p != 0 {
                    host.call(m, FREE, 0, &[p]);
                }
                let table = m.r32(this + 4);
                m.w32(table.wrapping_add(i * 8), 0);
                i += 1;
                if (i as i32) >= i32::from(m.r16(this + 8)) {
                    break;
                }
            }
        }
        let table = m.r32(this + 4);
        host.call(m, FREE, 0, &[table]);
        m.w32(this + 4, 0);
    }
    m.w16(this + 8, 0);
}
/// 0x004D64B0: destruye el vector de mensajes (0x639DA8 + liberar), vacía los contadores y deja 0xFFFF en +0xA de cada comerciante.
pub fn message_pool_clear(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let p = m.r32(this);
    if p != 0 {
        let cnt = m.r32(p.wrapping_sub(4));
        host.call(m, 0x0063_9da8, cnt, &[p, 0x10, cnt, 0x0051_a880]);
        host.call(m, FREE, 0, &[p.wrapping_sub(4)]);
    }
    m.w32(this, 0);
    m.w16(this + 6, 0);
    m.w16(this + 4, 0);
    m.w16(this + 0xa, 0);
    m.w16(this + 8, 0xffff);
    let mut i = 0u32;
    if m.r16(0x006d_e4aa) != 0 {
        loop {
            let merchant = host.call(m, 0x5303c0, 0x006d_e4a0, &[i]);
            m.w16(merchant.wrapping_add(0xa), 0xffff);
            i += 1;
            if (i as i32) >= i32::from(m.r16(0x006d_e4aa)) {
                break;
            }
        }
    }
}
/// 0x0050A2B0: suma `0x518460(ship, a[1])` de los barcos del jugador actual en la cadena del convoy `a[0]`.
pub fn get_convoy_player_ships_cargo_units(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let player = m.r32(0x006d_fc14);
    if a[0] >= u32::from(m.r16(this + 0xf6)) {
        return 0;
    }
    let convoy = m.r32(this + 8).wrapping_add(a[0].wrapping_mul(60));
    let mut i = u32::from(m.r16(convoy + 0xa));
    let mut sum = 0u32;
    if (i as i32) < i32::from(m.r16(this + 0xf4)) {
        loop {
            let ship = m.r32(this + 4).wrapping_add(i.wrapping_mul(0x180));
            if u32::from(m.r8(ship)) == player {
                let r = host.call(m, 0x518460, ship, &[a[1]]);
                sum = sum.wrapping_add(r);
            }
            i = u32::from(m.r16(ship + 6));
            if (i as i32) >= i32::from(m.r16(this + 0xf4)) {
                break;
            }
        }
    }
    sum
}

/// 0x005171E0: detiene el barco: libera su ruta (+0x24, bloque propio si +0x36), avisa a la interfaz si es del jugador actual (0x44F940) y limpia estado.
pub fn ship_stop_and_clear_route(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) {
    let route = m.r32(this + 0x24);
    if route != 0 {
        if m.r8(this + 0x36) != 0 {
            let inner = m.r32(route);
            if inner != 0 {
                host.call(m, FREE, 0, &[inner]);
            }
            let again = m.r32(this + 0x24);
            host.call(m, FREE, 0, &[again]);
            m.w8(this + 0x36, 0);
        }
        m.w32(this + 0x24, 0);
        let owner = u32::from(m.r8(this));
        m.w32(this + 0x48, 0);
        if owner == m.r32(0x006d_fc14) {
            let ui = m.r32(0x006c_bdd8);
            host.call(m, 0x44f940, ui, &[a[0] & 0xffff]);
        }
    }
    let al = m.r8(this + 0x39);
    let f = m.r8(this + 0x3c) | 0xa8;
    m.w8(this + 0x3c, f);
    m.w16(this + 0x1c, 0);
    m.w16(this + 0x20, 0);
    m.w32(this + 0x28, 0);
    m.w32(this + 0x2c, 0);
    m.w32(this + 0x30, 0);
    m.w16(this + 0x34, 0);
    m.w16(this + 2, 0);
    m.w8(this + 0x38, al);
}
/// 0x0051A3C0: para cada barco de la cadena del convoy de `this`, pone a 1 las existencias (+0x54, 24 dwords) de `this` que estén a 0 donde el otro barco tiene carga.
pub fn ship_mark_convoy_wares_as_placeholders(m: &mut dyn Mem, this: u32) {
    let convoy = m.r16(this + 8);
    if convoy >= m.r16(0x006d_d896) {
        return;
    }
    let c = m.r32(0x006d_d7a8).wrapping_add(u32::from(convoy) * 60);
    let mut i = u32::from(m.r16(c + 0xa));
    if i >= u32::from(m.r16(0x006d_d894)) {
        return;
    }
    loop {
        let ship = m.r32(0x006d_d7a4).wrapping_add(i.wrapping_mul(0x180));
        i = u32::from(m.r16(ship + 6));
        for k in 0..24u32 {
            let mine = this + 0x54 + k * 4;
            let other = ship.wrapping_add(0x54 + k * 4);
            if (m.r32(other) as i32) > 0 && m.r32(mine) == 0 {
                m.w32(mine, 1);
            }
        }
        if i >= u32::from(m.r16(0x006d_d894)) {
            return;
        }
    }
}
/// 0x00518460: unidades netas de carga: (capacidad +0x10 − Σ peso de los 24 huecos − 10·(+0x154) + penalización de tripulación) / a[0] (división sin signo).
pub fn ship_get_net_cargo_capacity_in_units(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let mut s = m.r32(this + 0x10);
    for k in 0..24u32 {
        let b = u32::from(m.r8(this + 0x13c + k));
        if b < 6 {
            s = s.wrapping_sub(u32::from(m.r16(0x0067_2cb4 + b * 2)));
        }
    }
    let w = u32::from(m.r16(this + 0x154));
    let mut v = s.wrapping_sub(w.wrapping_mul(10));
    let sel = u32::from(m.r8(this + 0xe) & 3);
    let dl = u32::from(m.r8(0x0067_3664 + sel));
    let cx = u32::from(m.r16(this + 0x40));
    if cx > dl {
        v = v.wrapping_add(dl.wrapping_sub(cx).wrapping_mul(400));
    }
    v / a[0]
}
/// 0x00503DB0: reinicia el vector (+0,+4), libera el bloque anterior (+8) y reserva `a[0]*4` bytes; AL = reserva conseguida.
pub fn dword_array_allocate(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let old = m.r32(this + 8);
    m.w32(this, 0);
    m.w32(this + 4, 0);
    if old != 0 {
        host.call(m, FREE, 0, &[old]);
        m.w32(this + 8, 0);
    }
    let p = host.call(m, ALLOC, 0, &[a[0].wrapping_mul(4)]);
    m.w32(this + 8, p);
    bool32(p != 0)
}
/// 0x00544000: libera el paquete `this` y devuelve su enlace al siguiente (+0x420).
pub fn operation_packet_free_and_return_next(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let next = m.r32(this + 0x420);
    m.w32(this + 0x420, 0);
    host.call(m, FREE, 0, &[this]);
    next
}
/// 0x00543A70: libera la tabla de bloques (+0x14, contador en +0) y el bloque +0x18, y pone a cero la cola.
pub fn operation_queue_free(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let table = m.r32(this + 0x14);
    if table != 0 {
        let n = m.r32(this);
        m.w32(this + 0x14, 0);
        for i in 0..n {
            let p = m.r32(table.wrapping_add(i.wrapping_mul(4)));
            if p != 0 {
                host.call(m, FREE, 0, &[p]);
            }
        }
        host.call(m, FREE, 0, &[table]);
    }
    let blk = m.r32(this + 0x18);
    m.w32(this, 0);
    if blk != 0 {
        m.w32(this + 0x18, 0);
        host.call(m, FREE, 0, &[blk]);
    }
    for off in [0, 4, 8, 0xc, 0x10] {
        m.w32(this + off, 0);
    }
}
/// 0x0053EA50: añade un trozo de nombre del comerciante `+4` a la ventana de nombres (0x512C50): tipo 1/0 según +0x1A y byte +0x1C si `this[0] == 0xAA`; si no, tipo 2 y byte +0x1D.
pub fn op_append_merchant_name_chunk(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let idx = u32::from(m.r16(this + 4));
    if m.r16(this + 4) >= m.r16(0x006d_e4aa) {
        return;
    }
    let merchant = host.call(m, 0x5303c0, 0x006d_e4a0, &[idx]);
    if m.r32(this) == 0xaa {
        let kind = bool32(m.r8(merchant + 0x1a) & 1 == 0);
        let b = u32::from(m.r8(merchant + 0x1c));
        host.call(m, 0x512c50, 0x006d_daa0, &[kind, b, this + 8]);
    } else {
        let b = u32::from(m.r8(merchant + 0x1d));
        host.call(m, 0x512c50, 0x006d_daa0, &[2, b, this + 8]);
    }
}

/// 0x00533840: vacía la lista circular del centro comercial (centinela en +0x24, contador en +0x28) liberando cada nodo y el centinela.
pub fn trade_center_destruct(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let sentinel = m.r32(this + 0x24);
    let mut node = m.r32(sentinel);
    if node != sentinel {
        loop {
            let cur = node;
            node = m.r32(node);
            let prev = m.r32(cur + 4);
            let next = m.r32(cur);
            m.w32(prev, next);
            let next = m.r32(cur);
            let prev = m.r32(cur + 4);
            m.w32(next + 4, prev);
            host.call(m, FREE, 0, &[cur]);
            let c = m.r32(this + 0x28).wrapping_sub(1);
            m.w32(this + 0x28, c);
            if node == sentinel {
                break;
            }
        }
    }
    let s = m.r32(this + 0x24);
    host.call(m, FREE, 0, &[s]);
    m.w32(this + 0x24, 0);
    m.w32(this + 0x28, 0);
}
/// 0x00511020: destructor del pool de scripts: pone la tabla de métodos (0x673608) y vacía igual que `script_blob_pool_clear`.
pub fn script_blob_pool_destroy(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    m.w32(this, 0x0067_3608);
    script_blob_pool_clear(m, host, this);
}
/// 0x004F37E0: resta una referencia al bloque `a[0]` del pool de scripts; al llegar a cero lo libera y anula la entrada.
pub fn script_blob_pool_release(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) {
    let p = a[0];
    if p == 0 {
        return;
    }
    let count = u32::from(m.r16(this + 8));
    if count == 0 {
        return;
    }
    let table = m.r32(this + 4);
    for i in 0..count {
        if m.r32(table + i * 8) == p {
            let refs = m.r16(table + i * 8 + 6).wrapping_sub(1);
            m.w16(table + i * 8 + 6, refs);
            let table = m.r32(this + 4);
            if m.r16(table + i * 8 + 6) == 0 {
                let q = m.r32(table + i * 8);
                if q != 0 {
                    host.call(m, FREE, 0, &[q]);
                    let t = m.r32(this + 4);
                    m.w32(t + i * 8, 0);
                }
            }
            return;
        }
    }
}
/// 0x005350F0: valida las 8 entradas (byte +0xA00+k, bloque +0xA08+4k) del historial de ciudad y devuelve el tamaño de guardado (0xA08 + 0x28 por entrada viva).
pub fn town_info_history_validate_and_get_save_size(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let mut size = 0xa08u32;
    for k in 0..8u32 {
        let b = u16::from(m.r8(this + 0xa00 + k));
        let slot = this + 0xa08 + k * 4;
        if b < m.r16(0x006d_e4aa) {
            if m.r32(slot) == 0 {
                m.w8(this + 0xa00 + k, 0xff);
            } else {
                size += 0x28;
            }
        } else {
            m.w8(this + 0xa00 + k, 0xff);
            let p = m.r32(slot);
            if p != 0 {
                host.call(m, FREE, 0, &[p]);
                m.w32(slot, 0);
            }
        }
    }
    size
}
/// 0x00509930: cuenta barcos por comerciante (índices de lista en +0xE): `a[1][t]` los que no son del tipo 0x11, `a[0][t]` además con +0x42 < +0xF2, con t = (+8 != 0).
pub fn count_ships_and_captains(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) {
    let (first, second) = (a[0], a[1]);
    m.w32(first, 0);
    m.w32(first + 4, 0);
    m.w32(second, 0);
    m.w32(second + 4, 0);
    let mut i = i32::from(m.r16(0x006d_e4aa));
    while i > 0 {
        i -= 1;
        let merchant = host.call(m, 0x5303c0, 0x006d_e4a0, &[i as u32]);
        let t = u32::from(m.r16(merchant + 8) != 0);
        let mut cx = u32::from(m.r16(merchant + 0xe));
        while (cx as i32) < i32::from(m.r16(this + 0xf4)) {
            let ship = m.r32(this + 4).wrapping_add(cx * 0x180);
            if m.r16(ship + 0x134) != 0x11 {
                let v = m.r32(second + t * 4).wrapping_add(1);
                m.w32(second + t * 4, v);
                if m.r16(ship + 0x42) < m.r16(this + 0xf2) {
                    let v = m.r32(first + t * 4).wrapping_add(1);
                    m.w32(first + t * 4, v);
                }
            }
            cx = u32::from(m.r16(ship + 4));
        }
    }
}
/// 0x00533490: días de viaje por tierra entre dos ciudades (bytes +0x18 de `this`) según la tabla 0x672BF8 de pares: (16·coste + 0x287) >> 8; 0x287>>8 = 2 si no hay par.
pub fn get_overland_route_travel_days(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let s = u32::from(m.r8(this + 0x18 + (a[0] & 0xff)));
    let d = u32::from(m.r8(this + 0x18 + (a[1] & 0xff)));
    let mut edx = 0x10u32;
    for k in 0..7u32 {
        let p = 0x0067_2bf8 + k * 4;
        let x = u32::from(m.r8(p));
        let y = u32::from(m.r8(p + 1));
        let hit = if x == s { y == d } else if x == d { y == s } else { false };
        if hit {
            edx = 0x10 * u32::from(m.r8(p + 3));
            break;
        }
    }
    (edx.wrapping_add(0x287)) >> 8
}

/// 0x0050AB50: reasigna el convoy `a[0]` como cabeza de la lista del grupo: guarda el anterior (+0xEC) en su +8, fija estado `a[1]` (+0x12) y lo propaga a sus barcos (+0x134, +0x138=0, bandera 0x20 en +0x3C).
pub fn relink_convoy_with_state(m: &mut dyn Mem, this: u32, a: &[u32]) {
    let c = m.r32(0x006d_d7a8).wrapping_add(a[0].wrapping_mul(60));
    let state = a[1] as u16;
    let old = m.r16(this + 0xec);
    m.w16(c + 8, old);
    m.w16(this + 0xec, a[0] as u16);
    let mut i = u32::from(m.r16(c + 0xa));
    m.w16(c + 0x12, state);
    m.w16(c + 0x16, 0);
    if i >= u32::from(m.r16(this + 0xf4)) {
        return;
    }
    loop {
        let ship = m.r32(0x006d_d7a4).wrapping_add(i.wrapping_mul(0x180));
        let next = u32::from(m.r16(ship + 6));
        let f = m.r16(ship + 0x3c) | 0x20;
        m.w16(ship + 0x3c, f);
        m.w16(ship + 0x134, state);
        m.w16(ship + 0x138, 0);
        i = next;
        if i >= u32::from(m.r16(this + 0xf4)) {
            return;
        }
    }
}
/// 0x0050A340: reserva una lista (contador u16 + índices) con los convoyes del jugador en estado <4 o 0xF y sin banderas 0xA; devuelve el bloque (0 si falla la reserva).
pub fn build_public_convoy_index_list(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let n = u32::from(m.r16(this + 0xf6));
    let p = host.call(m, ALLOC, 0, &[n * 2 + 2]);
    if p == 0 {
        return 0;
    }
    m.w16(p, 0);
    let mut cx = u32::from(m.r16(this + 0xec));
    if cx >= u32::from(m.r16(this + 0xf6)) {
        return p;
    }
    loop {
        let c = m.r32(this + 8).wrapping_add(cx * 60);
        if u16::from(m.r8(c)) < m.r16(0x006d_e4aa) && m.r8(c + 0x14) & 0xa == 0 {
            let st = m.r16(c + 0x12);
            if st < 4 || st == 0xf {
                let k = m.r16(p).wrapping_add(1);
                m.w16(p, k);
                m.w16(p + u32::from(k) * 2, cx as u16);
            }
        }
        cx = u32::from(m.r16(c + 8));
        if cx >= u32::from(m.r16(this + 0xf6)) {
            return p;
        }
    }
}
/// 0x0052A950: suma el coste de reparación (0x52A9E0, `this` = ciudad) de todos los barcos del convoy del barco `a[0]`; si no tiene convoy, solo de ese barco.
pub fn town_calc_ship_repair_cost_for_convoy(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let ship = a[0];
    let convoy = m.r16(ship + 8);
    if convoy >= m.r16(0x006d_d896) {
        return host.call(m, 0x52a9e0, this, &[ship]);
    }
    let c = m.r32(0x006d_d7a8).wrapping_add(u32::from(convoy) * 60);
    let mut i = u32::from(m.r16(c + 0xa));
    let mut total = 0u32;
    if (i as i32) >= i32::from(m.r16(0x006d_d894)) {
        return 0;
    }
    loop {
        let s = m.r32(0x006d_d7a4).wrapping_add(i.wrapping_mul(0x180));
        i = u32::from(m.r16(s + 6));
        let r = host.call(m, 0x52a9e0, this, &[s]);
        total = total.wrapping_add(r);
        if (i as i32) >= i32::from(m.r16(0x006d_d894)) {
            return total;
        }
    }
}
/// 0x00518750: 1 si el barco navega libre en mar abierto (él o el líder de su convoy con estado 0xF y bandera 2) y la casilla (+0x1E,+0x22) no es de ciudad (0x517260 < 0); solo AL.
pub fn ship_is_sailing_free_at_open_sea(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let dx = m.r16(0x006d_d894);
    let convoy = m.r16(this + 8);
    if convoy >= dx {
        if m.r16(this + 0x50) < dx || m.r16(this + 0x52) < dx || m.r16(this + 0x134) != 0xf {
            return 0;
        }
    } else {
        let c = m.r32(0x006d_d7a8).wrapping_add(u32::from(convoy) * 60);
        let leader = m.r16(c + 0x10);
        if leader >= dx {
            return 0;
        }
        let s = m.r32(0x006d_d7a4).wrapping_add(u32::from(leader) * 0x180);
        if m.r16(s + 0x50) < dx || m.r16(s + 0x52) < dx {
            return 0;
        }
        if m.r8(c) != m.r8(this) || m.r16(c + 0x12) != 0xf || m.r8(c + 0x14) & 2 == 0 {
            return 0;
        }
    }
    let y = u32::from(m.r16(this + 0x22));
    let x = u32::from(m.r16(this + 0x1e));
    let r = host.call(m, 0x517260, 0, &[x, y]);
    bool32((r as i32) < 0)
}
/// 0x00549C60: tamaño total del guardado: suma de los tamaños de comerciantes (+0x7C), barcos, cartas, mensajes, etc. (siete llamadas de tamaño).
pub fn get_savegame_total_size(m: &mut dyn Mem, host: &mut dyn OpsHost) -> u32 {
    let mut s = host.call(m, 0x52f760, 0x006d_e4a0, &[]).wrapping_add(0x7c);
    for (f, ecx) in [(0x503f70u32, 0x006d_d7a0u32), (0x4d6270, 0x006d_d730), (0x4d4b40, 0x006d_d728), (0x4d8080, 0x006d_d73c), (0x6118f0, 0x006e_55d0)] {
        s = s.wrapping_add(host.call(m, f, ecx, &[]));
    }
    host.call(m, 0x511b90, 0x006d_daa0, &[]).wrapping_add(s)
}

/// 0x004D7140: añade un mensaje de tipo 0x44 al pool (crece por 0x4D6B40 si está lleno), lo engancha a la lista del comerciante `a[0]` (+0xA) y rellena sus campos.
pub fn add_message_type_44(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) {
    if m.r16(this + 6) <= m.r16(this + 4) {
        host.call(m, 0x4d6b40, this, &[]);
    }
    let who = a[0] & 0xff;
    if who >= u32::from(m.r16(0x006d_e4aa)) {
        return;
    }
    let merchant = host.call(m, 0x5303c0, 0x006d_e4a0, &[who]);
    let head = m.r16(this + 4);
    let base = m.r32(this);
    let rec = base.wrapping_add(u32::from(head) << 4);
    let next = m.r16(rec + 6);
    m.w16(this + 4, next);
    let old = m.r16(merchant + 0xa);
    m.w16(rec + 6, old);
    m.w16(merchant + 0xa, head);
    let b0 = m.r8(0x006d_e4a0);
    m.w8(rec, b0);
    let b1 = m.r8(0x006d_e4a1) | 0x80;
    m.w8(rec + 1, b1);
    let w = m.r16(0x006d_e4a2);
    m.w16(rec + 2, w);
    m.w8(rec + 5, a[1] as u8);
    m.w16(rec + 8, a[2] as u16);
    m.w16(rec + 0xa, a[3] as u16);
    m.w8(rec + 4, 0x44);
    m.w16(rec + 0xc, a[4] as u16);
    m.w16(rec + 0xe, a[5] as u16);
}

/// 0x0053E890: operación «crear ruta con paradas»: libera la ruta actual del barco (o del líder de su convoy, mismo propietario) por 0x4D4DD0 y crea `+0xC` registros de
/// ruta nuevos (0x4D4C90) encadenados en círculo, con propietario `+8` y la bandera 4 en el primero.
pub fn op_create_route_with_stops(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let nships = u32::from(m.r16(0x006d_d894));
    let idx = m.r32(this + 4);
    if idx >= nships {
        return;
    }
    if m.r32(this + 8) >= m.r32(0x006d_e4b0) || m.r32(this + 0xc) < 1 {
        return;
    }
    let ships = m.r32(0x006d_d7a4);
    let mut ship = idx.wrapping_mul(0x180).wrapping_add(ships);
    let convoy = m.r16(ship + 8);
    if convoy < m.r16(0x006d_d894) {
        let owner = u32::from(m.r8(ship));
        let c = m.r32(0x006d_d7a8).wrapping_add(u32::from(convoy) * 60);
        let leader = u32::from(m.r16(c + 0x10));
        m.w32(this + 4, leader);
        if leader >= u32::from(m.r16(0x006d_d894)) {
            return;
        }
        ship = m.r32(0x006d_d7a4).wrapping_add(leader * 0x180);
        if owner != u32::from(m.r8(ship)) {
            return;
        }
    }
    let first = ship + 0x132;
    let cur = m.r16(first);
    if cur < m.r16(0x006d_d72a) {
        host.call(m, 0x4d4dd0, 0x006d_d728, &[u32::from(cur), 1]);
        m.w16(first, 0xffff);
    }
    let mut prev = first;
    loop {
        let cnt = m.r32(this + 0xc);
        if cnt == 0 {
            break;
        }
        m.w32(this + 0xc, cnt - 1);
        let r = host.call(m, 0x4d4c90, 0x006d_d728, &[]);
        m.w16(prev, r as u16);
        let tbl = m.r32(0x006d_d72c);
        let rec = tbl.wrapping_add((r & 0xffff).wrapping_mul(0xdc));
        if r as u16 == m.r16(first) {
            m.w8(rec + 3, 4);
        }
        let owner = m.r8(this + 8);
        m.w8(rec + 2, owner);
        let id = m.r16(first);
        m.w16(rec, id);
        prev = rec;
    }
    let c = m.r32(this + 0xc).wrapping_sub(1);
    m.w32(this + 0xc, c);
}

/// 0x00538100: operación «mover mercancía entre barco y oficina»: con +0xE != 0 pasa de la oficina al barco (limitada por el hueco 0x5182B0 y la existencia, vía 0x5180C0),
/// si no del barco a la oficina (vía 0x4FF6F0); después recalcula el barco (0x5182B0, 0x517770 con 4).
pub fn op_transfer_ware_ship_office(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let sidx = m.r16(this + 8);
    if sidx >= m.r16(0x006d_d894) || m.r16(this + 0xa) >= 0x18 {
        return;
    }
    let ship = m.r32(0x006d_d7a4).wrapping_add(u32::from(sidx) * 0x180);
    if m.r16(ship + 0x134) >= 4 {
        return;
    }
    let owner = u32::from(m.r8(ship));
    let town = u32::from(m.r8(ship + 0x39));
    let office = host.call(m, 0x5308a0, 0x006d_e4a0, &[owner, town, 0]);
    if office == 0 {
        return;
    }
    let mut amount = m.r32(this + 4) as i32;
    if amount < 0 {
        amount = amount.wrapping_neg();
    }
    let idx = u32::from(m.r16(this + 0xa));
    if m.r8(this + 0xe) != 0 {
        let room = host.call(m, 0x5182b0, ship, &[]) as i32;
        if amount > room {
            amount = if room <= 0 { 0 } else { room };
        }
        let have = m.r32(office + idx * 4 + 4) as i32;
        if amount >= have {
            amount = have;
        }
        let price = m.r32(office + idx * 4 + 0x3b8);
        host.call(m, 0x5180c0, ship, &[idx, amount as u32, price]);
        let left = m.r32(office + idx * 4 + 4).wrapping_sub(amount as u32);
        m.w32(office + idx * 4 + 4, left);
    } else {
        let have = m.r32(ship + idx * 4 + 0x54) as i32;
        if amount > have {
            amount = have;
        }
        m.w32(ship + idx * 4 + 0x54, have.wrapping_sub(amount) as u32);
        let price = m.r32(ship + idx * 4 + 0xb4);
        host.call(m, 0x4ff6f0, office, &[idx, amount as u32, price]);
    }
    host.call(m, 0x5182b0, ship, &[]);
    host.call(m, 0x517770, ship, &[4]);
}

/// 0x00542420: fija el rango de visibilidad del mar (+8: 1 corto, 3 largo, otro medio) en 0x6DD89A..0x6DD89E, reinicia la matriz de distancias (0x531BA0),
/// vacía los mensajes (0x4D64B0) y crea el mensaje de tipo 0xE con (+4, +6) en 0x4D8CF0.
pub fn op_set_sea_visibility_range(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let (a, b, c) = match m.r8(this + 8) {
        1 => (0x0e10, 0x1324, 0x3c),
        3 => (0x4c90, 0x57e4, 0x8c),
        _ => (0x2710, 0x2f44, 0x64),
    };
    m.w16(0x006d_d89a, a);
    m.w16(0x006d_d89c, b);
    m.w16(0x006d_d89e, c);
    host.call(m, 0x531ba0, 0x006d_e4a0, &[]);
    host.call(m, 0x4d64b0, 0x006d_d730, &[]);
    let arg = m.r32(0x006d_e4b4).wrapping_add(0x100);
    let p = host.call(m, 0x4d8cf0, 0x006d_d73c, &[arg]);
    if p != 0 {
        m.w16(p + 6, 0xe);
        let x = m.r16(this + 4);
        m.w16(p + 8, x);
        let y = m.r16(this + 6);
        m.w16(p + 0xa, y);
    }
}
/// 0x0051FEC0: recorre la cola de defensa (+0x75C, 8 bytes por entrada, enlace u16 en +4 desde +0x768, tope +0x776) y convierte las piezas (a[0], a[1]) de tipo
/// 0x37/0x38 en 0x34/0x35, a cero el byte +6.
pub fn town_finish_defense_piece_at(m: &mut dyn Mem, this: u32, a: &[u32]) {
    let mut i = u32::from(m.r16(this + 0x768));
    if i >= u32::from(m.r16(this + 0x776)) {
        return;
    }
    loop {
        let e = m.r32(this + 0x75c).wrapping_add(i * 8);
        i = u32::from(m.r16(e + 4));
        if m.r8(e) == a[0] as u8 && m.r8(e + 1) == a[1] as u8 {
            let t = m.r8(e + 3);
            if t == 0x37 || t == 0x38 {
                m.w8(e + 6, 0);
                m.w8(e + 3, if t == 0x37 { 0x34 } else { 0x35 });
                i = 0xffff;
            }
        }
        if i >= u32::from(m.r16(this + 0x776)) {
            return;
        }
    }
}
/// 0x0051D680: suma una unidad de milicia: +0x2E0 += 0x14, +0x2E4 baja 15 (mínimo 0), recalcula +0x2D4, cuenta `a[0]` en +0x998 y +0x844 += 5.
pub fn militia_add_unit_apply_population(m: &mut dyn Mem, this: u32, a: &[u32]) {
    let e0 = m.r32(this + 0x2e0).wrapping_add(0x14);
    m.w32(this + 0x2e0, e0);
    let e4 = m.r32(this + 0x2e4) as i32;
    let e4 = if e4 < 0xf { 0 } else { e4 - 0xf };
    m.w32(this + 0x2e4, e4 as u32);
    let total = m.r32(this + 0x2dc).wrapping_add(m.r32(this + 0x2d8)).wrapping_add(e0).wrapping_add(e4 as u32);
    m.w32(this + 0x2d4, total);
    let slot = this.wrapping_add(a[0]).wrapping_add(0x998);
    let v = m.r8(slot).wrapping_add(1);
    m.w8(slot, v);
    let w = m.r16(this + 0x844).wrapping_add(5);
    m.w16(this + 0x844, w);
}
/// 0x00541CC0: libera la ranura de jugador de la sala (+4) en el objeto 0x6CBB94 y suelta el comerciante que la ocupaba.
pub fn op_reset_lobby_player_slot(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let slot = m.r32(this + 4);
    let base = m.r32(0x006c_bb94);
    if slot == m.r32(0x006d_f31c) {
        host.call(m, 0x43e4f0, base, &[]);
        return;
    }
    m.w8(base.wrapping_add(slot).wrapping_add(0x204), 1);
    host.call(m, 0x43e1f0, base, &[slot]);
    let base = m.r32(0x006c_bb94);
    m.w8(base.wrapping_add(slot).wrapping_add(0x1fc), 0);
    let base = m.r32(0x006c_bb94);
    m.w8(base.wrapping_add(slot).wrapping_add(0x1f4), 0);
    let mut base = m.r32(0x006c_bb94);
    if m.r8(base + 0x20c) != 0 {
        let who = m.r32(base.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1d4));
        if who != 0xffff_ffff {
            let merchant = host.call(m, 0x5303c0, 0x006d_e4a0, &[who]);
            let b = m.r8(merchant + 9) & 0x7f;
            m.w8(merchant + 9, b);
            base = m.r32(0x006c_bb94);
        }
        m.w32(base.wrapping_add(slot.wrapping_mul(4)).wrapping_add(0x1d4), 0xffff_ffff);
        base = m.r32(0x006c_bb94);
    }
    let b = m.r8(0x006d_e4b0);
    m.w8(base.wrapping_add(slot).wrapping_add(0x1c4), b);
    let base = m.r32(0x006c_bb94);
    host.call(m, 0x43e3c0, base, &[]);
}

fn copy_bytes(m: &mut dyn Mem, dst: u32, src: u32, n: u32) {
    for k in 0..n {
        let b = m.r8(src.wrapping_add(k));
        m.w8(dst.wrapping_add(k), b);
    }
}
/// 0x0050FCE0: serializa la entrada de ticks de ciudad en `a[0]` (16 bytes: +4, +8 y los bytes +0xD,+0xC,+0xF,+0xE,+0x10,+0x11,+0x13,+0x12); devuelve el final.
pub fn town_tick_entry_serialize(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let buf = a[0];
    copy_bytes(m, buf, this + 4, 4);
    copy_bytes(m, buf + 4, this + 8, 4);
    for (k, off) in [0xdu32, 0xc, 0xf, 0xe, 0x10, 0x11, 0x13, 0x12].into_iter().enumerate() {
        let b = m.r8(this + off);
        m.w8(buf + 8 + k as u32, b);
    }
    buf + 16
}
/// 0x0050FFF0: serializa la instalación (16 bytes seguidos desde `this`) en `a[0]`; devuelve el final.
pub fn facility_serialize(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    copy_bytes(m, a[0], this, 16);
    a[0] + 16
}
/// 0x00510070: lee la instalación (16 bytes) desde `a[0]` hacia `this`; devuelve el final.
pub fn facility_deserialize(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    copy_bytes(m, this, a[0], 16);
    a[0] + 16
}
/// 0x0052DE90: serializa el almacén en `a[0]` (0x2C0 bytes: +0, +4, +0x64, +0xC4, +0x124, +0x13C, +0x2BC); devuelve el final.
pub fn storage_serialize(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let buf = a[0];
    for (off, n) in [(0u32, 4u32), (4, 0x60), (0x64, 0x60), (0xc4, 0x60), (0x124, 0x18), (0x13c, 0x180), (0x2bc, 4)] {
        copy_bytes(m, buf + off, this + off, n);
    }
    buf + 0x2c0
}
/// 0x00549BA0: fija el intervalo de autoguardado (+0x944) según el índice 0..4; otro valor usa el último.
pub fn set_autosave_interval_by_index(m: &mut dyn Mem, this: u32, a: &[u32]) {
    let v = match a[0] {
        0 => 0x240c_8400,
        1 => 0x0002_bf20,
        2 => 0x0004_93e0,
        3 => 0x0009_27c0,
        4 => 0x000d_bba0,
        _ => 0x001b_7740,
    };
    m.w32(this + 0x944, v);
}
/// 0x0051B2D0: tamaño serializado de la ciudad: bloques económico, 0x51E9F0, astillero, 0x528790, instalaciones (21 bytes cada una + 0x5B) y la lista de entradas de ticks (+0x9B4).
pub fn town_get_serialized_size(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let mut s = host.call(m, 0x527900, this, &[]).wrapping_add(0x14);
    s = s.wrapping_add(host.call(m, 0x51e9f0, this, &[]));
    s = s.wrapping_add(host.call(m, 0x52a1f0, this, &[]));
    s = s.wrapping_add(host.call(m, 0x528790, this, &[]));
    let n = host.call(m, 0x50ffe0, this + 0x840, &[]);
    s = s.wrapping_add(n.wrapping_mul(21)).wrapping_add(0x5b);
    let mut node = m.r32(this + 0x9b4);
    while node != 0 {
        s = s.wrapping_add(host.call(m, 0x50fcd0, 0, &[]));
        node = m.r32(node);
    }
    s
}

/// 0x00525D80: serializa el bloque de taberna (20 bytes desde +0x82C) en `a[0]`; devuelve el final.
pub fn town_serialize_tavern_block(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    copy_bytes(m, a[0], this + 0x82c, 20);
    a[0] + 20
}
/// 0x0052A200: serializa el bloque de astillero (+0x810..0x822 y +0x824..0x82C, 26 bytes) en `a[0]`; devuelve el final.
pub fn town_serialize_shipyard_block(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    copy_bytes(m, a[0], this + 0x810, 18);
    copy_bytes(m, a[0] + 18, this + 0x824, 8);
    a[0] + 26
}
/// 0x0052A2B0: lee el bloque de astillero desde `a[0]` hacia la ciudad; devuelve el final.
pub fn town_deserialize_shipyard_block(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    copy_bytes(m, this + 0x810, a[0], 18);
    copy_bytes(m, this + 0x824, a[0] + 18, 8);
    a[0] + 26
}

/// 0x0050FD40: carga una entrada de ticks desde `a[0]` (versión `a[1]`; < 0x80 no trae +8): +4, +8, dos u16 en big-endian (+0xC, +0xE), bytes +0x10/+0x11 y un i16 big-endian (+0x12); devuelve el final.
pub fn town_tick_entry_load_from_buffer(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let mut s = a[0];
    copy_bytes(m, this + 4, s, 4);
    s += 4;
    if a[1] < 0x80 {
        m.w32(this + 8, 0);
    } else {
        copy_bytes(m, this + 8, s, 4);
        s += 4;
    }
    let be = |m: &mut dyn Mem, p: u32| (u16::from(m.r8(p)) << 8).wrapping_add(u16::from(m.r8(p + 1)));
    let v = be(m, s);
    m.w16(this + 0xc, v);
    let v = be(m, s + 2);
    m.w16(this + 0xe, v);
    let b = m.r8(s + 4);
    m.w8(this + 0x10, b);
    let b = m.r8(s + 5);
    m.w8(this + 0x11, b);
    let hi = i16::from(m.r8(s + 6) as i8) << 8;
    let v = hi.wrapping_add(i16::from(m.r8(s + 7) as i8)) as u16;
    m.w32(this, 0);
    m.w16(this + 0x12, v);
    s + 8
}

/// 0x0051AC00: mueve la carta `a[0]` a `this` (16 bytes) y deja la origen marcada: +4, +5 y +6 a 0xFF/0xFFFF, cuerpo +8..+0x10 a cero.
pub fn letter_move_from(m: &mut dyn Mem, this: u32, a: &[u32]) {
    let src = a[0];
    let b = m.r8(src);
    m.w8(this, b);
    let b = m.r8(src + 1);
    m.w8(this + 1, b);
    let w = m.r16(src + 2);
    m.w16(this + 2, w);
    let b = m.r8(src + 4);
    m.w8(this + 4, b);
    let b5 = m.r8(src + 5);
    m.w8(src + 4, 0xff);
    m.w8(this + 5, b5);
    m.w8(src + 5, 0xff);
    let w = m.r16(src + 6);
    m.w16(this + 6, w);
    m.w16(src + 6, 0xffff);
    copy_bytes(m, this + 8, src + 8, 8);
    for k in 0..8 {
        m.w8(src + 8 + k, 0);
    }
}

/// 0x00527910: serializa el bloque económico de la ciudad: primero el almacén (0x52DE90 devuelve el cursor) y después 0x414 bytes seguidos desde +0x2C0; devuelve el final.
pub fn town_serialize_economy_block(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let p = host.call(m, 0x52de90, this, &[a[0]]);
    copy_bytes(m, p, this + 0x2c0, 0x414);
    p.wrapping_add(0x414)
}

/// 0x0051B330: serializa la ciudad en `a[0]`: bloques (economía 0x527910, taberna 0x525D80, 0x51EA30, astillero 0x52A200, 0x5287A0) separados por 0x7F, las 21 instalaciones
/// (0x50FFF0), campos sueltos (+0x9A4…+0x9D8), el contador de la lista de entradas de ticks (+0x9B4) y cada entrada (0x50FCE0); devuelve el final.
pub fn town_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut p = a[0];
    m.w8(p, 0x7f);
    p = host.call(m, 0x527910, this, &[p + 1]);
    for f in [0x525d80u32, 0x51ea30, 0x52a200, 0x5287a0] {
        m.w8(p, 0x7f);
        p = host.call(m, f, this, &[p + 1]);
    }
    m.w8(p, 0x7f);
    p += 1;
    for k in 0..0x15u32 {
        p = host.call(m, 0x50fff0, this + ((0x84 + k) << 4), &[p]);
    }
    for (off, n) in [(0x9a4u32, 0x10u32), (0x990, 4), (0x994, 2), (0x996, 2)] {
        copy_bytes(m, p, this + off, n);
        p += n;
    }
    for k in 0..4u32 {
        let b = m.r8(this + 0x998 + k);
        m.w8(p, b);
        let b = m.r8(this + 0x99c + k);
        m.w8(p + 1, b);
        p += 2;
    }
    for (off, n) in [(0x9a0u32, 2u32), (0x9a2, 2), (0x9b8, 4), (0x9bc, 1), (0x9bd, 1), (0x9c0, 4), (0x9c4, 1), (0x9c6, 0x12), (0x9d8, 0x12)] {
        copy_bytes(m, p, this + off, n);
        p += n;
    }
    let mut count = 0u32;
    let mut node = m.r32(this + 0x9b4);
    if node != 0 {
        loop {
            count += 1;
            node = m.r32(node);
            if node == 0 {
                break;
            }
        }
    }
    m.w8(p, count as u8);
    m.w8(p + 1, (count >> 8) as u8);
    p += 2;
    let mut node = m.r32(this + 0x9b4);
    while node != 0 {
        p = host.call(m, 0x50fce0, node, &[p]);
        node = m.r32(node);
    }
    p
}

/// 0x005014E0: lee un convoy desde `a[0]` (versión `a[1]`): campos hasta +0x12 con corrección del contador según versión (<0x70/<0x71), +0x14…+0x3A, y relleno 0xFF
/// de +0x24/+0x34/+0x38 en guardados anteriores a 0x73; devuelve el final.
pub fn convoy_deserialize(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let mut s = a[0];
    let ver = a[1];
    let b = m.r8(s);
    m.w8(this, b);
    let b = m.r8(s + 1);
    m.w8(this + 1, b);
    s += 2;
    for (off, n) in [(2u32, 2u32), (4, 4), (8, 2), (0xa, 2), (0xc, 4), (0x10, 2), (0x12, 2)] {
        copy_bytes(m, this + off, s, n);
        s += n;
    }
    if ver < 0x70 {
        let v = m.r16(this + 0x12);
        if v > 9 && v < 0xff {
            m.w16(this + 0x12, v + 1);
        }
    }
    if ver < 0x71 {
        let v = m.r16(this + 0x12);
        if v > 0xa && v < 0xff {
            m.w16(this + 0x12, v + 1);
        }
    }
    for (off, n) in [(0x14u32, 2u32), (0x16, 2), (0x18, 4), (0x1c, 2)] {
        copy_bytes(m, this + off, s, n);
        s += n;
    }
    let b = m.r8(s);
    m.w8(this + 0x1e, b);
    s += 1;
    let b = m.r8(s);
    m.w8(this + 0x1f, b);
    s += 1;
    if ver < 0x73 {
        copy_bytes(m, this + 2, s, 2);
        s += 8;
    }
    copy_bytes(m, this + 0x20, s, 4);
    s += 4;
    if ver < 0x73 {
        for k in 0..0x10 {
            m.w8(this + 0x24 + k, 0xff);
        }
        for k in 0..4 {
            m.w8(this + 0x34 + k, 0xff);
        }
        m.w16(this + 0x38, 0xffff);
        return s;
    }
    copy_bytes(m, this + 0x24, s, 0x10);
    s += 0x10;
    copy_bytes(m, this + 0x34, s, 4);
    s += 4;
    copy_bytes(m, this + 0x38, s, 2);
    s + 2
}

/// 0x00543490: reinicia los nombres del comerciante `+4` en la ventana de nombres (0x512BA0) salvo que tenga la bandera 4 en +8; el tercer argumento es el byte bajo del índice.
pub fn op_reset_merchant_name_slots(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    let idx = m.r32(this + 4);
    let count = u32::from(m.r16(0x006d_e4aa));
    if idx >= count {
        return;
    }
    let merchant = host.call(m, 0x5303c0, 0x006d_e4a0, &[idx]);
    if m.r8(merchant + 8) & 4 != 0 {
        return;
    }
    let flag = u32::from(m.r8(merchant + 0x1a) != 0);
    let third = u32::from(m.r8(this + 4));
    host.call(m, 0x512ba0, 0x006d_daa0, &[merchant + 0x1c, merchant + 0x1d, third, flag]);
}

/// 0x0052DF20: lee el almacén desde `a[0]` (versión `a[1]`): antes de 0x5B no trae +0x124 (se pone a cero y se saltan 0xC bytes) y antes de 0x64 no trae +0x2BC; devuelve el final.
pub fn storage_deserialize(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let mut s = a[0];
    let ver = a[1];
    for (off, n) in [(0u32, 4u32), (4, 0x60), (0x64, 0x60), (0xc4, 0x60)] {
        copy_bytes(m, this + off, s, n);
        s += n;
    }
    if ver < 0x5b {
        for k in 0..0x18 {
            m.w8(this + 0x124 + k, 0);
        }
        s += 0xc;
    } else {
        copy_bytes(m, this + 0x124, s, 0x18);
        s += 0x18;
    }
    copy_bytes(m, this + 0x13c, s, 0x180);
    s += 0x180;
    if ver < 0x64 {
        m.w32(this + 0x2bc, 0);
        return s;
    }
    copy_bytes(m, this + 0x2bc, s, 4);
    s + 4
}

/// 0x00503E30: constructor de la lista de barcos (punteros, contadores y las 16 entradas de 12 bytes de +0x20..+0xDE inicializadas); devuelve `this`.
pub fn ships_construct(m: &mut dyn Mem, this: u32) -> u32 {
    for off in [0u32, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c] {
        m.w32(this + off, 0);
    }
    for (off, v) in [(0xe0u32, 0u16), (0xe2, 0), (0xe4, 0), (0xe6, 0xffff), (0xe8, 0xffff), (0xea, 0xffff), (0xec, 0xffff), (0xf2, 0), (0xf4, 0), (0xf6, 0), (0xf8, 0)] {
        m.w16(this + off, v);
    }
    for k in 1..=16u32 {
        let c = this + 0xea - 0xc * k;
        m.w16(c - 0xa, 0xffff);
        m.w16(c, 0);
        m.w16(c - 8, 0);
        m.w16(c - 2, 0xff);
    }
    this
}
/// 0x005119B0: constructor del banco de nombres (a cero todo salvo contadores); devuelve `this`.
pub fn name_bank_init(m: &mut dyn Mem, this: u32) -> u32 {
    m.w32(this, 0);
    m.w32(this + 4, 0);
    for k in 0..0x28u32 {
        m.w32(this + 8 + k * 4, 0);
    }
    for off in 0xa8..=0xcc {
        if off % 4 == 0 {
            m.w32(this + off, 0);
        }
    }
    for off in [0xd0u32, 0xd2, 0xd4, 0xd6, 0xd8, 0xda, 0xdc] {
        m.w16(this + off, 0);
    }
    m.w32(this + 0xe0, 0);
    m.w32(this + 0xe4, 0);
    this
}
/// 0x00511B90: tamaño de guardado del banco de nombres: 0x53 + longitud (con terminador) de 20 cadenas por cada una de las tres tablas + 4 (+ el valor de la lista de personalizados).
pub fn name_bank_get_save_size(m: &mut dyn Mem, this: u32) -> u32 {
    let mut size = 0x53u32;
    for (base_off, idx_off, start_off) in [(0xa8u32, 0xbcu32, 0xd0u32), (0xac, 0xc0, 0xd2), (0xb0, 0xc4, 0xd6)] {
        let base = m.r32(this + base_off);
        let mut e = m.r32(this + idx_off).wrapping_add(u32::from(m.r16(this + start_off)) * 2);
        for _ in 0..20 {
            let mut p = base.wrapping_add(u32::from(m.r16(e)));
            loop {
                let b = m.r8(p);
                size = size.wrapping_add(1);
                p = p.wrapping_add(1);
                if b == 0 {
                    break;
                }
            }
            e = e.wrapping_add(2);
        }
    }
    size = size.wrapping_add(4);
    let extra = m.r32(this + 0xcc);
    if extra != 0 {
        let d = u32::from(m.r16(this + 0xda)).wrapping_sub(u32::from(m.r16(this + 0xd8)));
        size = size.wrapping_add(u32::from(m.r16(extra.wrapping_add(d.wrapping_mul(2)))));
    }
    size
}

/// 0x00512BA0 (ret 0x10): elige la ranura de nombre `start+idx` (con el bit 0x80 en la rama sin `flag`), la escribe en `*out1`, y limpia 25 bytes de su cadena; lo mismo con la tabla 3 y `*out2`. Devuelve 0.
pub fn name_bank_reset_merchant_name_slots(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let (out1, out2, idx, flag) = (a[0], a[1], a[2] as u8, a[3] as u8);
    let (slot, base, tab);
    if flag != 0 {
        slot = m.r8(this + 0xd0).wrapping_add(idx);
        m.w8(out1, slot);
        base = m.r32(this + 0xa8);
        tab = m.r32(this + 0xbc);
    } else {
        slot = m.r8(this + 0xd2).wrapping_add(idx);
        m.w8(out1, slot);
        base = m.r32(this + 0xac);
        tab = m.r32(this + 0xc0);
        m.w8(out1, slot | 0x80);
    }
    let idx1 = slot as u32;
    let p = base.wrapping_add(u32::from(m.r16(tab.wrapping_add(idx1 * 2))));
    for k in 0..25 {
        m.w8(p.wrapping_add(k), 0);
    }
    let s2 = m.r8(this + 0xd6).wrapping_add(idx);
    m.w8(out2, s2);
    let t2 = m.r32(this + 0xc4);
    let w2 = u32::from(m.r16(t2.wrapping_add(u32::from(s2) * 2)));
    let p = m.r32(this + 0xb0).wrapping_add(w2);
    for k in 0..25 {
        m.w8(p.wrapping_add(k), 0);
    }
    0
}

/// 0x00503F70: tamaño de guardado de la lista de barcos: `0x10*N + 0x27`, más cada barco (stride 0x180), cada convoy (stride 0x3C, 0x3A cada uno) y `+0xC0` más el tamaño de las cinco listas auxiliares no nulas.
pub fn ships_get_save_size(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let mut size = auto_trader_get_record_size().wrapping_mul(u32::from(m.r16(this + 0xf2))).wrapping_add(0x27);
    let ships = m.r32(this + 4);
    for k in 0..i32::from(m.r16(this + 0xf4)) {
        let s = ships.wrapping_add((k as u32).wrapping_mul(0x180));
        size = size.wrapping_add(ship_get_save_size(m, host, s));
    }
    for _ in 0..i32::from(m.r16(this + 0xf6)) {
        size = size.wrapping_add(convoy_get_save_size());
    }
    size = size.wrapping_add(0xc0);
    for k in 0..5u32 {
        let p = m.r32(this + 0xc + 4 * k);
        if p != 0 {
            size = size.wrapping_add(host.call(m, 0x5137b0, p, &[]));
        }
    }
    size
}

/// 0x00618F80: instala la tabla virtual `0x67AF70` y continúa en el constructor base `0x00618F90`; devuelve lo que este devuelva.
pub fn local_map_ship_list_init(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    m.w32(this, 0x67af70);
    host.call(m, 0x618f90, this, &[])
}
/// 0x00534600 (cdecl): recorre la lista global `0x6DE584` buscando el nodo con clave `+4 == a`, `+0xC == b` y `+0xD == c`; si lo hay salta a `0x534330` con ese nodo. Si no, deja `c` en EAX (con la lista vacía EAX es el valor previo del llamador: aquí 0).
pub fn trade_offer_accept_by_key(m: &mut dyn Mem, host: &mut dyn OpsHost, a: u32, b: u32, c: u32) -> u32 {
    let mut node = m.r32(0x6de584);
    if node == 0 {
        return 0;
    }
    while node != 0 {
        if m.r32(node + 4) == a && u32::from(m.r8(node + 0xc)) == b && u32::from(m.r8(node + 0xd)) == c {
            return host.call(m, 0x534330, node, &[a, b, c]);
        }
        node = m.r32(node);
    }
    c
}
/// 0x005005F0: máscara de la oficina: OR de los índices (no bits) de las 24 ranuras con existencias (`+0xC4`) más, por cada producto de la cadena `0x6DE4A0` desde `+0x2CC`, `1 << tabla[0x672C2C + tipo]`; bit 0x2000 si el bit 1, y 0x400 si el bit 2 y la ciudad de la oficina tiene `>0` en `+0x2CC`.
pub fn office_get_produced_wares_mask(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let mut mask = 0u32;
    for k in 0..0x18u32 {
        if (m.r32(this + 0xc4 + 4 * k) as i32) > 0 {
            mask |= k;
        }
    }
    let mut idx = u32::from(m.r16(this + 0x2cc));
    while (idx as i32) < i32::from(m.r16(0x6de4a6)) {
        let rec = host.call(m, 0x5303b0, 0x6de4a0, &[idx]);
        let kind = u32::from(m.r8(rec + 6));
        idx = u32::from(m.r16(rec + 8));
        let sh = u32::from(m.r8(0x672c2c + kind));
        mask |= 1u32.wrapping_shl(sh & 31);
    }
    if mask & 2 != 0 {
        mask |= 0x2000;
    }
    if mask & 4 != 0 {
        let town = u32::from(m.r8(this + 0x2c6));
        let off = town.wrapping_mul(319).wrapping_mul(8);
        let base = m.r32(0x6de508);
        if m.r32(base.wrapping_add(off).wrapping_add(0x2cc)) != 0 {
            mask |= 0x400;
        }
    }
    mask
}

/// 0x00539CF0: envía el convoy `+4` hacia la ciudad `+8` (índice `<` recuento): si ya está en ella, está ocupado (`+0x12 != 0`) o su primer barco no es válido, no hace nada; si no, guarda destino, rumbo `(+0xC << 8)` y estado 1, y recorre la cadena de sus barcos poniendo `+0x3C |= 0x20`, rumbo en `+0x138` y estado 1 en `+0x134`. EAX no es un valor de retorno definido.
pub fn op_convoy_depart_to_town(m: &mut dyn Mem, this: u32) -> u32 {
    let conv_i = m.r32(this + 4);
    if conv_i >= u32::from(m.r16(0x6dd896)) {
        return 0;
    }
    let town = m.r32(this + 8);
    let towns = m.r32(0x6de4b0);
    if town >= towns {
        return 0;
    }
    let conv = m.r32(0x6dd7a8).wrapping_add(conv_i.wrapping_mul(60));
    if u32::from(m.r8(conv + 0x1f)) == town || m.r16(conv + 0x12) != 0 || u32::from(m.r16(conv + 0x1c)) >= towns {
        return 0;
    }
    m.w8(conv + 0x1f, town as u8);
    let heading = m.r16(this + 0xc) << 8;
    m.w16(conv + 0x16, heading);
    m.w16(conv + 0x12, 1);
    let mut idx = u32::from(m.r16(conv + 0xa));
    let count = |m: &mut dyn Mem| u32::from(m.r16(0x6dd894));
    if idx >= count(m) {
        return 0;
    }
    loop {
        let ship = m.r32(0x6dd7a4).wrapping_add(idx.wrapping_mul(0x180));
        let next = u32::from(m.r16(ship + 6));
        let flags = m.r16(ship + 0x3c) | 0x20;
        m.w16(ship + 0x3c, flags);
        m.w16(ship + 0x138, heading);
        m.w16(ship + 0x134, 1);
        idx = next;
        if idx >= count(m) {
            return 0;
        }
    }
}
/// 0x00611309 (ret 4): suma progreso a la barra de batalla naval `+0x1E94` (10 bits de valor y byte alto) por cada bit del argumento, usando la escala `(1000-v)*(1000-(1000-e)) / 1000` con `e` decreciente (`e = (e*e+500)/1000`, inicial 0x3B6). Con `+0xAD2 == 3` no hace nada. Como el original, solo termina si el argumento es `< 0x8000`.
pub fn op_sea_battle_add_progress(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    if m.r8(this + 0xad2) == 3 {
        return 0;
    }
    let arg = a[0];
    let mut edi: u32 = 0x3b6;
    let mut ebx: u32 = 1;
    loop {
        if (ebx & arg) & 0xffff != 0 {
            let si = u32::from(m.r16(this + 0x1e94));
            let mut edx = (0x3e8u32).wrapping_sub(si & 0x3ff);
            let ebp = (0x3e8u32).wrapping_sub((0x3e8u32).wrapping_sub(edi) & 0xffff);
            edx = (edx as i32).wrapping_mul(ebp as i32) as u32;
            edx = edx.wrapping_add(0x1f4);
            let hi = ((i64::from(0x10624dd3i32) * i64::from(edx as i32)) >> 32) as i32;
            let mut q = hi >> 6;
            q += ((q as u32) >> 31) as i32;
            let high = ((si >> 10) & 0xff) << 10;
            let v = high.wrapping_sub(q as u32).wrapping_add(0x3e8);
            m.w16(this + 0x1e94, v as u16);
        }
        ebx = ebx.wrapping_add(ebx);
        if (ebx & 0xffff) > (arg & 0xffff) {
            return 0;
        }
        let e = edi.wrapping_mul(edi).wrapping_add(0x1f4);
        edi = ((u64::from(0x10624dd3u32) * u64::from(e)) >> 38) as u32;
    }
}
/// 0x00512FC0 (ret 8): si `slot == +0xDA - 1` y existe el bloque `+0xE4`, sustituye el texto de la última ranura añadida: decrementa su desplazamiento (`+0xCC`, índice `+0xDA - +0xD8`), copia la cadena de `src` (con el cero) sobre el búfer `+0xB8` y deja el cero final. Devuelve 1, o 0 si no aplica (solo AL).
pub fn name_bank_rewrite_last_added_ship_name(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let (mut src, slot) = (a[0], a[1] & 0xffff);
    let da = u32::from(m.r16(this + 0xda));
    if slot != da.wrapping_sub(1) {
        return 0;
    }
    if m.r32(this + 0xe4) == 0 {
        return 0;
    }
    let idx = da.wrapping_sub(u32::from(m.r16(this + 0xd8)));
    let tbl = m.r32(this + 0xcc);
    let slot_addr = tbl.wrapping_add(idx.wrapping_mul(2));
    let v = m.r16(slot_addr).wrapping_sub(1);
    m.w16(slot_addr, v);
    loop {
        let bl = m.r8(src);
        let off = u32::from(m.r16(slot_addr));
        let buf = m.r32(this + 0xb8);
        m.w8(buf.wrapping_add(off), bl);
        let v = m.r16(slot_addr).wrapping_add(1);
        m.w16(slot_addr, v);
        src = src.wrapping_add(1);
        if bl == 0 {
            break;
        }
    }
    let off = u32::from(m.r16(slot_addr));
    let buf = m.r32(this + 0xb8);
    m.w8(buf.wrapping_add(off), 0);
    1
}

/// 0x00503ED0: destructor de la lista de barcos: destruye y libera las cinco listas auxiliares (`+0xC..`), destruye cada barco (`0x516840`) y libera la tabla de barcos, y libera los búferes `+0` y `+8`. EAX no es un valor definido.
pub fn ships_destruct(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    for k in 0..5u32 {
        let p = m.r32(this + 0xc + 4 * k);
        if p != 0 {
            host.call(m, 0x513760, p, &[]);
            host.call(m, FREE, 0, &[p]);
        }
    }
    let mut ret = 0;
    if m.r32(this + 4) != 0 {
        let count = u32::from(m.r16(this + 0xf4));
        for i in 0..count {
            let ship = m.r32(this + 4).wrapping_add(i.wrapping_mul(0x180));
            host.call(m, 0x516840, ship, &[i]);
        }
        let p = m.r32(this + 4);
        ret = host.call(m, FREE, 0, &[p]);
        m.w32(this + 4, 0);
    }
    let p = m.r32(this);
    if p != 0 {
        ret = host.call(m, FREE, 0, &[p]);
    }
    let p = m.r32(this + 8);
    if p != 0 {
        ret = host.call(m, FREE, 0, &[p]);
    }
    ret
}
/// 0x005118A0 (ret 4): destructor con liberación opcional (`flag & 1`) del almacén de scripts: instala la tabla `0x673608`, libera cada puntero de la tabla `+4` (pares de 8 bytes, recuento `+8`) y la tabla, pone el recuento a cero y libera `this` si procede. Devuelve `this`.
pub fn script_blob_pool_scalar_deleting_destructor(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    m.w32(this, 0x673608);
    if m.r32(this + 4) != 0 {
        let mut i = 0u32;
        if m.r16(this + 8) != 0 {
            loop {
                let table = m.r32(this + 4);
                let p = m.r32(table.wrapping_add(i * 8));
                if p != 0 {
                    host.call(m, FREE, 0, &[p]);
                }
                let table = m.r32(this + 4);
                m.w32(table.wrapping_add(i * 8), 0);
                i += 1;
                if i as i32 >= i32::from(m.r16(this + 8)) {
                    break;
                }
            }
        }
        let table = m.r32(this + 4);
        host.call(m, FREE, 0, &[table]);
        m.w32(this + 4, 0);
    }
    m.w16(this + 8, 0);
    if a[0] as u8 & 1 != 0 {
        host.call(m, FREE, 0, &[this]);
    }
    this
}
/// 0x00513060: amplía en 0x20 entradas la tabla de desplazamientos de nombres `+0xCC` (capacidad `+0xDC`): reserva `2*cap+0x40`, copia la anterior y la libera. Sin tabla previa deja la nueva con un cero inicial. EAX sin valor definido salvo el primer caso (0).
pub fn name_bank_grow_dynamic_ship_offsets(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let cap = u32::from(m.r16(this + 0xdc));
    let new = host.call(m, ALLOC, 0, &[cap * 2 + 0x40]);
    let old = m.r32(this + 0xcc);
    if old == 0 {
        m.w16(new, 0);
        m.w16(this + 0xdc, (cap as u16).wrapping_add(0x20));
        m.w32(this + 0xcc, new);
        return 0;
    }
    copy_bytes(m, new, old, cap * 2);
    let old = m.r32(this + 0xcc);
    let ret = host.call(m, FREE, 0, &[old]);
    m.w16(this + 0xdc, (cap as u16).wrapping_add(0x20));
    m.w32(this + 0xcc, new);
    ret
}
/// 0x005130D0: amplía en 0x100 bytes el búfer de nombres `+0xB8` (tamaño `+0xE0`): copia el anterior, lo libera y pone a cero los 0x100 nuevos. Devuelve el tamaño nuevo.
pub fn name_bank_grow_dynamic_ship_blob(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let size = m.r32(this + 0xe0);
    let new = host.call(m, ALLOC, 0, &[size.wrapping_add(0x100)]);
    let size = m.r32(this + 0xe0);
    if size > 0 {
        let old = m.r32(this + 0xb8);
        copy_bytes(m, new, old, size);
        let old = m.r32(this + 0xb8);
        host.call(m, FREE, 0, &[old]);
        let off = m.r32(this + 0xe0).wrapping_add(new);
        for k in 0..0x100 {
            m.w8(off.wrapping_add(k), 0);
        }
    } else {
        for k in 0..0x100 {
            m.w8(new.wrapping_add(k), 0);
        }
    }
    let size = m.r32(this + 0xe0).wrapping_add(0x100);
    m.w32(this + 0xb8, new);
    m.w32(this + 0xe0, size);
    size
}
/// 0x0051B290: destructor de ciudad: libera la lista enlazada `+0x9B4` nodo a nodo y continúa en `0x0051E8A0`.
pub fn town_destructor(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    if m.r32(this + 0x9b4) != 0 {
        loop {
            let node = m.r32(this + 0x9b4);
            let next = m.r32(node);
            host.call(m, FREE, 0, &[node]);
            m.w32(this + 0x9b4, next);
            if next == 0 {
                break;
            }
        }
    }
    host.call(m, 0x51e8a0, this, &[])
}
/// 0x00549C40: destruye los 4 textos de preparación de ciudad (`+0x1BC`, 0x28 bytes cada uno) con el iterador de vector `0x639DA8`.
pub fn destroy_town_setup_name_strings(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let p = this.wrapping_add(0x1bc);
    host.call(m, 0x639da8, p, &[p, 4, 0x28, 0x64f253])
}
/// 0x00611740 (ret 4): destructor virtual con liberación opcional: llama a `0x603BC0` y, con `flag & 1`, libera `this`. Devuelve `this`.
pub fn delete_this(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    host.call(m, 0x603bc0, this, &[]);
    if a[0] as u8 & 1 != 0 {
        host.call(m, FREE, 0, &[this]);
    }
    this
}

/// 0x005348B0: une en `this` los mapas de descubrimiento (palabras `+0..+0x2F`, `+0x30..+0x5F` y `+0x60`) de todos los comerciantes con datos (`+0x468 != 0`, bloque desde `+0x4D8`). EAX sin valor definido.
pub fn map_discovery_rebuild_union_of_merchants(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let mut i = 0u32;
    if m.r16(0x6de4aa) == 0 {
        return 0;
    }
    loop {
        let rec = host.call(m, 0x5303c0, 0x6de4a0, &[i]);
        if m.r32(rec.wrapping_add(0x468)) != 0 {
            let blk = rec.wrapping_add(0x4d8);
            let v = m.r32(this + 0x60) | m.r32(blk.wrapping_add(0x60));
            m.w32(this + 0x60, v);
            for k in 0..12u32 {
                let a = m.r32(this + 4 * k) | m.r32(blk.wrapping_add(4 * k));
                m.w32(this + 4 * k, a);
                let b = m.r32(this + 0x30 + 4 * k) | m.r32(blk.wrapping_add(0x30 + 4 * k));
                m.w32(this + 0x30 + 4 * k, b);
            }
        }
        i += 1;
        if i as i32 >= i32::from(m.r16(0x6de4aa)) {
            return 0;
        }
    }
}
/// 0x005465B0: lee (una vez) el intervalo de autoguardado de la configuración (`0x4BE0B0`), lo acota a 2..=0x2760 y lo multiplica por 60.000 en `this+0x944`. Devuelve ese campo.
pub fn operations_load_autosave_interval(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let cur = m.r32(this + 0x944);
    if cur != 0 {
        return cur;
    }
    let mut v = host.call(m, 0x4be0b0, 0x6dd524, &[0x6c381c, 0x6c3814, 0xf, 0x6c3804]);
    if v < 2 {
        v = 2;
    } else if v > 0x2760 {
        v = 0x2760;
    }
    let v = v.wrapping_mul(60000);
    m.w32(this + 0x944, v);
    v
}
/// 0x00512730: reparte los tres bloques de cadenas del banco de nombres (`[this]..[this+4]`) en las tablas globales `0x6DDA00` (40 ranuras, liberando la anterior si queda fuera del bloque), `0x6DD960` y `0x6DD8C0` (40 cadenas cada una, topadas al final del bloque).
pub fn name_bank_populate_town_name_tables(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let mut esi = m.r32(this);
    let mut off = 0u32;
    while off < 0xa0 {
        let slot = this + 8 + off;
        let old = m.r32(slot);
        if old != 0 && (old < m.r32(this) || old >= m.r32(this + 4)) {
            host.call(m, FREE, 0, &[old]);
        }
        m.w32(slot, esi);
        m.w32(0x6dda00 + off, esi);
        loop {
            let b = m.r8(esi);
            esi = esi.wrapping_add(1);
            if b == 0 {
                break;
            }
        }
        off += 4;
    }
    for table in [0x6dd960u32, 0x6dd8c0] {
        let mut ecx = 0u32;
        while ecx < 0xa0 {
            m.w32(table + ecx, esi);
            if m.r8(esi) == 0 {
                esi = esi.wrapping_add(1);
            } else {
                let end = m.r32(this + 4);
                loop {
                    esi = esi.wrapping_add(1);
                    if esi >= end {
                        esi = end.wrapping_sub(1);
                        break;
                    }
                    if m.r8(esi) == 0 {
                        esi = esi.wrapping_add(1);
                        break;
                    }
                }
            }
            ecx += 4;
        }
    }
    0
}

/// 0x00512830 (ret 8): copia el nombre de 0x30 bytes `src` a la ranura personalizada `idx` (< 0x28; reserva 0x30 si la ranura está vacía o aún apunta al bloque base) y, para cada ciudad (tabla `0x6DE4B8`) que usa ese `idx`, repunta `0x6DDA00` a la ranura y `0x6DD960` a la cadena `idx+0x28` del bloque base. EAX sin valor definido.
pub fn name_bank_set_custom_town_name(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let (idx, src) = (a[0], a[1]);
    if idx >= 0x28 {
        return 0;
    }
    let cell = this.wrapping_add(idx.wrapping_mul(4)).wrapping_add(8);
    let cur = m.r32(cell);
    if cur == 0 || !(cur < m.r32(this) || cur >= m.r32(this + 4)) {
        let p = host.call(m, ALLOC, 0, &[0x30]);
        m.w32(cell, p);
    }
    let slot = m.r32(cell);
    copy_bytes(m, slot, src, 0x30);
    for ecx in 0..0x28u32 {
        if u32::from(m.r8(0x6de4b8 + ecx)) != idx {
            continue;
        }
        let slot = m.r32(cell);
        m.w32(ecx * 4 + 0x6dda00, slot);
        let base = m.r32(this);
        m.w32(ecx * 4 + 0x6dd960, base);
        let mut left = idx + 0x28;
        while left > 0 {
            let p = m.r32(ecx * 4 + 0x6dd960);
            let b = m.r8(p);
            m.w32(ecx * 4 + 0x6dd960, p.wrapping_add(1));
            if b != 0 {
                continue;
            }
            left -= 1;
        }
    }
    0
}
/// 0x00512C50 (ret 0xC): añade hasta 12 caracteres de `text` al final de un nombre de ranura (tipo 0/1/2 = tabla de nombres de pila, apellidos u otra; otro tipo: `idx` es el puntero), sin pasar de 0x16 caracteres en total. No hace nada si `idx` está fuera de rango o el nombre ya es largo. EAX sin valor definido.
pub fn name_bank_append_custom_name_text(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let (kind, arg, text) = (a[0], a[1], a[2]);
    let mut p;
    match kind {
        0 | 1 | 2 => {
            let (start_off, idx_off, base_off, i) = match kind {
                0 => (0xd0u32, 0xbcu32, 0xa8u32, arg),
                1 => (0xd2, 0xc0, 0xac, arg & 0x7f),
                _ => (0xd6, 0xc4, 0xb0, arg),
            };
            if i >= u32::from(m.r16(this + start_off)) + 0x14 {
                return 0;
            }
            let tbl = m.r32(this + idx_off);
            let w = u32::from(m.r16(tbl.wrapping_add(i.wrapping_mul(2))));
            p = m.r32(this + base_off).wrapping_add(w);
        }
        _ => p = arg,
    }
    let mut len = 0u32;
    if m.r8(p) != 0 {
        loop {
            len += 1;
            if len >= 0x17 {
                return 0;
            }
            p = p.wrapping_add(1);
            if m.r8(p) == 0 {
                break;
            }
        }
    }
    let mut copied = 0u32;
    let mut src = text;
    if len < 0x17 {
        loop {
            len += 1;
            let n = copied;
            copied += 1;
            if n as i32 >= 0xc {
                break;
            }
            let b = m.r8(src);
            if b == 0 {
                break;
            }
            m.w8(p, b);
            p = p.wrapping_add(1);
            src = src.wrapping_add(1);
            if len >= 0x17 {
                break;
            }
        }
    }
    m.w8(p, 0);
    0
}
/// 0x0050DDE0 (ret 4): recuenta los estados de los barcos del comerciante `idx`: sin dueño en el convoy `+0x100/+0x102` (según la tabla de saltos `0x50DEDC`), con convoy y estado `<=4` en `+0x104` y estado 0xF en `+0x106`. EAX sin valor definido.
pub fn recount_merchant_ship_state_counters(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    for off in [0x100u32, 0x102, 0x104, 0x106] {
        m.w16(this + off, 0);
    }
    let arg = a[0];
    if arg >= u32::from(m.r16(0x6de4aa)) {
        return 0;
    }
    let rec = host.call(m, 0x5303c0, 0x6de4a0, &[arg]);
    let mut ship_i = u32::from(m.r16(rec.wrapping_add(0xe)));
    let count = u32::from(m.r16(this + 0xf4));
    if ship_i >= count {
        return 0;
    }
    loop {
        let ship = m.r32(this + 4).wrapping_add(ship_i.wrapping_mul(0x180));
        let conv = u32::from(m.r16(ship + 8));
        let state = u32::from(m.r16(ship + 0x134));
        if conv < u32::from(m.r16(this + 0xf6)) {
            let convs = m.r32(this + 8);
            let owner = u32::from(m.r16(convs.wrapping_add(conv * 60 + 0x10)));
            if owner == ship_i {
                if state <= 4 {
                    let v = m.r16(this + 0x104).wrapping_add(1);
                    m.w16(this + 0x104, v);
                } else if state == 0xf {
                    let v = m.r16(this + 0x106).wrapping_add(1);
                    m.w16(this + 0x106, v);
                }
            }
        } else if state <= 0xf {
            let k = u32::from(m.r8(0x50dee8 + state));
            match m.r32(0x50dedc + k * 4) {
                0x50deb5 => {
                    let v = m.r16(this + 0x100).wrapping_add(1);
                    m.w16(this + 0x100, v);
                }
                0x50debe => {
                    let v = m.r16(this + 0x102).wrapping_add(1);
                    m.w16(this + 0x102, v);
                }
                _ => {}
            }
        }
        ship_i = u32::from(m.r16(ship + 4));
        if ship_i >= count {
            return 0;
        }
    }
}

/// 0x005268D0 (ret 8): busca entre los dos contratos de suministro del jugador (`+0x830`, ids de 16 bits) el primero con `rec+0x16 ==` jugador actual (`0x6DFC14`); copia sus 0x10 bytes (`rec+8`) a `out` y escribe su id en `*id_out`. Si no hay, rellena `out` con un registro por defecto y `*id_out = 0xFFFF`. Devuelve `out`.
pub fn town_get_player_supply_contract_record(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let (out, id_out) = (a[0], a[1]);
    for k in 0..2u32 {
        let id = m.r16(this + 0x830 + 2 * k);
        if id < m.r16(0x6dd748) {
            let rec = host.call(m, 0x4d8db0, 0x6dd73c, &[u32::from(id)]);
            if u32::from(m.r8(rec.wrapping_add(0x16))) == m.r32(0x6dfc14) {
                let id2 = u32::from(m.r16(this + 0x830 + 2 * k));
                let rec2 = host.call(m, 0x4d8db0, 0x6dd73c, &[id2]);
                copy_bytes(m, out, rec2.wrapping_add(8), 0x10);
                let id3 = m.r16(this + 0x830 + 2 * k);
                m.w16(id_out, id3);
                return out;
            }
        }
    }
    let b = m.r8(this + 0x2c0);
    m.w8(out, b);
    m.w8(out + 1, 0);
    m.w8(out + 2, 0);
    m.w8(out + 3, 0);
    m.w16(out + 8, 0);
    let t = m.r32(0x6de4b4).wrapping_add(0x16d00);
    m.w16(out + 0xa, 0);
    m.w32(out + 4, t);
    m.w16(out + 0xc, 0);
    let p = m.r8(0x6dfc14);
    m.w8(out + 0xe, p);
    m.w8(out + 0xf, 0);
    m.w16(id_out, 0xffff);
    out
}
/// 0x00529E20 (ret 8): soborno esperado del comerciante `a0` (`< 0x6DE4AA`) en la posición `a1 & 0xFF < 4`: según su reputación (`rec+0x39c[rec+0x19]`, tabla de saltos `0x529EF4`) y un azar `0x639C03 % 11`, `((azar + 4*v + 0x10) * 500)` (x2 si la ranura `this+0x6DC+pos` es válida); `0x80000000` si los índices no valen.
pub fn town_calculate_expected_bribe(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let (arg1, arg2) = (a[0], a[1]);
    if (arg1 as u16) >= m.r16(0x6de4aa) || (arg2 as u8) >= 4 {
        return 0x8000_0000;
    }
    let rec = host.call(m, 0x5303c0, 0x6de4a0, &[arg1 & 0xffff]);
    let k = u32::from(m.r8(rec.wrapping_add(0x19)));
    let rep = u32::from(m.r8(k.wrapping_add(rec).wrapping_add(0x39c)));
    let mut v = arg1 & 0xff;
    if rep <= 7 {
        v = match m.r32(rep.wrapping_mul(4).wrapping_add(0x529ef4)) {
            0x529e6e => 0,
            0x529e75 => 1,
            0x529e7c => 2,
            0x529e83 => 3,
            0x529e8a => 5,
            0x529e91 => 7,
            0x529e98 => 0xa,
            0x529e9f => 0xf,
            _ => v,
        };
    }
    let r = host.call(m, 0x639c03, k, &[]) as i32; // ECX sobrante: índice del byte de reputación
    let rem = r.wrapping_rem(11) as u32;
    let pos = arg2 & 0xff;
    let eax = rem.wrapping_add(v.wrapping_mul(4)).wrapping_add(0x10).wrapping_mul(500);
    let slot = u32::from(m.r8(pos.wrapping_add(this).wrapping_add(0x6dc)));
    if slot >= u32::from(m.r16(0x6de4aa)) {
        eax
    } else {
        eax.wrapping_add(eax)
    }
}

/// 0x005051E0 (ret 4): ocupa la primera de las 16 ranuras de entidades meteorológicas (`+0x20`, stride 0xC) cuyo primer campo sea `>= +0xF4`, la rellena desde `src` (byte 0, palabras +4, +8, +0xC), llama a `0x507D60` y `0x50CE50`, y marca `[0x6CBDD8]+0x2B22 = 1`. Sin ranura libre devuelve `this+0xE0`; si no, el puntero global `0x6CBDD8`.
pub fn op_spawn_weather_entity(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let src = a[0];
    let limit = m.r16(this + 0xf4);
    let mut i = 0u32;
    let mut p = this + 0x20;
    while m.r16(p) < limit {
        i += 1;
        p += 0xc;
        if i >= 0x10 {
            return p;
        }
    }
    let edi = this + i * 12;
    let b = u16::from(m.r8(src));
    m.w16(edi + 0x28, b);
    let w4 = m.r16(src + 4);
    let slot = edi + 0x20;
    m.w16(edi + 0x22, w4);
    m.w16(edi + 0x2a, 0);
    let r = host.call(m, 0x507d60, this, &[slot, i, 4]);
    let w8 = m.r16(src + 8);
    m.w16(this + (i * 3 + 9) * 4, w8);
    let wc = m.r16(src + 0xc);
    m.w16(edi + 0x26, wc);
    let w8 = m.r16(src + 8);
    m.w16(r.wrapping_add(0x1e), w8);
    let wc = m.r16(src + 0xc);
    m.w16(r.wrapping_add(0x22), wc);
    let id = u32::from(m.r16(slot));
    host.call(m, 0x50ce50, this, &[id, 0xffff]);
    let g = m.r32(0x6cbdd8);
    if g != 0 {
        m.w8(g + 0x2b22, 1);
    }
    g
}

/// 0x004E2BC4: puja vigente de una subasta (tarea programada): valida la tarea ligada (`+0xC`, tipo 0xA o 0x36, mismo señor `+0x11 == +0x15`, tiempo no pasado y estado `<=7`, comerciante válido) y, según su estado, avanza el reloj de la tarea (`+1`, `+3`) o fija el nuevo ganador (`+0xE`, `+0x17`, estado 4). 1 si la procesa, 0 si no.
pub fn st_auction_standing_bid(m: &mut dyn Mem, this: u32) -> u32 {
    let base = m.r32(this);
    let t = base.wrapping_add(u32::from(m.r16(this + 8)).wrapping_mul(24));
    let other = u32::from(m.r16(t + 0xc));
    if other >= u32::from(m.r16(this + 0xc)) {
        return 0;
    }
    let o = base.wrapping_add(other.wrapping_mul(24));
    let kind = m.r16(o + 6);
    if kind != 0xa && kind != 0x36 {
        return 0;
    }
    if m.r8(o + 0x15) != m.r8(t + 0x11) {
        return 0;
    }
    let time = m.r32(o);
    if time < m.r32(0x6de4b4) {
        return 0;
    }
    let state = m.r8(o + 0x14);
    if state > 7 {
        return 0;
    }
    let who = m.r8(t + 0x10);
    if u16::from(who) >= m.r16(0x6de4aa) {
        return 0;
    }
    if state <= 3 {
        m.w32(t, time.wrapping_add(1));
        return 1;
    }
    let cur = m.r16(o + 8);
    if cur != m.r16(t + 0xe) {
        let v = m.r32(t).wrapping_add(3);
        m.w32(t, v);
        return 1;
    }
    let si = m.r16(o + 0x10);
    let di = m.r16(o + 0xe);
    if (u32::from(di) + u32::from(si)) as i32 > m.r32(t + 8) as i32 {
        return 0;
    }
    if m.r8(o + 0x17) != who && state != 7 {
        m.w16(o + 0xe, di.wrapping_add(si));
        let w = m.r8(t + 0x10);
        m.w8(o + 0x17, w);
        m.w8(o + 0x14, 4);
        m.w16(o + 0x12, 0);
    }
    let v = m.r32(t).wrapping_add(1);
    m.w32(t, v);
    1
}
/// 0x0052AFC0 (ret 4): estima (en formato tiempo `día<<8 | hora`) cuándo acabará de construirse el barco encargado en la ciudad: recorre la cadena de barcos desde `+0x81C` (hasta el barco `a0`), suma los días pendientes (`+0x14 - +0x18`) y el retraso (`(0x1C - +0x18) << 8`), divide por la capacidad diaria (`+0x854 * 100`) y suma el instante actual `0x6DE4B4` (la hora se compara con `+0x2C0 * 8 + 7`). Devuelve 0 si el primer barco no es válido.
pub fn town_estimate_ship_build_finish_time(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let arg = a[0];
    let first = m.r16(this + 0x81c);
    let ax = u32::from(m.r16(this + 0x854));
    let count = m.r32(0x6dd894) & 0xffff;
    let now = m.r32(0x6de4b4);
    let edi = ax.wrapping_mul(100);
    let (mut suma, mut sumb) = (0u32, 0u32);
    if u32::from(first) >= count {
        return 0;
    }
    let mut edx = u32::from(first);
    loop {
        let ship = m.r32(0x6dd7a4).wrapping_add(edx.wrapping_mul(0x180));
        let mut s = m.r32(ship + 0x18) as i32;
        if s < 0x1c {
            if s < 0 {
                m.w32(ship + 0x18, 0);
            }
            s = m.r32(ship + 0x18) as i32;
            sumb = sumb.wrapping_add((0x1ci32.wrapping_sub(s) as u32) << 8);
        }
        suma = suma.wrapping_add(m.r32(ship + 0x14).wrapping_sub(s as u32));
        if edx == arg {
            break;
        }
        edx = u32::from(m.r16(ship + 6));
        if edx >= count {
            break;
        }
    }
    let num = (edi as i32).wrapping_add(suma as i32).wrapping_sub(1);
    let mut q = num.wrapping_div(edi as i32);
    let thr = (u32::from(m.r8(this + 0x2c0)) * 8 + 7) as i32;
    if ((now & 0xff) as i32) < thr {
        q = q.wrapping_sub(1);
    }
    ((q as u32) << 8).wrapping_add(sumb).wrapping_add(now & 0xffff_ff00)
}

/// 0x00509030 (ret 8): sustituye en todos los barcos el índice de parada de ruta (`+0x132`) igual a `a0` por `a1`. EAX sin valor definido.
pub fn replace_ship_route_stop_index(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    let (old, new) = (a[0] as u16, a[1] as u16);
    let ships = m.r32(this + 4);
    let n = u32::from(m.r16(this + 0xf4));
    for i in 0..n {
        let p = ships.wrapping_add(i * 0x180 + 0x132);
        if m.r16(p) == old {
            m.w16(p, new);
        }
    }
    0
}
/// 0x00533D10 (ret 4): serializa la lista de ofertas enlazada (19 bytes por nodo: `+4`, `+8`, 4 bytes `+C..+F`, `+0x10`, dos ceros y el byte de continuación) en `buf`; devuelve el final.
pub fn trade_offer_list_save(m: &mut dyn Mem, this: u32, buf: u32) -> u32 {
    let mut p = buf;
    copy_bytes(m, p, this + 4, 4);
    p += 4;
    copy_bytes(m, p, this + 8, 4);
    p += 4;
    for k in 0..4 {
        let b = m.r8(this + 0xc + k);
        m.w8(p, b);
        p += 1;
    }
    copy_bytes(m, p, this + 0x10, 4);
    p += 4;
    m.w8(p, 0);
    m.w8(p + 1, 0);
    p += 2;
    let next = m.r32(this);
    if next != 0 {
        m.w8(p, 1);
        trade_offer_list_save(m, next, p + 1)
    } else {
        m.w8(p, 0);
        p + 1
    }
}
/// 0x0053E7C0: ordena al barco `this+4` (o al líder de su convoy) fijar la parada `+8/+0xC` con el modo `(+0x10 == 1)` mediante `0x4D4F60`; si la parada queda inválida marca el barco (`+0x3D |= 8`, `+0x136 &= 0xFC`) y continúa en `0x48FD90`.
pub fn op_move_route_stop(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let count = m.r32(0x6dd894);
    let idx = m.r32(this + 4);
    if idx >= (count & 0xffff) {
        return 0;
    }
    let ships = m.r32(0x6dd7a4);
    let mut s = ships.wrapping_add(idx.wrapping_mul(0x180));
    let conv = m.r16(s + 8);
    if conv < m.r16(0x6dd896) {
        let cv = m.r32(0x6dd7a8).wrapping_add(u32::from(conv) * 60);
        let lead = m.r16(cv + 0x10);
        if lead > count as u16 {
            return 0;
        }
        s = ships.wrapping_add(u32::from(lead) * 0x180);
    }
    let p = s + 0x132;
    if m.r16(p) >= m.r16(0x6dd72a) {
        return 0;
    }
    let mode = u32::from(m.r32(this + 0x10) == 1);
    let (a1, a2) = (m.r32(this + 8), m.r32(this + 0xc));
    host.call(m, 0x4d4f60, 0x6dd728, &[p, a1, a2, mode]);
    if m.r16(p) >= m.r16(0x6dd72a) {
        let v = m.r8(s + 0x136) & 0xfc;
        let f = m.r8(s + 0x3d) | 8;
        m.w8(s + 0x3d, f);
        m.w8(s + 0x136, v);
    }
    let e = m.r32(0x6ce6d0);
    host.call(m, 0x48fd90, e, &[])
}

/// 0x0053E610: edita la ciudad de la parada `this+4`: con `this+8 != 0` inserta una parada nueva (`0x4D4C90`) delante enlazando la anterior; si no, o bien la borra (ciudad `>= recuento`: reindexa los barcos con `0x509030` y libera con `0x4D4E80`) o cambia su ciudad a `this+0xC`, reajustando el barco `this+0x10` y limpiando las banderas de la parada. EAX sin valor definido.
pub fn op_edit_route_stop_town(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let stop = m.r32(this + 4);
    if stop >= u32::from(m.r16(0x6dd72a)) {
        return 0;
    }
    let table = m.r32(0x6dd72c);
    let mut edi = table.wrapping_add(stop.wrapping_mul(220));
    if m.r32(this + 8) != 0 {
        if m.r32(this + 0xc) >= m.r32(0x6de4b0) {
            return 0;
        }
        let r = host.call(m, 0x4d4c90, 0x6dd728, &[]);
        let table = m.r32(0x6dd72c);
        let a = table.wrapping_add(m.r32(this + 4).wrapping_mul(220));
        let b = table.wrapping_add((r & 0xffff).wrapping_mul(220));
        let w = m.r16(a);
        m.w16(b, w);
        m.w16(a, r as u16);
        let t = m.r8(this + 0xc);
        m.w8(b + 2, t);
        m.w8(b + 3, 0);
        return 0;
    }
    let town = m.r32(this + 0xc);
    if town >= m.r32(0x6de4b0) {
        let w = u32::from(m.r16(edi));
        let arg = if w == stop {
            0xffff
        } else {
            if m.r8(edi + 3) & 4 == 4 {
                let o = table.wrapping_add(w.wrapping_mul(220) + 3);
                let v = m.r8(o) | 4;
                m.w8(o, v);
            }
            w
        };
        let s4 = m.r32(this + 4);
        host.call(m, 0x509030, 0x6dd7a0, &[s4, arg]);
        let s4 = m.r32(this + 4);
        host.call(m, 0x4d4e80, 0x6dd728, &[s4]);
        return 0;
    }
    if town == u32::from(m.r8(edi + 2)) {
        return 0;
    }
    let keep = m.r8(edi + 3) & 4;
    let t = m.r8(this + 0xc);
    m.w8(edi + 2, t);
    m.w8(edi + 3, keep);
    let ship_i = m.r32(this + 0x10);
    if ship_i < (m.r32(0x6dd894) & 0xffff) {
        let s = m.r32(0x6dd7a4).wrapping_add(ship_i.wrapping_mul(0x180));
        let cur = m.r32(this + 4);
        if u32::from(m.r16(s + 0x132)) == cur {
            let dl = m.r8(s + 0x136) & 0xfc;
            let f = m.r8(s + 0x3c) | 0x20;
            m.w8(s + 0x3c, f);
            m.w8(s + 0x136, dl);
            if m.r16(s + 0x134) >= 0xc {
                let w = u32::from(m.r16(this + 0x10));
                host.call(m, 0x5171e0, s, &[w]);
            }
        }
        let owner = u32::from(m.r8(s));
        if owner < u32::from(m.r16(0x6de4aa)) {
            let st = u32::from(m.r8(edi + 2));
            if host.call(m, 0x5308a0, 0x6de4a0, &[owner, st, 0]) != 0 {
                return 0;
            }
        }
    }
    edi = edi.wrapping_add(0x7c);
    for k in 0..0x18u32 {
        if m.r32(edi + 4 * k - 0x60) == 0 {
            m.w32(edi + 4 * k, 0);
        }
    }
    0
}

/// 0x00533DA0 (ret 4): carga de `buf` la lista de ofertas (formato de `trade_offer_list_save`, con un bloque de relleno de 16 bits tras el nodo): rellena `this` (`+4`, `+8`, `+0xC..+0xF`, `+0x10`), libera la cola anterior y, si el flag lo pide, reserva un nodo nuevo (0x14 bytes) y continúa. Devuelve el final leído.
pub fn trade_offer_list_load(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, buf: u32) -> u32 {
    copy_bytes(m, this + 4, buf, 4);
    copy_bytes(m, this + 8, buf + 4, 4);
    for k in 0..4u32 {
        let b = m.r8(buf + 8 + k);
        m.w8(this + 0xc + k, b);
    }
    copy_bytes(m, this + 0x10, buf + 12, 4);
    let skip = u32::from(m.r16(buf + 16));
    let end = buf.wrapping_add(16).wrapping_add(skip).wrapping_add(2);
    let next = m.r32(this);
    if next != 0 {
        let after = m.r32(next);
        if after != 0 {
            trade_offer_free_following_nodes(m, host, after);
        }
        m.w32(next, 0);
        host.call(m, FREE, 0, &[next]);
    }
    m.w32(this, 0);
    if m.r8(end) == 0 {
        return end + 1;
    }
    let p = host.call(m, ALLOC, 0, &[0x14]);
    m.w32(p, 0);
    m.w32(this, p);
    trade_offer_list_load(m, host, p, end + 1)
}

/// 0x0053ECB0: resuelve una oferta de comercio `(this+4, this+8, this+0xC)`: sin flag `+0x10` la acepta (`0x534600`), la cancela y reembolsa (`0x5340D0`) y, para los dos comerciantes (`+8` y `+0xC`), localiza en la lista de eventos `0x6DD730` el aviso de tipo 0x1E/0x1F de esa oferta y lo retira con `0x4D6A50`. EAX sin valor definido.
pub fn op_resolve_trade_offer(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let (k1, k2, k3) = (m.r32(this + 4), m.r32(this + 8), m.r32(this + 0xc));
    if m.r32(this + 0x10) == 0 {
        trade_offer_accept_by_key(m, host, k1, k2, k3);
    }
    host.call(m, 0x5340d0, 0, &[0x6de584, k1, k2, k3]);
    for (who, kind) in [(k2, 0x1eu8), (k3, 0x1f)] {
        if who >= u32::from(m.r16(0x6de4aa)) {
            if kind == 0x1e {
                continue;
            }
            return 0;
        }
        let rec = host.call(m, 0x5303c0, 0x6de4a0, &[who]);
        let mut idx = u32::from(m.r16(rec.wrapping_add(0xa)));
        let count = u32::from(m.r16(0x6dd736));
        if idx >= count {
            if kind == 0x1e {
                continue;
            }
            return 0;
        }
        let base = m.r32(0x6dd730);
        loop {
            let p = base.wrapping_add(idx << 4);
            if m.r8(p + 4) == kind && m.r32(p + 8) == k1 && u32::from(m.r16(p + 0xc)) == k2 && u32::from(m.r16(p + 0xe)) == k3 {
                host.call(m, 0x4d6a50, 0x6dd730, &[who, idx]);
                break;
            }
            idx = u32::from(m.r16(p + 6));
            if idx >= count {
                break;
            }
        }
    }
    0
}

/// 0x005340D0 (cdecl, `cell, key, b, c`): quita de la lista enlazada `cell` la oferta `(+4, +0xC, +0xD) == (key, b, c)` y devuelve lo comprometido: según su tipo (`+0xF`) 0 = reembolso de dinero/mercancía al comerciante `b` (`0x5308A0`) o, sin su registro, a un barco suyo con hueco (`0x5182B0`/`0x517770`); 1 = devuelve el barco `+0x10` a la lista libre `0x6DD888`; 2 = suma `+0x10` a la caja del comerciante `+0xC`. Libera el nodo. EAX sin valor definido.
pub fn trade_offer_cancel_and_refund(m: &mut dyn Mem, host: &mut dyn OpsHost, a: &[u32]) -> u32 {
    let (head, key, b, c) = (a[0], a[1], a[2], a[3]);
    if head == 0 || m.r32(head) == 0 {
        return 0;
    }
    let mut cur = head;
    let node;
    loop {
        let n = m.r32(cur);
        if m.r32(n + 4) == key && u32::from(m.r8(n + 0xc)) == b && u32::from(m.r8(n + 0xd)) == c {
            node = n;
            break;
        }
        if m.r32(n) == 0 {
            return 0;
        }
        cur = n;
    }
    let next = m.r32(node);
    m.w32(cur, next);
    let kind = m.r8(node + 0xf);
    m.w32(node, 0);
    let mut free_only = false;
    match kind {
        0 => {
            if m.r16(node + 0x10) >= 0x18 {
                free_only = true;
            } else {
                let arg = u32::from(m.r8(node + 0xe)); // el 0x5308A0 solo usa 16 bits (los altos son ruido del llamador)
                let rec = host.call(m, 0x5308a0, 0x6de4a0, &[b, arg, 0]);
                let slot = u32::from(m.r16(node + 0x10));
                let per = if m.r8(0x672c14 + slot) != 0 { 200u32 } else { 2000 };
                let amount = u32::from(m.r16(node + 0x12)).wrapping_mul(per);
                if rec != 0 {
                    let cell = rec.wrapping_add(slot * 4 + 4);
                    let v = m.r32(cell).wrapping_add(amount);
                    m.w32(cell, v);
                } else if b < u32::from(m.r16(0x6de4aa)) {
                    let town = host.call(m, 0x5303c0, 0x6de4a0, &[b]);
                    let mut idx = u32::from(m.r16(town.wrapping_add(0xe)));
                    loop {
                        if idx >= (m.r32(0x6dd894) & 0xffff) {
                            break;
                        }
                        let ship = m.r32(0x6dd7a4).wrapping_add(idx.wrapping_mul(0x180));
                        if m.r8(ship + 0x39) == m.r8(node + 0xe) && m.r16(ship + 0x134) < 4 {
                            let r = host.call(m, 0x5182b0, ship, &[]);
                            if (r as i32) >= amount as i32 {
                                let cell = ship + slot * 4 + 0x54;
                                let v = m.r32(cell).wrapping_add(amount);
                                m.w32(cell, v);
                                host.call(m, 0x517770, ship, &[4]);
                                host.call(m, 0x5182b0, ship, &[]);
                                break;
                            }
                        }
                        idx = u32::from(m.r16(ship + 4));
                        if idx >= (m.r32(0x6dd894) & 0xffff) {
                            break;
                        }
                    }
                }
            }
        }
        1 => {
            let ci = u32::from(m.r16(node + 0x10));
            if ci as u16 >= m.r16(0x6dd894) || ci >= (m.r32(0x6dd894) & 0xffff) {
                free_only = true;
            } else {
                let ship = m.r32(0x6dd7a4).wrapping_add(ci * 0x180);
                if m.r16(ship + 6) != 0xffff {
                    free_only = true;
                } else {
                    m.w16(ship + 0x134, 0);
                    let f = m.r8(ship + 0x3c) | 0x20;
                    m.w8(ship + 0x3c, f);
                    let head_free = m.r16(0x6dd888);
                    m.w16(ship + 6, head_free);
                    m.w16(0x6dd888, ci as u16);
                }
            }
        }
        2 => {
            let who = m.r8(node + 0xc);
            if u16::from(who) >= m.r16(0x6de4aa) {
                free_only = true;
            } else {
                let rec = host.call(m, 0x5303c0, 0x6de4a0, &[u32::from(who)]);
                let add = m.r32(node + 0x10);
                let v = m.r32(rec).wrapping_add(add);
                m.w32(rec, v);
            }
        }
        _ => free_only = true,
    }
    if !free_only {
        let tail = m.r32(node);
        if tail != 0 {
            trade_offer_free_following_nodes(m, host, tail);
        }
    }
    m.w32(node, 0);
    host.call(m, FREE, 0, &[node]);
    0
}

/// 0x00534330 (`this` = oferta): ejecuta la transferencia de una oferta aceptada entre dos comerciantes (`+0xD` comprador/`+0xC` vendedor): mueve el dinero `+8` y, según el tipo (`+0xF`), 0 = mercancía (`0x5007A0` con coste `+0x12 * 200/2000`), 1 = barco `+0x10` (de la lista del vendedor a la del comprador, `0x4F5900`), 2 = dinero extra `+0x10`; deja la oferta consumida (`+0xF = 2`, `+8 = +0x10 = 0`). EAX sin valor definido.
pub fn trade_offer_accept_transfer(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let count = m.r16(0x6de4aa);
    let buyer = m.r8(this + 0xd);
    let seller = m.r8(this + 0xc);
    if u16::from(buyer) >= count || u16::from(seller) >= count {
        return 0;
    }
    let bp = host.call(m, 0x5303c0, 0x6de4a0, &[u32::from(buyer)]);
    let seller_i = u32::from(m.r8(this + 0xc));
    let sb = host.call(m, 0x5303c0, 0x6de4a0, &[seller_i]);
    let price = m.r32(this + 8) as i32;
    if price > 0 && (m.r32(bp) as i32) < price {
        return 0;
    }
    let v = m.r32(bp).wrapping_sub(price as u32);
    m.w32(bp, v);
    let v = m.r32(sb).wrapping_add(m.r32(this + 8));
    m.w32(sb, v);
    let consume = |m: &mut dyn Mem| {
        m.w8(this + 0xf, 2);
        m.w32(this + 8, 0);
        m.w32(this + 0x10, 0);
    };
    let add = |m: &mut dyn Mem, a: u32, d: u32| {
        let v = m.r32(a).wrapping_add(d);
        m.w32(a, v);
    };
    match m.r8(this + 0xf) {
        2 => {
            let extra = m.r32(this + 0x10);
            let v = m.r32(bp).wrapping_add(extra);
            m.w32(bp, v);
            add(m, bp + 0x4b8, extra);
            add(m, sb + 0x4bc, extra);
            let p = m.r32(this + 8);
            add(m, bp + 0x4bc, p);
            add(m, sb + 0x4b8, p);
            consume(m);
        }
        1 => {
            let ship_i = u32::from(m.r16(this + 0x10));
            let total = m.r32(0x6dd894) & 0xffff;
            if ship_i as u16 >= m.r16(0x6dd894) {
                consume(m);
                return 0;
            }
            let ships = m.r32(0x6dd7a4);
            if ship_i < total {
                let s = ships.wrapping_add(ship_i * 0x180);
                if m.r16(s + 6) == 0xffff {
                    m.w16(s + 0x134, 0);
                    let f = m.r8(s + 0x3c) | 0x20;
                    m.w8(s + 0x3c, f);
                    let head = m.r16(0x6dd888);
                    m.w16(s + 6, head);
                    m.w16(0x6dd888, ship_i as u16);
                }
            }
            let mut link = sb + 0xe;
            let mut cur = m.r16(link);
            if cur >= m.r16(0x6dd894) {
                consume(m);
                return 0;
            }
            let want = m.r16(this + 0x10);
            while cur != want {
                let s = ships.wrapping_add(u32::from(cur) * 0x180);
                link = s + 4;
                cur = m.r16(link);
                if cur >= m.r16(0x6dd894) {
                    m.w8(this + 0xf, 2);
                    m.w32(this + 8, 0);
                    m.w32(this + 0x10, 0);
                    return 0;
                }
            }
            let ship = ships.wrapping_add(u32::from(want) * 0x180);
            let next = m.r16(ship + 4);
            m.w16(link, next);
            let first = m.r16(bp + 0xe);
            m.w16(ship + 4, first);
            let w = m.r16(this + 0x10);
            m.w16(bp + 0xe, w);
            let old = u32::from(m.r8(ship));
            let rec = host.call(m, 0x5303c0, 0x6de4a0, &[old]);
            host.call(m, 0x4f5900, rec, &[]);
            let nb = m.r8(this + 0xd);
            let f = m.r8(ship + 0x3d) | 4;
            m.w8(ship + 0x3d, f);
            m.w8(ship, nb);
            let v = m.r32(bp + 0x47c).wrapping_add(1);
            let w = m.r16(bp + 0x478).wrapping_add(1);
            m.w16(bp + 0x478, w);
            m.w32(bp + 0x47c, v);
            let v = m.r32(sb + 0x47c).wrapping_sub(1);
            let w = m.r16(sb + 0x478).wrapping_sub(1);
            m.w16(sb + 0x478, w);
            m.w32(sb + 0x47c, v);
            let p = m.r32(this + 8);
            add(m, bp + 0x4bc, p);
            add(m, sb + 0x4b8, p);
            consume(m);
        }
        0 => {
            let a0 = u32::from(m.r8(this + 0xd));
            let a1 = u32::from(m.r8(this + 0xe));
            let rec = host.call(m, 0x5308a0, 0x6de4a0, &[a0, a1, 0]);
            if rec != 0 {
                let p = m.r32(this + 8);
                let slot = u32::from(m.r16(this + 0x10));
                let per = if m.r8(0x672c14 + slot) != 0 { 200u32 } else { 2000 };
                let amount = u32::from(m.r16(this + 0x12)).wrapping_mul(per);
                host.call(m, 0x5007a0, rec, &[slot, amount, p]);
                let p = m.r32(this + 8);
                add(m, bp + 0x4a0, p);
                let p = m.r32(this + 8);
                add(m, sb + 0x49c, p);
            }
            consume(m);
        }
        _ => consume(m),
    }
    0
}

/// 0x0053DF00: activa (`this+8 != 0`) o desactiva la ruta comercial del barco `this+4` (o del líder de su convoy). Desactivar limpia banderas y estado del barco o de todo su convoy; activar avanza la parada hasta una válida (`0x4D5AB0`) y, según el estado del barco (0xF, 8, o el resto con `0x519BA0`), lo despacha a la ciudad de la parada (`0x5172A0`), lo libera de un convoy (estado 6, devuelve el convoy a la lista libre `0x6DD88C`) o limpia su rumbo. EAX sin valor definido.
pub fn op_set_trade_route_active(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let n_ships = |m: &mut dyn Mem| m.r32(0x6dd894) & 0xffff;
    let idx = m.r32(this + 4);
    if idx >= n_ships(m) {
        return 0;
    }
    let ships = m.r32(0x6dd7a4);
    let mut esi = ships.wrapping_add(idx.wrapping_mul(0x180));
    let conv = m.r16(esi + 8);
    if m.r32(this + 8) == 0 {
        if conv >= m.r16(0x6dd896) {
            let dl = m.r8(esi + 0x136);
            let f = m.r8(esi + 0x3d) | 8;
            m.w8(esi + 0x3d, f);
            m.w8(esi + 0x136, dl & 0xfc);
            if m.r16(esi + 0x134) <= 1 {
                let b = m.r8(esi + 0x39);
                m.w8(esi + 0x38, b);
            }
            return 0;
        }
        let cv = m.r32(0x6dd7a8).wrapping_add(u32::from(conv) * 60);
        if m.r16(cv + 0x12) <= 1 {
            let b = m.r8(cv + 0x1c);
            m.w8(cv + 0x1f, b);
        }
        let mut ecx = u32::from(m.r16(cv + 0xa));
        if ecx >= n_ships(m) {
            return 0;
        }
        loop {
            let ship = m.r32(0x6dd7a4).wrapping_add(ecx.wrapping_mul(0x180));
            if m.r8(ship) == m.r8(cv) {
                let cl = m.r8(ship + 0x136) & 0xfc;
                let w = m.r16(ship + 0x3c) | 0x800;
                m.w16(ship + 0x3c, w);
                m.w8(ship + 0x136, cl);
            }
            if m.r16(ship + 0x134) <= 1 {
                let b = m.r8(ship + 0x39);
                m.w8(ship + 0x38, b);
            }
            ecx = u32::from(m.r16(ship + 6));
            if ecx >= n_ships(m) {
                return 0;
            }
        }
    }
    if conv < m.r16(0x6dd896) {
        let cv = m.r32(0x6dd7a8).wrapping_add(u32::from(conv) * 60);
        let lead = u32::from(m.r16(cv + 0x10));
        m.w32(this + 4, lead);
        esi = m.r32(0x6dd7a4).wrapping_add(lead.wrapping_mul(0x180));
    } else if m.r16(esi + 0x42) >= m.r16(0x6dd892) {
        return 0;
    }
    if m.r8(esi + 0x15c) != 0 {
        return 0;
    }
    let stop = u32::from(m.r16(esi + 0x132));
    let invalid = |m: &mut dyn Mem, esi: u32| {
        let cl = m.r8(esi + 0x136) & 0xfc;
        let f = m.r8(esi + 0x3d) | 8;
        m.w8(esi + 0x3d, f);
        m.w8(esi + 0x136, cl);
    };
    if stop as u16 >= m.r16(0x6dd72a) || m.r16(esi + 0x42) >= m.r16(0x6dd892) {
        invalid(m, esi);
        return 0;
    }
    let edi = stop;
    host.call(m, 0x51a3c0, esi, &[]);
    let mut ok = host.call(m, 0x4d5ab0, 0x6dd728, &[esi]) as u8 != 0;
    while !ok {
        let table = m.r32(0x6dd72c);
        let s = u32::from(m.r16(esi + 0x132));
        let nxt = m.r16(table.wrapping_add(s.wrapping_mul(220)));
        m.w16(esi + 0x132, nxt);
        if edi == u32::from(nxt) {
            return 0;
        }
        ok = host.call(m, 0x4d5ab0, 0x6dd728, &[esi]) as u8 != 0;
    }
    host.call(m, 0x51a450, esi, &[]);
    let state = m.r16(esi + 0x134);
    if state == 0xf {
        let f = m.r8(esi + 0x3d) | 8;
        m.w8(esi + 0x3d, f);
        m.w8(esi + 0x136, 1);
        let s = u32::from(m.r16(esi + 0x132));
        let table = m.r32(0x6dd72c);
        let town = u32::from(m.r8(table.wrapping_add(s.wrapping_mul(220) + 2)));
        if town >= m.r32(0x6de4b0) {
            return 0;
        }
        let w = u32::from(m.r16(this + 4));
        return host.call(m, 0x5172a0, esi, &[town, 0, 0, w]);
    }
    if state == 8 {
        let q = u32::from(m.r8(esi + 0x39));
        let towns = m.r32(0x6de4b0);
        let rem = q % towns;
        let off = rem.wrapping_mul(319).wrapping_mul(8);
        let base = m.r32(0x6de508);
        let cur = u32::from(m.r16(base.wrapping_add(off).wrapping_add(0x994)));
        let ebx = m.r32(this + 4);
        if cur == ebx {
            m.w32(base.wrapping_add(off).wrapping_add(0x990), 0);
            let base2 = m.r32(0x6de508);
            m.w16(base2.wrapping_add(off).wrapping_add(0x994), 0xffff);
        }
        let w = m.r16(esi + 0x3c) | 0x820;
        m.w16(esi + 0x3c, w);
        m.w8(esi + 0x37, rem as u8);
        m.w8(esi + 0x136, 1);
        let s = u32::from(m.r16(esi + 0x132));
        let table = m.r32(0x6dd72c);
        let al = m.r8(table.wrapping_add(s.wrapping_mul(220) + 2));
        let cl = m.r8(esi + 0x39);
        m.w8(esi + 0x38, al);
        if al == cl {
            m.w16(esi + 0x134, 3);
            let dx = 0x40u16.wrapping_sub(m.r16(esi + 0x138));
            m.w16(esi + 0x138, dx);
        } else {
            m.w16(esi + 0x134, 2);
        }
        return 0;
    }
    if host.call(m, 0x519ba0, esi, &[]) as u8 == 0 {
        return 0;
    }
    let state = m.r16(esi + 0x134);
    let f = m.r8(esi + 0x3d) | 8;
    m.w8(esi + 0x3d, f);
    m.w8(esi + 0x136, 1);
    if state <= 1 {
        let c = m.r16(esi + 8);
        if c >= m.r16(0x6dd896) {
            m.w16(esi + 0x138, 0);
            return 0;
        }
        let cv = m.r32(0x6dd7a8).wrapping_add(u32::from(c) * 60);
        let mut eax = u32::from(m.r16(cv + 0xa));
        m.w16(cv + 0x16, 0);
        loop {
            if eax >= n_ships(m) {
                return 0;
            }
            let ship = m.r32(0x6dd7a4).wrapping_add(eax.wrapping_mul(0x180));
            let next = u32::from(m.r16(ship + 6));
            m.w16(ship + 0x138, 0);
            eax = next;
        }
    }
    if state != 6 {
        return 0;
    }
    let cx = m.r16(esi + 8);
    let bx = m.r16(0x6dd896);
    let merchant = u32::from(m.r8(esi + 0x39)).wrapping_mul(319);
    let base = m.r32(0x6de508);
    if cx < bx {
        let convs = m.r32(0x6dd7a8);
        let mut link = base.wrapping_add(merchant.wrapping_mul(8)).wrapping_add(0x820);
        let mut ax = m.r16(link);
        while ax != cx {
            if ax >= bx {
                return 0;
            }
            link = convs.wrapping_add(u32::from(ax) * 60 + 8);
            ax = m.r16(link);
        }
        let conv_p = convs.wrapping_add(u32::from(cx) * 60);
        let nxt = m.r16(conv_p + 8);
        m.w16(link, nxt);
        let free_head = m.r16(0x6dd88c);
        m.w16(conv_p + 8, free_head);
        let me = m.r16(esi + 8);
        m.w16(0x6dd88c, me);
        m.w16(conv_p + 0x12, 0);
        let mut eax = u32::from(m.r16(conv_p + 0xa));
        while eax < n_ships(m) {
            let ship = m.r32(0x6dd7a4).wrapping_add(eax.wrapping_mul(0x180));
            let next = u32::from(m.r16(ship + 6));
            let w = m.r16(ship + 0x3c) | 0x100;
            m.w16(ship + 0x3c, w);
            m.w16(ship + 0x134, 0);
            eax = next;
        }
        let a = u32::from(m.r16(conv_p + 0x10));
        return host.call(m, 0x613920, 0x6e6220, &[a]);
    }
    let want = m.r32(this + 4);
    let mut link = base.wrapping_add(merchant.wrapping_mul(8)).wrapping_add(0x81e);
    let mut dx = m.r16(link);
    while u32::from(dx) != want {
        if dx >= m.r16(0x6dd894) {
            return 0;
        }
        let ship = m.r32(0x6dd7a4).wrapping_add(u32::from(dx) * 0x180);
        link = ship + 6;
        dx = m.r16(link);
    }
    let nxt = m.r16(esi + 6);
    m.w16(link, nxt);
    m.w16(esi + 6, 0xffff);
    let cx = m.r32(this + 4);
    if cx < n_ships(m) {
        let ship = m.r32(0x6dd7a4).wrapping_add(cx.wrapping_mul(0x180));
        if m.r16(ship + 6) == 0xffff {
            m.w16(ship + 0x134, 0);
            let f = m.r8(ship + 0x3c) | 0x20;
            m.w8(ship + 0x3c, f);
            let head = m.r16(0x6dd888);
            m.w16(ship + 6, head);
            m.w16(0x6dd888, cx as u16);
        }
    }
    m.w16(esi + 0x134, 5);
    m.w16(esi + 0x138, 0);
    let a = u32::from(m.r16(this + 4));
    host.call(m, 0x613920, 0x6e6220, &[a])
}
/// 0x00638010 (ventana de tutorial): activa («1») el botón del objeto `+0x198` y el del `+0xB0` con su método virtual `+0x34`; después, si la página actual (`+0x28C`) llega al final (`+0x288 - 1`, comparación con signo) deja el botón `+0xB0` activo solo si `+0x2A4 < +0x2A8 - 1`; si no, con la página `<= 0` desactiva («0») el botón `+0x198`. Devuelve lo último que devolvió una llamada virtual o, con la página positiva, la propia página.
pub fn tutorial_window_update_navigation_buttons(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let prev = this.wrapping_add(0x198);
    let next = this.wrapping_add(0xb0);
    let vt = m.r32(prev);
    let f = m.r32(vt.wrapping_add(0x34));
    host.call(m, f, prev, &[1]);
    let vt = m.r32(next);
    let f = m.r32(vt.wrapping_add(0x34));
    host.call(m, f, next, &[1]);
    let page = m.r32(this.wrapping_add(0x28c));
    let last = m.r32(this.wrapping_add(0x288)).wrapping_sub(1);
    if (page as i32) >= (last as i32) {
        let lo_raw = m.r32(this.wrapping_add(0x2a4));
        let lo = lo_raw as i32;
        let hi = m.r32(this.wrapping_add(0x2a8)).wrapping_sub(1) as i32;
        let vt = m.r32(next);
        let f = m.r32(vt.wrapping_add(0x34));
        return host.call(m, f, next, &[(lo_raw & 0xffff_ff00) | bool32(lo < hi)]);
    }
    if (page as i32) > 0 {
        return page;
    }
    let vt = m.r32(prev);
    let f = m.r32(vt.wrapping_add(0x34));
    host.call(m, f, prev, &[0])
}
/// 0x006381B0 (ret 4): destructor con liberación opcional de la ventana de tutorial: llama al destructor base `0x636BA0` y, con `flag & 1`, libera `this`. Devuelve `this`.
pub fn tutorial_window_scalar_deleting_dtor(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    host.call(m, 0x636ba0, this, &[]);
    if a[0] as u8 & 1 != 0 {
        host.call(m, FREE, 0, &[this]);
    }
    this
}
/// 0x006381D0 (ret 4): envoltura de la anterior para la base secundaria (`this - 0x94`).
pub fn tutorial_window_secondary_deleting_dtor_thunk(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    tutorial_window_scalar_deleting_dtor(m, host, this.wrapping_sub(0x94), a)
}
/// 0x0061A6D0 (destructor de `LocalMapShip`): instala la tabla virtual `0x67B0D4`; si hay objeto en `+0x134` y existe `0x6E51AC`, llama a su método virtual `+4` (objeto `+0x3F8` de este, con el valor como argumento); con la marca `+0x160` limpia el byte `0x6E59CD`. EAX sin valor definido.
pub fn local_map_ship_destruct(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    m.w32(this, 0x67b0d4);
    let obj = m.r32(this.wrapping_add(0x134));
    if obj != 0 {
        let world = m.r32(0x6e51ac);
        if world != 0 {
            let sub = world.wrapping_add(0x3f8);
            let vt = m.r32(sub);
            let f = m.r32(vt.wrapping_add(4));
            host.call(m, f, sub, &[obj]);
        }
    }
    if m.r8(this.wrapping_add(0x160)) != 0 {
        m.w8(0x6e59cd, 0);
    }
    0
}

/// 0x005336B0 (ret 8): estado del trayecto terrestre del comerciante `a0` desde la ciudad en `*a1`: si el cursor `*a1` ya está fuera de la tabla `this+0x18` busca la ciudad vecina (tabla `0x672BF8`) y avanza el cursor; recorre las dos listas de eventos del comerciante hasta el de tipo 0x27. Devuelve 5 si no hay ciudad/enlace, 1 o 2 si falta el evento, 3 si el otro comerciante lleva la bandera 0x10 en `+0x2C8`, y si no 4 (bandera 0x10) o 0.
pub fn get_overland_route_status(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let who = a[0] & 0xff;
    let cursor = a[1];
    let off = who.wrapping_mul(319).wrapping_mul(8);
    let mut esi = m.r32(this + 0x68).wrapping_add(off);
    let count = m.r32(this + 0x10);
    let mut cl = m.r8(cursor);
    if u32::from(cl) >= count {
        let bl = m.r8(this.wrapping_add(who).wrapping_add(0x18));
        cl = 0xff;
        let mut i = 0u32;
        while i < 7 {
            let dl = m.r8(0x672bf8 + i * 4);
            if dl == bl {
                cl = m.r8(0x672bf9 + i * 4);
                i = 7;
            } else if m.r8(0x672bf9 + i * 4) == bl {
                cl = dl;
                i = 7;
            }
            i += 1;
        }
        if cl >= 0x28 {
            return 5;
        }
        m.w8(cursor, 0);
        if m.r8(this + 0x18) != cl {
            loop {
                let al = m.r8(cursor).wrapping_add(1);
                m.w8(cursor, al);
                if u32::from(al) >= m.r32(this + 0x10) {
                    return 5;
                }
                if m.r8(this + u32::from(al) + 0x18) == cl {
                    break;
                }
            }
        }
    }
    for round in 0..2 {
        let mut ecx = u32::from(m.r16(esi + 0x764));
        loop {
            if (ecx as i32) >= i32::from(m.r16(esi + 0x776)) {
                return if round == 0 { 1 } else { 2 };
            }
            let rec = host.call(m, 0x51f1f0, esi, &[ecx]);
            let kind = m.r8(rec.wrapping_add(3));
            ecx = u32::from(m.r16(rec.wrapping_add(4)));
            if kind == 0x27 {
                break;
            }
        }
        if round == 0 {
            esi = m.r32(this + 0x68).wrapping_add(off);
        }
    }
    let base = m.r32(this + 0x68);
    if m.r32(base.wrapping_add(off).wrapping_add(0x2c8)) & 0x10 == 0x10 {
        return 3;
    }
    let idx = u32::from(m.r8(cursor));
    let flag = m.r8(base.wrapping_add(idx.wrapping_mul(319).wrapping_mul(8)).wrapping_add(0x2c8)) & 0x10;
    if flag == 0x10 { 4 } else { 0 }
}

/// 0x0052F760: tamaño total del guardado del mundo: 0xC6 + ciudades (`0x51B2D0`, stride 0x9F8) + `n*n+4` + `0x633530` por entrada (stride 0x1024 sobre `[0x6DF270]`) + `0x4D3240 * word(+6)` + oficinas (`0x4FED30`, stride 0x44C) + `0x4F3FE0` (stride 0x650) + `0x5350F0(+0xE8)` + 15 ranuras (`(word<<4)+0x1E`) + `0x533CF0` de la lista de ofertas + 2.
pub fn game_world_get_save_size(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let mut total = 0xc6u32;
    let n = m.r32(this + 0x10);
    for i in 0..n {
        let ecx = m.r32(this + 0x68).wrapping_add(i.wrapping_mul(0x9f8));
        let r = host.call(m, 0x51b2d0, ecx, &[]);
        total = total.wrapping_add(r);
        if i + 1 >= m.r32(this + 0x10) {
            break;
        }
    }
    let n = m.r32(this + 0x10);
    let mut ecx = n.wrapping_mul(n);
    total = total.wrapping_add(ecx).wrapping_add(4);
    if n != 0 {
        let mut i = 0u32;
        loop {
            let base = m.r32(0x6df270);
            ecx = base.wrapping_add(i.wrapping_mul(0x1024));
            let r = host.call(m, 0x633530, ecx, &[]);
            total = total.wrapping_add(r);
            i += 1;
            if i >= m.r32(this + 0x10) {
                break;
            }
        }
    }
    let mult = host.call(m, 0x4d3240, ecx, &[]);
    total = total.wrapping_add(mult.wrapping_mul(u32::from(m.r16(this + 6))));
    if m.r16(this + 8) != 0 {
        let mut i = 0u32;
        loop {
            let ecx = m.r32(this + 0x74).wrapping_add(i.wrapping_mul(0x44c));
            let r = host.call(m, 0x4fed30, ecx, &[]);
            total = total.wrapping_add(r);
            i += 1;
            if i as i32 >= i32::from(m.r16(this + 8)) {
                break;
            }
        }
    }
    if m.r16(this + 0xa) != 0 {
        let mut i = 0u32;
        loop {
            let ecx = m.r32(this + 0x78).wrapping_add(i.wrapping_mul(0x650));
            let r = host.call(m, 0x4f3fe0, ecx, &[]);
            total = total.wrapping_add(r);
            i += 1;
            if i as i32 >= i32::from(m.r16(this + 0xa)) {
                break;
            }
        }
    }
    let r = host.call(m, 0x5350f0, this + 0xe8, &[]);
    total = total.wrapping_add(r);
    for k in 0..15u32 {
        let w = u32::from(m.r16(this + 0xb2c + k * 0x2c));
        total = total.wrapping_add(w << 4).wrapping_add(0x1e);
    }
    let list = m.r32(this + 0xe4);
    if list != 0 {
        let r = host.call(m, 0x533cf0, list, &[]);
        total = total.wrapping_add(r);
    }
    total.wrapping_add(2)
}
/// 0x0051D6F0 (ret 4): días estimados hasta que la ciudad construya el edificio `a0` de su cola (`+0x762` lista, `+0x776` tamaño): con capacidad `+0x864` (se reduce en 5 por cada edificio anterior que consume materiales) calcula `ceil(coste / capacidad)` y lo convierte con `0x52DFF0(+0x2C0, días)`. `0x10000000` si el edificio no está en la cola; 0 si no tiene coste; -1 si falta algo o la cola va por delante.
pub fn town_get_construction_eta_days(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let arg = a[0];
    let flag = (m.r32(this + 0x2c8) >> 4) & 1;
    let size = u32::from(m.r16(this + 0x776));
    let mut cap = u32::from(m.r16(this + 0x864));
    let mut local: u32 = 0xffff_ffff;
    if arg >= size {
        return 0x1000_0000;
    }
    let rec = host.call(m, 0x51f1f0, this, &[arg]);
    if m.r8(rec.wrapping_add(6)) == 0 {
        return 0;
    }
    let mut edi = u32::from(m.r16(this + 0x762));
    if edi >= u32::from(m.r16(this + 0x776)) {
        return 0x1000_0000;
    }
    let needs_material = |m: &mut dyn Mem, rec: u32| -> bool {
        let row = m.r32(this + 0x7ac).wrapping_mul(u32::from(m.r8(rec)));
        let idx = row.wrapping_add(u32::from(m.r8(rec + 1)));
        let table = m.r32(this + 0x80c);
        m.r8(idx.wrapping_add(table)) >= 0x80
    };
    loop {
        let rec = host.call(m, 0x51f1f0, this, &[edi]);
        if edi == arg {
            if cap == 0 {
                return local;
            }
            if flag != 0 && (m.r8(rec + 3) > 0x30 || !needs_material(m, rec)) {
                return 0xffff_ffff;
            }
            let who = u32::from(m.r8(this + 0x2c0));
            let cost = u32::from(m.r8(rec + 6));
            let days = if cap as i32 >= 5 {
                (cost + 4) / 5
            } else {
                ((cost + cap).wrapping_sub(1) as i32 / cap as i32) as u32
            };
            return host.call(m, 0x52dff0, 0, &[who, days]);
        }
        if cap as i32 > 0 {
            let skip = flag != 0 && (m.r8(rec + 3) > 0x31 || !needs_material(m, rec));
            if !skip {
                cap = if cap as i32 > 5 { cap - 5 } else { 0 };
            }
        } else {
            local = local.wrapping_sub(1);
        }
        edi = u32::from(m.r16(rec + 4));
        if edi >= u32::from(m.r16(this + 0x776)) {
            return 0x1000_0000;
        }
    }
}

/// 0x005396B0: operación «mover mercancía entre convoy y oficina» (`op+4` cantidad con signo, `op+8` convoy, `op+0xA` mercancía, `op+0xC` dueño,
/// `op+0xE` sentido). Con `+0xE == 0` recorre los barcos del convoy (cadena `ship+6` desde `convoy+0xA`) y pasa su carga de esa mercancía a la
/// oficina del dueño en la ciudad del convoy (`convoy+0x1C`), barco a barco hasta agotar la cantidad; si no, carga en los barcos lo que la
/// oficina tiene, en múltiplos del lote (200 o 2000), limitado por el hueco libre de cada barco. Cada barco tocado se recalcula (0x5182B0) y se marca (`+0x3C |= 4`).
pub fn op_move_wares_convoy_office(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) {
    const CONVOY_COUNT: u32 = 0x006d_d896;
    const CONVOY_TABLE: u32 = 0x006d_d7a8;
    let (convoy_idx, ware, owner) = (m.r16(this + 8), u32::from(m.r16(this + 0xa)), m.r16(this + 0xc));
    if convoy_idx >= m.r16(CONVOY_COUNT) || ware >= 0x18 || owner >= m.r16(0x006d_e4aa) {
        return;
    }
    let convoy = m.r32(CONVOY_TABLE).wrapping_add(u32::from(convoy_idx) * 0x3c);
    let town = u32::from(m.r16(convoy + 0x1c));
    let office = host.call(m, 0x5308a0, 0x006d_e4a0, &[u32::from(owner), town, 0]);
    if office == 0 {
        return;
    }
    let mut amount = m.r32(this + 4) as i32;
    if amount < 0 {
        amount = amount.wrapping_neg();
    }
    let mut remaining = amount;
    let count = |m: &mut dyn Mem| i32::from(m.r16(SHIP_COUNT));
    let ship_at = |m: &mut dyn Mem, i: u32| m.r32(SHIPS).wrapping_add(i.wrapping_mul(0x180));
    let mut index = u32::from(m.r16(convoy + 0xa));
    if m.r8(this + 0xe) == 0 {
        if (index as i32) >= count(m) {
            return;
        }
        loop {
            let ship = ship_at(m, index);
            let next = u32::from(m.r16(ship + 6));
            if u16::from(m.r8(ship)) == owner {
                let have = m.r32(ship + ware * 4 + 0x54) as i32;
                let price = m.r32(ship + ware * 4 + 0xb4);
                if have < amount {
                    amount = amount.wrapping_sub(have);
                    host.call(m, 0x4ff6f0, office, &[ware, have as u32, price]);
                    m.w32(ship + ware * 4 + 0x54, 0);
                    host.call(m, 0x5182b0, ship, &[]);
                    let flags = m.r8(ship + 0x3c);
                    m.w8(ship + 0x3c, flags | 4);
                } else {
                    m.w32(ship + ware * 4 + 0x54, (have.wrapping_sub(amount)) as u32);
                    host.call(m, 0x4ff6f0, office, &[ware, amount as u32, price]);
                    host.call(m, 0x5182b0, ship, &[]);
                    let flags = m.r8(ship + 0x3c);
                    m.w8(ship + 0x3c, flags | 4);
                    return;
                }
            }
            if (next as i32) >= count(m) {
                return;
            }
            index = next;
        }
    }
    let stock = m.r32(office + ware * 4 + 4) as i32;
    if stock == 0 {
        return;
    }
    let unit: i32 = if crate::shipyard::WARE_SMALL_UNIT[ware as usize] != 0 { 200 } else { 2000 };
    if amount > stock {
        remaining = stock;
        amount = stock;
    }
    if (index as i32) >= count(m) {
        return;
    }
    loop {
        let ship = ship_at(m, index);
        let next = u32::from(m.r16(ship + 6));
        if u16::from(m.r8(ship)) == owner {
            let mut free = host.call(m, 0x5182b0, ship, &[]) as i32;
            if free < 0 {
                free = 0;
            }
            if free >= unit {
                let price = m.r32(office + ware * 4 + 0x3b8);
                if free >= amount {
                    host.call(m, 0x5180c0, ship, &[ware, amount as u32, price]);
                    host.call(m, 0x5182b0, ship, &[]);
                    let flags = m.r8(ship + 0x3c);
                    m.w8(ship + 0x3c, flags | 4);
                    let left = (m.r32(office + ware * 4 + 4) as i32).wrapping_sub(amount);
                    m.w32(office + ware * 4 + 4, left as u32);
                    return;
                }
                let moved = (free / unit).wrapping_mul(unit);
                host.call(m, 0x5180c0, ship, &[ware, moved as u32, price]);
                remaining = remaining.wrapping_sub(moved);
                host.call(m, 0x5182b0, ship, &[]);
                let flags = m.r8(ship + 0x3c);
                m.w8(ship + 0x3c, flags | 4);
                let left = (m.r32(office + ware * 4 + 4) as i32).wrapping_sub(moved);
                m.w32(office + ware * 4 + 4, left as u32);
                amount = remaining;
            }
        }
        if (next as i32) >= count(m) {
            return;
        }
        index = next;
    }
}
/// 0x00638080 (`TutorialWindow`, SEH; marco de pila modelado con `esp` de entrada): construye una cadena temporal («0x6C7AE4»), la pasa al virtual `+0x118`
/// (que desapila 3 palabras: el puntero al temporal y el par de la pila con `+0x2A4 + 1`), copia el resultado en la cadena local, redibuja las
/// ventanas globales (`0x6CBB40`: `0x428C70`, `0x428D40`, `0x42AD80(0)`; `0x441810`, `0x462280(1)`), avisa a `0x4335B0` con `4`, llama al virtual `+0x144`
/// y termina con la cadena `0x6C7B00` y el virtual `+0x128` (dos palabras en la pila). EAX es lo que devuelve el último destructor (`0x64F253`).
/// El valor residual de ECX empujado a `0x4335B0` no está definido en el original (se enmascara a 0).
pub fn tutorial_window_load_next_tutorial(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {
    let e = esp;
    let at = |o: u32| e.wrapping_sub(o);
    let old_fs = m.r32(0);
    m.w32(0, at(0xc));
    m.w32(at(4), 0);
    m.w32(at(8), 0x66_86f2);
    m.w32(at(0xc), old_fs);
    let empty = m.r32(0x6c7ccc);
    m.w32(at(0x18), empty);
    let pair = |m: &mut dyn Mem| {
        let x = m.r32(this.wrapping_add(0x2a4)).wrapping_add(1);
        m.w32(at(0x20), x);
        m.w32(at(0x24), x);
        m.w32(at(0x14), at(0x24));
    };
    pair(m);
    host.call(m, 0x64f2c1, at(0x24), &[0x6c7ae4]);
    let vt = m.r32(this);
    let f = m.r32(vt.wrapping_add(0x118));
    let a1 = m.r32(at(0x24));
    let a2 = m.r32(at(0x20));
    let r = host.call(m, f, this, &[at(0x14), a1, a2]);
    m.w8(at(4), 1);
    host.call(m, 0x64f340, at(0x18), &[r]);
    m.w8(at(4), 0);
    host.call(m, 0x64f253, at(0x14), &[]);
    let t = m.r32(0x6cbb40);
    host.call(m, 0x428c70, t, &[]);
    let t = m.r32(0x6cbb40);
    host.call(m, 0x428d40, t, &[]);
    let t = m.r32(0x6cbb40);
    host.call(m, 0x42ad80, t, &[0]);
    host.call(m, 0x441810, 0, &[]);
    host.call(m, 0x462280, 0, &[1]);
    m.w32(at(0x20), 4);
    m.w32(at(0x10), at(0x24));
    host.call(m, 0x64efc8, at(0x24), &[at(0x18)]);
    let w = m.r32(0x6cbb7c);
    host.call(m, 0x4335b0, w, &[0, 4]);
    let vt = m.r32(this);
    let f = m.r32(vt.wrapping_add(0x144));
    host.call(m, f, this, &[]);
    pair(m);
    m.w32(at(0x10), at(0x24));
    host.call(m, 0x64f2c1, at(0x24), &[0x6c7b00]);
    let vt = m.r32(this);
    let f = m.r32(vt.wrapping_add(0x128));
    let a1 = m.r32(at(0x24));
    let a2 = m.r32(at(0x20));
    host.call(m, f, this, &[a1, a2]);
    m.w32(at(4), 0xffff_ffff);
    let r = host.call(m, 0x64f253, at(0x18), &[]);
    let saved = m.r32(at(0xc));
    m.w32(0, saved);
    r
}

/// 0x005351D0 (ret 4): carga el historial de la ciudad desde `src`: libera los ocho punteros `this+0xA08`, copia `0xA00` bytes sobre `this`, luego 8 bytes de índices a `this+0xA00` y, por cada índice `< 0xFF`, reserva 0x28 bytes, copia el registro y guarda el puntero en `this+0xA08+4k`. Devuelve el final leído.
pub fn town_info_history_load(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let (src, dest) = (a[0], this + 0xa08);
    for k in 0..8u32 {
        let p = m.r32(this + 0xa08 + 4 * k);
        if p != 0 {
            host.call(m, FREE, 0, &[p]);
            m.w32(this + 0xa08 + 4 * k, 0);
        }
        m.w8(this + 0xa00 + k, 0xff);
    }
    copy_bytes(m, this, src, 0xa00);
    let mut s = src + 0xa00;
    copy_bytes(m, this + 0xa00, s, 8);
    s += 8;
    for k in 0..8u32 {
        if m.r8(this + 0xa00 + k) < 0xff {
            let p = host.call(m, ALLOC, 0, &[0x28]);
            copy_bytes(m, p, s, 0x28);
            m.w32(dest + 4 * k, p);
            s += 0x28;
        }
    }
    s
}

/// 0x004FEBD0 (ret 4): inicializa una oficina (`this`, 0x44C bytes): campos `+0x2C4..0x2F2` a su valor por defecto (`0xFFFF`/0), ranuras de 24 palabras a 0, `+0x2E4..` rellenado con 2, y al final `memset(this, 0, 0x2C0)`. El flag `a0` (byte) elige los valores de `+0x2EC/2EE/2F0` (`0x32` o el byte global `0x672A48`).
pub fn office_init(m: &mut dyn Mem, this: u32, a: &[u32]) -> u32 {
    m.w16(this + 0x2c4, 0xffff);
    m.w16(this + 0x2c8, 0xffff);
    m.w8(this + 0x2c6, 0xff);
    m.w8(this + 0x2c7, 0);
    m.w16(this + 0x2ca, 0xffff);
    m.w16(this + 0x2cc, 0xffff);
    m.w16(this + 0x2ce, 0xffff);
    m.w16(this + 0x2d0, 0xffff);
    m.w16(this + 0x2d2, 0);
    m.w16(this + 0x2d4, 0);
    m.w32(this + 0x2e8, 0);
    let mut p = this + 0x2de;
    for _ in 0..3 {
        m.w16(p - 6, 0);
        m.w16(p, 0);
        p += 2;
    }
    for k in 0..4 {
        m.w8(this + 0x2e4 + k, 2);
    }
    m.w16(this + 0x2d6, 0);
    if (a[0] & 0xff) != 0 {
        m.w16(this + 0x2ec, 0);
        m.w16(this + 0x2ee, 0x32);
        let b = u16::from(m.r8(0x672a48));
        m.w16(this + 0x2f0, b);
    } else {
        m.w16(this + 0x2ec, 0x32);
        m.w16(this + 0x2ee, 0);
        m.w16(this + 0x2f0, 0);
    }
    m.w16(this + 0x2f2, 0xffff);
    let mut eax = this + 0x3b4;
    for _ in 0..0x18 {
        m.w32(eax, 0);
        eax = eax.wrapping_sub(4);
        m.w32(eax.wrapping_sub(0x60), 0);
        m.w32(eax, 0);
        m.w32(eax.wrapping_add(0x64), 0);
    }
    for k in 0..12u32 {
        m.w32(this + 0x418 + 4 * k, 0);
    }
    m.w16(this + 0x448, 0);
    m.w16(this + 0x44a, 0);
    for k in 0..0x2c0 {
        m.w8(this + k, 0);
    }
    0
}

/// 0x00511A50: libera un banco de nombres: los 0x28 punteros `this+8..` que no caen dentro de `[this+0, this+4]`, el buffer `this+0` (queda a 0) y los diez punteros `+0xA8..+0xCC` (a 0 tras liberar). Sin retorno útil.
pub fn name_bank_free(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    for k in 0..0x28u32 {
        let p = m.r32(this + 8 + 4 * k);
        if p != 0 && (p < m.r32(this) || p > m.r32(this + 4)) {
            host.call(m, FREE, 0, &[p]);
        }
    }
    let p = m.r32(this);
    if p != 0 {
        host.call(m, FREE, 0, &[p]);
        m.w32(this, 0);
    }
    for off in [0xa8u32, 0xac, 0xb0, 0xb4, 0xbc, 0xc0, 0xc4, 0xc8, 0xb8, 0xcc] {
        let p = m.r32(this + off);
        if p != 0 {
            host.call(m, FREE, 0, &[p]);
            m.w32(this + off, 0);
        }
    }
    0
}

/// 0x0053CE30 (ret 4): despacha la orden de construcción de `this` (registro: `+4..+9` bytes, `+5` ciudad): con `a0 != 0` normaliza el tipo `+8` con la tabla `0x672FC0` (0x2C va directo a `0x51B890`); luego por tipo `<0x30` `0x521900`, `0x30..0x33` `0x523830`, `0x34..0x35` `0x523d70`, `0x36..0x39` `0x522B20(tipo-0x36)`. Fuera de rango o ciudad inexistente: sin retorno.
pub fn op_construct_town_structure(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let idx = u32::from(m.r8(this + 5));
    if idx >= m.r32(0x6de4b0) {
        return 0;
    }
    let town = m.r32(0x6de508).wrapping_add(idx.wrapping_mul(0x9f8));
    let b = |m: &mut dyn Mem, o: u32| u32::from(m.r8(this + o));
    if (a[0] & 0xff) != 0 {
        let bl = b(m, 8);
        let mut edx = 0x36u32;
        if bl == 0x2c {
            let x = b(m, 4);
            return host.call(m, 0x51b890, town, &[x]);
        }
        loop {
            let v = i32::from(m.r8(0x672fbf + edx) as i8) as u32;
            edx = edx.wrapping_sub(1);
            if v == bl {
                m.w8(this + 8, edx as u8);
                if (edx as i32) < 0 {
                    return 0;
                }
                break;
            }
            if (edx as i32) > 0 {
                continue;
            }
            return 0;
        }
    }
    let t = b(m, 8);
    if t < 0x30 {
        let args = [b(m, 4), b(m, 6), b(m, 7), t, b(m, 9)];
        return host.call(m, 0x521900, town, &args);
    }
    if t <= 0x33 {
        let args = [b(m, 6), b(m, 7), b(m, 4)];
        return host.call(m, 0x523830, town, &args);
    }
    if t < 0x36 {
        let args = [b(m, 6), b(m, 7), b(m, 4)];
        return host.call(m, 0x523d70, town, &args);
    }
    if t < 0x3a {
        return host.call(m, 0x522b20, town, &[t - 0x36]);
    }
    0
}

/// 0x005312E0 (ret 0xC): suma por mercancía (24 ranuras) los tres totales del mundo en `a0`, `a1`, `a2` (se ponen a cero 0x60 bytes cada uno): por ciudad `+0x64 + 4k` (`a1 += [+0x2AC]+[+0]`, `a0 += [+0x60]`, `a2 += [+0x42C]`), por cada oficina de la cadena `+0x784`/`+0x2CA` (`a0 += [+0xC4+4k]`, `a1 += [+0x64+4k]`) y por cada nodo de la lista `+0x9B4` (`a0[t] += [nodo+4]`, `a2[t] += 0x50FFB0(nodo)` con `t = byte[0x672C2C + nodo.byte(+0x10)]`).
pub fn sum_world_production_consumption_per_ware(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let (p0, p1, p2) = (a[0], a[1], a[2]);
    for p in [p1, p0, p2] {
        for k in 0..0x60u32 {
            m.w8(p + k, 0);
        }
    }
    let n = m.r32(this + 0x10);
    for i in 0..n {
        let town = m.r32(this + 0x68).wrapping_add(i.wrapping_mul(0x9f8));
        for k in 0..24u32 {
            let c = town + 0x64 + 4 * k;
            let v1 = m.r32(c + 0x2ac).wrapping_add(m.r32(c));
            { let o = p1 + 4 * k; let v = m.r32(o).wrapping_add(v1); m.w32(o, v); }
            { let o = p0 + 4 * k; let v = m.r32(o).wrapping_add(m.r32(c + 0x60)); m.w32(o, v); }
            { let o = p2 + 4 * k; let v = m.r32(o).wrapping_add(m.r32(c + 0x42c)); m.w32(o, v); }
        }
        let mut idx = u32::from(m.r16(town + 0x784));
        if idx < u32::from(m.r16(this + 8)) {
            loop {
                let rec = m.r32(this + 0x74).wrapping_add(idx.wrapping_mul(0x44c));
                for k in 0..24u32 {
                    let c = rec + 0x64 + 4 * k;
                    { let o = p0 + 4 * k; let v = m.r32(o).wrapping_add(m.r32(c + 0x60)); m.w32(o, v); }
                    { let o = p1 + 4 * k; let v = m.r32(o).wrapping_add(m.r32(c)); m.w32(o, v); }
                }
                idx = u32::from(m.r16(rec + 0x2ca));
                if idx >= u32::from(m.r16(this + 8)) {
                    break;
                }
            }
        }
        let mut node = m.r32(town + 0x9b4);
        while node != 0 {
            let ti = u32::from(m.r8(node + 0x10));
            let t = u32::from(m.r8(0x672c2c + ti));
            { let o = p0 + 4 * t; let v = m.r32(o).wrapping_add(m.r32(node + 4)); m.w32(o, v); }
            let r = host.call(m, 0x50ffb0, node, &[]);
            { let o = p2 + 4 * t; let v = m.r32(o).wrapping_add(r); m.w32(o, v); }
            node = m.r32(node);
        }
    }
    0
}

/// 0x0051A9B0 (ret 8): deserializa una carta desde `a0`: bytes `+0/+1`, `+2..4` (2 bytes), `+4`, `+5`, `+6..8` (2 bytes); según el tipo `+4`, `0x29/0x47/0x63/0x83` copia 8 bytes en `+8` y reserva un buffer de `[+0xC]` bytes que rellena; `0x3C..0x40`/`0x71` reserva 0x10 bytes (cabecera, `[0]` = tamaño) y luego su cuerpo; el resto copia 8 bytes a `+8`. Devuelve el puntero tras lo leído.
pub fn letter_deserialize(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut s = a[0];
    let b0 = m.r8(s);
    m.w8(this, b0);
    let b1 = m.r8(s + 1);
    m.w8(this + 1, b1);
    copy_bytes(m, this + 2, s + 2, 2);
    let ty = m.r8(s + 4);
    m.w8(this + 4, ty);
    let b5 = m.r8(s + 5);
    m.w8(this + 5, b5);
    copy_bytes(m, this + 6, s + 6, 2);
    s += 8;
    let ty = m.r8(this + 4);
    if matches!(ty, 0x29 | 0x47 | 0x63 | 0x83) {
        copy_bytes(m, this + 8, s, 8);
        s = s.wrapping_add(8);
        let n = m.r32(this + 0xc);
        let p = host.call(m, ALLOC, 0, &[n]);
        m.w32(this + 8, p);
        if p != 0 {
            let n = m.r32(this + 0xc);
            copy_bytes(m, p, s, n);
        }
        return s.wrapping_add(m.r32(this + 0xc));
    }
    if (0x3c..=0x40).contains(&ty) || ty == 0x71 {
        let p = host.call(m, ALLOC, 0, &[0x10]);
        m.w32(this + 8, p);
        copy_bytes(m, p, s, 0x10);
        s = s.wrapping_add(0x10);
        let n = m.r32(p);
        let q = host.call(m, ALLOC, 0, &[n]);
        m.w32(this + 0xc, q);
        let h = m.r32(this + 8);
        let n = m.r32(h);
        copy_bytes(m, q, s, n);
        let h = m.r32(this + 8);
        return s.wrapping_add(m.r32(h));
    }
    copy_bytes(m, this + 8, s, 8);
    s.wrapping_add(8)
}

/// 0x0052BD80 (ret 4): inicializa un centro comercial a partir de 4 bytes de semilla `a0`: `+4 = b0 % 50`; con `<30` toma `+5/+0/+1` de las tablas `0x673944`/`0x673920` (si no 2/0xFF/0xFF); `+2` y `+3` de la tabla `0x67392C` (3 por grupo) con `b2 % 3` y `b3 & 3` (si coinciden busca la primera distinta `<0x18`); pone `+6 = 0xFF`, `+7 = 0`, `+8 = -1`, `+0xC..+0x1C = 0` y vacía la lista circular `+0x24` (libera nodos, `+0x28` decrece).
pub fn op_init_trade_center_from_seed(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let sd = a[0];
    let dl = u32::from(m.r8(sd)) % 0x32;
    m.w8(this + 4, dl as u8);
    let grp;
    if dl < 0x1e {
        let e = dl * 2;
        let g = u32::from(m.r8(0x673944 + e));
        let v = m.r8(0x673945 + e);
        m.w8(this + 5, v);
        let c = m.r8(g * 2 + 0x673920);
        m.w8(this, c);
        let d = m.r8(g * 2 + 0x673921);
        m.w8(this + 1, d);
        grp = g;
    } else {
        grp = 6;
        m.w8(this + 5, 2);
        m.w8(this, 0xff);
        m.w8(this + 1, 0xff);
    }
    let ecx = grp * 3;
    m.w8(this + 6, 0xff);
    m.w8(this + 7, 0);
    let r = u32::from(m.r8(sd + 2)) % 3;
    let al = m.r8(r + ecx + 0x67392c);
    m.w8(this + 2, al);
    let q = u32::from(m.r8(sd + 3)) & 3;
    let al = m.r8(q + ecx + 0x67392c);
    let dl = m.r8(this + 2);
    m.w8(this + 3, al);
    if al == dl {
        let mut i = 0u32;
        while i < 3 {
            let t = m.r8(ecx + i + 0x67392c);
            if t < 0x18 && t != dl {
                m.w8(this + 3, t);
                i = 4;
            }
            i += 1;
        }
    }
    m.w32(this + 8, 0xffff_ffff);
    for o in [0xcu32, 0x10, 0x14, 0x18] {
        m.w32(this + o, 0);
    }
    m.w16(this + 0x1c, 0);
    let head = m.r32(this + 0x24);
    let mut cur = m.r32(head);
    while cur != head {
        let node = cur;
        cur = m.r32(cur);
        let prev = m.r32(node + 4);
        let next = m.r32(node);
        m.w32(prev, next);
        let next = m.r32(node);
        let prev = m.r32(node + 4);
        m.w32(next + 4, prev);
        host.call(m, FREE, 0, &[node]);
        let c = m.r32(this + 0x28).wrapping_sub(1);
        m.w32(this + 0x28, c);
    }
    0
}

/// 0x00532420 (ret 8): prepara el estado inicial del mundo. `a1` es un contador (`[esp+0x1c]`, reutilizado luego como índice); `a0` no se usa. Fases: (1) recorre las `n = word[this+0xA]` oficinas en orden descendente: limpia el bit 15 de `+8` (activa bit0) y deja `+0x464 = 1.0f`; (2) recorre las oficinas en orden descendente y, para las de flags 0, si existe una oficina previa con el mismo byte `+0x19` y `flags&7 == 1`, marca la oficina 0 (`+0x464 = 2.0f`, `flags |= 8`) mientras quede contador; (3) marca más oficinas con `flags&7 == 1` y `+0x464 == 1.0f` mientras quede contador; (4) recorre la cadena de `0x4D8DB0` (`word+4`, 0xFFFF si `word+6 == 3`) hasta superar `[0x6DD748]`; (5) elige en `+0x8E` la ciudad con `+0x2C8` bit 0x40 sin oficina propia cuyo byte `+0x18+k` esté más cerca de 0xE; (6) `+0x8E = 0` si quedó 0xFF, `+0x8A = 0`, `+0xC = [+0x14]`.
pub fn reset_game_world_start_state(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut cnt = a[1];
    let n = u32::from(m.r16(this + 0xa));
    let off = |i: u32| i.wrapping_mul(0x650);
    // fase 1
    if n > 0 {
        let mut ecx = off(n);
        let mut edi = n;
        loop {
            ecx = ecx.wrapping_sub(0x650);
            let base = m.r32(this + 0x78);
            let addr = base.wrapping_add(ecx).wrapping_add(8);
            let ax = m.r16(addr);
            if (ax >> 8) & 0x80 != 0 {
                m.w16(addr, ax & 0x7fff);
                let b = m.r32(this + 0x78);
                let o = b.wrapping_add(ecx).wrapping_add(8);
                let v = m.r8(o) | 1;
                m.w8(o, v);
            }
            let base = m.r32(this + 0x78);
            edi -= 1;
            m.w32(base.wrapping_add(ecx).wrapping_add(0x464), 0x3f80_0000);
            if edi == 0 {
                break;
            }
        }
    }
    // fase 2
    let mut ebp = u32::from(m.r16(this + 0xa));
    let mut ebx = off(ebp).wrapping_add(0); // se resta 0x650 por iteración
    let mut exit_all = false;
    if ebp > 0 {
        ebx = off(ebp);
        loop {
            let edi = m.r32(this + 0x78);
            ebx = ebx.wrapping_sub(0x650);
            ebp -= 1;
            let cur = ebx;
            if m.r16(cur.wrapping_add(edi).wrapping_add(8)) == 0 {
                let mut eax = u32::from(m.r16(this + 0xa));
                if eax > 0 {
                    let dl = m.r8(cur.wrapping_add(edi).wrapping_add(0x19));
                    let mut ecx = off(eax).wrapping_add(edi).wrapping_add(8).wrapping_sub(0x650);
                    let mut found = false;
                    loop {
                        eax -= 1;
                        if m.r8(ecx.wrapping_add(0x11)) == dl && (m.r8(ecx) & 7) == 1 {
                            found = true;
                            break;
                        }
                        if eax == 0 || (eax as i32) <= 0 {
                            break;
                        }
                        ecx = ecx.wrapping_sub(0x650);
                    }
                    if found {
                        if cnt == 0 {
                            exit_all = true;
                            break;
                        }
                        m.w32(edi.wrapping_add(0x464), 0x4000_0000);
                        let b = m.r32(this + 0x78);
                        let o = b.wrapping_add(8);
                        let v = m.r8(o) | 8;
                        m.w8(o, v);
                        cnt -= 1;
                    }
                }
            }
            if ebp == 0 {
                break;
            }
        }
    }
    let _ = exit_all;
    // fase 3
    if cnt != 0 {
        let mut ebp3 = 0u32;
        let mut edi = 0u32;
        loop {
            if ebp3 as i32 >= i32::from(m.r16(this + 0xa)) {
                break;
            }
            let eax = m.r32(this + 0x78).wrapping_add(edi);
            let cx = u32::from(m.r16(eax + 8));
            if (cx & 7) == 1 && m.r32(eax + 0x464) == 0x3f80_0000 {
                m.w32(eax + 0x464, 0x4000_0000);
                m.w16(eax + 8, ((cx | 8) & 0xffff) as u16);
                cnt -= 1;
            }
            ebp3 += 1;
            edi = edi.wrapping_add(0x650);
            if cnt == 0 {
                break;
            }
        }
    }
    // fase 4
    let mut cur = u32::from(m.r32(0x6dd744) as u16);
    let lim = |m: &mut dyn Mem| u32::from(m.r32(0x6dd748) as u16);
    if (cur as i32) < (lim(m) as i32) {
        loop {
            let p = host.call(m, 0x4d8db0, 0x6dd73c, &[cur]);
            let mut next = u32::from(m.r16(p + 4));
            if m.r16(p + 6) == 3 {
                next = 0xffff;
            }
            cur = next;
            if (cur as i32) >= (lim(m) as i32) {
                break;
            }
        }
    }
    // fase 5
    let towns = m.r32(this + 0x10);
    m.w8(this + 0x8e, 0xff);
    if towns != 0 {
        let mut k = 0u32;
        let mut ptr = m.r32(this + 0x68).wrapping_add(0x2c8);
        loop {
            if (m.r32(ptr) & 0x40) as u8 == 0x40 {
                let edi = u32::from(m.r16(this + 0xa));
                let mut own = false;
                let mut j = 0u32;
                if edi > 0 {
                    loop {
                        let rec = m.r32(this + 0x78).wrapping_add(off(j));
                        if m.r16(rec + 8) == 0 && u32::from(m.r8(rec + 0x19)) == k {
                            own = true;
                            j = 0x3e8;
                        }
                        j += 1;
                        if (j as i32) >= (edi as i32) {
                            break;
                        }
                    }
                }
                if !own {
                    let cl = m.r8(this + 0x8e);
                    let mut take = cl == 0xff;
                    if !take {
                        let al = m.r8(this + 0x18 + k);
                        let d1 = if al > 0xe { u32::from(al) - 0xe } else { 0xe - u32::from(al) };
                        let al2 = m.r8(this + 0x18 + u32::from(cl));
                        let d2 = if al2 > 0xe { u32::from(al2) - 0xe } else { 0xe - u32::from(al2) };
                        take = (d1 as i32) < (d2 as i32);
                    }
                    if take {
                        m.w8(this + 0x8e, k as u8);
                    }
                }
            }
            k += 1;
            ptr = ptr.wrapping_add(0x9f8);
            if k >= m.r32(this + 0x10) {
                break;
            }
        }
    }
    if m.r8(this + 0x8e) == 0xff {
        m.w8(this + 0x8e, 0);
    }
    let v = m.r32(this + 0x14);
    m.w8(this + 0x8a, 0);
    m.w32(this + 0xc, v);
    0
}

/// 0x0053CAF0: compra/venta de un lote de armamento entre una ciudad (`this+0x10 & 0xFF`) y el registro de armamento `0x5308A0`/`0x5303C0` (`this+4`): con `this+0x10 < 0x100` la ciudad entrega al comerciante `this+0xC` unidades (si `+8 < 6` paga con el stock `+0x124+4k` y coste `0x672CB4[k]`; si no, descuenta un lote `+0x2BC`), restando el stock global `[P]` y sumando a `[P+0x4BC]`; con `>= 0x100` es la operación contraria sobre `[P]` y `[P+0x4B8]`. Sin retorno útil.
pub fn op_buy_sell_ship_armament_lot(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let cnt = u32::from(m.r16(0x6de4aa));
    let id = m.r32(this + 4);
    if id >= cnt {
        return 0;
    }
    let idx = m.r32(this + 0x10) & 0xff;
    if idx >= m.r32(0x6de4b0) {
        return 0;
    }
    let town = m.r32(0x6de508).wrapping_add(idx.wrapping_mul(0x9f8));
    let id16 = u32::from(m.r16(this + 4));
    let q = host.call(m, 0x5308a0, 0x6de4a0, &[id16, idx, 0]);
    let p = host.call(m, 0x5303c0, 0x6de4a0, &[id]);
    let n = m.r32(this + 0xc);
    let k = m.r32(this + 8);
    let cost = |m: &mut dyn Mem, k: u32| u32::from(m.r16(0x672cb4 + 2 * k));
    if m.r32(this + 0x10) < 0x100 {
        if (m.r32(p) as i32) < (n as i32) {
            return 0;
        }
        if k >= 6 {
            let c = m.r32(town + 0x2bc);
            if c < 1 {
                return 0;
            }
            m.w32(town + 0x2bc, c - 1);
            let c = m.r32(q + 0x2bc).wrapping_add(1);
            m.w32(q + 0x2bc, c);
        } else {
            let st = m.r32(town + k * 4 + 0x124);
            let d = cost(m, k);
            if (st as i32) < (d as i32) {
                return 0;
            }
            m.w32(town + k * 4 + 0x124, st - d);
            let k2 = m.r32(this + 8);
            let d2 = cost(m, k2);
            let v = m.r32(q + k2 * 4 + 0x124).wrapping_add(d2);
            m.w32(q + k2 * 4 + 0x124, v);
        }
        let nn = m.r32(this + 0xc);
        let v = m.r32(p).wrapping_sub(nn);
        let e = m.r32(p + 0x4bc);
        m.w32(p, v);
        let nn = m.r32(this + 0xc);
        m.w32(p + 0x4bc, e.wrapping_add(nn));
        return 0;
    }
    if k < 6 {
        let st = m.r32(q + k * 4 + 0x124);
        let d = cost(m, k);
        if (st as i32) < (d as i32) {
            return 0;
        }
        m.w32(q + k * 4 + 0x124, st - d);
        let k2 = m.r32(this + 8);
        let d2 = cost(m, k2);
        let v = m.r32(town + k2 * 4 + 0x124).wrapping_add(d2);
        m.w32(town + k2 * 4 + 0x124, v);
    } else {
        let c = m.r32(q + 0x2bc);
        if c < 1 {
            return 0;
        }
        m.w32(q + 0x2bc, c - 1);
        let c = m.r32(town + 0x2bc).wrapping_add(1);
        m.w32(town + 0x2bc, c);
    }
    let nn = m.r32(this + 0xc);
    let v = m.r32(p).wrapping_add(nn);
    let e = m.r32(p + 0x4b8);
    m.w32(p, v);
    let nn = m.r32(this + 0xc);
    m.w32(p + 0x4b8, e.wrapping_add(nn));
    0
}

/// 0x00504750 (ret 8): reinicia la lista de barcos de un comerciante: libera con `0x513760`+FREE los cinco punteros `this+0xC..`, el buffer `this+0`, el array de barcos `this+4` (cada uno con `0x516840(i)`, stride 0x180, `word+0xF4` elementos) y `this+8`; reinicia contadores y ranuras (16 ranuras de 12 bytes hacia atrás desde `+0xEA`: `0xFFFF`, 0, 0, `0xFF`), guarda `a0` en `+0xEE` y `a1 & 0x1FF` normalizado (`> 0x11D` suma `0xFEE3`, `< 0xF` suma `0x96`, 16 bits) en `+0xF0`.
pub fn ships_reset_for_new_game(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    for i in 0..5u32 {
        let p = m.r32(this + 0xc + 4 * i);
        if p != 0 {
            host.call(m, 0x513760, p, &[]);
            host.call(m, FREE, 0, &[p]);
            m.w32(this + 0xc + 4 * i, 0);
        }
    }
    let p = m.r32(this);
    m.w16(this + 0xf8, 0);
    if p != 0 {
        host.call(m, FREE, 0, &[p]);
        m.w32(this, 0);
    }
    let arr = m.r32(this + 4);
    m.w16(this + 0xe0, 0);
    m.w16(this + 0xf2, 0);
    if arr != 0 {
        if m.r16(this + 0xf4) != 0 {
            let mut i = 0u32;
            let mut off = 0u32;
            loop {
                let base = m.r32(this + 4).wrapping_add(off);
                host.call(m, 0x516840, base, &[i]);
                i += 1;
                let n = u32::from(m.r16(this + 0xf4));
                off = off.wrapping_add(0x180);
                if (i as i32) >= (n as i32) {
                    break;
                }
            }
        }
        let p = m.r32(this + 4);
        host.call(m, FREE, 0, &[p]);
    }
    let p8 = m.r32(this + 8);
    m.w32(this + 4, 0);
    m.w16(this + 0xf4, 0);
    m.w16(this + 0xe2, 0);
    if p8 != 0 {
        host.call(m, FREE, 0, &[p8]);
    }
    m.w16(this + 0xee, a[0] as u16);
    let mut c = a[1] & 0x1ff;
    m.w32(this + 8, 0);
    m.w16(this + 0xf6, 0);
    m.w16(this + 0xe4, 0);
    for o in [0xe6u32, 0xe8, 0xea, 0xec] {
        m.w16(this + o, 0xffff);
    }
    if (c as u16) > 0x11d {
        c = c.wrapping_add(0xfee3);
    }
    if (c as u16) < 0xf {
        c = c.wrapping_add(0x96);
    }
    m.w16(this + 0xf0, c as u16);
    let mut e = this + 0xea;
    for _ in 0..0x10 {
        e = e.wrapping_sub(0xc);
        m.w16(e.wrapping_sub(0xa), 0xffff);
        m.w16(e, 0);
        m.w16(e.wrapping_sub(8), 0);
        m.w16(e.wrapping_sub(2), 0xff);
    }
    for o in [0x100u32, 0x102, 0x104, 0x106] {
        m.w16(this + o, 0);
    }
    0
}

/// 0x0053D6D0: intercambia armamento entre una ciudad (`this+0xA`) y el registro `0x5308A0(id=this+8, ciudad)` / `0x5303C0(id)` (`P`): tipo `k = word(this+0xC)`, cantidad `q = word(this+0xE)`, importe `this+4`, sentido `byte(this+0x10)`. Con `k >= 6` mueve lotes `+0x2BC` (con tope 1.000.000.000) y con `k < 6` unidades de stock `+0x124+4k` valoradas con `0x672CB4[k]`; en el sentido de compra resta `this+4` de `[P]` y suma a `[P+0x4A0]`, en el de venta lo suma a `[P]` y a `[P+0x49C]`. Los chequeos de saldo/capacidad abortan sin cambios. Sin retorno útil.
pub fn op_trade_ship_armament_with_office(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let id = u32::from(m.r16(this + 8));
    if id >= u32::from(m.r16(0x6de4aa)) {
        return 0;
    }
    let tw = u32::from(m.r16(this + 0xa));
    if tw >= m.r32(0x6de4b0) {
        return 0;
    }
    let p = host.call(m, 0x5303c0, 0x6de4a0, &[id]);
    let q = host.call(m, 0x5308a0, 0x6de4a0, &[id, tw, 0]);
    if q == 0 {
        return 0;
    }
    let town = m.r32(0x6de508).wrapping_add(tw.wrapping_mul(0x9f8));
    let dir = m.r8(this + 0x10);
    let k = u32::from(m.r16(this + 0xc));
    let qty = u32::from(m.r16(this + 0xe));
    let amount = m.r32(this + 4);
    let sell_tail = |m: &mut dyn Mem, off: u32| {
        // [P] -= amount ; [P+off] += amount  (compra)
        let a = m.r32(this + 4);
        let v = m.r32(p);
        let e = m.r32(p + off);
        m.w32(p, v.wrapping_sub(a));
        let a2 = m.r32(this + 4);
        m.w32(p + off, e.wrapping_add(a2));
    };
    let buy_tail = |m: &mut dyn Mem, off: u32| {
        let a = m.r32(this + 4);
        let v = m.r32(p).wrapping_add(a);
        let e = m.r32(p + off);
        m.w32(p, v);
        let a2 = m.r32(this + 4);
        m.w32(p + off, e.wrapping_add(a2));
    };
    if k >= 6 {
        if dir != 0 {
            let have = m.r32(town + 0x2bc);
            if qty > have {
                return 0;
            }
            let room = 0xffff_ffffu32.wrapping_sub(m.r32(q + 0x2bc));
            if qty > room {
                return 0;
            }
            if (m.r32(p) as i32) < (amount as i32) {
                return 0;
            }
            m.w32(town + 0x2bc, have - qty);
            let v = m.r32(q + 0x2bc).wrapping_add(qty);
            m.w32(q + 0x2bc, v);
            sell_tail(m, 0x4a0);
            return 0;
        }
        let have = m.r32(q + 0x2bc);
        if qty > have {
            return 0;
        }
        let t = m.r32(town + 0x2bc);
        if 0x3b9a_ca00u32.wrapping_sub(qty) >= t {
            m.w32(town + 0x2bc, t.wrapping_add(qty));
        } else {
            m.w32(town + 0x2bc, 0x3b9a_ca00);
        }
        let v = m.r32(q + 0x2bc).wrapping_sub(qty);
        m.w32(q + 0x2bc, v);
        buy_tail(m, 0x49c);
        return 0;
    }
    let c = u32::from(m.r16(0x672cb4 + 2 * k));
    if dir != 0 {
        let t = m.r32(town + k * 4 + 0x124);
        let max = (t as i32).wrapping_div(c as i32);
        if (qty as i32) > max {
            return 0;
        }
        let u = m.r32(q + k * 4 + 0x124);
        let room = 0xffff_ffffu32.wrapping_sub(u.wrapping_mul(c));
        if qty > room {
            return 0;
        }
        if (m.r32(p) as i32) < (amount as i32) {
            return 0;
        }
        let cost = c.wrapping_mul(qty);
        m.w32(town + k * 4 + 0x124, t.wrapping_sub(cost));
        let k2 = u32::from(m.r16(this + 0xc));
        let c2 = u32::from(m.r16(0x672cb4 + 2 * k2));
        let u2 = m.r32(q + k2 * 4 + 0x124).wrapping_add(c2.wrapping_mul(qty));
        m.w32(q + k2 * 4 + 0x124, u2);
        sell_tail(m, 0x4a0);
        return 0;
    }
    let cost = c.wrapping_mul(qty);
    if (cost as i32) > (m.r32(q + k * 4 + 0x124) as i32) {
        return 0;
    }
    let t = m.r32(town + k * 4 + 0x124);
    if (0x3b9a_ca00u32.wrapping_sub(cost) as i32) >= (t as i32) {
        m.w32(town + k * 4 + 0x124, t.wrapping_add(cost));
    } else {
        m.w32(town + k * 4 + 0x124, 0x3b9a_ca00);
    }
    let u = m.r32(q + k * 4 + 0x124).wrapping_sub(cost);
    m.w32(q + k * 4 + 0x124, u);
    buy_tail(m, 0x49c);
    0
}

/// 0x00527AC0 (ret 8): deserializa el bloque económico de la ciudad desde el flujo que da `0x52DF20(a0, a1)` (`a1` = versión): cuatro bytes `+0x2C0..0x2C3` (con `[0x6DE4B8+b0] = b1` y `[0x6DE4E0+b0] = b2`), y bloques `+0x2C4` (2), `+0x2C6` (2), `+0x2C8/2CC/2D0/2D4` (4 c/u), `+0x2D8`/`+0x2E8` (0x10), `+0x2F8/300/308` (8), `+0x310/370/3D0/430/490` (0x60), `+0x4F0` (0x180) y `+0x670` (0x60); según la versión (`<0x5B` salta 0x18 y `<0x65` salta 0x10, si no copia 4 bytes) rellena `+0x6D0`; finalmente llama `0x528070(this, [+0x4F0] > 1)`. Devuelve el puntero tras lo leído.
pub fn town_deserialize_economy_block(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let (a0, a1) = (a[0], a[1]);
    let mut s = host.call(m, 0x52df20, this, &[a0, a1]);
    for k in 0..4u32 {
        let b = m.r8(s);
        m.w8(this + 0x2c0 + k, b);
        s = s.wrapping_add(1);
    }
    let i0 = u32::from(m.r8(this + 0x2c0));
    let v1 = m.r8(this + 0x2c1);
    m.w8(0x6de4b8 + i0, v1);
    let i0 = u32::from(m.r8(this + 0x2c0));
    let v2 = m.r8(this + 0x2c2);
    m.w8(0x6de4e0 + i0, v2);
    for (off, n) in [
        (0x2c4u32, 2u32),
        (0x2c6, 2),
        (0x2c8, 4),
        (0x2cc, 4),
        (0x2d0, 4),
        (0x2d4, 4),
        (0x2d8, 0x10),
        (0x2e8, 0x10),
        (0x2f8, 8),
        (0x300, 8),
        (0x308, 8),
        (0x310, 0x60),
        (0x370, 0x60),
        (0x3d0, 0x60),
        (0x430, 0x60),
        (0x490, 0x60),
        (0x4f0, 0x180),
        (0x670, 0x60),
    ] {
        copy_bytes(m, this + off, s, n);
        s = s.wrapping_add(n);
    }
    if a1 < 0x5b {
        s = s.wrapping_add(0x18);
        m.w32(this + 0x6d0, 0);
    } else if a1 < 0x65 {
        s = s.wrapping_add(0x10);
        m.w32(this + 0x6d0, 0);
    } else {
        copy_bytes(m, this + 0x6d0, s, 4);
        s = s.wrapping_add(4);
    }
    let v = m.r32(this + 0x4f0) as i32;
    host.call(m, 0x528070, this, &[u32::from(v > 1)]);
    s
}

/// 0x00530230 (ret 4): reinicia el mundo para una partida nueva con semilla/modo `a0`: libera los bloques `+0x70`, `+0x74`, destruye con `0x639DA8` (vector con cabecera de contador `p-4`) los arrays de oficinas `+0x78` (0x650, dtor `0x4F3FA0`), ciudades `+0x68` (0x9F8, dtor `0x51B290`) y `[0x6DF270]` (0x1024, dtor `0x633520`), vacía la lista `+0xE4` (`0x533E80`), libera `+0x6C`, pone a cero contadores, guarda `a0` en `+0x14`/`+0xC` y `+0x8A = a0 != 0` (tras `0x5310D0`), limpia `+0x18` y `+0x40` (0x28) y las siete ranuras de `+0x91` (stride 0xC); `+0x8B = 0x5A`.
pub fn game_world_reset_for_new_game(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let a0 = a[0];
    let p = m.r32(this + 0x70);
    if p != 0 {
        host.call(m, FREE, 0, &[p]);
        m.w32(this + 0x70, 0);
    }
    let p = m.r32(this + 0x74);
    m.w16(this + 6, 0);
    m.w16(this + 0x7c, 0);
    if p != 0 {
        host.call(m, FREE, 0, &[p]);
        m.w32(this + 0x74, 0);
    }
    let p = m.r32(this + 0x78);
    m.w16(this + 8, 0);
    m.w16(this + 0x7e, 0);
    if p != 0 {
        let n = m.r32(p.wrapping_sub(4));
        host.call(m, 0x639da8, n, &[p, 0x650, n, 0x4f3fa0]);
        host.call(m, FREE, 0, &[p.wrapping_sub(4)]);
        m.w32(this + 0x78, 0);
    }
    let l = m.r32(this + 0xe4);
    m.w16(this + 0xa, 0);
    m.w16(this + 0x80, 0);
    if l != 0 {
        trade_offer_free_following_nodes(m, host, l);
        m.w32(this + 0xe4, 0);
    }
    let p = m.r32(this + 0x68);
    if p != 0 {
        let n = m.r32(p.wrapping_sub(4));
        host.call(m, 0x639da8, 0, &[p, 0x9f8, n, 0x51b290]);
        host.call(m, FREE, 0, &[p.wrapping_sub(4)]);
        m.w32(this + 0x68, 0);
    }
    let p = m.r32(this + 0x6c);
    m.w32(this + 0x10, 0);
    if p != 0 {
        host.call(m, FREE, 0, &[p]);
        m.w32(this + 0x6c, 0);
    }
    let p = m.r32(0x6df270);
    if p != 0 {
        let n = m.r32(p.wrapping_sub(4));
        host.call(m, 0x639da8, n, &[p, 0x1024, n, 0x633520]);
        host.call(m, FREE, 0, &[p.wrapping_sub(4)]);
        m.w32(0x6df270, 0);
    }
    m.w32(this + 0x14, a0);
    host.call(m, 0x5310d0, this, &[]);
    m.w32(this + 0xc, a0);
    m.w8(this + 0x8a, u8::from(a0 != 0));
    for k in 0..0x28u32 {
        m.w8(this + 0x18 + k, 0);
        m.w8(this + 0x40 + k, 0);
    }
    m.w8(this + 0x8d, 0xff);
    m.w8(this + 0x8e, 0);
    let mut e = this + 0x91;
    for _ in 0..7 {
        m.w8(e - 1, 0xff);
        m.w8(e, 0xff);
        m.w32(e + 7, 0);
        m.w8(e + 2, 0);
        m.w32(e + 3, 0);
        e += 0xc;
    }
    m.w8(this + 0x89, 0);
    m.w8(this + 0x8c, 0);
    m.w8(this + 0x8b, 0x5a);
    m.w8(this + 0x8f, 0xff);
    0
}

/// 0x005434E0: prepara el estado de una partida nueva con el registro `this` (`+4` semilla, `+8`/`+0xA` palabras, `+0xC` palabra, `+0x10` byte): vacía las listas globales (`0x4D64B0`, `0x4D4C60`, `0x54DDF0`), reinicia los barcos (`0x504750(+8, +0xA)`), normaliza la semilla (`< 0x474A00` -> `(s & 0xFF) + 0x474A00`) y la usa en `0x530230` (mundo), `0x5352B0`, `0x4D8B10(semilla, +0xC)` y `0x52E1A0(+0x10)`. Sin retorno.
pub fn op_reset_game_state_for_new_game(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    host.call(m, 0x4d64b0, 0x6dd730, &[]);
    host.call(m, 0x4d4c60, 0x6dd728, &[]);
    host.call(m, 0x54ddf0, 0x6dfc90, &[]);
    let a0 = u32::from(m.r16(this + 8));
    let a1 = u32::from(m.r16(this + 0xa));
    host.call(m, 0x504750, 0x6dd7a0, &[a0, a1]);
    let mut seed = m.r32(this + 4);
    if (seed as i32) < 0x47_4a00 {
        seed = (seed & 0xff) + 0x47_4a00;
        m.w32(this + 4, seed);
    }
    let seed = m.r32(this + 4);
    host.call(m, 0x530230, 0x6de4a0, &[seed]);
    host.call(m, 0x5352b0, 0x6de588, &[]);
    let seed = m.r32(this + 4);
    let c = u32::from(m.r16(this + 0xc));
    host.call(m, 0x4d8b10, 0x6dd73c, &[seed, c]);
    let b = u32::from(m.r8(this + 0x10));
    host.call(m, 0x52e1a0, 0x6de3d8, &[b]);
    0
}

/// 0x00542980: crea el comerciante IA inicial de una ciudad (`this+4`): exige `[town+0x784] >= word[0x6DE4A8]`, reserva un mapa de `[0x6DDB74]` bytes a cero, marca en él el byte `+0x1D` de cada registro de `0x5303C0(i)` (`i` desde `word[0x6DE4AA]-1` a 0) y llama a `0x542560(+4, +0xC, +0x10, mapa)`; después libera el mapa. Sin retorno.
pub fn op_create_initial_ai_merchant_in_town(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let count = m.r32(0x6de4b0);
    let size = m.r32(0x6ddb74) & 0xffff;
    let tw = u32::from(m.r16(this + 4));
    let mut n = u32::from(m.r16(0x6de4aa));
    if tw >= count {
        return 0;
    }
    let town = m.r32(0x6de508).wrapping_add(tw.wrapping_mul(0x9f8));
    if m.r16(town + 0x784) < m.r16(0x6de4a8) {
        return 0;
    }
    let p = host.call(m, ALLOC, 0, &[size]);
    if p == 0 {
        return 0;
    }
    for k in 0..size {
        m.w8(p + k, 0);
    }
    while (n as i32) > 0 {
        n -= 1;
        let r = host.call(m, 0x5303c0, 0x6de4a0, &[n]);
        let b = u32::from(m.r8(r + 0x1d));
        if b < size {
            m.w8(p + b, 1);
        }
    }
    let w = u32::from(m.r16(this + 4));
    host.call(m, 0x542560, this, &[w, this + 0xc, this + 0x10, p]);
    host.call(m, FREE, 0, &[p]);
    0
}

/// 0x0050C9E0 (ret 8): disuelve el convoy `a0` (registro `this+8 + a0*0x3C`) tras un combate naval; `a1` (byte) indica si hubo captura. Si el registro `0x4D8DB0` del convoy es de tipo 0x24 y `a1 != 0` marca `+0x15 = 5`, `+0x16 = 0xFFFF` y limpia los bits 9/10 de `[town+0x2C8]`; después recorre la lista de barcos (`+0xA`, stride 0x180, enlace `+6`): los de `+0x18 <= 0` pasan a la lista `+0xEA` (`+0x134 = 0x11`), los demás a `+0xE8` (`+0x134 = 3`) o, si hay registro de convoy, se avisan con `0x6132C0` (`+0x134 = 9`, `+0x138 = 0x20`); por último libera el convoy a la lista `+0xE4` (0xFF) o `+0xEC`.
pub fn dissolve_convoy_after_sea_battle(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let a0 = a[0];
    let flag = (a[1] & 0xff) as u8;
    let conv = m.r32(this + 8).wrapping_add(a0.wrapping_mul(0x3c));
    let mut ebp = 0u32;
    let ax = u32::from(m.r16(conv + 2));
    let lim = m.r16(0x6dd748);
    if (ax as u16) < lim {
        let rec = host.call(m, 0x4d8db0, 0x6dd73c, &[ax]);
        ebp = rec;
        if m.r16(rec + 6) == 0x24 {
            if flag == 0 {
                // salta a 0x50CA60 con ebp = rec
                return dissolve_tail(m, host, this, a0, conv, ebp);
            }
            m.w8(rec + 0x15, 5);
            let t = u32::from(m.r8(rec + 0x14));
            m.w16(rec + 0x16, 0xffff);
            if t < m.r32(0x6de4b0) {
                let addr = m.r32(0x6de508).wrapping_add(t.wrapping_mul(0x9f8)).wrapping_add(0x2c8);
                let v = m.r32(addr) & 0xffff_f9ff;
                m.w32(addr, v);
            }
        }
        ebp = 0;
    }
    dissolve_tail(m, host, this, a0, conv, ebp)
}

fn dissolve_tail(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a0: u32, conv: u32, ebp: u32) -> u32 {
    let mut eax = conv + 0xa;
    let mut dx = m.r16(eax);
    if dx < m.r16(this + 0xf4) {
        loop {
            let esi = m.r32(this + 4).wrapping_add(u32::from(dx).wrapping_mul(0x180));
            let c = m.r32(esi + 0x18) as i32;
            if c <= 0 {
                let d = m.r16(esi + 6);
                m.w16(esi + 8, 0xffff);
                let cx = m.r16(eax);
                m.w16(eax, d);
                m.w16(esi + 0x134, 0x11);
                let d2 = m.r16(this + 0xea);
                m.w16(esi + 6, d2);
                m.w16(this + 0xea, cx);
            } else if ebp == 0 {
                let d = m.r16(esi + 6);
                m.w16(esi + 8, 0xffff);
                let cx = m.r16(eax);
                m.w16(eax, d);
                m.w16(esi + 0x134, 3);
                let d2 = m.r16(this + 0xe8);
                m.w16(esi + 6, d2);
                m.w16(this + 0xe8, cx);
                m.w16(esi + 0x138, 0);
            } else {
                host.call(m, 0x6132c0, 0x6e6220, &[u32::from(dx)]);
                m.w16(esi + 0x134, 9);
                m.w16(esi + 0x138, 0x20);
                eax = esi + 6;
            }
            dx = m.r16(eax);
            if dx >= m.r16(this + 0xf4) {
                break;
            }
        }
    }
    let si = a0 as u16;
    if m.r16(conv + 0xa) >= m.r16(this + 0xf4) {
        for k in 0..0x3cu32 {
            m.w8(conv + k, 0xff);
        }
        let cx = m.r16(this + 0xe4);
        m.w16(conv + 8, cx);
        m.w16(this + 0xe4, si);
    } else {
        let d = m.r16(this + 0xec);
        m.w16(conv + 0x12, 9);
        m.w16(conv + 8, d);
        m.w16(this + 0xec, si);
    }
    0
}

/// 0x00517990 (ret 8): carga un barco desde `a0` con versión `a1`: lee `+0x134` (2 bytes, corregido a +1 en versiones `< 0x70`/`< 0x71` cuando está entre 10 y 0xFE); si `+0x134 >= 0x17` solo lee `+6` (y `+0x137` desde la 0x83) y deja el barco vacío (`+0 = 0xFF`, `+4/+0xA/+0xC = 0xFFFF`, `+0x24 = 0`, `+0x36 = 0`); si no delega la cabecera en `0x516520(src, versión)` y copia `+0x54/+0xB4` (0x60), `+0x114/124/128/12C`, `+0x130/132`, bytes `+0x136/137/13A/13B`, `+0x138`, `+0x13C` (0x18), `+0x154`, `+0x156/157/158/159/15A` (según versión `0x79`), `+0x15C` (bool), `+0x15D`, `+0x15E` (big endian de 2 bytes) y `+0x160` (0x20); termina con `0x5182B0`. Devuelve el puntero tras lo leído.
pub fn ship_load(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut s = a[0];
    let ver = a[1] & 0xffff;
    copy_bytes(m, this + 0x134, s, 2);
    s = s.wrapping_add(2);
    if ver < 0x70 {
        let ax = m.r16(this + 0x134);
        if ax > 9 && ax < 0xff {
            m.w16(this + 0x134, ax + 1);
        }
    }
    if ver < 0x71 {
        let ax = m.r16(this + 0x134);
        if ax > 0xa && ax < 0xff {
            m.w16(this + 0x134, ax + 1);
        }
    }
    if m.r16(this + 0x134) >= 0x17 {
        copy_bytes(m, this + 6, s, 2);
        s = s.wrapping_add(2);
        if ver < 0x83 {
            m.w8(this + 0x137, 0);
        } else {
            let b = m.r8(s);
            s = s.wrapping_add(1);
            m.w8(this + 0x137, b);
        }
        m.w8(this, 0xff);
        m.w16(this + 4, 0xffff);
        m.w32(this + 0x24, 0);
        m.w8(this + 0x36, 0);
        m.w16(this + 0xc, 0xffff);
        m.w16(this + 0xa, 0xffff);
        return s;
    }
    s = host.call(m, 0x516520, this, &[s, ver]);
    for (off, n) in [(0x54u32, 0x60u32), (0xb4, 0x60), (0x114, 4), (0x124, 4), (0x128, 4), (0x12c, 4), (0x130, 2), (0x132, 2)] {
        copy_bytes(m, this + off, s, n);
        s = s.wrapping_add(n);
    }
    let dl = m.r8(s);
    s = s.wrapping_add(1);
    m.w8(this + 0x136, dl);
    let al = m.r8(s);
    s = s.wrapping_add(1);
    m.w8(this + 0x137, al);
    copy_bytes(m, this + 0x13a, s, 1);
    s = s.wrapping_add(1);
    copy_bytes(m, this + 0x13b, s, 1);
    s = s.wrapping_add(1);
    copy_bytes(m, this + 0x138, s, 2);
    s = s.wrapping_add(2);
    copy_bytes(m, this + 0x13c, s, 0x18);
    s = s.wrapping_add(0x18);
    copy_bytes(m, this + 0x154, s, 2);
    let al = m.r8(s.wrapping_add(2));
    s = s.wrapping_add(3);
    m.w8(this + 0x156, al);
    let cl = m.r8(s);
    s = s.wrapping_add(1);
    m.w8(this + 0x157, cl);
    if ver < 0x79 {
        m.w8(this + 0x158, 0);
        m.w8(this + 0x159, 0xff);
        m.w16(this + 0x15a, 0);
    } else {
        let d = m.r8(s);
        s = s.wrapping_add(1);
        m.w8(this + 0x158, d);
        let al = m.r8(s);
        s = s.wrapping_add(1);
        m.w8(this + 0x159, al);
        copy_bytes(m, this + 0x15a, s, 2);
        s = s.wrapping_add(2);
    }
    let d = m.r8(s);
    s = s.wrapping_add(1);
    m.w8(this + 0x15c, u8::from(d != 0));
    let al = m.r8(s);
    s = s.wrapping_add(1);
    m.w8(this + 0x15d, al);
    let cx = u16::from(m.r8(s));
    s = s.wrapping_add(1);
    let dx = u16::from(m.r8(s));
    s = s.wrapping_add(1);
    m.w16(this + 0x15e, (cx << 8).wrapping_add(dx));
    copy_bytes(m, this + 0x160, s, 0x20);
    host.call(m, 0x5182b0, this, &[]);
    s.wrapping_add(0x20)
}

/// 0x00509D10 (ret 8): une el barco `a1` (de la lista libre `+0xE8`) al convoy de `a0` (convoy `this+8 + word(ship0+8)*0x3C`), tras comprobar flags del convoy (`+0x14`: bit 2 exige mismo byte `+0`, si no capacidad por `0x673660` y carga `+0x18*5 >= +0x14`; bit 0x10 impide; bit 4 exige `ship+0xE < 2`), la clase `+0x39 == word(+0x1C)`, la ciudad (`+0x2C8` bit 0), el estado `+0x134` (0 o 1 con `+0x138 <= +0x16`) y, si `+0x12 == 9`, `0x5039F0(a1)`. Si todo pasa lo saca de la lista libre, lo enlaza en `+0xA`, avisa a `0x516840`, `0x44F940`, `0x50D040`, `0x501BE0` y, para convoyes 9/0xA, libera su hueco en la tabla `0x4D8DB0(+2)+8..` y llama `0x4F8D20`. Sin retorno.
pub fn op_join_ship_to_convoy(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let (a0, a1) = (a[0], a[1]);
    let cnt = u32::from(m.r16(this + 0xf4));
    if a0 >= cnt || a1 >= cnt {
        return 0;
    }
    let base = m.r32(this + 4);
    let dx = m.r16(0x6dd896);
    let ship0 = base.wrapping_add(a0.wrapping_mul(0x180));
    let esi = base.wrapping_add(a1.wrapping_mul(0x180));
    let ax = m.r16(ship0 + 8);
    if ax >= dx {
        return 0;
    }
    let edi = m.r32(this + 8).wrapping_add(u32::from(ax).wrapping_mul(0x3c));
    let flags = m.r16(edi + 0x14);
    if flags & 2 == 2 {
        if m.r8(edi) != m.r8(esi) {
            return 0;
        }
    } else {
        let idx = u32::from(m.r8(esi + 0xe)) & 3;
        let cx = u16::from(m.r8(0x673660 + idx));
        if m.r16(esi + 0x40) < cx {
            return 0;
        }
        let ecx = m.r32(esi + 0x18).wrapping_mul(5);
        if (ecx as i32) < (m.r32(esi + 0x14) as i32) {
            return 0;
        }
    }
    if m.r16(esi + 8) < dx {
        return 0;
    }
    if flags & 0x10 == 0x10 {
        return 0;
    }
    if flags & 4 == 4 && m.r8(esi + 0xe) >= 2 {
        return 0;
    }
    if u16::from(m.r8(esi + 0x39)) != m.r16(edi + 0x1c) {
        return 0;
    }
    if m.r8(esi + 0xe) & 2 != 0 {
        let t = u32::from(m.r8(edi + 0x1f));
        if t < m.r32(0x6de4b0) {
            let town = m.r32(0x6de508).wrapping_add(t.wrapping_mul(0x9f8));
            if m.r32(town + 0x2c8) & 1 != 0 {
                return 0;
            }
        }
    }
    let st = m.r16(esi + 0x134);
    if st != 0 {
        if st != 1 {
            return 0;
        }
        if m.r16(esi + 0x138) > m.r16(edi + 0x16) {
            return 0;
        }
    }
    if m.r16(edi + 0x12) == 9 {
        let r = host.call(m, 0x5039f0, edi, &[a1]);
        if r & 0xff == 0 {
            return 0;
        }
    }
    let mut dxw = m.r16(this + 0xe8);
    let mut link = this + 0xe8;
    let mut eax = u32::from(dxw);
    while eax != a1 {
        if dxw >= m.r16(this + 0xf4) {
            return 0;
        }
        let b = m.r32(this + 4);
        link = eax.wrapping_mul(0x180).wrapping_add(b).wrapping_add(6);
        dxw = m.r16(link);
        eax = u32::from(dxw);
    }
    let nx = m.r16(esi + 6);
    m.w16(link, nx);
    let cx = m.r16(edi + 0xa);
    m.w16(esi + 6, cx);
    m.w16(edi + 0xa, a1 as u16);
    let ax = m.r16(ship0 + 8);
    let dl = m.r8(esi + 0x136);
    let v = m.r8(esi + 0x3c) | 0x60;
    m.w8(esi + 0x3c, v);
    m.w16(esi + 8, ax);
    m.w8(esi + 0x136, dl & 0xfc);
    host.call(m, 0x516840, esi, &[a1]);
    let g = m.r32(0x6dfc14);
    if u32::from(m.r8(esi)) == g {
        let c = m.r32(0x6cbdd8);
        host.call(m, 0x44f940, c, &[a1]);
    }
    host.call(m, 0x50d040, this, &[a1]);
    host.call(m, 0x501be0, edi, &[]);
    let t = m.r16(edi + 0x12);
    m.w32(edi + 0x18, 0xffff_ffff);
    if t == 9 || t == 0xa {
        let w = u32::from(m.r16(edi + 2));
        let rec = host.call(m, 0x4d8db0, 0x6dd73c, &[w]);
        let s = u32::from(m.r8(esi));
        m.w16(esi + 0x138, 0);
        let p = host.call(m, 0x5303c0, 0x6de4a0, &[s]);
        host.call(m, 0x4f8d20, p, &[]);
        let mut k = 0u32;
        loop {
            let d = m.r8(rec + k + 8);
            let b = m.r8(esi);
            if d == b {
                m.w8(rec + k + 8, 0xff);
                k = 8;
            }
            k += 1;
            if (k as i32) >= 8 {
                break;
            }
        }
    }
    0
}

/// 0x00501EB0: inicia la salida del convoy `this` (`+0x10` barco jefe): si el barco no está listo (`0x519C40`/`0x519BA0`) lo marca (`+0x18 |= 0x20`, estado 0) y devuelve 0; con la ciudad en bloqueo (`[+0x2C8] & 0x4000200`) marca estado 1/`0x100`; si no recorre la cadena de barcos (`+0xA`, enlace `+6`): los no listos pasan a `0x509F80`, los listos suman su carga (`+0x54`, 24 palabras) a la ciudad de origen/destino (`+0x670`/`+0x370`) y quedan en estado 2; al final fija `+0x1E`, llama `0x501BE0`, `+0x12 = 2`, `+0x16 = 1` y devuelve 1.
pub fn convoy_begin_departure(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let ax = u32::from(m.r16(this + 0x10));
    if (ax as u16) >= m.r16(0x6dd894) {
        m.w16(this + 0x12, 0);
        return 0;
    }
    let ship_at = |m: &mut dyn Mem, i: u32| m.r32(0x6dd7a4).wrapping_add(i.wrapping_mul(0x180));
    let esi0 = ship_at(m, ax);
    let fail = |m: &mut dyn Mem| -> u32 {
        let v = m.r32(this + 0x18) | 0x20;
        m.w16(this + 0x12, 0);
        m.w32(this + 0x18, v);
        let d = m.r8(esi0 + 0x39);
        m.w8(esi0 + 0x38, d);
        m.w16(esi0 + 0x134, 0);
        m.w16(esi0 + 0x3c, 0xa0);
        0
    };
    if host.call(m, 0x519c40, esi0, &[]) & 0xff == 0 {
        return fail(m);
    }
    if host.call(m, 0x519ba0, esi0, &[]) & 0xff == 0 {
        return fail(m);
    }
    let t0 = u32::from(m.r16(this + 0x1c));
    if t0 < m.r32(0x6de4b0) {
        let town = m.r32(0x6de508).wrapping_add(t0.wrapping_mul(0x9f8));
        if m.r32(town + 0x2c8) & 0x0400_0200 != 0 {
            let v = m.r32(this + 0x18) | 0x20;
            m.w32(this + 0x18, v);
            let d = m.r8(this + 0x1c);
            m.w16(this + 0x12, 1);
            m.w16(this + 0x16, 0x100);
            m.w8(esi0 + 0x38, d);
            m.w16(esi0 + 0x134, 1);
            m.w16(esi0 + 0x3c, 0xa0);
            m.w16(esi0 + 0x138, 0x100);
            let w = u32::from(m.r16(this + 0x10));
            host.call(m, 0x516840, esi0, &[w]);
            return 0;
        }
    }
    host.call(m, 0x516bc0, esi0, &[]);
    let g = m.r32(0x6dfc14);
    if u32::from(m.r8(this)) == g {
        let c = m.r32(0x6cbdd8);
        let w = u32::from(m.r16(this + 0x10));
        host.call(m, 0x44f940, c, &[w]);
    }
    let f = m.r8(this + 0x14) & 0xef;
    m.w8(this + 0x14, f);
    let lim = m.r32(0x6dd894) & 0xffff;
    let mut ebx = u32::from(m.r16(this + 0xa));
    if ebx < lim {
        loop {
            let esi = ship_at(m, ebx);
            if host.call(m, 0x519ba0, esi, &[]) & 0xff == 0 {
                let d = m.r8(esi + 0x39);
                let f = m.r8(esi + 0x3c) | 0x40;
                m.w8(esi + 0x38, d);
                m.w8(esi + 0x3c, f);
                let v = m.r32(this + 0x18) | 0x4020;
                let nx = u32::from(m.r16(esi + 6));
                m.w32(this + 0x18, v);
                host.call(m, 0x509f80, 0x6dd7a0, &[ebx]);
                m.w16(esi + 0x134, 0);
                ebx = nx;
            } else {
                let t = u32::from(m.r8(this + 0x1f));
                if t < m.r32(0x6de4b0) {
                    let town = m.r32(0x6de508).wrapping_add(t.wrapping_mul(0x9f8)).wrapping_add(0x670);
                    for k in 0..24u32 {
                        let v = m.r32(town + 4 * k).wrapping_add(m.r32(esi + 0x54 + 4 * k));
                        m.w32(town + 4 * k, v);
                    }
                }
                let t2 = u32::from(m.r16(this + 0x1c));
                let town2 = m.r32(0x6de508).wrapping_add(t2.wrapping_mul(0x9f8)).wrapping_add(0x370);
                for k in 0..24u32 {
                    let v = m.r32(town2 + 4 * k).wrapping_add(m.r32(esi + 0x54 + 4 * k));
                    m.w32(town2 + 4 * k, v);
                }
                if m.r8(esi) != m.r8(this) {
                    let f = m.r8(this + 0x14) & 0xfd;
                    m.w8(this + 0x14, f);
                }
                m.w16(esi + 0x134, 2);
                let cl = m.r8(this + 0x1c);
                m.w8(esi + 0x37, cl);
                let dl = m.r8(this + 0x1f);
                let f = m.r8(esi + 0x3c) | 0x20;
                m.w8(esi + 0x3c, f);
                m.w8(esi + 0x38, dl);
                let ecx = m.r32(esi0 + 0x48);
                ebx = u32::from(m.r16(esi + 6));
                m.w32(esi + 0x48, ecx);
            }
            let lim = m.r32(0x6dd894) & 0xffff;
            if ebx >= lim {
                break;
            }
        }
    }
    let cl = m.r8(this + 0x1c);
    m.w8(this + 0x1e, cl);
    host.call(m, 0x501be0, this, &[]);
    m.w16(this + 0x12, 2);
    m.w16(this + 0x16, 1);
    1
}

/// 0x004D98F0 (ret 4): operación de participaciones de compañía con la petición `a0` (bytes `+0` origen, `+1` cantidad, `+2` y `+3` ciudades/bancos, dword `+4` importe) sobre la lista de registros `this` (`[this]` base, stride 0x18, `word+8` cabeza, `word+0xC` tamaño; cada registro: tipo `+6 == 0x18`, enlace `+4`, `+0x10/0x11` claves, `+0x12/0x13` cantidades). Con `+3 < word[0x6DE4AA]` (y `+2 >=`) retira acciones de una cuenta `0xFF` hacia el titular, en otro caso agrupa/crea registros (`0x4D8CF0`) y mueve el importe entre los registros de ciudad `0x5303C0` (`+0`, `+0x4B8`, `+0x4BC`, `+0x46C`). Sin retorno útil.
pub fn op_trade_company_shares(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let edi = a[0];
    let cnt = u32::from(m.r16(0x6de4aa));
    let b3 = u32::from(m.r8(edi + 3));
    let mut esi = u32::from(m.r16(this + 8));
    // acceso a registro idx
    let rec = |m: &mut dyn Mem, idx: u32| m.r32(this).wrapping_add(idx.wrapping_mul(0x18));
    if b3 < cnt {
        // camino A
        if u32::from(m.r8(edi + 2)) < cnt {
            return 0;
        }
        let ebp = host.call(m, 0x5303c0, 0x6de4a0, &[b3]);
        let amount = m.r32(edi + 4);
        if m.r32(ebp) < amount {
            return 0;
        }
        let size = u32::from(m.r16(this + 0xc));
        let found;
        loop {
            if esi >= size {
                return 0;
            }
            let r = rec(m, esi);
            if m.r16(r + 6) == 0x18 && m.r8(r + 0x10) == m.r8(edi) && m.r8(r + 0x11) == 0xff {
                found = r;
                break;
            }
            esi = u32::from(m.r16(r + 4));
        }
        let ecx = found;
        let dl = m.r8(ecx + 0x12);
        if dl == 0 {
            return 0;
        }
        let al = m.r8(edi + 1);
        if dl > al {
            m.w8(ecx + 0x12, dl - al);
        } else {
            m.w8(ecx + 0x12, 0);
        }
        let dl = m.r8(ecx + 0x13);
        let al = m.r8(edi + 1);
        if dl >= al {
            m.w8(ecx + 0x13, dl - al);
        } else {
            m.w8(ecx + 0x13, 0);
        }
        let g = m.r32(0x6de4b4);
        let owner = m.r32(ecx + 0xc);
        m.w32(ecx + 8, g);
        let amount = m.r32(edi + 4);
        let v = m.r32(ebp).wrapping_sub(amount);
        let e = m.r32(ebp + 0x4bc);
        m.w32(ebp, v);
        let amount2 = m.r32(edi + 4);
        m.w32(ebp + 0x4bc, e.wrapping_add(amount2));
        let bl = m.r8(edi + 3);
        if bl == m.r8(edi) {
            return 0;
        }
        let head = u32::from(m.r16(this + 8));
        let size = u32::from(m.r16(this + 0xc));
        let mut idx = head;
        while idx < size {
            let r = rec(m, idx);
            if m.r16(r + 6) == 0x18 && m.r8(r + 0x10) == m.r8(edi) && m.r8(r + 0x11) == bl {
                let dl = m.r8(edi + 1);
                let al = m.r8(r + 0x12).wrapping_add(dl);
                m.w8(r + 0x12, al);
                m.w8(r + 0x13, 0);
                return 0;
            }
            idx = u32::from(m.r16(r + 4));
        }
        let g = m.r32(0x6de4b4).wrapping_add(1);
        let n = host.call(m, 0x4d8cf0, this, &[g]);
        m.w16(n + 6, 0x18);
        m.w32(n + 8, 0);
        m.w32(n + 0xc, owner);
        let dl = m.r8(edi);
        m.w8(n + 0x10, dl);
        let cl = m.r8(edi + 3);
        m.w8(n + 0x11, cl);
        let dl = m.r8(edi + 1);
        m.w8(n + 0x12, dl);
        m.w8(n + 0x13, 0);
        return 0;
    }
    // camino B
    let b2 = u32::from(m.r8(edi + 2));
    if b2 >= cnt {
        return 0;
    }
    let edx = host.call(m, 0x5303c0, 0x6de4a0, &[b2]);
    let cl = m.r8(edi);
    let size = u32::from(m.r16(this + 0xc));
    let al = m.r8(edi + 2);
    let ebp: u32;
    if al == cl {
        let mut sum = 0u32;
        while esi < size {
            let r = rec(m, esi);
            if m.r16(r + 6) == 0x18 && m.r8(r + 0x10) == m.r8(edi) {
                sum = sum.wrapping_add(u32::from(m.r8(r + 0x12)));
            }
            esi = u32::from(m.r16(r + 4));
        }
        if esi >= size && sum >= 0x2d && false {
            return 0;
        }
        // 0x4D9B89: tras el bucle exige suma < 0x2D
        if sum >= 0x2d {
            return 0;
        }
        let q = u32::from(m.r8(edi + 1)).wrapping_add(sum);
        if q > 0x2d {
            m.w8(edi + 1, (0x2du8).wrapping_sub(sum as u8));
        }
        let c = m.r32(edx + 0x46c) as i32;
        ebp = if c <= 0 { 0 } else { c as u32 };
    } else {
        let mut idx = esi;
        let found;
        loop {
            if idx >= size {
                return 0;
            }
            let r = rec(m, idx);
            if m.r16(r + 6) == 0x18 && m.r8(r + 0x11) == al && m.r8(r + 0x10) == m.r8(edi) {
                found = r;
                break;
            }
            idx = u32::from(m.r16(r + 4));
        }
        let ecx = found;
        let dl = m.r8(ecx + 0x12);
        if dl == 0 {
            return 0;
        }
        let a1 = m.r8(edi + 1);
        if dl < a1 {
            m.w8(edi + 1, dl);
            m.w8(ecx + 0x12, 0);
        } else {
            m.w8(ecx + 0x12, dl - a1);
        }
        let dl = m.r8(ecx + 0x13);
        let a1 = m.r8(edi + 1);
        ebp = m.r32(ecx + 0xc);
        if dl > a1 {
            m.w8(ecx + 0x13, dl - a1);
        } else {
            m.w8(ecx + 0x13, 0);
        }
        let g = m.r32(0x6de4b4);
        m.w32(ecx + 8, g);
    }
    let amount = m.r32(edi + 4);
    let v = m.r32(edx).wrapping_add(amount);
    let e = m.r32(edx + 0x4b8);
    m.w32(edx, v);
    let amount2 = m.r32(edi + 4);
    m.w32(edx + 0x4b8, e.wrapping_add(amount2));
    let head = u32::from(m.r16(this + 8));
    let size = u32::from(m.r16(this + 0xc));
    let mut idx = head;
    while idx < size {
        let r = rec(m, idx);
        if m.r16(r + 6) == 0x18 && m.r8(r + 0x10) == m.r8(edi) && m.r8(r + 0x11) == 0xff {
            let dl = m.r8(edi + 1);
            let al = m.r8(r + 0x12).wrapping_add(dl);
            m.w8(r + 0x12, al);
            return 0;
        }
        idx = u32::from(m.r16(r + 4));
    }
    let g = m.r32(0x6de4b4).wrapping_add(1);
    let n = host.call(m, 0x4d8cf0, 0x6dd73c, &[g]);
    m.w16(n + 6, 0x18);
    let g2 = m.r32(0x6de4b4);
    m.w32(n + 8, g2);
    m.w32(n + 0xc, ebp);
    let dl = m.r8(edi);
    m.w8(n + 0x11, 0xff);
    m.w8(n + 0x10, dl);
    let cl = m.r8(edi + 1);
    m.w8(n + 0x12, cl);
    m.w8(n + 0x13, 0);
    0
}

/// 0x00512D40 (ret 8): busca el nombre `a0` en el banco de nombres `this` (dos tablas de offsets de 16 bits: `[+0xC8]` sobre `[+0xB4]` con `word+0xD8` entradas y `[+0xCC]` sobre `[+0xB8]` con `word+0xDA - word+0xD8` entradas) y guarda su índice en `word[a1]` (`+0xE4 = 0`); si no existe lo añade a la segunda tabla (creciendo `[+0xCC]` en pasos de 0x20 entradas y `[+0xB8]` en 0x100 bytes con ALLOC/FREE) y deja `+0xE4 = 1`. Sin retorno útil.
pub fn name_bank_find_or_add_ship_name(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let (name, out) = (a[0], a[1]);
    let first = m.r8(name);
    let same = |m: &mut dyn Mem, p: u32| -> bool {
        if m.r8(p) != first {
            return false;
        }
        let mut i = 0u32;
        loop {
            if m.r8(p + i) == 0 {
                return true;
            }
            i += 1;
            if m.r8(name + i) != m.r8(p + i) {
                return false;
            }
        }
    };
    let n1 = u32::from(m.r16(this + 0xd8));
    if n1 != 0 {
        let base = m.r32(this + 0xb4);
        let arr = m.r32(this + 0xc8);
        for j in 0..n1 {
            let off = u32::from(m.r16(arr + 2 * j));
            if same(m, base.wrapping_add(off)) {
                m.w16(out, j as u16);
                m.w32(this + 0xe4, 0);
                return 0;
            }
        }
    }
    let n2 = u32::from(m.r16(this + 0xda));
    let ed = n2.wrapping_sub(n1);
    if ed != 0 {
        let base = m.r32(this + 0xb8);
        let arr = m.r32(this + 0xcc);
        let mut j = 0u32;
        loop {
            let off = u32::from(m.r16(arr + 2 * j));
            if same(m, base.wrapping_add(off)) {
                let v = (m.r16(this + 0xd8)).wrapping_add(j as u16);
                m.w16(out, v);
                m.w32(this + 0xe4, 0);
                return 0;
            }
            j += 1;
            if j >= ed {
                break;
            }
        }
    }
    let ebx = ed.wrapping_add(1);
    let cap = u32::from(m.r16(this + 0xdc));
    if ebx >= cap {
        let new = host.call(m, ALLOC, 0, &[cap * 2 + 0x40]);
        let old = m.r32(this + 0xcc);
        if old == 0 {
            m.w16(new, 0);
        } else {
            let c = u32::from(m.r16(this + 0xdc)) * 2;
            copy_bytes(m, new, old, c);
            host.call(m, FREE, 0, &[old]);
        }
        let c = m.r16(this + 0xdc).wrapping_add(0x20);
        m.w16(this + 0xdc, c);
        m.w32(this + 0xcc, new);
    }
    let arr = m.r32(this + 0xcc);
    let scap = m.r32(this + 0xe0);
    let off = u32::from(m.r16(arr.wrapping_add(ed.wrapping_mul(2))));
    if (off.wrapping_add(0x18) as i32) >= (scap as i32) {
        let new = host.call(m, ALLOC, 0, &[scap.wrapping_add(0x100)]);
        let scap2 = m.r32(this + 0xe0);
        if scap2 != 0 {
            let old = m.r32(this + 0xb8);
            copy_bytes(m, new, old, scap2);
            host.call(m, FREE, 0, &[old]);
            let s2 = m.r32(this + 0xe0);
            for k in 0..0x100u32 {
                m.w8(new.wrapping_add(s2).wrapping_add(k), 0);
            }
        } else {
            for k in 0..0x100u32 {
                m.w8(new + k, 0);
            }
        }
        let s3 = m.r32(this + 0xe0).wrapping_add(0x100);
        m.w32(this + 0xb8, new);
        m.w32(this + 0xe0, s3);
    }
    let w = m.r16(this + 0xd8).wrapping_add(ed as u16);
    let n = m.r16(this + 0xda).wrapping_add(1);
    m.w16(this + 0xda, n);
    m.w16(out, w);
    let arr = m.r32(this + 0xcc);
    let slot = arr.wrapping_add(ebx.wrapping_mul(2));
    let prev = m.r16(slot.wrapping_sub(2));
    m.w16(slot, prev);
    let mut p = name;
    loop {
        let arr = m.r32(this + 0xcc);
        let slot = arr.wrapping_add(ebx.wrapping_mul(2));
        let off = u32::from(m.r16(slot));
        let base = m.r32(this + 0xb8);
        let ch = m.r8(p);
        m.w8(off.wrapping_add(base), ch);
        let arr = m.r32(this + 0xcc);
        let slot = arr.wrapping_add(ebx.wrapping_mul(2));
        let v = m.r16(slot).wrapping_add(1);
        m.w16(slot, v);
        let d = m.r8(p);
        p += 1;
        if d == 0 {
            break;
        }
    }
    m.w32(this + 0xe4, 1);
    0
}

/// 0x004DE4F0: la IA elige dónde expandirse. Recorre los registros `0x5303C0(i)` (`i` descendente, `word[0x6DE4AA]` entradas) con `[+0] >= 0x9C40`, `word(+8) != 0` y bit 4 de `+8` apagado; divide su valor entre la longitud de la cadena `0x530400` (máx. 5) y se queda con el mayor (`L10`). Si no hay candidato suma `0x1500` al registro de `this` (`[this] + word(this+8)*0x18`); si lo hay puntúa cada ciudad con el flag `+0x76C` bit 8: suma de `0x3E8` por ranura `+0x878+0x10k` (18) con `word > 0x3E8`, dividida por el tamaño de la cadena, descartando las poco productivas (`esi/7 < ebx/10`), y suma al registro `0x5308A0(L10, ciudad, 1)` `+ (ciudad + L10 + 0x78)*72`; sin ciudad válida suma `0x2400`. Sin retorno útil.
pub fn st_ai_pick_expansion(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32) -> u32 {
    let mut ebx = u32::from(m.r16(0x6de4aa));
    let mut ebp = 0x9c40u32;
    let mut l10 = 0xffffu32;
    let l1c_init = 0xffffu32;
    let rec_add = |m: &mut dyn Mem, v: u32| {
        let w = u32::from(m.r16(this + 8));
        let rec = m.r32(this).wrapping_add(w.wrapping_mul(0x18));
        let x = m.r32(rec).wrapping_add(v);
        m.w32(rec, x);
    };
    if ebx != 0 {
        loop {
            ebx -= 1;
            let edi = host.call(m, 0x5303c0, 0x6de4a0, &[ebx]);
            let score = m.r32(edi);
            let skip = score < ebp || m.r16(edi + 8) == 0 || (m.r8(edi + 8) & 4) == 4;
            if !skip {
                let k = u32::from(m.r16(edi + 0xc));
                let mut p = host.call(m, 0x530400, 0x6de4a0, &[k]);
                let mut esi = 0u32;
                let mut too_long = false;
                if p != 0 {
                    loop {
                        esi += 1;
                        let nk = u32::from(m.r16(p + 0x2c8));
                        p = host.call(m, 0x530400, 0x6de4a0, &[nk]);
                        if p == 0 {
                            break;
                        }
                    }
                    too_long = esi >= 6;
                }
                if !too_long {
                    let eax = if esi != 0 { m.r32(edi) / esi } else { m.r32(edi) };
                    if eax > ebp {
                        ebp = eax;
                        l10 = ebx;
                    }
                }
            }
            if ebx == 0 {
                break;
            }
        }
    }
    let cnt = u32::from(m.r16(0x6de4aa));
    if l10 >= cnt {
        rec_add(m, 0x1500);
        return 0;
    }
    let mut ecx = m.r32(0x6de4b0);
    let mut best_town = l1c_init;
    let mut best_score = 0u32;
    if ecx != 0 {
        let mut i = 0u32;
        let mut off = 0u32;
        loop {
            let ebp = off.wrapping_add(m.r32(0x6de508));
            if (m.r32(ebp + 0x76c) >> 8) & 1 != 0 {
                let key = u32::from(m.r16(ebp + 0x784));
                let mut esi = 0u32;
                let mut bx = 0u32;
                let mut cntn = 0u32;
                let mut edi = host.call(m, 0x530400, 0x6de4a0, &[key]);
                let mut skip_town = false;
                if edi != 0 {
                    loop {
                        let k1 = u32::from(m.r16(edi + 0x2c4));
                        let r = host.call(m, 0x5303c0, 0x6de4a0, &[k1]);
                        if m.r16(r + 8) != 0 {
                            let mut c = u32::from(m.r16(edi + 0x2cc));
                            let lim = u32::from(m.r16(0x6de4a6));
                            if c < lim {
                                loop {
                                    let e = host.call(m, 0x5303b0, 0x6de4a0, &[c]);
                                    let d4 = u32::from(m.r16(e + 4));
                                    c = u32::from(m.r16(e + 8));
                                    esi = esi.wrapping_add(d4);
                                    let dc = u32::from(m.r16(e + 0xc));
                                    bx = bx.wrapping_add(dc);
                                    if c >= u32::from(m.r16(0x6de4a6)) {
                                        break;
                                    }
                                }
                            }
                        }
                        cntn += 1;
                        let k2 = u32::from(m.r16(edi + 0x2ca));
                        edi = host.call(m, 0x530400, 0x6de4a0, &[k2]);
                        if edi == 0 {
                            break;
                        }
                    }
                    if bx != 0 {
                        let hi10 = ((u64::from(bx) * 0xcccc_cccdu64) >> 32) as u32;
                        let hi7 = ((u64::from(esi) * 0x2492_4925u64) >> 32) as u32;
                        let q7 = ((esi.wrapping_sub(hi7) >> 1).wrapping_add(hi7)) >> 2;
                        if q7 < (hi10 >> 3) {
                            skip_town = true;
                        }
                    }
                }
                if !skip_town {
                    let mut eax = 0u32;
                    for k in 0..18u32 {
                        if m.r16(ebp + 0x878 + 0x10 * k) > 0x3e8 {
                            eax = eax.wrapping_add(0x3e8);
                        }
                    }
                    if cntn != 0 {
                        eax /= cntn;
                    }
                    if best_score < eax {
                        best_score = eax;
                        best_town = i;
                    }
                }
            }
            i += 1;
            off = off.wrapping_add(0x9f8);
            ecx = m.r32(0x6de4b0);
            if i >= ecx {
                break;
            }
        }
    }
    if best_town >= ecx {
        rec_add(m, 0x2400);
        return 0;
    }
    host.call(m, 0x5308a0, 0x6de4a0, &[l10, best_town, 1]);
    let v = best_town.wrapping_add(l10).wrapping_add(0x78).wrapping_mul(72);
    rec_add(m, v);
    0
}

/// 0x005353C0: byte `[ptr + a0]` de la tabla de la ranura del jugador actual (`[0x6DFC14]`) entre los ocho ids `this+0xA00+i`; `0xF` si no hay ranura o su puntero (`this+0xA08+4i`) es nulo.
pub fn town_player_slot_table_byte(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let cur = m.r32(0x6dfc14);
    for i in 0..8u32 {
        if u32::from(m.r8(this + 0xa00 + i)) == cur {
            let p = m.r32(this + 0xa08 + i * 4);
            if p == 0 {
                return 0xf;
            }
            return u32::from(m.r8(p.wrapping_add(a[0])));
        }
    }
    0xf
}

/// 0x00517F70: `word(this+0x40)*2` más, si `word(this+0x42) < word[0x6DD892]`, el word `+0xC` de la fila `0x6DD7A0 + idx*16`; si no, 2 cuando `word(this+0x40) != 0`.
pub fn record_class_base_plus_double_count(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, _a: &[u32]) -> u32 {
    let idx = u32::from(m.r16(this + 0x42));
    let cnt = u32::from(m.r16(this + 0x40));
    if m.r16(this + 0x42) < m.r16(0x6dd892) {
        let base = m.r32(0x6dd7a0);
        let w = u32::from(m.r16(idx.wrapping_mul(16).wrapping_add(base).wrapping_add(0xc)));
        w.wrapping_add(cnt.wrapping_mul(2))
    } else {
        (if cnt != 0 { 2u32 } else { 0u32 }).wrapping_add(cnt.wrapping_mul(2))
    }
}

/// 0x004D7AC0: recorre filas de 16 bytes desde el índice `a0` (`< word(this+6)`, con signo) siguiendo `word(+6)`; devuelve 1 si alguna tiene tipo `0x5F` y `word(+0xA) == a1`.
pub fn list_find_entry_kind_5f_with_id(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let n = i32::from(m.r16(this + 6));
    let mut idx = a[0] as i32;
    if idx >= n {
        return 0;
    }
    let base = m.r32(this);
    loop {
        let rec = base.wrapping_add((idx as u32).wrapping_shl(4));
        if m.r8(rec + 4) == 0x5f && u32::from(m.r16(rec + 0xa)) == a[1] {
            return 1;
        }
        idx = i32::from(m.r16(rec + 6));
        if idx >= n {
            return 0;
        }
    }
}

/// 0x00516280: `0x54` si no hay `this+0x24` (y pone `this+0x36 = 0`) o la bandera calculada es falsa; si no `[[this+0x24]+4]*4 + 0x56`. La bandera es 1 si `byte(+0x37)` o `byte(+0x38)` `>= [0x6DE4B0]`, y `byte(+0x36) != 0` en otro caso.
pub fn office_cost_0x54_or_len(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, _a: &[u32]) -> u32 {
    let esi = m.r32(this + 0x24);
    let flag = if esi == 0 {
        m.w8(this + 0x36, 0);
        false
    } else {
        let lim = m.r32(0x6de4b0);
        if u32::from(m.r8(this + 0x37)) >= lim || u32::from(m.r8(this + 0x38)) >= lim {
            true
        } else {
            m.r8(this + 0x36) != 0
        }
    };
    if flag {
        m.r32(esi + 4).wrapping_mul(4).wrapping_add(0x56)
    } else {
        0x54
    }
}

/// 0x00506290: para cada una de las cinco ranuras `this+0xC+4k` con puntero, sigue la cadena de filas de `0x180` bytes (`[this+4]`, siguiente = `word(+4)`) mientras el índice sea `< word(this+0xF4)`; devuelve el total de pasos.
pub fn convoy_count_ship_chain_lengths(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, _a: &[u32]) -> u32 {
    let mut count = 0u32;
    for k in 0..5u32 {
        let p = m.r32(this + 0xc + 4 * k);
        if p == 0 {
            continue;
        }
        let mut idx = u32::from(m.r16(p + 0xc));
        let lim = u32::from(m.r16(this + 0xf4));
        if idx >= lim {
            continue;
        }
        let base = m.r32(this + 4);
        loop {
            count = count.wrapping_add(1);
            idx = u32::from(m.r16(idx.wrapping_mul(0x180).wrapping_add(base).wrapping_add(4)));
            if idx >= lim {
                break;
            }
        }
    }
    count
}

/// 0x00512B20: puntero al nombre `a0` del banco: primeras `word(+0xD8)` entradas con la tabla `[+0xC8]`/base `[+0xB4]`, siguientes hasta `word(+0xDA)` con `[+0xCC]`/`[+0xB8]`; fuera de rango, tabla fija `0x6C0CB4[(-1-a0)&3]`.
pub fn name_bank_get_name_ptr(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let ax = a[0] as u16;
    let d8 = m.r16(this + 0xd8);
    if ax < d8 {
        let t = m.r32(this + 0xc8);
        let w = u32::from(m.r16(t.wrapping_add(u32::from(ax) * 2)));
        return w.wrapping_add(m.r32(this + 0xb4));
    }
    if ax < m.r16(this + 0xda) {
        let t = m.r32(this + 0xcc);
        let i = u32::from(ax).wrapping_sub(u32::from(d8));
        let w = u32::from(m.r16(t.wrapping_add(i.wrapping_mul(2))));
        return w.wrapping_add(m.r32(this + 0xb8));
    }
    let k = 0xffff_ffffu32.wrapping_sub(a[0]) & 3;
    m.r32(0x6c0cb4 + k * 4)
}

/// 0x00503B00: ¿la tarea `word(this+2)` (`< word[0x6DD748]`, vía `0x4D8DB0`) es de tipo `0x34`, con `byte(+0x15) >= 7` y `word(+0x16) == a0`?
pub fn task_hanse_target_matches(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let id = m.r16(this + 2);
    if id < m.r16(0x6dd748) {
        let rec = host.call(m, 0x4d8db0, 0x6dd73c, &[u32::from(id)]);
        if m.r16(rec + 6) == 0x34 && m.r8(rec + 0x15) >= 7 {
            return u32::from(u32::from(m.r16(rec + 0x16)) == a[0]);
        }
    }
    0
}

/// 0x00545B20: valores por defecto de una ciudad: 24 filas (`+0x64/+0xC4/+0x310` a 1, bloque `+0x4F0+16k` = 1,2,3,4), contadores, 21 ranuras en `+0x84A+16k` (índice y copia del byte `+0x2C0`) y `+0x300/+0x304 = 0x320032`. Devuelve `this+0x300`.
pub fn town_init_default_slots(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, _a: &[u32]) -> u32 {
    for i in 0..24u32 {
        m.w32(this + 0x64 + 4 * i, 1);
        m.w32(this + 0xc4 + 4 * i, 1);
        m.w32(this + 0x310 + 4 * i, 1);
        let e = this + 0x4f4 + 16 * i;
        m.w32(e - 4, 1);
        m.w32(e, 2);
        m.w32(e + 4, 3);
        m.w32(e + 8, 4);
    }
    m.w16(this + 0x996, 0);
    m.w16(this + 0x994, 0xffff);
    m.w16(this + 0x2c6, 0x28);
    for o in [0x9a4u32, 0x9a8, 0x9ac, 0x9b0] {
        m.w32(this + o, 0);
    }
    for k in 0..0x15u32 {
        let e = this + 0x84a + 16 * k;
        m.w8(e - 4, k as u8);
        m.w16(e, 0);
        m.w32(e + 2, 0);
        let b = m.r8(this + 0x2c0);
        m.w8(e - 3, b);
    }
    m.w16(this + 0x9a2, 0xffff);
    m.w32(this + 0x300, 0x0032_0032);
    m.w32(this + 0x304, 0x0032_0032);
    this + 0x300
}

/// 0x00527890: tras `0x52DE70`, pone a cero `+0x2C8/+0x2D0`, las tablas de 24 entradas (`+4`, `+0x370`, `+0x3D0`, `+0x430`, `+0x490`, `+0x670` y el bloque `+0x4F0+16k` de cuatro) y `+0x6D0`. Devuelve `this`.
pub fn town_clear_slot_tables(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _a: &[u32]) -> u32 {
    host.call(m, 0x52de70, this, &[]);
    m.w32(this + 0x2c8, 0);
    m.w32(this + 0x2d0, 0);
    for i in 0..24u32 {
        let eax = this + 0x370 + 4 * i;
        m.w32(eax - 0x36c, 0);
        m.w32(eax, 0);
        m.w32(eax + 0x60, 0);
        m.w32(eax + 0xc0, 0);
        m.w32(eax + 0x120, 0);
        let e = this + 0x4f4 + 16 * i;
        m.w32(e - 4, 0);
        m.w32(e, 0);
        m.w32(e + 4, 0);
        m.w32(e + 8, 0);
        m.w32(eax + 0x300, 0);
    }
    m.w32(this + 0x6d0, 0);
    this
}

/// 0x00539C60: guarda en la fila de `0x6DD7A8` (60 bytes, índice `word(this+4)`) la terna `word(this+8)`, `word(this+0xA)`, `byte(this+0xC)` en la ranura `word(this+6)`, si la fila apunta a un índice válido y (`word(+0x12) >= 0xF` o `0x519BA0` acepta la fila de `0x6DD7A4`).
pub fn record_store_slot_triple(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _a: &[u32]) -> u32 {
    let idx = m.r16(this + 4);
    if idx >= m.r16(0x6dd896) {
        return 0;
    }
    let base = m.r32(0x6dd7a8);
    let rec = base.wrapping_add(u32::from(idx) * 60);
    let j = m.r16(rec + 0x10);
    if j >= m.r16(0x6dd894) {
        return 0;
    }
    if m.r16(rec + 0x12) < 0xf {
        let obj = m.r32(0x6dd7a4).wrapping_add(u32::from(j) * 0x180);
        if (host.call(m, 0x519ba0, obj, &[]) & 0xff) == 0 {
            return 0;
        }
    }
    let s = u32::from(m.r16(this + 6));
    let v8 = m.r16(this + 8);
    m.w16(rec + s * 4 + 0x24, v8);
    let s = u32::from(m.r16(this + 6));
    let va = m.r16(this + 0xa);
    m.w16(rec + s * 4 + 0x26, va);
    let s = u32::from(m.r16(this + 6));
    let vc = m.r8(this + 0xc);
    m.w8(rec + s + 0x34, vc);
    0
}

/// 0x0050AAE0: guarda la ranura anterior `word(this+0xEC)` en la fila `a0` (60 bytes, base `[this+8]`, campo `+8`), pasa `a0` a `this+0xEC` y, si el índice `word(fila+0x10) >= word(this+0xF4)`, llama a `0x5017F0(fila)`; termina con `0x50CE50(word(fila+0x10), 0xFFFF)`.
pub fn list_activate_slot_and_relink(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let prev = m.r16(this + 0xec);
    let off = (a[0] & 0xffff).wrapping_mul(60);
    let rec = m.r32(this + 8).wrapping_add(off);
    m.w16(rec + 8, prev);
    m.w16(this + 0xec, a[0] as u16);
    if m.r16(rec + 0x10) >= m.r16(this + 0xf4) {
        host.call(m, 0x5017f0, rec, &[]);
    }
    let w = u32::from(m.r16(rec + 0x10));
    host.call(m, 0x50ce50, this, &[w, 0xffff]);
    0
}

/// 0x00535160: serializa en `a0` el historial de la ciudad: `0xA00` bytes de `this`, los 8 ids (`+0xA00`) y, por cada id distinto de `0xFF`, 40 bytes del puntero `+0xA08+4k`. Devuelve el final.
pub fn town_history_serialize(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut d = a[0];
    copy_bytes(m, d, this, 0xa00);
    d = d.wrapping_add(0xa00);
    copy_bytes(m, d, this + 0xa00, 8);
    d = d.wrapping_add(8);
    for k in 0..8u32 {
        if m.r8(this + 0xa00 + k) != 0xff {
            let p = m.r32(this + 0xa08 + 4 * k);
            copy_bytes(m, d, p, 0x28);
            d = d.wrapping_add(0x28);
        }
    }
    d
}

/// 0x005137E0: serializa la cabecera de banda pirata en `a0`: words `+8,+0xA,+0xC`, bytes `+0xE,+0xF`, words `+0x10,+0x12`, bytes `+0x14,+0x15`, word `+0x16`; luego `0` o `0x7F` + `0x4F9680([this])`, y `word(+8)` filas de 8 bytes de `[this+4]`. Devuelve el final.
pub fn pirate_band_header_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut d = a[0];
    for (off, n) in [(8u32, 2u32), (0xa, 2), (0xc, 2)] {
        copy_bytes(m, d, this + off, n);
        d = d.wrapping_add(2);
    }
    let b = m.r8(this + 0xe);
    m.w8(d, b);
    d = d.wrapping_add(1);
    let b = m.r8(this + 0xf);
    m.w8(d, b);
    d = d.wrapping_add(1);
    copy_bytes(m, d, this + 0x10, 2);
    d = d.wrapping_add(2);
    copy_bytes(m, d, this + 0x12, 2);
    d = d.wrapping_add(2);
    let b = m.r8(this + 0x14);
    m.w8(d, b);
    d = d.wrapping_add(1);
    let b = m.r8(this + 0x15);
    m.w8(d, b);
    d = d.wrapping_add(1);
    copy_bytes(m, d, this + 0x16, 2);
    d = d.wrapping_add(2);
    let owner = m.r32(this);
    if owner == 0 {
        m.w8(d, 0);
        d = d.wrapping_add(1);
    } else {
        m.w8(d, 0x7f);
        d = d.wrapping_add(1);
        d = host.call(m, 0x4f9680, owner, &[d]);
    }
    let n = u32::from(m.r16(this + 8));
    if n == 0 {
        return d;
    }
    let src = m.r32(this + 4);
    copy_bytes(m, d, src, n * 8);
    d.wrapping_add(n * 8)
}

/// 0x0051A8D0 (ret 4): serializa una carta en `a0`: bytes `+0/+1`, 2 bytes de `+2`, bytes `+4/+5`, 2 bytes de `+6` y la carga según el tipo `+4` (`0x29/0x47/0x63/0x83`: 8 bytes de `+8` y `[+0xC]` bytes de `[+8]`; `0x3C..0x40/0x71`: 0x10 bytes de `[+8]` y `[[+8]]` bytes de `[+0xC]`; resto: 8 bytes de `+8`). Devuelve el final.
pub fn letter_serialize(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut d = a[0];
    let b = m.r8(this);
    m.w8(d, b);
    let b = m.r8(this + 1);
    m.w8(d + 1, b);
    d = d.wrapping_add(2);
    copy_bytes(m, d, this + 2, 2);
    d = d.wrapping_add(2);
    let ty = m.r8(this + 4);
    m.w8(d, ty);
    let b = m.r8(this + 5);
    m.w8(d + 1, b);
    d = d.wrapping_add(2);
    copy_bytes(m, d, this + 6, 2);
    d = d.wrapping_add(2);
    let ty = m.r8(this + 4);
    if matches!(ty, 0x29 | 0x47 | 0x63 | 0x83) {
        let p = m.r32(this + 8);
        copy_bytes(m, d, this + 8, 8);
        d = d.wrapping_add(8);
        let n = m.r32(this + 0xc);
        copy_bytes(m, d, p, n);
        return d.wrapping_add(m.r32(this + 0xc));
    }
    if (0x3c..=0x40).contains(&ty) || ty == 0x71 {
        let p = m.r32(this + 8);
        copy_bytes(m, d, p, 0x10);
        d = d.wrapping_add(0x10);
        let q = m.r32(this + 0xc);
        let hp = m.r32(this + 8);
        let n = m.r32(hp);
        copy_bytes(m, d, q, n);
        let hp = m.r32(this + 8);
        let n2 = m.r32(hp);
        return d.wrapping_add(n2);
    }
    copy_bytes(m, d, this + 8, 8);
    d.wrapping_add(8)
}

/// 0x0051AB20 (ret 4): copia la carta `a0` en `this` duplicando sus cargas (`0x29/0x47/0x63/0x83`: buffer de `[+0xC]` bytes, a cero si falla la reserva; `0x3C..0x40/0x71`: cabecera de 0x10 y cuerpo de `[cab]` bytes; resto: 8 bytes).
pub fn letter_copy_construct(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let src = a[0];
    let b = m.r8(src);
    m.w8(this, b);
    let b = m.r8(src + 1);
    m.w8(this + 1, b);
    let w = m.r16(src + 2);
    m.w16(this + 2, w);
    let ty = m.r8(src + 4);
    m.w8(this + 4, ty);
    let b = m.r8(src + 5);
    m.w8(this + 5, b);
    if matches!(ty, 0x29 | 0x47 | 0x63 | 0x83) {
        copy_bytes(m, this + 8, src + 8, 8);
        let n = m.r32(this + 0xc);
        let p = host.call(m, ALLOC, 0, &[n]);
        m.w32(this + 8, p);
        if p == 0 {
            m.w32(this + 0xc, 0);
            return 0;
        }
        let n = m.r32(this + 0xc);
        let s = m.r32(src + 8);
        copy_bytes(m, p, s, n);
        return 0;
    }
    if (0x3c..=0x40).contains(&ty) || ty == 0x71 {
        let p1 = host.call(m, ALLOC, 0, &[0x10]);
        m.w32(this + 8, p1);
        let s = m.r32(src + 8);
        copy_bytes(m, p1, s, 0x10);
        let hp = m.r32(this + 8);
        let n = m.r32(hp);
        let p2 = host.call(m, ALLOC, 0, &[n]);
        m.w32(this + 0xc, p2);
        let hp = m.r32(this + 8);
        let n = m.r32(hp);
        let s2 = m.r32(src + 0xc);
        copy_bytes(m, p2, s2, n);
        return 0;
    }
    copy_bytes(m, this + 8, src + 8, 8);
    0
}

/// 0x005306B0: toma el primer hueco libre de una lista de filas de 20 bytes (`[this+0x70]`, índice libre en `word(this+0x7C)`, capacidad `word(this+6)`); si no queda, la amplía (0x80 o capacidad+0x40), la rellena con `0xFF`, copia las filas antiguas y encadena los nuevos huecos en el word `+8`. Deja el siguiente hueco en `word(this+0x7C)` y marca la fila con `0xFFFF`.
pub fn list_grow_array_20(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _a: &[u32]) -> u32 {
    let cap = m.r16(this + 6);
    if m.r16(this + 0x7c) >= cap {
        m.w16(this + 0x7c, cap);
        if cap < 0x14 {
            m.w16(this + 6, 0x80);
        } else {
            m.w16(this + 6, cap.wrapping_add(0x40));
        }
        let ncap = u32::from(m.r16(this + 6));
        let p = host.call(m, ALLOC, 0, &[ncap * 20]);
        for k in 0..ncap * 20 {
            m.w8(p.wrapping_add(k), 0xff);
        }
        let old = m.r32(this + 0x70);
        let used = u32::from(m.r16(this + 0x7c));
        copy_bytes(m, p, old, used * 20);
        if old != 0 {
            host.call(m, FREE, 0, &[old]);
        }
        let mut free = m.r16(this + 0x7c);
        m.w32(this + 0x70, p);
        let cap2 = m.r16(this + 6);
        if free < cap2 {
            loop {
                let nx = free.wrapping_add(1);
                m.w16(p.wrapping_add(u32::from(free) * 20 + 8), nx);
                free = nx;
                if free >= m.r16(this + 6) {
                    break;
                }
            }
        }
    }
    let idx = u32::from(m.r16(this + 0x7c));
    let rec = m.r32(this + 0x70).wrapping_add(idx * 20);
    let nxt = m.r16(rec + 8);
    m.w16(this + 0x7c, nxt);
    m.w16(rec + 8, 0xffff);
    0
}

/// 0x005013D0 (ret 4): serializa los primeros `0x3A` bytes de `this` en `a0` (los desplazamientos de origen y destino coinciden: bytes, words y dwords en secuencia). Devuelve `a0+0x3A`.
pub fn record_serialize_58(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    copy_bytes(m, a[0], this, 0x3a);
    a[0].wrapping_add(0x3a)
}

/// 0x0052BB30 (ret 4): serializa un centro comercial: `0x1E` bytes de `this` y, por cada nodo de la lista circular `[this+0x24]`, 16 bytes de `nodo+8`. Devuelve el final.
pub fn trade_center_serialize(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut d = a[0];
    copy_bytes(m, d, this, 0x1e);
    d = d.wrapping_add(0x1e);
    let h0 = m.r32(this + 0x24);
    let mut n = m.r32(h0);
    while n != m.r32(this + 0x24) {
        copy_bytes(m, d, n + 8, 0x10);
        n = m.r32(n);
        d = d.wrapping_add(0x10);
    }
    d
}

/// 0x005347F0 (ret 4): serializa en `a0` el dword `this+0x60`, `0x30` bytes de `this` y `0x30` de `this+0x30`. Devuelve `a0+0x64`.
pub fn pair_block_serialize(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut d = a[0];
    copy_bytes(m, d, this + 0x60, 4);
    d = d.wrapping_add(4);
    copy_bytes(m, d, this, 0x30);
    d = d.wrapping_add(0x30);
    copy_bytes(m, d, this + 0x30, 0x30);
    d.wrapping_add(0x30)
}

/// 0x005162D0 (ret 4): serializa una orden (84 bytes) en `a0`: campos `+4`, bytes sueltos `+0/+0xE/+0xF/+1/+3/+2`, dwords y words de `+0x10..+0x35`, el byte calculado (0 sin `+0x24`; 1 si `+0x37` o `+0x38 >= [0x6DE4B0]`; si no `+0x36`), bytes `+0x37..+0x3B` y words/dwords hasta `+0x50`; si el byte es distinto de cero añade la lista `[+0x24]` (`word(+4)` pares de words desde `[[+0x24]]`). Devuelve el final.
pub fn ship_order_record_serialize(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut d = a[0];
    copy_bytes(m, d, this + 4, 2);
    d = d.wrapping_add(2);
    for off in [0u32, 0xe, 0xf, 1, 3, 2] {
        let b = m.r8(this + off);
        m.w8(d, b);
        d = d.wrapping_add(1);
    }
    for (off, n) in [(0x10u32, 4u32), (0x14, 4), (6, 2), (8, 2), (0xa, 2), (0xc, 2), (0x18, 4), (0x1c, 4), (0x20, 4), (0x24, 4), (0x28, 4), (0x2c, 4), (0x30, 4), (0x34, 2)] {
        copy_bytes(m, d, this + off, n);
        d = d.wrapping_add(n);
    }
    let list = m.r32(this + 0x24);
    let flag: u8 = if list == 0 {
        0
    } else {
        let lim = m.r32(0x6de4b0);
        if u32::from(m.r8(this + 0x37)) >= lim || u32::from(m.r8(this + 0x38)) >= lim {
            1
        } else {
            m.r8(this + 0x36)
        }
    };
    m.w8(d, flag);
    d = d.wrapping_add(1);
    for off in [0x37u32, 0x38, 0x39, 0x3b, 0x3a] {
        let b = m.r8(this + off);
        m.w8(d, b);
        d = d.wrapping_add(1);
    }
    for (off, n) in [(0x3cu32, 2u32), (0x3e, 2), (0x40, 2), (0x42, 2), (0x44, 4), (0x48, 4), (0x4c, 4), (0x50, 4)] {
        copy_bytes(m, d, this + off, n);
        d = d.wrapping_add(n);
    }
    if flag != 0 {
        let n = m.r16(list + 4);
        m.w16(d, n);
        d = d.wrapping_add(2);
        let arr = m.r32(list);
        for i in 0..u32::from(n) {
            copy_bytes(m, d, arr.wrapping_add(i * 4), 2);
            d = d.wrapping_add(2);
            copy_bytes(m, d, arr.wrapping_add(i * 4 + 2), 2);
            d = d.wrapping_add(2);
        }
    }
    d
}

/// 0x005177D0 (ret 4): serializa un barco en `a0`: `word(+0x134)`; si es `>= 0x17` solo `word(+6)` y `byte(+0x137)`; si no, la orden `0x5162D0` y los bloques `+0x54`/`+0xB4` (0x60), `+0x114..+0x135`, bytes `+0x136..+0x13B`, `+0x13C` (0x18), `+0x154..+0x15F` y `+0x160` (0x20). Devuelve el final.
pub fn ship_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut d = a[0];
    copy_bytes(m, d, this + 0x134, 2);
    d = d.wrapping_add(2);
    if m.r16(this + 0x134) >= 0x17 {
        copy_bytes(m, d, this + 6, 2);
        d = d.wrapping_add(2);
        let b = m.r8(this + 0x137);
        m.w8(d, b);
        return d.wrapping_add(1);
    }
    d = ship_order_record_serialize(m, host, this, &[d]);
    for (off, n) in [(0x54u32, 0x60u32), (0xb4, 0x60), (0x114, 4), (0x124, 4), (0x128, 4), (0x12c, 4), (0x130, 2), (0x132, 2)] {
        copy_bytes(m, d, this + off, n);
        d = d.wrapping_add(n);
    }
    for off in [0x136u32, 0x137] {
        let b = m.r8(this + off);
        m.w8(d, b);
        d = d.wrapping_add(1);
    }
    for (off, n) in [(0x13au32, 1u32), (0x13b, 1), (0x138, 2), (0x13c, 0x18), (0x154, 2)] {
        copy_bytes(m, d, this + off, n);
        d = d.wrapping_add(n);
    }
    for off in [0x156u32, 0x157, 0x158, 0x159] {
        let b = m.r8(this + off);
        m.w8(d, b);
        d = d.wrapping_add(1);
    }
    copy_bytes(m, d, this + 0x15a, 2);
    d = d.wrapping_add(2);
    for off in [0x15cu32, 0x15d, 0x15f, 0x15e] {
        let b = m.r8(this + off);
        m.w8(d, b);
        d = d.wrapping_add(1);
    }
    copy_bytes(m, d, this + 0x160, 0x20);
    d.wrapping_add(0x20)
}

/// 0x00504010 (ret 4): serializa un convoy en `a0`: marca `0x85`, once words `+0xE0..+0xF6`, cuatro más (`+0xE6`, `+0xFA`, `+0xFC`, `+0xFE`) y `+0xF8`; `word(+0xF2)` entradas de 16 bytes vía `0x4FDE30`, `word(+0xF4)` barcos (`0x5177D0`, filas de `0x180` en `[+4]`), `word(+0xF6)` registros (`0x5013D0`, `0x3C` en `[+8]`), `0xC0` bytes de `+0x20` y cinco bandas `0x5137E0` (`0`/`0x7F` + cabecera) de `+0xC`. Devuelve el final.
pub fn convoy_block_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut d = a[0];
    m.w16(d, 0x85);
    d = d.wrapping_add(2);
    for off in [0xe0u32, 0xe2, 0xe4, 0xe8, 0xea, 0xec, 0xee, 0xf0, 0xf2, 0xf4, 0xf6, 0xe6, 0xfa, 0xfc, 0xfe, 0xf8] {
        copy_bytes(m, d, this + off, 2);
        d = d.wrapping_add(2);
    }
    let n1 = u32::from(m.r16(this + 0xf2));
    for i in 0..n1 {
        let ecx = m.r32(this).wrapping_add(i * 0x10);
        d = host.call(m, 0x4fde30, ecx, &[d]);
    }
    let n2 = u32::from(m.r16(this + 0xf4));
    for k in 0..n2 {
        let rec = m.r32(this + 4).wrapping_add(k * 0x180);
        d = ship_serialize(m, host, rec, &[d]);
    }
    let n3 = u32::from(m.r16(this + 0xf6));
    for k in 0..n3 {
        let rec = m.r32(this + 8).wrapping_add(k * 0x3c);
        d = record_serialize_58(m, host, rec, &[d]);
    }
    copy_bytes(m, d, this + 0x20, 0xc0);
    d = d.wrapping_add(0xc0);
    for k in 0..5u32 {
        let p = m.r32(this + 0xc + 4 * k);
        if p == 0 {
            m.w8(d, 0);
            d = d.wrapping_add(1);
        } else {
            m.w8(d, 0x7f);
            d = d.wrapping_add(1);
            d = pirate_band_header_serialize(m, host, p, &[d]);
        }
    }
    d
}

/// 0x0052F870 (ret 4): serializa el estado del mundo en `a0`: cabecera (bytes/words `+0..+0xA`, `+0x10`, `+0xC`, `+0x14`, `+0x7C/+0x7E`, dos bloques de 0x28, siete de 12 bytes desde `+0x90`), `dword(+0x10)` ciudades (`0x51B330`, filas de `0x9F8` en `[+0x68]`), su matriz `n*n` de `[+0x6C]`, marcador `F1 F2 F3 F4`, `0x6335A0` por entrada de `[0x6DF270]` (`0x1024`), `word(+6)` entradas `0x4D3250` (`0x14`), la cadena de `[+0x74]` (filas de `0x44C`, `+0x2C4 = 0xFFFF`), `word(+8)` filas `0x4FED50`, `word(+0xA)` filas `0x4F4010` (`0x650`), el historial `0x535160` (`this+0xE8`), quince centros `0x52BB30` (`+0xB10`, `0x2C`), la lista de ofertas (`0x533D10`) y el byte `+0x8F`. Devuelve el final.
pub fn world_state_serialize(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut d = a[0];
    for (off, n) in [(0u32, 1u32), (1, 1), (2, 2), (4, 2), (0x10, 4), (6, 2), (8, 2), (0xa, 2)] {
        copy_bytes(m, d, this + off, n);
        d = d.wrapping_add(n);
    }
    let b = m.r8(this + 0x89);
    m.w8(d, b);
    let b = m.r8(this + 0x8c);
    m.w8(d + 1, b);
    let f = m.r8(this + 0x8a);
    m.w8(d + 2, if f != 0 { 0x7f } else { 0 });
    let b = m.r8(this + 0x8b);
    m.w8(d + 3, b);
    d = d.wrapping_add(4);
    for (off, n) in [(0xcu32, 4u32), (0x14, 4), (0x7c, 2), (0x7e, 2), (0x18, 0x28), (0x40, 0x28)] {
        copy_bytes(m, d, this + off, n);
        d = d.wrapping_add(n);
    }
    let b = m.r8(this + 0x8d);
    m.w8(d, b);
    let b = m.r8(this + 0x8e);
    m.w8(d + 1, b);
    d = d.wrapping_add(2);
    for k in 0..7u32 {
        copy_bytes(m, d, this + 0x90 + 12 * k, 12);
        d = d.wrapping_add(12);
    }
    let n = m.r32(this + 0x10);
    for i in 0..n {
        let ecx = m.r32(this + 0x68).wrapping_add(i.wrapping_mul(0x9f8));
        d = host.call(m, 0x51b330, ecx, &[d]);
    }
    let nn = n.wrapping_mul(n);
    let src = m.r32(this + 0x6c);
    copy_bytes(m, d, src, nn);
    d = d.wrapping_add(nn);
    for (k, v) in [0xf1u8, 0xf2, 0xf3, 0xf4].iter().enumerate() {
        m.w8(d + k as u32, *v);
    }
    d = d.wrapping_add(4);
    for i in 0..n {
        let ecx = m.r32(0x6df270).wrapping_add(i.wrapping_mul(0x1024));
        d = host.call(m, 0x6335a0, ecx, &[d]);
    }
    let c6 = u32::from(m.r16(this + 6));
    for i in 0..c6 {
        let ecx = m.r32(this + 0x70).wrapping_add(i * 0x14);
        d = host.call(m, 0x4d3250, ecx, &[d]);
    }
    let mut idx = u32::from(m.r16(this + 0x7e));
    while idx < u32::from(m.r16(this + 8)) {
        let rec = m.r32(this + 0x74).wrapping_add(idx.wrapping_mul(0x44c));
        m.w16(rec + 0x2c4, 0xffff);
        let rec = m.r32(this + 0x74).wrapping_add(idx.wrapping_mul(0x44c));
        idx = u32::from(m.r16(rec + 0x2c8));
    }
    let c8 = u32::from(m.r16(this + 8));
    for i in 0..c8 {
        let ecx = m.r32(this + 0x74).wrapping_add(i * 0x44c);
        d = host.call(m, 0x4fed50, ecx, &[d]);
    }
    let ca = u32::from(m.r16(this + 0xa));
    for i in 0..ca {
        let ecx = m.r32(this + 0x78).wrapping_add(i * 0x650);
        d = host.call(m, 0x4f4010, ecx, &[d]);
    }
    d = town_history_serialize(m, host, this + 0xe8, &[d]);
    for k in 0..15u32 {
        d = trade_center_serialize(m, host, this + 0xb10 + 0x2c * k, &[d]);
    }
    let offers = m.r32(this + 0xe4);
    if offers != 0 {
        m.w8(d, 1);
        d = host.call(m, 0x533d10, offers, &[d.wrapping_add(1)]);
    } else {
        m.w8(d, 0);
        d = d.wrapping_add(1);
    }
    let b = m.r8(this + 0x8f);
    m.w8(d, b);
    d.wrapping_add(1)
}

/// División con signo de 64 bits de la CRT (`0x63A6E0`, lldiv): devuelve el dword bajo del cociente truncado hacia cero. Los llamadores solo usan EAX.
fn lldiv_lo(a: i64, b: i64) -> i32 {
    a.wrapping_div(b) as i32
}

/// 0x0052EE90 (ret 0x10): cantidad máxima de la mercancía `a2` que la ciudad `a1` puede comprar/vender a la ciudad `a0` (`a3` = precio/unidad): interpola entre los valores `+0x4F0/+0x4F4` de cada ciudad con las divisiones i64 de la CRT. `0` si no hay margen.
pub fn max_trade_amount(m: &mut dyn Mem, _host: &mut dyn OpsHost, _this: u32, a: &[u32]) -> u32 {
    let base = m.r32(0x6de508);
    let t0 = base.wrapping_add(a[0].wrapping_mul(2552));
    let t1 = base.wrapping_add(a[1].wrapping_mul(2552));
    let w = a[2];
    let cap0 = m.r32(t0.wrapping_add(w.wrapping_mul(4)).wrapping_add(4)) as i32;
    let lo0 = m.r32(t0.wrapping_add(w.wrapping_mul(16)).wrapping_add(0x4f4)) as i32;
    let hi0 = m.r32(t0.wrapping_add(w.wrapping_mul(16)).wrapping_add(0x4f0)) as i32;
    let mid = lo0.wrapping_add(hi0) >> 1;
    if mid >= cap0 {
        return 0;
    }
    let g = |m: &mut dyn Mem, off: u32| m.r32(t1.wrapping_add(w.wrapping_mul(4)).wrapping_add(off)) as i32;
    let x = g(m, 0x310).wrapping_sub(g(m, 0xc4)).wrapping_add(g(m, 0x64)).wrapping_mul(a[3] as i32);
    let mut d = g(m, 0x670).wrapping_sub(x).wrapping_add(g(m, 4));
    if d < 0 {
        d = 0;
    }
    let lo1 = m.r32(t1.wrapping_add(w.wrapping_mul(16)).wrapping_add(0x4f4)) as i32;
    if lo1 < d {
        return 0;
    }
    let hi1 = m.r32(t1.wrapping_add(w.wrapping_mul(16)).wrapping_add(0x4f0)) as i32;
    let q = lo1.wrapping_sub(hi1) >> 3;
    let u = hi1.wrapping_add(q).wrapping_add(q.wrapping_mul(2));
    let e = cap0.wrapping_sub(lo0);
    if lo1.wrapping_sub(d) < e {
        return lo1.wrapping_sub(d) as u32;
    }
    let f = cap0.wrapping_sub(mid);
    let t = u.wrapping_sub(d);
    if t >= 0 && t > f {
        return f as u32;
    }
    let de = d.wrapping_add(e);
    let (base_v, c1, c2);
    if de >= u {
        let g1 = lo1.wrapping_sub(de);
        let p = i64::from(lo0.wrapping_sub(mid)) * i64::from(g1);
        let qd = lldiv_lo(p, i64::from(lo1.wrapping_sub(u)));
        base_v = e;
        c1 = g1;
        c2 = lo0.wrapping_sub(qd).wrapping_sub(mid);
    } else {
        let v = u.wrapping_sub(de);
        let wv = e.wrapping_add(v);
        let g2 = cap0.wrapping_sub(wv).wrapping_sub(mid);
        let p = i64::from(lo1.wrapping_sub(u)) * i64::from(g2);
        let qd = lldiv_lo(p, i64::from(lo0.wrapping_sub(mid)));
        base_v = wv;
        c1 = qd;
        c2 = g2;
    }
    let z = i64::from(c1) * i64::from(c2);
    let mut r = base_v;
    if z > 0 {
        r = r.wrapping_add(lldiv_lo(z, i64::from(c1.wrapping_add(c2))));
    }
    r as u32
}

/// 0x00538210: operación de traspaso de armas entre un barco (`0x6DD7A4 + idx*384`) y una oficina. `this`: `+4` arma, `+8` jugador (bit 31 = de la oficina al barco), `+0xC` barco, `+0x10` cantidad. Armas `< 6` van en las ranuras de 24 bytes (`+0x13C`, pares con el marcador 6/7), el resto es el contador `+0x154`. Cierra con `0x517770(barco, 0x10)` y `0x5182B0`.
pub fn op_transfer_weapons_ship_office(m: &mut dyn Mem, host: &mut dyn OpsHost, op: u32, _a: &[u32]) -> u32 {
    fn finish(m: &mut dyn Mem, host: &mut dyn OpsHost, sh: u32) -> u32 {
        host.call(m, 0x517770, sh, &[0x10]);
        host.call(m, 0x5182b0, sh, &[])
    }
    let lim = m.r32(0x6de4b0);
    let a8 = m.r32(op + 8);
    let idx8 = a8 & 0xff;
    if idx8 >= lim {
        return 0;
    }
    let ship = m.r32(op + 0xc);
    if ship >= u32::from(m.r16(0x6dd894)) {
        return 0;
    }
    let sh = m.r32(0x6dd7a4).wrapping_add(ship.wrapping_mul(384));
    if m.r16(sh + 0x134) >= 4 {
        return 0;
    }
    let al = m.r8(sh + 0x39);
    if idx8 != u32::from(al) {
        return 0;
    }
    let b0 = u32::from(m.r8(sh));
    let off = host.call(m, 0x5308a0, 0x6de4a0, &[b0, u32::from(al), 0]);
    if off == 0 {
        return 0;
    }
    let w = m.r32(op + 4);
    let slot_add = |m: &mut dyn Mem, w: u32, delta: u32, sub: bool| {
        let p = off.wrapping_add(0x124).wrapping_add(w.wrapping_mul(4));
        let v = m.r32(p);
        m.w32(p, if sub { v.wrapping_sub(delta) } else { v.wrapping_add(delta) });
    };
    if a8 & 0x8000_0000 != 0 {
        let sw = m.r16(sh + 8);
        if sw < m.r16(0x6dd896) {
            let rec = m.r32(0x6dd7a8).wrapping_add(u32::from(sw) * 60);
            if u32::from(m.r16(rec + 0x10)) == m.r32(op + 0xc) && (m.r8(rec + 0x14) & 2) == 0 {
                return 0;
            }
        }
        if w >= 6 {
            let cx = u32::from(m.r16(sh + 0x154));
            let n = m.r32(op + 0x10);
            if n >= cx {
                let v = m.r32(off + 0x2bc).wrapping_add(cx);
                m.w32(off + 0x2bc, v);
                m.w16(sh + 0x154, 0);
            } else {
                m.w16(sh + 0x154, (cx as u16).wrapping_sub(n as u16));
                let v = m.r32(off + 0x2bc).wrapping_add(n);
                m.w32(off + 0x2bc, v);
            }
            return finish(m, host, sh);
        }
        let t = u32::from(m.r16(0x672cb4 + w * 2));
        let take = |m: &mut dyn Mem, ecx: u32| -> bool {
            u32::from(m.r8(sh + ecx + 0x13c)) == w && ((ecx & 1) == 1 || m.r8(sh + (ecx | 1) + 0x13c) != 6)
        };
        if t < 0x7d0 {
            let mut ecx = 0x18u32;
            loop {
                ecx -= 1;
                if take(m, ecx) {
                    let n = m.r32(op + 0x10);
                    if n == 0 {
                        break;
                    }
                    m.w32(op + 0x10, n - 1);
                    m.w8(sh + ecx + 0x13c, 0xff);
                    slot_add(m, w, t, false);
                }
                if ecx == 0 {
                    break;
                }
            }
            let mut ecx = 0x18u32;
            loop {
                ecx -= 1;
                if take(m, ecx) {
                    let n = m.r32(op + 0x10);
                    m.w32(op + 0x10, n.wrapping_sub(1));
                    if n == 0 {
                        return finish(m, host, sh);
                    }
                    m.w8(sh + ecx + 0x13c, 0xff);
                    slot_add(m, w, t, false);
                }
                if ecx == 0 {
                    break;
                }
            }
        } else {
            let mut ecx = 0x18u32;
            loop {
                ecx -= 1;
                if u32::from(m.r8(sh + ecx + 0x13c)) == w {
                    let n = m.r32(op + 0x10);
                    m.w32(op + 0x10, n.wrapping_sub(1));
                    if n == 0 {
                        return finish(m, host, sh);
                    }
                    m.w8(sh + ecx + 0x13c, 0xff);
                    m.w8(sh + ecx + 0x13d, 0xff);
                    slot_add(m, w, t, false);
                }
                if ecx == 0 {
                    break;
                }
            }
        }
        return finish(m, host, sh);
    }
    if w >= 6 {
        let cap = host.call(m, 0x5182b0, sh, &[]) as i32;
        let mut e = (cap.wrapping_div(10)) as u32;
        let n = m.r32(op + 0x10);
        if n < e {
            e = n;
        }
        let o = m.r32(off + 0x2bc);
        if o < e {
            e = o;
        }
        let cur = m.r16(sh + 0x154);
        m.w16(sh + 0x154, cur.wrapping_add(e as u16));
        let o2 = m.r32(off + 0x2bc);
        m.w32(off + 0x2bc, o2.wrapping_sub(e & 0xffff));
        return finish(m, host, sh);
    }
    let t = u32::from(m.r16(0x672cb4 + w * 2));
    let have = m.r32(off + 0x124 + w * 4) as i32;
    let q1 = have.wrapping_div(t as i32) as u32;
    if m.r32(op + 0x10) > q1 {
        m.w32(op + 0x10, q1);
    }
    let cap = host.call(m, 0x5182b0, sh, &[]) as i32;
    let q2 = cap.wrapping_div(t as i32) as u32;
    if m.r32(op + 0x10) > q2 {
        m.w32(op + 0x10, q2);
    }
    if t < 0x7d0 {
        let mut ecx = 0u32;
        loop {
            if m.r8(sh + ecx + 0x13c) == 0xff && m.r8(sh + ecx + 0x13d) == 7 {
                let n = m.r32(op + 0x10);
                if n == 0 {
                    ecx = 0x18;
                } else {
                    m.w32(op + 0x10, n - 1);
                    slot_add(m, w, t, true);
                    m.w8(sh + ecx + 0x13c, w as u8);
                }
            }
            ecx += 2;
            if ecx >= 0x18 {
                break;
            }
        }
        let mut ecx = 0u32;
        loop {
            if m.r8(sh + ecx + 0x13c) == 0xff {
                let n = m.r32(op + 0x10);
                m.w32(op + 0x10, n.wrapping_sub(1));
                if n == 0 {
                    ecx = 0x18;
                } else {
                    slot_add(m, w, t, true);
                    m.w8(sh + ecx + 0x13c, w as u8);
                }
            }
            ecx += 1;
            if ecx >= 0x18 {
                break;
            }
        }
    } else {
        let mut ecx = 0u32;
        loop {
            if m.r8(sh + ecx + 0x13c) == 0xff && m.r8(sh + ecx + 0x13d) == 0xff {
                let n = m.r32(op + 0x10);
                m.w32(op + 0x10, n.wrapping_sub(1));
                if n == 0 {
                    ecx = 0x18;
                } else {
                    slot_add(m, w, t, true);
                    m.w8(sh + ecx + 0x13c, w as u8);
                    m.w8(sh + ecx + 0x13d, 6);
                }
            }
            ecx += 2;
            if ecx >= 0x18 {
                break;
            }
        }
    }
    finish(m, host, sh)
}

/// 0x0052F030 (ret 0x14): como `max_trade_amount`, pero además escribe en `*a4` un factor `f32` calculado con la FPU. La FPU del juego trabaja a 53 bits: `f64` con redondeo a `f32` en cada `fstp dword`. Devuelve la cantidad (como `0x52EE90`) o `0`.
pub fn max_trade_amount_float(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    fn fl(m: &mut dyn Mem, addr: u32) -> f64 {
        f64::from(f32::from_bits(m.r32(addr)))
    }
    fn f32r(v: f64) -> f64 {
        f64::from(v as f32)
    }
    let base = m.r32(0x6de508);
    let t0 = base.wrapping_add(a[0].wrapping_mul(2552));
    let t1 = base.wrapping_add(a[1].wrapping_mul(2552));
    let w = a[2];
    let p = a[4];
    m.w32(p, 0);
    let w4 = w.wrapping_mul(4);
    let w16 = w.wrapping_mul(16);
    let cap0 = m.r32(t0.wrapping_add(w4).wrapping_add(4)) as i32;
    let lo0p = t0.wrapping_add(w16).wrapping_add(0x4f4);
    let lo0 = m.r32(lo0p) as i32;
    let hi0 = m.r32(t0.wrapping_add(w16).wrapping_add(0x4f0)) as i32;
    let mid = hi0.wrapping_add(lo0) >> 1;
    if mid >= cap0 {
        return 0;
    }
    let r1 = |m: &mut dyn Mem, off: u32| m.r32(t1.wrapping_add(w4).wrapping_add(off)) as i32;
    let x = r1(m, 0x310).wrapping_sub(r1(m, 0xc4)).wrapping_add(r1(m, 0x64)).wrapping_mul(a[3] as i32);
    let mut d = r1(m, 0x670).wrapping_sub(x).wrapping_add(r1(m, 4));
    if d < 0 {
        d = 0;
    }
    let lo1 = m.r32(t1.wrapping_add(w16).wrapping_add(0x4f4)) as i32;
    if lo1 < d {
        return 0;
    }
    let hi1 = m.r32(t1.wrapping_add(w16).wrapping_add(0x4f0)) as i32;
    let q = lo1.wrapping_sub(hi1) >> 3;
    let u = hi1.wrapping_add(q).wrapping_add(q.wrapping_mul(2));
    let c1 = fl(m, 0x673a08);
    let c2 = fl(m, 0x673a0c);
    let val = if d < hi1 {
        let tf = fl(m, this + 0x64);
        let st = (tf - c1) * f32r(f64::from(d));
        let st = st / f32r(f64::from(hi1));
        tf - st
    } else {
        let x = f32r(c1 - c2);
        let tmp = f32r(f64::from(d.wrapping_sub(hi1)));
        let prod = f32r(tmp * x);
        let den = f32r(f64::from(lo1.wrapping_sub(hi1)));
        let qq = f32r(prod / den);
        c1 - qq
    };
    m.w32(p, (val as f32).to_bits());
    let e = cap0.wrapping_sub(lo0);
    let (ebx, ebp, mut s38);
    if e > 0 {
        let v0 = m.r32(t0.wrapping_add(w16).wrapping_add(0x4f8)) as i32;
        let pcur = fl(m, p);
        let out = if cap0 < v0 {
            let c3 = fl(m, 0x6739f0);
            let c4 = fl(m, 0x6739f4);
            let a_ = f32r(c3 - c4);
            let b = f32r(f64::from(v0.wrapping_sub(cap0)));
            let prod = f32r(b * a_);
            let lo0n = m.r32(lo0p) as i32;
            let den = f32r(f64::from(v0.wrapping_sub(lo0n)));
            let qq = f32r(prod / den);
            pcur - (c4 + qq)
        } else {
            let v1 = m.r32(t0.wrapping_add(w16).wrapping_add(0x4fc)) as i32;
            let c4 = fl(m, 0x6739f4);
            let c5 = fl(m, 0x6739f8);
            if cap0 < v1 {
                let a_ = f32r(c4 - c5);
                let b = f32r(f64::from(v1.wrapping_sub(cap0)));
                let prod = f32r(b * a_);
                let den = f32r(f64::from(v1.wrapping_sub(v0)));
                let qq = f32r(prod / den);
                pcur - (c5 + qq)
            } else {
                pcur - c5
            }
        };
        m.w32(p, (out as f32).to_bits());
        let lo1md = lo1.wrapping_sub(d);
        if lo1md < e {
            return lo1md as u32;
        }
        ebx = d.wrapping_add(e);
        ebp = cap0.wrapping_sub(e);
        s38 = e;
    } else {
        let c6 = fl(m, 0x6739ec);
        let c3 = fl(m, 0x6739f0);
        let a_ = f32r(c6 - c3);
        let b = f32r(f64::from(lo0.wrapping_sub(cap0)));
        let sum = f32r(b + a_);
        let den = f32r(f64::from(lo0.wrapping_sub(hi0)));
        let qq = f32r(sum / den);
        let pcur = fl(m, p);
        let v = ((pcur - (c3 + qq)) as f32).to_bits();
        let c7 = fl(m, 0x66bc6c);
        m.w32(p, v);
        if !(c7 < f64::from(f32::from_bits(v))) {
            return 0;
        }
        ebx = d;
        ebp = cap0;
        s38 = 0i32;
    }
    let (mut ebx, mut ebp) = (ebx, ebp);
    let eax = u.wrapping_sub(ebx);
    if eax >= 0 {
        let ecx = ebp.wrapping_sub(mid);
        if eax >= ecx {
            return s38.wrapping_add(ecx) as u32;
        }
        ebp = ebp.wrapping_sub(eax);
        s38 = s38.wrapping_add(eax);
        ebx = ebx.wrapping_add(eax);
    }
    let t = lo1.wrapping_sub(u);
    let num = ebx.wrapping_sub(u).wrapping_sub(mid).wrapping_add(ebp).wrapping_mul(t);
    let den = t.wrapping_sub(mid).wrapping_add(lo0);
    let r = num.wrapping_div(den).wrapping_add(u);
    if r >= ebx {
        r.wrapping_sub(ebx).wrapping_add(s38) as u32
    } else {
        s38 as u32
    }
}

/// 0x00511C60 (ret 4): serializa el banco de nombres en `a0`: 40 pares `(0x41, 0)`, tres bloques de 20 cadenas (tablas de words `[+0xBC]/[+0xC0]/[+0xC4]` desde `word(+0xD0/+0xD2/+0xD6)`, pools `[+0xA8]/[+0xAC]/[+0xB0]`) cada uno precedido de `#`, y la cola de `word(+0xDA)-word(+0xD8)` nombres extra (`[+0xCC]`, pool `[+0xB8]`). Antes de copiar cada cadena pone a cero el byte previo al inicio de la siguiente. Devuelve el final.
pub fn name_bank_serialize(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let mut d = a[0];
    for _ in 0..0x28 {
        m.w8(d, 0x41);
        m.w8(d + 1, 0);
        d = d.wrapping_add(2);
    }
    for (tbl_off, start_off, pool_off) in [(0xbcu32, 0xd0u32, 0xa8u32), (0xc0, 0xd2, 0xac), (0xc4, 0xd6, 0xb0)] {
        for k in 0..0x14u32 {
            let tbl = m.r32(this + tbl_off);
            let start = u32::from(m.r16(this + start_off));
            let nxt = u32::from(m.r16(tbl.wrapping_add((start + k) * 2 + 2)));
            let pool = m.r32(this + pool_off);
            m.w8(pool.wrapping_add(nxt).wrapping_sub(1), 0);
            let tbl = m.r32(this + tbl_off);
            let start = u32::from(m.r16(this + start_off));
            let off = u32::from(m.r16(tbl.wrapping_add((start + k) * 2)));
            let pool = m.r32(this + pool_off);
            let mut s = pool.wrapping_add(off);
            loop {
                let b = m.r8(s);
                m.w8(d, b);
                d = d.wrapping_add(1);
                s = s.wrapping_add(1);
                if b == 0 {
                    break;
                }
            }
        }
        m.w8(d, 0x23);
        d = d.wrapping_add(1);
    }
    let cnt = m.r16(this + 0xda).wrapping_sub(m.r16(this + 0xd8));
    m.w16(d, cnt);
    d = d.wrapping_add(2);
    let t2 = m.r32(this + 0xcc);
    if t2 == 0 {
        m.w8(d, 0);
        m.w8(d + 1, 0);
        return d.wrapping_add(2);
    }
    copy_bytes(m, d, t2.wrapping_add(u32::from(cnt) * 2), 2);
    d = d.wrapping_add(2);
    let l = u32::from(m.r16(t2.wrapping_add(u32::from(cnt) * 2)));
    let pool2 = m.r32(this + 0xb8);
    copy_bytes(m, d, pool2, l);
    d.wrapping_add(l)
}

/// 0x00519B50: copia `byte(+0x38)` en `+0x13B` y llama a `0x5196B0` (si difiere de `byte(+0x13A)`) o `0x5193F0` (si coincide); termina con `0x519920(byte(+0x13B))`, cuyo valor devuelve.
pub fn letter_slot_refresh(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _a: &[u32]) -> u32 {
    let al = m.r8(this + 0x38);
    let cl = m.r8(this + 0x13a);
    m.w8(this + 0x13b, al);
    if al != cl {
        host.call(m, 0x5196b0, this, &[]);
    } else {
        host.call(m, 0x5193f0, this, &[]);
    }
    let b = u32::from(m.r8(this + 0x13b));
    host.call(m, 0x519920, this, &[b])
}

/// 0x00511690 (ret 4): busca en la tabla `[this+4]` (`word(+8)` entradas de 8 bytes) la de `word(+4)` igual a `a0` (16 bits); si su puntero (`+0`) es nulo llama a `0x5116F0(a0, idx)`; incrementa el contador `word(+6)` y devuelve el puntero. `0` si no existe.
pub fn handle_table_find_ref(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let n = u32::from(m.r16(this + 8));
    let tbl = m.r32(this + 4);
    for i in 0..n {
        if m.r16(tbl.wrapping_add(i * 8 + 4)) == a[0] as u16 {
            if m.r32(tbl.wrapping_add(i * 8)) == 0 {
                host.call(m, 0x5116f0, this, &[a[0], i]);
            }
            let tbl = m.r32(this + 4);
            let c = m.r16(tbl.wrapping_add(i * 8 + 6)).wrapping_add(1);
            m.w16(tbl.wrapping_add(i * 8 + 6), c);
            let tbl = m.r32(this + 4);
            return m.r32(tbl.wrapping_add(i * 8));
        }
    }
    0
}

/// 0x0054A420: tras `0x4D8E70(0x14)`, para cada registro `0x5303C0(i)` con `word(+8) == 0` (`word[0x6DE4AA]` entradas) crea una tarea `0x4D8CF0([0x6DE4B4]+i)` con tipo `0x14`, `+8 = 0` y `+0xC = i`. Sin retorno útil.
pub fn seed_default_tasks(m: &mut dyn Mem, host: &mut dyn OpsHost, _this: u32, _a: &[u32]) -> u32 {
    host.call(m, 0x4d8e70, 0x6dd73c, &[0x14]);
    let mut i = 0u32;
    while i < u32::from(m.r16(0x6de4aa)) {
        let rec = host.call(m, 0x5303c0, 0x6de4a0, &[i]);
        if m.r16(rec + 8) == 0 {
            let id = m.r32(0x6de4b4).wrapping_add(i);
            let p = host.call(m, 0x4d8cf0, 0x6dd73c, &[id]);
            m.w16(p + 6, 0x14);
            m.w32(p + 8, 0);
            m.w16(p + 0xc, i as u16);
        }
        i += 1;
    }
    0
}

/// 0x00530780 (ret 4): activa la fila `a0` (20 bytes, `[this+0x70]`) de una lista enlazada por el word `+8` (libre en `word(this+0x7C)`, capacidad `word(this+6)`); si ya figura en la cadena no hace nada. Duplica su contador `word(+4)`, lo descuenta de `+0x2E0`/`+0x2D8+4`... de la ciudad `byte(+7)` y deja en `town+0x2D4` la suma de los cuatro dwords `+0x2D8..+0x2E4`.
pub fn index_chain_activate_and_charge(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let cap = m.r16(this + 6);
    let di = m.r16(this + 0x7c);
    let mut ax = di;
    if ax < cap {
        loop {
            if u32::from(ax) == a[0] {
                return 0;
            }
            let tbl = m.r32(this + 0x70);
            ax = m.r16(tbl.wrapping_add(u32::from(ax) * 20 + 8));
            if ax >= cap {
                break;
            }
        }
    }
    let rec = m.r32(this + 0x70).wrapping_add(a[0].wrapping_mul(20));
    m.w16(rec + 8, di);
    m.w16(this + 0x7c, a[0] as u16);
    let s0 = m.r16(rec + 4);
    if s0 == 0 {
        return 0;
    }
    let cnt = m.r32(this + 0x10);
    let b = u32::from(m.r8(rec + 7));
    if b >= cnt {
        return 0;
    }
    let s1 = s0.wrapping_mul(2);
    m.w16(rec + 4, s1);
    let tb = m.r32(this + 0x68);
    let town = tb.wrapping_add(b.wrapping_mul(2552));
    let ebx = m.r32(town + 0x2e0) as i32;
    let p = town + 0x2d8;
    let edi = i32::from(s1);
    if ebx >= edi {
        m.w32(p + 8, ebx.wrapping_sub(edi) as u32);
    } else {
        let s2 = s1.wrapping_mul(2);
        let s3 = s2.wrapping_sub(ebx as u16);
        m.w16(rec + 4, s3);
        m.w32(p + 8, 0);
    }
    let e = m.r32(p + 4) as i32;
    let s = i32::from(m.r16(rec + 4));
    m.w32(p + 4, if e >= s { e.wrapping_sub(s) as u32 } else { 0 });
    let b2 = u32::from(m.r8(rec + 7));
    let tb2 = m.r32(this + 0x68);
    m.w32(tb2.wrapping_add(b2.wrapping_mul(2552)) + 0x2d4, 0);
    for j in 0..4u32 {
        let b3 = u32::from(m.r8(rec + 7));
        let t = m.r32(this + 0x68).wrapping_add(b3.wrapping_mul(2552));
        let v = m.r32(t + 0x2d8 + 4 * j);
        let q = m.r32(t + 0x2d4);
        m.w32(t + 0x2d4, q.wrapping_add(v));
    }
    0
}

/// 0x00539B50: reinicia la ranura de una ruta (`[0x6DD7A8] + word(this+8)*60`) si apunta a un barco válido (`0x519BA0` si `word(+0x12) < 0xF`): `this` pasa a `+4 = barco`, `+8 = 0xFF`, `+0xC = 0x76`, `+0x10 = 0x640` (`0x537670`), limpia `+0x24` (16 bytes a `0xFF`) y `+0x34` (4 a cero), cobra el importe anterior `[this+4]` (como mucho el saldo de `0x5303C0(byte(+0))`, nunca negativo) y lo suma a `+0x20`. Sube `byte[0x6DE52B]` a `0x76`. Sin retorno útil.
pub fn route_slot_reset_and_charge(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _a: &[u32]) -> u32 {
    let ax = m.r16(this + 8);
    if ax >= m.r16(0x6dd896) {
        return 0;
    }
    let rec = m.r32(0x6dd7a8).wrapping_add(u32::from(ax) * 60);
    let j = m.r16(rec + 0x10);
    if j >= m.r16(0x6dd894) {
        return 0;
    }
    if m.r16(rec + 0x12) < 0xf {
        let obj = m.r32(0x6dd7a4).wrapping_add(u32::from(j) * 0x180);
        if (host.call(m, 0x519ba0, obj, &[]) & 0xff) == 0 {
            return 0;
        }
    }
    let mut ebx = m.r32(this + 4) as i32;
    let bp = m.r16(this + 0xa);
    let jj = u32::from(m.r16(rec + 0x10));
    m.w32(this + 4, jj);
    m.w32(this + 8, 0xff);
    m.w32(this + 0xc, 0x76);
    m.w32(this + 0x10, 0x640);
    host.call(m, 0x537670, this, &[]);
    m.w16(rec + 0x38, bp);
    for k in 0..0x10 {
        m.w8(rec + 0x24 + k, 0xff);
    }
    for k in 0..4 {
        m.w8(rec + 0x34 + k, 0);
    }
    let owner = u32::from(m.r8(rec));
    if (owner as u16) < m.r16(0x6de4aa) {
        let p = host.call(m, 0x5303c0, 0x6de4a0, &[owner]);
        let ecx = m.r32(p) as i32;
        if ebx > ecx {
            ebx = if ecx > 0 { ecx } else { 0 };
        }
        m.w32(p, ecx.wrapping_sub(ebx) as u32);
        let al = m.r8(this + 0xc);
        if al > m.r8(0x6de52b) {
            m.w8(0x6de52b, al);
        }
    }
    let v = m.r32(rec + 0x20).wrapping_add(ebx as u32);
    m.w32(rec + 0x20, v);
    0
}

/// 0x00534F10 (ret 0x10): elige dos words de salida (`a0`, `a1`) para la semilla `a3`. Con `a2 != 0` busca en los 15 registros de 44 bytes de `0x6DEFB4` (desde `a3&0xFF % 15`, con vuelta completa) uno con `[0x6DE4B4] > dword(+4)` y `byte(+0) < 0x1E` y, si no, uno solo con `byte(+0) < 0x1E`; devuelve entonces los words de `0x673858 + byte*4`. Si no, recorre el bitmap `this` desde `0x64 + (a3&0xFF) % ([0x6DF2E4]-0x64)` hasta `[0x6DF2E4]` (y una vuelta desde `0x64`) buscando un bit apagado y devuelve los words de `[0x6DF2E0] + idx*4`; sin hueco, `0xFFFF` en ambos.
pub fn slot_pick_with_bitmap(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let (out0, out1, flag, seed) = (a[0], a[1], a[2] & 0xff != 0, a[3]);
    if flag {
        let edi = m.r32(0x6de4b4);
        let s = (seed & 0xff) % 15;
        let mut found = None;
        let mut eax = s;
        let mut wrapped = false;
        loop {
            let rec = 0x6defb4 + 44 * eax;
            if edi > m.r32(rec + 4) && m.r8(rec) < 0x1e {
                found = Some(eax);
                break;
            }
            eax += 1;
            if eax < 15 {
                continue;
            }
            if !wrapped {
                eax = 0;
                wrapped = true;
                continue;
            }
            break;
        }
        if found.is_none() {
            let mut eax = s;
            loop {
                if m.r8(0x6defb4 + 44 * eax) < 0x1e {
                    found = Some(eax);
                    break;
                }
                eax += 1;
                if eax >= 15 {
                    break;
                }
            }
        }
        if let Some(i) = found {
            let b = u32::from(m.r8(0x6defb4 + 44 * i));
            let w0 = m.r16(0x673858 + b * 4);
            m.w16(out0, w0);
            let w1 = m.r16(0x673858 + b * 4 + 2);
            m.w16(out1, w1);
            return 0;
        }
    }
    let esi = m.r32(0x6df2e4);
    let mut edx = (seed & 0xff) % esi.wrapping_sub(0x64) + 0x64;
    let mut second = false;
    loop {
        let bit = m.r32(this.wrapping_add((edx >> 5) * 4)) & (1u32 << (edx & 31));
        if bit == 0 {
            let tb = m.r32(0x6df2e0);
            let w0 = m.r16(tb.wrapping_add(edx * 4));
            m.w16(out0, w0);
            let w1 = m.r16(tb.wrapping_add(edx * 4 + 2));
            m.w16(out1, w1);
            return 0;
        }
        edx += 1;
        if edx < esi {
            continue;
        }
        if second {
            m.w16(out0, 0xffff);
            m.w16(out1, 0xffff);
            return 0;
        }
        second = true;
        edx = 0x64;
    }
}

/// 0x00541200: recorre las entidades `[[tabla 0x6E55D0 + byte(+4)*4]+4]` de los índices `byte(+5)*64 ..< byte(+5)*64+32` (máscara `dword(+0x10)`) y `byte(+5)*64+32 ..< (byte(+5)+1)*64` (máscara `dword(+0xC)`); a cada entidad marcada con su bit la pasa por `0x6270C7(word(+6), word(+8))` y limpia su bit 0 de `+0x12A` (y pone `word(+0x128) = 0xFFFF` si el bit 3 está apagado). Sin retorno útil.
pub fn entity_mask_clear_flags(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _a: &[u32]) -> u32 {
    fn body(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, mask_off: u32, e: u32) {
        let bit = 1u32 << (e & 0x1f);
        if m.r32(this + mask_off) & bit == 0 {
            return;
        }
        let kind = u32::from(m.r8(this + 4));
        let obj = m.r32(0x6e55d0 + kind * 4);
        let arr = m.r32(obj + 4);
        let edi = m.r32(arr.wrapping_add(e * 4));
        if edi == 0 {
            return;
        }
        let w8 = u32::from(m.r16(this + 8));
        let w6 = u32::from(m.r16(this + 6));
        host.call(m, 0x6270c7, edi + 4, &[w6, w8]);
        let al = m.r8(edi + 0x12a);
        if al & 8 == 0 {
            m.w16(edi + 0x128, 0xffff);
        }
        m.w8(edi + 0x12a, al & 0xfe);
    }
    let kind = u32::from(m.r8(this + 4));
    if m.r32(0x6e55d0 + kind * 4) == 0 {
        return 0;
    }
    if m.r32(this + 0x10) != 0 {
        let mut e = u32::from(m.r8(this + 5)) * 64;
        loop {
            if e >= u32::from(m.r8(this + 5)) * 64 + 0x20 {
                break;
            }
            body(m, host, this, 0x10, e);
            e = (e + 1) & 0xffff;
            if e >= u32::from(m.r8(this + 5)) * 64 + 0x20 {
                break;
            }
        }
    }
    if m.r32(this + 0xc) == 0 {
        return 0;
    }
    let c5 = u32::from(m.r8(this + 5));
    let mut e = (c5 * 64 + 0x20) & 0xffff;
    if e >= (c5 + 1) * 64 {
        return 0;
    }
    loop {
        body(m, host, this, 0xc, e);
        e = (e + 1) & 0xffff;
        if e >= (u32::from(m.r8(this + 5)) + 1) * 64 {
            break;
        }
    }
    0
}

/// 0x00612A20: inserta o actualiza `(clave, byte)` en el cubo `idx` de una tabla de cubos (`this+0xC30` = tabla de 12 bytes por cubo `{datos, capacidad, cuenta}`, `this+0xC34` = número de cubos). Si `idx` está fuera, crece la tabla de 100 en 100 (realloc 0x63A29F) y pone a cero los cubos nuevos. Si la clave existe en el cubo, cambia su byte y devuelve el índice; si no, la añade al final (ampliando de 10 en 10) y devuelve la dirección de la cuenta del cubo.
pub fn bucket_table_insert_key(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let idx = a[0];
    let key = a[1];
    let val = a[2] as u8;
    let nb = m.r32(this + 0xc34);
    if idx < nb {
        let tb = m.r32(this + 0xc30);
        let e = tb.wrapping_add(idx.wrapping_mul(12));
        let cnt = m.r32(e + 8);
        if cnt != 0 {
            let data = m.r32(e);
            let mut i = 0u32;
            while i < cnt {
                if m.r32(data.wrapping_add(i * 8)) == key {
                    m.w8(data.wrapping_add(i * 8 + 4), val);
                    return i;
                }
                i += 1;
            }
        }
    } else {
        let d = idx.wrapping_sub(nb).wrapping_add(100);
        let grow = d.wrapping_sub(d % 100);
        let nnb = nb.wrapping_add(grow);
        m.w32(this + 0xc34, nnb);
        let old = m.r32(this + 0xc30);
        let p = host.call(m, 0x63a29f, 0, &[old, nnb.wrapping_mul(12)]);
        m.w32(this + 0xc30, p);
        let mut i = nnb.wrapping_sub(grow);
        while i < m.r32(this + 0xc34) {
            let e = p.wrapping_add(i.wrapping_mul(12));
            m.w32(e + 8, 0);
            m.w32(e + 4, 0);
            m.w32(e, 0);
            i += 1;
        }
    }
    let tb = m.r32(this + 0xc30);
    let e = tb.wrapping_add(idx.wrapping_mul(12));
    let cap = m.r32(e + 4);
    if m.r32(e + 8) == cap {
        m.w32(e + 4, cap.wrapping_add(10));
        let ncap = m.r32(e + 4);
        let data = m.r32(e);
        let p = host.call(m, 0x63a29f, 0, &[data, ncap << 3]);
        m.w32(e, p);
    }
    let data = m.r32(e);
    let cnt = m.r32(e + 8);
    m.w32(data.wrapping_add(cnt.wrapping_mul(8)), key);
    m.w8(data.wrapping_add(cnt.wrapping_mul(8) + 4), val);
    m.w32(e + 8, cnt.wrapping_add(1));
    e + 8
}

/// 0x00612B80: devuelve AL=1 si la clave `a[1]` no está en el cubo `a[0]` de la tabla de cubos de `this+0xC30` (cero si el índice supera el número de cubos o el cubo está vacío), y AL=0 si existe.
pub fn bucket_table_key_missing(m: &mut dyn Mem, _host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let idx = a[0];
    if idx > m.r32(this + 0xc34) {
        return 1;
    }
    let tb = m.r32(this + 0xc30);
    let e = tb.wrapping_add(idx.wrapping_mul(12));
    let cnt = m.r32(e + 8);
    if cnt == 0 {
        return 1;
    }
    let data = m.r32(e);
    let mut i = 0u32;
    while i < cnt {
        if m.r32(data.wrapping_add(i * 8)) == a[1] {
            return 0;
        }
        i += 1;
    }
    1
}

/// 0x00618F90: destruye una lista indexada: recorre la cadena de `u16` (`+0x10` inicio, `+0xC` siguientes), destruye (0x61A6D0) y libera cada elemento de `+4`, libera las tres tablas, libera las nueve tablas auxiliares de `+0x18..` y las tres de `+0x48/4C/50` por elemento, y deja todos los campos a cero/0xFFFF. Sin retorno útil.
pub fn shipyard_list_destroy(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, _a: &[u32]) -> u32 {
    const FREE_CRT: u32 = 0x63a1b6;
    let mut di = u32::from(m.r16(this + 0x10));
    while di != 0xffff {
        let tb = m.r32(this + 4);
        let ebx = m.r32(tb.wrapping_add(di * 4));
        if ebx != 0 {
            host.call(m, 0x61a6d0, ebx, &[]);
            host.call(m, FREE, 0, &[ebx]);
        }
        let nx = m.r32(this + 0xc);
        di = u32::from(m.r16(nx.wrapping_add(di * 2)));
    }
    if m.r32(this + 4) != 0 {
        for o in [4u32, 8, 0xc] {
            let v = m.r32(this + o);
            host.call(m, FREE_CRT, 0, &[v]);
        }
    }
    if m.r32(this + 0x18) != 0 {
        for o in [0x18u32, 0x1c, 0x24, 0x28, 0x2c, 0x30, 0x38, 0x3c, 0x40] {
            let v = m.r32(this + o);
            host.call(m, FREE_CRT, 0, &[v]);
        }
        let cnt = m.r16(this + 0x5a);
        if cnt != 0 {
            let mut b = 0u32;
            loop {
                let d = (b & 0xffff) * 4;
                for o in [0x48u32, 0x4c, 0x50] {
                    let t = m.r32(this + o);
                    let v = m.r32(t.wrapping_add(d));
                    host.call(m, FREE_CRT, 0, &[v]);
                }
                b = b.wrapping_add(1);
                if (b & 0xffff) >= u32::from(m.r16(this + 0x5a)) {
                    break;
                }
            }
        }
        for o in [0x48u32, 0x4c, 0x50] {
            let v = m.r32(this + o);
            host.call(m, FREE_CRT, 0, &[v]);
        }
    }
    for o in [4u32, 8, 0xc, 0x18, 0x1c, 0x24, 0x28, 0x2c, 0x30, 0x38, 0x3c, 0x40, 0x48, 0x4c, 0x50] {
        m.w32(this + o, 0);
    }
    for o in [0x10u32, 0x12, 0x20, 0x34, 0x44] {
        m.w16(this + o, 0xffff);
    }
    for o in [0x14u32, 0x16, 0x54, 0x56, 0x58, 0x5a] {
        m.w16(this + o, 0);
    }
    0
}

/// 0x005008C0: busca en la cadena de registros de la ciudad `byte(this+0x2C6)` (`0x51F1F0`, siguiente en `word(+4)`, con segunda vuelta desde `word(town+0x762)`) uno cuyo tipo/subtipo/clase (`a[1]`, `a[2]`, `word(this+0x2C4)`) coincidan. Rellena el bloque `a[0]` (+0x20 cuenta de registros con igual `+3`, +0x22, +0x23, +0x24 plazo) y devuelve 1; devuelve 0 si no hay coincidencia. Si el registro no tiene `+6`, recorre la cadena de `0x5303B0` hasta una entrada con `byte(+6)` igual y la entrega a `0x4D3B40`.
pub fn office_find_matching_slot(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let out = a[0];
    let town = m.r32(0x6de508).wrapping_add(u32::from(m.r8(this + 0x2c6)) * 0x9f8);
    let mut ebp = u32::from(m.r16(this + 0x2ce));
    let mut flag = false;
    let mut esi: u32 = 0;
    loop {
        let n = u32::from(m.r16(town + 0x776));
        if ebp >= n {
            if flag {
                return 0;
            }
            let bp = u32::from(m.r16(town + 0x762));
            if bp >= n {
                return 0;
            }
            ebp = bp;
            flag = true;
        }
        esi = host.call(m, 0x51f1f0, town, &[ebp]);
        let ok = u32::from(m.r8(esi)) == a[1]
            && u32::from(m.r8(esi + 1)) == a[2]
            && m.r16(this + 0x2c4) == u16::from(m.r8(esi + 2));
        if ok {
            let c4 = m.r16(this + 0x2c4) as u8;
            m.w8(out + 0x23, c4);
            let b3 = m.r8(esi + 3);
            m.w8(out + 0x22, b3);
            m.w16(out + 0x20, 0);
            let mut eax = u32::from(m.r16(this + 0x2ce));
            while eax < u32::from(m.r16(town + 0x776)) {
                esi = host.call(m, 0x51f1f0, town, &[eax]);
                let dl = m.r8(esi + 3);
                eax = u32::from(m.r16(esi + 4));
                if dl == m.r8(out + 0x22) {
                    let c = m.r16(out + 0x20).wrapping_add(1);
                    m.w16(out + 0x20, c);
                }
            }
            let r = host.call(m, 0x51f1f0, town, &[ebp]);
            if m.r8(r + 6) != 0 {
                let v = host.call(m, 0x51d6f0, town, &[ebp]);
                m.w32(out + 0x24, v);
                return 1;
            }
            m.w32(out + 0x24, 0);
            let mut edx = u32::from(m.r16(this + 0x2cc));
            while edx < u32::from(m.r16(0x6de4a6)) {
                let e = host.call(m, 0x5303b0, 0x6de4a0, &[edx]);
                if m.r8(e + 6) == m.r8(out + 0x22) {
                    host.call(m, 0x4d3b40, e, &[out, this]);
                    return 1;
                }
                edx = u32::from(m.r16(e + 8));
            }
        }
        ebp = u32::from(m.r16(esi + 4));
    }
}

/// 0x0050BA80: recorre la cadena de registros `0x4D8DB0` desde `dword(0x6DD744)` (límite `dword(0x6DD748)`) buscando los de tipo 0x24 cuyo `byte(+0x14)` iguale `byte(+0x39)` de la entrada `a[0]` de `this+4` (paso 0x180); en la lista libre de `this+0xEC` (nodos de 60 bytes en `this+8`, enlace `word(+8)`) separa el nodo con `word(+0x12)==9` y `word(+0x1C)` igual, que recuerda en `ebp`. Si no hay ninguno devuelve 0. Con `0x611CD0` ≥ 0xFF devuelve el nodo a la lista libre y 0; si no, toma un nodo de `this+0xE4` (llamando a `0x509AE0` si hace falta), lo inicializa (`0x501680`) y llama a `0x605420`; devuelve 1.
pub fn slot_release_matching(m: &mut dyn Mem, host: &mut dyn OpsHost, this: u32, a: &[u32]) -> u32 {
    let arg = a[0];
    let mut ebp: u32 = 0xffff;
    let mut cur = m.r32(0x6dd744) & 0xffff;
    while cur < (m.r32(0x6dd748) & 0xffff) {
        let eax = host.call(m, 0x4d8db0, 0x6dd73c, &[cur]);
        let ecx;
        let towns = m.r32(this + 4);
        let want = m.r8(towns.wrapping_add(arg.wrapping_mul(0x180) + 0x39));
        if m.r16(eax + 6) == 0x24 && m.r8(eax + 0x14) == want {
            let mut cx = u32::from(m.r16(this + 0xec));
            let mut edi = this + 0xec;
            while cx < u32::from(m.r16(this + 0xf6)) {
                let ebx = m.r32(this + 8);
                let rec = ebx.wrapping_add(cx.wrapping_mul(60));
                if m.r16(rec + 0x12) == 9 && u32::from(m.r16(rec + 0x1c)) == u32::from(m.r8(eax + 0x14)) {
                    ebp = cx;
                    let nx = m.r16(rec + 8);
                    m.w16(edi, nx);
                    m.w16(rec + 8, 0xffff);
                }
                cx = u32::from(m.r16(rec + 8));
                edi = rec + 8;
            }
            ecx = 0xffffu32;
        } else {
            ecx = u32::from(m.r16(eax + 4));
        }
        cur = ecx;
        if cur >= (m.r32(0x6dd748) & 0xffff) {
            break;
        }
    }
    if ebp >= u32::from(m.r16(this + 0xf6)) {
        return 0;
    }
    let r = host.call(m, 0x611cd0, 0x6e55d0, &[]);
    let ebx = r & 0xff;
    if ebx >= 0xff {
        let lb = m.r32(this + 8);
        let rec = lb.wrapping_add(ebp.wrapping_mul(60));
        let w = m.r16(this + 0xec);
        m.w16(rec + 8, w);
        m.w16(this + 0xec, ebp as u16);
        return 0;
    }
    let edi = m.r32(this + 4).wrapping_add(arg.wrapping_mul(0x180));
    if m.r16(edi + 8) >= m.r16(this + 0xf6) {
        host.call(m, 0x509ae0, this, &[]);
        let e4 = m.r16(this + 0xe4);
        m.w16(edi + 8, e4);
        let lb = m.r32(this + 8);
        let rec = lb.wrapping_add(u32::from(e4) * 60);
        let dx = m.r16(rec + 8);
        m.w16(this + 0xe4, dx);
        m.w16(rec + 8, 0xffff);
        m.w16(edi + 0x134, 0x14);
        host.call(m, 0x501680, rec, &[arg, 0x10]);
    }
    let lb = m.r32(this + 8);
    let w = u32::from(m.r16(lb.wrapping_add(ebp.wrapping_mul(60) + 0x10)));
    let obj = m.r32(0x6e55d0 + ebx * 4);
    host.call(m, 0x605420, obj, &[arg, w, 0, 0]);
    1
}
