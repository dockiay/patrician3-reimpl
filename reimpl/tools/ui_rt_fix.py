"""Vuelve a elevar, para el enlazador de interfaz, las funciones de `p3_core::lifted` cuyas llamadas virtuales tienen mal deducido
el número de argumentos, y escribe `p3-mvp/src/ui_rt/fix_lifted.rs` con su tabla por dirección (se consulta antes que la de p3-core).

`tools/lift.py` deduce del patrón de pila cuántos argumentos retira cada llamada indirecta; cuando el método real retira otros
(p. ej. `get_height()` con un argumento pendiente de la llamada siguiente, o `set_position(x, y, z)` contado como uno), la pila del
llamante queda desplazada al enlazar las funciones entre sí. Las pruebas de paridad no lo ven porque sus llamadas son ganchos que
retiran lo deducido. El enlazador anota los casos al ejecutar (`P3_ARGLOG=fichero` en `ui_window`: «llamante retorno reales
deducidas») y esta herramienta los acumula en `tools/ui_rt_fix.sites` y regenera con `lift.py --nover`.

Uso (desde `reimpl/`): `python tools/ui_rt_fix.py --binary Patrician3.exe [registro...]`."""
import argparse, json, re, subprocess, sys, tempfile
from pathlib import Path
import pefile

ROOT = Path(__file__).resolve().parent.parent
SITES = ROOT / "tools/ui_rt_fix.sites"
OUT = ROOT / "p3-mvp/src/ui_rt/fix_lifted.rs"
CATALOG = ROOT.parent / "research/catalog/functions.tsv"
# Funciones que llaman a rutinas matemáticas del CRT (`sqrt` `0x63AAE4`, `_ftol` `0x639C28`): solo valen con `--fpu-exit`, que las
# modela en línea (sin él quedan como llamadas ausentes sobre la pila x87). 0x4901C0: iconos de artillería de los convoyes de la Hansa.
# 0x40C5F0 (`convoy_choice_window_layout`, ventana «Create convoy» del panel del barco): `ceil` 0x63A5CE → `fild` → `fmul` → `_ftol`;
# sin `--fpu-exit` el resultado de `ceil` no llegaba a la pila x87 del elevado, ancho y alto salían 0 y la ventana no se veía.
# Se re-elevan aunque no tengan sitios de llamada virtual en `ui_rt_fix.sites`.
FPU_FIRST = {0x4901C0, 0x40C5F0}
# El selector del armero necesita _ftol incluso cuando el modo ampliado no
# puede unir sus ramas de pila. Mantener x87 también en la alternativa normal.
FPU_FALLBACK = {0x5EBD70}

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--binary", required=True)
    ap.add_argument("logs", nargs="*")
    # `--only 0xF[,0xG]`: re-eleva solo esas funciones y conserva el resto de `fix_lifted.rs` tal cual (sin volver a elevar las
    # otras 500 largas con un `lift.py` que pudo cambiar desde entonces).
    ap.add_argument("--only", default="")
    a = ap.parse_args()
    only = {int(x, 16) for x in a.only.split(",") if x}
    pe = pefile.PE(a.binary, fast_load=True)
    names = {}
    for l in CATALOG.read_text().splitlines()[1:]:
        p = l.split("\t")
        if p[0].startswith("0x"):
            names[int(p[0], 16)] = p[1]
    sites = {}
    if SITES.exists():
        for l in SITES.read_text().split("\n"):
            if l and not l.startswith("#"):
                c, r, n = l.split()
                sites[(int(c, 16), int(r, 16))] = int(n)
    for log in a.logs:
        for l in Path(log).read_text().splitlines():
            c, r, real, _ = l.split()
            c, r = int(c, 16), int(r, 16)
            # Solo llamadas indirectas: las directas (`E8 rel32`) las resuelve el elevador contra el destino.
            if pe.get_data(r - 0x400000 - 5, 1) == b"\xe8" or c not in names:
                continue
            sites[(c, r)] = int(real)
    SITES.write_text("# llamante retorno argumentos_reales (generado por tools/ui_rt_fix.py a partir de P3_ARGLOG)\n" +
                     "".join(f"{c:#010x} {r:#010x} {n}\n" for (c, r), n in sorted(sites.items())))
    wanted = {c for c, _ in sites} | FPU_FIRST
    funcs = sorted(wanted & only) if only else sorted(wanted)
    body, rows, failed = [], [], []
    with tempfile.TemporaryDirectory() as tmp:
        out, spec, nover = Path(tmp) / "f.rs", Path(tmp) / "f.json", Path(tmp) / "nover.json"
        for f in funcs:
            nover.write_text(json.dumps({names[f]: {str(r): n for (c, r), n in sites.items() if c == f}}))
            src = ""
            # `--fpu-exit` primero: modela en línea las rutinas x87 del CRT (`_ftol`, `sqrt`, `acos`...; `Cpu::crt_call`) y recoge con `fret` el
            # ST0 que devuelven los llamados elevados; sin él, una llamada a `_ftol` o `sqrt` va al enlazador, que no ve la pila x87.
            attempts = [["--lax", "--ext", "--fpu-exit"], ["--lax", "--ext"]]
            if f in FPU_FALLBACK:
                attempts.append(["--lax", "--fpu-exit"])
            attempts.append(["--lax"])
            for flags in attempts:
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
                failed.append(f)
                continue
            chunk = re.sub(r"^pub fn \w+\(", f"pub fn f_{f:08x}(", chunks[0], count=1)
            chunk = re.sub(r"c\.push\(m,(rm\(m,[^;]*?\))\);", r"{ let v = \1; c.push(m, v); }", chunk)
            # Rutas del CRT con resultado en EDX/ST0 que `host.call` (solo EAX, sin la pila x87) pierde: se hacen en línea (grupo lib, 14/11;
            # `__allmul` 0x63A6A0 añadida el 14/11: sin ella `select_nodes` 0x444800 perdía la parte alta y las rutas cruzaban tierra).
            chunk = re.sub(r"\{ let args = \[\]; c\.r\[0\] = host\.call\(m, 0x639c28, c\.r\[1\], &args\); \}", "c.crt_call(m, 0x639c28);", chunk)
            for native in (0x63a6a0, 0x63a6e0, 0x63c610):
                chunk = re.sub(rf"\{{ let args = \[[^\n]*\]; c\.r\[0\] = host\.call\(m, {native:#x}, c\.r\[1\], &args\); \}}", f"p3_core::economy::seh_pending::native(&mut c, m, {native:#x});", chunk)
            body.append(f"/// `{names[f]}` (`{f:#010x}`).\n" + chunk)
            rows.append(f)
    if only and OUT.exists():
        old = OUT.read_text()
        pat = r"(?ms)^/// `\w+` \(`(0x[0-9a-f]{8})`\)\.\n.*?(?=^/// `\w+` \(`0x[0-9a-f]{8}`\)\.\n|\Z)"
        kept = {int(m.group(1), 16): m.group(0) for m in re.finditer(pat, old)}
        for f, chunk in zip(rows, body):
            kept[f] = chunk
        rows = sorted(kept)
        body = [kept[f] if kept[f].endswith("\n") else kept[f] + "\n" for f in rows]
    head = ["//! Generado por `reimpl/tools/ui_rt_fix.py` con `tools/lift.py --nover` (no editar a mano): funciones de `p3_core::lifted`",
            "//! elevadas de nuevo con los argumentos reales de sus llamadas virtuales (`tools/ui_rt_fix.sites`).",
            "#![allow(unused_mut, unused_variables, unused_assignments, non_snake_case, clippy::all)]",
            "use p3_core::economy::ops_misc::OpsHost;", "use p3_core::flatmem::Mem;", "use p3_core::x86rt::*;", "",
            f"pub static FIX: [(u32, super::core_table::CoreFn); {len(rows)}] = ["]
    head += [f"    ({x:#x}, f_{x:08x})," for x in rows] + ["];", ""]
    OUT.write_text("\n".join(head) + "".join(body))
    print(f"{len(rows)} funciones en {OUT.relative_to(ROOT)}; sin elevar: {', '.join(f'{x:#x}' for x in failed) or 'ninguna'}")

main()
