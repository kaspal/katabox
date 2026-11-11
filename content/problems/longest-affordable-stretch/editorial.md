## Approach: a sliding window

Because costs are non-negative, extending a stretch never makes it cheaper. So scan the stages with a right end and keep the shortest-possible left end:

1. Add `costs[right]` to the window total.
2. While the total is over `budget`, remove `costs[left]` and move `left` one step right.
3. Now `[left, right]` is the longest valid stretch ending at `right`: update the answer with `right - left + 1`.

If one stage alone is over budget, the loop empties the window (`left = right + 1`) and the length is 0.

### Walkthrough

`costs = [4, 2, 1, 6, 1, 1]`, `budget = 8`:

| right | total after adding | shrink | window | length |
| --- | --- | --- | --- | --- |
| 0 | 4 | – | [0, 0] | 1 |
| 1 | 6 | – | [0, 1] | 2 |
| 2 | 7 | – | [0, 2] | 3 |
| 3 | 13 | drop 4, 2 → 7 | [2, 3] | 2 |
| 4 | 8 | – | [2, 4] | 3 |
| 5 | 9 | drop 1 → 8 | [3, 5] | 3 |

The best length is 3.

**Complexity:** O(n) time (each stage enters and leaves the window once), O(1) extra space. Trying every start and extending it is O(n²) when the budget is large.
