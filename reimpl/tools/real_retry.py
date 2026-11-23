#!/usr/bin/env python3
"""Segunda pasada para funciones cuya traza sobre volcados reales no coincide por la cuenta de argumentos de una llamada (nover):
junta los `nover` observados por snaptrace (/tmp/w/report_tmp_*_FUNC.json), vuelve a elevar solo esas funciones en un módulo nuevo y
deja la lista de nombres para trazarlas otra vez. Uso: real_retry.py MODULO_NUEVO NOMBRE... (nombres de funciones, ya elevadas en otro módulo)."""
import csv, glob, json, subprocess, sys, collections
from pathlib import Path
R = Path(__file__).resolve().parents[1]
mod, names = sys.argv[1], sys.argv[2:]
import os
CATP = os.environ.get('CAT', str(R.parent/'research/catalog/functions.tsv'))
cat = {r['name']: r for r in csv.DictReader(open(CATP, encoding='utf-8'), delimiter='\t')}
nov = {}
for n in names:
    votes = collections.defaultdict(collections.Counter)
    for p in glob.glob(f'/tmp/w/report_tmp_*_{n}.json') + glob.glob('/tmp/w/report_r_*.json'):
        d = json.load(open(p)).get(n, [])
        for rep in d:
            for site, k in (rep.get('nover') or {}).items(): votes[site][k] += 1
    if votes: nov[n] = {site: c.most_common(1)[0][0] for site, c in votes.items()}
# los sitios observados en pasadas anteriores se conservan (cada pasada solo ve los que aún difieren)
ACC = Path('/tmp/nover_acc.json'); acc = json.load(open(ACC)) if ACC.exists() else {}
for n in names:
    acc.setdefault(n, {}).update(nov.get(n, {}))
    if acc[n]: nov[n] = dict(acc[n])
json.dump(acc, open(ACC, 'w'))
json.dump(nov, open('/tmp/nover_retry.json', 'w'))
addrs = [cat[n]['address'][2:].lstrip('0') for n in names]
r = subprocess.run([sys.executable, str(R/'tools/lift.py'), '--binary', '/tmp/Patrician3.exe', '--catalog', CATP,
                    '--lax', '--ext', '--guess', *(['--localargs'] if os.environ.get('LOCALARGS') else []), *(['--fpu-exit'] if os.environ.get('FPUEXIT') else []), '--nover', '/tmp/nover_retry.json', '--out', str(R/f'p3-core/src/lifted/{mod}.rs'), '--spec', str(R/f'oracle/lifted_{mod}.json')] + [x for sz in os.environ.get('SIZES','').split(',') if sz for x in ('--size', sz)] + addrs, capture_output=True, text=True)
print(r.stdout[-500:], r.stderr[-300:])
print({n: nov.get(n) for n in names})
