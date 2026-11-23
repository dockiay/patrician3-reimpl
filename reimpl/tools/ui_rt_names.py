"""Eleva con `tools/lift.py` la carga del banco de nombres del original (`name_bank_load_name_files` `0x005121C0`, que lee
`NamenStaedte_eng.txt` y los ficheros de nombres de pila/apellido por el CPR) a `p3-mvp/src/ui_rt/names_lifted.rs`, para sembrar
el banco `0x006DDAA0` en el enlazador de interfaz (las páginas de edificios leen de él los nombres de ciudades y comerciantes).
Uso (desde `reimpl/`): `python tools/ui_rt_names.py --binary Patrician3.exe`."""
import argparse, re, subprocess, sys, tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "p3-mvp/src/ui_rt/names_lifted.rs"
CATALOG = ROOT.parent / "research/catalog/functions.tsv"
SEEDS = [0x5121C0, 0x5125E0, 0x5128D0, 0x51E440, 0x512730, 0x4F6420, 0x4F6450]

ap = argparse.ArgumentParser()
ap.add_argument("--binary", required=True)
a = ap.parse_args()
with tempfile.TemporaryDirectory() as tmp:
    out, spec = Path(tmp) / "n.rs", Path(tmp) / "n.json"
    subprocess.run([sys.executable, str(ROOT / "tools/lift.py"), "--binary", a.binary, "--catalog", str(CATALOG), "--lax", "--ext",
                    "--out", str(out), "--spec", str(spec)] + [f"{x:#x}" for x in SEEDS], check=True, capture_output=True)
    src = out.read_text().split("\npub fn call_by_name")[0]
chunks = re.split(r"(?m)^(?=pub fn )", src)[1:]
body, rows = [], []
for c in chunks:
    pc = re.search(r"let mut pc: u32 = (0x[0-9a-f]+);", c).group(1)
    f = int(pc, 16)
    c = re.sub(r"^pub fn \w+\(", f"pub fn f_{f:08x}(", c, count=1)
    c = re.sub(r"c\.push\(m,(rm\(m,[^;]*?\))\);", r"{ let v = \1; c.push(m, v); }", c)
    body.append(c)
    rows.append(f)
head = ["//! Generado por `reimpl/tools/ui_rt_names.py` con `tools/lift.py` (no editar a mano): carga del banco de nombres.",
        "#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]",
        "use p3_core::economy::ops_misc::OpsHost;", "use p3_core::flatmem::Mem;", "use p3_core::x86rt::*;", "",
        f"pub static NAMES: [(u32, super::core_table::CoreFn); {len(rows)}] = ["]
head += [f"    ({x:#x}, f_{x:08x})," for x in rows] + ["];", ""]
OUT.write_text("\n".join(head) + "".join(body))
print(f"{len(rows)} funciones en {OUT.relative_to(ROOT)}")
