"""Barrido de ventanas por el enlazador de interfaz: para cada fila de `p3-mvp/examples/ui_windows.tsv` construye la ventana,
carga su maqueta y la dibuja con `examples/ui_window` (40 s como máximo), y anota píxeles dibujados y funciones ausentes.
Uso (desde `reimpl/`, con el ejemplo compilado en release): `python tools/ui_rt_sweep.py DIR_DEL_JUEGO SALIDA.json [DIR_PNG]`."""
import subprocess, re, json, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parent.parent
A = sys.argv[1]
OUT = sys.argv[2]
PNG = Path(sys.argv[3]) if len(sys.argv) > 3 else Path("/tmp")
B = str(ROOT / "target/release/examples/ui_window")
res = []
for l in open(ROOT / "p3-mvp/examples/ui_windows.tsv"):
    if l.startswith("#"):
        continue
    n, c, lay, d = l.split()
    try:
        r = subprocess.run([B, A + "/Patrician3.exe", A + "/p2arch0_eng.cpr", str(PNG / f"{n}.png"), "4000", c, "4024b0:400,300", "=48:1", lay, d + ":0,0,0,0"],
                           capture_output=True, text=True, timeout=40)
        out = r.stdout
        st = "ok" if r.returncode == 0 else "pánico: " + (re.findall(r"panicked at ([^\n]*)", r.stderr) or ["?"])[0][:120]
    except subprocess.TimeoutExpired:
        out, st = "", "tiempo agotado"
    px = re.search(r"píxeles dibujados: (\d+)", out)
    miss = re.search(r"faltan \(función, llamante\): \{(.*)\}", out)
    missing = sorted(set(re.findall(r"\(([0-9a-f]+), [0-9a-f]+\)", miss.group(1)))) if miss else []
    res.append(dict(window=n, ctor=c, layout=lay, draw=d, status=st, pixels=int(px.group(1)) if px else None, missing=missing))
    print(n, st, res[-1]["pixels"], missing[:8], flush=True)
json.dump(res, open(OUT, "w"), indent=1)
