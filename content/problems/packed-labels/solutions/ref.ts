import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const labelCount = Number(tokens[0]);
const ends: number[][] = [];
const letters: string[][] = [];
for (let l = 1; l <= labelCount; l++) {
  const label = tokens[l];
  const labelEnds: number[] = [];
  const labelLetters: string[] = [];
  let count = 0;
  let total = 0;
  for (let j = 0; j < label.length; j++) {
    const c = label.charCodeAt(j);
    if (c >= 48 && c <= 57) {
      count = count * 10 + (c - 48);
    } else {
      total += count;
      labelEnds.push(total);
      labelLetters.push(label[j]);
      count = 0;
    }
  }
  ends.push(labelEnds);
  letters.push(labelLetters);
}
const q = Number(tokens[1 + labelCount]);
let pos = 2 + labelCount;
const out: string[] = [];
for (let j = 0; j < q; j++) {
  const i = Number(tokens[pos]) - 1;
  const k = Number(tokens[pos + 1]);
  pos += 2;
  const e = ends[i];
  // First run whose end is at least k.
  let lo = 0;
  let hi = e.length - 1;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (e[mid] >= k) hi = mid;
    else lo = mid + 1;
  }
  out.push(letters[i][lo]);
}
process.stdout.write(out.join("\n") + "\n");
