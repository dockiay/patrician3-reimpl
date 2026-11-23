// Worker de prueba: instancia el .wasm real de p3-web-save y ejecuta órdenes como lo haría el juego.
import { WorkerSaves } from '../p3-saves-worker.js';

const side = new WorkerSaves(self);
const bytes = await (await fetch('/p3_web_save.wasm')).arrayBuffer();
const { instance } = await WebAssembly.instantiate(bytes, { env: side.imports() });
side.bind(instance.exports);
const n = await side.install('/juego');
const x = instance.exports;
self.postMessage({ type: 't-ready', preloaded: n });
self.addEventListener('message', (ev) => {
  const m = ev.data;
  if (m.type === 't-write') {
    const [pp, pl] = side.str(m.path);
    const dp = side.put(m.data);
    self.postMessage({ type: 't-wrote', ok: x.p3_save_hostfs_write(pp, pl, dp, m.data.length) });
  } else if (m.type === 't-len') {
    const [pp, pl] = side.str(m.path);
    self.postMessage({ type: 't-len', len: x.p3_save_hostfs_len(pp, pl) });
  }
});
