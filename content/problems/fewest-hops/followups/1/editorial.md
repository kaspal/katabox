## Idea

Breadth-first search visits the stations in order of distance from station `0`, so by the
time a station `u` is taken from the queue, every fewest route to `u` has been counted:
each of them ends with a hop from a station one step closer, and all of those were taken
from the queue earlier.

Keep `dist` (−1 for unseen) and `ways`, with `dist[0] = 0` and `ways[0] = 1`. For each
station `u` taken from the queue and each neighbour `v`:

- `dist[v] = −1`: first time seen. Set `dist[v] = dist[u] + 1`, `ways[v] = ways[u]`, and
  queue `v`.
- `dist[v] = dist[u] + 1`: another fewest way in. Add `ways[u]` to `ways[v]`.
- otherwise the hop does not lie on a fewest route; ignore it.

The answer is `ways[n − 1]` (0 when `n − 1` was never reached). Every addition is reduced
modulo 1 000 003, so values stay below 2·10⁶, safe in every language.

## Complexity

O(n + m) time for `m` links and O(n + m) memory for the adjacency lists. Enumerating routes
instead is exponential: a ladder of 2 stations per level has 2 to the power of its number of
levels.
