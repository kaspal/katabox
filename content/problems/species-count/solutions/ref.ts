import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;
const next = (): number => Number(tokens[pos++]);

const n = next();
const ids: number[] = [];
for (let i = 0; i < n; i++) ids.push(next());
const q = next();
const byEnd: number[][] = Array.from({ length: n + 1 }, () => []);
const starts = new Int32Array(q);
for (let k = 0; k < q; k++) {
  starts[k] = next();
  byEnd[next()].push(k);
}
// Offline, by right end: while the log is read up to photo r, mark only the latest
// photo of each species. Then the species of l..r are the marks in l..r. A Fenwick
// tree keeps the marks' prefix counts.
const tree = new Int32Array(n + 1);
const add = (at: number, delta: number): void => {
  for (let i = at; i <= n; i += i & -i) tree[i] += delta;
};
const prefix = (at: number): number => {
  let total = 0;
  for (let i = at; i > 0; i -= i & -i) total += tree[i];
  return total;
};
const latest = new Map<number, number>();
const answer = new Int32Array(q);
for (let r = 1; r <= n; r++) {
  const before = latest.get(ids[r - 1]);
  if (before !== undefined) add(before, -1);
  latest.set(ids[r - 1], r);
  add(r, 1);
  for (const k of byEnd[r]) answer[k] = prefix(r) - prefix(starts[k] - 1);
}
process.stdout.write(answer.join("\n") + "\n");
