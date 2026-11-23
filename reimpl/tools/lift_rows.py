#!/usr/bin/env python3
"""Añade a reimpl/functions.tsv una fila por función portada con tools/lift.py y comprobada por trazas (solo las que pasan la prueba y no tienen ya fila).
Uso: lift_rows.py SPEC.json REPORT.json MODULO TAREA PRUEBA [--passed NOMBRES.txt]"""
import csv,json,sys
spec=json.load(open(sys.argv[1]));report={}
for p in sys.argv[2].split(','):report.update(json.load(open(p)))
module,task,test=sys.argv[3],sys.argv[4],sys.argv[5]
passed=None
if len(sys.argv)>7 and sys.argv[6]=='--passed':passed=set(open(sys.argv[7]).read().split())
cat={r['address'].lower():r for r in csv.DictReader(open('/home/user/patrician3-research/research/catalog/functions.tsv'),delimiter='\t')}
path='/home/user/patrician3-research/reimpl/functions.tsv'
txt=open(path).read();have={l.split('\t')[0].lower() for l in txt.splitlines()}
rows=[]
for f in spec['funcs']:
    a='0x%08x'%f['addr'];r=report.get(f['name'])
    if not r or r['accepted']<3:continue
    if passed is not None and f['name'] not in passed:continue
    if a.lower() in have:continue
    cov=100*r['edges']/max(r['static_edges'],1)
    ind=' Llamadas virtuales: cuenta de argumentos deducida del patrón de pila (equilibrio de pila comprobado), no contra el método real.' if f.get('indirect') else ''
    note=f"Portada automáticamente (reimpl/tools/lift.py, modelo p3_core::x86rt); {r['accepted']} trazas x86 con todas las llamadas como ganchos; aristas de código cubiertas {r['edges']}/{r['static_edges']} ({cov:.0f} %); registros y bytes escritos idénticos.{ind}"
    rows.append('\t'.join([a.upper().replace('0X','0x'),'probada_emulador',f"p3_core::{module}::{f['name']}",task,test,note]))
if not txt.endswith('\n'):txt+='\n'
open(path,'w').write(txt+'\n'.join(rows)+('\n' if rows else ''))
print(len(rows),'filas añadidas')
