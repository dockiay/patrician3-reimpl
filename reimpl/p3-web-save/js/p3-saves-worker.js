// WEB-4: lado del Worker del juego. Une el módulo `.wasm` (con `p3-web-save`) con `SaveStore` del hilo principal.
//
// Uso en el Worker, antes de arrancar el juego:
//   const side = new WorkerSaves(self);
//   const { instance } = await WebAssembly.instantiate(bytes, { env: { ...otras, ...side.imports() } });
//   side.bind(instance.exports);            // `memory` y las funciones p3_save_*
//   await side.install('/juego');           // espera la precarga del hilo principal y llama a p3_save_install
// Tras `install`, todo `HostFs` bajo `<juego>/save` es síncrono y en memoria. Los mensajes `p3-save-import` y `p3-save-forget`
// se atienden cuando el Worker cede el control al bucle de eventos (en cada fotograma, no mientras está bloqueado).

const enc = new TextEncoder();
const dec = new TextDecoder();

export class WorkerSaves {
  constructor(port) {
    this.port = port;
    this.x = null;
    this.ready = new Promise((r) => { this.resolveReady = r; });
    port.addEventListener('message', (ev) => this.onMessage(ev.data));
    if (port.start) port.start();
  }

  /** Funciones que el `.wasm` importa (módulo `env`). */
  imports() {
    return {
      p3_save_commit: (pp, pl, dp, dl, kind) => {
        const mem = new Uint8Array(this.x.memory.buffer);
        const rel = dec.decode(mem.subarray(pp, pp + pl));
        // `slice` copia fuera de la memoria de wasm; se transfiere el buffer sin volver a copiarlo.
        const data = kind === 1 ? mem.slice(dp, dp + dl) : null;
        this.port.postMessage({ type: 'p3-save-commit', rel, data }, data ? [data.buffer] : []);
      },
      p3_save_now_secs: () => Math.floor(Date.now() / 1000),
    };
  }

  bind(exports) { this.x = exports; }

  put(bytes) {
    const p = this.x.p3_save_alloc(bytes.length);
    new Uint8Array(this.x.memory.buffer, p, bytes.length).set(bytes);
    return p;
  }

  str(s) {
    const b = enc.encode(s);
    return [this.put(b), b.length];
  }

  preload(rel, data, modifiedSecs, validate) {
    const [pp, pl] = this.str(rel);
    const dp = this.put(data);
    const ok = this.x.p3_save_preload(pp, pl, dp, data.length, modifiedSecs, validate ? 1 : 0);
    this.x.p3_save_free(pp, Math.max(pl, 1));
    this.x.p3_save_free(dp, Math.max(data.length, 1));
    return ok === 1;
  }

  async install(root) {
    const init = await this.ready;
    const [rp, rl] = this.str(root);
    this.x.p3_save_install(rp, rl);
    this.x.p3_save_free(rp, Math.max(rl, 1));
    for (const f of init.files) this.preload(f.rel, f.data, f.modified, false);
    return init.files.length;
  }

  onMessage(m) {
    if (!m || typeof m !== 'object') return;
    if (m.type === 'p3-save-init') this.resolveReady(m);
    else if (m.type === 'p3-save-import') {
      // Valida con el cargador de `p3-save` y, si lo acepta, queda en memoria: el menú la ve en la lista de partidas.
      const ok = this.preload(m.rel, m.data, Math.floor(Date.now() / 1000), true);
      this.port.postMessage({ type: 'p3-save-import-result', id: m.id, ok });
    } else if (m.type === 'p3-save-forget') {
      const [pp, pl] = this.str(m.rel);
      this.x.p3_save_forget(pp, pl);
      this.x.p3_save_free(pp, Math.max(pl, 1));
    }
  }
}
