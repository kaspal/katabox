## Approach: all-pairs shortest paths (Floyd–Warshall)

With at most 100 ports and up to 10⁴ queries, compute the shortest time between **every** pair once:

1. `dist[i][i] = 0`; `dist[a][b] = dist[b][a] = min(current, minutes)` for each route (keep the fastest of parallel routes); everything else ∞.
2. For `k = 0 … n − 1`, for every `i, j`: `dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])`.

After step 2 with a given `k`, `dist[i][j]` is the shortest time using only ports `0..k` as stopovers; after the last `k`, any port. Each query is then a lookup, with ∞ reported as −1.

**Complexity:** O(n³ + m + q) time, O(n²) space: at most 10⁶ updates. Running Dijkstra for every query is also correct, but repeats work across queries.
