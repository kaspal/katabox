## The idea

Insertion sort grows a sorted part on the left, one value at a time, the way many people sort a hand of cards. It lifts the next value out (the **key**), slides every bigger value in the sorted part one place to the right to open a gap, and drops the key into the gap.

## Steps

1. The first value alone is a sorted part.
2. Lift the next value out as the `key`; its old place becomes a gap.
3. While the value left of the gap is bigger than the key, shift it right into the gap.
4. Drop the key into the gap.
5. Repeat until every value has been inserted.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n)` (already sorted: no shifts) |
| Average | `O(n²)` |
| Worst | `O(n²)` (reversed input) |
| Extra space | `O(1)` |
| Stable | Yes: the key never passes an equal value |
| In place | Yes |

## Pitfalls

- **Comparing with `>=`.** Moving the key past equal values breaks stability. The loop must stop at the first value that is not bigger.
- **Losing the key.** Shifting overwrites `a[i]`, so the key must be saved before the loop starts.
- The work is the number of out-of-order pairs: "Nearly sorted" input is almost free, "Reversed" is the worst case.

## When to use it

For small arrays (a few dozen values) and for data that is already nearly sorted, insertion sort is often the fastest choice. Real libraries switch to it for short ranges inside merge sort and quick sort. It also works **online**: it can sort values as they arrive.

## Related topics

- [Bubble Sort](learn:sorting/bubble-sort): the same `O(n²)` family, with swaps instead of shifts.
- [Quick Sort](learn:sorting/quick-sort): for large arrays.
