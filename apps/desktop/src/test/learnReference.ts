// What each Learn topic's samples must print for an input (ADR 0029), written
// independently of the tracers: Vitest checks every tracer's output (and so cases.json)
// against these, and kb-pack checks the samples against cases.json in the sandbox.
const sorted = (xs: readonly number[]) => [...xs].sort((a, b) => a - b);

const log2 = (n: number) => Math.floor(Math.log2(n)) + 1;
const sum = (xs: readonly number[]) => xs.reduce((a, b) => a + b, 0);

/** Hanoi by the iterative rule (not the recursion): move m goes from peg (m & (m - 1)) % 3
 * to ((m | (m - 1)) + 1) % 3, which ends on the third peg for odd n and the second for even
 * n; relabel so the tower always ends on peg 3. */
function hanoiMoves(n: number): number[] {
  const label = n % 2 === 1 ? [1, 2, 3] : [1, 3, 2];
  const out: number[] = [];
  for (let m = 1; m < 2 ** n; m++) out.push(label[(m & (m - 1)) % 3]!, label[((m | (m - 1)) + 1) % 3]!);
  return out;
}


/** Stored operations as [code, args…] with a fixed width (the topics' own encoding). */
function ops(input: readonly number[], width: number, header = 0): number[][] {
  const out: number[][] = [];
  for (let k = header; k < input.length; k += width + 1) out.push(input.slice(k, k + width + 1));
  return out;
}


/** Letters 0-25 separated by -1, as words. */
function words(input: readonly number[]): string[] {
  return input.reduce<string[]>((w, v) => (v === -1 ? [...w, ""] : [...w.slice(0, -1), w[w.length - 1]! + String.fromCharCode(97 + v)]), [""]);
}

/** A BST's root as nested objects (smaller left, others right), for traversals. */
type T = { v: number; l?: T; r?: T };
function bstOf(values: readonly number[]): T | undefined {
  let root: T | undefined;
  for (const v of values) {
    const node: T = { v };
    if (!root) root = node;
    else {
      let cur = root;
      for (;;) {
        if (v < cur.v) {
          if (!cur.l) { cur.l = node; break; }
          cur = cur.l;
        } else {
          if (!cur.r) { cur.r = node; break; }
          cur = cur.r;
        }
      }
    }
  }
  return root;
}

export const REFERENCE: Record<string, (input: readonly number[]) => number[]> = {
  "big-o-growth": ([n]) => [1, log2(n!), n!, n! * log2(n!), n! * n!, 2 ** n!],
  "recursion-call-stack": (xs) => [sum(xs)],
  "towers-of-hanoi": ([n]) => hanoiMoves(n!),
  "bit-manipulation": ([x, y]) => [x! & y!, x! | y!, x! ^ y!, x! * 2, Math.floor(x! / 2), [...x!.toString(2)].filter((c) => c === "1").length],
  "prefix-sums": (xs) => {
    const p = xs.map((_, k) => sum(xs.slice(0, k)));
    p.push(sum(xs));
    const lo = Math.floor(xs.length / 4);
    const hi = Math.floor((3 * xs.length) / 4);
    return [...p, sum(xs.slice(lo, hi + 1))];
  },
  "dynamic-array": (xs) => {
    const cap = 2 ** Math.ceil(Math.log2(xs.length));
    return [cap, cap - 1, ...xs];
  },
  // Linear structures: plain array simulations.
  "dynamic-array-ops": (input) => {
    const a: number[] = [];
    for (const [op, x, y] of ops(input, 2)) {
      if (op === 1) a.push(x!);
      else if (op === 2) a.splice(Math.min(x!, a.length), 0, y!);
      else if (a.length > 0) a.splice(Math.min(x!, a.length - 1), 1);
    }
    return a;
  },
  stack: (input) => {
    const a: number[] = [];
    const out: number[] = [];
    for (const [op, x] of ops(input, 1)) {
      if (op === 1) a.push(x!);
      else if (op === 2) out.push(a.length ? a.pop()! : -1);
      else out.push(a.length ? a[a.length - 1]! : -1);
    }
    return out;
  },
  "queue-ring-buffer": (input) => {
    const cap = input[0]!;
    const q: number[] = [];
    const out: number[] = [];
    for (const [op, x] of ops(input, 1, 1)) {
      if (op === 1) {
        if (q.length < cap) q.push(x!);
      } else out.push(q.length ? q.shift()! : -1);
    }
    return out;
  },
  "deque-monotonic-queue": ([k, ...a]) => {
    const out: number[] = [];
    for (let hi = k! - 1; hi < a.length; hi++) out.push(Math.max(...a.slice(hi - k! + 1, hi + 1)));
    return out;
  },
  "singly-linked-list": (input) => {
    const a: number[] = [];
    for (const [op, x] of ops(input, 1)) {
      if (op === 1) a.unshift(x!);
      else if (op === 2) a.push(x!);
      else if (a.includes(x!)) a.splice(a.indexOf(x!), 1);
    }
    return a;
  },
  "doubly-linked-list": (input) => {
    const a: number[] = [];
    const out: number[] = [];
    for (const [op, x] of ops(input, 1)) {
      if (op === 1) a.unshift(x!);
      else if (op === 2) a.push(x!);
      else if (op === 3) out.push(a.length ? a.shift()! : -1);
      else out.push(a.length ? a.pop()! : -1);
    }
    return [...out, 0, ...a, 0, ...[...a].reverse()];
  },
  "reverse-list": (a) => [...a].reverse(),
  "floyd-cycle": ([pos]) => [pos!],
  "lru-cache": (input) => {
    const cap = input[0]!;
    const order: number[] = []; // keys, least recent first
    const value = new Map<number, number>();
    const out: number[] = [];
    for (const [op, key, v] of ops(input, 2, 1)) {
      const has = order.includes(key!);
      if (has) order.splice(order.indexOf(key!), 1);
      if (op === 1) {
        if (has) order.push(key!);
        out.push(has ? value.get(key!)! : -1);
      } else {
        order.push(key!);
        value.set(key!, v!);
        if (order.length > cap) value.delete(order.shift()!);
      }
    }
    return out;
  },
  // Hashing and trees.
  "separate-chaining": (keys) => {
    const buckets: number[][] = Array.from({ length: 7 }, () => []);
    for (const k of keys) if (!buckets[k % 7]!.includes(k)) buckets[k % 7]!.push(k);
    return buckets.flatMap((b) => [b.length, ...b]);
  },
  "open-addressing": (input) => {
    const set = new Set<number>();
    const out: number[] = [];
    for (const [op, k] of ops(input, 1)) {
      if (op === 1) set.add(k!);
      else if (op === 2) out.push(set.has(k!) ? 1 : 0);
      else set.delete(k!);
    }
    return out;
  },
  "bloom-filter": (input) => {
    const bits = new Set<number>();
    const pos = (k: number) => [k % 16, (7 * k + 3) % 16, (11 * k + 5) % 16];
    const out: number[] = [];
    for (const [op, k] of ops(input, 1)) {
      if (op === 1) pos(k!).forEach((p) => bits.add(p));
      else out.push(pos(k!).every((p) => bits.has(p)) ? 1 : 0);
    }
    return out;
  },
  "rolling-hash": ([len, ...rest]) => {
    // Every window hashed from scratch.
    const codes = rest;
    const out: number[] = [];
    for (let i = 0; i + len! <= codes.length; i++) {
      let h = 0;
      for (const c of codes.slice(i, i + len!)) h = (h * 26 + c) % 101;
      out.push(h);
    }
    return out;
  },
  "tree-traversals": (values) => {
    const pre: number[] = [];
    const ino: number[] = [];
    const post: number[] = [];
    const walk = (t?: T) => {
      if (!t) return;
      pre.push(t.v);
      walk(t.l);
      ino.push(t.v);
      walk(t.r);
      post.push(t.v);
    };
    walk(bstOf(values));
    return [...pre, ...ino, ...post];
  },
  "level-order": (values) => {
    const out: number[] = [];
    const levels: T[][] = [[bstOf(values)!]];
    while (levels[levels.length - 1]!.length) {
      const level = levels[levels.length - 1]!;
      out.push(...level.map((t) => t.v));
      levels.push(level.flatMap((t) => [t.l, t.r].filter((c): c is T => !!c)));
    }
    return out;
  },
  "bst-operations": (input) => {
    const multiset: number[] = [];
    const out: number[] = [];
    for (const [op, k] of ops(input, 1)) {
      if (op === 1) multiset.push(k!);
      else if (op === 2) out.push(multiset.includes(k!) ? 1 : 0);
      else if (multiset.includes(k!)) multiset.splice(multiset.indexOf(k!), 1);
    }
    return [...out, 0, ...sorted(multiset)];
  },
  "binary-heap": (input) => {
    const bag: number[] = [];
    const out: number[] = [];
    for (const [op, k] of ops(input, 1)) {
      if (op === 1) bag.push(k!);
      else if (!bag.length) out.push(-1);
      else {
        const m = Math.min(...bag);
        bag.splice(bag.indexOf(m), 1);
        out.push(m);
      }
    }
    return out;
  },
  trie: (input) => {
    const all = words(input);
    const prefix = all[all.length - 1]!;
    const ws = all.slice(0, -1);
    return [ws.filter((w) => w.startsWith(prefix)).length, ws.includes(prefix) ? 1 : 0];
  },
  "union-find": (input) => {
    // A plain union by size with full path compression, written separately.
    const n = input[0]!;
    const parent = Array.from({ length: n }, (_, k) => k);
    const size = new Array<number>(n).fill(1);
    const root = (x: number): number => (parent[x] === x ? x : (parent[x] = root(parent[x]!)));
    const out: number[] = [];
    for (const [op, a, b] of ops(input, 2, 1)) {
      if (op === 2) out.push(root(a!));
      else {
        let ra = root(a!);
        let rb = root(b!);
        if (ra === rb) continue;
        if (size[ra]! < size[rb]!) [ra, rb] = [rb, ra];
        parent[rb] = ra;
        size[ra]! += size[rb]!;
      }
    }
    return [...out, parent.filter((p, k) => p === k).length];
  },
  "tree-diameter": (parents) => {
    // The longest of all shortest distances, by BFS from every node.
    const n = parents.length + 1;
    const adj: number[][] = Array.from({ length: n }, () => []);
    parents.forEach((p, k) => {
      adj[p]!.push(k + 1);
      adj[k + 1]!.push(p);
    });
    let best = 0;
    for (let s = 0; s < n; s++) {
      const dist = new Array<number>(n).fill(-1);
      dist[s] = 0;
      const q = [s];
      while (q.length) {
        const u = q.shift()!;
        for (const v of adj[u]!) if (dist[v] === -1) { dist[v] = dist[u]! + 1; q.push(v); }
      }
      best = Math.max(best, ...dist);
    }
    return [best];
  },
  "bubble-sort": sorted,
  "insertion-sort": sorted,
  "quick-sort": sorted,
  "selection-sort": sorted,
  "shell-sort": sorted,
  "merge-sort": sorted,
  "quick-sort-3way": sorted,
  "heap-sort": sorted,
  "counting-sort": sorted,
  "radix-sort": sorted,
  "bucket-sort": sorted,
  "tim-sort": sorted,
  quickselect: ([k, ...values]) => [sorted(values)[k! - 1]!],
  "linear-search": ([t, ...a]) => [a.indexOf(t!)],
  "binary-search": ([t, ...a]) => {
    let lo = 0;
    let hi = a.length - 1;
    while (lo <= hi) {
      const m = (lo + hi) >> 1;
      if (a[m] === t) return [m];
      if (a[m]! < t!) lo = m + 1;
      else hi = m - 1;
    }
    return [-1];
  },
  "lower-upper-bound": ([t, ...a]) => [a.filter((v) => v < t!).length, a.filter((v) => v <= t!).length],
  "binary-search-answer": ([days, ...w]) => {
    // The first capacity that fits, by trying every one in turn.
    const need = (c: number) => {
      let d = 1;
      let load = 0;
      for (const x of w) {
        if (load + x > c) {
          d++;
          load = 0;
        }
        load += x;
      }
      return d;
    };
    for (let c = Math.max(...w); ; c++) if (need(c) <= days!) return [c];
  },
  "exponential-search": ([t, ...a]) => {
    if (a[0] === t) return [0];
    let bound = 1;
    while (bound < a.length && a[bound]! < t!) bound *= 2;
    for (let lo = bound >> 1, hi = Math.min(bound, a.length - 1); lo <= hi; ) {
      const m = (lo + hi) >> 1;
      if (a[m] === t) return [m];
      if (a[m]! < t!) lo = m + 1;
      else hi = m - 1;
    }
    return [-1];
  },
  "two-pointers": ([t, ...a]) => {
    for (let i = 0, j = a.length - 1; i < j; ) {
      const s = a[i]! + a[j]!;
      if (s === t) return [i, j];
      if (s < t!) i++;
      else j--;
    }
    return [-1, -1];
  },
  "sliding-window": ([limit, ...a]) => {
    // Every window, by brute force: for each end, the longest window that fits; ties keep
    // the earlier window, as the sample does (it only takes strictly longer ones).
    let best = 0;
    let start = 0;
    for (let hi = 0; hi < a.length; hi++) {
      for (let lo = 0; lo <= hi; lo++) {
        const sum = a.slice(lo, hi + 1).reduce((x, y) => x + y, 0);
        if (sum <= limit!) {
          if (hi - lo + 1 > best) {
            best = hi - lo + 1;
            start = lo;
          }
          break;
        }
      }
    }
    return [best, start];
  },
  "monotonic-stack": (a) => a.map((v, i) => a.slice(i + 1).find((x) => x > v) ?? -1),
};

/** Hand-checked golden runs: the input, then the marker behind each step and its line in
 * sample.py / sample.ts, worked out by hand from the samples. */
export type Golden = {
  input: number[];
  markers: (string | null)[];
  py: (number | null)[];
  ts: (number | null)[];
};

export const GOLDEN: Record<string, Golden> = {
  "separate-chaining": {
    input: [3, 10, 3],
    markers: [null, "hash", "scan", "insert", "hash", "scan", "insert", "hash", "scan", null],
    py: [null, 7, 8, 10, 7, 8, 10, 7, 8, null],
    ts: [null, 6, 7, 10, 6, 7, 10, 6, 7, null],
  },
  "open-addressing": {
    input: [1, 3, 1, 11, 2, 11, 3, 3, 2, 11],
    markers: [null, "hash", "place", "hash", "probe", "place", "hash", "probe", "found", "hash", "tombstone", "hash", "probe", "found", null],
    py: [null, 10, 17, 10, 12, 17, 10, 12, 20, 10, 25, 10, 12, 20, null],
    ts: [null, 9, 18, 9, 11, 18, 9, 11, 22, 9, 28, 9, 11, 22, null],
  },
  "bloom-filter": {
    input: [1, 1, 2, 1, 2, 2],
    markers: [null, "set", "set", "set", "check", "check", "check", "answer", "check", "answer", null],
    py: [null, 14, 14, 14, 18, 18, 18, 21, 18, 21, null],
    ts: [null, 18, 18, 18, 23, 23, 23, 28, 23, 28, null],
  },
  "rolling-hash": {
    input: [2, 0, 1, 2],
    markers: [null, "add", "add", "remove", "shift-in", null],
    py: [null, 10, 10, 13, 14, null],
    ts: [null, 9, 9, 13, 14, null],
  },
  "tree-traversals": {
    input: [2, 1, 3],
    markers: [null, "pre", "pre", "pre", "in", "in", "in", "post", "post", "post", null],
    py: [null, 20, 20, 20, 28, 28, 28, 36, 36, 36, null],
    ts: [null, 20, 20, 20, 29, 29, 29, 38, 38, 38, null],
  },
  "level-order": {
    input: [2, 1, 3],
    markers: [null, "visit", "enqueue", "enqueue", "visit", "visit", null],
    py: [null, 24, 28, 28, 24, 24, null],
    ts: [null, 22, 26, 26, 22, 22, null],
  },
  "bst-operations": {
    input: [1, 5, 1, 3, 1, 8, 2, 3, 3, 5],
    markers: [null, "root", "descend", "attach-left", "descend", "attach-right", "search", "result", "visit", "copy", "visit", "splice", null],
    py: [null, 19, 16, 21, 16, 23, 28, 29, 37, 47, 37, 42, null],
    ts: [null, 23, 19, 25, 19, 27, 34, 36, 47, 58, 47, 52, null],
  },
  "binary-heap": {
    input: [1, 5, 1, 3, 2, 0, 2, 0, 2, 0],
    markers: [null, "append", "append", "compare-up", "swap-up", "take", "move-last", "take", "empty", null],
    py: [null, 2, 2, 6, 8, 15, 18, 15, 14, null],
    ts: [null, 2, 2, 6, 9, 18, 21, 18, 16, null],
  },
  trie: {
    input: [0, 1, -1, 0],
    markers: [null, "new-node", "follow", "new-node", "follow", "mark-end", "walk", "count"],
    py: [null, 12, 13, 12, 13, 15, 23, 24],
    ts: [null, 11, 13, 11, 13, 16, 25, 27],
  },
  "union-find": {
    input: [3, 1, 0, 1, 2, 1, 0, 1, 1, 0],
    markers: [null, "link", "climb", null, "climb", "same", null],
    py: [null, 20, 9, null, 9, 17, null],
    ts: [null, 28, 13, null, 13, 25, null],
  },
  "tree-diameter": {
    input: [0, 1],
    markers: [null, "first-bfs", "visit", "reach", "visit", "reach", "visit", "second-bfs", "visit", "reach", "visit", "reach", "visit", null],
    py: [null, 25, 9, 14, 9, 14, 9, 26, 9, 14, 9, 14, 9, null],
    ts: [null, 26, 7, 11, 7, 11, 7, 27, 7, 11, 7, 11, 7, null],
  },
  stack: {
    input: [1, 5, 2, 0, 3, 0],
    markers: [null, "push", "pop", "peek", null],
    py: [null, 5, 7, 9, null],
    ts: [null, 6, 8, 10, null],
  },
  "queue-ring-buffer": {
    input: [2, 1, 4, 1, 6, 1, 8, 2, 0, 2, 0, 2, 0],
    markers: [null, "enqueue", "enqueue", "full", "dequeue", "dequeue", "empty", null],
    py: [null, 11, 11, 9, 18, 18, 17, null],
    ts: [null, 15, 15, 12, 24, 24, 22, null],
  },
  "deque-monotonic-queue": {
    input: [2, 3, 1, 4],
    markers: [null, "push-back", "push-back", "emit", "pop-back", "pop-back", "push-back", "emit", null],
    py: [null, 9, 9, 13, 8, 8, 9, 13, null],
    ts: [null, 8, 8, 13, 6, 6, 8, 13, null],
  },
  "singly-linked-list": {
    input: [1, 5, 2, 9, 3, 9],
    markers: [null, "link", "new-head", "append", "search", "unlink", null],
    py: [null, 12, 13, 22, 26, 31, null],
    ts: [null, 15, 16, 26, 32, 38, null],
  },
  "doubly-linked-list": {
    input: [1, 5, 2, 9, 4, 0, 3, 0, 3, 0],
    markers: [null, "link-front", "link-back", "unlink-back", "unlink-front", "empty-front", null],
    py: [null, 15, 24, 46, 35, 33, null],
    ts: [null, 17, 25, 47, 36, 33, null],
  },
  "reverse-list": {
    input: [4, 7],
    markers: [null, "save", "flip", "advance", "save", "flip", "advance", "new-head"],
    py: [null, 19, 20, 21, 19, 20, 21, 22],
    ts: [null, 24, 25, 26, 24, 25, 26, 28],
  },
  "floyd-cycle": {
    input: [0, 5, 9],
    markers: [null, "move", "move", "meet", "reset", "found"],
    py: [null, 20, 20, 22, 25, 29],
    ts: [null, 22, 22, 24, 30, 35],
  },
  "lru-cache": {
    input: [1, 2, 1, 10, 2, 2, 20, 1, 1, 0, 1, 2, 0],
    markers: [null, "put", "put", "evict", "miss", "hit", null],
    py: [null, 18, 18, 20, 11, 12, null],
    ts: [null, 23, 23, 25, 12, 14, null],
  },
  "dynamic-array-ops": {
    input: [1, 7, 0, 2, 0, 3, 2, 1, 5, 3, 0, 0],
    markers: [null, "write", "shift-right", "write", "grow", "shift-right", "write", "shift-left", "shift-left", "shrink", null],
    py: [null, 11, 10, 11, 8, 10, 11, 16, 16, 17, null],
    ts: [null, 12, 10, 12, 7, 10, 12, 18, 18, 20, null],
  },
  "linear-search": {
    input: [5, 3, 5, 1],
    markers: [null, "check", "check", "found"],
    py: [null, 3, 3, 4],
    ts: [null, 3, 3, 4],
  },
  "binary-search": {
    input: [5, 1, 3, 5, 7],
    markers: [null, "mid", "compare", "mid", "found"],
    py: [null, 4, 7, 4, 6],
    ts: [null, 5, 9, 5, 7],
  },
  "lower-upper-bound": {
    input: [3, 1, 3, 3, 5],
    markers: [null, "lower", "lower", "lower", "lower-done", "upper", "upper", "upper-done", null],
    py: [null, 5, 5, 5, 9, 16, 16, 20, null],
    ts: [null, 6, 6, 6, 12, 20, 20, 26, null],
  },
  "binary-search-answer": {
    input: [2, 3, 1, 2],
    markers: [null, "try", "check", "try", "check", "answer"],
    py: [null, 14, 15, 14, 15, 19],
    ts: [null, 18, 19, 18, 19, 25],
  },
  "exponential-search": {
    input: [5, 1, 3, 5, 7, 9],
    markers: [null, "first", "double", "double", "range", "mid", "compare", "mid", "found"],
    py: [null, 2, 5, 5, 7, 9, 12, 9, 11],
    ts: [null, 2, 4, 4, 7, 10, 14, 10, 12],
  },
  "two-pointers": {
    input: [10, 1, 3, 5, 7],
    markers: [null, "sum", "move", "sum", "found"],
    py: [null, 4, 7, 4, 6],
    ts: [null, 5, 9, 5, 7],
  },
  "sliding-window": {
    input: [6, 4, 3, 2],
    markers: [null, "grow", "check", "grow", "shrink", "check", "grow", "check", null],
    py: [null, 5, 9, 5, 7, 9, 5, 9, null],
    ts: [null, 7, 12, 7, 9, 12, 7, 12, null],
  },
  "monotonic-stack": {
    input: [2, 1, 3],
    markers: [null, "push", "compare", "push", "compare", "pop", "compare", "pop", "push", null],
    py: [null, 7, 5, 7, 5, 6, 5, 6, 7, null],
    ts: [null, 8, 5, 8, 5, 6, 5, 6, 8, null],
  },
  "selection-sort": {
    input: [3, 1, 2],
    markers: [null, "start", "compare", "new-min", "compare", "swap", "start", "compare", "new-min", "swap", null],
    py: [null, 4, 6, 7, 6, 8, 4, 6, 7, 8, null],
    ts: [null, 4, 6, 7, 6, 10, 4, 6, 7, 10, null],
  },
  "shell-sort": {
    input: [3, 1, 2],
    markers: [null, "gap", "take", "compare", "shift", "insert", "take", "compare", "shift", "compare", "insert", null],
    py: [null, 3, 5, 7, 8, 10, 5, 7, 8, 7, 10, null],
    ts: [null, 3, 5, 7, 8, 11, 5, 7, 8, 7, 11, null],
  },
  "merge-sort": {
    input: [3, 1, 2],
    markers: [null, "split", "split", "base", "base", "copy", "pick", "take-right", "pick", "take-left", "base", "copy", "pick", "take-left", "pick", "take-right", "pick", "take-left", null],
    py: [null, 4, 4, 2, 2, 11, 14, 18, 14, 15, 2, 11, 14, 15, 14, 18, 14, 15, null],
    ts: [null, 3, 3, 2, 2, 10, 14, 18, 14, 15, 2, 10, 14, 15, 14, 18, 14, 15, null],
  },
  "quick-sort-3way": {
    input: [3, 1, 2],
    markers: [null, "pivot", "compare", "less", "compare", "less", "pivot", "compare", "greater", "base", null],
    py: [null, 4, 7, 8, 7, 8, 4, 7, 12, 2, null],
    ts: [null, 3, 8, 9, 8, 9, 3, 8, 13, 2, null],
  },
  "heap-sort": {
    input: [3, 1, 2],
    markers: [null, "build", "pick", "compare", null, "extract", "pick", "compare", "extract", null],
    py: [null, 4, 13, 15, null, 6, 13, 15, 6, null],
    ts: [null, 4, 15, 18, null, 7, 15, 18, 7, null],
  },
  "counting-sort": {
    input: [3, 1, 2],
    markers: [null, "count", "count", "count", "write", "write", "write", null],
    py: [null, 4, 4, 4, 8, 8, 8, null],
    ts: [null, 4, 4, 4, 9, 9, 9, null],
  },
  "radix-sort": {
    input: [3, 1, 2],
    markers: [null, "pass", "distribute", "distribute", "distribute", "collect", "collect", "collect", null],
    py: [null, 3, 6, 6, 6, 10, 10, 10, null],
    ts: [null, 2, 5, 5, 5, 10, 10, 10, null],
  },
  "bucket-sort": {
    input: [3, 1, 2],
    markers: [null, "scatter", "scatter", "scatter", "sort", "gather", "gather", "gather", "sort", "sort", "sort", "sort", null],
    py: [null, 4, 4, 4, 7, 9, 9, 9, 7, 7, 7, 7, null],
    ts: [null, 4, 4, 4, 8, 10, 10, 10, 8, 8, 8, 8, null],
  },
  "tim-sort": {
    input: [3, 1, 2],
    markers: [null, "run", "take", "compare", "shift", "insert", "take", "compare", "shift", "compare", "insert", null],
    py: [null, 7, 19, 21, 22, 24, 19, 21, 22, 21, 24, null],
    ts: [null, 6, 19, 21, 22, 25, 19, 21, 22, 21, 25, null],
  },
  quickselect: {
    input: [1, 3, 1, 2],
    markers: [null, "pivot", "compare", "compare", "swap", "place", "side", "pivot", "place", "found", null],
    py: [null, 14, 17, 17, 18, 20, 7, 14, 20, 5, null],
    ts: [null, 16, 19, 19, 20, 24, 7, 16, 24, 6, null],
  },
  "big-o-growth": {
    input: [2],
    markers: [null, "constant", "log", "log", "log", "linear", "nlogn", "quadratic", "exponential", null],
    py: [null, 2, 10, 10, 10, 18, 27, 36, 43, null],
    ts: [null, 2, 9, 9, 9, 17, 26, 36, 45, null],
  },
  "recursion-call-stack": {
    input: [5, 7, 2],
    markers: [null, "check", "split", "check", "split", "check", "leaf", "check", "leaf", "combine", "check", "leaf", "combine", null],
    py: [null, 2, 4, 2, 4, 2, 3, 2, 3, 7, 2, 3, 7, null],
    ts: [null, 2, 5, 2, 5, 2, 3, 2, 3, 8, 2, 3, 8, null],
  },
  "towers-of-hanoi": {
    input: [1],
    markers: [null, "check", "check", "move", "check", null],
    py: [null, 2, 2, 5, 2, null],
    ts: [null, 2, 2, 4, 2, null],
  },
  "bit-manipulation": {
    input: [6, 3],
    markers: [null, "and", "or", "xor", "left", "right", "clear", "clear", "count", null],
    py: [null, 2, 3, 4, 5, 6, 9, 9, 11, null],
    ts: [null, 2, 3, 4, 5, 6, 9, 9, 12, null],
  },
  "prefix-sums": {
    input: [4, 1, 3],
    markers: [null, "add", "add", "add", "query", null],
    py: [null, 4, 4, 4, 9, null],
    ts: [null, 4, 4, 4, 10, null],
  },
  "dynamic-array": {
    input: [8, 5, 9],
    markers: [null, "check", "write", "check", "copy", "write", "check", "copy", "copy", "write", null],
    py: [null, 7, 14, 7, 11, 14, 7, 11, 11, 14, null],
    ts: [null, 7, 16, 7, 11, 16, 7, 11, 11, 16, null],
  },
  "quick-sort": {
    input: [3, 1, 2],
    markers: [null, "pivot", "compare", "compare", "swap", "place", "base", "base", null],
    py: [null, 10, 13, 13, 14, 16, 2, 2, null],
    ts: [null, 9, 12, 12, 13, 17, 2, 2, null],
  },
  "bubble-sort": {
    input: [3, 1, 2],
    markers: [null, "pass", "compare", "swap", "compare", "swap", "check", "pass", "compare", "check", null],
    py: [null, 3, 6, 7, 6, 7, 9, 3, 6, 9, null],
    ts: [null, 3, 6, 7, 6, 7, 11, 3, 6, 11, null],
  },
  "insertion-sort": {
    input: [3, 1, 2],
    markers: [null, "take", "compare", "shift", "insert", "take", "compare", "shift", "compare", "insert", null],
    py: [null, 3, 5, 6, 8, 3, 5, 6, 5, 8, null],
    ts: [null, 3, 5, 6, 9, 3, 5, 6, 5, 9, null],
  },
};

/** Inputs that make each topic work hardest (the step cap must hold with room). */
export const WORST: Record<string, number[]> = {
  "big-o-growth": [16],
  "towers-of-hanoi": [5],
};
