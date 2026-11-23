#!/usr/bin/env python3
"""Servidor local de WEB-6: sirve web/dist/ con aislamiento de origen cruzado.

    python web/serve.py [--port 8765] [--dir web/dist] [--host 127.0.0.1]

- COOP same-origin + COEP require-corp + CORP: activan SharedArrayBuffer.
- Sirve `X.br` / `X.gz` precomprimidos si el navegador los acepta (los crea build_web.py).
- Tipos correctos (`application/wasm` es obligatorio para compilar en streaming).
- Sw/index/build.json sin caché; archivos `p3.<hash>.wasm` inmutables.
- Solo 127.0.0.1 por defecto. Fuera de localhost los navegadores desactivan service
  worker y SharedArrayBuffer salvo con HTTPS; es uso privado, no lo publiques.
"""
import argparse, email.utils, mimetypes, os, re, sys
from http import HTTPStatus
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

HERE = Path(__file__).resolve().parent
TYPES = {".wasm": "application/wasm", ".js": "text/javascript; charset=utf-8", ".mjs": "text/javascript; charset=utf-8",
         ".json": "application/json", ".webmanifest": "application/manifest+json", ".html": "text/html; charset=utf-8",
         ".css": "text/css; charset=utf-8", ".png": "image/png", ".svg": "image/svg+xml"}
NO_CACHE = {"sw.js", "index.html", "build.json", "manifest.webmanifest", ""}


class Handler(SimpleHTTPRequestHandler):
    server_version = "p3-serve"

    def log_message(self, fmt, *args):
        if os.environ.get("P3_SERVE_QUIET") != "1":
            super().log_message(fmt, *args)

    def isolation(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        self.send_header("Cross-Origin-Resource-Policy", "same-origin")

    def resolve(self):
        rel = self.path.split("?", 1)[0].split("#", 1)[0]
        p = Path(self.translate_path(rel))
        if p.is_dir():
            p = p / "index.html"
        return p

    def do_GET(self): self.send(True)
    def do_HEAD(self): self.send(False)

    def send(self, body):
        p = self.resolve()
        root = Path(self.directory).resolve()
        try:
            p.resolve().relative_to(root)
        except ValueError:
            return self.send_error(HTTPStatus.FORBIDDEN)
        if not p.is_file():
            return self.send_error(HTTPStatus.NOT_FOUND)
        accept = self.headers.get("Accept-Encoding", "")
        real, enc = p, None
        for e, ext in (("br", ".br"), ("gzip", ".gz")):
            alt = Path(str(p) + ext)
            if re.search(rf"\b{e}\b", accept) and alt.is_file():
                real, enc = alt, e
                break
        size = real.stat().st_size
        start, end, status = 0, size - 1, HTTPStatus.OK
        rng = self.headers.get("Range")
        if rng and enc is None and (m := re.fullmatch(r"bytes=(\d*)-(\d*)", rng.strip())):
            a, b = m.groups()
            if a == "":
                start, end = max(0, size - int(b)), size - 1
            else:
                start, end = int(a), int(b) if b else size - 1
            end = min(end, size - 1)
            if start > end:
                self.send_response(HTTPStatus.REQUESTED_RANGE_NOT_SATISFIABLE)
                self.send_header("Content-Range", f"bytes */{size}")
                self.end_headers()
                return
            status = HTTPStatus.PARTIAL_CONTENT
        self.send_response(status)
        self.send_header("Content-Type", TYPES.get(p.suffix.lower()) or mimetypes.guess_type(str(p))[0] or "application/octet-stream")
        if enc:
            self.send_header("Content-Encoding", enc)
        self.send_header("Vary", "Accept-Encoding")
        self.send_header("Accept-Ranges", "none" if enc else "bytes")
        self.send_header("Content-Length", str(end - start + 1))
        if status == HTTPStatus.PARTIAL_CONTENT:
            self.send_header("Content-Range", f"bytes {start}-{end}/{size if not enc else size}")
        if p.suffix == ".wasm":
            self.send_header("X-P3-Size", str(p.stat().st_size))
            self.send_header("Access-Control-Expose-Headers", "X-P3-Size")
        self.send_header("Last-Modified", email.utils.formatdate(p.stat().st_mtime, usegmt=True))
        if p.name in NO_CACHE:
            self.send_header("Cache-Control", "no-cache")
        elif re.fullmatch(r"p3\.[0-9a-f]{12}\.wasm", p.name):
            self.send_header("Cache-Control", "public, max-age=31536000, immutable")
        else:
            self.send_header("Cache-Control", "no-cache")
        self.isolation()
        self.end_headers()
        if body:
            with open(real, "rb") as f:
                f.seek(start)
                left = end - start + 1
                while left > 0:
                    chunk = f.read(min(1 << 20, left))
                    if not chunk:
                        break
                    try:
                        self.wfile.write(chunk)
                    except (BrokenPipeError, ConnectionResetError):
                        return
                    left -= len(chunk)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--port", type=int, default=8765)
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--dir", default=str(HERE / "dist"))
    a = ap.parse_args()
    d = Path(a.dir)
    if not (d / "build.json").exists():
        sys.exit(f"{d} no tiene build.json: ejecuta antes `python web/build_web.py`")
    Handler.directory = str(d)
    srv = ThreadingHTTPServer((a.host, a.port), lambda *x, **k: Handler(*x, directory=str(d), **k))
    srv.daemon_threads = True
    print(f"sirviendo {d} en http://{'localhost' if a.host == '127.0.0.1' else a.host}:{a.port}/  (Ctrl+C para parar)")
    try:
        srv.serve_forever()
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
