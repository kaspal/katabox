## The idea

When the values are small whole numbers, there is no need to compare them at all. Counting sort keeps one **bucket** (a counter) per possible value, here 0 to 9. A first pass drops each value into its bucket, so a bucket's height is how many times its value appears. A second pass empties the buckets in value order, writing each value back as many times as it was counted.

This sample writes the values themselves, which is all plain numbers need. When each value carries a record (a name with a score), the usual version is stable: it turns the counts into prefix sums, so each value knows where its first copy goes, then walks the input from left to right and puts every record in the next free slot for its value, in a separate output array. Equal values keep their input order.

## Steps

1. Make `counts` with one cell per value from 0 to `max(a)`, all zero.
2. For each value `v` in the array, add one to `counts[v]`.
3. Set `k = 0`, the next position to write.
4. For each value `v` from 0 up, write `v` at `a[k]` `counts[v]` times, moving `k` on each time.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n + k)` (`k` = number of possible values) |
| Average | `O(n + k)` |
| Worst | `O(n + k)` |
| Extra space | `O(k)` for the counts |
| Stable | Yes, in the prefix-sum version |
| In place | No |

## Pitfalls

- **A huge range.** The cost grows with `k`, not only with `n`. Ten values between 0 and a billion would need a billion counters. Counting sort pays off only when `k` is not much bigger than `n`.
- **Negative values.** `counts[v]` needs `v ≥ 0`. Shift every value by the minimum first: `counts[v - min]`.
- **Losing the records.** Rewriting values from the counts throws away anything attached to them. To move whole records, use the prefix-sum version.

## When to use it

Use it for small integer keys: marks from 0 to 10, ages, letters of an alphabet, digits. It never compares two values, so it is not bound by the `O(n log n)` limit of comparison sorts. It is also the inner step of radix sort, which sorts bigger numbers one digit at a time with a stable counting pass.

## Related topics

- [Prefix sums](learn:foundations/prefix-sums): how the counts become positions in the stable version.
- [Radix Sort](learn:sorting/radix-sort): counting, one digit at a time.
- [Bucket Sort](learn:sorting/bucket-sort): buckets that hold a range of values instead of one.
