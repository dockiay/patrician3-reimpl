//! Orden del registro de oficina de la versión 0x85 (`office_serialize`/`office_deserialize`).
//! La representación guardada tiene el mismo tamaño que memoria, pero un orden diferente.

pub const OFFICE_LEN: usize = 0x44c;
/// Lee exclusivamente el registro actual de versión 0x85; el llamante valida la versión.
pub fn decode_office(saved: &[u8; OFFICE_LEN]) -> [u8; OFFICE_LEN] {
    crate::layout::image(&crate::layout::OFFICE, OFFICE_LEN, saved).try_into().expect("tamaño fijo")
}
/// Ordena el registro de memoria como `office_serialize` sin modificar los campos.
pub fn encode_office(memory: &[u8; OFFICE_LEN]) -> [u8; OFFICE_LEN] {
    let mut saved=[0; OFFICE_LEN];
    for seg in &crate::layout::OFFICE {
        let s=seg.pos as usize;let m=seg.obj as usize;let n=seg.len as usize;
        saved[s..s+n].copy_from_slice(&memory[m..m+n]);
    }
    saved
}
