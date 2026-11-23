// Registro del service worker, instalación y estado sin conexión (WEB-6).
// Eventos sobre `window`: p3-pwa-progress {got,total}, p3-pwa-ready, p3-pwa-update.
const emit = (n, d) => window.dispatchEvent(new CustomEvent(n, { detail: d }));

if (!self.crossOriginIsolated) {
  console.warn("[p3] sin aislamiento de origen cruzado: SharedArrayBuffer no estará disponible. " +
    "Sirve la página con web/serve.py (COOP/COEP) en localhost.");
}

if ("serviceWorker" in navigator && (location.protocol === "https:" || location.hostname === "localhost" || location.hostname === "127.0.0.1")) {
  navigator.serviceWorker.addEventListener("message", (e) => {
    if (e.data?.type === "p3-progress") emit("p3-pwa-progress", e.data);
    if (e.data?.type === "p3-ready") emit("p3-pwa-ready", e.data);
  });
  navigator.serviceWorker.register("sw.js", { updateViaCache: "none" }).then((reg) => {
    const waiting = () => reg.waiting && navigator.serviceWorker.controller && emit("p3-pwa-update", { apply: () => reg.waiting.postMessage("p3-skip-waiting") });
    waiting();
    reg.addEventListener("updatefound", () => reg.installing?.addEventListener("statechange", waiting));
    // Si el SW pedido por primera vez no controla aún la página, recargar una vez para
    // que lo haga (hace falta para que el Worker herede la caché y las cabeceras).
    if (!navigator.serviceWorker.controller) {
      navigator.serviceWorker.addEventListener("controllerchange", () => {
        if (!sessionStorage.getItem("p3-reloaded")) { sessionStorage.setItem("p3-reloaded", "1"); location.reload(); }
      }, { once: true });
    }
  }).catch((e) => console.warn("[p3] service worker:", e));
}

// Los datos de OPFS (EXE, CPR, partidas) no se borran con la caché, pero sí si el
// navegador libera espacio: pedir almacenamiento persistente.
navigator.storage?.persist?.().then((ok) => emit("p3-pwa-persist", { ok })).catch(() => {});

let deferred;
window.addEventListener("beforeinstallprompt", (e) => { e.preventDefault(); deferred = e; emit("p3-pwa-installable", { install: () => deferred.prompt() }); });
