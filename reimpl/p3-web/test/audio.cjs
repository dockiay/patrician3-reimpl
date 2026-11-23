// Prueba de sonido en Chromium sin cabeza: arranca el menú con el EXE/CPR reales y mide el anillo de audio durante N segundos.
// Uso: node test/audio.cjs CARPETA_CON_EXE_Y_CPR [segundos=30]   (los archivos del juego NO están en Git)
// Informa: vaciados del anillo (underruns: el AudioWorklet no encontró datos), fotogramas por delante y RMS de la salida.
const { chromium } = require('playwright');
const { spawn } = require('child_process');
const path = require('path');
const [dir, secs = '30'] = process.argv.slice(2);
(async () => {
  const srv = spawn('python3', [path.join(__dirname, '..', 'serve.py'), '8098'], { stdio: 'inherit', env: { ...process.env, P3_GAME_DIR: dir } });
  await new Promise(r => setTimeout(r, 800));
  const browser = await chromium.launch({ args: ['--no-sandbox', '--autoplay-policy=no-user-gesture-required'] });
  const page = await (await browser.newContext({ viewport: { width: 1000, height: +(process.env.P3_H || 800) } })).newPage();
  page.on('console', m => console.log('[consola]', m.text()));
  page.on('pageerror', e => console.log('[error de página]', e.message));
  const music = require('fs').existsSync(path.join(dir, 'Musik')) ? require('child_process').execSync(`cd "${dir}" && find Musik -type f`).toString().trim().split('\n') : [];
  console.log('música suelta:', music.length);
  await page.goto('http://127.0.0.1:8098/');
  // EXE y CPR: el servidor de pruebas (P3_GAME_DIR) los ofrece al botón «desde el servidor». La música suelta se escribe en OPFS
  // (game/Musik/...) desde la página con los bytes que Playwright sirve en /__music/ (serve.py no ofrece más que el EXE y el CPR).
  await page.route('**/__music/**', (route) => route.fulfill({ path: path.join(dir, decodeURIComponent(new URL(route.request().url()).pathname.slice('/__music/'.length))), headers: { 'Cross-Origin-Resource-Policy': 'same-origin' } }));
  await page.evaluate(async (m) => {
    const root = await navigator.storage.getDirectory();
    const game = await root.getDirectoryHandle('game', { create: true });
    for (const rel of m) {
      let d = game; const parts = rel.split('/'), name = parts.pop();
      for (const p of parts) d = await d.getDirectoryHandle(p, { create: true });
      const w = await (await d.getFileHandle(name, { create: true })).createWritable();
      await w.write(await (await fetch('/__music/' + rel)).arrayBuffer()); await w.close();
    }
  }, music);
  await page.click('#fromserver');
  await page.waitForFunction(() => !document.getElementById('play').disabled, null, { timeout: 120000 });
  await page.click('#play');
  await page.waitForFunction(() => document.body.classList.contains('running'), null, { timeout: 180000 });
  // P3_CLICKS="x,y,espera_ms;..." (coordenadas de la página a 1000×800): pulsa en el menú para llegar a la ciudad (p. ej. «Nueva partida»).
  await page.waitForTimeout(8000);
  await page.screenshot({ path: 'audio_menu.png' });
  for (const c of (process.env.P3_CLICKS || '').split(';').filter(Boolean)) {
    const [x, y, wait = 3000] = c.split(',').map(Number);
    await page.mouse.move(x, y, { steps: 4 }); await page.mouse.click(x, y); await page.waitForTimeout(wait);
    await page.screenshot({ path: `audio_click_${x}_${y}.png` });
  }
  const t0 = Date.now(); let base = null, last = null;
  while (Date.now() - t0 < secs * 1000) {
    await page.waitForTimeout(2000);
    last = await page.evaluate(() => window.p3AudioStats && window.p3AudioStats());
    if (!base && last && last.rms > 0) base = last; // tras el arranque (antes del primer fotograma el anillo está vacío y es normal)
    console.log(`t=${((Date.now() - t0) / 1000).toFixed(0)}s`, JSON.stringify(last));
  }
  await page.screenshot({ path: 'audio_menu.png' });
  console.log('log:', (await page.textContent('#log')).slice(-300));
  await browser.close(); srv.kill();
  const lost = base && last ? last.underruns - base.underruns : null;
  console.log(`vaciados desde que suena: ${lost} (en el arranque: ${base && base.underruns}); rms final ${last && last.rms}`);
  process.exit(lost === 0 ? 0 : 3);
})().catch(e => { console.error(e); process.exit(1); });
