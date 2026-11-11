## Approach: two pointers from both ends

Put `lo` on the first item and `hi` on the last, and keep the best total seen (start at `-1`).

- If `prices[lo] + prices[hi] > budget`, the item at `hi` is too expensive even with the cheapest remaining partner, so it can be discarded: `hi -= 1`.
- Otherwise the pair fits. Record the total; every other partner of `lo` to the left of `hi` gives a smaller total, so `lo` has found its best partner: `lo += 1`.

Each step discards one item, so the loop runs fewer than n times.

### Walkthrough

`prices = [3, 5, 8, 12, 20]`, `budget = 19`:

| lo | hi | total | action | best |
| --- | --- | --- | --- | --- |
| 3 | 20 | 23 | over: hi left | -1 |
| 3 | 12 | 15 | fits: lo right | 15 |
| 5 | 12 | 17 | fits: lo right | 17 |
| 8 | 12 | 20 | over: hi left | 17 |

`lo` and `hi` meet; the answer is 17.

**Complexity:** O(n) time, O(1) extra space. Checking every pair is O(n²), too slow for 5·10⁴ prices.
