#!/usr/bin/env python3
"""Genera p3-render/src/ret_pop.rs: para cada función del catálogo cuyos `ret` alcanzables coinciden y retiran argumentos, las palabras que retira
(`ret N` → N/4). El anfitrión de J08 las usa en `Host::callee_pop` para las llamadas indirectas cuyo `ret N` el generador no ve.
Uso (desde reimpl/): python tools/ret_pop_table.py /tmp/Patrician3.exe ../research/catalog/functions.tsv"""
import csv, sys
sys.path.insert(0, 'oracle')
from lift_x86 import Image
img = Image(sys.argv[1])
rows = []
for r in csv.DictReader(open(sys.argv[2], encoding='utf-8'), delimiter='\t'):
    a = int(r['address'], 16)
    try:
        cfg = img._cfg_rets(a)
    except Exception:
        cfg = None
    if cfg is not None and len(cfg) == 1:
        n = next(iter(cfg))
        if n:
            rows.append((a, n))
rows.sort()
TAIL = r'''/// Primera fila de [`RET_POP`] de cada tramo de 256 direcciones (`dirección >> 8`), más un centinela: la búsqueda se queda en
/// las pocas filas del tramo en lugar de bisecar la tabla entera en cada llamada del enlazador.
fn buckets() -> &'static [u16] {
    static BUCKETS: std::sync::OnceLock<Vec<u16>> = std::sync::OnceLock::new();
    BUCKETS.get_or_init(|| {
        let first = RET_POP[0].0 >> 8;
        let last = RET_POP[RET_POP.len() - 1].0 >> 8;
        let mut out = vec![0u16; (last - first + 2) as usize];
        let mut row = 0usize;
        for (i, slot) in out.iter_mut().enumerate() {
            while row < RET_POP.len() && ((RET_POP[row].0 >> 8) - first) < i as u32 {
                row += 1;
            }
            *slot = row as u16;
        }
        out
    })
}

/// Palabras de pila que retira el destino `target` al volver (0 si es cdecl o desconocido).
pub fn ret_pop(target: u32) -> u32 {
    let b = buckets();
    let Some(i) = (target >> 8).checked_sub(RET_POP[0].0 >> 8).map(|i| i as usize).filter(|i| i + 1 < b.len()) else { return 0 };
    RET_POP[usize::from(b[i])..usize::from(b[i + 1])].iter().find(|e| e.0 == target).map_or(0, |e| u32::from(e.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bucket_lookup_matches_binary_search_for_every_row_and_its_neighbours() {
        let slow = |t: u32| RET_POP.binary_search_by_key(&t, |e| e.0).map_or(0, |i| u32::from(RET_POP[i].1));
        for &(a, _) in RET_POP.iter() {
            for t in [a.wrapping_sub(1), a, a + 1, a + 0x100] {
                assert_eq!(ret_pop(t), slow(t), "{t:#x}");
            }
        }
        for t in [0, 0x400000, 0x4011ef, 0x65adbd, 0x65ae00, 0x66a000, u32::MAX] {
            assert_eq!(ret_pop(t), slow(t), "{t:#x}");
        }
    }
}
'''

out = ['// Generado por tools/ret_pop_table.py: (dirección, palabras que retira el `ret N` de la función). No editar a mano.',
       f'pub static RET_POP: [(u32, u8); {len(rows)}] = [']
out += [f'    ({a:#x}, {n}),' for a, n in rows]
out.append('];')
out.append('')
# Búsqueda por tramos de 256 direcciones y su prueba contra la bisección (WEB-6): se copia tal cual.
out.append(TAIL.rstrip('\n'))
open('p3-render/src/ret_pop.rs', 'w').write('\n'.join(out) + '\n')
print(len(rows), 'funciones con ret N')
