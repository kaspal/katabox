## The idea

Lomuto's partition in [Quick Sort](learn:sorting/quick-sort) sends every value equal to the pivot to one side, so duplicates make it slow. The **3-way partition** (the "Dutch national flag" scheme) splits the range into three parts: smaller than the pivot, equal to it, and bigger. The equal block is finished in one pass. Here the pivot is the first value, and three pointers hold the parts: `a[lo..lt-1]` is smaller, `a[lt..i-1]` equal, `a[i..gt]` unchecked and `a[gt+1..hi]` bigger.

### Hoare's partition

Hoare's original scheme (not animated here) moves two indices inward from the ends of the range. The left one stops at a value not smaller than the pivot, the right one at a value not bigger; the pair is swapped and both move on, until they cross. Each swap fixes two values at once, so it swaps less than Lomuto, and since both indices stop at equal values, duplicates are shared between the sides. But the pivot doesn't necessarily end in its final place: the range is split where the indices crossed, and both sides are sorted again.

## Steps

1. Take `pivot = a[lo]`; set `lt = lo`, `i = lo + 1`, `gt = hi`.
2. While `i <= gt`, look at `a[i]`.
3. Smaller: swap it with `a[lt]`, then move both `lt` and `i` on.
4. Bigger: swap it with `a[gt]` and move `gt` back. `i` stays, because the value that came from `gt` is unchecked.
5. Equal: just move `i` on.
6. Sort `lo..lt-1` and `gt+1..hi` the same way; `lt..gt` is done.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n)` (all values equal: one pass) |
| Average | `O(n log n)` |
| Worst | `O(n²)` (sorted or reversed input, with the first value as pivot) |
| Extra space | `O(log n)` stack on average, `O(n)` in the worst case |
| Stable | No: swaps jump over equal values |
| In place | Yes |

## Pitfalls

- **Moving `i` after a swap with `gt`.** The value that arrives at `i` is unchecked, so it must be compared first. After a swap with `lt` the value that lands at `i` came from the equal block (or is the same value, when `lt` = `i`), so it is already checked and `i` moves on.
- **Sorted input.** With the first value as pivot, sorted or reversed input leaves one side empty every time, as in Lomuto. A random pivot fixes it.
- **Duplicates are the point.** Try "Few unique": the recursion never goes deeper than the number of distinct values.

## When to use it

Lomuto is the simplest to write, but it swaps the most and suffers from duplicates. Hoare swaps the least and suits mostly distinct values. The 3-way partition wins when keys repeat a lot: grades, ages, categories. All three only change how one range is split; the recursion around them is the same.

## Related topics

- [Quick Sort](learn:sorting/quick-sort): the Lomuto partition, simpler but weak on duplicates.
- [Quickselect](learn:sorting/quickselect): one partition at a time, keeping one side only.
- [Counting Sort](learn:sorting/counting-sort): with only a few possible values, no comparisons at all.
