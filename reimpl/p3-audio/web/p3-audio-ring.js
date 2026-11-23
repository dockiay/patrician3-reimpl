// Disposición del anillo de audio: debe coincidir con p3-audio/src/ring.rs (VERSION 1).
export const MAGIC = 0x50334152, VERSION = 1, HEADER_WORDS = 8;
export const W = { MAGIC: 0, CAPACITY: 1, WRITE: 2, READ: 3, RATE: 4, UNDERRUNS: 5, FLAGS: 6 };
export const FLAG_CONSUMER_RUNNING = 1;

/** Vistas sobre el anillo que vive en la memoria compartida (`buffer` = SharedArrayBuffer, `byteOffset` = ring_ptr). */
export function ringViews(buffer, byteOffset) {
  const head = new Int32Array(buffer, byteOffset, HEADER_WORDS);
  if ((head[W.MAGIC] >>> 0) !== ((MAGIC ^ VERSION) >>> 0)) throw new Error('anillo de audio: cabecera inválida');
  const capacity = head[W.CAPACITY] >>> 0;
  const data = new Float32Array(buffer, byteOffset + HEADER_WORDS * 4, capacity * 2);
  return { head, data, capacity, mask: capacity - 1, rate: head[W.RATE] >>> 0 };
}

/** Lee `frames` fotogramas en los canales `left`/`right` (Float32Array); el resto queda en silencio. Devuelve los leídos. */
export function ringRead(v, left, right, frames = left.length) {
  const w = Atomics.load(v.head, W.WRITE) >>> 0;
  const r = Atomics.load(v.head, W.READ) >>> 0;
  const avail = Math.min(((w - r) >>> 0), v.capacity);
  const n = Math.min(frames, avail);
  for (let i = 0; i < n; i++) {
    const at = (((r + i) >>> 0) & v.mask) * 2;
    left[i] = v.data[at];
    right[i] = v.data[at + 1];
  }
  for (let i = n; i < frames; i++) { left[i] = 0; right[i] = 0; }
  Atomics.store(v.head, W.READ, (r + n) | 0);
  if (n < frames) Atomics.add(v.head, W.UNDERRUNS, 1);
  return n;
}
