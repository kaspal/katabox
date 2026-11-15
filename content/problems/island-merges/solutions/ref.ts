import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const r = Number(tokens[0]);
const c = Number(tokens[1]);
const q = Number(tokens[2]);
const width = c + 2; // a border of sea around the map: no bounds checks
const parent = new Int32Array((r + 2) * width).fill(-1); // -1: sea; otherwise union-find over land

const root = (x: number): number => {
  while (parent[x] !== x) {
    parent[x] = parent[parent[x]];
    x = parent[x];
  }
  return x;
};

let islands = 0;
const out: number[] = [];
for (let k = 0; k < q; k++) {
  const cell = Number(tokens[3 + 2 * k]) * width + Number(tokens[4 + 2 * k]);
  if (parent[cell] === -1) {
    // A new island, then one fewer for every different island it touches.
    parent[cell] = cell;
    islands++;
    for (const next of [cell + 1, cell - 1, cell + width, cell - width]) {
      if (parent[next] !== -1) {
        const a = root(cell);
        const b = root(next);
        if (a !== b) {
          parent[b] = a;
          islands--;
        }
      }
    }
  }
  out.push(islands);
}
process.stdout.write(out.join("\n") + "\n");
