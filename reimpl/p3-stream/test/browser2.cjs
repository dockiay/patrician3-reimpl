// Interacción real en Chromium: clic de ratón y toque táctil. node browser2.cjs URL CLAVE prefijo
const { chromium } = require('playwright');
(async () => {
  const [url, pw, pre] = process.argv.slice(2);
  const browser = await chromium.launch();
  const ctx = await browser.newContext({ httpCredentials: { username: 'u', password: pw }, viewport: { width: 1280, height: 720 }, hasTouch: true });
  const page = await ctx.newPage();
  await page.goto(url + '/#stats,auto');
  await page.waitForTimeout(2500);
  const t0 = Date.now();
  await page.mouse.move(470, 150); await page.mouse.down(); await page.mouse.up();
  await page.waitForTimeout(2500);
  await page.screenshot({ path: pre + '-clic.png' });
  // toque: cerrar con Esc del panel y tocar otro edificio
  await page.evaluate(() => document.getElementById('bEsc').click()); await page.waitForTimeout(1500);
  await page.touchscreen.tap(300, 340); await page.waitForTimeout(2500);
  await page.screenshot({ path: pre + '-toque.png' });
  console.log('stats:', JSON.stringify(await page.evaluate(() => window.__stats)));
  await browser.close();
})();
