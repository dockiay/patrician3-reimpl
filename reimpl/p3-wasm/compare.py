#!/usr/bin/env python3
"""Compara dos JSON de los bancos (nativo y WASM): `compare.py nativo.json wasm.json`. Sale con 1 si difieren."""
import json, sys
a, b = (json.load(open(p)) for p in sys.argv[1:3])
if a == b:
    print("IGUAL"); sys.exit(0)
for k in a:
    if k == "frames":
        d = [i + 1 for i, (x, y) in enumerate(zip(a[k], b[k])) if x != y]
        print("fotogramas distintos:", d[:10], f"({len(d)}/{len(a[k])})")
    elif k == "days":
        d = [x["day"] for x, y in zip(a[k], b[k]) if x != y]
        print("días distintos:", d[:10])
    elif a[k] != b.get(k):
        print(k, a[k], b.get(k))
sys.exit(1)
