"""Eleva con `tools/lift.py` el código de biblioteca (CRT y MFC enlazados en el EXE, a partir de `0x00639000`) que piden las
ventanas en el enlazador de interfaz, y escribe `p3-mvp/src/ui_rt/lib_lifted.rs` con su tabla por dirección.

No es un porte con paridad del juego: son rutinas de biblioteca (CString, listas MFC, iteradores de vectores...) que el
enlazador necesita para ejecutar los portes; se cierran por sus llamadas directas a otras rutinas de biblioteca. Lo que
el enlazador ya sustituye en nativo (`EXCLUDE`) no se eleva.

Uso (desde `reimpl/`): `python tools/ui_rt_lib.py --binary Patrician3.exe DIRECCIÓN...`; las direcciones semilla se añaden a
`tools/ui_rt_lib.seeds`, que conserva el conjunto para regenerar."""
import argparse, re, subprocess, sys, tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SEEDS = ROOT / "tools/ui_rt_lib.seeds"
OUT = ROOT / "p3-mvp/src/ui_rt/lib_lifted.rs"
CATALOG = ROOT.parent / "research/catalog/functions.tsv"
LIB_START = 0x00639000
# Sustituidas en nativo por `ui_rt::Runtime::builtin` (asignador, memoria, cadenas, CString, sprintf/sscanf).
EXCLUDE = {0x64F7B9, 0x63CB41, 0x64F7E2, 0x63CB7F, 0x63A1B6, 0x63A29F, 0x64F253, 0x63A790, 0x63AEC0, 0x639F20, 0x63ABE0,
           0x63ADD0, 0x63ADE0, 0x639CDA, 0x64F2C1, 0x64F390, 0x64CE80, 0x64DF80, 0x639D74, 0x648DD0, 0x63BA90, 0x63C115,
           0x63AD50, 0x63B4A3, 0x63E410, 0x64F142, 0x641427, 0x641488, 0x64EB5D, 0x639E28, 0x643DB0, 0x63C47B, 0x639CF0}

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--binary", required=True)
    ap.add_argument("seeds", nargs="*")
    a = ap.parse_args()
    catalog = {int(l.split("\t")[0], 16) for l in CATALOG.read_text().splitlines()[1:] if l.startswith("0x")}
    seeds = {int(x, 16) for x in SEEDS.read_text().split()} if SEEDS.exists() else set()
    seeds |= {int(x, 16) for x in a.seeds}
    SEEDS.write_text("".join(f"{x:#010x}\n" for x in sorted(seeds)))
    want = {x for x in seeds if x >= LIB_START and x not in EXCLUDE and x in catalog}
    failed = set()
    with tempfile.TemporaryDirectory() as tmp:
        out, spec = Path(tmp) / "lib.rs", Path(tmp) / "lib.json"
        for _ in range(12):
            todo = sorted(want - failed)
            r = subprocess.run([sys.executable, str(ROOT / "tools/lift.py"), "--binary", a.binary, "--catalog", str(CATALOG),
                                "--lax", "--ext", "--out", str(out), "--spec", str(spec)] + [f"{x:#x}" for x in todo],
                               capture_output=True, text=True)
            src = out.read_text()
            got = {int(x, 16) for x in re.findall(r"let mut pc: u32 = (0x[0-9a-f]+);", src)}
            failed |= set(todo) - got
            callees = {int(x, 16) for x in re.findall(r"host\.call\(m, (0x[0-9a-f]+),", src)}
            new = {x for x in callees if x >= LIB_START and x not in EXCLUDE and x in catalog} - want
            if not new:
                break
            want |= new
    src = src.split("\npub fn call_by_name")[0]
    body = []
    rows = []
    for chunk in re.split(r"(?m)^(?=pub fn )", src)[1:]:
        m = re.search(r"let mut pc: u32 = (0x[0-9a-f]+);", chunk)
        addr = int(m.group(1), 16)
        chunk = re.sub(r"^pub fn \w+\(", f"pub fn f_{addr:08x}(", chunk, count=1)
        # `c.push(m, rm(m, ...))` toma `m` dos veces: se evalúa antes el valor.
        chunk = re.sub(r"c\.push\(m,(rm\(m,[^;]*?\))\);", r"{ let v = \1; c.push(m, v); }", chunk)
        # `__aulldiv` (0x6461A0) y `__aullrem` (0x646210): `stdcall`, 64 bits sin signo; devuelven EDX:EAX (el anfitrión solo devuelve EAX).
        for fa, op in ((0x6461a0, "checked_div"), (0x646210, "checked_rem")):
            chunk = re.sub(r"\{ let args = \[m\.r32\(c\.r\[4\]\.wrapping_add\(4\)\), m\.r32\(c\.r\[4\]\.wrapping_add\(8\)\), m\.r32\(c\.r\[4\]\.wrapping_add\(12\)\), m\.r32\(c\.r\[4\]\.wrapping_add\(16\)\)\]; c\.r\[0\] = host\.call\(m, " + hex(fa) + r", c\.r\[1\], &args\); \}",
                           "{ let (a, b) = (u64::from(m.r32(c.r[4].wrapping_add(4))) | u64::from(m.r32(c.r[4].wrapping_add(8))) << 32, u64::from(m.r32(c.r[4].wrapping_add(12))) | u64::from(m.r32(c.r[4].wrapping_add(16))) << 32); let q: u64 = a." + op + "(b).unwrap_or(0); c.r[0] = q as u32; c.r[2] = (q >> 32) as u32; }", chunk)
        body.append(chunk)
        rows.append(addr)
    head = ["//! Generado por `reimpl/tools/ui_rt_lib.py` con `tools/lift.py` (no editar a mano): CRT/MFC del EXE que necesita el",
            "//! enlazador de interfaz. Rutinas de biblioteca, sin prueba de paridad propia.",
            "#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]",
            "use p3_core::economy::ops_misc::OpsHost;", "use p3_core::flatmem::Mem;", "use p3_core::x86rt::*;", "",
            f"pub static LIB: [(u32, super::core_table::CoreFn); {len(rows)}] = ["]
    head += [f"    ({x:#x}, f_{x:08x})," for x in sorted(rows)] + ["];", ""]
    OUT.write_text("\n".join(head) + "".join(body))
    print(f"{len(rows)} rutinas en {OUT.relative_to(ROOT)}; sin elevar: {', '.join(f'{x:#x}' for x in sorted(failed)) or 'ninguna'}")

main()
