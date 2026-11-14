import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;
const next = (): number => Number(tokens[pos++]);

const n = next();
const levels = new Int32Array(n);
for (let i = 0; i < n; i++) levels[i] = next();
// low[k][i]: the lowest of the 2^k readings starting at i.
const low: Int32Array[] = [levels];
for (let k = 1; 1 << k <= n; k++) {
  const prev = low[k - 1];
  const half = 1 << (k - 1);
  const row = new Int32Array(n - (1 << k) + 1);
  for (let i = 0; i < row.length; i++) {
    const a = prev[i];
    const b = prev[i + half];
    row[i] = a < b ? a : b;
  }
  low.push(row);
}

const q = next();
const out: number[] = [];
for (let i = 0; i < q; i++) {
  const l = next() - 1;
  const r = next() - 1;
  const k = 31 - Math.clz32(r - l + 1);
  const row = low[k];
  const a = row[l];
  const b = row[r - (1 << k) + 1];
  out.push(a < b ? a : b);
}
process.stdout.write(out.join("\n") + "\n");
