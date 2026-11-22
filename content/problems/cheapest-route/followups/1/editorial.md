## Idea

The road limit breaks Dijkstra: the cheapest way into a town may use too many roads to
finish in time. Bellman-Ford counts roads naturally. Let `best[v]` be the cheapest toll to
`v` using at most `r` roads; at `r = 0` only `start` costs 0. One round tries every road
`[a, b, t]` both ways: `next[b] = min(next[b], best[a] + t)` and the same for `a`. Reading
from `best` and writing to a copy `next` makes each round add at most one road. After `k`
rounds, `best[goal]` is the answer (or `-1` if it is still infinite). If a round changes
nothing, later rounds will not either, so you can stop early.

## Complexity

O(k · (n + m)) time for `m` roads (at most 50 · 10⁴ relaxations per direction) and O(n)
extra memory.
