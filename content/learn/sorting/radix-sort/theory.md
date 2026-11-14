## The idea

Radix sort sorts numbers one digit at a time, starting from the ones: the least significant digit, or LSD. Each pass drops every value into one of ten buckets by its current digit, then collects the buckets from 0 to 9 back into the array. The trick is that every pass is **stable**: inside a bucket, values keep the order they arrived in. So after the tens pass, two values with the same tens digit are still ordered by their ones digit from the pass before. After the last digit, the whole array is sorted.

## Steps

1. Start with `place = 1`, the ones digit.
2. Make ten empty `buckets`. Taking the values in array order, append each `v` to bucket `v // place % 10`.
3. Collect: walk the buckets from 0 to 9 and write their values back, in order, from position 0.
4. Multiply `place` by 10 and repeat while `place <= max(a)`.

## Cost

| Case | Cost |
|---|---|
| Best | `O(d · (n + 10))` (`d` = digits of the biggest value) |
| Average | `O(d · (n + 10))` |
| Worst | `O(d · (n + 10))` |
| Extra space | `O(n + 10)` for the buckets |
| Stable | Yes: every pass keeps arrival order |
| In place | No |

## Pitfalls

- **Starting from the wrong end.** LSD goes from the ones up. Running the same loop from the hundreds down scrambles the order the earlier digits set.
- **An unstable pass.** If a pass reorders the values inside a bucket, the work of the earlier passes is lost. Appending at the end of a bucket and reading it from the front keeps it stable.
- **Short numbers.** 8 has no tens or hundreds digit; `v // place % 10` gives 0, which is exactly right. In the default input, 8 waits in bucket 0 in the last two passes.
- **Negative values.** The digit formula assumes `v ≥ 0`. Sort negatives separately, or shift every value first.

## When to use it

Radix sort fits fixed-width keys: integers, dates, postal codes, fixed-length strings. Its cost grows with the number of digits, not with `log n`, so it beats comparison sorts when there are many short keys. Real versions often use base 256, one byte per pass, to need fewer passes.

## Related topics

- [Counting Sort](learn:sorting/counting-sort): its stable prefix-sum version is the usual per-digit pass.
- [Bucket Sort](learn:sorting/bucket-sort): buckets by range instead of by digit.
- [Bit manipulation basics](learn:foundations/bit-manipulation): base-256 passes read each byte with a shift and a mask.
