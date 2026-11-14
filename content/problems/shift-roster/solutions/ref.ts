import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const n = Number(tokens[0]);
const names: string[] = [];
const hours: number[] = [];
let total = 0;
for (let i = 0; i < n; i++) {
  names.push(tokens[1 + 2 * i]);
  const h = Number(tokens[2 + 2 * i]);
  hours.push(h);
  total += h;
}
const out: string[] = [String(total)];
for (let i = 0; i < n; i++) {
  // hours > total / n, in whole numbers
  if (hours[i] * n > total) out.push(`${names[i]} ${hours[i]}`);
}
if (out.length === 1) out.push("none");
process.stdout.write(out.join("\n") + "\n");
