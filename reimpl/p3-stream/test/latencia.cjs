// Latencia de extremo a extremo con la imagen quieta (pantalla Opciones del juego): mueve el ratón y mide, en la página, desde el
// evento hasta que se descodifica el primer fotograma nuevo. node latencia.cjs URL CLAVE
const { chromium } = require('playwright');
(async () => {
  const [url, pw] = process.argv.slice(2);
  const browser = await chromium.launch();
  const ctx = await browser.newContext({ httpCredentials: { username: 'u', password: pw }, viewport: { width: 1280, height: 720 } });
  const page = await ctx.newPage();
  await page.goto(url + '/#auto');
  await page.waitForTimeout(2000);
  await page.evaluate(() => document.getElementById('bEsc').click());
  await page.waitForTimeout(2500);
  const lat = [];
  for (let i = 0; i < 25; i++) {
    await page.mouse.move(300 + (i % 2) * 150, 300 + i * 6);
    await page.waitForTimeout(400);
    lat.push(await page.evaluate(() => (window.__st.firstDec - window.__st.lastMoveT)));
  }
  lat.sort((a, b) => a - b);
  console.log('latencia ms (ordenadas):', lat.map((x) => Math.round(x)).join(' '), '| mediana', Math.round(lat[12]));
  await browser.close();
})();
