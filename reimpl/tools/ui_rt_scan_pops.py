"""Busca en el EXE llamadas indirectas cuyo número de argumentos retirados quedó mal deducido en el código elevado.

Dos comprobaciones estáticas sobre cada función del catálogo (tramos rectos, sin seguir saltos):
1. Una limpieza `add esp, K` que retira más de lo que queda apilado si las llamadas indirectas anteriores retiran lo deducido
   (lo apilado era de una llamada cdecl posterior: caso `0x579EC0`, sombra de los efectos de ciudad).
2. Una llamada indirecta seguida de inmediato por `add esp, K` cuando el elevado ya le retira argumentos (puntero a función cdecl
   retirado dos veces: caso `0x41E680`, cierre de pergaminos con función de aviso).
No sustituye a `P3_ARGLOG`: no sigue saltos ni llamadas directas sin entrada en `ret_pop`. Cada línea es un candidato a revisar
con el desensamblado antes de añadirlo a `tools/ui_rt_fix.sites`.

Uso (desde `reimpl/`, con pefile y capstone): python tools/ui_rt_scan_pops.py Patrician3.exe .
"""
import sys,pefile,capstone,glob,os,re
from capstone import x86 as X
pe=pefile.PE(sys.argv[1]); base=pe.OPTIONAL_HEADER.ImageBase; img=pe.get_memory_mapped_image()
root=sys.argv[2]
funcs=[]
for l in open(root+'/../research/catalog/functions.tsv',encoding='utf-8',errors='replace').read().splitlines()[1:]:
    p=l.split('\t')
    if p[0].startswith('0x'):
        try: funcs.append((int(p[0],16),int(p[3])))
        except: pass
retpop={int(a,16):int(n) for a,n in re.findall(r'\((0x[0-9a-f]+), (\d+)\)',open(root+'/p3-render/src/ret_pop.rs').read())}
ded={}
pat=re.compile(r'c\.push\(m, (0x[0-9a-f]+)\); let args = \[(.*?)\]; c\.r\[0\] = host\.call\(m, target[^\n]*\n\s*c\.r\[4\] = c\.r\[4\]\.wrapping_add\((\d+)\)')
files=glob.glob(root+'/p3-core/src/lifted/*.rs')+glob.glob(root+'/p3-render/src/j08/*.rs')+glob.glob(root+'/p3-mvp/src/ui_rt/grp_*lifted.rs')
for f in files+[root+'/p3-mvp/src/ui_rt/fix_lifted.rs']:
    s=open(f,encoding='utf-8',errors='replace').read()
    for m in pat.finditer(s):
        ded[int(m.group(1),16)]=((int(m.group(3))-4)//4, os.path.basename(f))
md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_32); md.detail=True
sites=open(root+'/tools/ui_rt_fix.sites').read()
for start,size in funcs:
    if size<=0 or size>60000: continue
    ins=list(md.disasm(img[start-base:start-base+size],start))
    targets=set()
    for i in ins:
        if (i.mnemonic.startswith('j') or i.mnemonic.startswith('loop')) and i.operands and i.operands[0].type==X.X86_OP_IMM: targets.add(i.operands[0].imm)
    cnt=0; ind=[]; valid=True
    for i in ins:
        mn=i.mnemonic
        if i.address in targets: cnt=0; ind=[]; valid=True
        if mn=='push': cnt+=1
        elif mn=='pop': cnt-=1
        elif mn=='sub' and i.op_str.startswith('esp, '):
            try: cnt+=int(i.op_str.split(', ')[1],0)//4
            except: valid=False
        elif mn=='call':
            op=i.operands[0]; ret=i.address+i.size
            if op.type==X.X86_OP_IMM:
                n=retpop.get(op.imm)
                if n is None: valid=False
                else: cnt-=n
            elif op.type==X.X86_OP_MEM and op.mem.base==0 and op.mem.index==0:
                # import: stdcall/cdecl desconocido
                valid=False
            else:
                d=ded.get(ret)
                if d is None: valid=False
                else:
                    cnt-=d[0]
                    if d[0]>0: ind.append((ret,d[0],d[1]))
        elif mn=='add' and i.op_str.startswith('esp, '):
            try: k=int(i.op_str.split(', ')[1],0)//4
            except: k=0; valid=False
            if valid and ind and k>cnt and cnt+sum(x[1] for x in ind)>=k:
                for ret,dn,f in ind:
                    insite=('%#010x %#010x'%(start,ret)) in sites
                    print('%#x ret %#x pops_deducidos %d (limpieza %d, en pila %d) %s %s'%(start,ret,dn,k,cnt,f,'(sitio ya)' if insite else ''))
            cnt=0; ind=[]; valid=True
        elif mn=='ret' or mn.startswith('j'):
            cnt=0; ind=[]; valid=True
        elif 'esp' in i.op_str.split(',')[0] and mn not in('mov','cmp','test','lea','movzx','movsx') :
            valid=False

# Comprobación 2: puntero a función cdecl limpiado dos veces.
for start,size in funcs:
    if size<=0 or size>60000: continue
    ins=list(md.disasm(img[start-base:start-base+size],start))
    for k,i in enumerate(ins[:-1]):
        if i.mnemonic!='call': continue
        op=i.operands[0]
        if op.type==X.X86_OP_IMM or (op.type==X.X86_OP_MEM and op.mem.base==0 and op.mem.index==0): continue
        nx=ins[k+1]
        if not(nx.mnemonic=='add' and nx.op_str.startswith('esp, ')): continue
        K=int(nx.op_str.split(', ')[1],0)//4
        ret=i.address+i.size; d=ded.get(ret)
        if not d or d[0]==0: continue
        # pushes hacia atrÃ¡s hasta la llamada anterior
        p=0; j=k-1
        while j>=0 and ins[j].mnemonic not in('call','ret') and not ins[j].mnemonic.startswith('j'):
            if ins[j].mnemonic=='push': p+=1
            j-=1
        print('%#x ret %#x pops_deducidos %d limpieza %d pushes_previos %d %s %s'%(start,ret,d[0],K,p,d[1],'(sitio)' if ('%#010x %#010x'%(start,ret)) in sites else ''))
