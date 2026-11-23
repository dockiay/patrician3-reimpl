//! Lista libre del gestor de instalaciones (`0x005306B0`/`0x00530780`).
use crate::town::{off, Town};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FacilityPool {
    pub records: Vec<[u8; 20]>,
    pub free: u16,
}
fn get(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}
fn put(b: &mut [u8], at: usize, v: u16) {
    b[at..at + 2].copy_from_slice(&v.to_le_bytes())
}
impl FacilityPool {
    /// Reserva una ranura, ampliando de 0 a 128 o de 64 en 64 como el original.
    pub fn allocate(&mut self) -> Result<u16, String> {
        let old = self.records.len();
        if old > u16::MAX as usize {
            return Err("capacidad de negocios".into());
        }
        if usize::from(self.free) >= old {
            let count = if old < 20 {
                128
            } else {
                old.checked_add(64)
                    .filter(|x| *x <= u16::MAX as usize)
                    .ok_or("capacidad de negocios")?
            };
            self.records.resize(count, [0xff; 20]);
            for i in old..count {
                put(&mut self.records[i], 8, (i + 1) as u16);
            }
            self.free = old as u16;
        }
        let i = self.free;
        self.free = get(&self.records[i as usize], 8);
        put(&mut self.records[i as usize], 8, 0xffff);
        Ok(i)
    }
    /// Devuelve la ranura y retira su plantilla restante de la población urbana.
    /// Devuelve `false` si la ranura ya figuraba en la lista libre.
    pub fn release(&mut self, index: u16, towns: &mut [Town]) -> Result<bool, String> {
        if usize::from(index) >= self.records.len() {
            return Err("índice de negocio".into());
        }
        let mut cur = self.free;
        let mut seen = vec![false; self.records.len()];
        while usize::from(cur) < self.records.len() {
            if cur == index {
                return Ok(false);
            }
            if seen[cur as usize] {
                return Err("ciclo en lista libre de negocios".into());
            }
            seen[cur as usize] = true;
            cur = get(&self.records[cur as usize], 8);
        }
        let f = &mut self.records[index as usize];
        put(f, 8, self.free);
        self.free = index;
        let n = get(f, 4);
        let city = usize::from(f[7]);
        if n == 0 || city >= towns.len() {
            return Ok(true);
        }
        let town = &mut towns[city];
        let mut people = n.wrapping_mul(2);
        put(f, 4, people);
        let poor = town.pop(2);
        if poor < i32::from(people) {
            people = people.wrapping_mul(2).wrapping_sub(poor as u16);
            put(f, 4, people);
            town.set_pop(2, 0)
        } else {
            town.set_pop(2, poor - i32::from(people))
        }
        let middle = town.pop(1);
        let people = get(f, 4);
        town.set_pop(
            1,
            if middle < i32::from(people) {
                0
            } else {
                middle - i32::from(people)
            },
        );
        let total = (0..4).fold(0i32, |v, g| v.wrapping_add(town.pop(g)));
        town.set_i32(off::CITIZENS_TOTAL, total);
        Ok(true)
    }
}
