import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;
const next = (): number => Number(tokens[pos++]);

const t = next();
const out: number[] = [];
for (let c = 0; c < t; c++) {
  const n = next();
  const k = next();
  const values = new Float64Array(n);
  for (let i = 0; i < n; i++) values[i] = next();
  values.sort(); // typed arrays sort numerically
  // Two pointers on the sorted values: if the smallest and the largest fit, the
  // smallest fits with everything in between too.
  let count = 0;
  let i = 0;
  let j = n - 1;
  while (i < j) {
    if (values[i] + values[j] <= k) {
      count += j - i;
      i++;
    } else {
      j--;
    }
  }
  out.push(count);
}
process.stdout.write(out.join("\n") + "\n");
