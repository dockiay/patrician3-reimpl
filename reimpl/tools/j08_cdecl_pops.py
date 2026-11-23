"""Retira de los módulos `p3-render/src/j08/lift_*.rs` los desapilados falsos tras llamadas cdecl (`esp += N` decimal emitido como
`ret N` del llamado cuando `oracle/lift_x86.py` `callee_sig` da 0). Quedaban 143 tras envoltorios `sgl_*` (p. ej. `ui_render_text_at`
0x4BB3E0) generados antes de la corrección del 13/11: `ship_side_menu_draw_cargo_list` 0x48F510 colgaba el juego al cerrar el
comercio con carga en el barco (14/11/2026). Uso: `python3 tools/j08_cdecl_pops.py EXE [--fix]` desde `reimpl/`; sin `--fix` solo
cuenta (debe dar 0)."""
import re,sys,glob
sys.path.insert(0,'oracle')
from lift_x86 import Image
img=Image(sys.argv[1])
cache={}
def sig(t):
    if t not in cache:
        try: cache[t]=img.callee_sig(t)
        except Exception: cache[t]='err'
    return cache[t]
call=re.compile(r'h\.call_at\(esp, m, (0x[0-9a-f]+),')
pop=re.compile(r'^\s*esp = esp\.wrapping_add\((\d+)\);\s*$')
tot={}
fix='--fix' in sys.argv
for f in sorted(glob.glob('p3-render/src/j08/lift_*.rs')):
    L=open(f).read().split('\n'); ch=0
    for i,l in enumerate(L[:-1]):
        mm=call.search(l)
        if not mm: continue
        t=int(mm.group(1),16); p=pop.match(L[i+1])
        if p and sig(t)==0:
            tot[(t)]=tot.get(t,0)+1
            if fix: L[i+1]=None; ch+=1
    if fix and ch:
        open(f,'w').write('\n'.join(x for x in L if x is not None)); print(f,ch)
print(sum(tot.values()), {hex(k):v for k,v in sorted(tot.items(),key=lambda x:-x[1])[:30]})
sys.exit(1 if tot and not fix else 0)
