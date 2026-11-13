## Approach: a Fenwick tree over the differences

Let `diff` start at all zeros. Adding `delta` to gauges `a..b` changes the differences at two places: `diff[a] += delta` and `diff[b + 1] −= delta` (if `b + 1` is a gauge). The correction of gauge `i` is then `diff[0] + … + diff[i]`, a prefix sum.

A Fenwick tree keeps `diff` so that both a point update and a prefix sum take O(log n):

- `"add" [a, b, delta]`: `add(a, delta)`, and `add(b + 1, −delta)` when `b + 1 < n`.
- `"read" [i]`: `readings[i] + prefix(i + 1)`.

### Walkthrough

`readings = [10, 20, 30, 40]`: `"add" [1, 3, 5]` puts +5 at 1. `"read" [2]` is 30 + (0 + 5 + 0) = 35. `"add" [0, 1, −2]` puts −2 at 0 and +2 at 2. `"read" [1]` is 20 + (−2 + 5) = 23.

**Complexity:** O(q log n) time, O(n) space. Updating every gauge of the range is O(n) per `"add"`.
