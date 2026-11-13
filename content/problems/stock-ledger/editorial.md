## Approach: a Fenwick tree

A Fenwick tree over `n` positions stores in `tree[k]` the sum of a block of positions ending at `k` whose length is the lowest set bit of `k`. Two short loops give everything:

- **add(i, d):** for `k = i + 1`, then `k += k & −k`, while `k ≤ n`: `tree[k] += d`.
- **prefix(i)** (positions 0 to i − 1): for `k = i`, then `k −= k & −k`, while `k > 0`: add `tree[k]`.

Build it by adding each initial `stock[i]`. A `"set"` becomes `add(i, count − current[i])` (and update `current[i]`); a `"total"` is `prefix(b + 1) − prefix(a)`.

**Complexity:** O((n + q) log n) time, O(n) space. Summing the shelves for each `"total"` is O(n) per query, O(n·q) overall.
