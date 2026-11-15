import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;
const next = (): number => Number(tokens[pos++]);

const n = next();
const q = next();
const prices = new Float64Array(n);
for (let i = 0; i < n; i++) prices[i] = next();
prices.sort(); // typed arrays sort numerically
// total[k - 1]: the cost of the k cheapest rations (at most 5·10¹³: exact in a double).
const total = new Float64Array(n);
let sum = 0;
for (let i = 0; i < n; i++) {
  sum += prices[i];
  total[i] = sum;
}

/** The number of totals within the budget: the first index whose total exceeds it. */
function rations(budget: number): number {
  let lo = 0;
  let hi = n;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (total[mid] <= budget) lo = mid + 1;
    else hi = mid;
  }
  return lo;
}

const out: number[] = [];
for (let i = 0; i < q; i++) out.push(rations(next()));
process.stdout.write(out.join("\n") + "\n");
