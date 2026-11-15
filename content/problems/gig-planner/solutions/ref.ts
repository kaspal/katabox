import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const n = Number(tokens[0]);
const start = new Float64Array(n);
const end = new Float64Array(n);
const pay = new Float64Array(n);
for (let i = 0; i < n; i++) {
  start[i] = Number(tokens[1 + 3 * i]);
  end[i] = Number(tokens[2 + 3 * i]);
  pay[i] = Number(tokens[3 + 3 * i]);
}
const order = Array.from({ length: n }, (_, i) => i).sort((a, b) => end[a] - end[b]);
const ends = Float64Array.from(order, (i) => end[i]);

// best[i]: the best plan using only the first i offers (by end): a higher total wins,
// then fewer gigs. Either offer i is skipped, or it is taken after the best plan of the
// offers that end by its start (a binary search). Totals stay below 10¹⁴: exact.
const total = new Float64Array(n + 1);
const count = new Float64Array(n + 1);
for (let i = 0; i < n; i++) {
  const g = order[i];
  let lo = 0;
  let hi = i; // the number of offers among the first i that end by start[g]
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (ends[mid] <= start[g]) lo = mid + 1;
    else hi = mid;
  }
  const t = total[lo] + pay[g];
  const c = count[lo] + 1;
  if (t > total[i] || (t === total[i] && c < count[i])) {
    total[i + 1] = t;
    count[i + 1] = c;
  } else {
    total[i + 1] = total[i];
    count[i + 1] = count[i];
  }
}
process.stdout.write(`${total[n]} ${count[n]}\n`);
