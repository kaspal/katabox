## The bug

The sweep keeps a current block `[cur_s, cur_e]` and either extends its end or closes it and opens a new one. That only works when shifts arrive in order of their start, because the block's start is never moved back. The starter sorts by the wrong key:

```python
    for s, e in sorted(shifts, key=lambda x: x[1]):
```

Sorted by end, a shift that starts before the current block can arrive later. It overlaps, so only `cur_e` is updated, and the time between its start and `cur_s` is never counted.

With `shifts = [[1, 10], [2, 3]]`, the order becomes `[2, 3]`, `[1, 10]`. The block starts as 2 to 3, then `[1, 10]` overlaps and stretches it to 2 to 10, so the starter returns 8. The correct answer is 9 (1 to 10).

## The fix

Sort by start (Python's default ordering of lists compares the start first):

```python
    for s, e in sorted(shifts):
```

In TypeScript, sort with `(x, y) => x[0] - y[0]` instead of comparing the ends.

**Complexity:** O(n log n) time for the sort, O(n) extra space for the sorted copy.
