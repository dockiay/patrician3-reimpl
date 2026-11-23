#!/usr/bin/env python3
"""Añade a reimpl/functions.tsv las funciones de tests/fixtures/lifted_real_passed.txt (trazas x86 sobre volcados reales, oracle/snaptrace.py)
que aún no tienen fila. Uso: real_rows.py TAREA  (desde la raíz del repo o reimpl/)."""
import csv, json, sys, collections
from pathlib import Path
R = Path(__file__).resolve().parents[1]
task = sys.argv[1]
passed = [l.split() for l in (R/'p3-core/tests/fixtures/lifted_real_passed.txt').read_text().splitlines() if l.strip()]
cat = {r['name']: r for r in csv.DictReader(open(R.parent/'research/catalog/functions.tsv', encoding='utf-8'), delimiter='\t')}
byname = collections.defaultdict(list)
for r in cat.values(): byname[r['name']].append(r)
path = R/'functions.tsv'; txt = path.read_text(); have = {l.split('\t')[0].lower() for l in txt.splitlines()}
counts = {}
rows = []
for mod, name in passed:
    fx = R/'p3-core/tests/fixtures'/('lifted_real1.json' if mod == 'real1' else f'lifted_r_{mod}.json')
    if fx not in counts: counts[fx] = collections.Counter(c['fn'] for c in json.load(open(fx))['cases'])
    n = counts[fx][name]
    spec = json.load(open(R/'oracle'/f'lifted_{mod}.json'))
    f = next((f for f in spec['funcs'] if f['name'] == name), None)
    if not f: continue
    a = '0x%08X' % f['addr']
    if a.lower() in have or n < 3: continue
    ind = ' Llamadas virtuales: cuenta de argumentos deducida del patrón de pila, no contra el método real.' if f.get('indirect') else ''
    note = (f"Portada con reimpl/tools/lift.py (p3_core::x86rt); {n} trazas x86 sobre volcados reales del juego inglés (oracle/snaptrace.py: objeto, vtables y globales del volcado; "
            f"si el volcado no tiene instancia viva, objeto construido con la vtable original, no capturado); llamadas como ganchos; registros y bytes escritos idénticos.{ind}")
    rows.append('\t'.join([a, 'probada_emulador', f'p3_core::lifted::{mod}::{name}', task, 'reimpl/p3-core/tests/x86_lifted.rs#lifted_real_match_x86', note]))
if not txt.endswith('\n'): txt += '\n'
path.write_text(txt + '\n'.join(rows) + ('\n' if rows else ''))
print(len(rows), 'filas añadidas')
