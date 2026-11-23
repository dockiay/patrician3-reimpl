const { chromium } = require('playwright');
(async () => {
  const [url, pw, shot, w, h, cx, cy] = process.argv.slice(2);
  const b = await chromium.launch(); const c = await b.newContext({ httpCredentials: { username: 'u', password: pw }, viewport: { width: +w, height: +h } });
  const p = await c.newPage(); await p.goto(url + '/#auto'); await p.waitForTimeout(2500);
  if (cx) { await p.mouse.click(+cx, +cy); await p.waitForTimeout(1500); }
  await p.screenshot({ path: shot }); await b.close();
})();
