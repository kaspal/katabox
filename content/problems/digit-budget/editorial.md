## Approach: digit DP

Write `limit` as its digits `d[0..L)`, most significant first. Count numbers from 0 to `limit` (as L-digit strings with leading zeros, which do not change the digit sum) whose digits add up to `total`:

```
count(pos, sum, tight):
    if sum > total: return 0
    if pos == L: return 1 if sum == total else 0
    top = d[pos] if tight else 9
    return Σ_{x = 0..top} count(pos + 1, sum + x, tight and x == top)
```

The states with `tight = false` depend only on `(pos, sum)` and repeat constantly, so memoise them: at most 16 × 136 states with 10 choices each. The tight path is a single chain. Since `total ≥ 1`, the number 0 (digit sum 0) is never counted, so the answer is exactly the tickets 1..limit.

**Complexity:** O(L · total · 10) time, about 2·10⁴ steps. Checking every number up to `limit` is O(limit · L).
