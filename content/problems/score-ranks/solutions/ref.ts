import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const n = Number(tokens[0]);
const scores: number[] = [];
for (let i = 1; i <= n; i++) scores.push(Number(tokens[i]));
// Distinct scores, highest first.
scores.sort((a, b) => b - a);
const distinct: number[] = [];
for (const s of scores) {
  if (distinct.length === 0 || distinct[distinct.length - 1] !== s) distinct.push(s);
}
const q = Number(tokens[1 + n]);
const answers: number[] = [];
for (let j = 0; j < q; j++) {
  const x = Number(tokens[2 + n + j]);
  // Binary search in a decreasing list: first index whose score is ≤ x.
  let lo = 0;
  let hi = distinct.length;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (distinct[mid] > x) lo = mid + 1;
    else hi = mid;
  }
  answers.push(lo < distinct.length && distinct[lo] === x ? lo + 1 : 0);
}
process.stdout.write(answers.join(" ") + "\n");
