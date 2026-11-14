## The idea

Insertion sort moves a value only one place per shift, so a small value near the end needs many shifts to travel left. Shell sort first runs insertion sort on values a **gap** apart, so a value can jump far in a single move. This sample uses Shell's original gaps: half the length, then half again, down to 1. The pass with gap 1 is plain insertion sort, but by then the array is nearly sorted and little work is left.

## Steps

1. Start with `gap = len(a) // 2`.
2. For each `i` from `gap` to the end, lift `a[i]` out as the `key`.
3. While the value `gap` places left of the hole is bigger than the key, move it `gap` places right; the hole moves `gap` places left.
4. Drop the key into the hole.
5. Halve the gap and repeat. The pass with gap 1 finishes the sort.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n log n)` (sorted input: one comparison per value in each pass) |
| Average | Depends on the gap sequence |
| Worst | `O(n²)` with Shell's gaps |
| Extra space | `O(1)` |
| Stable | No: a long jump can pass an equal value |
| In place | Yes |

## Pitfalls

- **Comparing with the neighbour.** The key is compared with the value `gap` places left, `a[j - gap]`, not `a[j - 1]`, and the loop must check `j >= gap` so it never reads before position 0.
- **Shell's gaps can be poor.** When the length is a power of two, every gap is even until the last pass, so values on even positions never meet values on odd positions before gap 1. Inputs built to exploit that push the work to `O(n²)`.
- **Better sequences exist.** Other gap lists, such as Knuth's 1, 4, 13, 40, … or lists found by experiment, avoid that trap and are much faster in practice. Only the gaps change; the loop stays the same.

## When to use it

Shell sort is short, in place, with no recursion and no extra memory, which helps on small embedded systems or for medium arrays when no library sort is at hand. For large inputs or a guaranteed bound, prefer merge sort or heap sort.

## Related topics

- [Insertion Sort](learn:sorting/insertion-sort): each pass is insertion sort with steps of `gap`.
- [Merge Sort](learn:sorting/merge-sort): a guaranteed `O(n log n)`, at the price of a buffer.
- [Big-O growth curves](learn:foundations/big-o-growth): why the gap sequence changes the growth.
