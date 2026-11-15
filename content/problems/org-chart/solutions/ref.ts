import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;
const next = (): number => Number(tokens[pos++]);

const n = next();
const team: number[][] = Array.from({ length: n + 1 }, () => []);
for (let i = 2; i <= n; i++) team[next()].push(i);
// Number the employees in depth-first order, with an explicit stack (a chart can be
// 10⁵ levels deep: far too deep for recursion). first[x] is x's number; last[x] the
// largest number in x's team, so the team is exactly first[x]..last[x].
const first = new Int32Array(n + 1);
const last = new Int32Array(n + 1);
let clock = 0;
const stack: number[] = [1]; // x to enter, or -x to leave
while (stack.length > 0) {
  const x = stack.pop()!;
  if (x < 0) {
    last[-x] = clock;
    continue;
  }
  first[x] = ++clock;
  stack.push(-x);
  for (const y of team[x]) stack.push(y);
}
const q = next();
const out: string[] = [];
for (let i = 0; i < q; i++) {
  const a = next();
  const b = next();
  out.push(a !== b && first[a] <= first[b] && first[b] <= last[a] ? "YES" : "NO");
}
process.stdout.write(out.join("\n") + "\n");
