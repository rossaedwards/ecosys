import { chromium } from "playwright";

const url = process.argv[2] || "http://127.0.0.1:8080/";
const out = process.argv[3] || "/workspace/screenshots/vibe-qa.png";

const browser = await chromium.launch({ args: ["--autoplay-policy=no-user-gesture-required"] });
const page = await browser.newPage({ viewport: { width: 390, height: 844 } });
const errors = [];
page.on("pageerror", (e) => errors.push(String(e)));
page.on("console", (msg) => {
  if (msg.type() === "error") errors.push(msg.text());
});
await page.goto(url, { waitUntil: "networkidle" });
await page.waitForTimeout(800);

await page.locator('[data-testid="play-toggle"]').click();
await page.waitForTimeout(1800);
const playLabel = await page.locator('[data-testid="play-toggle"]').getAttribute("aria-label");
await page.screenshot({ path: out.replace(".png", "-playing.png") });

await page.getByRole("button", { name: "VASP" }).click();
await page.waitForTimeout(500);
await page.screenshot({ path: out.replace(".png", "-vasp.png") });
await page.keyboard.press("Escape");
await page.waitForTimeout(400);

await page.getByRole("button", { name: "Scene" }).click();
await page.waitForTimeout(500);
const accountText = await page.locator("text=Account").locator("..").innerText();
await page.screenshot({ path: out.replace(".png", "-settings.png") });

console.log(JSON.stringify({ errors, playLabel, accountText }, null, 2));
await browser.close();
