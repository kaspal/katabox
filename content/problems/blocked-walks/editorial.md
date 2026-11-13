## Approach: inclusion–exclusion over the first closed crossing

The number of walks from `(a, b)` to `(c, d)` with `a ≤ c`, `b ≤ d` is `paths = C((c − a) + (d − b), c − a)`. With factorials and inverse factorials up to `rows + cols` (2·10⁵ < p, so all inverses exist), each `paths` is O(1).

Sort the closed crossings by row, then by column, and append the destination. A crossing can only be reached from crossings earlier in this order. Define `good[i]` as the walks from the start to point `i` that touch no closed crossing before it:

`good[i] = paths(start → i) − Σ over j < i with r_j ≤ r_i and c_j ≤ c_i of good[j] · paths(j → i)`.

Every walk to `i` that touches a closed crossing has a **first** one, `j`, and is counted exactly once in the sum (its part up to `j` is one of the `good[j]` walks, the rest is any walk from `j` to `i`). The answer is `good` of the destination.

### Walkthrough

Example 1: the closed crossing (1, 1) has `good = paths((0,0) → (1,1)) = 2`. The destination has `paths = 6`, minus `2 · paths((1,1) → (2,2)) = 2 · 2`: 6 − 4 = 2.

**Complexity:** O(rows + cols + k²) time with k ≤ 1000 closed crossings. The grid DP is O(rows · cols), 10¹⁰ steps.
