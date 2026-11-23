#!/usr/bin/env python3
"""WEB-6: compila, reduce y empaqueta el juego web para uso local y sin conexión.

    python web/build_web.py                      # cargo + wasm-opt + dist/
    python web/build_web.py --wasm x.wasm        # empaqueta un .wasm ya compilado
    python web/build_web.py --site reimpl/p3-web/www   # añade la página de p3-web

Salida en `web/dist/` (ignorada por Git): `p3.<hash>.wasm` (+ `.br`/`.gz`), `build.json`,
`sw.js`, `manifest.webmanifest`, iconos y `pwa.js`/`p3-load.js`. Sírvase con
`python web/serve.py`. Uso privado: no se publica en ninguna web (el .wasm contiene
código traducido del EXE; ver coordination/ARQUITECTURA_WEB.md).
"""
import argparse, gzip, hashlib, json, os, platform, shutil, subprocess, sys, time, urllib.request, zipfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
REIMPL = HERE.parent
SHELL = HERE / "shell"
BINARYEN = "123"


def sh(cmd, **kw):
    print("+", " ".join(map(str, cmd)), flush=True)
    subprocess.run(list(map(str, cmd)), check=True, **kw)


def find_wasm_opt(allow_download=True):
    """PATH -> copia en web/.tools -> npm (binaryen) -> descarga de GitHub."""
    exe = shutil.which("wasm-opt")
    if exe:
        return [exe]
    tools = HERE / ".tools"
    for cand in list(tools.glob("binaryen-*/bin/wasm-opt*")):
        if cand.suffix in ("", ".exe"):
            return [str(cand)]
    npm_bin = tools / "node_modules" / ".bin" / ("wasm-opt.cmd" if os.name == "nt" else "wasm-opt")
    if npm_bin.exists():
        return [str(npm_bin)]
    if not allow_download:
        return None
    npm = shutil.which("npm")
    if npm:
        try:
            tools.mkdir(exist_ok=True)
            sh([npm, "install", "--prefix", tools, f"binaryen@{BINARYEN}"])
            if npm_bin.exists():
                return [str(npm_bin)]
        except Exception as e:  # noqa
            print("npm falló:", e)
    sysname = {"Windows": "windows", "Linux": "linux", "Darwin": "macos"}[platform.system()]
    arch = "arm64" if platform.machine().lower() in ("arm64", "aarch64") else "x86_64"
    url = f"https://github.com/WebAssembly/binaryen/releases/download/version_{BINARYEN}/binaryen-version_{BINARYEN}-{arch}-{sysname}.tar.gz"
    try:
        tools.mkdir(exist_ok=True)
        tgz = tools / "binaryen.tar.gz"
        print("descargando", url)
        urllib.request.urlretrieve(url, tgz)
        shutil.unpack_archive(tgz, tools)
        tgz.unlink()
        for cand in tools.glob("binaryen-*/bin/wasm-opt*"):
            if cand.suffix in ("", ".exe"):
                return [str(cand)]
    except Exception as e:  # noqa
        print("descarga falló:", e)
    return None


def compress(path: Path):
    """gzip -9 siempre; brotli -q 11 si el módulo `brotli` está (pip install brotli)."""
    data = path.read_bytes()
    out = {}
    with open(str(path) + ".gz", "wb") as f:
        f.write(gzip.compress(data, 9, mtime=0))
    out["gz"] = os.path.getsize(str(path) + ".gz")
    try:
        import brotli
        with open(str(path) + ".br", "wb") as f:
            f.write(brotli.compress(data, quality=11, lgwin=24))
        out["br"] = os.path.getsize(str(path) + ".br")
    except ImportError:
        print("aviso: sin módulo brotli (pip install brotli); solo gzip")
    return out


def sha(path: Path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for b in iter(lambda: f.read(1 << 20), b""):
            h.update(b)
    return h.hexdigest()


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--wasm", help=".wasm ya compilado (omite cargo)")
    ap.add_argument("--crate", default="p3-wasm")
    ap.add_argument("--profile", default="wasm-release")
    # -O3 y -Os dejan el mismo tamaño (26,9 MB de 35,2), pero -Os va un 30 % más lento en V8 (reimpl/WEB_RENDIMIENTO.md).
    ap.add_argument("--opt", default="-O3", choices=["-O2", "-O3", "-Os", "-Oz", "none"],
                    help="nivel de wasm-opt (-O3 por defecto; none = sin wasm-opt)")
    ap.add_argument("--converge", action="store_true", help="wasm-opt --converge (lento, algo menor)")
    ap.add_argument("--site", help="carpeta estática de p3-web (index.html, js…) que se copia a dist/")
    ap.add_argument("--out", default=str(HERE / "dist"))
    ap.add_argument("--features", default="", help="--features para cargo")
    a = ap.parse_args()

    t0 = time.time()
    out = Path(a.out)
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)
    work = HERE / ".work"
    work.mkdir(exist_ok=True)

    # 1. compilar
    if a.wasm:
        src = Path(a.wasm)
    else:
        if not (REIMPL / a.crate).exists():
            sys.exit(f"no existe reimpl/{a.crate}: usa --wasm RUTA o espera a WEB-0")
        sh(["rustup", "target", "add", "wasm32-unknown-unknown"], cwd=REIMPL)
        cmd = ["cargo", "build", "--profile", a.profile, "-p", a.crate, "--target", "wasm32-unknown-unknown"]
        if a.features:
            cmd += ["--features", a.features]
        sh(cmd, cwd=REIMPL)
        cands = list((REIMPL / "target" / "wasm32-unknown-unknown" / a.profile).glob("*.wasm"))
        if not cands:
            sys.exit("cargo no produjo ningún .wasm")
        src = max(cands, key=lambda p: p.stat().st_size)
    raw = src.stat().st_size

    # 2. wasm-opt
    opt = work / "p3.opt.wasm"
    used_opt = False
    if a.opt != "none":
        wo = find_wasm_opt()
        if wo:
            cmd = wo + [a.opt, "--all-features", "--strip-debug", "--strip-producers",
                        "--strip-dwarf", str(src), "-o", str(opt)]
            if a.converge:
                cmd.insert(len(wo) + 1, "--converge")
            sh(cmd)
            used_opt = True
        else:
            print("AVISO: no hay wasm-opt (instala binaryen o npm); se empaqueta sin reducir")
    if not used_opt:
        shutil.copyfile(src, opt)

    # 3. nombre con hash, compresión
    digest = sha(opt)
    wasm_name = f"p3.{digest[:12]}.wasm"
    shutil.copyfile(opt, out / wasm_name)
    sizes = {"raw": raw, "opt": opt.stat().st_size}
    sizes.update(compress(out / wasm_name))

    # 4. cascarón de la página
    for f in SHELL.iterdir():
        if f.name in ("sw.js",):
            continue
        shutil.copyfile(f, out / f.name)
    if a.site:
        site = Path(a.site)
        if not site.is_dir():
            sys.exit(f"--site {site} no es una carpeta")
        # el .wasm suelto de desarrollo (www/p3_web.wasm) no se copia: el de dist es `p3.<hash>.wasm`
        shutil.copytree(site, out, dirs_exist_ok=True, ignore=shutil.ignore_patterns("*.wasm", ".gitignore"))
    index = out / "index.html"
    html = index.read_text(encoding="utf-8")
    inject = ""
    if "manifest.webmanifest" not in html:
        inject += '<link rel="manifest" href="manifest.webmanifest">\n<meta name="theme-color" content="#1b1208">\n'
    if "pwa.js" not in html:
        inject += '<script type="module" src="pwa.js"></script>\n'
    if inject:
        html = html.replace("</head>", inject + "</head>", 1) if "</head>" in html else inject + html
        index.write_text(html, encoding="utf-8")

    # 5. manifiesto de precarga y service worker
    files = sorted(p.relative_to(out).as_posix() for p in out.rglob("*")
                   if p.is_file() and not p.name.endswith((".gz", ".br")) and p.name != "sw.js")
    h = hashlib.sha256()
    for rel in files:
        h.update(rel.encode())
        h.update(sha(out / rel).encode())
    version = h.hexdigest()[:12]
    build = {"version": version, "wasm": wasm_name, "files": files, "sizes": sizes,
             "built": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
    (out / "build.json").write_text(json.dumps(build, indent=1), encoding="utf-8")
    sw = (SHELL / "sw.js").read_text(encoding="utf-8")
    (out / "sw.js").write_text(sw.replace("__VERSION__", version), encoding="utf-8")
    # comprime también el resto de recursos de texto grandes
    for rel in files:
        if rel.endswith((".js", ".html", ".json", ".css", ".webmanifest", ".svg")):
            compress(out / rel)

    mb = lambda n: f"{n/1048576:.1f} MB"
    print(f"\nlisto: {out}  (versión {version}, {time.time()-t0:.0f} s)")
    print(f"  .wasm: {mb(sizes['raw'])} -> {mb(sizes['opt'])} tras wasm-opt"
          + "".join(f" -> {mb(sizes[k])} {k}" for k in ("gz", "br") if k in sizes))
    print("  servir: python web/serve.py   y abrir http://localhost:8765/")


if __name__ == "__main__":
    main()
