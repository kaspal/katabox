import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const t = Number(tokens[0]);
let pos = 1;
const out: string[] = [];
for (let day = 0; day < t; day++) {
  const n = Number(tokens[pos]);
  let total = 0;
  let highest = 0;
  for (let k = 1; k <= n; k++) {
    const price = Number(tokens[pos + k]);
    total += price;
    if (price > highest) highest = price;
  }
  pos += 1 + n;
  out.push(`${n} ${total} ${highest}`);
}
process.stdout.write(out.join("\n") + "\n");
