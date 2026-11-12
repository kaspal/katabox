For one dispute you can walk up from `a`, marking every boss, and then walk up from `b` until you hit a marked person. With 5·10⁴ disputes in a hierarchy 5·10⁴ levels deep, that is far too slow. You need to climb many levels at once.
---
First compute every member's depth (levels below the president). Bring the deeper member up to the depth of the other; then, if they differ, move both up **together** until their bosses coincide. Climbing `d` levels can be done in O(log d) jumps if you know each member's boss 1, 2, 4, 8, … levels up.
---
**Binary lifting**: `up[0][v] = boss[v]` (the president points to themselves) and `up[k][v] = up[k − 1][up[k − 1][v]]`. To equalise depths, jump by the powers of two in the depth difference. Then for `k` from high to low: if `up[k][a] ≠ up[k][b]`, jump both. The answer is `up[0][a]` (or `a` if the two were already equal).
