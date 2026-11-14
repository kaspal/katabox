## The idea

Bubble sort walks along the array comparing **neighbours**. Whenever a value is bigger than the one to its right, the two swap places. After one full pass the biggest value has been carried all the way to the end, like a bubble rising to the surface, and it never moves again. Each following pass can stop one position earlier.

## Steps

1. Start a pass over the unsorted part, from the first position to the last unsorted one.
2. Compare `a[j]` with `a[j + 1]`.
3. If `a[j]` is bigger, swap them.
4. At the end of the pass, the last unsorted value is in its final place.
5. If a whole pass made no swap, the array is sorted: stop early.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n)` (already sorted: one pass, no swaps) |
| Average | `O(n²)` |
| Worst | `O(n²)` (reversed input) |
| Extra space | `O(1)` |
| Stable | Yes: equal values never swap |
| In place | Yes |

## Pitfalls

- **Forgetting the early exit.** Without the `swapped` flag, sorted input still costs `n²/2` comparisons. Try the "Nearly sorted" input and watch how few passes it needs.
- **Swapping on equal values.** Using `>=` instead of `>` makes the sort unstable and does useless work on the "Few unique" input.
- Every swap fixes exactly one pair that is out of order, so the number of swaps equals the number of such pairs (inversions). Reversed input has the most.

## When to use it

Almost never for real data: insertion sort does the same job with fewer writes. Bubble sort is worth knowing because its swap count measures how unsorted an array is, and because "swap neighbours until nothing changes" is the idea behind several puzzle problems.

## Related topics

- [Insertion Sort](learn:sorting/insertion-sort): also `O(n²)`, but shifts instead of swapping and is faster in practice.
- [Quick Sort](learn:sorting/quick-sort): `O(n log n)` on average.
