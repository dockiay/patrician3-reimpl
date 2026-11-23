#!/usr/bin/env python3
"""Portado automático x86 → Rust (hilo de diálogos, 11/11/2026).

Traduce funciones del Patrician3.exe inglés (enteras, sin x87, sin llamadas indirectas ni tablas de saltos) a Rust sobre el modelo de CPU
`p3_core::x86rt` (registros, banderas CF/ZF/SF/OF, memoria por `Mem`, llamadas por `OpsHost`). El resultado es una máquina de bloques básicos instrucción a
instrucción: no es código idiomático, pero su paridad con el x86 se comprueba por trazas (`oracle/lifttrace.py` + `p3-core/tests/x86_lifted.rs`).

Uso: python reimpl/tools/lift.py --binary EXE --catalog research/catalog/functions.tsv --out p3-core/src/lifted/NAME.rs --spec oracle/lifted_NAME.json DIR1 DIR2 ...
(las direcciones, en hexadecimal). Lo que no se pueda traducir se devuelve como «sin soporte» con el motivo.
"""
import argparse,csv,json,re,sys
from pathlib import Path
import pefile,capstone
from capstone.x86 import *

REG={'eax':(0,32),'ecx':(1,32),'edx':(2,32),'ebx':(3,32),'esp':(4,32),'ebp':(5,32),'esi':(6,32),'edi':(7,32),
 'ax':(0,16),'cx':(1,16),'dx':(2,16),'bx':(3,16),'sp':(4,16),'bp':(5,16),'si':(6,16),'di':(7,16),
 'al':(0,8),'cl':(1,8),'dl':(2,8),'bl':(3,8),'ah':(4,8),'ch':(5,8),'dh':(6,8),'bh':(7,8)}
JCC={'jp':10,'jnp':11,'jpe':10,'jpo':11,'jo':0,'jno':1,'jb':2,'jae':3,'je':4,'jne':5,'jbe':6,'ja':7,'js':8,'jns':9,'jl':12,'jge':13,'jle':14,'jg':15,
 'jz':4,'jnz':5,'jc':2,'jnc':3,'jnb':3,'jnae':2,'jna':6,'jnbe':7,'jnl':13,'jnge':12,'jng':14,'jnle':15}
SETCC={'set'+k[1:]:v for k,v in JCC.items()}
class Unsupported(Exception):pass
COMMENTS=False

NOPUSH_F={'fld','fild','fld1','fldz','fldpi','fldcw','fnstcw','fstcw','fnstsw','fstsw','fldl2e','fldl2t','fldlg2','fldln2','fninit','fnclex','fwait','fnop','fldenv','fnstenv'}
def fret_after(ins,k):
    """True si tras la llamada ins[k] la primera instrucción x87 consume ST0 sin apilar antes (la llamada devuelve un float en ST0)."""
    for j in range(k+1,min(k+5,len(ins))):
        m=ins[j].mnemonic
        if m=='add' and ins[j].op_str.startswith('esp'):continue
        if m in('pop','mov','lea') :continue
        if m.startswith('f') and m not in NOPUSH_F:return True
        if m=='call' and ins[j].operands[0].type==X86_OP_IMM and ins[j].operands[0].imm==0x639c28:return True  # _ftol consume ST0 (p. ej. ceil -> _ftol)
        return False
    return False

CRT_MATH={0x639c28,0x63aae4,0x63c264,0x63c314,0x63c3c4,0x63b2c4,0x63b1f5,0x63a5ce}  # rutinas del CRT con efecto en la FPU; modeladas en Cpu::crt_call con --fpu-exit

class Lifter:
    def __init__(s,binary,catalog):
        s.pe=pefile.PE(binary);s.base=s.pe.OPTIONAL_HEADER.ImageBase
        s.cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_32);s.cs.detail=True
        s.cat={int(r['address'],16):r for r in csv.DictReader(open(catalog),delimiter='\t')}
        s.callee_cache={};s.sizes={};s.lax=False;s.nover={};s.ext=False;s.plan=None;s.fpuexit=False;s.guess=False;s.localargs=False;s.cur_name=None
    def code(s,a,n):return s.pe.get_data(a-s.base,n)
    def x87(s,i):
        mn=i.mnemonic;ops=i.operands
        def mem():
            if ops[0].type!=X86_OP_MEM or ops[0].size not in(4,8):raise Unsupported('x87 '+mn+' '+i.op_str)
            return s.ea(ops[0]),ops[0].size
        def sti(op):
            if op.type!=X86_OP_REG:raise Unsupported('x87 '+mn+' '+i.op_str)
            nm=s.cs.reg_name(op.reg)
            if not nm.startswith('st'):raise Unsupported('x87 '+mn+' '+i.op_str)
            return int(nm[2:].strip('()') or 0)
        ARI={'fadd':'+','fmul':'*','fdiv':'/','fsub':'-','fsubr':'r','fdivr':'q'}
        isreg=bool(ops) and ops[0].type==X86_OP_REG
        if mn=='fld':
            if isreg:return [f"c.fld_st({sti(ops[0])});"]
            a,z=mem();return [f"c.fld(c.ld(m,{a},{z}));"]
        if mn=='fild':
            a,z=mem();return [f"c.fild{32 if z==4 else 64}(m,{a});"]
        if mn=='fstp':
            if isreg:return [f"c.fstp_st({sti(ops[0])});"]
            a,z=mem();return [f"c.fstp{32 if z==4 else 64}(m,{a});"]
        if mn=='fst':
            if isreg:return [f"c.fst_st({sti(ops[0])});"]
            a,z=mem();return [f"c.fst{32 if z==4 else 64}(m,{a});"]
        if mn=='fistp':
            a,z=mem();return [f"c.fistp(m,{a},{z});"]
        if mn=='fchs' :return ["c.fchs();"]
        if mn=='fabs' :return ["c.fabs();"]
        if mn=='fsqrt':return ["c.fsqrt();"]
        if mn=='fxch':return [f"c.fxch({sti(ops[0]) if ops else 1});"]
        if mn in ARI:
            if isreg:
                d=sti(ops[0]);sr=sti(ops[1]) if len(ops)>1 else 0
                return [f"c.fop_st('{ARI[mn]}',{d},{sr},false);"]
            a,z=mem();return [f"{{ let v = c.ld(m,{a},{z}); c.fop('{ARI[mn]}',v); }}"]
        if mn[:-1] in ARI and mn.endswith('p'):
            if len(ops)==1 and sti(ops[0])==1 and mn[:-1] in('fadd','fmul','fdiv','fsub'):return [f"c.fopp('{ARI[mn[:-1]]}');"]
            if len(ops)==0:d,sr=1,0
            elif len(ops)==1:d,sr=sti(ops[0]),0
            else:d,sr=sti(ops[0]),sti(ops[1])
            return [f"c.fop_st('{ARI[mn[:-1]]}',{d},{sr},true);"]
        if mn=='fcomp':
            if isreg:return [f"c.fcom_st({sti(ops[0])},true);"]
            a,z=mem();return [f"{{ let v = c.ld(m,{a},{z}); c.fcomp(v); }}"]
        if mn=='fcom':
            if isreg:return [f"c.fcom_st({sti(ops[0])},false);"]
            a,z=mem();return [f"{{ let v = c.ld(m,{a},{z}); c.fcom_v(v,false); }}"]
        if mn=='fnstsw' and i.op_str=='ax':return ["c.r[0] = (c.r[0] & 0xffff_0000) | c.fnstsw();"]
        raise Unsupported('x87 '+mn+' '+i.op_str)
    def tail_n(s,ins,name):
        """Argumentos (palabras) que hereda una llamada de cola virtual: el `ret imm` propio de la función o, si no lo hay, los parámetros del catálogo."""
        imm=[i.operands[0].imm for i in ins if i.mnemonic=='ret' and i.operands]
        if imm:return max(imm)//4
        r=[x for x in s.cat.values() if x['name']==name][0]
        p=int(r['params'] or 0)
        return max(p-1,0) if r['calling_convention']=='__thiscall' else p
    def jtab_targets(s,i):
        o=i.operands[0]
        if o.type!=X86_OP_MEM or not o.mem.index or o.mem.scale!=4 or o.mem.segment==X86_REG_FS:raise Unsupported('jmp indirecto')
        lo,hi=s.rng;t=o.mem.disp&0xffffffff;res=[]
        while True:
            try:v=int.from_bytes(s.code(t+4*len(res),4),'little')
            except Exception:break
            if not lo<=v<hi:break
            res.append(v)
        if not res:raise Unsupported('tabla de saltos vacía')
        return res
    def extent(s,t):
        """Tamaño real de la función en t: el del catálogo recorta las que tienen funclets `Catch@` dentro (el catálogo no cuenta sus bytes)."""
        if t in s.sizes:return s.sizes[t]
        n=int(s.cat[t]['bytes'])
        if not hasattr(s,'_starts'):
            s._starts=sorted((a,r.get('kind')) for a,r in s.cat.items())
            import bisect;s._bis=bisect
        st=s._starts;i=s._bis.bisect_left(st,(t,''))
        j=i+1;inside=False;nxt=None
        while j<len(st):
            a,k=st[j]
            if k=='eh_funclet' and a<t+n:inside=True
            elif a>=t+n or k!='eh_funclet':
                nxt=a if k!='eh_funclet' else None;break
            j+=1
        if inside and nxt is not None and nxt-t<n+0x400:return nxt-t
        return n
    def reachable_instructions(s,addr,size):
        """Decode reachable blocks, excluding inline switch tables and dead data.

        Used explicitly for catalog entries split into command labels. Calls
        remain external; only local control flow belongs to this function.
        """
        lo,hi=addr,addr+size
        s.rng=(lo,hi)
        pending=[addr];seen={}
        while pending:
            at=pending.pop()
            while lo<=at<hi and at not in seen:
                i=next(s.cs.disasm(s.code(at,min(15,hi-at)),at,count=1),None)
                if i is None:raise Unsupported(f'instrucción inaccesible en {at:#x}')
                seen[at]=i
                nxt=at+i.size
                if i.mnemonic in ('ret','retf','int3'):break
                if i.mnemonic=='jmp':
                    if i.operands[0].type==X86_OP_IMM:
                        target=i.operands[0].imm
                        if lo<=target<hi:pending.append(target)
                    else:
                        try:pending.extend(s.jtab_targets(i))
                        except Unsupported:
                            # Virtual tail calls leave this function.
                            if i.operands[0].type==X86_OP_MEM and i.operands[0].mem.index:
                                raise
                    break
                if i.mnemonic in JCC:
                    target=i.operands[0].imm
                    if not lo<=target<hi:raise Unsupported(f'rama fuera del rango en {at:#x}')
                    pending.append(target)
                at=nxt
        instructions=sorted(seen.values(),key=lambda i:i.address)
        for a,b in zip(instructions,instructions[1:]):
            if a.address+a.size>b.address:raise Unsupported(f'instrucciones solapadas en {b.address:#x}')
        return instructions
    def callee_pops(s,t,depth=0):
        """Bytes de argumentos que desapila el llamado (`ret imm16`), o None si no se encuentra ninguno en su cuerpo (cdecl o thunk)."""
        if (t,s.ext) in s.callee_cache:return s.callee_cache[(t,s.ext)]
        if s.ext and t in s.cat and s.cat[t].get('kind')=='thunk' and s.cat[t]['calling_convention']=='__stdcall' and s.cat[t]['params'].isdigit():
            return 4*int(s.cat[t]['params'])  # envoltorio de una importación stdcall (WritePrivateProfileStringA...)
        n=s.extent(t) if t in s.cat else 0x200
        res=None
        if s.ext:
            # modo ampliado: recorrido por el flujo desde la entrada (el desensamblado lineal se pierde con tablas de saltos incrustadas)
            seen=set();work=[t];rets=set();thunk=None
            try:
                while work and len(seen)<6000:
                    a=work.pop()
                    while a not in seen and t<=a<t+n+0x40:
                        seen.add(a)
                        i=next(s.cs.disasm(s.code(a,15),a),None)
                        if i is None:break
                        mn=i.mnemonic
                        if mn=='ret':rets.add(i.operands[0].imm if i.operands else 0);break
                        if mn=='jmp':
                            o=i.operands[0]
                            if o.type==X86_OP_IMM:
                                if t<=o.imm<t+n:a=o.imm;continue
                                if depth<4:rets.add(s.callee_pops(o.imm,depth+1))
                            elif a==t:thunk=True
                            break
                        if mn in JCC:work.append(i.operands[0].imm)
                        a+=i.size
            except Exception:pass
            rets.discard(None)
            res=max(rets) if rets else None
            s.callee_cache[(t,s.ext)]=res;return res
        try:
            for i in s.cs.disasm(s.code(t,min(max(n,16),0x1000)),t):
                if i.mnemonic=='ret':
                    res=i.operands[0].imm if i.operands else 0;break
                if i.mnemonic=='jmp' and i.address==t:break  # thunk
                if i.mnemonic=='jmp' and i.operands[0].type==X86_OP_IMM and not (t<=i.operands[0].imm<t+n) and depth<4:
                    res=s.callee_pops(i.operands[0].imm,depth+1);break  # cola de la función fuera de su rango
        except Exception:pass
        s.callee_cache[(t,s.ext)]=res;return res
    # ---------- operandos
    def ea(s,op):
        m=op.mem;parts=[]
        if m.base:parts.append(f"c.r[{REG[s.cs.reg_name(m.base)][0]}]")
        if m.index:
            sc=m.scale;ix=f"c.r[{REG[s.cs.reg_name(m.index)][0]}]"
            parts.append(ix if sc==1 else f"{ix}.wrapping_mul({sc})")
        disp=m.disp&0xffffffff
        e=parts[0] if parts else "0u32"
        for p in parts[1:]:e=f"{e}.wrapping_add({p})"
        if disp or not parts:e=f"{e}.wrapping_add({disp:#x})" if parts else f"{disp:#x}u32"
        return e
    def width(s,op):return op.size*8
    def rd(s,op,w=None):
        w=w or s.width(op)
        if op.type==X86_OP_REG:
            r,rw=REG[s.cs.reg_name(op.reg)];return f"c.get({r},{rw})"
        if op.type==X86_OP_IMM:return f"{op.imm&((1<<w)-1):#x}u32"
        if op.type==X86_OP_MEM:return f"rm(m,{s.ea(op)},{w})"
        raise Unsupported('operando')
    def wr(s,op,val,w=None):
        w=w or s.width(op)
        if op.type==X86_OP_REG:
            r,rw=REG[s.cs.reg_name(op.reg)];return f"c.set({r},{rw},{val});"
        if op.type==X86_OP_MEM:return f"{{ let a = {s.ea(op)}; wm(m,a,{w},{val}); }}"
        raise Unsupported('destino')
    # ---------- traducción de una instrucción: devuelve (líneas, tipo, objetivo)
    def tr(s,i,fn):
        mn=i.mnemonic;ops=i.operands;L=[]
        if mn=='rep movsd':return ["c.rep_movsd(m);"],None,None
        mm=re.fullmatch(r'(rep )?(stos|movs)([bwd])',mn)
        if mm:
            w={'b':8,'w':16,'d':32}[mm.group(3)]
            return [f"c.{mm.group(2)}(m,{w},{'true' if mm.group(1) else 'false'});"],None,None
        if mn=='cld':return [],None,None
        if mn.startswith('f') and mn not in('fs',):return s.x87(i),None,None
        if ops and ops[0].type==X86_OP_REG and s.cs.reg_name(ops[0].reg) in('esp','sp') and mn not in('add','sub','push','pop','call','ret','cmp','test','mov','and','lea') and not s.lax:raise Unsupported('escritura de esp: '+mn)
        if mn in('mov',):
            if ops[0].type==X86_OP_MEM and ops[0].mem.segment==X86_REG_FS or ops[1].type==X86_OP_MEM and ops[1].mem.segment==X86_REG_FS:pass
            w=s.width(ops[0]);return [s.wr(ops[0],s.rd(ops[1],w),w)],None,None
        if mn in('movzx','movsx'):
            ws=s.width(ops[1]);wd=s.width(ops[0]);v=s.rd(ops[1],ws)
            if mn=='movsx':v=f"sx({v},{ws})&{(1<<wd)-1:#x}u32"
            return [s.wr(ops[0],v,wd)],None,None
        if mn=='lea':
            return [s.wr(ops[0],s.ea(ops[1]),32)],None,None
        if mn=='push':
            v=s.rd(ops[0],32) if ops[0].type!=X86_OP_IMM else f"{ops[0].imm&0xffffffff:#x}u32"
            if ops[0].type==X86_OP_MEM and ops[0].size!=4:raise Unsupported('push mem')
            return [f"c.push(m,{v});"],None,None
        if mn=='pop':
            if ops[0].type==X86_OP_REG and REG[s.cs.reg_name(ops[0].reg)][0]==4:raise Unsupported('pop esp')
            return [f"{{ let v = c.pop(m); {s.wr(ops[0],'v',32)} }}"],None,None
        bin2={'add':'add','sub':'sub','adc':'adc','sbb':'sbb','and':'and','or':'or','xor':'xor'}
        if mn in bin2:
            w=s.width(ops[0]);b=s.rd(ops[1],w)
            if ops[1].type==X86_OP_IMM:b=f"{sximm(ops[1].imm,w):#x}u32"
            return [f"{{ let a = {s.rd(ops[0],w)}; let r = c.{bin2[mn]}(a,{b},{w}); {s.wr(ops[0],'r',w)} }}"],None,None
        if mn in('cmp','test'):
            w=s.width(ops[0]);b=s.rd(ops[1],w)
            if ops[1].type==X86_OP_IMM:b=f"{sximm(ops[1].imm,w):#x}u32"
            f='sub' if mn=='cmp' else 'and'
            return [f"{{ let a = {s.rd(ops[0],w)}; c.{f}(a,{b},{w}); }}"],None,None
        if mn in('inc','dec','neg','not'):
            w=s.width(ops[0])
            if mn=='not':return [f"{{ let a = {s.rd(ops[0],w)}; {s.wr(ops[0],f'!a & {mask(w):#x}u32',w)} }}"],None,None
            return [f"{{ let a = {s.rd(ops[0],w)}; let r = c.{mn}(a,{w}); {s.wr(ops[0],'r',w)} }}"],None,None
        if mn in('shl','shr','sar','sal'):
            w=s.width(ops[0]);mm='shl' if mn=='sal' else mn
            n=s.rd(ops[1],8) if len(ops)>1 else '1u32'
            return [f"{{ let a = {s.rd(ops[0],w)}; let n = {n}; let r = c.{mm}(a,n,{w}); {s.wr(ops[0],'r',w)} }}"],None,None
        if mn=='imul':
            if len(ops)==1:
                w=s.width(ops[0]);b=s.rd(ops[0],w)
                if w==8:return [f"{{ let (lo,hi) = c.imul1(c.get(0,8),{b},8); c.set(0,16,lo | (hi<<8)); }}"],None,None
                if w==16:return [f"{{ let (lo,hi) = c.imul1(c.get(0,16),{b},16); c.set(0,16,lo); c.set(2,16,hi); }}"],None,None
                return [f"{{ let (lo,hi) = c.imul1(c.r[0],{b},32); c.r[0]=lo; c.r[2]=hi; }}"],None,None
            w=s.width(ops[0])
            if len(ops)==2:a=s.rd(ops[0],w);b=s.rd(ops[1],w)
            else:
                a=s.rd(ops[1],w);b=f"{sximm(ops[2].imm,w):#x}u32"
            return [f"{{ let a = {a}; let r = c.imul(a,{b},{w}); {s.wr(ops[0],'r',w)} }}"],None,None
        if mn in('mul','div','idiv'):
            w=s.width(ops[0])
            if w!=32:raise Unsupported(mn+str(w))
            b=s.rd(ops[0],32)
            if mn=='mul':return [f"{{ let b = {b}; let (lo,hi) = c.mul1(c.r[0],b,32); c.r[0]=lo; c.r[2]=hi; }}"],None,None
            return [f"{{ let b = {b}; c.{mn}32(b); }}"],None,None
        if mn=='cdq':return ["c.r[2] = if c.r[0] & 0x8000_0000 != 0 { 0xffff_ffff } else { 0 };"],None,None
        if mn=='cwde':return ["c.r[0] = sx(c.r[0] & 0xffff,16);"],None,None
        if mn in SETCC:
            return [s.wr(ops[0],f"u32::from(c.cond({SETCC[mn]}))",8)],None,None
        if mn=='nop':return [],None,None
        if mn=='leave':return ["c.r[4] = c.r[5]; c.r[5] = c.pop(m);"],None,None
        if mn=='xchg':
            w=s.width(ops[0]);return [f"{{ let a = {s.rd(ops[0],w)}; let b = {s.rd(ops[1],w)}; {s.wr(ops[0],'b',w)} {s.wr(ops[1],'a',w)} }}"],None,None
        if mn in JCC:return [],'jcc',(JCC[mn],ops[0].imm)
        if mn=='jmp':
            if ops[0].type!=X86_OP_IMM:
                o=ops[0]
                if o.type==X86_OP_REG or (o.type==X86_OP_MEM and not (o.mem.index and o.mem.scale==4 and o.mem.base==0)):return [],'vtail',s.rd(o,32)
                return [],'jtab',(s.ea(o),s.jtab_targets(i))
            return [],'jmp',ops[0].imm
        if mn=='ret':return [],'ret',(ops[0].imm if ops else 0)
        if mn=='call':
            if ops[0].type!=X86_OP_IMM:
                if ops[0].type==X86_OP_MEM and ops[0].mem.segment==X86_REG_FS:raise Unsupported('call fs')
                return [],'icall',s.rd(ops[0],32)
            return [],'call',ops[0].imm
        raise Unsupported(mn)
    # ---------- función completa
    def lift(s,addr,size,name):
        code=s.code(addr,size)
        ins=s.reachable_instructions(addr,size) if getattr(s,'reachable',False) else list(s.cs.disasm(code,addr))
        if not getattr(s,'reachable',False) and sum(i.size for i in ins)!=size:raise Unsupported('desensamblado incompleto')
        by={i.address:i for i in ins}
        leaders={addr};s.rng=(addr,addr+size)
        for k,i in enumerate(ins):
            r=s.tr(i,name)[1:]
            if r[0] in('jcc','jmp'):
                t=r[1][1] if r[0]=='jcc' else r[1]
                if t not in by:
                    if r[0]=='jmp' and t in s.cat:pass  # salto final a otra función (llamada de cola)
                    else:raise Unsupported(f'salto fuera de la función {t:#x}')
                else:leaders.add(t)
                if k+1<len(ins):leaders.add(ins[k+1].address)
            elif r[0]=='jtab':
                for t in r[1][1]:
                    if t not in by:raise Unsupported('tabla fuera de la función')
                    leaders.add(t)
                if k+1<len(ins):leaders.add(ins[k+1].address)
            elif r[0]=='ret' and k+1<len(ins):leaders.add(ins[k+1].address)
        saves=s.save_pushes(ins)
        s.cur_name=name;s.plan=s.plan_depth(ins,by) if s.ext else None
        fx='c.fpu_exit(); ' if s.fpuexit and any(i.mnemonic.startswith('f') and i.mnemonic!='fs' for i in ins) else ''
        spec=dict(addr=addr,size=size,name=name,calls={},sites={},consts=set(),globals=set(),ret_imm=0,conv=s.cat[addr]['calling_convention'],params=s.cat[addr]['params'])
        out=[f"pub fn {name}(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> [u32; 8] {{",
             "    let mut c = Cpu { r: regs, ..Cpu::default() };",f"    let mut pc: u32 = {addr:#x};","    loop {","        match pc {"]
        body={}
        cur=None;blk=[]
        for k,i in enumerate(ins):
            if i.address in leaders:
                if cur is not None:body[cur]=blk
                cur=i.address;blk=[]
            lines,kind,tgt=s.tr(i,name)
            # análisis
            for op in i.operands:
                if op.type==X86_OP_MEM and not op.mem.base and not op.mem.index and op.mem.segment!=X86_REG_FS:spec['globals'].add(op.mem.disp&0xffffffff)
                if op.type==X86_OP_IMM and i.mnemonic in('cmp','test','sub','and','mov','add'):spec['consts'].add(op.imm&0xffffffff)
            if COMMENTS:blk.append(f"            // {i.address:#x}: {i.mnemonic} {i.op_str}")
            blk+= ["            "+l for l in lines]
            nxt=i.address+i.size
            if kind=='jcc':
                cc,t=tgt;blk.append(f"            if c.cond({cc}) {{ pc = {t:#x}; continue; }}")
                blk.append(f"            pc = {nxt:#x}; continue;")
            elif kind=='jmp' and tgt not in by:
                n,pops=s.call_info(tgt,i,ins,k)
                if pops is False:n=0
                if (name,0x35000100) in s.nover:n=s.nover[(name,0x35000100)]  # llamada de cola: n observado en las trazas (retorno STOP)
                spec['calls'][tgt]=dict(n=n,pops=True,tail=True)
                args=", ".join(f"m.r32(c.r[4].wrapping_add({4+4*j}))" for j in range(n))
                if s.fpuexit and tgt in CRT_MATH:blk.append(f"            c.crt_call(m, {tgt:#x});")
                else:blk.append(f"            {{ let args = [{args}]; c.r[0] = host.call(m, {tgt:#x}, c.r[1], &args); }}")
                blk.append(f"            c.r[4] = c.r[4].wrapping_add({4+4*n}); {fx}return c.r;")
            elif kind=='jmp':blk.append(f"            pc = {tgt:#x}; continue;")
            elif kind=='vtail':
                n=s.tail_n(ins,name);spec['vtail']=n;spec['indirect']=True
                args=", ".join(f"m.r32(c.r[4].wrapping_add({4+4*j}))" for j in range(n))
                blk.append(f"            {{ let target = {tgt}; let args = [{args}]; c.r[0] = host.call(m, target, c.r[1], &args); }}")
                blk.append(f"            c.r[4] = c.r[4].wrapping_add({4+4*n}); {fx}return c.r;")
            elif kind=='jtab':blk.append(f"            pc = m.r32({tgt[0]}); continue;")
            elif kind=='ret':
                spec['ret_imm']=max(spec['ret_imm'],tgt)
                blk.append(f"            c.r[4] = c.r[4].wrapping_add({4+tgt}); {fx}return c.r;")
            elif kind=='icall':
                n=s.nover.get((name,nxt)) if (name,nxt) in s.nover else (s.plan[nxt] if s.plan is not None and nxt in s.plan else s.pending_args(ins,k,leaders,saves))
                spec['sites'][nxt]=dict(n=n,pops=True,target=None);spec['indirect']=True
                args=", ".join(f"m.r32(c.r[4].wrapping_add({4+4*j}))" for j in range(n))
                blk.append(f"            {{ let target = {tgt}; c.push(m, {nxt:#x}); let args = [{args}]; c.r[0] = host.call(m, target, c.r[1], &args); }}")
                if s.fpuexit and fret_after(ins,k):blk.append("            c.fret();");spec.setdefault('fret',[]).append(nxt)
                blk.append(f"            c.r[4] = c.r[4].wrapping_add({4+4*n});")
                if k+1<len(ins) and ins[k+1].address in leaders:blk.append(f"            pc = {nxt:#x}; continue;")
            elif kind=='call':
                n,pops=s.call_info(tgt,i,ins,k)
                if (name,nxt) in s.nover:n,pops=s.nover[(name,nxt)],True
                spec['calls'][tgt]=dict(n=n,pops=pops);spec['sites'][nxt]=dict(n=n,pops=pops,target=tgt)
                args=", ".join(f"m.r32(c.r[4].wrapping_add({4+4*j}))" for j in range(n))
                if tgt==0x63ca98:
                    # cabecera SEH (`_EH_prolog`): [s-4]=ebp, [s-8]=-1, [s-12]=eax (manejador), [s-16]=fs:[0]; fs:[0]=s-16; ebp=s-4; esp=s-16; eax=retorno
                    blk.append(f"            {{ let s = c.r[4]; let fs0 = m.r32(0); m.w32(s.wrapping_sub(4), c.r[5]); m.w32(s.wrapping_sub(8), 0xffff_ffff); m.w32(s.wrapping_sub(12), c.r[0]); m.w32(s.wrapping_sub(16), fs0); m.w32(0, s.wrapping_sub(16)); c.r[5] = s.wrapping_sub(4); c.r[4] = s.wrapping_sub(16); c.r[0] = {nxt:#x}; }}")
                    if k+1<len(ins) and ins[k+1].address in leaders:blk.append(f"            pc = {nxt:#x}; continue;")
                    continue
                blk.append(f"            c.push(m, {nxt:#x});")
                if s.fpuexit and tgt in CRT_MATH:blk.append(f"            c.crt_call(m, {tgt:#x});")
                elif tgt in (0x63a6a0,0x63a6e0):
                    # __allmul / __alldiv (ret 0x10): el resultado de 64 bits va en EDX:EAX; `host.call` solo devuelve EAX, así que se hace en línea.
                    a=f"((m.r32(c.r[4].wrapping_add(4)) as u64) | ((m.r32(c.r[4].wrapping_add(8)) as u64) << 32)) as i64"
                    b=f"((m.r32(c.r[4].wrapping_add(12)) as u64) | ((m.r32(c.r[4].wrapping_add(16)) as u64) << 32)) as i64"
                    if tgt==0x63a6a0:blk.append(f"            {{ let r = ({a}).wrapping_mul({b}); c.r[0] = r as u32; c.r[2] = (r >> 32) as u32; }}")
                    else:blk.append(f"            {{ let r = ({a}).checked_div({b}).expect(\"división por cero del x86\"); c.r[0] = r as u32; c.r[2] = (r >> 32) as u32; }}")
                elif tgt==0x63b920:
                    # _chkstk: reserva EAX bytes de pila (vuelve con esp = esp - eax); EAX queda con el valor del gancho
                    blk.append(f"            {{ let sz = c.r[0]; c.r[0] = host.call(m, {tgt:#x}, c.r[1], &[]); c.r[4] = c.r[4].wrapping_sub(sz); }}")
                else:
                    blk.append(f"            {{ let args = [{args}]; c.r[0] = host.call(m, {tgt:#x}, c.r[1], &args); }}")
                    if s.fpuexit and fret_after(ins,k):blk.append("            c.fret();");spec.setdefault('fret',[]).append(nxt)
                blk.append(f"            c.r[4] = c.r[4].wrapping_add({4+(4*n if pops else 0)});")
                if k+1<len(ins) and ins[k+1].address in leaders:blk.append(f"            pc = {nxt:#x}; continue;")
            else:
                if k+1<len(ins) and ins[k+1].address in leaders:blk.append(f"            pc = {nxt:#x}; continue;")
        body[cur]=blk
        for a in sorted(body):
            out.append(f"            {a:#x} => {{");out+=body[a]
            if not body[a] or not any(l.strip().startswith(('pc =','c.r[4] = c.r[4]','if c.cond')) for l in body[a][-4:]):
                pass
            out.append("            }")
        out+=["            _ => unreachable!(\"pc fuera de la función: {pc:#x}\"),","        }","    }","}",""]
        spec['x87']=bool(fx);spec['consts']=sorted(spec['consts']);spec['globals']=sorted(spec['globals']);spec['indirect']=spec.get('indirect',False)
        try:
            if not s.ext:s.check_depth(ins,by,spec)
        except Unsupported as e:
            if not s.lax:raise
            spec['depth_warning']=str(e)
        edges=[]
        for i in ins:
            nxt=i.address+i.size;mn=i.mnemonic
            if mn=='ret':continue
            if mn=='jmp' and i.operands[0].type!=X86_OP_IMM and not (i.operands[0].type==X86_OP_MEM and i.operands[0].mem.index and i.operands[0].mem.scale==4 and i.operands[0].mem.base==0):continue
            if mn=='jmp' and i.operands[0].type!=X86_OP_IMM:edges+=[[i.address,t] for t in s.jtab_targets(i)]
            elif mn=='jmp':
                if i.operands[0].imm in by:edges.append([i.address,i.operands[0].imm])
            elif mn in JCC:edges+= [[i.address,i.operands[0].imm],[i.address,nxt]]
            elif nxt in by:edges.append([i.address,nxt])
        spec['edges']=edges
        return "\n".join(out),spec
    def plan_depth(s,ins,by):
        """Modo ampliado (pila/SEH, 11/11/2026): recorre el grafo con estado (base, profundidad, ebp) y deduce cuántos argumentos retira cada llamada indirecta
        (`call [edx+8]`, `call ebp`...) como profundidad actual menos la profundidad de marco (la de la primera llamada menos sus argumentos). Soporta
        `mov ebp, esp`, `mov esp, ebp`, `lea esp,[ebp-N]`, `leave` y `and esp, -8` (nueva base). Devuelve {dirección tras la llamada: n}."""
        def esp_dst(i):return i.operands and i.operands[0].type==X86_OP_REG and s.cs.reg_name(i.operands[0].reg)=='esp'
        def reg(i,k):return i.operands[k].type==X86_OP_REG and s.cs.reg_name(i.operands[k].reg)
        index={i.address:k for k,i in enumerate(ins)}
        sv=s.save_pushes(ins);prefix_saves=set();frame_ebp=[]
        def stop_args(k):
            n=0;j=k-1;sv=s.save_pushes(ins)
            while j>=0:
                m=ins[j].mnemonic
                if m=='push':
                    if j not in sv:n+=1
                elif m in('call','pop','ret','jmp') or m.startswith('j') or (m in('add','sub','and') and esp_dst(ins[j])) or (m=='mov' and 'fs:' in ins[j].op_str and ins[j].op_str.endswith('esp')):break
                j-=1
            return n
        def tr_state(k,st,infer_n):
            i=ins[k];mn=i.mnemonic;base,d,ebp,fb=st
            if mn=='push':
                d+=4
                if k in sv and not infer_n is None and k not in prefix_saves:fb+=4
            elif mn=='pop':
                d-=4
                if reg(i,0)=='ebp':ebp=frame_ebp[0] if (frame_ebp and getattr(s,'ebp_restore',True)) else None
                if reg(i,0) in('ebx','ebp','esi','edi') and any(x.mnemonic=='ret' for x in ins[k+1:k+14]) and not any(x.mnemonic in('call','jmp') or x.mnemonic in JCC for x in ins[k+1:k+14]):fb-=4
            elif mn=='leave':
                if ebp is None:raise Unsupported('leave sin marco')
                base,d=ebp;d-=4;ebp=None
            elif mn in('add','sub') and esp_dst(i):
                if i.operands[1].type!=X86_OP_IMM:raise Unsupported('esp variable')
                d+=i.operands[1].imm*(-1 if mn=='add' else 1)
            elif mn=='and' and esp_dst(i):base,d=i.address,0
            elif mn=='mov' and esp_dst(i):
                if reg(i,1)=='ebp' and ebp is not None:base,d=ebp
                elif reg(i,1)=='ebp' and ebp is None and ins[0].mnemonic not in('push','sub'):base,d=0,4  # fragmento que empieza a mitad de la función madre: ebp apunta al ebp guardado
                else:raise Unsupported('mov esp no soportado')
            elif mn=='lea' and esp_dst(i):
                o=i.operands[1].mem
                if s.cs.reg_name(o.base)=='ebp' and not o.index and ebp is not None:base,d=ebp;d-=o.disp
                else:raise Unsupported('lea esp no soportado')
            elif mn=='mov' and reg(i,0)=='ebp' and reg(i,1)=='esp':
                ebp=(base,d)
                if not frame_ebp:frame_ebp.append(ebp)
            elif mn=='call':
                nxt=i.address+i.size
                if i.operands[0].type==X86_OP_IMM:
                    n,pops=s.call_info(i.operands[0].imm,i,ins,k)
                    if i.operands[0].imm==0x63ca98:
                        ebp=(base,d+4)  # la cabecera SEH deja ebp apuntando a su copia guardada
                        if not frame_ebp:frame_ebp.append(ebp)
                    if pops:d-=4*n
                else:
                    n=infer_n(k,base,d,fb);d-=4*n
            return (base,d,ebp,fb)
        # profundidad de marco: tras el prólogo (guardas de registros, cabecera SEH, locales reservados, `sub esp`)
        st=(0,0,None,0);base_ref=0;d_base=0;lead=True
        for k,i in enumerate(ins):
            mn=i.mnemonic
            if mn in('call','ret','jmp') or mn in JCC:break
            fr=False
            if lead:
                if mn=='push' or (mn in('sub','and') and esp_dst(i)) or (mn=='mov' and (('fs:' in i.op_str) or (reg(i,0)=='ebp' and reg(i,1)=='esp'))):fr=True
                elif mn=='mov' and i.operands[0].type==X86_OP_REG and i.operands[1].type==X86_OP_MEM and 'fs:' in i.op_str:fr=True
                else:lead=False
            if not fr and mn=='push' and k in sv:fr=True
            if mn=='and' and esp_dst(i):base_ref=i.address
            st=tr_state(k,st,None)
            if fr and mn!='mov':d_base=st[1];base_ref=st[0];prefix_saves.add(k)
        res={}
        def demand1(k,P,mode):
            """Palabras que deben seguir pendientes tras la llamada indirecta `k` (hay `P` pendientes) porque las retira después un consumidor conocido
            (llamada directa con `ret N`, `add esp`, `pop`) y no alcanzan las apiladas después. Recorrido lineal hasta la primera bifurcación; las llamadas
            indirectas posteriores se suponen que retiran todo lo apilado desde la anterior. Un consumo que rebasaría `P` toca el marco (epílogo): se detiene."""
            avail=0;deficit=0
            def take(m):
                nonlocal avail,deficit
                if avail>=m:avail-=m;return True
                if deficit+m-avail>P:return False
                deficit+=m-avail;avail=0;return True
            def frame_epilogue(j):
                """¿Los `pop`/`add esp` desde `j` hasta el `ret` liberan justo el marco (`d_base`)?"""
                w=0
                for x in ins[j:j+14]:
                    if x.mnemonic=='pop':w+=1
                    elif x.mnemonic=='add' and esp_dst(x) and x.operands[1].type==X86_OP_IMM:w+=x.operands[1].imm//4
                    elif x.mnemonic=='ret':return 4*w==d_base
                    elif x.mnemonic in('mov','lea','xor','movzx','or','and','test','cmp','nop'):continue
                    else:return False
                return False
            for j in range(k+1,min(len(ins),k+160)):
                i=ins[j];mn=i.mnemonic
                if mn=='push':avail+=1
                elif mn=='pop':
                    if avail:avail-=1
                    elif frame_epilogue(j):break
                    elif not take(1):break
                elif mn=='add' and esp_dst(i) and i.operands[1].type==X86_OP_IMM:
                    if avail<i.operands[1].imm//4 and frame_epilogue(j):break
                    if not take(i.operands[1].imm//4):break
                elif mn=='call':
                    if i.operands[0].type==X86_OP_IMM:
                        n,pops=s.call_info(i.operands[0].imm,i,ins,j)
                        m=max(0,n) if pops else 0
                    else:m=avail if mode=='all' else 0
                    if not take(m):break
                elif mn in('ret','jmp') or mn in JCC:break
                elif mn=='mov' and esp_dst(i) or mn=='and' and esp_dst(i) or mn=='leave':break
            return deficit,avail
        def demand(k,P):
            """Elige entre suponer que las llamadas indirectas posteriores retiran sus argumentos (`all`, thiscall) o no (`none`, convención cdecl de las clases sgl):
            vale la que deja la pila vacía al final del tramo."""
            ra=demand1(k,P,'all');rb=demand1(k,P,'none')
            if ra[1]==0:return ra[0]
            if rb[1]==0:return rb[0]
            return ra[0]
        def infer(k,base,d,fb):
            nx=ins[k].address+ins[k].size
            if (s.cur_name,nx) in s.nover:
                res[nx]=s.nover[(s.cur_name,nx)];return res[nx]
            try:
                if base_ref is None or base!=base_ref:raise Unsupported('llamada indirecta con base de pila distinta')
                q,r=divmod(d-d_base-fb,4)
                if r or q<0:raise Unsupported(f'argumentos de llamada indirecta no deducibles en {ins[k].address:#x}')
                q=max(0,q-demand(k,q))
                if s.localargs:q=min(q,stop_args(k))
            except Unsupported:
                if not s.guess:raise
                q=0
            res[nx]=q;return q
        seen={ins[0].address:(0,0,None,0)};work=[ins[0].address];s.dbg=(seen,d_base,res)
        while work:
            a=work.pop();k=index[a];i=ins[k];mn=i.mnemonic;st=seen[a]
            if mn=='ret':
                if (st[0]!=0 or st[1]!=0) and not s.guess:raise Unsupported(f'pila desequilibrada en ret {a:#x}: {st[1]}')
                continue
            nxt=a+i.size
            if mn=='jmp' and i.operands[0].type!=X86_OP_IMM:
                o=i.operands[0]
                if o.type==X86_OP_REG or (o.type==X86_OP_MEM and not (o.mem.index and o.mem.scale==4 and o.mem.base==0)):
                    if (st[0]!=0 or st[1]!=0) and not s.guess:raise Unsupported(f'pila desequilibrada en salto virtual {a:#x}: {st[1]}')
                    continue
                succ=s.jtab_targets(i)
            elif mn=='jmp':
                t=i.operands[0].imm
                if t in by:succ=[t]
                else:
                    if (st[0]!=0 or st[1]!=0) and not s.guess:raise Unsupported(f'pila desequilibrada en salto final {a:#x}: {st[1]}')
                    continue
            elif mn in JCC:succ=[i.operands[0].imm,nxt]
            else:succ=[nxt]
            ns=tr_state(k,st,infer)
            for t in succ:
                if t not in by:raise Unsupported('flujo sale de la función')
                if t in seen:
                    if seen[t][:3]!=ns[:3] and not s.guess:raise Unsupported(f'pila incoherente en {t:#x}')  # `fb` (bytes de guardas) solo sirve para deducir llamadas indirectas: una guarda condicional no es incoherencia
                else:seen[t]=ns;work.append(t)
        return res
    def check_depth(s,ins,by,spec):
        """Profundidad de pila estática: debe ser coherente en las confluencias y valer 0 en cada `ret` (valida las cuentas de argumentos de las llamadas)."""
        depth={ins[0].address:0};work=[ins[0].address]
        while work:
            a=work.pop();d=depth[a];i=by[a];mn=i.mnemonic;nxt=a+i.size
            if mn=='push':d+=4
            elif mn=='pop':d-=4
            elif mn=='leave':raise Unsupported('leave')
            elif mn in('add','sub') and i.operands[0].type==X86_OP_REG and s.cs.reg_name(i.operands[0].reg)=='esp':
                d+= i.operands[1].imm*(-1 if mn=='add' else 1)
            elif mn=='call':
                st=spec['sites'].get(nxt)
                if st and st['pops']:d-=4*st['n']
            succ=[]
            if mn=='ret':
                if d!=0:raise Unsupported(f'pila desequilibrada en ret {a:#x}: {d}')
                continue
            if mn=='jmp' and i.operands[0].type!=X86_OP_IMM:
                o=i.operands[0]
                if o.type==X86_OP_REG or (o.type==X86_OP_MEM and not (o.mem.index and o.mem.scale==4 and o.mem.base==0)):
                    if d!=0:raise Unsupported(f'pila desequilibrada en salto virtual {a:#x}: {d}')
                    continue
                succ=s.jtab_targets(i)
            elif mn=='jmp':
                t=i.operands[0].imm
                if t in by:succ=[t]
                else:
                    if d!=0:raise Unsupported(f'pila desequilibrada en salto final {a:#x}: {d}')
            elif mn in JCC:succ=[i.operands[0].imm,nxt]
            else:succ=[nxt]
            for t in succ:
                if t not in by:raise Unsupported('flujo sale de la función')
                if t in depth:
                    if depth[t]!=d:raise Unsupported(f'pila incoherente en {t:#x}')
                else:depth[t]=d;work.append(t)
    def save_pushes(s,ins):
        """Índices de los `push reg` que son guardas de registros preservados: primera aparición de cada registro que se saca (`pop`) antes de un `ret`."""
        popped=set()
        for k,i in enumerate(ins):
            lo_,hi_=ins[0].address,ins[-1].address+ins[-1].size
            if i.mnemonic=='ret' or (i.mnemonic=='jmp' and (i.operands[0].type!=X86_OP_IMM or not lo_<=i.operands[0].imm<hi_)):
                j=k-1
                lim=k-14
                while j>=0 and j>=lim and not (ins[j].mnemonic in('call','jmp','ret','push','leave') or ins[j].mnemonic.startswith('j')):
                    if ins[j].mnemonic=='pop':popped.add(s.cs.reg_name(ins[j].operands[0].reg))
                    j-=1
        res=set();seen=set()
        for k,i in enumerate(ins):
            if i.mnemonic=='push' and i.operands[0].type==X86_OP_REG:
                r=s.cs.reg_name(i.operands[0].reg)
                if r in popped and r not in seen:res.add(k)
                seen.add(r)
        return res
    def pending_args(s,ins,k,leaders,saves):
        n=0;j=k-1
        while j>=0:
            m=ins[j].mnemonic
            if m=='push':
                if j not in saves:n+=1
            elif m in('call','pop','ret','jmp') or m.startswith('j') or (m in('add','sub') and ins[j].operands[0].type==X86_OP_REG and s.cs.reg_name(ins[j].operands[0].reg)=='esp'):break
            if ins[j].address in leaders and j!=k-1:break
            j-=1
        return n
    def call_info(s,t,i,ins,k):
        if t==0x63ca98:return -4,True  # cabecera SEH del CRT (`mov eax,manejador; call 0x63ca98`): deja 16 bytes más en la pila (ver el emisor)
        pops=s.callee_pops(t)
        if pops is not None:return pops//4,True
        # cdecl o sin `ret`: los argumentos los retira el llamador (`add esp, N` justo después)
        nxt=ins[k+1] if k+1<len(ins) else None
        if nxt is not None and nxt.mnemonic=='add' and nxt.operands[0].type==X86_OP_REG and s.cs.reg_name(nxt.operands[0].reg)=='esp' and nxt.operands[1].type==X86_OP_IMM:
            return nxt.operands[1].imm//4,False
        return 0,False
def mask(w):return (1<<w)-1
def sximm(v,w):return v&mask(w)
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--binary',required=True);ap.add_argument('--catalog',required=True)
    ap.add_argument('--reachable',action='store_true',help='decode reachable blocks, excluding inline data');ap.add_argument('--comments',action='store_true');ap.add_argument('--lax',action='store_true');ap.add_argument('--nover',default=None);ap.add_argument('--guess',action='store_true');ap.add_argument('--localargs',action='store_true');ap.add_argument('--fpu-exit',action='store_true');ap.add_argument('--ext',action='store_true',help='solo el modo ampliado (pila/SEH/x87)');ap.add_argument('--size',action='append',default=[],help='DIR=BYTES: tamaño real cuando el catálogo recorta la función');ap.add_argument('--out',required=True);ap.add_argument('--spec',required=True);ap.add_argument('funcs',nargs='+')
    a=ap.parse_args();global COMMENTS;COMMENTS=a.comments;L=Lifter(a.binary,a.catalog);L.lax=a.lax;L.guess=a.guess;L.localargs=a.localargs;L.fpuexit=a.fpu_exit;L.reachable=a.reachable
    for o_ in a.size:L.sizes[int(o_.split('=')[0],16)]=int(o_.split('=')[1],0)
    if a.nover:
        for nm,sites in json.load(open(a.nover)).items():
            for site,n in sites.items():L.nover[(nm,int(site))]=n
    src=["//! Funciones portadas automáticamente con `reimpl/tools/lift.py` (modelo `p3_core::x86rt`); no editar a mano: regenerar.","#![allow(unused_mut, unused_variables, unused_assignments, clippy::all)]","use crate::economy::ops_misc::OpsHost;","use crate::flatmem::Mem;","use crate::x86rt::*;",""]
    specs=[];unsup={}
    for f in a.funcs:
        ad=int(f,16);r=L.cat[ad];nm=r['name'];sz=L.extent(ad) if ad in L.cat else int(r['bytes'])
        for o in a.size:
            k,v=o.split('=')
            if int(k,16)==ad:sz=int(v,0)
        try:
            L.ext=a.ext;code,spec=L.lift(ad,sz,nm)
        except Unsupported as e:
            if a.ext:unsup[f]=str(e);continue
            try:L.ext=True;code,spec=L.lift(ad,sz,nm);spec['ext']=True
            except Unsupported as e2:unsup[f]=str(e);continue
            finally:L.ext=False
        except KeyError as e:unsup[f]='sin catálogo';continue
        src.append(code);specs.append(spec)
    src.append("pub fn call_by_name(name: &str, m: &mut dyn Mem, host: &mut dyn OpsHost, regs: [u32; 8]) -> Option<[u32; 8]> {\n    Some(match name {")
    for sp in specs:src.append(f'        "{sp["name"]}" => {sp["name"]}(m, host, regs),')
    src.append("        _ => return None,\n    })\n}\n")
    Path(a.out).write_text("\n".join(src));Path(a.spec).write_text(json.dumps(dict(funcs=specs,unsupported=unsup),indent=1))
    print(len(specs),'traducidas;',len(unsup),'sin soporte');
    from collections import Counter;print(Counter(unsup.values()).most_common(12))
if __name__=='__main__':main()
