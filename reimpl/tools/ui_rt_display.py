"""Eleva de nuevo, para `p3-mvp/src/ui_rt/display.rs`, las dos funciones que colocan la pantalla de juego según la resolución:
`game_screen_apply_resolution_layout` (`0x00429A40`) y `reposition_ui_elements_top_bar_patch` (`0x00429FE0`).

Las versiones de `p3_core::lifted` deducen del patrón de pila cuántos argumentos retira cada llamada virtual, y en las ramas
que sus trazas no recorrieron (1280×1024 sobre todo) la deducción falla: `get_width()` con el `push 0` de la llamada siguiente
contado como suyo, `set_offset(x, y)` contado como uno... Al enlazar las funciones de verdad la pila queda desplazada. Aquí el
número real sale de la ranura virtual: todos los objetos que tocan son ventanas (`window_*` de `0x00402470..0x00402660`,
`0x004BDAC0/E0`), con retirada conocida. Mismo mecanismo que `tools/ui_rt_fix.py` (`lift.py --nover`), en fichero aparte.

Uso (desde `reimpl/`): `python tools/ui_rt_display.py --binary Patrician3.exe`."""
import argparse, json, re, subprocess, sys, tempfile
from pathlib import Path
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_32

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "p3-mvp/src/ui_rt/display_lifted.rs"
CATALOG = ROOT.parent / "research/catalog/functions.tsv"
# Función → fin (exclusivo).
FUNCS = {0x00429A40: 0x00429FD1, 0x00429FE0: 0x0042A535}
# Argumentos que retira cada ranura de la tabla virtual de ventana (`ret N` de la función de la ranura).
SLOT_ARGS = {0x08: 2, 0x44: 0, 0x48: 0, 0x4C: 3, 0x64: 3, 0x68: 1, 0x6C: 0, 0x70: 0, 0x78: 2, 0x84: 1, 0x88: 0, 0x8C: 0,
             0xC8: 0, 0xCC: 1, 0x120: 1}

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--binary", required=True)
    a = ap.parse_args()
    pe = pefile.PE(a.binary, fast_load=True)
    names = {}
    for l in CATALOG.read_text().splitlines()[1:]:
        p = l.split("\t")
        if p[0].startswith("0x"):
            names[int(p[0], 16)] = p[1]
    md = Cs(CS_ARCH_X86, CS_MODE_32)
    body, rows, report = [], [], []
    with tempfile.TemporaryDirectory() as tmp:
        out, spec, nover = Path(tmp) / "f.rs", Path(tmp) / "f.json", Path(tmp) / "nover.json"
        for f, end in FUNCS.items():
            sites = {}
            for ins in md.disasm(pe.get_data(f - 0x400000, end - f), f):
                if ins.address >= end:
                    break
                mm = re.fullmatch(r"dword ptr \[e\w\w(?: \+ (0x[0-9a-f]+|\d+))?\]", ins.op_str)
                if ins.mnemonic == "call" and mm:
                    slot = int(mm.group(1) or "0", 0)
                    if slot not in SLOT_ARGS:
                        sys.exit(f"{ins.address:#x}: ranura {slot:#x} sin retirada conocida")
                    sites[ins.address + ins.size] = SLOT_ARGS[slot]
            report.append(f"{f:#x}: {len(sites)} llamadas virtuales")
            nover.write_text(json.dumps({names[f]: {str(r): n for r, n in sites.items()}}))
            src = ""
            for flags in (["--lax", "--ext"], ["--lax"], ["--lax", "--ext", "--fpu-exit"]):
                if out.exists():
                    out.unlink()
                subprocess.run([sys.executable, str(ROOT / "tools/lift.py"), "--binary", a.binary, "--catalog", str(CATALOG),
                                "--nover", str(nover), "--out", str(out), "--spec", str(spec), f"{f:#x}"] + flags,
                               capture_output=True, text=True)
                src = out.read_text().split("\npub fn call_by_name")[0] if out.exists() else ""
                if "let mut pc: u32" in src:
                    break
            chunks = re.split(r"(?m)^(?=pub fn )", src)[1:]
            if not chunks:
                sys.exit(f"{f:#x}: no se pudo elevar")
            chunk = re.sub(r"^pub fn \w+\(", f"pub fn f_{f:08x}(", chunks[0], count=1)
            chunk = re.sub(r"c\.push\(m,(rm\(m,[^;]*?\))\);", r"{ let v = \1; c.push(m, v); }", chunk)
            body.append(f"/// `{names[f]}` (`{f:#010x}`).\n" + chunk)
            rows.append(f)
    head = ["//! Generado por `reimpl/tools/ui_rt_display.py` con `tools/lift.py --nover` (no editar a mano): las dos funciones que colocan",
            "//! la pantalla de juego según la resolución, con los argumentos reales de sus llamadas virtuales.",
            "#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]",
            "use p3_core::economy::ops_misc::OpsHost;", "use p3_core::flatmem::Mem;", "use p3_core::x86rt::*;", "",
            f"pub static DISPLAY: [(u32, super::core_table::CoreFn); {len(rows)}] = ["]
    head += [f"    ({x:#x}, f_{x:08x})," for x in rows] + ["];", ""]
    OUT.write_text("\n".join(head) + "".join(body))
    print("; ".join(report), "->", OUT.relative_to(ROOT))

main()
