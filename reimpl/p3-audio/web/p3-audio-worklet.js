// AudioWorkletProcessor que vacía el anillo de audio del juego (p3-audio::ring). Cargar con
// `ctx.audioWorklet.addModule('p3-audio-worklet.js')`. Las funciones del anillo van incluidas (un worklet no puede
// importar módulos en todos los navegadores); su copia comentada para pruebas está en p3-audio-ring.js.
const HEADER_WORDS = 8, W_CAPACITY = 1, W_WRITE = 2, W_READ = 3, W_UNDERRUNS = 5, W_FLAGS = 6;
const MAGIC_V = (0x50334152 ^ 1) >>> 0;

class P3AudioProcessor extends AudioWorkletProcessor {
  constructor(options) {
    super();
    const { buffer, byteOffset } = options.processorOptions;
    this.head = new Int32Array(buffer, byteOffset, HEADER_WORDS);
    if ((this.head[0] >>> 0) !== MAGIC_V) throw new Error('anillo de audio: cabecera inválida');
    this.capacity = this.head[W_CAPACITY] >>> 0;
    this.mask = this.capacity - 1;
    this.data = new Float32Array(buffer, byteOffset + HEADER_WORDS * 4, this.capacity * 2);
    Atomics.or(this.head, W_FLAGS, 1);
    this.port.onmessage = (e) => { if (e.data === 'stop') this.stopped = true; };
  }
  process(_inputs, outputs) {
    if (this.stopped) { Atomics.and(this.head, W_FLAGS, ~1); return false; }
    const out = outputs[0];
    const left = out[0], right = out.length > 1 ? out[1] : null;
    const frames = left.length;
    const w = Atomics.load(this.head, W_WRITE) >>> 0;
    const r = Atomics.load(this.head, W_READ) >>> 0;
    const n = Math.min(frames, Math.min(((w - r) >>> 0), this.capacity));
    const d = this.data, mask = this.mask;
    for (let i = 0; i < n; i++) {
      const at = (((r + i) >>> 0) & mask) * 2;
      left[i] = d[at];
      if (right) right[i] = d[at + 1]; else left[i] = 0.5 * (d[at] + d[at + 1]);
    }
    for (let i = n; i < frames; i++) { left[i] = 0; if (right) right[i] = 0; }
    Atomics.store(this.head, W_READ, (r + n) | 0);
    if (n < frames) Atomics.add(this.head, W_UNDERRUNS, 1);
    return true;
  }
}
registerProcessor('p3-audio', P3AudioProcessor);
