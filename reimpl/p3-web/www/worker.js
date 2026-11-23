// Worker del juego (WEB-3/WEB-4): carga p3_web.wasm, le abre los archivos del jugador de OPFS (EXE, CPR, música: lectura por tramos,
// el CPR no se copia a la memoria de WASM) y no vuelve del bucle del juego. Las partidas (`save/`) viven en memoria dentro del
// wasm (`p3-web-save`) y cada archivo terminado se manda a la página, que lo guarda en OPFS.
import { pop, waitFrame } from './ring.js';
import { WorkerSaves } from './p3-saves-worker.js';

const GAME_DIR = 'game';
const ROOT = '/juego'; // carpeta del juego tal como la ve el enlazador
let img = null, canvas, ctx, ring, memory, exports, seenFrame = 0, started = false, presented = 0, ready = null;
const handles = []; // id -> FileSystemSyncAccessHandle (solo lectura en la práctica)

const say = (type, data = {}) => postMessage({ type, ...data });
const saves = new WorkerSaves(self);

async function gameDir(create) {
  const root = await navigator.storage.getDirectory();
  return root.getDirectoryHandle(GAME_DIR, { create });
}

// Manejador de `rel` (con `/`) dentro de `game/`, creando las carpetas intermedias.
async function handleAt(rel, create) {
  const parts = rel.split('/').filter(Boolean), name = parts.pop();
  let dir = await gameDir(create);
  for (const p of parts) dir = await dir.getDirectoryHandle(p, { create });
  return dir.getFileHandle(name, { create });
}

// Copia un File del jugador a OPFS por tramos de 8 MB con acceso síncrono. `name` puede llevar subcarpetas (`Musik/a.mp3`).
async function importFile(name, file) {
  const fh = await handleAt(name, true);
  const h = await fh.createSyncAccessHandle();
  try {
    h.truncate(0);
    const CH = 8 << 20;
    for (let at = 0; at < file.size; at += CH) {
      const buf = new Uint8Array(await file.slice(at, at + CH).arrayBuffer());
      h.write(buf, { at });
      say('progress', { name, done: Math.min(at + CH, file.size), total: file.size });
    }
    h.flush();
  } finally { h.close(); }
}

// Descarga un archivo del propio servidor (/_game/NOMBRE) a OPFS por tramos; para el móvil, que no tiene selector de carpetas.
async function importUrl(name, url) {
  const resp = await fetch(url);
  if (!resp.ok) throw new Error(`${name}: HTTP ${resp.status}`);
  const total = Number(resp.headers.get('content-length')) || 0;
  const fh = await handleAt(name, true);
  const h = await fh.createSyncAccessHandle();
  try {
    h.truncate(0);
    let at = 0;
    const rd = resp.body.getReader();
    for (;;) {
      const { done, value } = await rd.read();
      if (done) break;
      h.write(value, { at }); at += value.length;
      say('progress', { name, done: at, total: total || at });
    }
    h.flush();
  } finally { h.close(); }
}

async function listGame() {
  try {
    const dir = await gameDir(false), out = {};
    for await (const [n, h] of dir.entries()) if (h.kind === 'file') out[n] = (await h.getFile()).size;
    return out;
  } catch { return {}; }
}

// Tras recargar la página el Worker anterior puede tardar un momento en soltar sus accesos síncronos (son exclusivos): se reintenta.
async function syncHandle(h) {
  for (let i = 0; ; i++) {
    try { return await h.createSyncAccessHandle(); } catch (e) {
      if (e.name !== 'NoModificationAllowedError' || i >= 20) throw e;
      await new Promise((r) => setTimeout(r, 250));
    }
  }
}

// Abre todos los archivos de `game/` (y subcarpetas, p. ej. Musik/) y se los registra al wasm con su ruta relativa.
async function openGameFiles() {
  for (const h of handles.splice(0)) { try { h.close(); } catch { /* ya cerrado */ } }
  const enc = new TextEncoder();
  const put = (s) => { const b = enc.encode(s); const p = exports.p3_alloc(b.length); new Uint8Array(memory.buffer, p, b.length).set(b); return [p, b.length]; };
  const walk = async (dir, prefix, depth) => {
    for await (const [name, h] of dir.entries()) {
      if (h.kind === 'directory') { if (depth < 3) await walk(h, `${prefix}${name}/`, depth + 1); continue; }
      const access = await syncHandle(h);
      const id = handles.push(access) - 1;
      const [pp, pl] = put(prefix + name);
      exports.p3_fs_add(id, pp, pl, access.getSize());
      exports.p3_free(pp, Math.max(pl, 1));
    }
  };
  await walk(await gameDir(false), '', 0);
  return handles.length;
}

let audioSab = null; // anillo de audio compartido con el worklet (mismo formato que p3-audio::ring)
const A = { MAGIC: 0, CAP: 1, WRITE: 2, READ: 3, RATE: 4, UNDERRUNS: 5, FLAGS: 6, HEADER: 8 };
// Sincroniza el anillo de audio de la memoria de WASM (no compartida) con el SharedArrayBuffer que vacía el AudioWorklet:
// trae la posición de lectura/vaciados del worklet y copia al SAB los fotogramas nuevos. Barato: unos miles de muestras.
function audioSync(ptr) {
  if (!audioSab) return;
  const sh = new Int32Array(audioSab, 0, A.HEADER), wh = new Int32Array(memory.buffer, ptr, A.HEADER);
  wh[A.READ] = Atomics.load(sh, A.READ);
  wh[A.UNDERRUNS] = Atomics.load(sh, A.UNDERRUNS);
  wh[A.FLAGS] = Atomics.load(sh, A.FLAGS);
  const cap = wh[A.CAP] >>> 0, mask = cap - 1;
  const from = Atomics.load(sh, A.WRITE) >>> 0, to = wh[A.WRITE] >>> 0;
  const n = (to - from) >>> 0;
  if (!n) return;
  const src = new Float32Array(memory.buffer, ptr + A.HEADER * 4, cap * 2), dst = new Float32Array(audioSab, A.HEADER * 4, cap * 2);
  const at = (from & mask) * 2, first = Math.min(n, cap - (from & mask));
  dst.set(src.subarray(at, at + first * 2), at);
  if (first < n) dst.set(src.subarray(0, (n - first) * 2), 0);
  Atomics.store(sh, A.WRITE, to | 0);
}

const decoder = new TextDecoder();
const imports = {
  p3: {
    p3_present(ptr, w, h) {
      if (++presented === 1) say('log', { level: 0, text: `primer fotograma ${w}x${h}` });
      if (presented % 300 === 0) say('log', { level: 0, text: `fotograma ${presented}: memoria wasm ${(memory.buffer.byteLength / 1048576) | 0} MB, heap JS ${((performance.memory && performance.memory.usedJSHeapSize) / 1048576) | 0} MB` });
      // Se copia el fotograma y se transfiere (sin OffscreenCanvas: el de un Worker que nunca cede no publicaba, y con
      // transferToImageBitmap el navegador acumulaba gigas de bitmaps); la página lo pinta con putImageData.
      const rgba = new Uint8Array(memory.buffer, ptr, w * h * 4).slice();
      postMessage({ type: 'frame', w, h, rgba: rgba.buffer }, [rgba.buffer]);
    },
    p3_input_pop(ptr) { return pop(ring, new Uint32Array(memory.buffer, ptr, 4)); },
    p3_frame_wait() { seenFrame = waitFrame(ring, seenFrame); },
    p3_now_ms() { return performance.now(); },
    p3_log(level, ptr, len) {
      const t = decoder.decode(new Uint8Array(memory.buffer, ptr, len));
      say('log', { level, text: t });
    },
    p3_finished(code) { say('finished', { code }); },
    p3_audio_attach(ptr, bytes) {
      audioSab = new SharedArrayBuffer(bytes);
      new Uint8Array(audioSab, 0, A.HEADER * 4).set(new Uint8Array(memory.buffer, ptr, A.HEADER * 4)); // cabecera: magia, capacidad, frecuencia
      say('audio', { buffer: audioSab });
    },
    p3_audio_sync(ptr) { audioSync(ptr); },
    // Lectura posicional de un archivo abierto (el CPR entra por tramos, nunca entero).
    p3_fs_read(id, offset, ptr, len) {
      try { return handles[id].read(new Uint8Array(memory.buffer, ptr, len), { at: offset }); } catch (e) { say('log', { level: 1, text: `lectura OPFS ${id}@${offset}: ${e}` }); return -1; }
    },
  },
  env: saves.imports(),
};

async function loadModule(wasmUrl) {
  // Con la carcasa de WEB-6 (build_web.py) existe `p3-load.js` + `build.json`: compila en streaming desde la caché del service worker.
  try { const { loadP3Module } = await import('./p3-load.js'); return await loadP3Module(); } catch { /* desarrollo: p3_web.wasm suelto */ }
  return WebAssembly.compileStreaming(fetch(wasmUrl));
}

// Instancia el motor y deja listos el sistema de archivos y las partidas; así las importaciones de partidas se validan ya en el menú
// previo. Los archivos del juego se abren justo antes de jugar (un acceso síncrono de OPFS es exclusivo, y aquí también se importan).
async function prepare(wasmUrl) {
  say('status', { text: 'Cargando el motor…' });
  const instance = await WebAssembly.instantiate(await loadModule(wasmUrl), imports);
  exports = instance.exports; memory = exports.memory;
  saves.bind(exports);
  const enc = new TextEncoder().encode(ROOT);
  const p = exports.p3_alloc(enc.length); new Uint8Array(memory.buffer, p, enc.length).set(enc);
  exports.p3_fs_begin(p, enc.length);
  exports.p3_free(p, enc.length);
  await saves.install(ROOT); // espera la precarga de save/ que manda la página
  say('ready');
}

async function start(opts) {
  started = true;
  const files = await listGame();
  const names = Object.keys(files);
  if (!names.some(n => n.toLowerCase() === 'patrician3.exe') || !names.some(n => n.toLowerCase().endsWith('.cpr'))) {
    started = false;
    return say('error', { text: 'Faltan Patrician3.exe o el archivo .cpr: elige la carpeta del juego.' });
  }
  await ready;
  say('status', { text: 'Abriendo los archivos del juego…' });
  await openGameFiles();
  say('running');
  const { modern = 0, w = 0, h = 0, monitorW = 1920, monitorH = 1080, audioRate = 0 } = opts;
  // Poca memoria en móviles (o con `lowMemory: true/false` en las opciones): ver `Runtime::set_low_memory`.
  if (opts.lowMemory ?? /iPhone|iPad|iPod|Android/.test(navigator.userAgent)) exports.p3_set_low_memory?.(1);
  exports.p3_start(modern, w, h, monitorW, monitorH, audioRate | 0); // no vuelve hasta cerrar el juego
}

onmessage = async (e) => {
  const m = e.data;
  try {
    if (m.type === 'init') {
      ring = m.ring;
      ready = prepare(m.wasmUrl);
      say('files', { files: await listGame() });
      await ready;
    } else if (m.type === 'import-url') { try { for (const [n, u] of m.files) await importUrl(n, u); } catch (err) { say('error', { text: String(err) }); } say('files', { files: await listGame() }); }
    else if (m.type === 'import') { for (const [n, f] of m.files) await importFile(n, f); say('files', { files: await listGame() }); }
    else if (m.type === 'start' && !started) await start(m);
  } catch (err) { say('error', { text: String(err && err.stack || err) }); }
};
