## Approach: backtracking over non-decreasing bags

Sort `coins`. Explore bags with `pick(start, remaining)`:

- if `remaining == 0`, the current bag pays exactly: record a copy;
- otherwise, for `i` from `start` on, while `coins[i] ≤ remaining`: push `coins[i]`, call `pick(i, remaining − coins[i])`, pop.

Passing `i` (not `i + 1`) lets a coin repeat; never going back to smaller coins makes every bag non-decreasing, so each bag appears exactly once. Since the coins are tried in increasing order at every position, the bags come out in lexicographic order: no final sort is needed.

Stopping as soon as `coins[i] > remaining` (the coins are sorted) prunes the search.

**Complexity:** proportional to the number of partial bags explored, small under these limits (target ≤ 60 and at most 1000 answers).
