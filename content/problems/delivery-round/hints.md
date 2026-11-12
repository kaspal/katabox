Trying every order of the 11 other stops means 11! ≈ 4·10⁷ rounds, each summed over 12 rides. Many orders share the same beginning: once the courier has visited a certain **set** of stops and stands at a certain stop, the order used to get there no longer matters for the rest.
---
Let the state be `(visited set, current stop)`. The cheapest way to finish from that state (visit the remaining stops and return to 0) depends only on the state. There are 2ⁿ · n states.
---
Encode the visited set as a bitmask. `best(mask, last)` = `dist[last][0]` when every stop is visited, else the minimum over unvisited `j` of `dist[last][j] + best(mask | 1 << j, j)`. Memoise it; the answer is `best(1, 0)`.
