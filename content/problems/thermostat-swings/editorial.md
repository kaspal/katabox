## Approach: two monotonic queues

For the **maximum**, keep a double-ended queue of indices whose readings are strictly decreasing from front to back:

```
for i, r in enumerate(readings):
    while maxq and readings[maxq[-1]] <= r: maxq.pop()   # they can never be the max again
    maxq.append(i)
    if maxq[0] <= i - k: maxq.popleft()                   # it left the window
```

When the window ends at `i` (that is, `i ≥ k − 1`), `readings[maxq[0]]` is its maximum. Every index removed from the back is dominated: a later reading at least as large stays in the window at least as long. So the front is always the largest reading still in the window.

Run the same logic with `>=` for the **minimum** in a second queue, and output `max − min` for each window.

### Walkthrough

`[3, 1, 4, 1, 5, 9, 2, 6]`, k = 3. For the first window, the max-queue ends as [4] and the min-queue as [1, 4] (values): swing 3. At index 5 (value 9) the max-queue becomes [9] and the min-queue [1, 5, 9], so the window [1, 5, 9] swings 8.

**Complexity:** O(n) time, since each index enters and leaves each queue once, and O(k) memory. Scanning each window is O(n · k).
