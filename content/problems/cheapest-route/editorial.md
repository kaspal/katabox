## Approach: Dijkstra's algorithm with a heap

Build an adjacency list (each road both ways). Keep `dist[t]`, the cheapest known cost to each town, starting at ∞ except `dist[start] = 0`, and a min-heap with `(0, start)`.

Repeatedly pop the cheapest `(d, t)`:

- if `d > dist[t]`, it is an outdated entry: skip it;
- if `t == goal`, return `d`;
- otherwise, for each road `(t, u, toll)`, if `d + toll < dist[u]`, set `dist[u] = d + toll` and push it.

If the heap empties first, `goal` is unreachable: return `-1`.

Because tolls are positive, the first time a town is popped its cost is final.

### Walkthrough

Example 1, from town 0:

| pop | improves | heap after |
| --- | --- | --- |
| (0, 0) | 1 → 5, 2 → 2 | (2, 2), (5, 1) |
| (2, 2) | 3 → 8, 1 → 3 | (3, 1), (5, 1), (8, 3) |
| (3, 1) | 3: 3 + 5 = 8, not cheaper | (5, 1), (8, 3) |
| (5, 1) | outdated | (8, 3) |
| (8, 3) | goal | |

**Complexity:** O((n + m) log m) time and O(n + m) space. Bellman-Ford (relax every road, round after round, until nothing changes) is O(n·m): up to 10⁹ steps when the cheapest route has many roads.
