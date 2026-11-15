import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;
const next = (): number => Number(tokens[pos++]);

const t = next();
const out: number[] = [];
for (let c = 0; c < t; c++) {
  const n = next();
  const heights = new Float64Array(n);
  for (let i = 0; i < n; i++) heights[i] = next();
  heights.sort(); // typed arrays sort numerically
  // The middle singer, or the lower of the two middle ones when n is even.
  out.push(heights[(n - 1) >> 1]);
}
process.stdout.write(out.join("\n") + "\n");
