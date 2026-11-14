## The idea

Selection sort splits the array into a sorted part on the left and an unsorted part on the right. Each pass scans the whole unsorted part for its smallest value and swaps it to the front of that part, where it stays for good. After `n − 1` passes the last value is already in place, so the array is sorted.

## Steps

1. Start a pass at position `i`: assume `a[i]` is the smallest so far (`smallest = i`).
2. Compare every later value `a[j]` with `a[smallest]`.
3. When `a[j]` is smaller, remember its position: `smallest = j`.
4. At the end of the scan, swap `a[i]` and `a[smallest]`. Position `i` is now final.
5. Move `i` one place right and repeat until one value is left.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n²)` (the scan never stops early) |
| Average | `O(n²)` |
| Worst | `O(n²)` |
| Extra space | `O(1)` |
| Stable | No: the long swap can jump over an equal value |
| In place | Yes |

## Pitfalls

- **Expecting sorted input to be fast.** Selection sort always makes `n(n − 1)/2` comparisons: 45 for the 10 default values, whatever their order. Try "Nearly sorted" and then "Reversed": the comparison count stays the same.
- **Swapping on every new minimum.** Swap once, at the end of the pass. Swapping inside the scan does extra work and turns it into a clumsy bubble sort.
- **Assuming it is stable.** In `[5, 5, 2]` the first pass swaps 2 with the first 5, which lands after the second 5. The original order of equal values is lost.

## When to use it

Selection sort makes at most `n − 1` swaps, fewer than any other simple sort, so it can make sense when writing a value costs much more than reading one. Otherwise insertion sort is just as short and adapts to nearly sorted data. The idea of "pick the smallest, then repeat" comes back in heap sort, which finds each next value with a heap instead of a full scan.

## Related topics

- [Insertion Sort](learn:sorting/insertion-sort): the same `O(n²)` family, but fast on nearly sorted input.
- [Bubble Sort](learn:sorting/bubble-sort): many swaps per pass instead of one.
- [Heap Sort](learn:sorting/heap-sort): selection with a heap, in `O(n log n)`.
