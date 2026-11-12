## Approach: a min-heap of the best k

Walk through the scores and keep the best `k` seen so far in a **min-heap**, whose top is the smallest kept score:

- while the heap has fewer than `k` scores, push the new one;
- otherwise, if the new score beats the top, replace the top with it.

At the end the heap holds exactly the `k` highest scores (with repeats). Sort them from highest to lowest.

```
heap = []
for s in scores:
    if len(heap) < k: push(heap, s)
    elif s > heap.top: replace_top(heap, s)
return heap sorted descending
```

### Walkthrough

`scores = [132, 201, 87, 201, 156, 99]`, `k = 3`: the heap becomes {132}, {132, 201}, {87, 132, 201}. Then 201 replaces 87, 156 replaces 132, and 99 is too low. Sorted: `[201, 201, 156]`.

**Complexity:** O(n log k) time and O(k) extra space. Sorting all scores is O(n log n) and is fine here too; the heap shows the pattern that scales when `k` is small and the scores keep coming.
