import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;
const next = (): number => Number(tokens[pos++]);

const n = next();
const m = next();
const unlocks: number[][] = Array.from({ length: n + 1 }, () => []);
const missing = new Int32Array(n + 1); // prerequisites not passed yet (a repeated rule counts twice)
for (let i = 0; i < m; i++) {
  const a = next();
  const b = next();
  unlocks[a].push(b);
  missing[b]++;
}
// Kahn's algorithm, one term at a time: a term is every course whose last missing
// prerequisite was passed in the term before.
let term: number[] = [];
for (let c = 1; c <= n; c++) if (missing[c] === 0) term.push(c);
const lines: string[] = [];
let passed = 0;
while (term.length > 0) {
  term.sort((x, y) => x - y);
  lines.push(term.join(" "));
  passed += term.length;
  const following: number[] = [];
  for (const a of term) {
    for (const b of unlocks[a]) {
      if (--missing[b] === 0) following.push(b);
    }
  }
  term = following;
}
if (passed < n) process.stdout.write("IMPOSSIBLE\n");
else process.stdout.write(`${lines.length}\n${lines.join("\n")}\n`);
