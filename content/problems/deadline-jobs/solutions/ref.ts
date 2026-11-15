import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const n = Number(tokens[0]);
const deadline = new Float64Array(n);
const fee = new Float64Array(n);
for (let i = 0; i < n; i++) {
  deadline[i] = Number(tokens[1 + 2 * i]);
  fee[i] = Number(tokens[2 + 2 * i]);
}
const order = Array.from({ length: n }, (_, i) => i).sort((a, b) => fee[b] - fee[a]);
// Best fee first, each on the latest free day by its deadline (only days 1..n matter).
// free[d] points towards the latest free day ≤ d; day 0 means "none left". Taking a day
// links it to the day before (union-find with path halving).
const free = new Int32Array(n + 1);
for (let d = 0; d <= n; d++) free[d] = d;
const latest = (d: number): number => {
  while (free[d] !== d) {
    free[d] = free[free[d]];
    d = free[d];
  }
  return d;
};
let total = 0; // at most 10⁵ · 10⁹ = 10¹⁴: exact in a double
for (const j of order) {
  const day = latest(Math.min(deadline[j], n));
  if (day > 0) {
    free[day] = day - 1;
    total += fee[j];
  }
}
process.stdout.write(`${total}\n`);
