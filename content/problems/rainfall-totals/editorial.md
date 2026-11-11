## Approach: prefix sums

Compute running totals once: `prefix[0] = 0` and `prefix[d + 1] = prefix[d] + rain[d]`, so `prefix[k]` is the rain of the first `k` days. The rain from `first` to `last` is the rain of the first `last + 1` days minus the rain of the first `first` days: `prefix[last + 1] - prefix[first]`.

### Walkthrough

`rain = [3, 0, 7, 2, 5]` gives `prefix = [0, 3, 3, 10, 12, 17]`.

| query | computation | answer |
| --- | --- | --- |
| [0, 2] | prefix[3] − prefix[0] = 10 − 0 | 10 |
| [1, 3] | prefix[4] − prefix[1] = 12 − 3 | 9 |
| [4, 4] | prefix[5] − prefix[4] = 17 − 12 | 5 |

**Complexity:** O(n + q) time and O(n) extra space, for n days and q queries. Summing each range directly is O(n·q) in the worst case.
