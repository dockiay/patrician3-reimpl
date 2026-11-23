// Carga del .wasm: compilación en streaming desde la caché del service worker (o red).
// Sirve igual en la página que en el Worker del juego:
//   import { loadP3Module } from "./p3-load.js";
//   const module = await loadP3Module(msg => ...);   // WebAssembly.Module
//   WebAssembly.instantiate(module, imports)
// Con `compileStreaming` Chrome guarda además el código compilado (code cache) junto
// a la respuesta cacheada, y las siguientes aperturas no vuelven a compilar los 30+ MB.
export async function loadP3Module(onProgress) {
  const base = new URL("./", import.meta.url);
  const build = await (await fetch(new URL("build.json", base), { cache: "no-cache" }).catch(() =>
    caches.match(new URL("build.json", base)))).json();
  const url = new URL(build.wasm, base);
  let res = await caches.match(url);
  if (!res) {
    res = await fetch(url);
    if (!res.ok) throw new Error("no se pudo cargar " + build.wasm + ": " + res.status);
  }
  const total = build.sizes.opt;
  if (onProgress && res.body) {
    let got = 0;
    const tap = new TransformStream({ transform(c, ctl) { got += c.length; onProgress(got, total); ctl.enqueue(c); } });
    res = new Response(res.body.pipeThrough(tap), { headers: { "Content-Type": "application/wasm" } });
  } else if (!res.headers.get("Content-Type")?.startsWith("application/wasm")) {
    res = new Response(res.body, { headers: { "Content-Type": "application/wasm" } });
  }
  if (WebAssembly.compileStreaming) return WebAssembly.compileStreaming(res);
  return WebAssembly.compile(await res.arrayBuffer());
}
export async function p3Version() {
  const base = new URL("./", import.meta.url);
  return (await (await fetch(new URL("build.json", base))).json()).version;
}
