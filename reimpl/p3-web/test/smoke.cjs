// Prueba sin archivos del juego: la página instancia el wasm (con excepciones), el Worker deja listos HostFs y partidas, y el
// selector/lista de partidas responde. Uso: node test/smoke.cjs
const { chromium } = require('playwright');
const { spawn } = require('child_process');
const path = require('path');
(async () => {
  const srv = spawn('python3', [path.join(__dirname, '..', 'serve.py'), '8098'], { stdio: 'inherit' });
  await new Promise(r => setTimeout(r, 800));
  const browser = await chromium.launch({ args: ['--no-sandbox'] });
  const page = await (await browser.newContext({ viewport: { width: 1000, height: 800 } })).newPage();
  const errors = [];
  page.on('console', m => console.log('[consola]', m.text()));
  page.on('pageerror', e => { errors.push(e.message); console.log('[error de página]', e.message); });
  await page.goto('http://127.0.0.1:8098/');
  console.log('aislado:', await page.evaluate(() => crossOriginIsolated));
  await page.waitForFunction(() => document.getElementById('status').textContent === '' || /Error|error|Falta/.test(document.getElementById('status').textContent), null, { timeout: 60000 });
  console.log('estado:', JSON.stringify(await page.textContent('#status')));
  console.log('partidas:', JSON.stringify(await page.textContent('#saves')));
  await browser.close(); srv.kill();
  process.exit(errors.length ? 1 : 0);
})().catch(e => { console.error(e); process.exit(1); });
