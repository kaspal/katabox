## Approach: union-find with a running count

Keep `clubs = n`. For each merge `[a, b]`, find the representatives of both players; if they differ, join the two clubs and decrease `clubs`. Append `clubs` after every merge.

The representatives come from a **union-find** (disjoint-set) structure:

```
find(x):                       # follow parents to the root, compressing the path
    while parent[x] != x:
        parent[x] = parent[parent[x]]
        x = parent[x]
    return x
union(a, b):
    ra, rb = find(a), find(b)
    if ra == rb: return False
    attach the smaller tree under the larger one
    return True
```

With path compression and union by size, each operation costs almost constant amortised time (the inverse Ackermann function α).

### Walkthrough

Example 1: `[0, 1]` joins two clubs (4 left), `[3, 4]` too (3), `[1, 0]` finds the same root (still 3), `[1, 4]` joins {0, 1} and {3, 4} (2).

**Complexity:** O((n + m) α(n)) time and O(n) memory. Relabelling a whole club on every merge is O(n) per merge, O(n · m) in all.
