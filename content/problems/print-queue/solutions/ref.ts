import { readFileSync } from "node:fs";

const tokens = readFileSync(0, "utf8").split(/\s+/).filter((t) => t.length > 0);
const n = Number(tokens[0]);
const arrival = new Float64Array(n);
const priority = new Float64Array(n);
const pages = new Float64Array(n);
for (let i = 0; i < n; i++) {
  arrival[i] = Number(tokens[1 + 3 * i]);
  priority[i] = Number(tokens[2 + 3 * i]);
  pages[i] = Number(tokens[3 + 3 * i]);
}
const byArrival = Array.from({ length: n }, (_, i) => i).sort((x, y) => arrival[x] - arrival[y] || x - y);

/** The printer's rule: does job x go before job y? */
const before = (x: number, y: number): boolean =>
  priority[x] !== priority[y] ? priority[x] > priority[y] : arrival[x] !== arrival[y] ? arrival[x] < arrival[y] : x < y;

// A binary heap of waiting jobs, the next one to print on top.
const heap: number[] = [];
function push(x: number): void {
  heap.push(x);
  let i = heap.length - 1;
  while (i > 0) {
    const parent = (i - 1) >> 1;
    if (!before(heap[i], heap[parent])) break;
    [heap[i], heap[parent]] = [heap[parent], heap[i]];
    i = parent;
  }
}
function pop(): number {
  const top = heap[0];
  const last = heap.pop()!;
  if (heap.length > 0) {
    heap[0] = last;
    let i = 0;
    for (;;) {
      const l = 2 * i + 1;
      const r = l + 1;
      let m = i;
      if (l < heap.length && before(heap[l], heap[m])) m = l;
      if (r < heap.length && before(heap[r], heap[m])) m = r;
      if (m === i) break;
      [heap[i], heap[m]] = [heap[m], heap[i]];
      i = m;
    }
  }
  return top;
}

const order: number[] = [];
let t = 0;
let next = 0;
while (order.length < n) {
  if (heap.length === 0 && arrival[byArrival[next]] > t) t = arrival[byArrival[next]]; // wait
  while (next < n && arrival[byArrival[next]] <= t) push(byArrival[next++]);
  const job = pop();
  order.push(job + 1);
  t += pages[job];
}
process.stdout.write(order.join(" ") + "\n");
