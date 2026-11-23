// Pilota el juego en Chromium sin cabeza por HTTP (para probar a mano o con scripts): node test/drive.cjs CARPETA_JUEGO [puerto]
//   /click?x=&y=  (coordenadas del juego)   /move?x=&y=   /key?code=Enter   /shot?name=a.png   /wait?ms=   /log   /status
//   /import?file=RUTA.pat   /eval?js=EXPR   /restart   /quit
const { chromium } = require('playwright');
const { spawn } = require('child_process');
const http = require('http'), path = require('path'), fs = require('fs');
const [dir, cport = '9100'] = process.argv.slice(2);
(async () => {
  // P3_DIST=1: sirve el paquete de web/dist (service worker, p3-load.js, .wasm optimizado) con web/serve.py y los archivos del juego
  // se inyectan interceptando /_game/ (ese servidor no los tiene).
  const dist = !!process.env.P3_DIST, base = dist ? 'http://localhost:8766/' : 'http://127.0.0.1:8097/';
  const srv = dist ? spawn('python3', [path.join(__dirname, '..', '..', 'web', 'serve.py')], { stdio: 'ignore' })
    : spawn('python3', [path.join(__dirname, '..', 'serve.py'), '8097'], { stdio: 'ignore', env: { ...process.env, P3_GAME_DIR: dir } });
  await new Promise(r => setTimeout(r, 800));
  const browser = await chromium.launch({ args: ['--no-sandbox', '--enable-unsafe-swiftshader'] });
  const page = await (await browser.newContext({ viewport: { width: 1000, height: 800 }, acceptDownloads: true, serviceWorkers: process.env.P3_DIST ? 'block' : 'allow' })).newPage();
  const logs = [];
  page.on('console', m => logs.push('[consola] ' + m.text()));
  page.on('pageerror', e => logs.push('[error] ' + e.message));
  if (dist) { // proxy en 8766: /_game/ desde el disco y el resto de web/serve.py (que no tiene los archivos del juego)
    http.createServer((q, r) => {
      const h = { 'Cross-Origin-Opener-Policy': 'same-origin', 'Cross-Origin-Embedder-Policy': 'require-corp', 'Cross-Origin-Resource-Policy': 'same-origin' };
      if (q.url.startsWith('/_game/')) { const f = path.join(dir, decodeURIComponent(q.url.split('?')[0].split('/').pop())); r.writeHead(200, { ...h, 'content-length': fs.statSync(f).size }); return fs.createReadStream(f).pipe(r); }
      const u = http.request({ host: 'localhost', port: 8765, path: q.url, method: q.method, headers: q.headers }, (p) => { r.writeHead(p.statusCode, p.headers); p.pipe(r); });
      q.pipe(u);
    }).listen(8766);
  }
  await page.goto(base);
  if (dist) { await page.waitForTimeout(10000); await page.waitForLoadState('load'); console.log('crossOriginIsolated', await page.evaluate(() => crossOriginIsolated)); }
  // P3_FROM_SERVER=1: botón «Cargar desde el PC» (lista de serve.py: CPR de cada idioma, idioma/*.pack y Musik/**).
  if (process.env.P3_FROM_SERVER) await page.click('#fromserver');
  else await page.evaluate(async () => {
    const dt = new DataTransfer();
    for (const n of ['Patrician3.exe', 'p2arch0_eng.cpr']) dt.items.add(new File([await (await fetch('/_game/' + n)).blob()], n));
    const input = document.getElementById('pickfallback'); input.files = dt.files; input.dispatchEvent(new Event('change'));
  });
  const play = async () => {
    await page.waitForFunction(() => !document.getElementById('play').disabled, null, { timeout: 600000 });
    await page.selectOption('#res', process.env.P3_RES || 'saved');
    await page.click('#play');
    await page.waitForFunction(() => document.body.classList.contains('running'), null, { timeout: 120000 });
  };
  await play();
  const pt = async (x, y) => { const r = await page.evaluate(() => { const c = document.getElementById('screen'), b = c.getBoundingClientRect(); return { l: b.left, t: b.top, w: b.width, h: b.height, cw: c.width, ch: c.height }; }); return [r.l + x * r.w / r.cw, r.t + y * r.h / r.ch]; };
  http.createServer(async (q, s) => {
    const u = new URL(q.url, 'http://x'), p = u.searchParams; let out = 'ok';
    try {
      switch (u.pathname) {
        case '/click': { const [x, y] = await pt(+p.get('x'), +p.get('y')); await page.mouse.move(x, y, { steps: 3 }); await page.waitForTimeout(150); await page.mouse.down(); await page.waitForTimeout(80); await page.mouse.up(); break; }
        case '/move': { const [x, y] = await pt(+p.get('x'), +p.get('y')); await page.mouse.move(x, y, { steps: 3 }); break; }
        case '/key': await page.keyboard.press(p.get('code')); break;
        case '/type': await page.keyboard.type(p.get('text')); break;
        case '/shot': await page.locator('#screen').screenshot({ path: p.get('name') }); break;
        case '/wait': await page.waitForTimeout(+p.get('ms')); break;
        case '/log': out = logs.splice(0).join('\n'); break;
        case '/status': out = await page.evaluate(() => document.getElementById('status').textContent + ' | ' + document.getElementById('log').textContent.slice(-600)); break;
        case '/eval': out = JSON.stringify(await page.evaluate(p.get('js'))); break;
        case '/import': { const f = p.get('file'); await page.setInputFiles('#importfile', f); await page.waitForTimeout(1500); break; }
        case '/restart': await page.reload(); await play(); break; // recarga la página y vuelve a jugar (idioma elegido en Opciones)
        case '/quit': s.end('bye'); await browser.close(); srv.kill(); process.exit(0);
      }
    } catch (e) { out = 'ERR ' + e.message; }
    s.end(out + '\n');
  }).listen(+cport, '127.0.0.1');
  console.log('listo en', cport);
})().catch(e => { console.error(e); process.exit(1); });
