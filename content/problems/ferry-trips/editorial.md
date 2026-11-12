## Approach: greedy pairing with two pointers

Sort the weights. Consider the heaviest hiker `H` and the lightest `L`:

- If `L + H > limit`, nobody can share with `H`, so `H` crosses alone.
- Otherwise `H` and `L` share a trip. This never hurts: in any plan where `L` travels with someone else `X`, swapping `L` and `H`'s companion keeps every trip within the limit (because `X ≤ H`).

Both cases remove `H`, so each step is one trip:

```
sort weights; lo = 0; hi = n - 1; trips = 0
while lo <= hi:
    if weights[lo] + weights[hi] <= limit: lo += 1
    hi -= 1; trips += 1
```

### Walkthrough

`weights = [60, 45, 80, 35]` sorted is `[35, 45, 60, 80]`, `limit = 100`:

| lo | hi | pair fits? | trip |
| --- | --- | --- | --- |
| 35 | 80 | 115 > 100: no | 80 alone |
| 35 | 60 | 95: yes | 60 + 35 |
| 45 | 45 | (same hiker) | 45 alone |

3 trips.

**Complexity:** O(n log n) for sorting, then O(n). Looking through all remaining hikers for each one's best companion is O(n²).
