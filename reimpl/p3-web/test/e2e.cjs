// Prueba de punta a punta en Chromium sin cabeza: sirve www/, importa EXE y CPR por el <input>, arranca y hace clic en el menú.
// Uso: node test/e2e.cjs CARPETA_CON_EXE_Y_CPR [captura.png]   (los archivos del juego NO están en Git)
const { chromium } = require('playwright');
const { spawn } = require('child_process');
const path = require('path');
const [dir, shot = 'menu.png'] = process.argv.slice(2);
(async () => {
  const srv = spawn('python3', [path.join(__dirname, '..', 'serve.py'), '8099'], { stdio: 'inherit', env: { ...process.env, P3_GAME_DIR: dir } });
  await new Promise(r => setTimeout(r, 800));
  const browser = await chromium.launch({ args: ['--no-sandbox'] });
  const page = await (await browser.newContext({ viewport: { width: 1000, height: 800 } })).newPage();
  page.on('console', m => console.log('[consola]', m.text()));
  page.on('pageerror', e => console.log('[error de página]', e.message));
  await page.goto('http://127.0.0.1:8099/');
  console.log('aislado:', await page.evaluate(() => crossOriginIsolated));
  // Playwright tarda mucho en subir 248 MB por CDP: el servidor de pruebas los sirve y la página crea los File.
  await page.evaluate(async () => {
    const dt = new DataTransfer();
    for (const n of ['Patrician3.exe', 'p2arch0_eng.cpr']) dt.items.add(new File([await (await fetch('/_game/' + n)).blob()], n));
    const input = document.getElementById('pickfallback'); input.files = dt.files; input.dispatchEvent(new Event('change'));
  });
  await page.waitForFunction(() => !document.getElementById('play').disabled, null, { timeout: 120000 });
  console.log('archivos:', await page.textContent('#files'));
  await page.click('#play');
  const t0 = Date.now();
  await page.waitForFunction(() => document.body.classList.contains('running'), null, { timeout: 180000 });
  console.log('arrancó en', Date.now() - t0, 'ms');
  await page.waitForTimeout(+process.env.P3_WAIT || 60000);
  await page.screenshot({ path: shot });
  // Ratón: pasar sobre «Options» y pulsar «Credits» (posiciones de la captura a 1000×800).
  await page.mouse.move(250, 490, { steps: 5 });
  await page.waitForTimeout(1500);
  await page.screenshot({ path: shot.replace('.png', '_hover.png') });
  await page.mouse.click(220, 545);
  await page.waitForTimeout(4000);
  await page.screenshot({ path: shot.replace('.png', '_click.png') });
  console.log('log:', (await page.textContent('#log')).slice(-300));
  await browser.close(); srv.kill();
})().catch(e => { console.error(e); process.exit(1); });
