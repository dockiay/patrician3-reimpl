"""Eleva con tools/lift.py las funciones ausentes de un grupo (hilo «Enlazar las funciones que faltan», 14/11) para el enlazador ui_rt.
Uso: python reimpl/tools/ui_rt_grp.py GRUPO --binary Patrician3.exe [--size 0xDIR=0xTAM ...]
Lee tools/ui_rt_grp_GRUPO.seeds (una dirección hex por línea; # comenta) y escribe p3-mvp/src/ui_rt/grp_GRUPO_lifted.rs.
Para una función cuyo tamaño del catálogo está recortado, pasar --size."""
import argparse, os, re, subprocess, sys, tempfile
os.environ["PYTHONUTF8"] = "1"
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

CATALOG = ROOT.parent / "research/catalog/functions.tsv"
ap = argparse.ArgumentParser()
ap.add_argument("group")
ap.add_argument("--binary", required=True)
ap.add_argument("--size", action="append", default=[])
ap.add_argument("--catalog", default=str(CATALOG), help="catálogo a usar (p. ej. uno con filas añadidas para funciones sin catalogar)")
a = ap.parse_args()
OUT = ROOT / f"p3-mvp/src/ui_rt/grp_{a.group}_lifted.rs"
SEEDS = [int(l.split()[0], 16) for l in (ROOT / f"tools/ui_rt_grp_{a.group}.seeds").read_text().splitlines() if l.strip() and not l.startswith("#")]
SEEDS = list(dict.fromkeys(SEEDS))
with tempfile.TemporaryDirectory() as tmp:
    out, spec = Path(tmp) / "n.rs", Path(tmp) / "n.json"
    nov = ROOT / f"tools/ui_rt_grp_{a.group}.nover"  # opcional: {nombre: {retorno: nº de argumentos}} para llamadas indirectas
    nv = ["--nover", str(nov)] if nov.exists() else []
    subprocess.run([sys.executable, str(ROOT / "tools/lift.py"), *nv, "--binary", a.binary, "--catalog", a.catalog, "--lax", "--ext", "--fpu-exit"] + [
                    "--out", str(out), "--spec", str(spec)] + [t for s in a.size for t in ("--size", s)] + [f"{x:#x}" for x in SEEDS], check=True, capture_output=True)
    import json
    for k, v in json.loads(spec.read_text(encoding="utf-8")).get("unsupported", {}).items():
        print(f"sin soporte {k}: {v}", file=sys.stderr)
    src = out.read_text(encoding="utf-8").split("\npub fn call_by_name")[0]
chunks = re.split(r"(?m)^(?=pub fn )", src)[1:]
body, rows = [], []
for c in chunks:
    f = int(re.search(r"let mut pc: u32 = (0x[0-9a-f]+);", c).group(1), 16)
    c = re.sub(r"^pub fn [^(]+\(", f"pub fn f_{f:08x}(", c, count=1)
    c = re.sub(r"c\.push\(m,(rm\(m,[^;]*?\))\);", r"{ let v = \1; c.push(m, v); }", c)
    for native in (0x63a6a0, 0x63a6e0, 0x63c610, 0x639c28):
        c = re.sub(rf"\{{ let args = \[[^\n]*\]; c.r\[0\] = host.call\(m, {native:#x}, c.r\[1\], &args\); \}}",
                   f"p3_core::economy::seh_pending::native(&mut c, m, {native:#x});", c)
    body.append(c); rows.append(f)
head = [f"//! Generado por `reimpl/tools/ui_rt_grp.py {a.group}` con `tools/lift.py` (no editar a mano): grupo `{a.group}`.",
        "#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]",
        "use p3_core::economy::ops_misc::OpsHost;", "use p3_core::flatmem::Mem;", "use p3_core::x86rt::*;", "",
        f"pub static GRP_{a.group.upper()}: [(u32, super::core_table::CoreFn); {len(rows)}] = ["]
head += [f"    ({x:#x}, f_{x:08x})," for x in rows] + ["];", ""]
OUT.write_text("\n".join(head) + "".join(body), encoding="utf-8")
print(f"{len(rows)} funciones en {OUT.relative_to(ROOT)}")
