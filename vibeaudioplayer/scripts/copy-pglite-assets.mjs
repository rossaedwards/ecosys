import { copyFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";

const srcDir = "node_modules/@electric-sql/pglite/dist";
const destDir = ".vercel/output/functions/__server.func/_libs";

if (!existsSync(destDir)) {
  console.log("[pglite-assets] skip — vercel output not present");
  process.exit(0);
}

mkdirSync(destDir, { recursive: true });
for (const name of ["pglite.data", "pglite.wasm", "initdb.wasm"]) {
  const from = join(srcDir, name);
  const to = join(destDir, name);
  if (!existsSync(from)) {
    console.warn(`[pglite-assets] missing ${from}`);
    continue;
  }
  copyFileSync(from, to);
  console.log(`[pglite-assets] copied ${name} -> ${dirname(to)}`);
}
