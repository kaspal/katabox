import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const n = Number(tokens[0]);
const k = Number(tokens[1]);
const ids = tokens.slice(2, 2 + n).map(Number);

/** How many stretches have at most m different sources (a sliding window). */
function atMost(m: number): number {
  if (m === 0) return 0;
  const count = new Map<number, number>();
  let left = 0;
  let total = 0;
  for (let right = 0; right < n; right++) {
    count.set(ids[right], (count.get(ids[right]) ?? 0) + 1);
    while (count.size > m) {
      const y = ids[left++];
      const c = count.get(y)! - 1;
      if (c === 0) count.delete(y);
      else count.set(y, c);
    }
    // Every stretch ending at `right` and starting at left..right qualifies.
    total += right - left + 1;
  }
  return total;
}

// Exactly K = at most K minus at most K − 1.
process.stdout.write(`${atMost(k) - atMost(k - 1)}\n`);
