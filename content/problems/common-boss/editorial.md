## Approach: depths and binary lifting

The closest common boss is the **lowest common ancestor** (LCA) of `a` and `b` in the tree given by `boss`.

**Preparation.** Build children lists and walk breadth-first from the president to get every member's `depth` (the president has depth 0). Then build the jump table:

```
up[0][v] = boss[v]   (the president: themselves)
up[k][v] = up[k − 1][ up[k − 1][v] ]    # the boss 2^k levels above v
```

with `k` up to ⌈log₂ n⌉. Each level of the table is computed from the previous one in O(n).

**One query.**

1. Let `a` be the deeper member. Lift it by `d = depth[a] − depth[b]` levels: for every bit `k` set in `d`, `a = up[k][a]`. Now both are at the same depth.
2. If `a == b`, that member is the answer (one was above the other).
3. Otherwise, for `k` from the highest level down to 0: if `up[k][a] != up[k][b]`, set `a = up[k][a]`, `b = up[k][b]`. These jumps never pass the common ancestor, because they only happen while the ancestors still differ. In the end `a` and `b` are the two different children just below the answer.
4. The answer is `up[0][a]`.

### Walkthrough

Example 1, query (3, 5): depths 2 and 2. `up[1][3] = 0 = up[1][5]`, so no jump at level 1; `up[0][3] = 1 ≠ 2 = up[0][5]`, so jump to a = 1, b = 2. Answer `up[0][1] = 0`.

**Complexity:** O(n log n) to build the table and O(log n) per query, with O(n log n) memory. Walking up from both members for every query is O(q · n) in a deep hierarchy.
