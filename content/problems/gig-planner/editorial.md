## Approach: dynamic programming over offers sorted by end, with binary search

Sort the offers by `end` (the input is in no particular order) and keep the sorted ends in a list `ends`. Let `best[i]` be the best plan that uses only the first `i` offers, written as the pair `(total, −count)`: a larger pair is a better plan, since a higher total wins and, for equal totals, fewer gigs give a larger `−count`.

For offer `i` (0-based) with `start` and `pay`, there are two choices:

- **skip it:** the best plan is `best[i]`;
- **take it:** every other gig in the plan must end by `start`. Among the first `i` offers those are exactly the first `j`, where `j = bisect_right(ends, start, 0, i)`, so the best plan is `best[j]` plus this gig: `(total + pay, −count − 1)`.

```
best[0] = (0, 0)
best[i + 1] = max(best[i], best[j] + (pay, −1))
```

The answer is `best[n]`, printed as `total` and `count`.

**Worked example (Example 1).** Sorted by end: `(1,4,50)`, `(1,6,80)`, `(4,6,30)`, `(6,9,20)`, so `ends = [4, 6, 6, 9]`.

| i | offer | j | take | best[i + 1] |
| --- | --- | --- | --- | --- |
| 0 | 1-4, 50 | 0 | (50, −1) | (50, −1) |
| 1 | 1-6, 80 | 0 | (80, −1) | (80, −1) |
| 2 | 4-6, 30 | 1 | (80, −2) | (80, −1) |
| 3 | 6-9, 20 | 3 | (100, −2) | (100, −2) |

At row 2 the plan `50 + 30` ties with `80` on total but has one more gig, so it loses; that is why the answer is `100 2` and not `100 3`.

**Why it is correct.** Take the best plan among the first `i + 1` offers. If it does not use offer `i`, it is a plan among the first `i`, so it is at most `best[i]`. If it does, its other gigs end by `start`; offers among the first `i` with a later end cannot be in it, and sorting by end puts all those that end by `start` before position `j`. So the rest is a plan among the first `j`, at most `best[j]`. Both choices are themselves valid plans, so the maximum is exact.

The tie-break can live inside the DP because adding a gig changes every plan the same way: if `(a, −c) ≥ (a′, −c′)` then `(a + pay, −c − 1) ≥ (a′ + pay, −c′ − 1)`. A plan that is best on the pair stays best after the step, so keeping only the best pair per prefix loses nothing.

**Complexity:** O(n log n) for the sort and one binary search per offer; O(n) memory.

**The obvious approach is too slow.** Computing the best plan that ends with each offer by trying every earlier offer is O(n²): with 5·10⁴ offers that is about 1.25·10⁹ comparisons, far beyond the limit.

**Practical notes:** read all input at once and print the single line once. Totals reach 10⁵ · 10⁹ = 10¹⁴: use 64-bit integers in Rust (`i64`/`u64`, not `i32`); a TypeScript `Number` is exact up to 2⁵³ and PHP integers are 64-bit. Python compares tuples directly; in the other languages compare the total first, then the count. The binary search must find the first offer that ends **after** `start` (`<=` in the test), so that a gig ending exactly at `start` counts as compatible.
