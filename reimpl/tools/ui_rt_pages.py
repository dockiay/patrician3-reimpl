"""Barrido de páginas de las ventanas de edificio por el enlazador de interfaz: como `ui_rt_buildings.py` (escena interior con la
pintura bajo el pergamino, ventana creada como en `0x00425EEE` y abierta por `+0x120`), y además elige la página como el menú lateral
(`select_page(u8)` sobre la ventana global, p. ej. `0x005A4CC4` para la oficina) y ejecuta su `update` antes de dibujar, como el bucle
de pantallas. Anota píxeles y funciones ausentes por ventana y página.
Uso (desde `reimpl/`): `P3_SAVE=partida.pat [ARGLOG=fichero] python tools/ui_rt_pages.py DIR_DEL_JUEGO SALIDA.json DIR_PNG [VENTANA ...]`."""
import json, os, re, subprocess, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parent.parent
A, OUT, PNG = sys.argv[1:4]
ONLY = set(sys.argv[4:])
PNG = Path(PNG)
B = str(ROOT / "target/release/examples/ui_window")
PNG.mkdir(parents=True, exist_ok=True)
env = dict(os.environ, P3_CALLS="100000000", P3_HOST="1", P3_SCENE="1")
if os.environ.get("ARGLOG"):
    env["P3_ARGLOG"] = os.environ["ARGLOG"]
# ventana: (select_page, update, páginas, global de la ventana, menú lateral); el número de páginas sale de la tabla de saltos o la comparación de cada select_page.
PAGES = {l.split()[0]: l.split()[1:] for l in open(ROOT / "p3-mvp/examples/ui_building_pages.tsv") if not l.startswith("#") and l.strip()}
res = []
for l in open(ROOT / "p3-mvp/examples/ui_building_windows.tsv"):
    if l.startswith("#"):
        continue
    name, size, ctor, scene = l.split()
    if ONLY and name not in ONLY or name not in PAGES:
        continue
    sel, upd, pages, wglob, menu = PAGES[name]
    for page in range(int(pages)):
        steps = ["0x10000", "0x465010", "0x4653a0:s=./scripts/animation.ini,0", "4024b0:451,537,0", "glob:6cc7e0", "child", "4b21a0:0",
                 "0x466320", f"0x466430:{scene}", "0x4665d0"]
        if menu != "-":
            # menú lateral del edificio, creado como en `0x00425EEE` (maqueta `sidemenu.ini`); no se cuelga de la contenedora: las
            # páginas solo lo consultan (p. ej. la taberna limpia su selección con `0x005A6E20` sobre `[0x006E54F8]`).
            msize, mctor, mglob = menu.split(":")
            steps += [f"new:{msize}:{mctor}", "v8:s=./scripts/sidemenu.ini,0", f"glob:{mglob[2:]}"]
        steps += [f"new:{size}:{ctor}", f"glob:{wglob[2:]}", "v8:s=./scripts/BuildingParchment.ini,0", "4024b0:425,510,0", "4b21a0:0", "v120"]
        if sel != "-":
            steps.append(f"{sel}:{page}")
        if upd != "-":
            steps.append(upd)
        steps.append("^0x4b58c0:0,0,0,0")
        png = PNG / f"{name}_p{page}.png"
        try:
            r = subprocess.run([B, A + "/Patrician3.exe", A + "/p2arch0_eng.cpr", str(png)] + steps,
                               capture_output=True, text=True, timeout=300, env=env)
            out = r.stdout
            st = "ok" if r.returncode == 0 else "pánico: " + (re.findall(r"panicked at ([^\n]*\n[^\n]*)", r.stderr) or ["?"])[0][:200]
        except subprocess.TimeoutExpired:
            out, st = "", "tiempo agotado"
        px = re.search(r"píxeles dibujados: (\d+)", out)
        miss = re.search(r"faltan \(función, llamante\): \{(.*)\}", out)
        missing = sorted(set(re.findall(r"\(([0-9a-f]+), ([0-9a-f]+)\)", miss.group(1)))) if miss else []
        other = [m for m in re.findall(r"(sgl[^\n]*falta[^\n]*|import[^\n]*falta[^\n]*)", out)]
        aborted = "abortado true" in out
        res.append(dict(window=name, page=page, status=st, aborted=aborted, pixels=int(px.group(1)) if px else None,
                        missing=missing, other=other))
        print(name, page, st, "abortado" if aborted else "", res[-1]["pixels"], missing[:12], other[:3], flush=True)
json.dump(res, open(OUT, "w"), indent=1)
