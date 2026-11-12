## Approach: count by the last piece

Every paving of a path of length `i` ends with exactly one of three pieces, and removing it leaves a paving of a shorter path:

```
ways[0] = 1                       # the empty path: one way (lay nothing)
ways[i] = ways[i − 1]             # ends with a square
        + ways[i − 2]             # ends with a short slab   (if i ≥ 2)
        + ways[i − 3]             # ends with a long slab    (if i ≥ 3)
```

Different last pieces give different sequences, and every sequence is counted once, so the sum is exact. Compute `i` from 1 to `n`, keeping each value modulo 1 000 000 007; only the last three values are needed at any time.

### Walkthrough

`ways = 1, 1, 2, 4, 7, 13` for lengths 0 to 5: for instance `ways[4] = 4 + 2 + 1 = 7` and `ways[5] = 7 + 4 + 2 = 13`.

**Complexity:** O(n) time and O(1) memory with three running values. The plain recursion recomputes the same lengths and grows like 1.84ⁿ.
