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


/** An AVL tree built immutably (new nodes on the way back up), for the AVL topic. */
type AvlT = { v: number; l: AvlT | null; r: AvlT | null; h: number } | null;
function avlInsert(t: AvlT, v: number): AvlT {
  const h = (x: AvlT) => x?.h ?? 0;
  const mk = (val: number, l: AvlT, r: AvlT): AvlT => ({ v: val, l, r, h: 1 + Math.max(h(l), h(r)) });
  if (!t) return mk(v, null, null);
  const n = v < t.v ? mk(t.v, avlInsert(t.l, v), t.r) : mk(t.v, t.l, avlInsert(t.r, v));
  const bal = h(n!.l) - h(n!.r);
  const rotR = (x: NonNullable<AvlT>) => mk(x.l!.v, x.l!.l, mk(x.v, x.l!.r, x.r));
  const rotL = (x: NonNullable<AvlT>) => mk(x.r!.v, mk(x.v, x.l, x.r!.l), x.r!.r);
  if (bal > 1) return rotR(v >= n!.l!.v ? mk(n!.v, rotL(n!.l!), n!.r)! : n!);
  if (bal < -1) return rotL(v < n!.r!.v ? mk(n!.v, n!.l, rotR(n!.r!))! : n!);
  return n;
}

/** A 2-3 tree (keys ≤ 2 per node; equal keys go right). Left-leaning red-black trees
 * are 2-3 trees in disguise: a 3-node [a, b] is b with a red left child a. */
type T23 = { keys: number[]; kids: T23[] };
function insert23(t: T23, key: number): T23 | [T23, number, T23] {
  let keys = [...t.keys];
  let kids = [...t.kids];
  const i = keys.filter((k) => k <= key).length;
  if (!kids.length) keys = sorted([...keys, key]);
  else {
    const r = insert23(kids[i]!, key);
    if (Array.isArray(r)) {
      keys.splice(i, 0, r[1]);
      kids.splice(i, 1, r[0], r[2]);
    } else kids[i] = r;
  }
  if (keys.length < 3) return { keys, kids };
  return [{ keys: [keys[0]!], kids: kids.slice(0, 2) }, keys[1]!, { keys: [keys[2]!], kids: kids.slice(2) }];
}
function build23(values: readonly number[]): T23 {
  let root: T23 = { keys: [], kids: [] };
  for (const v of values) {
    const r = insert23(root, v);
    root = Array.isArray(r) ? { keys: [r[1]], kids: [r[0], r[2]] } : r;
  }
  return root;
}


/** A stored graph [n, directed, u, v, w, …], read independently of engine/graph.ts. */
function graphOf(input: readonly number[]) {
  const n = input[0]!;
  const directed = input[1] === 1;
  const edges: [number, number, number][] = [];
  for (let k = 2; k < input.length; k += 3) edges.push([input[k]!, input[k + 1]!, input[k + 2]!]);
  // Neighbours of x in kb_graph's order: scan the edges, both ends when undirected.
  const nbrs = (x: number) => {
    const out: number[] = [];
    for (const [u, v] of edges) {
      if (u === x) out.push(v);
      else if (!directed && v === x) out.push(u);
    }
    return out;
  };
  return { n, directed, edges, nbrs };
}

/** All-pairs shortest distances (Floyd-Warshall); null = unreachable. */
function allPairs(n: number, arcs: readonly [number, number, number][]): (number | null)[][] {
  const d: (number | null)[][] = Array.from({ length: n }, (_, i) => Array.from({ length: n }, (_, j) => (i === j ? 0 : null)));
  for (const [u, v, w] of arcs) if (d[u]![v] === null || w < d[u]![v]!) d[u]![v] = w;
  for (let k = 0; k < n; k++)
    for (let i = 0; i < n; i++)
      for (let j = 0; j < n; j++) {
        const a = d[i]![k];
        const b = d[k]![j];
        if (a !== null && b !== null && (d[i]![j] === null || a + b < d[i]![j]!)) d[i]![j] = a + b;
      }
  return d;
}
const arcsOf = (g: ReturnType<typeof graphOf>, unit = false) =>
  g.edges.flatMap(([u, v, w]) => {
    const c = unit ? 1 : w;
    return g.directed ? [[u, v, c] as [number, number, number]] : [[u, v, c] as [number, number, number], [v, u, c] as [number, number, number]];
  });


/** Pieces of an undirected graph, leaving out one node and/or one edge (brute force). */
function piecesWithout(n: number, edges: readonly [number, number, number][], skipNode = -1, skipEdge = -1): number {
  const label = new Array<number>(n).fill(-1);
  let count = 0;
  for (let s = 0; s < n; s++) {
    if (s === skipNode || label[s] !== -1) continue;
    label[s] = count;
    const todo = [s];
    while (todo.length) {
      const x = todo.pop()!;
      edges.forEach(([u, v], k) => {
        if (k === skipEdge || u === skipNode || v === skipNode) return;
        const y = u === x ? v : v === x ? u : -1;
        if (y >= 0 && label[y] === -1) {
          label[y] = count;
          todo.push(y);
        }
      });
    }
    count++;
  }
  return count;
}


/** A stored grid, read independently of engine/grid.ts. */
function gridOf(input: readonly number[]) {
  const [rows, cols, sr, sc, gr, gc] = input as number[];
  const cell = (r: number, c: number) => input[6 + r * cols! + c]!;
  const open = (r: number, c: number) => r >= 0 && r < rows! && c >= 0 && c < cols! && cell(r, c) !== 1;
  const steps = [[-1, 0], [0, 1], [1, 0], [0, -1]] as const;
  return { rows: rows!, cols: cols!, start: [sr!, sc!] as const, goal: [gr!, gc!] as const, cell, open, steps };
}

/** Cheapest cost from start to goal by relaxing every cell until nothing changes. */
function gridCost(input: readonly number[], unit: boolean): number {
  const g = gridOf(input);
  const d = Array.from({ length: g.rows }, () => new Array<number>(g.cols).fill(Infinity));
  d[g.start[0]]![g.start[1]] = 0;
  for (let changed = true; changed; ) {
    changed = false;
    for (let r = 0; r < g.rows; r++)
      for (let c = 0; c < g.cols; c++) {
        if (!g.open(r, c)) continue;
        for (const [dr, dc] of g.steps) {
          if (!g.open(r + dr, c + dc)) continue;
          const w = unit ? 1 : Math.max(1, g.cell(r, c));
          if (d[r + dr]![c + dc]! + w < d[r]![c]!) {
            d[r]![c] = d[r + dr]![c + dc]! + w;
            changed = true;
          }
        }
      }
  }
  const best = d[g.goal[0]]![g.goal[1]]!;
  return best === Infinity ? -1 : best;
}


/** Lexicographic order of number lists (a prefix comes first). */
const lexi = (a: readonly number[], b: readonly number[]) => {
  for (let k = 0; k < Math.min(a.length, b.length); k++) if (a[k] !== b[k]) return a[k]! - b[k]!;
  return a.length - b.length;
};

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
  // Trees, part 2.
  "avl-rotations": (values) => {
    let t: AvlT = null;
    for (const v of values) t = avlInsert(t, v);
    const out: number[] = [];
    const walk = (x: AvlT) => {
      if (!x) return;
      out.push(x.v);
      walk(x.l);
      walk(x.r);
    };
    walk(t);
    return out;
  },
  "red-black-insert": (values) => {
    // Read the 2-3 tree as a red-black tree.
    const vals: number[] = [];
    const reds: number[] = [];
    const walk = (t: T23 | undefined) => {
      if (!t) return;
      if (t.keys.length === 1) {
        vals.push(t.keys[0]!);
        reds.push(0);
        walk(t.kids[0]);
        walk(t.kids[1]);
      } else {
        vals.push(t.keys[1]!, t.keys[0]!);
        reds.push(0, 1);
        walk(t.kids[0]);
        walk(t.kids[1]);
        walk(t.kids[2]);
      }
    };
    walk(build23(values));
    return [...vals, ...reds];
  },
  "b-tree-insert": (values) => {
    const out: number[] = [];
    let level = [build23(values)];
    while (level.length) {
      for (const t of level) out.push(...t.keys, 0);
      level = level.flatMap((t) => t.kids);
    }
    return out;
  },
  "segment-tree": (input) => {
    const n = input[0]!;
    const a = input.slice(1, 1 + n);
    const out: number[] = [];
    for (const [op, x, y] of ops(input, 2, 1 + n)) {
      if (op === 1) out.push(a.slice(x, y! + 1).reduce((p, q) => p + q, 0));
      else a[x!] = y!;
    }
    return out;
  },
  "lazy-propagation": (input) => {
    const a = new Array<number>(input[0]!).fill(0);
    const out: number[] = [];
    for (const [op, l, r, v] of ops(input, 3, 1)) {
      if (op === 1) for (let k = l!; k <= r!; k++) a[k]! += v!;
      else out.push(a.slice(l, r! + 1).reduce((p, q) => p + q, 0));
    }
    return out;
  },
  "fenwick-tree": (input) => {
    const a = new Array<number>(input[0]!).fill(0);
    const out: number[] = [];
    for (const [op, i, v] of ops(input, 2, 1)) {
      if (op === 1) a[i!]! += v!;
      else out.push(a.slice(0, i! + 1).reduce((p, q) => p + q, 0));
    }
    return out;
  },
  "lca-binary-lifting": (input) => {
    // Mark a's ancestors, then climb from b to the first marked one.
    const m = input[0]!;
    const parent = [0, ...input.slice(1, 1 + m)];
    const flat = input.slice(1 + m);
    const out: number[] = [];
    for (let k = 0; k < flat.length; k += 2) {
      const seen = new Set<number>();
      for (let x = flat[k]!; ; x = parent[x]!) {
        seen.add(x);
        if (x === 0) break;
      }
      let y = flat[k + 1]!;
      while (!seen.has(y)) y = parent[y]!;
      out.push(y);
    }
    return out;
  },
  // Graphs, part 1.
  "graph-representations": (input) => {
    const g = graphOf(input);
    const matrix = Array.from({ length: g.n }, () => new Array<number>(g.n).fill(0));
    for (const [u, v, w] of arcsOf(g)) matrix[u]![v] = w;
    return [...Array.from({ length: g.n }, (_, x) => g.nbrs(x)).flatMap((l) => [l.length, ...l]), ...matrix.flat()];
  },
  bfs: (input) => {
    const g = graphOf(input);
    return allPairs(g.n, arcsOf(g, true))[0]!.map((d) => d ?? -1);
  },
  dfs: (input) => {
    // Iterative, with an explicit stack of (node, next neighbour index).
    const g = graphOf(input);
    const seen = new Set<number>([0]);
    const order = [0];
    const finished: number[] = [];
    const stack: [number, number][] = [[0, 0]];
    while (stack.length) {
      const top = stack[stack.length - 1]!;
      const list = g.nbrs(top[0]);
      if (top[1] === list.length) {
        finished.push(top[0]);
        stack.pop();
        continue;
      }
      const v = list[top[1]++]!;
      if (!seen.has(v)) {
        seen.add(v);
        order.push(v);
        stack.push([v, 0]);
      }
    }
    return [...order, ...finished];
  },
  "cycle-detection": (input) => {
    // A directed graph has a cycle exactly when repeatedly deleting sources cannot empty it.
    const g = graphOf(input);
    const alive = new Set(Array.from({ length: g.n }, (_, k) => k));
    for (;;) {
      const source = [...alive].find((x) => !g.edges.some(([u, v]) => v === x && alive.has(u)));
      if (source === undefined) break;
      alive.delete(source);
    }
    return [alive.size > 0 ? 1 : 0];
  },
  "topological-sort": (input) => {
    // Kahn's order, by levels: each level is the queue's next run, in the order it was filled.
    const g = graphOf(input);
    const indeg = new Array<number>(g.n).fill(0);
    for (const [, v] of g.edges) indeg[v]!++;
    const order: number[] = [];
    for (let k = 0; k < g.n; k++) if (indeg[k] === 0) order.push(k);
    for (let i = 0; i < order.length; i++)
      for (const v of g.nbrs(order[i]!)) if (--indeg[v]! === 0) order.push(v);
    return order;
  },
  "connected-components": (input) => {
    // Union-find, then number the components by their smallest node.
    const g = graphOf(input);
    const parent = Array.from({ length: g.n }, (_, k) => k);
    const root = (x: number): number => (parent[x] === x ? x : root(parent[x]!));
    for (const [u, v] of g.edges) parent[root(u)] = root(v);
    const names = new Map<number, number>();
    return Array.from({ length: g.n }, (_, x) => {
      if (!names.has(root(x))) names.set(root(x), names.size);
      return names.get(root(x))!;
    });
  },
  "bipartite-check": (input) => {
    // Side = parity of the distance from the smallest node of the component.
    const g = graphOf(input);
    const d = allPairs(g.n, arcsOf(g, true));
    const side = Array.from({ length: g.n }, (_, x) => {
      const first = d.findIndex((row) => row[x] !== null);
      return d[first]![x]! % 2;
    });
    return g.edges.some(([u, v]) => side[u] === side[v]) ? [-1] : side;
  },
  dijkstra: (input) => {
    const g = graphOf(input);
    return allPairs(g.n, arcsOf(g))[0]!.map((d) => d ?? -1);
  },
  "bellman-ford": (input) => {
    // A negative cycle reachable from 0 shows as a negative d[k][k] on a reachable k.
    const g = graphOf(input);
    const d = allPairs(g.n, arcsOf(g));
    if (d.some((row, k) => d[0]![k] !== null && row[k]! < 0)) return [1];
    return [0, ...d[0]!.map((x) => x ?? 999)];
  },
  // Graphs, part 2.
  "floyd-warshall": (input) => {
    // Bellman-Ford from every source (weights are positive here).
    const g = graphOf(input);
    return Array.from({ length: g.n }, (_, s) => {
      const d = new Array<number>(g.n).fill(999);
      d[s] = 0;
      for (let round = 0; round < g.n; round++) for (const [u, v, w] of g.edges) if (d[u]! + w < d[v]!) d[v] = d[u]! + w;
      return d;
    }).flat();
  },
  "zero-one-bfs": (input) => {
    const g = graphOf(input);
    return allPairs(g.n, arcsOf(g))[0]!.map((d) => d ?? -1);
  },
  prim: (input) => {
    // O(n²) Prim: each round adds the outside node with the cheapest edge into the tree
    // (ties: smaller node), which is the order a (weight, node) heap pops them.
    const g = graphOf(input);
    const inTree = new Set([0]);
    const order = [0];
    let total = 0;
    for (;;) {
      let best: [number, number] | null = null;
      for (const [u, v, w] of g.edges)
        for (const [a, b] of [[u, v], [v, u]] as const)
          if (inTree.has(a) && !inTree.has(b) && (!best || w < best[0] || (w === best[0] && b < best[1]))) best = [w, b];
      if (!best) break;
      inTree.add(best[1]);
      order.push(best[1]);
      total += best[0];
    }
    return [total, ...order];
  },
  kruskal: (input) => {
    // The same greedy choice, with component labels relabelled on every merge.
    const g = graphOf(input);
    const comp = Array.from({ length: g.n }, (_, k) => k);
    let total = 0;
    const used: number[] = [];
    for (const [u, v, w] of [...g.edges].sort((a, b) => a[2] - b[2] || a[0] - b[0] || a[1] - b[1])) {
      if (comp[u] === comp[v]) continue;
      const from = comp[u];
      for (let k = 0; k < g.n; k++) if (comp[k] === from) comp[k] = comp[v]!;
      total += w;
      used.push(u, v);
    }
    return [total, ...used];
  },
  scc: (input) => {
    // Membership from mutual reachability; numbering in the order Kosaraju's second pass
    // meets components: by the reversed finish order of an iterative DFS.
    const g = graphOf(input);
    const reach = allPairs(g.n, arcsOf(g, true));
    const seen = new Set<number>();
    const finished: number[] = [];
    for (let s = 0; s < g.n; s++) {
      if (seen.has(s)) continue;
      seen.add(s);
      const stack: [number, number][] = [[s, 0]];
      while (stack.length) {
        const top = stack[stack.length - 1]!;
        const list = g.nbrs(top[0]);
        if (top[1] === list.length) {
          finished.push(top[0]);
          stack.pop();
        } else {
          const v = list[top[1]++]!;
          if (!seen.has(v)) {
            seen.add(v);
            stack.push([v, 0]);
          }
        }
      }
    }
    const label = new Array<number>(g.n).fill(-1);
    let count = 0;
    for (const u of finished.reverse()) {
      if (label[u] !== -1) continue;
      for (let v = 0; v < g.n; v++) if (reach[u]![v] !== null && reach[v]![u] !== null) label[v] = count;
      count++;
    }
    return label;
  },
  "bridges-articulation": (input) => {
    // Brute force: an edge is a bridge, a node a cut vertex, when removing it adds pieces.
    const g = graphOf(input);
    const base = piecesWithout(g.n, g.edges);
    const bridges = g.edges
      .filter((_, k) => piecesWithout(g.n, g.edges, -1, k) > base)
      .map(([u, v]) => [Math.min(u, v), Math.max(u, v)] as [number, number])
      .sort((a, b) => a[0] - b[0] || a[1] - b[1]);
    const cuts = Array.from({ length: g.n }, (_, x) => x).filter((x) => piecesWithout(g.n, g.edges, x) > base - (g.nbrs(x).length === 0 ? 1 : 0));
    return [...bridges.flat(), -1, ...cuts];
  },
  "max-flow": (input) => {
    // Max flow = min cut: try every set S with the source and without the sink.
    const g = graphOf(input);
    const t = g.n - 1;
    let best = Infinity;
    for (let mask = 0; mask < 1 << g.n; mask++) {
      if (!(mask & 1) || mask & (1 << t)) continue;
      const cut = g.edges.reduce((sum, [u, v, c]) => sum + (mask & (1 << u) && !(mask & (1 << v)) ? c : 0), 0);
      best = Math.min(best, cut);
    }
    return [best];
  },
  "eulerian-path": (input) => {
    // Degrees and reachability decide existence; the recursive Hierholzer gives the path.
    const g = graphOf(input);
    const deg = Array.from({ length: g.n }, (_, x) => g.nbrs(x).length);
    const odd = deg.flatMap((d, x) => (d % 2 ? [x] : []));
    if (odd.length !== 0 && odd.length !== 2) return [-1];
    const adj: [number, number][][] = Array.from({ length: g.n }, () => []);
    g.edges.forEach(([u, v], k) => {
      adj[u]!.push([v, k]);
      adj[v]!.push([u, k]);
    });
    const used = new Set<number>();
    const path: number[] = [];
    const walk = (u: number): void => {
      for (const [v, k] of adj[u]!) {
        if (used.has(k)) continue;
        used.add(k);
        walk(v);
      }
      path.push(u);
    };
    walk(odd.length ? odd[0]! : g.edges[0]![0]);
    return used.size === g.edges.length ? path.reverse() : [-1];
  },
  // Grid pathfinding.
  "grid-bfs": (input) => [gridCost(input, true)],
  "grid-dijkstra": (input) => [gridCost(input, false)],
  "a-star": (input) => [gridCost(input, false)],
  "greedy-best-first": (input) => {
    // The same rule with a plain list: take the open entry with the smallest (h, r, c).
    const g = gridOf(input);
    const h = (r: number, c: number) => Math.abs(r - g.goal[0]) + Math.abs(c - g.goal[1]);
    const paid = new Map<string, number>([[`${g.start[0]},${g.start[1]}`, 0]]);
    const open: [number, number][] = [[g.start[0], g.start[1]]];
    while (open.length) {
      let best = 0;
      open.forEach(([r, c], j) => {
        const [br, bc] = open[best]!;
        if (h(r, c) < h(br, bc) || (h(r, c) === h(br, bc) && (r < br || (r === br && c < bc)))) best = j;
      });
      const [r, c] = open.splice(best, 1)[0]!;
      if (r === g.goal[0] && c === g.goal[1]) return [paid.get(`${r},${c}`)!];
      for (const [dr, dc] of g.steps) {
        const key = `${r + dr},${c + dc}`;
        if (!g.open(r + dr, c + dc) || paid.has(key)) continue;
        paid.set(key, paid.get(`${r},${c}`)! + Math.max(1, g.cell(r + dr, c + dc)));
        open.push([r + dr, c + dc]);
      }
    }
    return [-1];
  },
  "maze-generation": ([k, seed]) => {
    // Recursive carving with the same generator.
    const size = 2 * k! + 1;
    const grid = Array.from({ length: size }, () => new Array<number>(size).fill(1));
    let state = seed!;
    const carve = (r: number, c: number): void => {
      grid[r]![c] = 0;
      for (;;) {
        const options = ([[-1, 0], [0, 1], [1, 0], [0, -1]] as const).filter(([dr, dc]) => r + 2 * dr > 0 && r + 2 * dr < size && c + 2 * dc > 0 && c + 2 * dc < size && grid[r + 2 * dr]![c + 2 * dc] === 1);
        if (!options.length) return;
        state = (state * 48271) % 2147483647;
        const [dr, dc] = options[state % options.length]!;
        grid[r + dr]![c + dc] = 0;
        carve(r + 2 * dr, c + 2 * dc);
      }
    };
    carve(1, 1);
    return grid.flat();
  },
  // Dynamic programming, part 1.
  "fibonacci-memo-table": ([n]) => {
    // Binet-free: two running numbers.
    const table = [0];
    let [a, b] = [0, 1];
    for (let k = 1; k <= n!; k++) {
      [a, b] = [b, a + b];
      table.push(a);
    }
    return [table[n!]!, ...table];
  },
  "climbing-stairs": ([n]) => {
    // ways(m) = Σ C(m − k, k): choose which k of the m − k moves are double steps.
    const choose = (a: number, b: number) => {
      let r = 1;
      for (let j = 1; j <= b; j++) r = (r * (a - b + j)) / j;
      return Math.round(r);
    };
    return Array.from({ length: n! + 1 }, (_, m) => {
      let total = 0;
      for (let k = 0; 2 * k <= m; k++) total += choose(m - k, k);
      return total;
    });
  },
  "house-robber": (values) => {
    // Top-down: best(i) for the first i houses; trace back preferring to skip.
    const memo = new Map<number, number>();
    const best = (i: number): number => {
      if (i <= 0) return 0;
      if (!memo.has(i)) memo.set(i, Math.max(best(i - 1), best(i - 2) + values[i - 1]!));
      return memo.get(i)!;
    };
    const chosen: number[] = [];
    for (let i = values.length; i > 0; ) {
      if (best(i) === best(i - 1)) i--;
      else {
        chosen.unshift(i - 1);
        i -= 2;
      }
    }
    return [best(values.length), ...chosen];
  },
  "maximum-subarray": (values) => {
    // Every run; ties: the earliest end, then the latest start (what one pass finds first).
    let best = [values[0]!, 0, 0];
    for (let end = 0; end < values.length; end++)
      for (let start = end; start >= 0; start--) {
        const sum = values.slice(start, end + 1).reduce((x, y) => x + y, 0);
        if (sum > best[0]!) best = [sum, start, end];
      }
    return best;
  },
  "coin-change": ([amount, ...coins]) => {
    // Breadth-first over totals: the first time a total is reached, it used fewest coins.
    const seen = new Map<number, number>([[0, 0]]);
    const queue = [0];
    while (queue.length) {
      const t = queue.shift()!;
      for (const c of coins) {
        if (t + c <= amount! && !seen.has(t + c)) {
          seen.set(t + c, seen.get(t)! + 1);
          queue.push(t + c);
        }
      }
    }
    return [seen.get(amount!) ?? -1];
  },
  "knapsack-01": ([capacity, ...flat]) => {
    // Top-down over (items left, capacity); trace back: packed when it changes the value.
    const items = Array.from({ length: flat.length / 2 }, (_, k) => [flat[2 * k]!, flat[2 * k + 1]!] as const);
    const memo = new Map<string, number>();
    const best = (i: number, c: number): number => {
      if (i === 0) return 0;
      const key = `${i},${c}`;
      if (!memo.has(key)) {
        const [w, v] = items[i - 1]!;
        memo.set(key, Math.max(best(i - 1, c), w <= c ? best(i - 1, c - w) + v : -Infinity));
      }
      return memo.get(key)!;
    };
    const chosen: number[] = [];
    let c = capacity!;
    for (let i = items.length; i > 0; i--) {
      if (best(i, c) !== best(i - 1, c)) {
        chosen.unshift(i - 1);
        c -= items[i - 1]![0];
      }
    }
    return [best(items.length, capacity!), ...chosen];
  },
  "rod-cutting": (prices) => {
    // Top-down; the first piece is the shortest one that reaches the best total.
    const memo = new Map<number, number>([[0, 0]]);
    const best = (len: number): number => {
      if (!memo.has(len)) memo.set(len, Math.max(...Array.from({ length: len }, (_, k) => prices[k]! + best(len - k - 1))));
      return memo.get(len)!;
    };
    const pieces: number[] = [];
    for (let len = prices.length; len > 0; ) {
      const cut = Array.from({ length: len }, (_, k) => k + 1).find((k) => prices[k - 1]! + best(len - k) === best(len))!;
      pieces.push(cut);
      len -= cut;
    }
    return [best(prices.length), ...pieces];
  },
  // Dynamic programming, part 2, and greedy.
  lis: (values) => {
    // Top-down: L(i) = 1 + the best L(j) of a smaller earlier value (first j on ties).
    const memo = new Map<number, [number, number]>();
    const L = (i: number): [number, number] => {
      if (!memo.has(i)) {
        let best: [number, number] = [1, -1];
        for (let j = 0; j < i; j++) if (values[j]! < values[i]! && L(j)[0] + 1 > best[0]) best = [L(j)[0] + 1, j];
        memo.set(i, best);
      }
      return memo.get(i)!;
    };
    let end = 0;
    for (let k = 1; k < values.length; k++) if (L(k)[0] > L(end)[0]) end = k;
    const chain: number[] = [];
    for (let k = end; k !== -1; k = L(k)[1]) chain.unshift(values[k]!);
    return [chain.length, ...chain];
  },
  lcs: (input) => {
    const [a, b] = words(input) as [string, string];
    const memo = new Map<string, number>();
    const L = (i: number, j: number): number => {
      if (i === 0 || j === 0) return 0;
      const key = `${i},${j}`;
      if (!memo.has(key)) memo.set(key, a[i - 1] === b[j - 1] ? L(i - 1, j - 1) + 1 : Math.max(L(i - 1, j), L(i, j - 1)));
      return memo.get(key)!;
    };
    const out: number[] = [];
    for (let i = a.length, j = b.length; i > 0 && j > 0; ) {
      if (a[i - 1] === b[j - 1]) {
        out.unshift(a.charCodeAt(i - 1) - 97);
        i--;
        j--;
      } else if (L(i - 1, j) >= L(i, j - 1)) i--;
      else j--;
    }
    return [out.length, ...out];
  },
  "edit-distance": (input) => {
    const [a, b] = words(input) as [string, string];
    const memo = new Map<string, number>();
    const D = (i: number, j: number): number => {
      if (i === 0) return j;
      if (j === 0) return i;
      const key = `${i},${j}`;
      if (!memo.has(key)) memo.set(key, a[i - 1] === b[j - 1] ? D(i - 1, j - 1) : 1 + Math.min(D(i - 1, j - 1), D(i - 1, j), D(i, j - 1)));
      return memo.get(key)!;
    };
    return [D(a.length, b.length)];
  },
  "grid-paths": ([rows, cols, ...cells]) => {
    // Walk every right/down path.
    const walk = (r: number, c: number): number => {
      if (r >= rows! || c >= cols! || cells[r * cols! + c] === 1) return 0;
      if (r === rows! - 1 && c === cols! - 1) return 1;
      return walk(r + 1, c) + walk(r, c + 1);
    };
    return [walk(0, 0)];
  },
  "word-break": (input) => {
    const [text, ...dict] = words(input);
    const memo = new Map<number, boolean>();
    const can = (from: number): boolean => {
      if (from === text!.length) return true;
      if (!memo.has(from)) memo.set(from, dict.some((w) => text!.startsWith(w, from) && can(from + w.length)));
      return memo.get(from)!;
    };
    return [can(0) ? 1 : 0];
  },
  "matrix-chain": (dims) => {
    // Every bracketing, recursively, without a table.
    const best = (i: number, j: number): number => {
      if (i === j) return 0;
      let m = Infinity;
      for (let k = i; k < j; k++) m = Math.min(m, best(i, k) + best(k + 1, j) + dims[i]! * dims[k + 1]! * dims[j + 1]!);
      return m;
    };
    return [best(0, dims.length - 2)];
  },
  "bitmask-tsp": ([n, ...flat]) => {
    // Every order of the other cities.
    const d = (i: number, j: number) => flat[i * n! + j]!;
    const rest = Array.from({ length: n! - 1 }, (_, k) => k + 1);
    let best = Infinity;
    const permute = (left: number[], at: number, sum: number) => {
      if (!left.length) {
        best = Math.min(best, sum + d(at, 0));
        return;
      }
      left.forEach((c, k) => permute([...left.slice(0, k), ...left.slice(k + 1)], c, sum + d(at, c)));
    };
    permute(rest, 0, 0);
    return [best];
  },
  "activity-selection": (input) => {
    // The count by brute force over subsets; the set by repeatedly taking the compatible
    // activity that ends first (ties: earlier start, then earlier input).
    const iv = Array.from({ length: input.length / 2 }, (_, k) => [input[2 * k]!, input[2 * k + 1]!, k] as const);
    let most = 0;
    for (let mask = 0; mask < 1 << iv.length; mask++) {
      const pick = iv.filter((_, k) => mask & (1 << k)).sort((x, y) => x[0] - y[0]);
      if (pick.every((x, k) => k === 0 || pick[k - 1]![1] <= x[0])) most = Math.max(most, pick.length);
    }
    const chosen: number[] = [];
    let free = -Infinity;
    for (;;) {
      const next = iv.filter((x) => x[0] >= free && !chosen.includes(x[2])).sort((x, y) => x[1] - y[1] || x[0] - y[0] || x[2] - y[2])[0];
      if (!next) break;
      chosen.push(next[2]);
      free = next[1];
    }
    return chosen.length === most ? [most, ...chosen.sort((x, y) => x - y)] : [-1];
  },
  "fractional-knapsack": ([capacity, ...flat]) => {
    // Exact: compare ratios by cross-multiplication, keep the answer as a fraction.
    const items = Array.from({ length: flat.length / 2 }, (_, k) => ({ w: flat[2 * k]!, v: flat[2 * k + 1]!, k }));
    items.sort((a, b) => b.v * a.w - a.v * b.w || a.k - b.k);
    let num = 0;
    let den = 1;
    let room = capacity!;
    for (const it of items) {
      const take = Math.min(room, it.w);
      // num/den += it.v * take / it.w
      num = num * it.w + it.v * take * den;
      den = den * it.w;
      room -= take;
      if (!room) break;
    }
    const g = (x: number, y: number): number => (y === 0 ? x : g(y, x % y));
    const k = g(num, den);
    num /= k;
    den /= k;
    return [Math.floor(num / den), num % den, den];
  },
  "huffman-coding": (freqs) => {
    // Total bits = the sum of every merged weight; lengths from a list kept by hand.
    let forest = freqs.map((f, k) => ({ f, id: k, leaves: [k] }));
    const depth = new Array<number>(freqs.length).fill(0);
    let total = 0;
    let next = freqs.length;
    while (forest.length > 1) {
      forest.sort((x, y) => x.f - y.f || x.id - y.id);
      const [a, b] = forest;
      for (const leaf of [...a!.leaves, ...b!.leaves]) depth[leaf]!++;
      total += a!.f + b!.f;
      forest = [...forest.slice(2), { f: a!.f + b!.f, id: next++, leaves: [...a!.leaves, ...b!.leaves] }];
    }
    return [total, ...depth];
  },
  "jump-game": (jumps) => {
    // Breadth-first over squares.
    const seen = new Set([0]);
    const queue = [0];
    while (queue.length) {
      const i = queue.shift()!;
      for (let k = 1; k <= jumps[i]! && i + k < jumps.length; k++)
        if (!seen.has(i + k)) {
          seen.add(i + k);
          queue.push(i + k);
        }
    }
    if (seen.has(jumps.length - 1)) return [1, jumps.length - 1];
    return [0, Math.max(...[...seen].map((i) => i + jumps[i]!))];
  },
  "merge-intervals": (input) => {
    // Mark covered points and the half-steps between them, then read off the runs.
    const max = Math.max(...input);
    const covered = new Array<boolean>(2 * max + 1).fill(false);
    for (let k = 0; k < input.length; k += 2) for (let t = 2 * input[k]!; t <= 2 * input[k + 1]!; t++) covered[t] = true;
    const out: number[] = [];
    for (let t = 0; t < covered.length; t++) {
      if (covered[t] && !covered[t - 1]) out.push(t / 2);
      if (covered[t] && !covered[t + 1]) out.push(t / 2);
    }
    return out;
  },
  // Backtracking and strings.
  subsets: (values) => {
    // Every bitmask, as position lists, in lexicographic order (the depth-first order).
    const lists = Array.from({ length: 1 << values.length }, (_, m) => values.map((_, k) => k).filter((k) => m & (1 << k))).sort(lexi);
    return [lists.length, ...lists.flatMap((l) => [l.length, ...l.map((k) => values[k]!)])];
  },
  permutations: (values) => {
    // Next-permutation on positions, starting from 0, 1, 2, ….
    const n = values.length;
    const p = values.map((_, k) => k);
    const out: number[] = [];
    let count = 0;
    for (;;) {
      out.push(...p.map((k) => values[k]!));
      count++;
      let i = n - 2;
      while (i >= 0 && p[i]! > p[i + 1]!) i--;
      if (i < 0) break;
      let j = n - 1;
      while (p[j]! < p[i]!) j--;
      [p[i], p[j]] = [p[j]!, p[i]!];
      p.splice(i + 1, n - i - 1, ...p.slice(i + 1).reverse());
    }
    return [count, ...out];
  },
  "combination-sum": ([target, ...raw]) => {
    // Every vector of counts per candidate (sorted, each position distinct).
    const c = [...raw].sort((a, b) => a - b);
    const found: number[][] = [];
    const counts = new Array<number>(c.length).fill(0);
    const walk = (k: number, sum: number) => {
      if (k === c.length) {
        if (sum === target) found.push(counts.flatMap((n, j) => new Array<number>(n).fill(j)));
        return;
      }
      for (let n = 0; sum + n * c[k]! <= target!; n++) {
        counts[k] = n;
        walk(k + 1, sum + n * c[k]!);
      }
      counts[k] = 0;
    };
    walk(0, 0);
    found.sort(lexi);
    return [found.length, ...found.flatMap((f) => [f.length, ...f.map((j) => c[j]!)])];
  },
  "n-queens": ([n]) => {
    // Every column permutation; keep those with no shared diagonal.
    const sols: number[][] = [];
    const perm = (left: number[], acc: number[]) => {
      if (!left.length) {
        if (acc.every((a, i) => acc.every((b, j) => i === j || Math.abs(a - b) !== Math.abs(i - j)))) sols.push(acc);
        return;
      }
      left.forEach((x, k) => perm([...left.slice(0, k), ...left.slice(k + 1)], [...acc, x]));
    };
    perm(Array.from({ length: n! }, (_, k) => k), []);
    return [sols.length, ...(sols[0] ?? [])];
  },
  sudoku: ([size, ...cells]) => {
    // All completions; the smallest in reading order is what a first-cell, first-digit search finds.
    const box = Math.round(Math.sqrt(size!));
    const g = [...cells];
    const sols: number[][] = [];
    const fits = (at: number, d: number) =>
      g.every((v, k) => {
        if (v !== d || k === at) return true;
        const [r1, c1, r2, c2] = [Math.floor(at / size!), at % size!, Math.floor(k / size!), k % size!];
        return r1 !== r2 && c1 !== c2 && (Math.floor(r1 / box) !== Math.floor(r2 / box) || Math.floor(c1 / box) !== Math.floor(c2 / box));
      });
    const go = () => {
      if (sols.length > 50) return;
      const at = g.indexOf(0);
      if (at < 0) {
        sols.push([...g]);
        return;
      }
      for (let d = 1; d <= size!; d++)
        if (fits(at, d)) {
          g[at] = d;
          go();
          g[at] = 0;
        }
    };
    go();
    return sols.length ? sols.sort(lexi)[0]! : [-1];
  },
  "word-search": ([rows, cols, ...rest]) => {
    // Every matching path, in (start cell, direction) order; the first one.
    const cut = rest.indexOf(-1);
    const letters = rest.slice(0, cut);
    const word = rest.slice(cut + 1);
    const paths: number[][] = [];
    const extend = (path: number[]) => {
      if (paths.length) return;
      if (path.length === word.length) {
        paths.push(path);
        return;
      }
      const last = path[path.length - 1]!;
      for (const [dr, dc] of [[-1, 0], [0, 1], [1, 0], [0, -1]] as const) {
        const r = Math.floor(last / cols!) + dr;
        const c = (last % cols!) + dc;
        const k = r * cols! + c;
        if (r >= 0 && r < rows! && c >= 0 && c < cols! && !path.includes(k) && letters[k] === word[path.length]) extend([...path, k]);
      }
    };
    for (let k = 0; k < rows! * cols! && !paths.length; k++) if (letters[k] === word[0]) extend([k]);
    return paths.length ? [1, ...paths[0]!.flatMap((k) => [Math.floor(k / cols!), k % cols!])] : [0];
  },
  kmp: (input) => {
    const [text, pattern] = words(input) as [string, string];
    const hits: number[] = [];
    for (let i = 0; i + pattern.length <= text.length; i++) if (text.startsWith(pattern, i)) hits.push(i);
    return [hits.length, ...hits];
  },
  "z-function": (input) => REFERENCE.kmp!(input),
  "rabin-karp": (input) => {
    // Hash every window from scratch.
    const [text, pattern] = words(input) as [string, string];
    const hash = (w: string) => [...w].reduce((h, ch) => (h * 26 + ch.charCodeAt(0) - 97) % 101, 0);
    const hits: number[] = [];
    let collisions = 0;
    for (let i = 0; i + pattern.length <= text.length; i++) {
      const w = text.slice(i, i + pattern.length);
      if (hash(w) !== hash(pattern)) continue;
      if (w === pattern) hits.push(i);
      else collisions++;
    }
    return [hits.length, collisions, ...hits];
  },
  manacher: (input) => {
    // Every substring; the first longest palindrome.
    const [text] = words(input) as [string];
    let best = [1, 0];
    for (let len = text.length; len > 1 && best[0] === 1; len--)
      for (let i = 0; i + len <= text.length; i++) {
        const w = text.slice(i, i + len);
        if (w === [...w].reverse().join("")) {
          best = [len, i];
          break;
        }
      }
    return best;
  },
  "aho-corasick": (input) => {
    // Count each pattern separately, overlaps included.
    const [text, ...pats] = words(input);
    return pats.map((p) => {
      let n = 0;
      for (let i = 0; i + p.length <= text!.length; i++) if (text!.startsWith(p, i)) n++;
      return n;
    });
  },
  "suffix-array": (input) => {
    const [text] = words(input) as [string];
    return [...Array(text.length).keys()].sort((a, b) => (text.slice(a) < text.slice(b) ? -1 : 1));
  },
  // Math and geometry.
  "euclid-gcd": ([a, b]) => {
    // The divisor by trial; the step count from the recursive definition.
    let g = 1;
    for (let d = 1; d <= Math.min(a!, b!); d++) if (a! % d === 0 && b! % d === 0) g = d;
    const steps = (x: number, y: number): number => (y === 0 ? 0 : 1 + steps(y, x % y));
    return [g, steps(a!, b!)];
  },
  "sieve-eratosthenes": ([n]) => {
    const primes: number[] = [];
    for (let k = 2; k <= n!; k++) {
      let prime = true;
      for (let d = 2; d * d <= k; d++) if (k % d === 0) prime = false;
      if (prime) primes.push(k);
    }
    return [primes.length, ...primes];
  },
  "fast-power": ([base, exp, mod]) => {
    let r = 1 % mod!;
    for (let k = 0; k < exp!; k++) r = (r * base!) % mod!;
    return [r];
  },
  "modular-inverse": ([a, m]) => {
    for (let x = 0; x < m!; x++) if ((a! * x) % m! === 1 % m!) return [x];
    return [-1];
  },
  "pascal-ncr": ([n]) => {
    const row = [1];
    for (let k = 1; k <= n!; k++) row.push(Math.round((row[k - 1]! * (n! - k + 1)) / k));
    return row;
  },
  "convex-hull": (input) => {
    // Gift wrapping: from the lowest-leftmost point, the next corner has every other
    // point to its left (or strictly between, on the edge).
    const key = (p: readonly number[]) => `${p[0]},${p[1]}`;
    const pts = [...new Map(Array.from({ length: input.length / 2 }, (_, k) => [input[2 * k]!, input[2 * k + 1]!] as const).map((p) => [key(p), p])).values()].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
    if (pts.length <= 2) return pts.flat();
    const cr = (o: readonly number[], a: readonly number[], b: readonly number[]) => (a[0]! - o[0]!) * (b[1]! - o[1]!) - (a[1]! - o[1]!) * (b[0]! - o[0]!);
    const between = (p: readonly number[], q: readonly number[], r: readonly number[]) =>
      Math.min(p[0]!, q[0]!) <= r[0]! && r[0]! <= Math.max(p[0]!, q[0]!) && Math.min(p[1]!, q[1]!) <= r[1]! && r[1]! <= Math.max(p[1]!, q[1]!);
    const hull: (readonly number[])[] = [pts[0]!];
    for (;;) {
      const p = hull[hull.length - 1]!;
      const q = pts.find((c) => key(c) !== key(p) && pts.every((r) => key(r) === key(p) || key(r) === key(c) || cr(p, c, r) > 0 || (cr(p, c, r) === 0 && between(p, c, r))));
      if (!q || key(q) === key(hull[0]!)) break;
      hull.push(q);
    }
    return hull.flat();
  },
  "segment-intersection": ([x1, y1, x2, y2, x3, y3, x4, y4]) => {
    // Signs from the cross product; the answer from the two lines' parameters, exactly.
    const cr = (ax: number, ay: number, bx: number, by: number, cx: number, cy: number) => Math.sign((bx - ax) * (cy - ay) - (by - ay) * (cx - ax)) || 0;
    const signs = [cr(x3!, y3!, x4!, y4!, x1!, y1!), cr(x3!, y3!, x4!, y4!, x2!, y2!), cr(x1!, y1!, x2!, y2!, x3!, y3!), cr(x1!, y1!, x2!, y2!, x4!, y4!)];
    const [dx1, dy1, dx2, dy2] = [x2! - x1!, y2! - y1!, x4! - x3!, y4! - y3!];
    const den = dx1 * dy2 - dy1 * dx2;
    let meet: boolean;
    // A segment that is a single point meets the other one only by lying on it.
    const onSeg = (px: number, py: number, ax: number, ay: number, bx: number, by: number) =>
      cr(ax, ay, bx, by, px, py) === 0 && Math.min(ax, bx) <= px && px <= Math.max(ax, bx) && Math.min(ay, by) <= py && py <= Math.max(ay, by);
    if (dx1 === 0 && dy1 === 0) meet = onSeg(x1!, y1!, x3!, y3!, x4!, y4!);
    else if (dx2 === 0 && dy2 === 0) meet = onSeg(x3!, y3!, x1!, y1!, x2!, y2!);
    else if (den !== 0) {
      // P1 + t·d1 = P3 + u·d2 with t = tn / den, u = un / den.
      const tn = (x3! - x1!) * dy2 - (y3! - y1!) * dx2;
      const un = (x3! - x1!) * dy1 - (y3! - y1!) * dx1;
      const within = (v: number) => (den > 0 ? v >= 0 && v <= den : v <= 0 && v >= den);
      meet = within(tn) && within(un);
    } else if ((x3! - x1!) * dy1 - (y3! - y1!) * dx1 !== 0) meet = false;
    else {
      // Collinear: project on the longer axis and compare intervals.
      const axis = dx1 !== 0 || dx2 !== 0 ? 0 : 1;
      const [a1, a2, b1, b2] = axis === 0 ? [x1!, x2!, x3!, x4!] : [y1!, y2!, y3!, y4!];
      meet = Math.max(Math.min(a1, a2), Math.min(b1, b2)) <= Math.min(Math.max(a1, a2), Math.max(b1, b2));
    }
    return [meet ? 1 : 0, ...signs];
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
  // Databases: written differently from the tracers (nested loops, filters, walking up).
  "join-algorithms": (input) => {
    const n = input[0]!;
    const left = Array.from({ length: n }, (_, i) => [input[1 + 2 * i]!, input[2 + 2 * i]!]);
    const rest = input.slice(1 + 2 * n);
    const right = Array.from({ length: rest.length / 2 }, (_, i) => [rest[2 * i]!, rest[2 * i + 1]!]);
    return left.flatMap(([k, a]) => right.filter(([rk]) => rk === k).flatMap(([, b]) => [k!, a!, b!]));
  },
  "group-by": (input) => {
    const rows = Array.from({ length: input.length / 2 }, (_, i) => [input[2 * i]!, input[2 * i + 1]!]);
    return sorted([...new Set(rows.map(([g]) => g!))]).flatMap((g) => {
      const mine = rows.filter(([x]) => x === g).map(([, v]) => v!);
      return [g, mine.length, sum(mine)];
    });
  },
  "window-functions": (input) => {
    const rows = Array.from({ length: input.length / 2 }, (_, i) => [input[2 * i]!, input[2 * i + 1]!]);
    return rows.map(([p], i) => sum(rows.slice(0, i + 1).filter(([q]) => q === p).map(([, v]) => v!)));
  },
  "recursive-cte": (bosses) =>
    bosses.map((_, i) => {
      let depth = 0;
      for (let x = bosses[i]!; x !== -1; x = bosses[x]!) depth++;
      return depth;
    }),
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
  // Databases (worked out by hand from the samples).
  "join-algorithms": {
    input: [1, 2, 7, 2, 5, 3, 9],
    markers: [null, "build", "build", "probe", "match", null],
    py: [null, 11, 11, 14, 16, null],
    ts: [null, 10, 10, 14, 16, null],
  },
  "group-by": {
    input: [1, 5, 2, 3, 1, 4],
    markers: [null, "add", "add", "add", "emit", "emit", null],
    py: [null, 9, 9, 9, 12, 12, null],
    ts: [null, 6, 6, 6, 10, 10, null],
  },
  "window-functions": {
    input: [1, 5, 2, 3, 1, 4],
    markers: [null, "start", "start", "add", null],
    py: [null, 10, 10, 12, null],
    ts: [null, 7, 7, 9, null],
  },
  "recursive-cte": {
    input: [-1, 0, 1],
    markers: [null, "anchor", null, "join", null, "join", null, null],
    py: [null, 10, null, 17, null, 17, null, null],
    ts: [null, 7, null, 16, null, 16, null, null],
  },
  "euclid-gcd": {
    input: [6, 4],
    markers: [null, "divide", "divide", null],
    py: [null, 4, 4, null],
    ts: [null, 4, 4, null],
  },
  "sieve-eratosthenes": {
    input: [9],
    markers: [null, "prime", "cross", "cross", "cross", "prime", "cross", null],
    py: [null, 7, 9, 9, 9, 7, 9, null],
    ts: [null, 7, 9, 9, 9, 7, 9, null],
  },
  "fast-power": {
    input: [2, 5, 7],
    markers: [null, "multiply", "square", "square", "multiply", "square", null],
    py: [null, 6, 7, 7, 6, 7, null],
    ts: [null, 6, 8, 8, 6, 8, null],
  },
  "modular-inverse": {
    input: [3, 7],
    markers: [null, "divide", "divide", "divide", null],
    py: [null, 6, 6, 6, null],
    ts: [null, 6, 6, 6, null],
  },
  "pascal-ncr": {
    input: [3],
    markers: [null, null, null, "add", null, "add", "add", null],
    py: [null, null, null, 6, null, 6, 6, null],
    ts: [null, null, null, 6, null, 6, 6, null],
  },
  "convex-hull": {
    input: [0, 0, 2, 0, 1, 1],
    markers: [null, null, "keep", "keep", "drop", "keep", null, "keep-upper", "keep-upper", "keep-upper", null],
    py: [null, null, 13, 13, 12, 13, null, 17, 17, 17, null],
    ts: [null, null, 15, 15, 13, 15, null, 21, 21, 21, null],
  },
  "segment-intersection": {
    input: [0, 0, 4, 4, 0, 4, 4, 0],
    markers: [null, "side", "side", "side", "side", "cross"],
    py: [null, 14, 14, 14, 14, 16],
    ts: [null, 13, 13, 13, 13, 16],
  },
  subsets: {
    input: [1],
    markers: [null, "record", "choose", "record", "undo", null],
    py: [null, 5, 7, 5, 9, null],
    ts: [null, 5, 7, 5, 9, null],
  },
  permutations: {
    input: [1, 2],
    markers: [null, "choose", "choose", "record", "undo", "undo", "choose", "choose", "record", "undo", "undo", null],
    py: [null, 12, 12, 7, 14, 14, 12, 12, 7, 14, 14, null],
    ts: [null, 14, 14, 8, 16, 16, 14, 14, 8, 16, 16, null],
  },
  "combination-sum": {
    input: [4, 2, 3],
    markers: [null, "choose", "choose", "found", "prune", "choose", "prune", null],
    py: [null, 12, 12, 7, 11, 12, 11, null],
    ts: [null, 14, 14, 7, 12, 14, 12, null],
  },
  "n-queens": {
    input: [4],
    markers: [null, "place", "place", null, "remove", "place", "place", null, "remove", "remove", "remove", "place", "place", "place", "place", "solution", "remove", "remove", "remove", "remove", "place", "place", "place", "place", "solution", "remove", "remove", "remove", "remove", "place", "place", "place", null, "remove", "remove", "place", null, "remove", "remove", null],
    py: [null, 14, 14, null, 16, 14, 14, null, 16, 16, 16, 14, 14, 14, 14, 10, 16, 16, 16, 16, 14, 14, 14, 14, 10, 16, 16, 16, 16, 14, 14, 14, null, 16, 16, 14, null, 16, 16, null],
    ts: [null, 15, 15, null, 17, 15, 15, null, 17, 17, 17, 15, 15, 15, 15, 10, 17, 17, 17, 17, 15, 15, 15, 15, 10, 17, 17, 17, 17, 15, 15, 15, null, 17, 17, 15, null, 17, 17, null],
  },
  sudoku: {
    input: [4, 1, 2, 3, 4, 3, 4, 1, 2, 2, 1, 4, 3, 4, 3, 2, 0],
    markers: [null, "place", null],
    py: [null, 23, null],
    ts: [null, 24, null],
  },
  "word-search": {
    input: [1, 3, 0, 2, 0, -1, 0, 1],
    markers: [null, "step", "back", "step", "back", null],
    py: [null, 10, 17, 10, 17, null],
    ts: [null, 7, 14, 7, 14, null],
  },
  kmp: {
    input: [0, 1, -1, 0, 1, 0],
    markers: [null, "border", "border", null, null],
    py: [null, 10, 10, null, null],
    ts: [null, 10, 10, null, null],
  },
  "z-function": {
    input: [0, -1, 0],
    markers: [null, null, "extend", null, null],
    py: [null, null, 10, null, null],
    ts: [null, null, 12, null, null],
  },
  "rabin-karp": {
    input: [0, 1, -1, 1],
    markers: [null, null, "roll", "match", null],
    py: [null, null, 25, 21, null],
    ts: [null, null, 29, 22, null],
  },
  manacher: {
    input: [0, 0],
    markers: [null, "grow", null, "grow", "grow", null, "mirror", null],
    py: [null, 10, null, 10, 10, null, 8, null],
    ts: [null, 12, null, 12, 12, null, 9, null],
  },
  "aho-corasick": {
    input: [0, 1, -1, 0, 1, -1, 1],
    markers: [null, null, "link", null, null, "hit", "hit", null],
    py: [null, null, 23, null, null, 32, 32, null],
    ts: [null, null, 24, null, null, 37, 37, null],
  },
  "suffix-array": {
    input: [0, 1],
    markers: [null, "sort", null],
    py: [null, 9, null],
    ts: [null, 9, null],
  },
  lis: {
    input: [1, 2],
    markers: [null, null, null, "extend", "trace", "trace", null],
    py: [null, null, null, 8, 13, 13, null],
    ts: [null, null, null, 8, 17, 17, null],
  },
  lcs: {
    input: [0, 1, -1, 1],
    markers: [null, "carry", "match", "trace", null],
    py: [null, 9, 7, 13, null],
    ts: [null, 10, 8, 19, null],
  },
  "edit-distance": {
    input: [0, 1, -1, 1],
    markers: [null, "change", "same", null],
    py: [null, 13, 11, null],
    ts: [null, 12, 10, null],
  },
  "grid-paths": {
    input: [2, 2, 0, 0, 1, 0],
    markers: [null, null, "add", "blocked", "add", null],
    py: [null, null, 12, 6, 12, null],
    ts: [null, null, 12, 6, 12, null],
  },
  "word-break": {
    input: [0, 1, -1, 0, -1, 1],
    markers: [null, null, "fits", null, "fits", null],
    py: [null, null, 9, null, 9, null],
    ts: [null, null, 9, null, 9, null],
  },
  "matrix-chain": {
    input: [2, 3, 4],
    markers: [null, null, "split", null],
    py: [null, null, 11, null],
    ts: [null, null, 11, null],
  },
  "bitmask-tsp": {
    input: [2, 0, 5, 5, 0],
    markers: [null, "extend", null, null],
    py: [null, 18, null, null],
    ts: [null, 15, null, null],
  },
  "activity-selection": {
    input: [0, 2, 1, 3],
    markers: [null, null, "take", "clash", null],
    py: [null, null, 7, 10, null],
    ts: [null, null, 8, 11, null],
  },
  "fractional-knapsack": {
    input: [2, 1, 3, 2, 2],
    markers: [null, null, "take", "part", null],
    py: [null, null, 10, 13, null],
    ts: [null, null, 12, 15, null],
  },
  "huffman-coding": {
    input: [1, 2, 3],
    markers: [null, "merge", "merge", null],
    py: [null, 12, 12, null],
    ts: [null, 10, 10, null],
  },
  "jump-game": {
    input: [1, 0, 1],
    markers: [null, "farther", null, "stuck"],
    py: [null, 7, null, 5],
    ts: [null, 8, null, 5],
  },
  "merge-intervals": {
    input: [1, 3, 2, 4],
    markers: [null, null, "open", "extend", null],
    py: [null, null, 7, 5, null],
    ts: [null, null, 8, 6, null],
  },
  "fibonacci-memo-table": {
    input: [3],
    markers: [null, null, null, null, "store", null, "store", "store", null, "hit", "store", null, "fill", "fill", null],
    py: [null, null, null, null, 8, null, 8, 8, null, 3, 8, null, 17, 17, null],
    ts: [null, null, null, null, 6, null, 6, 6, null, 3, 6, null, 14, 14, null],
  },
  "climbing-stairs": {
    input: [2],
    markers: [null, "one", "one", "two", null],
    py: [null, 5, 5, 7, null],
    ts: [null, 5, 5, 7, null],
  },
  "house-robber": {
    input: [2, 7],
    markers: [null, null, "choose", "trace", null],
    py: [null, null, 8, 14, null],
    ts: [null, null, 8, 16, null],
  },
  "maximum-subarray": {
    input: [1, -2, 3],
    markers: [null, "restart", "extend", "restart", "record", null],
    py: [null, 6, 8, 6, 10, null],
    ts: [null, 9, 12, 9, 15, null],
  },
  "coin-change": {
    input: [3, 1, 2],
    markers: [null, null, "improve", null, "improve", "improve", null, "improve", null],
    py: [null, null, 9, null, 9, 9, null, 9, null],
    ts: [null, null, 8, null, 8, 8, null, 8, null],
  },
  "knapsack-01": {
    input: [1, 1, 5],
    markers: [null, "skip", "skip", "take", "trace", null],
    py: [null, 7, 7, 9, 13, null],
    ts: [null, 7, 7, 9, 17, null],
  },
  "rod-cutting": {
    input: [1, 5],
    markers: [null, null, "improve", null, "improve", "improve", "trace", null],
    py: [null, null, 8, null, 8, 8, 12, null],
    ts: [null, null, 8, null, 8, 8, 16, null],
  },
  "grid-bfs": {
    input: [2, 2, 0, 0, 0, 1, 0, 0, 0, 0],
    markers: [null, "visit", "discover", "discover", "visit", "goal", null],
    py: [null, 12, 18, 18, 12, 14, null],
    ts: [null, 11, 19, 19, 11, 13, null],
  },
  "grid-dijkstra": {
    input: [2, 2, 0, 0, 0, 1, 0, 0, 0, 0],
    markers: [null, "pop", "relax", "relax", "pop", "goal", null],
    py: [null, 16, 26, 26, 16, 20, null],
    ts: [null, 16, 29, 29, 16, 21, null],
  },
  "a-star": {
    input: [2, 2, 0, 0, 0, 1, 0, 0, 0, 0],
    markers: [null, "pop", "relax", "relax", "pop", "goal", null],
    py: [null, 20, 30, 30, 20, 24, null],
    ts: [null, 16, 29, 29, 16, 21, null],
  },
  "greedy-best-first": {
    input: [2, 2, 0, 0, 0, 1, 0, 0, 0, 0],
    markers: [null, "pop", "discover", "discover", "pop", "goal", null],
    py: [null, 20, 26, 26, 20, 22, null],
    ts: [null, 16, 24, 24, 16, 18, null],
  },
  "maze-generation": {
    input: [2, 1],
    markers: [null, "carve", "carve", "carve", "back", "back", "back", "back", null],
    py: [null, 26, 26, 26, 29, 29, 29, 29, null],
    ts: [null, 24, 24, 24, 27, 27, 27, 27, null],
  },
  "floyd-warshall": {
    input: [3, 1, 0, 1, 1, 1, 2, 1],
    markers: [null, "pivot", "pivot", "improve", "pivot", null],
    py: [null, 9, 9, 13, 9, null],
    ts: [null, 7, 7, 11, 7, null],
  },
  "zero-one-bfs": {
    input: [3, 0, 0, 1, 0, 1, 2, 1],
    markers: [null, "pop", "front", "pop", null, "back", "pop", null, null],
    py: [null, 9, 14, 9, null, 16, 9, null, null],
    ts: [null, 6, 11, 6, null, 13, 6, null, null],
  },
  prim: {
    input: [2, 0, 0, 1, 5],
    markers: [null, "pop", "take", "push", "pop", "take", null],
    py: [null, 9, 14, 17, 9, 14, null],
    ts: [null, 9, 15, 18, 9, 15, null],
  },
  kruskal: {
    input: [3, 0, 0, 1, 1, 1, 2, 1, 0, 2, 1],
    markers: [null, "consider", "join", "consider", "join", "consider", "skip", null],
    py: [null, 12, 16, 12, 16, 12, 14, null],
    ts: [null, 15, 20, 15, 20, 15, 17, null],
  },
  scc: {
    input: [2, 1, 0, 1, 1],
    markers: [null, null, null, "finish", "finish", null, null, "assign", null, "assign", null],
    py: [null, null, null, 10, 10, null, null, 23, null, 23, null],
    ts: [null, null, null, 9, 9, null, null, 17, null, 17, null],
  },
  "bridges-articulation": {
    input: [2, 0, 0, 1, 1],
    markers: [null, "enter", "enter", "low", "bridge", null],
    py: [null, 9, 9, 20, 22, null],
    ts: [null, 8, 8, 18, 20, null],
  },
  "max-flow": {
    input: [2, 1, 0, 1, 4],
    markers: [null, null, "reach", null, "augment", null, null],
    py: [null, null, 17, null, 27, null, null],
    ts: [null, null, 15, null, 24, null, null],
  },
  "eulerian-path": {
    input: [2, 0, 0, 1, 1],
    markers: [null, "walk", "back", "back", null],
    py: [null, 20, 22, 22, null],
    ts: [null, 20, 22, 22, null],
  },
  "graph-representations": {
    input: [2, 0, 0, 1, 5],
    markers: [null, "list", "matrix", "list-back", null],
    py: [null, 5, 6, 8, null],
    ts: [null, 5, 6, 8, null],
  },
  bfs: {
    input: [3, 0, 0, 1, 1, 1, 2, 1],
    markers: [null, "visit", "discover", "visit", null, "discover", "visit", null, null],
    py: [null, 9, 12, 9, null, 12, 9, null, null],
    ts: [null, 7, 10, 7, null, 10, 7, null, null],
  },
  dfs: {
    input: [2, 0, 0, 1, 1],
    markers: [null, "enter", "go", "enter", null, "leave", "leave", null],
    py: [null, 7, 10, 7, null, 11, 11, null],
    ts: [null, 8, 11, 8, null, 14, 14, null],
  },
  "cycle-detection": {
    input: [2, 1, 0, 1, 1, 1, 0, 1],
    markers: [null, "grey", "grey", "back-edge", null],
    py: [null, 8, 8, 11, null],
    ts: [null, 9, 9, 12, null],
  },
  "topological-sort": {
    input: [2, 1, 0, 1, 1],
    markers: [null, "take", "lower", "ready", "take", null],
    py: [null, 13, 15, 17, 13, null],
    ts: [null, 12, 14, 16, 12, null],
  },
  "connected-components": {
    input: [3, 0, 0, 1, 1],
    markers: [null, "new", null, "fill", null, "new", null, null],
    py: [null, 7, null, 13, null, 7, null, null],
    ts: [null, 6, null, 12, null, 6, null, null],
  },
  "bipartite-check": {
    input: [3, 0, 0, 1, 1, 1, 2, 1],
    markers: [null, "start", "colour", null, "colour", null, null],
    py: [null, 9, 15, null, 15, null, null],
    ts: [null, 5, 12, null, 12, null, null],
  },
  dijkstra: {
    input: [2, 0, 0, 1, 4],
    markers: [null, "pop", "relax", "pop", null, null],
    py: [null, 9, 14, 9, null, null],
    ts: [null, 8, 14, 8, null, null],
  },
  "bellman-ford": {
    input: [3, 1, 0, 1, 3],
    markers: [null, null, "relax", null, "settled", null, null],
    py: [null, null, 11, null, 14, null, null],
    ts: [null, null, 10, null, 15, null, null],
  },
  "avl-rotations": {
    input: [1, 2, 3],
    markers: [null, "place", "place", "balance", "place", "balance", "balance", "rotate-left", null],
    py: [null, 37, 37, 43, 37, 43, 43, 28, null],
    ts: [null, 40, 40, 45, 40, 45, 45, 31, null],
  },
  "red-black-insert": {
    input: [1, 2],
    markers: [null, "place", "root-black", "place", "rotate-left", "root-black", null],
    py: [null, 42, 68, 42, 18, 68, null],
    ts: [null, 45, 68, 45, 21, 68, null],
  },
  "b-tree-insert": {
    input: [1, 2, 3],
    markers: [null, "add-key", "add-key", "add-key", "split", "new-root", null],
    py: [null, 14, 14, 14, 28, 38, null],
    ts: [null, 17, 17, 17, 32, 43, null],
  },
  "segment-tree": {
    input: [2, 1, 2, 1, 0, 1, 2, 0, 5],
    markers: [null, "leaf", "leaf", "merge", null, "inside", null, null, "set", "recompute", null],
    py: [null, 3, 3, 8, null, 15, null, null, 23, 30, null],
    ts: [null, 3, 3, 9, null, 17, null, null, 25, 31, null],
  },
  "lazy-propagation": {
    input: [2, 1, 0, 0, 3, 2, 0, 1, 0],
    markers: [null, null, "tag", "recompute", null, "inside", null, null],
    py: [null, null, 18, 24, null, 31, null, null],
    ts: [null, null, 18, 25, null, 31, null, null],
  },
  "fenwick-tree": {
    input: [2, 1, 0, 1, 2, 1, 0],
    markers: [null, null, "add", "add", null, "read", null, null],
    py: [null, null, 4, 4, null, 12, null, null],
    ts: [null, null, 3, 3, null, 10, null, null],
  },
  "lca-binary-lifting": {
    input: [1, 0, 1, 0],
    markers: [null, "table", "table", null, "lift", "same", null],
    py: [null, 11, 11, null, 21, 23, null],
    ts: [null, 12, 12, null, 23, 27, null],
  },
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
  "sieve-eratosthenes": [120],
  "fast-power": [999, 9999, 997],
  "pascal-ncr": [12],
  "convex-hull": [0, 0, 20, 0, 20, 20, 0, 20, 10, 10, 5, 5, 15, 15, 10, 0, 0, 10, 20, 10, 10, 20, 3, 17],
  "n-queens": [6],
  sudoku: [9, 0, 5, 2, 0, 4, 8, 0, 7, 9, 1, 9, 7, 0, 0, 5, 0, 0, 0, 0, 0, 3, 7, 1, 9, 6, 0, 5, 0, 0, 0, 0, 7, 0, 2, 0, 0, 2, 6, 8, 9, 3, 0, 0, 0, 0, 0, 1, 5, 8, 2, 6, 0, 9, 4, 0, 0, 0, 6, 0, 0, 0, 0, 2, 5, 7, 0, 0, 0, 2, 9, 1, 0, 0, 0, 4, 0, 9, 0, 5, 0, 7],
  "combination-sum": [12, 2, 3, 4, 5],
  "word-search": [5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 0, 0, 0, 1],
  kmp: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, 0, 0, 0, 1],
  "aho-corasick": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1, 0, 0, -1, 0, 0, 0, -1, 0, 0, 0, 0, 0],
  lcs: [0, 1, 2, 3, 4, 0, 1, 2, -1, 2, 0, 1, 3, 4, 0, 2, 1],
  "edit-distance": [0, 1, 2, 3, 4, 0, 1, 2, -1, 2, 0, 1, 3, 4, 0, 2, 1],
  "grid-paths": [8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  "word-break": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, -1, 0, -1, 0, 0, -1, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0],
  "bitmask-tsp": [5, 0, 3, 4, 2, 7, 3, 0, 4, 6, 3, 4, 4, 0, 5, 8, 2, 6, 5, 0, 6, 7, 3, 8, 6, 0],
  "activity-selection": [0, 3, 1, 4, 2, 5, 3, 6, 4, 7, 5, 8, 6, 9, 7, 10, 8, 11, 9, 12],
  "fractional-knapsack": [20, 3, 9, 4, 8, 5, 7, 6, 6, 7, 5, 8, 4],
  "merge-intervals": [0, 3, 1, 4, 2, 5, 10, 12, 11, 13, 15, 16, 17, 20, 18, 19, 21, 22, 23, 24],
  "fibonacci-memo-table": [12],
  "climbing-stairs": [15],
  "coin-change": [20, 1, 2, 3, 4, 5],
  "knapsack-01": [12, 1, 20, 2, 19, 3, 18, 4, 17, 5, 16, 6, 15],
  "grid-bfs": [12, 16, 0, 0, 11, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
  "grid-dijkstra": [12, 16, 0, 0, 11, 15, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0],
  "a-star": [12, 16, 0, 0, 11, 15, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0],
  "greedy-best-first": [12, 16, 0, 0, 11, 15, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0, 0, 9, 0, 0, 0, 0, 4, 0, 0, 0, 0, 7, 0, 0, 0, 0, 2, 0, 0, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 0, 3, 0, 0, 0, 0, 6, 0, 0, 0],
  "maze-generation": [7, 9999],
  "floyd-warshall": [6, 1, 0, 1, 9, 0, 2, 8, 0, 4, 7, 1, 2, 6, 1, 3, 5, 1, 5, 4, 2, 0, 3, 2, 3, 9, 2, 4, 8, 3, 1, 7, 3, 4, 6, 3, 5, 5, 4, 0, 4, 4, 2, 3, 4, 5, 9],
  "zero-one-bfs": [12, 0, 0, 1, 0, 0, 2, 1, 1, 3, 0, 2, 3, 1, 3, 4, 0, 4, 5, 1, 5, 6, 0, 6, 7, 1, 7, 8, 0, 8, 9, 1, 9, 10, 0, 10, 11, 1, 0, 11, 0, 1, 5, 1, 2, 7, 0, 3, 9, 1, 4, 10, 0, 6, 11, 1, 1, 8, 0, 2, 10, 1],
  prim: [12, 0, 0, 1, 1, 0, 2, 6, 1, 3, 2, 2, 3, 7, 3, 4, 3, 4, 5, 8, 5, 6, 4, 6, 7, 9, 7, 8, 5, 8, 9, 1, 9, 10, 6, 10, 11, 2, 0, 11, 7, 1, 5, 3, 2, 7, 8, 3, 9, 4, 4, 10, 9, 6, 11, 5, 1, 8, 1, 2, 10, 6],
  kruskal: [12, 0, 0, 1, 1, 0, 2, 6, 1, 3, 2, 2, 3, 7, 3, 4, 3, 4, 5, 8, 5, 6, 4, 6, 7, 9, 7, 8, 5, 8, 9, 1, 9, 10, 6, 10, 11, 2, 0, 11, 7, 1, 5, 3, 2, 7, 8, 3, 9, 4, 4, 10, 9, 6, 11, 5, 1, 8, 1, 2, 10, 6],
  scc: [12, 1, 0, 1, 1, 0, 2, 1, 1, 3, 1, 2, 3, 1, 3, 4, 1, 4, 5, 1, 5, 6, 1, 6, 7, 1, 7, 8, 1, 8, 9, 1, 9, 10, 1, 10, 11, 1, 0, 11, 1, 1, 5, 1, 2, 7, 1, 3, 9, 1, 4, 10, 1, 6, 11, 1, 1, 8, 1, 2, 10, 1],
  "bridges-articulation": [12, 0, 0, 1, 1, 0, 2, 1, 1, 3, 1, 2, 3, 1, 3, 4, 1, 4, 5, 1, 5, 6, 1, 6, 7, 1, 7, 8, 1, 8, 9, 1, 9, 10, 1, 10, 11, 1, 0, 11, 1, 1, 5, 1, 2, 7, 1, 3, 9, 1, 4, 10, 1, 6, 11, 1, 1, 8, 1, 2, 10, 1],
  "max-flow": [8, 1, 0, 1, 1, 0, 2, 5, 0, 3, 9, 1, 4, 4, 2, 4, 8, 2, 5, 3, 3, 5, 7, 4, 6, 2, 5, 6, 6, 1, 7, 1, 4, 7, 5, 5, 7, 9, 6, 7, 4, 3, 6, 8],
  "eulerian-path": [8, 0, 0, 1, 1, 1, 2, 1, 2, 3, 1, 3, 4, 1, 4, 5, 1, 5, 6, 1, 6, 7, 1, 7, 0, 1, 0, 2, 1, 2, 4, 1, 4, 6, 1, 6, 0, 1, 1, 3, 1, 3, 5, 1],
  "graph-representations": [12, 1, 0, 1, 1, 0, 2, 1, 1, 3, 1, 2, 3, 1, 3, 4, 1, 4, 5, 1, 5, 6, 1, 6, 7, 1, 7, 8, 1, 8, 9, 1, 9, 10, 1, 10, 11, 1, 0, 11, 1, 1, 5, 1, 2, 7, 1, 3, 9, 1, 4, 10, 1, 6, 11, 1, 1, 8, 1, 2, 10, 1],
  bfs: [12, 0, 0, 1, 1, 0, 2, 1, 1, 3, 1, 2, 3, 1, 3, 4, 1, 4, 5, 1, 5, 6, 1, 6, 7, 1, 7, 8, 1, 8, 9, 1, 9, 10, 1, 10, 11, 1, 0, 11, 1, 1, 5, 1, 2, 7, 1, 3, 9, 1, 4, 10, 1, 6, 11, 1, 1, 8, 1, 2, 10, 1],
  dfs: [12, 0, 0, 1, 1, 0, 2, 1, 1, 3, 1, 2, 3, 1, 3, 4, 1, 4, 5, 1, 5, 6, 1, 6, 7, 1, 7, 8, 1, 8, 9, 1, 9, 10, 1, 10, 11, 1, 0, 11, 1, 1, 5, 1, 2, 7, 1, 3, 9, 1, 4, 10, 1, 6, 11, 1, 1, 8, 1, 2, 10, 1],
  "cycle-detection": [12, 1, 0, 1, 1, 0, 2, 1, 1, 3, 1, 2, 3, 1, 3, 4, 1, 4, 5, 1, 5, 6, 1, 6, 7, 1, 7, 8, 1, 8, 9, 1, 9, 10, 1, 10, 11, 1, 0, 11, 1, 1, 5, 1, 2, 7, 1, 3, 9, 1, 4, 10, 1, 6, 11, 1, 1, 8, 1, 2, 10, 1],
  "topological-sort": [12, 1, 0, 1, 1, 0, 2, 1, 1, 3, 1, 2, 3, 1, 3, 4, 1, 4, 5, 1, 5, 6, 1, 6, 7, 1, 7, 8, 1, 8, 9, 1, 9, 10, 1, 10, 11, 1, 0, 11, 1, 1, 5, 1, 2, 7, 1, 3, 9, 1, 4, 10, 1, 6, 11, 1, 1, 8, 1, 2, 10, 1],
  "connected-components": [12, 0, 0, 1, 1, 0, 2, 1, 1, 3, 1, 2, 3, 1, 3, 4, 1, 4, 5, 1, 5, 6, 1, 6, 7, 1, 7, 8, 1, 8, 9, 1, 9, 10, 1, 10, 11, 1, 0, 11, 1, 1, 5, 1, 2, 7, 1, 3, 9, 1, 4, 10, 1, 6, 11, 1, 1, 8, 1, 2, 10, 1],
  "bipartite-check": [12, 0, 0, 1, 1, 0, 2, 1, 1, 3, 1, 2, 3, 1, 3, 4, 1, 4, 5, 1, 5, 6, 1, 6, 7, 1, 7, 8, 1, 8, 9, 1, 9, 10, 1, 10, 11, 1, 0, 11, 1, 1, 5, 1, 2, 7, 1, 3, 9, 1, 4, 10, 1, 6, 11, 1, 1, 8, 1, 2, 10, 1],
  dijkstra: [12, 0, 0, 1, 1, 0, 2, 8, 1, 3, 6, 2, 3, 4, 3, 4, 2, 4, 5, 9, 5, 6, 7, 6, 7, 5, 7, 8, 3, 8, 9, 1, 9, 10, 8, 10, 11, 6, 0, 11, 4, 1, 5, 2, 2, 7, 9, 3, 9, 7, 4, 10, 5, 6, 11, 3, 1, 8, 1, 2, 10, 8],
  "bellman-ford": [12, 1, 10, 11, 9, 9, 10, 8, 8, 9, 7, 7, 8, 6, 6, 7, 5, 6, 11, 9, 5, 6, 8, 4, 5, 7, 4, 10, 6, 3, 4, 5, 3, 9, 9, 2, 3, 8, 2, 7, 7, 2, 10, 6, 1, 3, 5, 1, 5, 9, 1, 8, 8, 0, 1, 7, 0, 2, 6, 0, 11, 5],
  // 8 values, 8 operations over (nearly) the whole range.
  "segment-tree": [8, 20, 19, 18, 17, 16, 15, 14, 13, ...[1, 0, 7, 2, 3, 20, 1, 1, 6, 2, 4, 0, 1, 0, 7, 2, 7, 9, 1, 2, 5, 1, 0, 6]],
  "lazy-propagation": [8, ...[1, 0, 7, 9, 2, 1, 6, 0, 1, 1, 6, 9, 2, 0, 7, 0, 1, 2, 5, 9, 2, 3, 4, 0, 1, 0, 3, 9, 2, 0, 7, 0]],
  "fenwick-tree": [12, ...[1, 0, 9, 1, 2, 9, 1, 4, 9, 1, 6, 9, 1, 10, 9, 2, 11, 0, 2, 10, 0, 2, 6, 0, 2, 9, 0, 2, 11, 0]],
  // A 12-node path: the deepest lifts.
  "lca-binary-lifting": [11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 11, 1, 10, 3, 11, 6],
};
