"""Eleva la carga de mapas y creación del jugador para el enlazador UI.
Uso: python reimpl/tools/ui_rt_newgame.py --binary Patrician3.exe
"""
import argparse, csv, json, os, re, subprocess, sys, tempfile
os.environ["PYTHONUTF8"] = "1"
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "p3-mvp/src/ui_rt/newgame_lifted.rs"
CATALOG = ROOT.parent / "research/catalog/functions.tsv"
# Mapas, operaciones y callbacks del calentamiento inicial del mundo.
SEEDS = [
    0x442C20, 0x448040, 0x46F560, 0x49C410, 0x4B0450, 0x4C1B20, 0x4D2EC0, 0x4D2ED0,
    0x4D4A40, 0x4D6530, 0x4D6680, 0x4D6B40, 0x4D8B10, 0x4D8E70, 0x4DB330, 0x4DB7C0,
    0x4DBB00, 0x4DBBD0, 0x4DCA90, 0x4DCBF0, 0x4DCEA0, 0x4DDA40, 0x4DE4F0, 0x4DE790,
    0x4DEBA0, 0x4E2144, 0x4E2634, 0x4E3F44, 0x4E43E4, 0x4E4984, 0x4E8234, 0x4E8E24,
    0x4F4E50, 0x4F5080, 0x4F5130, 0x4F68D0, 0x4F72C0, 0x4F73D0, 0x4F7510, 0x4F9550,
    0x4F9B40, 0x4F9C80, 0x4FA1D0, 0x4FA740, 0x4FB8F0, 0x4FB9C0, 0x4FBD60, 0x4FBDB0,
    0x4FBEC0, 0x4FBFF0, 0x4FC130, 0x4FC1D0, 0x4FC4A0, 0x4FC630, 0x4FC8C0, 0x4FC990,
    0x4FCBD0, 0x4FD830, 0x4FE190, 0x5017F0, 0x501E10, 0x503D40, 0x503D50, 0x503D80,
    0x503DB0, 0x504970, 0x5051E0, 0x507D60, 0x5097C0, 0x509930, 0x509A50, 0x509AE0,
    0x509F80, 0x50A580, 0x50E380, 0x50F100, 0x50FE00, 0x5108E0, 0x5127E0, 0x512830,
    0x512B20, 0x512BA0, 0x512C50, 0x513BF0, 0x513D80, 0x513EC0, 0x5159C0, 0x516170,
    0x516930, 0x516B10, 0x517260, 0x517E00, 0x5181D0, 0x5193F0, 0x5196B0, 0x51A4E0,
    0x51A750, 0x51AB20, 0x51C360, 0x51D410, 0x51E8D0, 0x51F100, 0x51FAB0, 0x5205D0,
    0x521900, 0x523830, 0x5240A0, 0x524140, 0x5254C0, 0x526A50, 0x526AA0, 0x528630,
    0x5289B0, 0x528E60, 0x5299F0, 0x529F20, 0x52A020, 0x52A7D0, 0x52A8D0, 0x52AC80,
    0x52BD80, 0x530470, 0x531BA0, 0x531BD0, 0x531D50, 0x533510, 0x5352B0, 0x538A80,
    0x53F8B0, 0x541D70, 0x542420, 0x542560, 0x542860, 0x542A30, 0x542E40, 0x5434E0,
    0x543570, 0x543810, 0x544830, 0x5448A0, 0x544DE0, 0x544E00, 0x544E30, 0x544E50,
    0x545890, 0x5458D0, 0x545B20, 0x545BE0, 0x545CB0, 0x545F50, 0x5462F0, 0x549940,
    0x549C40, 0x54A480, 0x54AA70, 0x54BD60, 0x54C4C0, 0x54C810, 0x54CA20, 0x54E170,
    0x562C10, 0x563090, 0x613920,
]

ap = argparse.ArgumentParser()
ap.add_argument("--binary", required=True)
a = ap.parse_args()
with tempfile.TemporaryDirectory() as tmp:
    out, spec = Path(tmp) / "n.rs", Path(tmp) / "n.json"
    names = {int(r["address"], 16): r["name"] for r in csv.DictReader(CATALOG.open(encoding="utf-8"), delimiter="\t")}
    nover = Path(tmp) / "nover.json"
    # get_width() no retira el argumento pendiente de la llamada siguiente.
    import pefile
    from capstone import Cs, CS_ARCH_X86, CS_MODE_32
    pe = pefile.PE(a.binary, fast_load=True)
    # Las llamadas virtuales de este método son controles de ventana.
    slots = {0x44: 0, 0x48: 0, 0x64: 3, 0x6c: 0, 0x70: 0, 0xcc: 1}
    sites = {}
    for ins in Cs(CS_ARCH_X86, CS_MODE_32).disasm(pe.get_data(0x49c410 - 0x400000, 0xb20), 0x49c410):
        match = re.fullmatch(r"dword ptr \[e\w\w \+ (0x[0-9a-f]+)\]", ins.op_str)
        if ins.mnemonic == "call" and match:
            slot = int(match.group(1), 16)
            if slot not in slots:
                raise SystemExit(f"Ranura desconocida: {slot:#x}")
            sites[str(ins.address + ins.size)] = slots[slot]
    nover.write_text(json.dumps({names[0x49C410]: sites}), encoding="utf-8")
    subprocess.run([sys.executable, str(ROOT / "tools/lift.py"), "--binary", a.binary, "--catalog", str(CATALOG), "--lax", "--ext", "--fpu-exit", "--nover", str(nover),
                    "--out", str(out), "--spec", str(spec)] + [f"{x:#x}" for x in SEEDS], check=True, capture_output=True)
    import json
    report = json.loads(spec.read_text(encoding="utf-8"))
    src = out.read_text(encoding="utf-8").split("\npub fn call_by_name")[0]
    # El modo de pila lineal no puede fusionar ciertas ramas con cleanup
    # diferido. El elevador normal conserva ESP explícitamente por bloque.
    if report["unsupported"]:
        subprocess.run([sys.executable, str(ROOT / "tools/lift.py"), "--binary", a.binary, "--catalog", str(CATALOG), "--lax", "--fpu-exit", "--nover", str(nover),
                        "--out", str(out), "--spec", str(spec)] + list(report["unsupported"]), check=True, capture_output=True)
        fallback = json.loads(spec.read_text(encoding="utf-8"))
        if fallback["unsupported"]:
            raise SystemExit(fallback["unsupported"])
        src += "\npub fn " + out.read_text(encoding="utf-8").split("\npub fn call_by_name")[0].split("pub fn ", 1)[1]
chunks = re.split(r"(?m)^(?=pub fn )", src)[1:]
body, rows = [], []
for c in chunks:
    pc = re.search(r"let mut pc: u32 = (0x[0-9a-f]+);", c).group(1)
    f = int(pc, 16)
    c = re.sub(r"^pub fn \w+\(", f"pub fn f_{f:08x}(", c, count=1)
    c = re.sub(r"c\.push\(m,(rm\(m,[^;]*?\))\);", r"{ let v = \1; c.push(m, v); }", c)
    # Estos auxiliares CRT devuelven EDX:EAX y __allshr recibe CL; no pueden
    # pasar por OpsHost, cuyo retorno es solo EAX. Reutilizar el modelo probado.
    for native in (0x63a6a0, 0x63a6e0, 0x63c610, 0x639c28):
        c = re.sub(rf"\{{ let args = \[[^\n]*\]; c.r\[0\] = host.call\(m, {native:#x}, c.r\[1\], &args\); \}}",
                   f"p3_core::economy::seh_pending::native(&mut c, m, {native:#x});", c)
    body.append(c)
    rows.append(f)
head = ["//! Generado por `reimpl/tools/ui_rt_newgame.py` con `tools/lift.py` (no editar a mano): carga de mapas y creación del jugador de partida nueva.",
        "#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]",
        "use p3_core::economy::ops_misc::OpsHost;", "use p3_core::flatmem::Mem;", "use p3_core::x86rt::*;", "",
        f"pub static NEWGAME: [(u32, super::core_table::CoreFn); {len(rows)}] = ["]
head += [f"    ({x:#x}, f_{x:08x})," for x in rows] + ["];", ""]
OUT.write_text("\n".join(head) + "".join(body), encoding="utf-8")
print(f"{len(rows)} funciones en {OUT.relative_to(ROOT)}")
