import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const n = Number(tokens[0]);
const seen = new Set<number>();
const distinct = new Array<number>(n + 1).fill(0);
for (let p = 1; p <= n; p++) {
  seen.add(Number(tokens[p]));
  distinct[p] = seen.size;
}
const q = Number(tokens[1 + n]);
const out: string[] = [];
for (let j = 0; j < q; j++) {
  out.push(String(distinct[Number(tokens[2 + n + j])]));
}
process.stdout.write(out.join("\n") + "\n");
