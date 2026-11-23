// Pegamento del hilo principal: audio del juego sobre AudioContext + AudioWorklet (ver p3-audio/src/web.rs).
//
//   const ctx = await createAudioContext();          // dentro de un gesto del usuario (clic en «Jugar»)
//   worker.postMessage({ audioRate: ctx.sampleRate }); // el Worker crea WebAudio::new(rate, 150) y responde con ring_ptr
//   const node = await attachRing(ctx, wasmMemory, ringPtr, new URL('./p3-audio-worklet.js', import.meta.url));
//
// `wasmMemory` debe ser la WebAssembly.Memory compartida del Worker (build con atomics y bulk-memory).

/** Crea el contexto de audio; hay que llamarlo desde un gesto del usuario para que no empiece suspendido. */
export async function createAudioContext() {
  const ctx = new AudioContext({ latencyHint: 'interactive' });
  if (ctx.state === 'suspended') await ctx.resume();
  return ctx;
}

/** Conecta el anillo (en la memoria compartida, a `ringPtr` bytes) a la salida del contexto. */
export async function attachRing(ctx, wasmMemory, ringPtr, workletUrl) {
  if (!(wasmMemory.buffer instanceof SharedArrayBuffer)) {
    throw new Error('el audio necesita memoria WASM compartida (cabeceras COOP/COEP)');
  }
  await ctx.audioWorklet.addModule(workletUrl);
  const node = new AudioWorkletNode(ctx, 'p3-audio', {
    numberOfInputs: 0,
    numberOfOutputs: 1,
    outputChannelCount: [2],
    processorOptions: { buffer: wasmMemory.buffer, byteOffset: ringPtr },
  });
  node.connect(ctx.destination);
  return node;
}

/** Reanuda el contexto si el navegador lo suspendió (pestaña en segundo plano, política de autoplay). */
export function keepAudioAlive(ctx) {
  const resume = () => { if (ctx.state !== 'running') ctx.resume().catch(() => {}); };
  for (const ev of ['pointerdown', 'keydown']) addEventListener(ev, resume, { passive: true });
  ctx.onstatechange = resume;
}
