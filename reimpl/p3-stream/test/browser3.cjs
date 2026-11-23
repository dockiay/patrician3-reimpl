const { chromium } = require('playwright');
(async () => {
  const [url, pw, shot, x, y] = process.argv.slice(2);
  const browser = await chromium.launch();
  const ctx = await browser.newContext({ httpCredentials: { username: 'u', password: pw }, viewport: { width: 1280, height: 720 }, hasTouch: true });
  const page = await ctx.newPage();
  await page.goto(url + '/#auto');
  await page.waitForTimeout(2500);
  await page.touchscreen.tap(+x, +y); await page.waitForTimeout(2500);
  await page.screenshot({ path: shot });
  await browser.close();
})();
