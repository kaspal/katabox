import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const n = Number(tokens[0]);
let prev = Number(tokens[1]);
let total = prev;
let rises = 0;
let run = 1;
let longest = 1;
for (let i = 2; i <= n; i++) {
  const x = Number(tokens[i]);
  total += x;
  if (x === prev) {
    run++;
    if (run > longest) longest = run;
  } else {
    if (x > prev) rises++;
    run = 1;
  }
  prev = x;
}
process.stdout.write(`${total} ${rises} ${longest}\n`);
