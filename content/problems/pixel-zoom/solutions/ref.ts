import { readFileSync } from "node:fs";

const lines = readFileSync(0, "utf8").split("\n");
const [r, , k] = lines[0].trim().split(/\s+/).map(Number);
const out: string[] = [];
for (let y = 1; y <= r; y++) {
  const row = lines[y].trim();
  let wide = "";
  for (const ch of row) wide += ch.repeat(k);
  for (let j = 0; j < k; j++) out.push(wide);
}
process.stdout.write(out.join("\n") + "\n");
