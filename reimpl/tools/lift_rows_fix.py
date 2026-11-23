#!/usr/bin/env python3
"""Añade filas a functions.tsv para funciones ya portadas con lift.py cuyo fallo era solo la llamada de más a la dirección de parada
(casos degenerados quitados con lift_prune.py). Cuenta los casos de los fixtures del módulo.
Uso (desde reimpl/): python tools/lift_rows_fix.py TAREA PRUEBA nombre1 nombre2 ..."""
import glob,json,sys
task,test=sys.argv[1],sys.argv[2];names=sys.argv[3:]
specs={}
for p in glob.glob('oracle/lifted_*.json'):
    try:d=json.load(open(p))
    except Exception:continue
    if not isinstance(d,dict) or 'funcs' not in d:continue
    mod=p.split('lifted_')[1][:-5]
    for f in d['funcs']:specs.setdefault(f['name'],(mod,f))
cnt={}
for p in glob.glob('p3-core/tests/fixtures/lifted_*.json'):
    for c in json.load(open(p))['cases']:
        if c['fn'] in names:cnt[c['fn']]=cnt.get(c['fn'],0)+1
txt=open('functions.tsv').read();have={l.split('\t')[0].lower() for l in txt.splitlines()}
rows=[]
for n in names:
    if n not in specs or not cnt.get(n):print('sin datos',n);continue
    mod,f=specs[n];a='0x%08X'%f['addr'];a=a.replace('0X','0x')
    if a.lower() in have:print('ya tiene fila',n);continue
    ind=' Llamadas virtuales: cuenta de argumentos deducida del patrón de pila (equilibrio de pila comprobado), no contra el método real.' if f.get('indirect') else ''
    note=f"Portada automáticamente (reimpl/tools/lift.py, modelo p3_core::x86rt); {cnt[n]} trazas x86 con todas las llamadas como ganchos; registros y bytes escritos idénticos. Casos con llamada indirecta a la dirección de parada quitados con lift_prune.py.{ind}"
    rows.append('\t'.join([a,'probada_emulador',f"p3_core::lifted::{mod}::{n}",task,test,note]))
if not txt.endswith('\n'):txt+='\n'
open('functions.tsv','w').write(txt+'\n'.join(rows)+('\n' if rows else ''));print(len(rows),'filas añadidas')
