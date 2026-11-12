## Approach: backtracking, taking before leaving, with pruning

Go through the stamps in index order and decide for each one: take it or leave it. A recursive `search(i, missing)` explores the choices for stamps `i, i + 1, …` while `missing` cents are still needed, keeping the chosen indices in a list:

```
search(i, missing):
    if missing == 0:
        if best is None or len(chosen) < len(best): best = copy of chosen
        return
    if i == n or missing > suffix[i]: return          # cannot reach it any more
    if best and len(chosen) + 1 >= len(best): return  # cannot beat the best
    if values[i] <= missing:
        chosen.append(i); search(i + 1, missing - values[i]); chosen.pop()
    search(i + 1, missing)
```

`suffix[i]` is the sum of `values[i:]`.

**Why taking first gives the right tie-break.** Take two index lists `A` and `B` of the same length, where `A` is lexicographically smaller. At the first position where they differ, `A` has a smaller index `a`. The search reaches index `a` with the same earlier choices for both lists, and it explores "take `a`" (towards `A`) before "leave `a`" (towards `B`). So among equally short lists, the first one found is the lexicographically smallest. Replacing the answer only on **strictly** fewer stamps keeps it.

**Pruning** cuts whole subtrees: when even all remaining stamps are not enough, when a stamp is too valuable to take, and when the current list is already as long as the best one.

### Walkthrough

`values = [5, 3, 8, 2, 6]`, target 11. Taking 5 then 3 leaves 3 missing; nothing fits exactly (8 is too big, then 2 leaves 1, and 6 is too big), so the search backs up. 5 + 6 completes: best = [0, 4]. From then on only single-stamp answers could win, and none is worth 11, so 3 + 8 = [1, 2] is never kept.

**Complexity:** O(2ⁿ) calls in the worst case (n ≤ 18), much fewer with pruning, and O(n) recursion depth.
