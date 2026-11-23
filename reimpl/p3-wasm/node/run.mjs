// Ejecuta los bancos de p3-wasm en Node: `node run.mjs P3.wasm ui EXE CPR menu|city FRAMES`
//                                        `node run.mjs P3.wasm sim SAVE EXE CPR DAYS`
// Imprime el mismo JSON que `cargo run --example native_bench` (comparar con diff). Los tiempos van a stderr.
import fs from 'fs';
const [wasmPath, mode, ...a] = process.argv.slice(2);
const t0 = performance.now();
const marks = [];
const { instance } = await WebAssembly.instantiate(fs.readFileSync(wasmPath), { p3: { mark: (f) => marks.push([f, performance.now()]) } });
const x = instance.exports;
console.error(`instanciado en ${(performance.now() - t0).toFixed(0)} ms`);
const put = (path) => {
  const b = fs.readFileSync(path);
  const p = x.p3_alloc(b.length);
  new Uint8Array(x.memory.buffer, p, b.length).set(b); // memory.buffer se relee: puede haber crecido
  return [p, b.length];
};
if (mode === 'selftest') {
  const n = x.p3_selftest();
  console.log(n ? `autocomprobación: ${n} casos iguales` : 'autocomprobación: DIFIERE');
  process.exit(n ? 0 : 1);
}
const t1 = performance.now();
let n;
try {
if (mode === 'ui') {
  const [exe, cpr] = [put(a[0]), put(a[1])];
  n = x.p3_ui_bench(...exe, ...cpr, a[2] === 'city' ? 1 : 0, Number(a[3]));
} else {
  const [save, exe, cpr] = [put(a[0]), put(a[1]), put(a[2])];
  n = x.p3_sim_run(...save, ...cpr, ...exe, Number(a[3]));
}
} catch (e) {
  const m = x.p3_last_panic();
  console.error('PÁNICO:', Buffer.from(x.memory.buffer, x.p3_result_ptr(), m).toString() || e);
  process.exit(1);
}
console.error(`ejecutado en ${(performance.now() - t1).toFixed(0)} ms, memoria ${x.memory.buffer.byteLength >> 20} MiB`);
if (marks.length > 2 && process.env.P3_MARK) {
  // Tiempo hasta el primer fotograma, y por fotograma en régimen (mediana de los 30 últimos).
  const d = marks.slice(1).map((m, i) => m[1] - marks[i][1]);
  const tail = d.slice(-30).sort((p, q) => p - q);
  console.error(`marcas: primer fotograma ${(marks[1][1] - marks[0][1]).toFixed(0)} ms; mayor salto ${Math.max(...d.slice(1)).toFixed(0)} ms en el fotograma ${marks[d.indexOf(Math.max(...d.slice(1))) + 1][0]}; mediana de los últimos 30: ${tail[tail.length >> 1].toFixed(2)} ms`);
  if (process.env.P3_MARK === 'all') console.error(d.map((v, i) => `${marks[i + 1][0]}:${v.toFixed(1)}`).join(' '));
}
console.log(Buffer.from(x.memory.buffer, x.p3_result_ptr(), n).toString());
