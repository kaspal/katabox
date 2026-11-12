## Approach: two heaps around the middle

Keep two heaps:

- `low`, a **max-heap** with the smaller half of the readings;
- `high`, a **min-heap** with the larger half;

with the invariant that every value in `low` is ≤ every value in `high`, and that `low` has the same size as `high` or one more.

For each new reading `x`:

1. If `low` is empty or `x ≤ max(low)`, push `x` into `low`; otherwise into `high`.
2. Rebalance: if `low` has two more than `high`, move `max(low)` to `high`; if `high` has more than `low`, move `min(high)` to `low`.
3. The median is `max(low)` when the count is odd, and `(max(low) + min(high)) / 2` when it is even.

Moving the top of one heap to the other keeps the ordering invariant, because that top is the value closest to the other half.

### Walkthrough

`readings = [12, -4, 30, 7]`:

| reading | low | high | median |
| --- | --- | --- | --- |
| 12 | {12} | {} | 12 |
| −4 | {−4} | {12} | (−4 + 12) / 2 = 4 |
| 30 | {−4, 12} | {30} | 12 |
| 7 | {−4, 7} | {12, 30} | (7 + 12) / 2 = 9.5 |

(At reading 30, `low` got 12 back from `high` to restore the sizes; at reading 7, 12 moved up again.)

The medians are exact halves of integers, so they print as exact decimals in every language; the 10⁻⁶ tolerance is only a safety margin.

**Complexity:** O(n log n) time and O(n) space. Sorting every prefix again is O(n² log n), and inserting into a sorted list is O(n²) element moves.
