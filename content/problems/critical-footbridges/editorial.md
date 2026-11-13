## Approach: DFS with low-link values (Tarjan)

Run a DFS, numbering islands in visiting order: `tin[v]`. Define `low[v]` as the smallest `tin` reachable from `v`'s DFS subtree using tree footbridges downward and at most one non-tree footbridge (a "back edge") upward.

When the DFS returns from child `c` to parent `p` through footbridge `e`:

- `low[p] = min(low[p], low[c])`;
- if `low[c] > tin[p]`, no island under `c` can get back to `p` or above without `e`: **`e` is critical**.

A non-tree footbridge `v–u` to an already visited island updates `low[v] = min(low[v], tin[u])`. Only the footbridge used to reach `v` is skipped, by its index.

A recursive DFS would overflow on a chain of 10⁵ islands, so the DFS keeps an explicit stack of `(island, next neighbour index, footbridge used)`. Write each found footbridge as `[min, max]` and sort.

**Complexity:** O(n + m) time and space. Closing each footbridge and re-checking connectivity is O(m·(n + m)).
