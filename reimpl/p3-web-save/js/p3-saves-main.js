// WEB-4: partidas del navegador en OPFS, lado del hilo principal (o de un Worker de almacenamiento).
//
// El Worker del juego guarda en memoria (Rust, `SaveFs`) y manda cada archivo terminado con
// `{type:'p3-save-commit', rel, data|null}`. Aquí se escribe en OPFS, se importan y exportan `.pat`/`.pst` y se pide
// `navigator.storage.persist()` para que el navegador no borre las partidas. Todo bajo `save/ein` (las rutas `rel` son
// relativas a la carpeta del juego: `save/ein/AUTO.pat`). OPFS distingue mayúsculas y el juego no, así que cada tramo se
// busca sin distinguirlas.

const SAVE_EXT = /\.(pat|pst)$/i;
const MAX_IMPORT = 64 * 1024 * 1024;

export class SaveStore {
  /** @param {{root?: FileSystemDirectoryHandle, saveDir?: string}} [opts] `root` solo se pasa en pruebas. */
  constructor(opts = {}) {
    this.rootPromise = opts.root ? Promise.resolve(opts.root) : navigator.storage.getDirectory();
    this.saveDir = opts.saveDir ?? 'save/ein';
    this.queue = Promise.resolve();
    this.port = null;
    this.pending = new Map();
    this.nextId = 1;
    this.errors = [];
    this.onerror = null;
  }

  /** Pide almacenamiento persistente. Devuelve true si el navegador lo concede (o ya estaba concedido). */
  async persist() {
    try {
      if (!navigator.storage?.persist) return false;
      if (await navigator.storage.persisted()) return true;
      return await navigator.storage.persist();
    } catch {
      return false;
    }
  }

  /** Estado de la cuota para mostrarlo en la página. */
  async estimate() {
    try { return await navigator.storage.estimate(); } catch { return {}; }
  }

  // --- rutas -----------------------------------------------------------------------------------------------------------

  static parts(rel) {
    return rel.replaceAll('\\', '/').split('/').filter((p) => p && p !== '.');
  }

  async dir(parts, create) {
    let d = await this.rootPromise;
    for (const part of parts) {
      let found = null;
      for await (const [name, h] of d.entries()) {
        if (h.kind === 'directory' && name.toLowerCase() === part.toLowerCase()) { found = name; break; }
      }
      if (found === null) {
        if (!create) return null;
        found = part.toLowerCase();
      }
      d = await d.getDirectoryHandle(found, { create });
    }
    return d;
  }

  async fileHandle(rel, create) {
    const parts = SaveStore.parts(rel);
    const name = parts.pop();
    const d = await this.dir(parts, create);
    if (!d) return null;
    for await (const [n, h] of d.entries()) {
      if (h.kind === 'file' && n.toLowerCase() === name.toLowerCase()) return { d, h, name: n };
    }
    if (!create) return null;
    return { d, h: await d.getFileHandle(name, { create: true }), name };
  }

  // --- operaciones sobre OPFS ------------------------------------------------------------------------------------------

  /** Encadena las escrituras para que lleguen en el orden en que el juego las hizo. */
  enqueue(job) {
    const run = this.queue.then(job);
    this.queue = run.catch((e) => {
      this.errors.push(String(e));
      if (this.onerror) this.onerror(e);
    });
    return run;
  }

  /** Espera a que terminen las escrituras pendientes (antes de exportar o cerrar). */
  flush() { return this.queue; }

  write(rel, bytes) {
    return this.enqueue(async () => {
      const f = await this.fileHandle(rel, true);
      // `createWritable` escribe en un archivo temporal y lo sustituye al cerrar: una partida nunca queda a medias.
      const w = await f.h.createWritable();
      try { await w.write(bytes); } finally { await w.close(); }
    });
  }

  remove(rel) {
    return this.enqueue(async () => {
      const f = await this.fileHandle(rel, false);
      if (f) await f.d.removeEntry(f.name);
    });
  }

  /** Lista las partidas: `[{rel, name, size, modified}]`, ordenadas por fecha, la más reciente primero. */
  async list() {
    await this.flush();
    const d = await this.dir(SaveStore.parts(this.saveDir), false);
    const out = [];
    if (d) {
      for await (const [name, h] of d.entries()) {
        if (h.kind !== 'file' || !SAVE_EXT.test(name)) continue;
        const f = await h.getFile();
        out.push({ rel: `${this.saveDir}/${name}`, name, size: f.size, modified: f.lastModified });
      }
    }
    return out.sort((a, b) => b.modified - a.modified);
  }

  /** Todo lo que hay en `save/` con su contenido, para precargar la memoria del Worker. */
  async readAll() {
    await this.flush();
    const out = [];
    const walk = async (d, prefix) => {
      for await (const [name, h] of d.entries()) {
        if (h.kind === 'directory') await walk(h, `${prefix}/${name}`);
        else {
          const f = await h.getFile();
          out.push({ rel: `${prefix}/${name}`, data: new Uint8Array(await f.arrayBuffer()), modified: Math.floor(f.lastModified / 1000) });
        }
      }
    };
    const d = await this.dir(['save'], false);
    if (d) await walk(d, 'save');
    return out;
  }

  async read(rel) {
    await this.flush();
    const f = await this.fileHandle(rel, false);
    return f ? new Uint8Array(await (await f.h.getFile()).arrayBuffer()) : null;
  }

  // --- Worker del juego --------------------------------------------------------------------------------------------------

  /** Conecta con el Worker (o MessagePort) del juego: recibe sus confirmaciones y le manda la precarga. */
  attach(port) {
    this.port = port;
    port.addEventListener('message', (ev) => {
      const m = ev.data;
      if (!m || typeof m !== 'object') return;
      if (m.type === 'p3-save-commit') {
        if (m.data == null) this.remove(m.rel);
        else this.write(m.rel, m.data);
      } else if (m.type === 'p3-save-import-result') {
        const p = this.pending.get(m.id);
        if (p) { this.pending.delete(m.id); p(!!m.ok); }
      }
    });
    if (port.start) port.start();
  }

  /** Manda al Worker el contenido de `save/` (hay que hacerlo antes de arrancar el juego). */
  async sendInitialSaves() {
    const files = await this.readAll();
    this.port.postMessage({ type: 'p3-save-init', files }, files.map((f) => f.data.buffer));
    return files.length;
  }

  // --- importar y exportar ---------------------------------------------------------------------------------------------

  /** Pide al Worker que valide la partida; sin Worker solo se comprueba el tamaño mínimo. */
  validate(rel, bytes) {
    if (!SAVE_EXT.test(rel)) return Promise.resolve(false);
    if (!this.port) return Promise.resolve(bytes.length >= 64);
    const id = this.nextId++;
    return new Promise((resolve) => {
      this.pending.set(id, resolve);
      this.port.postMessage({ type: 'p3-save-import', id, rel, data: bytes });
    });
  }

  /**
   * Importa un `File` (`.pat` del juego original o de `p3-juego`, o `.pst`). Devuelve `{ok, rel, reason}`. Si ya existe una
   * partida con ese nombre, no la sustituye salvo con `overwrite`.
   */
  async importFile(file, { overwrite = false } = {}) {
    const name = file.name.replace(/[\\/:*?"<>|]/g, '_');
    const rel = `${this.saveDir}/${name}`;
    if (!SAVE_EXT.test(name)) return { ok: false, rel, reason: 'extension' };
    if (file.size > MAX_IMPORT || file.size === 0) return { ok: false, rel, reason: 'size' };
    await this.flush();
    if (!overwrite && (await this.fileHandle(rel, false))) return { ok: false, rel, reason: 'exists' };
    const bytes = new Uint8Array(await file.arrayBuffer());
    if (!(await this.validate(rel, bytes))) return { ok: false, rel, reason: 'invalid' };
    await this.write(rel, bytes);
    return { ok: true, rel };
  }

  /** Contenido de una partida como `Blob` para descargarla. */
  async exportBlob(rel) {
    const bytes = await this.read(rel);
    return bytes ? new Blob([bytes], { type: 'application/octet-stream' }) : null;
  }

  /** Descarga `rel` con el nombre original (solo en el hilo principal). */
  async download(rel) {
    const blob = await this.exportBlob(rel);
    if (!blob) return false;
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = SaveStore.parts(rel).pop();
    document.body.appendChild(a);
    a.click();
    a.remove();
    setTimeout(() => URL.revokeObjectURL(a.href), 10000);
    return true;
  }

  /** Borra una partida desde la página (el Worker la olvida). */
  async delete(rel) {
    await this.remove(rel);
    if (this.port) this.port.postMessage({ type: 'p3-save-forget', rel });
  }
}
