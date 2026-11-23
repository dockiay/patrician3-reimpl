"""Genera `p3-mvp/src/ui_rt/hand_table.rs`: direcciones del original → funciones de interfaz escritas a mano en `p3-core`
(`ui_input`, `ui_hud`, `dialogs::confirm`) con la firma `(m, host, this, esp)`. La dirección es la primera que cita su comentario `///`.
Uso: `python tools/ui_rt_hand.py` desde `reimpl/`."""
import re
from pathlib import Path
ROOT = Path(__file__).resolve().parent.parent
SRC = {"ui_dtors": "ui_dtors.rs", "ui_input": "ui_input.rs", "ui_hud": "ui_hud.rs", "dialogs::confirm": "dialogs/confirm.rs"}
SIG = re.compile(r"^pub fn (\w+)\(\s*_?m: &mut dyn Mem,\s*_?host: &mut dyn OpsHost,\s*_?this: u32,\s*_?esp: u32,?\s*\)(?: -> (u32|u8|u16))?")
rows = {}
for mod, f in SRC.items():
    L = (ROOT / "p3-core/src" / f).read_text().split("\n")
    for i, l in enumerate(L):
        m = SIG.match(l)
        if not m:
            continue
        j, doc = i - 1, []
        while j >= 0 and L[j].startswith("///"):
            doc.append(L[j]); j -= 1
        a = re.search(r"`0x0*([0-9A-Fa-f]{6})`", " ".join(reversed(doc)))
        if a:
            rows.setdefault(int(a.group(1), 16), (mod, m.group(1), m.group(2)))
out = ["//! Generado por `reimpl/tools/ui_rt_hand.py`; no editar a mano.", "use p3_core::economy::ops_misc::OpsHost;", "use p3_core::flatmem::Mem;", "",
       "pub type HandFn = fn(&mut dyn Mem, &mut dyn OpsHost, u32, u32) -> u32;", ""]
for addr, (mod, name, ret) in sorted(rows.items()):
    out.append(f"fn w_{addr:06x}(m: &mut dyn Mem, h: &mut dyn OpsHost, this: u32, esp: u32) -> u32 {{")
    out.append(f"    p3_core::{mod}::{name}(m, h, this, esp)" + (" as u32" if ret in ("u8", "u16") else "") if ret else f"    p3_core::{mod}::{name}(m, h, this, esp);\n    0")
    out.append("}")
out += ["", f"pub static HAND: [(u32, HandFn); {len(rows)}] = ["]
out += [f"    ({a:#x}, w_{a:06x})," for a in sorted(rows)]
out += ["];", ""]
(ROOT / "p3-mvp/src/ui_rt/hand_table.rs").write_text("\n".join(out))
print(len(rows), "funciones")
