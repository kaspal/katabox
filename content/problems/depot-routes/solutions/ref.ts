import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;
const next = (): number => Number(tokens[pos++]);

const n = next();
const m = next();
// Adjacency as linked lists in flat arrays: road end e goes to `to[e]`, costs `cost[e]`,
// and `link[e]` is the next road end of the same town (-1 ends the list).
const head = new Int32Array(n + 1).fill(-1);
const to = new Int32Array(2 * m);
const cost = new Int32Array(2 * m);
const link = new Int32Array(2 * m);
for (let i = 0; i < m; i++) {
  const a = next();
  const b = next();
  const w = next();
  to[2 * i] = b;
  cost[2 * i] = w;
  link[2 * i] = head[a];
  head[a] = 2 * i;
  to[2 * i + 1] = a;
  cost[2 * i + 1] = w;
  link[2 * i + 1] = head[b];
  head[b] = 2 * i + 1;
}

// Binary min-heap of (time, town) pairs in two parallel arrays.
const keys: number[] = [];
const towns: number[] = [];
function push(key: number, town: number): void {
  let i = keys.length;
  keys.push(key);
  towns.push(town);
  while (i > 0) {
    const p = (i - 1) >> 1;
    if (keys[p] <= key) break;
    keys[i] = keys[p];
    towns[i] = towns[p];
    i = p;
  }
  keys[i] = key;
  towns[i] = town;
}
function pop(): [number, number] {
  const topKey = keys[0];
  const topTown = towns[0];
  const lastKey = keys.pop()!;
  const lastTown = towns.pop()!;
  const size = keys.length;
  if (size > 0) {
    let i = 0;
    for (;;) {
      let c = 2 * i + 1;
      if (c >= size) break;
      if (c + 1 < size && keys[c + 1] < keys[c]) c++;
      if (keys[c] >= lastKey) break;
      keys[i] = keys[c];
      towns[i] = towns[c];
      i = c;
    }
    keys[i] = lastKey;
    towns[i] = lastTown;
  }
  return [topKey, topTown];
}

const dist = new Float64Array(n + 1).fill(Infinity);
dist[1] = 0;
push(0, 1);
while (keys.length > 0) {
  const [d, u] = pop();
  if (d > dist[u]) continue; // a stale entry
  for (let e = head[u]; e !== -1; e = link[e]) {
    const v = to[e];
    const nd = d + cost[e];
    if (nd < dist[v]) {
      dist[v] = nd;
      push(nd, v);
    }
  }
}

const q = next();
const out: string[] = [];
for (let i = 0; i < q; i++) {
  const d = dist[next()];
  out.push(d === Infinity ? "-1" : String(d));
}
process.stdout.write(out.join("\n") + "\n");
