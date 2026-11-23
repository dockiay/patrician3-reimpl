//! Carta 0x1B de `0x00519D90`: un barco humano se pierde en el mar y su dueño recibe el aviso.
//!
//! La función original compone un registro de 16 bytes (constructor `0x0051A850`: bytes 0..4 = `00 80 FF FF`, 6 y 7 a cero)
//! y, solo si `merchant+8 == 0`, lo entrega con `0x004D6530`. Paridad x86 en `tests/x86_ship_lost_letter.rs`.

/// Registro de la carta o `None` si el comerciante no es humano (`merchant+8 != 0`).
/// `captains` es la tabla de capitanes (16 bytes por entrada).
pub fn ship_lost_letter(ship: &[u8], merchant: &[u8], captains: &[u8]) -> Option<[u8; 16]> {
    if u16::from_le_bytes([merchant[8], merchant[9]]) != 0 {
        return None;
    }
    let r16 = |o: usize| u16::from_le_bytes([ship[o], ship[o + 1]]);
    let x = u16::from(ship[0xE] & 3).wrapping_add(r16(0x1E) << 2);
    let mut y = r16(0x22) & 0x7FF;
    let mut z = r16(0x15E) & 0x3FFF;
    let mut mark = 0xFFu8;
    let captain = usize::from(r16(0x42));
    if captain < captains.len() / 16 {
        let c = &captains[captain * 16..captain * 16 + 16];
        y = y.wrapping_add(u16::from(c[2]) << 11);
        z = z.wrapping_add((u16::from(c[2]) & 0xFFE0) << 9);
        mark = c[3];
    }
    let mut record = [0u8; 16];
    record[1] = 0x80;
    record[2] = 0xFF;
    record[3] = 0xFF;
    record[4] = 0x1B;
    record[5] = merchant[0x19];
    record[8..10].copy_from_slice(&x.to_le_bytes());
    record[10..12].copy_from_slice(&y.to_le_bytes());
    record[12..14].copy_from_slice(&z.to_le_bytes());
    record[14] = mark;
    record[15] = ship[0x40];
    Some(record)
}
