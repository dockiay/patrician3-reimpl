#!/usr/bin/env python3
"""Cliente de prueba de p3-stream (sin navegador): entra con contraseña, guarda el vídeo H.264 y el sonido, ejecuta un guion de
entrada y mide ancho de banda y latencia. Uso:
  P3_PASSWORD=... python3 client.py --url http://127.0.0.1:8090 --out DIR --secs 30 --guion "3:click 400 300;5:key 13"
Guion: «segundo:orden» separados por «;»; órdenes: click X Y [botón] (en píxeles del juego, 1024x768 por defecto), key VK, move X Y.
Después: ffmpeg -f h264 -i DIR/video.h264 -vf select=eq(n\\,N) -vframes 1 captura.png"""
import argparse, asyncio, base64, os, re, struct, sys, time, urllib.request, json
import websockets

ap = argparse.ArgumentParser()
ap.add_argument("--url", default="http://127.0.0.1:8090")
ap.add_argument("--out", default="out")
ap.add_argument("--secs", type=float, default=20)
ap.add_argument("--guion", default="")
ap.add_argument("--size", default="1024x768")
ap.add_argument("--wrong", action="store_true", help="comprueba que sin contraseña se rechaza")
a = ap.parse_args()
os.makedirs(a.out, exist_ok=True)
pw = os.environ.get("P3_PASSWORD", "")

def fetch(auth):
    req = urllib.request.Request(a.url + "/")
    if auth is not None:
        req.add_header("Authorization", "Basic " + base64.b64encode(b"u:" + auth.encode()).decode())
    try:
        return urllib.request.urlopen(req, timeout=10).read().decode()
    except urllib.error.HTTPError as e:
        return f"HTTP {e.code}"

async def main():
    print("sin contraseña:", fetch(None)[:30]); print("contraseña mala:", fetch("mala-mala")[:30])
    page = fetch(pw)
    m = re.search(r"const TOKEN = '([0-9a-f]+)'", page)
    assert m, "no hay token en la página"
    ws_url = a.url.replace("http", "ws") + "/ws?t="
    try:
        async with websockets.connect(ws_url + "malo"):
            print("ERROR: WebSocket sin token aceptado")
    except Exception as e:
        print("WebSocket sin token rechazado:", type(e).__name__)
    gw, gh = map(int, a.size.split("x"))
    script = []
    for part in filter(None, a.guion.split(";")):
        t, cmd = part.split(":", 1); script.append((float(t), cmd.split()))
    script.sort()
    vid = None; seg = 0; cur = None; pcm = open(os.path.join(a.out, "audio.pcm"), "wb")
    frames = []
    stats = dict(v=0, a=0, vb=0, ab=0, key=0, sizes=set(), rtt=[])
    first = {}
    async with websockets.connect(ws_url + m.group(1), max_size=None) as ws:
        t0 = time.time(); print("hello:", await ws.recv())
        async def sender():
            i = 0
            while time.time() - t0 < a.secs:
                now = time.time() - t0
                while i < len(script) and script[i][0] <= now:
                    c = script[i][1]; i += 1
                    fx = lambda x: int(int(x) * 65536 / gw); fy = lambda y: int(int(y) * 65536 / gh)
                    if c[0] in ("click", "move"):
                        await ws.send(f"m {fx(c[1])} {fy(c[2])}")
                        if c[0] == "click":
                            b = c[3] if len(c) > 3 else "0"
                            await asyncio.sleep(0.05); await ws.send(f"b {b} 1"); await asyncio.sleep(0.06); await ws.send(f"b {b} 0")
                    elif c[0] == "key":
                        await ws.send(f"k {c[1]} 1"); await asyncio.sleep(0.05); await ws.send(f"k {c[1]} 0")
                    print(f"{now:6.1f}s -> {c}")
                await ws.send(f"p {time.time()*1000}")
                await asyncio.sleep(0.25)
        task = asyncio.create_task(sender())
        while time.time() - t0 < a.secs:
            try:
                m2 = await asyncio.wait_for(ws.recv(), 1)
            except asyncio.TimeoutError:
                continue
            if isinstance(m2, str):
                if m2.startswith("p "): stats["rtt"].append(time.time()*1000 - float(m2[2:]))
                continue
            if m2[0] == 1:
                stats["v"] += 1; stats["vb"] += len(m2); stats["key"] += m2[1] & 1
                sz = struct.unpack("<HH", m2[2:6]); stats["sizes"].add(sz)
                if sz != cur:
                    cur = sz; seg += 1
                    if vid: vid.close()
                    vid = open(os.path.join(a.out, f"video{seg}.h264"), "wb"); open(os.path.join(a.out, "lastseg"), "w").write(str(seg)); nseg = 0
                vid.write(m2[6:]); nseg += 1; open(os.path.join(a.out, "lastn"), "w").write(str(nseg - 1)); frames.append((round(time.time()-t0,3), m2[1] & 1, len(m2)))
                first.setdefault("video", time.time() - t0)
            elif m2[0] == 2:
                stats["a"] += 1; stats["ab"] += len(m2) - 1; pcm.write(m2[1:])
        await task
    json.dump(frames, open(os.path.join(a.out, 'frames.json'), 'w'))
    secs = a.secs
    r = sorted(stats["rtt"])
    print(json.dumps(dict(video_frames=stats["v"], keyframes=stats["key"], sizes=sorted(stats["sizes"]), video_kbps=round(stats["vb"]*8/1000/secs, 1),
        audio_kbps=round(stats["ab"]*8/1000/secs, 1), first_video_s=round(first.get("video", -1), 2),
        rtt_ms_median=round(r[len(r)//2], 1) if r else None, rtt_ms_p95=round(r[int(len(r)*.95)], 1) if r else None)))
asyncio.run(main())
