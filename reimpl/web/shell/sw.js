// Service worker de WEB-6: instala todo (incluido el .wasm) en la caché para jugar sin
// conexión. __VERSION__ lo pone build_web.py (hash del contenido): cada compilación
// distinta crea caché nueva; la anterior se borra al activar la nueva.
const VERSION = "__VERSION__";
const CACHE = "p3-web-" + VERSION;

// SharedArrayBuffer exige aislamiento de origen cruzado: se reponen las cabeceras
// por si la respuesta sale de la caché o de un servidor que no las envía.
function isolate(res) {
  if (res.status === 0 || res.type === "opaque") return res;
  const h = new Headers(res.headers);
  h.set("Cross-Origin-Opener-Policy", "same-origin");
  h.set("Cross-Origin-Embedder-Policy", "require-corp");
  h.set("Cross-Origin-Resource-Policy", "same-origin");
  return new Response(res.body, { status: res.status, statusText: res.statusText, headers: h });
}

async function say(msg) {
  for (const c of await self.clients.matchAll({ includeUncontrolled: true })) c.postMessage(msg);
}

self.addEventListener("install", (ev) => {
  ev.waitUntil((async () => {
    const build = await (await fetch("build.json", { cache: "no-store" })).json();
    const cache = await caches.open(CACHE);
    const urls = [...new Set(["./", ...build.files, "build.json"])];
    let done = 0;
    // El .wasm se pide aparte y en último lugar para poder mostrar su progreso.
    const small = urls.filter((u) => u !== build.wasm);
    for (const u of small) { await cache.add(new Request(u, { cache: "reload" })); done++; }
    const res = await fetch(build.wasm, { cache: "reload" });
    if (!res.ok) throw new Error("wasm " + res.status);
    const total = +res.headers.get("x-p3-size") || build.sizes.opt;
    const [a, b] = res.body.tee();
    const put = cache.put(build.wasm, new Response(b, { headers: { "Content-Type": "application/wasm" } }));
    const rd = a.getReader(); let got = 0, last = 0;
    for (;;) {
      const { done: d, value } = await rd.read();
      if (d) break;
      got += value.length;
      if (got - last > (1 << 20)) { last = got; say({ type: "p3-progress", got, total }); }
    }
    await put;
    say({ type: "p3-ready", version: VERSION });
  })());
});

self.addEventListener("activate", (ev) => {
  ev.waitUntil((async () => {
    for (const k of await caches.keys()) if (k.startsWith("p3-web-") && k !== CACHE) await caches.delete(k);
    await self.clients.claim();
  })());
});

self.addEventListener("message", (ev) => {
  if (ev.data === "p3-skip-waiting") self.skipWaiting();
});

self.addEventListener("fetch", (ev) => {
  const req = ev.request;
  if (req.method !== "GET" || new URL(req.url).origin !== location.origin) return;
  ev.respondWith((async () => {
    const cache = await caches.open(CACHE);
    // Navegación: siempre el index de la caché (sirve sin red y con ?query).
    const hit = await cache.match(req, { ignoreSearch: true }) ||
      (req.mode === "navigate" ? await cache.match("./") : undefined);
    if (hit) return isolate(hit);
    try { return isolate(await fetch(req)); }
    catch (e) { return new Response("sin conexión", { status: 503 }); }
  })());
});
