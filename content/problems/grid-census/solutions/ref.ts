import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const r = Number(tokens[0]);
const c = Number(tokens[1]);
const rowTrees: number[] = new Array(r).fill(0);
const colTrees: number[] = new Array(c).fill(0);
for (let i = 0; i < r; i++) {
  const row = tokens[2 + i];
  for (let j = 0; j < c; j++) {
    if (row.charCodeAt(j) === 35) {
      // "#"
      rowTrees[i]++;
      colTrees[j]++;
    }
  }
}
const q = Number(tokens[2 + r]);
let pos = 3 + r;
const out: number[] = [];
for (let k = 0; k < q; k++) {
  const index = Number(tokens[pos + 1]);
  out.push(tokens[pos] === "row" ? rowTrees[index] : colTrees[index]);
  pos += 2;
}
process.stdout.write(out.join("\n") + "\n");
