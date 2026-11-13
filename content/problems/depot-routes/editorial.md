## Approach: one Dijkstra, then lookups

Every call asks for a shortest path **from the same junction**, the base. A single-source shortest-path search from junction 1 answers all of them at once, so run it once, store the time of every junction, and answer each call with a lookup.

All street times are positive, so **Dijkstra's algorithm** applies:

1. Build adjacency lists: a street `a b w` adds `(b, w)` to junction `a` and `(a, w)` to junction `b`. Parallel streets need no special care; the faster one wins on its own.
2. Set `dist[1] = 0` and every other junction to infinity; push `(0, 1)` into a min-heap.
3. Pop the smallest pair `(d, u)`. If `d > dist[u]`, it is a stale entry: skip it. Otherwise, for each street `(v, w)` from `u`, if `d + w < dist[v]`, set `dist[v] = d + w` and push `(d + w, v)`.
4. When the heap is empty, `dist[t]` is the answer for junction `t`, or `-1` if it is still infinity.

**Why it is correct:** with positive times, the first time a junction is popped with a non-stale time, no shorter route to it can exist: any other route leaves the set of finished junctions through a junction whose time is already at least as large. Junctions never pushed have no route from the base.

**Why once:** a search per call costs O(Q·(N + M) log N), around 10⁹ heap steps at the limits. One search costs O((N + M) log N) and each call O(1).

**Heaps in each language:** Python has `heapq` and Rust `BinaryHeap` (wrap pairs in `Reverse` for a min-heap). TypeScript has no heap: write a small binary heap over two parallel arrays (times and junctions) with sift-up and sift-down. PHP's `SplPriorityQueue` is a max-heap, so insert each junction with the **negated** time as its priority.

**Complexity:** O((N + M) log N + Q) time, O(N + M) memory. The longest possible time is (N − 1)·10⁴ < 10⁸, so plain integers suffice everywhere.

**Fast input:** read everything at once and walk the tokens with a position (`sys.stdin.buffer.read().split()`, `readFileSync(0, "utf8")`, `stream_get_contents(STDIN)`, `read_to_string`), and print all answers in one write.
