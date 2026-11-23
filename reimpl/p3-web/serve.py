#!/usr/bin/env python3
"""Servidor local de p3-web: sirve www/ con las cabeceras que exige SharedArrayBuffer (COOP/COEP). Uso: python serve.py [puerto]"""
import base64, functools, hmac, http.server, json, os, ssl, sys, urllib.parse

class H(http.server.SimpleHTTPRequestHandler):
    extensions_map = {**http.server.SimpleHTTPRequestHandler.extensions_map, ".wasm": "application/wasm", ".js": "text/javascript"}
    # Solo para pruebas locales: con P3_GAME_DIR=carpeta, /_game/NOMBRE sirve archivos de esa carpeta (el servidor escucha solo en 127.0.0.1).
    # P3_IDIOMAS=carpeta (opcional): si un p2arch0_<código>.cpr o idioma/*.pack no está en P3_GAME_DIR se busca ahí (p. ej. la de p3-juego).
    def translate_path(self, path):
        game = os.environ.get("P3_GAME_DIR")
        if game and path.startswith("/_game/"):
            rel = urllib.parse.unquote(path.split("?")[0][len("/_game/"):])
            parts = [p for p in rel.split("/") if p not in ("", ".", "..")]
            found = os.path.join(game, *parts)
            langs = os.environ.get("P3_IDIOMAS")
            if langs and not os.path.exists(found) and os.path.exists(os.path.join(langs, *parts)):
                return os.path.join(langs, *parts)
            return found
        return super().translate_path(path)

    # GET /_game/ -> {"exe": "Patrician3.exe", "cpr": "p2arch0_eng.cpr", "cprs": [...], "extra": ["idioma/spa.pack"], "music": [...]} (solo eso se ofrece; el móvil no puede elegir carpeta)
    # Con P3_PASSWORD definida se exige contraseña (Basic, usuario cualquiera) en TODO: el servidor sirve el EXE, el CPR y el .wasm.
    def _authorized(self):
        pw = os.environ.get("P3_PASSWORD")
        if not pw:
            return True
        h = self.headers.get("Authorization", "")
        if h.startswith("Basic "):
            try:
                given = base64.b64decode(h[6:]).decode("utf8", "replace").partition(":")[2]
                if hmac.compare_digest(given.encode(), pw.encode()):
                    return True
            except Exception:
                pass
        self.send_response(401); self.send_header("WWW-Authenticate", 'Basic realm="Patrician III"'); self.send_header("Content-Length", "0"); self.end_headers()
        return False

    def do_HEAD(self):
        if self._authorized():
            super().do_HEAD()

    def do_GET(self):
        if not self._authorized():
            return
        if self.path.split("?")[0] == "/_game/":
            game = os.environ.get("P3_GAME_DIR", "")
            names = os.listdir(game) if game and os.path.isdir(game) else []
            exe = next((n for n in names if n.lower() == "patrician3.exe"), None)
            cprs = sorted(n for n in names if n.lower().endswith(".cpr"))
            # Música suelta (carpeta Musik/**): sin ella el juego no suena
            music = []
            for d in (n for n in names if n.lower() == "musik" and os.path.isdir(os.path.join(game, n))):
                for base, _, fs in os.walk(os.path.join(game, d)):
                    music += [os.path.relpath(os.path.join(base, f), game).replace(os.sep, "/") for f in sorted(fs)]
            # Vídeos de las ventanas de suceso (Videos/*.sks, Vto.dll sustituida): sin ellos sale la imagen fija
            for d in (n for n in names if n.lower() == "videos" and os.path.isdir(os.path.join(game, n))):
                music += [d + "/" + f for f in sorted(os.listdir(os.path.join(game, d))) if f.lower().endswith(".sks")]
            # Idiomas: todos los p2arch0_<código>.cpr y los paquetes idioma/*.pack (menús traducidos), de P3_GAME_DIR y de P3_IDIOMAS
            langs, extra = [], []
            for base in [game, os.environ.get("P3_IDIOMAS", "")]:
                if not base or not os.path.isdir(base):
                    continue
                for n in sorted(os.listdir(base)):
                    if n.lower().startswith("p2arch0_") and n.lower().endswith(".cpr") and n.lower() not in [m.lower() for m in langs]:
                        langs.append(n)
                for d in (n for n in os.listdir(base) if n.lower() == "idioma" and os.path.isdir(os.path.join(base, n))):
                    extra += [f"idioma/{f}" for f in sorted(os.listdir(os.path.join(base, d))) if f.lower().endswith(".pack") and f"idioma/{f}" not in extra]
            body = json.dumps({"exe": exe, "cpr": next((n for n in cprs if n.lower() == "p2arch0_eng.cpr"), next((n for n in cprs if n.lower().startswith("p2arch0")), cprs[0] if cprs else None)), "cprs": langs, "extra": extra, "music": music}).encode()
            self.send_response(200); self.send_header("Content-Type", "application/json"); self.send_header("Content-Length", str(len(body))); self.end_headers(); self.wfile.write(body)
            return
        rel = urllib.parse.unquote(self.path.split("?")[0][len("/_game/"):]).lower()
        if self.path.startswith("/_game/") and not (os.environ.get("P3_GAME_DIR") and (rel.endswith((".exe", ".cpr")) and "/" not in rel or rel.startswith("musik/") and ".." not in rel or rel.startswith("idioma/") and rel.endswith(".pack") and rel.count("/") == 1 and ".." not in rel)):
            self.send_error(404); return
        # p3_web.wasm.gz (más nuevo que el .wasm, o sin .wasm): se sirve comprimido con Content-Encoding (menos datos al móvil).
        if self.path.split("?")[0].endswith(".wasm"):
            plain = self.translate_path(self.path)
            gz = plain + ".gz"
            if os.path.isfile(gz) and (not os.path.isfile(plain) or os.path.getmtime(gz) >= os.path.getmtime(plain)):
                data = open(gz, "rb").read()
                self.send_response(200); self.send_header("Content-Type", "application/wasm"); self.send_header("Content-Encoding", "gzip")
                self.send_header("Content-Length", str(len(data))); self.end_headers(); self.wfile.write(data)
                return
        super().do_GET()

    def end_headers(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        self.send_header("Cache-Control", "no-store")
        super().end_headers()

if __name__ == "__main__":
    # Uso: python serve.py [puerto] [--tls CERT KEY]. Siempre en 127.0.0.1: para el móvil se pone delante `tailscale serve` (https privado de tu tailnet).
    args = sys.argv[1:]
    tls = None
    if "--tls" in args:
        i = args.index("--tls"); tls = args[i + 1:i + 3]; del args[i:i + 3]
    port = int(args[0]) if args else 8080
    root = os.path.join(os.path.dirname(os.path.abspath(__file__)), "www")
    srv = http.server.ThreadingHTTPServer(("127.0.0.1", port), functools.partial(H, directory=root))
    if tls:
        ctx = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER); ctx.load_cert_chain(*tls); srv.socket = ctx.wrap_socket(srv.socket, server_side=True)
    srv.serve_forever()
