## The idea

Bucket sort spreads the values over a few **buckets** that cover equal ranges, sorts each small bucket, and reads the buckets back in order. Here the values go from 1 to 99 and there are five buckets of 20: 0–19, 20–39, 40–59, 60–79 and 80–99. Every value in a bucket is smaller than every value in the next one, so once each bucket is sorted, reading them from left to right gives a sorted array. When the values are spread evenly, each bucket holds only a handful, and sorting a handful is cheap.

## Steps

1. Make `count = 5` empty buckets.
2. Scatter: put each value `v` into bucket `v * count // 100`. For example, 43 goes to bucket 2, the range 40–59.
3. Sort each bucket. The sample calls the language's own sort, which is fine for tiny buckets; textbooks often use insertion sort here.
4. Gather: walk the buckets in order and write their values back from position 0.

## Cost

| Case | Cost |
|---|---|
| Best | `O(n + k)` (`k` = number of buckets) |
| Average | `O(n + k)` for spread-out values |
| Worst | `O(n²)`: every value in one bucket, sorted by insertion sort |
| Extra space | `O(n + k)` |
| Stable | Yes, when the sort inside each bucket is stable |
| In place | No |

## Pitfalls

- **Clustered values.** If most values fall into one range, one bucket gets most of the work and the gain is gone: the total costs whatever sorting that bucket costs. With insertion sort in the buckets that is `O(n²)`; with the built-in sort the sample uses, `O(n log n)`. Try your own input with every value between 20 and 39.
- **The bucket formula.** `v * count // 100` must send the biggest possible value to the last bucket, not past it. With values up to 99 it gives at most 4; allowing 100 would need a sixth bucket.
- **The number of buckets.** Too few and each bucket is big; too many and most are empty but still visited. For evenly spread data, about as many buckets as values is a common choice.

## When to use it

Use it when you know the values are spread fairly evenly over a known range: random fractions between 0 and 1, or measurements within known limits. The same scatter-by-range idea is behind histograms and behind splitting a huge sort across machines, each one taking a range of keys.

## Related topics

- [Counting Sort](learn:sorting/counting-sort): one bucket per value, so no bucket ever needs sorting.
- [Radix Sort](learn:sorting/radix-sort): buckets by digit, over several passes.
- [Insertion Sort](learn:sorting/insertion-sort): the classic sort for the small buckets.
