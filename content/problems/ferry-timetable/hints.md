There are many queries but few ports. Could you compute the answer for **every** pair of ports once, and then just look queries up?
---
Floyd–Warshall: `dist[i][j]` starts as the best direct route (0 on the diagonal, ∞ if none). Then allow the ports 0, 1, …, n − 1 one at a time as stopovers.
---
For each stopover `k`, for every `i, j`: `dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])`. After the last `k`, `dist` holds every shortest time; ∞ means unreachable (−1).
