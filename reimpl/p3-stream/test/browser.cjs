// Prueba de la página en Chromium sin cabeza: node browser.cjs URL CONTRASEÑA captura.png [segundos]
const { chromium } = require('playwright');
(async () => {
  const [url, pw, shot, secs = '8'] = process.argv.slice(2);
  const browser = await chromium.launch({ args: ['--autoplay-policy=no-user-gesture-required'] });
  const ctx = await browser.newContext({ httpCredentials: { username: 'u', password: pw }, viewport: { width: 1024, height: 768 } });
  const page = await ctx.newPage();
  page.on('console', (m) => console.log('consola:', m.text()));
  page.on('pageerror', (e) => console.log('error de página:', e.message));
  await page.goto(url + '/#stats,auto');
  const h264 = await page.evaluate(async () => 'VideoDecoder' in window ? JSON.stringify(await VideoDecoder.isConfigSupported({ codec: 'avc1.42c01f' })) : 'sin VideoDecoder');
  console.log('H.264 en este Chromium:', h264);
  await page.waitForTimeout(+secs * 1000);
  console.log('stats:', JSON.stringify(await page.evaluate(() => window.__stats)));
  await page.screenshot({ path: shot });
  await browser.close();
})();
