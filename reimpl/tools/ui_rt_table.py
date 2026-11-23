"""Genera `p3-mvp/src/ui_rt/core_table.rs`: dirección del original → función transcrita de `p3_core::lifted`.

La tabla deja que el enlazador de interfaz (`p3_mvp::ui_rt`) llame a las funciones elevadas por su dirección, como hace el
x86. Solo entra lo que declara `p3-core/src/lifted/mod.rs`. Uso: `python tools/ui_rt_table.py` desde `reimpl/`."""
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LIFTED = ROOT / "p3-core/src/lifted"
OUT = ROOT / "p3-mvp/src/ui_rt/core_table.rs"
SIG = re.compile(r"^pub fn (\w+)\(m: &mut dyn Mem, host: &mut dyn OpsHost, regs: \[u32; 8\]\) -> \[u32; 8\] \{\n    let mut c = Cpu \{[^\n]*\n    let mut pc: u32 = (0x[0-9a-f]+);", re.M)

mods = re.findall(r"^pub mod (\w+);", (LIFTED / "mod.rs").read_text(), re.M)
# Versiones con `_ftol`/`sqrt` en línea y retornos x87 (`--fpu-exit`): sustituyen a las de los lotes anteriores si coinciden con el x86.
PREFER = ("x87fix", "sob", "sob2")
passed = set((ROOT / "p3-core/tests/fixtures/lifted_sob_passed.txt").read_text().split()) if (ROOT / "p3-core/tests/fixtures/lifted_sob_passed.txt").exists() else set()
rows = {}
for mod in PREFER:
    if mod in mods:
        for name, addr in SIG.findall((LIFTED / f"{mod}.rs").read_text()):
            if name in passed:
                rows[int(addr, 16)] = f"p3_core::lifted::{mod}::{name}"
# El registro manda: si `functions.tsv` apunta a un módulo elevado concreto (versiones re-elevadas con los argumentos observados), se usa ese.
REG = re.compile(r"^(0x[0-9A-Fa-f]{8})\t[^\t]*\tp3_core::lifted::(\w+)::(\w+)\t", re.M)
defined = {mod: dict(SIG.findall((LIFTED / f"{mod}.rs").read_text())) for mod in mods}
for addr, mod, name in REG.findall((ROOT / "functions.tsv").read_text()):
    if mod in defined and name in defined[mod] and int(defined[mod][name], 16) == int(addr, 16):
        rows.setdefault(int(addr, 16), f"p3_core::lifted::{mod}::{name}")
for mod in sorted(mods):
    for name, addr in SIG.findall((LIFTED / f"{mod}.rs").read_text()):
        rows.setdefault(int(addr, 16), f"p3_core::lifted::{mod}::{name}")
lines = [
    "//! Generado por `reimpl/tools/ui_rt_table.py`; no editar a mano.",
    "use p3_core::economy::ops_misc::OpsHost;",
    "use p3_core::flatmem::Mem;",
    "",
    "pub type CoreFn = fn(&mut dyn Mem, &mut dyn OpsHost, [u32; 8]) -> [u32; 8];",
    "",
    f"pub static CORE: [(u32, CoreFn); {len(rows)}] = [",
]
lines += [f"    ({addr:#x}, {path}),".replace("0x", "0x", 1) for addr, path in sorted(rows.items())]
lines += ["];", ""]
OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_text("\n".join(lines))
print(f"{len(rows)} funciones en {OUT.relative_to(ROOT)}")
