## The idea

Quick sort picks one value, the **pivot**, and **partitions** the range: every value smaller than the pivot goes to its left, the rest to its right. After the partition the pivot sits in its final place, so the two sides can be sorted on their own, the same way. This version uses the Lomuto partition with the last value as the pivot.

## Steps

1. Take the last value of the range as the pivot.
2. Scan the range with `j`; `i` marks where the "smaller" part ends.
3. When `a[j]` is smaller than the pivot, swap it to position `i` and move `i` on.
4. Swap the pivot into position `i`: it is now in its final place.
5. Repeat on the left side and on the right side. A range of one value is already sorted.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n log n)` (every pivot splits the range in half) |
| Average | `O(n log n)` |
| Worst | `O(n²)` (every pivot is the smallest or biggest value) |
| Extra space | `O(log n)` stack on average, `O(n)` in the worst case |
| Stable | No: a swap can jump over equal values |
| In place | Yes |

## Pitfalls

- **Sorted or reversed input.** With the last value as pivot, already sorted input (and reversed input) makes every partition lopsided: one side is empty and the other holds everything else, so the work grows to `n²/2` comparisons. Try the "Reversed" input and watch the comparison count climb. Picking a random pivot, or the median of the first, middle and last values, fixes it.
- **Many equal values.** Lomuto sends every value equal to the pivot to the same side, so an array with few distinct values also degrades towards `n²`. Try the "Few unique" input. A three-way partition fixes it (see "Quick Sort: Hoare and 3-way", coming soon).
- **Off-by-one ranges.** The recursive calls are `lo .. p - 1` and `p + 1 .. hi`: the pivot itself is never included again.

## When to use it

Quick sort is the usual general-purpose sort for arrays in memory: it works in place and its inner loop is very fast. Use a random or median-of-three pivot, a three-way partition when there are many duplicates, and insertion sort for short ranges. When you need stability or a guaranteed `O(n log n)`, use merge sort or heap sort instead.

## Related topics

- [Insertion Sort](learn:sorting/insertion-sort): the usual choice for the short ranges at the bottom of the recursion.
- [Bubble Sort](learn:sorting/bubble-sort): a simple `O(n²)` sort to compare against.
- Quick Sort: Hoare and 3-way (coming soon).
