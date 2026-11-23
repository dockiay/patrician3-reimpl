"""Barrido de las ventanas de edificio por el enlazador de interfaz con la receta del juego: escena interior
(`BuildingSceneWindow`) con su pintura, ventana de contenido creada como en `0x00425EEE` (maqueta, 425x510, oculta), abierta por su
método virtual `+0x120` y dibujada por la pantalla contenedora. Con `--board`, la pintura queda bajo el pergamino (`0x4665D0`).
Anota píxeles, funciones ausentes y, en `ARGLOG`, las llamadas con argumentos mal deducidos (entrada de `tools/ui_rt_fix.py`).
Uso (desde `reimpl/`): `python tools/ui_rt_buildings.py DIR_DEL_JUEGO SALIDA.json DIR_PNG [ARGLOG] [--board]`."""
import json, os, re, subprocess, sys
from pathlib import Path
ROOT = Path(__file__).resolve().parent.parent
args = [a for a in sys.argv[1:] if not a.startswith("--")]
A, OUT, PNG = args[0], args[1], Path(args[2])
ARGLOG = args[3] if len(args) > 3 else None
BOARD = "--board" in sys.argv
B = str(ROOT / "target/release/examples/ui_window")
PNG.mkdir(parents=True, exist_ok=True)
env = dict(os.environ, P3_CALLS="100000000", P3_HOST="1", P3_SCENE="1")
if ARGLOG:
    env["P3_ARGLOG"] = ARGLOG
res = []
for l in open(ROOT / "p3-mvp/examples/ui_building_windows.tsv"):
    if l.startswith("#"):
        continue
    name, size, ctor, scene = l.split()
    steps = ["0x10000", "0x465010", "0x4653a0:s=./scripts/animation.ini,0", "4024b0:451,537,0", "glob:6cc7e0", "child", "4b21a0:0",
             "0x466320", f"0x466430:{scene}"] + (["0x4665d0"] if BOARD else []) + \
            [f"new:{size}:{ctor}", "v8:s=./scripts/BuildingParchment.ini,0", "4024b0:425,510,0", "4b21a0:0", "v120", "^0x4b58c0:0,0,0,0"]
    try:
        r = subprocess.run([B, A + "/Patrician3.exe", A + "/p2arch0_eng.cpr", str(PNG / f"{name}.png")] + steps,
                           capture_output=True, text=True, timeout=180, env=env)
        out = r.stdout
        st = "ok" if r.returncode == 0 else "pánico: " + (re.findall(r"panicked at ([^\n]*)", r.stderr) or ["?"])[0][:120]
    except subprocess.TimeoutExpired:
        out, st = "", "tiempo agotado"
    px = re.search(r"píxeles dibujados: (\d+)", out)
    miss = re.search(r"faltan \(función, llamante\): \{(.*)\}", out)
    missing = sorted(set(re.findall(r"\(([0-9a-f]+), [0-9a-f]+\)", miss.group(1)))) if miss else []
    aborted = "abortado true" in out
    res.append(dict(window=name, ctor=ctor, scene=int(scene), status=st, aborted=aborted, pixels=int(px.group(1)) if px else None, missing=missing))
    print(name, st, "abortado" if aborted else "", res[-1]["pixels"], missing[:10], flush=True)
json.dump(res, open(OUT, "w"), indent=1)
