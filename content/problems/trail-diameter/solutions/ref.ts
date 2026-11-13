import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
let pos = 0;
const next = (): number => Number(tokens[pos++]);

// Iterative BFS over a compact adjacency (neighbours of v in nbr[start[v] .. start[v+1]]).
// Returns [the last junction reached, its distance]: a farthest one.
function farthest(n: number, start: Int32Array, nbr: Int32Array, from: number): [number, number] {
  const dist = new Int32Array(n + 1).fill(-1);
  const queue = new Int32Array(n);
  let head = 0;
  let tail = 0;
  dist[from] = 0;
  queue[tail++] = from;
  while (head < tail) {
    const v = queue[head++];
    for (let e = start[v]; e < start[v + 1]; e++) {
      const w = nbr[e];
      if (dist[w] < 0) {
        dist[w] = dist[v] + 1;
        queue[tail++] = w;
      }
    }
  }
  const last = queue[tail - 1];
  return [last, dist[last]];
}

const t = next();
const out: number[] = [];
for (let c = 0; c < t; c++) {
  const n = next();
  const ea = new Int32Array(n - 1);
  const eb = new Int32Array(n - 1);
  const degree = new Int32Array(n + 2);
  for (let i = 0; i < n - 1; i++) {
    ea[i] = next();
    eb[i] = next();
    degree[ea[i]]++;
    degree[eb[i]]++;
  }
  const start = new Int32Array(n + 2);
  for (let v = 1; v <= n + 1; v++) start[v] = start[v - 1] + degree[v - 1];
  const fill = start.slice();
  const nbr = new Int32Array(2 * (n - 1));
  for (let i = 0; i < n - 1; i++) {
    nbr[fill[ea[i]]++] = eb[i];
    nbr[fill[eb[i]]++] = ea[i];
  }
  const [u] = farthest(n, start, nbr, 1);
  out.push(farthest(n, start, nbr, u)[1]);
}
process.stdout.write(out.join("\n") + "\n");
