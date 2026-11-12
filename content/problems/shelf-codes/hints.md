Counting the smaller numbers one by one is up to 10⁵ steps per query, and there can be 10⁵ queries. The list is sorted: what does that tell you about where the smaller numbers are?
---
All numbers smaller than `x` form a prefix of the list. The answer is the position of the first number that is **at least** `x` (or the list length if there is none).
---
Find that position by binary search: keep a range `[lo, hi)` that contains the first position with `codes[i] >= x`, look at the middle and keep the half that still contains it. About 17 steps per query.
