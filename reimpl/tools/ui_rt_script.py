"""Eleva el intérprete de guiones de misión (0x4ECF64) para el enlazador de interfaz.
Uso: python reimpl/tools/ui_rt_script.py --binary Patrician3.exe
Escribe p3-mvp/src/ui_rt/script_lifted.rs. El catálogo recorta la función: su tamaño real (hasta las tablas de saltos
0x4F2E34/0x4F3054) es 0x5ED0."""
import argparse, os, re, subprocess, sys, tempfile
os.environ["PYTHONUTF8"] = "1"
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "p3-mvp/src/ui_rt/script_lifted.rs"
CATALOG = ROOT.parent / "research/catalog/functions.tsv"
SEEDS = [0x4ECF64, 0x4F3860, 0x4F38A0, 0x4F3680, 0x4F38F0, 0x4F3960, 0x4D7B10,
         # captadores diminutos de game_world/ships/merchant/name_bank (0x4F3660..0x4F3960)
         0x4F38D0, 0x4F38E0, 0x4F3660, 0x4F3690, 0x4F36A0, 0x4F36B0, 0x4F36C0, 0x4F3740, 0x4F3760, 0x4F37C0,
         0x4F37D0, 0x4F3850, 0x4F3870, 0x4F3890, 0x4F38B0, 0x4F3940,
         # llamadas que el intérprete y las operaciones hacen y faltaban
         0x533210, 0x4D69D0, 0x4D7960, 0x5483D0, 0x535400,
         0x4D7030, 0x50DF00, 0x514D40, 0x539B50, 0x539C60, 0x53CE30]
# Funciones que van apareciendo ausentes al correr tutorial/campaña (una dirección hexadecimal por línea): se elevan con las demás.
EXTRA = ROOT / "tools/ui_rt_script.seeds"
if EXTRA.exists():
    SEEDS += [int(l.split()[0], 16) for l in EXTRA.read_text().splitlines() if l.strip() and not l.startswith("#")]
SEEDS = list(dict.fromkeys(SEEDS))
ap = argparse.ArgumentParser()
ap.add_argument("--binary", required=True)
a = ap.parse_args()
with tempfile.TemporaryDirectory() as tmp:
    out, spec = Path(tmp) / "n.rs", Path(tmp) / "n.json"
    subprocess.run([sys.executable, str(ROOT / "tools/lift.py"), "--binary", a.binary, "--catalog", str(CATALOG), "--lax", "--ext", "--fpu-exit",
                    "--size", "0x4ecf64=0x5ed0", "--out", str(out), "--spec", str(spec)] + [f"{x:#x}" for x in SEEDS], check=True, capture_output=True)
    import json
    for k, v in json.loads(spec.read_text(encoding="utf-8")).get("unsupported", {}).items():
        print(f"sin soporte {k}: {v}", file=sys.stderr)
    src = out.read_text(encoding="utf-8").split("\npub fn call_by_name")[0]
chunks = re.split(r"(?m)^(?=pub fn )", src)[1:]
body, rows = [], []
for c in chunks:
    f = int(re.search(r"let mut pc: u32 = (0x[0-9a-f]+);", c).group(1), 16)
    c = re.sub(r"^pub fn \w+\(", f"pub fn f_{f:08x}(", c, count=1)
    c = re.sub(r"c\.push\(m,(rm\(m,[^;]*?\))\);", r"{ let v = \1; c.push(m, v); }", c)
    for native in (0x63a6a0, 0x63a6e0, 0x63c610, 0x639c28):
        c = re.sub(rf"\{{ let args = \[[^\n]*\]; c.r\[0\] = host.call\(m, {native:#x}, c.r\[1\], &args\); \}}",
                   f"p3_core::economy::seh_pending::native(&mut c, m, {native:#x});", c)
    body.append(c); rows.append(f)
head = ["//! Generado por `reimpl/tools/ui_rt_script.py` con `tools/lift.py` (no editar a mano): intérprete de guiones de misión `0x004ECF64`.",
        "#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]",
        "use p3_core::economy::ops_misc::OpsHost;", "use p3_core::flatmem::Mem;", "use p3_core::x86rt::*;", "",
        f"pub static SCRIPT: [(u32, super::core_table::CoreFn); {len(rows)}] = ["]
head += [f"    ({x:#x}, f_{x:08x})," for x in rows] + ["];", ""]
OUT.write_text("\n".join(head) + "".join(body), encoding="utf-8")
print(f"{len(rows)} funciones en {OUT.relative_to(ROOT)}")
