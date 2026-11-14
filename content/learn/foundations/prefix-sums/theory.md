## The idea

Adding up a range of an array value by value costs one step per value, every time you ask. Prefix sums pay that cost once. Build a row `p` one cell longer than the array, where `p[i]` is the sum of everything left of position `i` (so `p[0] = 0`). Then the sum of positions `lo..hi` is just `p[hi + 1] − p[lo]`: the total up to the end of the range minus the total before it. Two reads, whatever the range's length.

## Steps

1. Make `p` with `len(a) + 1` cells and set `p[0] = 0`.
2. For each `i` from left to right: `p[i + 1] = p[i] + a[i]`.
3. To sum positions `lo..hi`, read `p[hi + 1]` and `p[lo]`.
4. Return `p[hi + 1] − p[lo]`.

## Cost

| Measure | Cost |
|---|---|
| Build | `O(n)`: one addition per value |
| Each range sum | `O(1)`: two reads and one subtraction |
| Changing one value | `O(n)`: every prefix after it changes |
| Extra space | `O(n)`: `n + 1` cells |

## Pitfalls

- **Off by one.** The range `lo..hi` includes both ends, so it is `p[hi + 1] − p[lo]`, not `p[hi] − p[lo]` (that would leave out `a[hi]`). The extra cell `p[0] = 0` is what makes a range starting at 0 work without a special case.
- **Updating the array.** After `a[i]` changes, every `p[j]` with `j > i` is stale. When values change between queries, use a Fenwick tree or a segment tree instead.
- **Overflow.** A prefix can be far bigger than any single value. In languages with fixed-size integers, sum into a 64-bit type. JavaScript numbers lose precision past 2⁵³.

## When to use it

Use prefix sums whenever an array stays fixed and you need many range sums: totals over date ranges, the number of matches between two positions, or finding a split point where the left and right sums balance. The same idea works with counts, with XOR instead of addition, and in two dimensions for rectangle sums.

## Related topics

- [Big-O growth curves](learn:foundations/big-o-growth): `q` queries cost `O(n + q)` instead of `O(n · q)`.
- [Bit manipulation basics](learn:foundations/bit-manipulation): XOR prefixes work the same way.
- Fenwick tree (coming soon): prefix sums that also allow updates.
- Sliding window (coming soon).
