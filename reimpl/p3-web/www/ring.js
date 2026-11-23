// Anillo de entrada en un SharedArrayBuffer: la página escribe eventos de 4 palabras y el Worker los lee (sin postMessage, porque
// el Worker del juego no vuelve nunca al bucle de eventos).
// Int32Array: [0]=cabeza (productor), [1]=cola (consumidor), [2]=fotograma, [3]=parada; después CAP eventos × 4 palabras.
export const CAP = 1024;
export const RING_WORDS = 4 + CAP * 4;
export function newRing() { return new SharedArrayBuffer(RING_WORDS * 4); }

export function push(ring, kind, a = 0, b = 0, c = 0) {
  const w = new Int32Array(ring);
  const head = Atomics.load(w, 0), tail = Atomics.load(w, 1);
  if (head - tail >= CAP) return false; // lleno: se pierde el evento
  const o = 4 + (head % CAP) * 4;
  w[o] = kind; w[o + 1] = a; w[o + 2] = b; w[o + 3] = c;
  Atomics.store(w, 0, head + 1);
  return true;
}

export function pop(ring, out) { // out: Uint32Array(4) en la memoria de WASM
  const w = new Int32Array(ring);
  const tail = Atomics.load(w, 1);
  if (tail === Atomics.load(w, 0)) return 0;
  const o = 4 + (tail % CAP) * 4;
  out[0] = w[o]; out[1] = w[o + 1]; out[2] = w[o + 2]; out[3] = w[o + 3];
  Atomics.store(w, 1, tail + 1);
  return 1;
}

export function tickFrame(ring) { const w = new Int32Array(ring); Atomics.add(w, 2, 1); Atomics.notify(w, 2); }

// Espera al siguiente fotograma de la página; con tiempo máximo para no colgarse con la pestaña oculta.
export function waitFrame(ring, seen) {
  const w = new Int32Array(ring);
  Atomics.wait(w, 2, seen, 40);
  return Atomics.load(w, 2);
}
