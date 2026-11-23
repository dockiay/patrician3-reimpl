// Prueba con node: el consumidor JS (el mismo bucle que el worklet) lee un anillo escrito con la disposición de ring.rs.
import { ringViews, ringRead, MAGIC, VERSION, W, HEADER_WORDS } from './p3-audio-ring.js';
import assert from 'node:assert/strict';
const cap = 256;
const buf = new SharedArrayBuffer((HEADER_WORDS + cap * 2) * 4);
const head = new Int32Array(buf, 0, HEADER_WORDS);
head[W.MAGIC] = MAGIC ^ VERSION; head[W.CAPACITY] = cap; head[W.RATE] = 44100;
const data = new Float32Array(buf, HEADER_WORDS * 4, cap * 2);
const v = ringViews(buf, 0);
head[W.WRITE] = -20; head[W.READ] = -20;                       // cerca del desbordamiento u32
let seq = 0;
const L = new Float32Array(128), R = new Float32Array(128);
for (let round = 0; round < 40; round++) {
  const w = Atomics.load(head, W.WRITE) >>> 0;
  for (let i = 0; i < 100; i++) { const at = (((w + i) >>> 0) & (cap - 1)) * 2; data[at] = ++seq; data[at + 1] = -seq; }
  Atomics.store(head, W.WRITE, (w + 100) | 0);
  let got = 0;
  while (got < 100) {
    const n = ringRead(v, L, R, Math.min(128, 100 - got + 28));
    for (let i = 0; i < n; i++) { assert.equal(L[i], seq - 99 + got + i - 0); assert.equal(R[i], -L[i]); }
    got += n;
    if (n === 0) break;
  }
}
assert.equal(head[W.UNDERRUNS] > 0, true);
console.log('ok: anillo JS coherente, vaciados', head[W.UNDERRUNS]);
