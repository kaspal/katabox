Build each bag coin by coin. To avoid listing the same bag twice in another order, only ever add coins that are at least as large as the last one.
---
Sort the coins. A recursive function `pick(start, remaining, bag)` tries each coin from index `start` on, keeping the bag non-decreasing.
---
If `remaining` is 0, record a copy of the bag. Otherwise, for each coin `c = coins[i]` with `i ≥ start` and `c ≤ remaining`, add it, recurse with `start = i` (it can be reused), and remove it. Trying the coins in increasing order yields the bags already in lexicographic order.
