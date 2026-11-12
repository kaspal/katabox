## Approach: count inversions with merge sort

**The answer is the number of inversions.** An inversion is a pair of positions `i < j` with `heights[i] > heights[j]`. Swapping two neighbours changes the relative order of only those two children, so the number of inversions changes by exactly one: down by one if they were an inversion, up by one otherwise. A sorted row has none, so at least `inversions` swaps are needed. Bubble sort, which only swaps neighbouring inversions, uses exactly that many. Equal heights are never an inversion and never need a swap.

**Counting them in O(n log n).** Merge sort splits the row in half, sorts each half recursively, and merges them. An inversion is either inside the left half, inside the right half (both counted by the recursive calls), or between the halves. During the merge, when the next smallest element comes from the right half while `k` elements of the left half are still waiting, those `k` elements are all taller and stand to its left: that is exactly `k` crossing inversions. Taking from the left on ties (`<=`) keeps equal heights from being counted.

```
sort(a):
    if len(a) <= 1: return a, 0
    left, x = sort(a[:mid]); right, y = sort(a[mid:])
    merge; whenever right[j] < left[i]: count += len(left) - i
    return merged, x + y + count
```

### Walkthrough

`[4, 3, 2, 1]`: the halves `[4, 3]` and `[2, 1]` have one inversion each and become `[3, 4]` and `[1, 2]`. Merging: 1 goes before both 3 and 4 (+2), 2 also (+2). Total 1 + 1 + 4 = 6.

**Complexity:** O(n log n) time and O(n) extra memory. The answer can reach about 5·10⁹, which needs 64-bit integers. Checking every pair is O(n²).
