#!/usr/bin/env python3
"""Añade a reimpl/functions.tsv una fila por función del lote ELV (pila/SEH/x87) que pasa la prueba `lifted_elv_match_x86` y no tiene fila.
Uso (desde reimpl/): python tools/lift_rows_elv.py tests/fixtures/lifted_elv_passed.txt report_a.json,report_b.json ..."""
import csv,json,re,sys
from pathlib import Path
passed=set(Path(sys.argv[1]).read_text().split())
report={}
for p in sys.argv[2].split(','):report.update(json.load(open(p)))
mods={}
for k in (1,2,3,4,5):
    for f in json.load(open(f'oracle/lifted_elv{k}.json'))['funcs']:mods[f['name']]=(k,f)
path='functions.tsv';txt=open(path).read();have={l.split('\t')[0].lower() for l in txt.splitlines()}
rows=[]
for name in sorted(passed):
    if name not in mods:continue
    k,f=mods[name];a='0x%08X'%f['addr'];a=a.replace('0X','0x');r=report.get(name)
    if a.lower() in have or not r:continue
    cov=100*r['edges']/max(r['static_edges'],1)
    ext=' Modo ampliado de pila/SEH.' if f.get('ext') else ''
    ind=' Llamadas virtuales: cuenta de argumentos deducida del patrón de pila (equilibrio de pila comprobado), no contra el método real.' if f.get('indirect') else ''
    x87=' Compara también profundidad y ST0 de la pila x87 a la salida (casos con fallo de pila x87 descartados: las llamadas interceptadas no reproducen la FPU del CRT).' if f.get('x87') else ''
    note=f"Portada automáticamente (reimpl/tools/lift.py, modelo p3_core::x86rt); {r['accepted']} trazas x86 con todas las llamadas como ganchos; aristas de código cubiertas {r['edges']}/{r['static_edges']} ({cov:.0f} %); registros y bytes escritos idénticos.{ext}{ind}{x87}"
    rows.append('\t'.join([a,'probada_emulador',f"p3_core::lifted::elv{k}::{name}",'ELV','cargo test -p p3-core --test x86_lifted lifted_elv',note]))
if not txt.endswith('\n'):txt+='\n'
open(path,'w').write(txt+'\n'.join(rows)+('\n' if rows else ''));print(len(rows),'filas añadidas')
