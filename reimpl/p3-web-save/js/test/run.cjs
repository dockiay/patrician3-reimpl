// Prueba de extremo a extremo en Chromium (OPFS real): node js/test/run.cjs RUTA_AUTO.pat RUTA_p3_web_save.wasm
const http = require('http'), fs = require('fs'), path = require('path');
const { chromium } = require('playwright');
const [pat, wasm] = process.argv.slice(2);
const js = path.join(__dirname, '..');
const types = { '.js': 'text/javascript', '.html': 'text/html', '.wasm': 'application/wasm' };
const srv = http.createServer((q, r) => {
  const u = q.url.split('?')[0];
  const f = u === '/p3_web_save.wasm' ? wasm : u === '/AUTO.pat' ? pat : u === '/' ? path.join(__dirname, 'page.html') : path.join(js, u.replace(/^\/js\//, '').replace(/^\//, ''));
  fs.readFile(f, (e, d) => { if (e) { r.writeHead(404); r.end(); } else { r.writeHead(200, { 'content-type': types[path.extname(f)] || 'application/octet-stream' }); r.end(d); } });
}).listen(0, async () => {
  const port = srv.address().port;
  const b = await chromium.launch();
  const p = await b.newPage();
  p.on('console', (m) => console.log('[page]', m.text()));
  p.on('pageerror', (e) => { console.log('ERROR', e); process.exitCode = 1; });
  await p.goto(`http://localhost:${port}/`);
  const res = await p.evaluate(() => window.runTest());
  console.log(JSON.stringify(res, null, 1));
  await b.close(); srv.close();
  if (!res.ok) process.exitCode = 1;
});
