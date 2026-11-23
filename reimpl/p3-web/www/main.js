import { newRing, push, tickFrame } from './ring.js';
import { vkOf } from './vk.js';
import { SaveStore } from './p3-saves-main.js';
import { createAudioContext, attachRing, keepAudioAlive } from './p3-audio-web.js';

const $ = (id) => document.getElementById(id);
const canvas = $('screen'), status = $('status'), logEl = $('log');
const EV = { MOVE: 1, BUTTON: 2, WHEEL: 3, KEY: 4, FOCUS: 5, QUIT: 6 };
const ring = newRing();
const worker = new Worker('worker.js', { type: 'module' });
let running = false, engineReady = false, haveFiles = false;

const say = (t) => { status.textContent = t; };
const log = (t, err) => { const d = document.createElement('div'); d.textContent = t; if (err) d.className = 'err'; logEl.append(d); };

if (!crossOriginIsolated) say('Falta el aislamiento de origen cruzado (COOP/COEP): usa serve.py o un alojamiento con esas cabeceras.');

const screen = canvas.getContext('2d', { alpha: false });
let latest = null;
const store = new SaveStore();
store.attach(worker); // las partidas que el juego guarda llegan por mensajes y se escriben en OPFS
worker.postMessage({ type: 'init', ring, wasmUrl: new URL('p3_web.wasm', location.href).href });
store.persist().then((ok) => { $('persist').textContent = ok ? 'Almacenamiento persistente concedido.' : 'El navegador puede borrar las partidas si falta espacio: exporta copias.'; });
store.sendInitialSaves().then(() => refreshSaves());

worker.onmessage = (e) => {
  const m = e.data;
  if (m.type === 'frame') latest = m;
  else if (m.type === 'files') {
    const names = Object.keys(m.files);
    const ok = names.some(n => n.toLowerCase() === 'patrician3.exe') && names.some(n => n.toLowerCase().endsWith('.cpr'));
    $('files').textContent = names.length ? names.map(n => `${n} (${(m.files[n] / 1048576).toFixed(1)} MB)`).join(', ') : 'ninguno';
    haveFiles = ok; updatePlay();
  } else if (m.type === 'progress') say(`Copiando ${m.name}: ${Math.round(100 * m.done / m.total)} %`);
  else if (m.type === 'status') say(m.text);
  else if (m.type === 'ready') { engineReady = true; say(''); updatePlay(); }
  else if (m.type === 'running') { running = true; $('savesbox').open = false; document.body.classList.add('running'); say(''); canvas.focus(); refreshSaves(); }
  else if (m.type === 'audio') attachAudio(m.buffer);
  else if (m.type === 'log') log(m.text, m.level);
  else if (m.type === 'error') { log(m.text, 1); say(m.text); }
  else if (m.type === 'finished') { running = false; document.body.classList.remove('running'); say(`El juego terminó (${m.code}). Recarga la página para volver a jugar.`); refreshSaves(); }
};

// --- carpeta del juego -> OPFS (solo se copian el EXE y el .cpr; nada sale del navegador) ---
// La música suelta (`Musik/**`) también se copia: sin ella el juego no suena. `rel` lleva `/`.
// Idiomas: todos los `p2arch0_<código>.cpr` de la carpeta (inglés + español = textos en español) e `idioma/*.pack` (menús traducidos).
const isMusik = (rel) => /^musik\//i.test(rel);
// Los vídeos de las ventanas de suceso (`Videos/*.sks`) van con la música: sin ellos sale la imagen fija.
const isVideo = (rel) => /^videos\/[^/]+\.sks$/i.test(rel);
const isLangCpr = (n) => /^p2arch0_[a-z0-9]{1,8}\.cpr$/.test(n);
function addFile(out, rel, file) {
  const parts = rel.split('/'), name = parts[parts.length - 1], n = name.toLowerCase();
  if ((isMusik(rel) && parts.length > 1) || isVideo(rel)) (out.music ||= []).push([rel, file]);
  else if (parts.length === 2 && /^idioma$/i.test(parts[0]) && n.endsWith('.pack')) (out.extra ||= []).push([`idioma/${name}`, file]);
  else if (/^idioma$/i.test(parts[parts.length - 2] || '')) return; // p. ej. el CPR derivado de p3-juego (idioma/p2arch0_spa.cpr)
  else if (n === 'patrician3.exe' && !out.exe) out.exe = [name, file];
  else if (isLangCpr(n)) { if (!(out.cprs ||= []).some(([m]) => m.toLowerCase() === n)) out.cprs.push([name, file]); }
  else if (n.endsWith('.cpr') && !out.cpr) out.cpr = [name, file];
}
async function walk(dir, depth, out, prefix = '') {
  for await (const [name, h] of dir.entries()) {
    if (h.kind === 'file') addFile(out, prefix + name, await h.getFile());
    else if (depth < 3 && (!prefix || isMusik(prefix))) await walk(h, depth + 1, out, `${prefix}${name}/`);
    else if (depth < 3) await walk(h, depth + 1, out, '');
  }
}
function sendImport(out) {
  const cprs = out.cprs?.length ? out.cprs : (out.cpr ? [out.cpr] : []);
  if (!out.exe || !cprs.length) return say('No encuentro Patrician3.exe y el archivo .cpr dentro de esa carpeta.');
  if (!out.music?.length) log('No hay carpeta Musik: el juego no tendrá música.');
  if (cprs.length > 1) log(`Idiomas: ${cprs.map(([n]) => n).join(', ')}${out.extra?.length ? '' : ' (sin idioma/*.pack: menús en inglés)'}`);
  worker.postMessage({ type: 'import', files: [out.exe, ...cprs, ...(out.extra || []), ...(out.music || [])] });
}
$('pick').onclick = async () => {
  if (navigator.storage?.persist) navigator.storage.persist().catch(() => {});
  if (window.showDirectoryPicker) {
    try { const out = {}; await walk(await showDirectoryPicker({ mode: 'read' }), 0, out); sendImport(out); } catch (e) { if (e.name !== 'AbortError') say(String(e)); }
  } else $('pickfallback').click();
};
$('pickfallback').onchange = (e) => {
  const out = {};
  for (const f of e.target.files) {
    const rel = (f.webkitRelativePath || f.name).split('/');
    const at = rel.findIndex((p) => p.toLowerCase() === 'musik' || p.toLowerCase() === 'videos');
    if (at >= 0 && at < rel.length - 1) addFile(out, rel.slice(at).join('/'), f);
    else if (rel.length >= 2 && rel[rel.length - 2].toLowerCase() === 'idioma') addFile(out, `idioma/${f.name}`, f);
    else addFile(out, f.name, f);
  }
  sendImport(out);
};
// Desde el servidor del PC (serve.py con P3_GAME_DIR): sirve en el móvil, que no puede elegir carpetas.
$('fromserver').onclick = async () => {
  try {
    const list = await (await fetch('_game/')).json();
    if (!list.exe || !list.cpr) return say('El servidor no tiene Patrician3.exe y el .cpr (¿falta P3_GAME_DIR?).');
    if (navigator.storage?.persist) navigator.storage.persist().catch(() => {});
    const url = (n) => new URL('_game/' + n.split('/').map(encodeURIComponent).join('/'), location.href).href;
    const cprs = list.cprs?.length ? list.cprs : [list.cpr];
    worker.postMessage({ type: 'import-url', files: [list.exe, ...cprs, ...(list.extra || []), ...(list.music || [])].map(n => [n, url(n)]) });
  } catch (e) { say('No pude leer los archivos del servidor: ' + e); }
};
function updatePlay() { $('play').disabled = !(haveFiles && engineReady) || running; }

// --- resolución: 800×600 original o moderna (el juego la cambia solo dentro de la partida; en los menús sigue a 800×600) ---
function monitorSize() {
  const d = window.devicePixelRatio || 1;
  return [Math.round(screen.width * d), Math.round(screen.height * d)];
}
function startOptions() {
  const v = $('res').value, [monitorW, monitorH] = monitorSize();
  if (v === 'saved') return { modern: 0, monitorW, monitorH };
  if (v === 'auto') return { modern: 1, monitorW, monitorH };
  const [w, h] = v.split('x').map(Number);
  return { modern: 2, w, h, monitorW, monitorH };
}
// Sonido: el AudioContext nace dentro del clic en «Jugar» (política de autoplay); el Worker crea el anillo y la página conecta el worklet.
let audioCtx = null, audioView = null, analyser = null, anBuf = null;
async function attachAudio(buffer) {
  try {
    const node = await attachRing(audioCtx, { buffer }, 0, new URL('p3-audio-worklet.js', location.href));
    analyser = audioCtx.createAnalyser(); analyser.fftSize = 2048; anBuf = new Float32Array(2048); node.connect(analyser); // solo medida
    audioView = new Int32Array(buffer, 0, 8);
    keepAudioAlive(audioCtx);
  } catch (e) { log(`sin sonido: ${e}`, 1); }
}
// Medidas para pruebas: vaciados del anillo (el worklet no encontró datos), fotogramas por delante, estado del contexto y RMS de la salida.
window.p3AudioStats = () => audioView && { underruns: Atomics.load(audioView, 5), buffered: (Atomics.load(audioView, 2) - Atomics.load(audioView, 3)) >>> 0, state: audioCtx.state, rate: audioCtx.sampleRate, rms: (analyser.getFloatTimeDomainData(anBuf), Math.sqrt(anBuf.reduce((a, x) => a + x * x, 0) / anBuf.length)) };
$('play').onclick = async () => {
  $('play').disabled = true;
  try { audioCtx = await createAudioContext(); } catch (e) { log(`sin sonido: ${e}`, 1); }
  worker.postMessage({ type: 'start', ...startOptions(), audioRate: audioCtx ? audioCtx.sampleRate : 0 });
};

// --- partidas: lista, importar, exportar, borrar. Importar y borrar solo con el juego parado (el Worker no atiende mensajes mientras
// juega); exportar vale siempre porque lo que el juego guarda ya está en OPFS. ---
const fmtDate = (t) => new Date(t).toLocaleString();
async function refreshSaves() {
  const ul = $('saves'), list = await store.list();
  ul.textContent = '';
  if (!list.length) { ul.textContent = 'Ninguna partida guardada todavía.'; return; }
  for (const s of list) {
    const li = document.createElement('li');
    li.append(`${s.name} (${(s.size / 1024).toFixed(0)} KB, ${fmtDate(s.modified)}) `);
    const dl = document.createElement('button'); dl.textContent = 'Exportar'; dl.onclick = () => store.download(s.rel);
    const del = document.createElement('button'); del.textContent = 'Borrar'; del.disabled = running;
    del.onclick = async () => { if (confirm(`¿Borrar ${s.name}?`)) { await store.delete(s.rel); refreshSaves(); } };
    li.append(dl, del); ul.append(li);
  }
}
$('import').onclick = () => $('importfile').click();
$('importfile').onchange = async (e) => {
  const reasons = { extension: 'no es .pat/.pst', size: 'tamaño no válido', exists: 'ya existe (bórrala antes)', invalid: 'el juego no la entiende' };
  for (const f of e.target.files) {
    const r = await store.importFile(f);
    log(r.ok ? `Importada ${f.name}` : `No se importó ${f.name}: ${reasons[r.reason] || r.reason}`, !r.ok);
  }
  e.target.value = ''; refreshSaves();
};
$('import').disabled = false;
setInterval(() => { if (running) refreshSaves(); }, 5000);

// --- entrada: eventos DOM -> anillo ---
const frac = (v, size) => Math.max(0, Math.min(65535, Math.floor(v / size * 65536)));
// Rectángulo que ocupa la imagen dentro del lienzo: en pantalla completa sobra margen (object-fit: contain) y no cuenta.
function contentRect() {
  const r = canvas.getBoundingClientRect(), ar = canvas.width / canvas.height;
  if (r.width / r.height > ar) { const w = r.height * ar; return { left: r.left + (r.width - w) / 2, top: r.top, width: w, height: r.height }; }
  const h = r.width / ar;
  return { left: r.left, top: r.top + (r.height - h) / 2, width: r.width, height: h };
}
const pos = (e) => { const r = contentRect(); push(ring, EV.MOVE, frac(e.clientX - r.left, r.width), frac(e.clientY - r.top, r.height)); };
canvas.addEventListener('pointermove', (e) => { pos(e); });
// Táctil (iPhone/iPad): un toque es clic izquierdo; con «Toque = botón derecho» marcado, clic derecho.
const btn = (e) => (e.pointerType === 'touch' && $('rc').checked) ? 2 : e.button;
canvas.addEventListener('pointerdown', (e) => { canvas.setPointerCapture(e.pointerId); pos(e); push(ring, EV.BUTTON, btn(e), 1); e.preventDefault(); });
canvas.addEventListener('pointerup', (e) => { pos(e); push(ring, EV.BUTTON, btn(e), 0); e.preventDefault(); });
if (navigator.maxTouchPoints > 0) $('rcwrap').hidden = false;
canvas.addEventListener('wheel', (e) => { push(ring, EV.WHEEL, (-Math.sign(e.deltaY) * 120) | 0); e.preventDefault(); }, { passive: false });
canvas.addEventListener('contextmenu', (e) => e.preventDefault());
const key = (down) => (e) => {
  if (!running || e.metaKey) return;
  const vk = vkOf(e.code);
  if (!vk) return;
  push(ring, EV.KEY, vk, down ? 1 : 0);
  e.preventDefault(); // F5, Tab, Alt, teclas de flecha… no deben actuar sobre el navegador mientras se juega
};
window.addEventListener('keydown', key(true));
window.addEventListener('keyup', key(false));
window.addEventListener('blur', () => push(ring, EV.FOCUS, 0));
window.addEventListener('focus', () => push(ring, EV.FOCUS, 1));
window.addEventListener('beforeunload', () => push(ring, EV.QUIT));

// Un tic por fotograma de la página: el Worker espera a él en cada presentación (40 ms como máximo).
(function tick() {
  if (latest) {
    const f = latest; latest = null;
    if (canvas.width !== f.w || canvas.height !== f.h) { canvas.width = f.w; canvas.height = f.h; canvas.style.setProperty('--ar', `${f.w} / ${f.h}`); canvas.style.setProperty('--arn', String(f.w / f.h)); }
    screen.putImageData(new ImageData(new Uint8ClampedArray(f.rgba), f.w, f.h), 0, 0);
  }
  tickFrame(ring); requestAnimationFrame(tick); })();
const toggleFull = () => { if (document.fullscreenElement) document.exitFullscreen(); else (canvas.requestFullscreen || canvas.webkitRequestFullscreen)?.call(canvas); };
$('full').onclick = toggleFull;
// Alt+Intro alterna la pantalla completa, como en el cliente de escritorio (el juego no recibe esa pulsación).
window.addEventListener('keydown', (e) => { if (e.altKey && e.code === 'Enter') { toggleFull(); e.preventDefault(); e.stopImmediatePropagation(); } }, true);
if (!canvas.requestFullscreen && !canvas.webkitRequestFullscreen) $('full').hidden = true; // iPhone: usa «Añadir a pantalla de inicio»
