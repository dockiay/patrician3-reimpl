#!/usr/bin/env python3
"""Quita de los fixtures de trazas (`lifted_*.json`) los casos DEGENERADOS que hacen fallar una función portada con `tools/lift.py`.

Un caso degenerado es aquel en que una llamada indirecta del x86 cae en la dirección de parada del emulador (0x3000000, la dirección de retorno
de la función, porque un puntero de vtable aleatorio apunta a la pila): el x86 termina ahí, sin llamar a nada, y el porte Rust hace una llamada
de más. NO es un fallo del elevador. Esta herramienta solo quita los casos cuyo fallo es exactamente ese (último elemento de la lista de llamadas
con destino 0x3000xxx); cualquier otro fallo se deja y se lista, porque sí es una discrepancia real.

Uso: python tools/lift_prune.py [--test x86_lifted] [--rounds 40]   (desde reimpl/)"""
import argparse,glob,json,re,subprocess,sys
ap=argparse.ArgumentParser();ap.add_argument('--test',default='x86_lifted');ap.add_argument('--rounds',type=int,default=40);a=ap.parse_args()
DEG=re.compile(r'\[3000[0-9a-f]{3}(, [0-9a-f]+)*\]\]$')
removed=0
for it in range(a.rounds):
    out=subprocess.run(['cargo','test','-p','p3-core','--test',a.test,'--','--nocapture'],capture_output=True,text=True).stderr
    fails=[l for l in out.split('\n') if l.startswith('FALLA ')]
    summ=[l for l in out.split('\n') if 'con fallos' in l]
    print(summ[-1] if summ else 'sin resumen')
    if not fails:break
    real=[];n_del=0;todo=[]
    for l in fails:
        m=re.match(r'FALLA (\w+): caso (\d+): (.*)',l)
        if not m:continue
        fn,n,msg=m.group(1),int(m.group(2)),m.group(3)
        if msg.startswith('sobran llamadas') and DEG.search(msg.strip()):todo.append((fn,n))
        else:real.append(l)
    # los índices son del fichero: se borran de mayor a menor para que no se desplacen entre sí
    for fn,n in sorted(todo,key=lambda t:-t[1]):
        for p in glob.glob('p3-core/tests/fixtures/lifted_*.json'):
            d=json.load(open(p));c=d['cases']
            if n<len(c) and c[n]['fn']==fn:
                del c[n];json.dump(d,open(p,'w'),separators=(',',':'));n_del+=1;break
    removed+=n_del
    if real:
        print('DISCREPANCIAS REALES (no se tocan):');print('\n'.join(r[:400] for r in real));break
    if not n_del:break
print('casos degenerados quitados:',removed)
