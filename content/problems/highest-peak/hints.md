Looking at one marker and its right neighbour, can you tell which side of the summit you are on?
---
If `heights[m] < heights[m + 1]`, you are still climbing: the summit is to the right of `m`. Otherwise you are at the summit or descending: the summit is at `m` or to its left.
---
Binary search with `lo = 0`, `hi = n − 1`: while `lo < hi`, take `m = (lo + hi) / 2`; climbing → `lo = m + 1`, else `hi = m`. The answer is `lo`.
