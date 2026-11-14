## The idea

A **subsequence** keeps some of the values in their original order, skipping the rest. The task is to find the longest one that rises strictly: every value larger than the one before it. Trying every subset of positions doubles the work with each value, but each answer must **end somewhere**, and that is the smaller question the table asks.

`length[i]` is the length of the longest rising subsequence that **ends at index `i`**. On its own, a value is a subsequence of length 1, so every cell starts at 1. Then the sample looks at every earlier index `j`: if `values[j] < values[i]`, the value at `i` can extend whatever ended at `j`, giving `length[j] + 1`. The cell keeps the largest such offer, and `prev[i]` remembers which `j` gave it. Only a **strictly** larger offer replaces the current one, so on a tie the earliest `j` stays.

The answer ends at the index with the largest length; on a tie the sample takes the **first** such index. Following `prev` from there walks the subsequence backwards. The output is the length, then the subsequence in order.

### The default input

Values `[7, 3, 8, 4, 9, 2, 10, 6, 11]`. The lengths come out as `[1, 1, 2, 2, 3, 1, 4, 3, 5]`. Index 2 (value 8) extends 7, and 3 offers the same 2, which does not replace it. Index 4 (value 9) gets 3 from 8. Index 6 (value 10) gets 4 from 9. Index 7 (value 6) gets 3 through 3 and 4. Index 8 (value 11) gets 5 from 10.

The traceback starts at index 8 and follows `prev`: 11, 10, 9, 8, 7. The output is `[5, 7, 8, 9, 10, 11]`. The chain 3, 8, 9, 10, 11 is just as long; the tie rule picked 7 because it came first.

## Steps

1. Set every `length[i]` to 1 and every `prev[i]` to -1.
2. For each `i`, and each `j < i` with `values[j] < values[i]`: if `length[j] + 1 > length[i]`, set `length[i]` to it and `prev[i] = j`.
3. Let `end` be the first index with the largest length.
4. Follow `prev` from `end` until -1, collecting values; reverse them and put the length in front.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n²)`: each index looks at every earlier index |
| Extra space | `O(n)` for `length` and `prev` |

## Pitfalls

- **Taking the last cell as the answer.** The longest subsequence does not have to end at the last value; scan for the maximum.
- **Strict versus non-strict.** `<` makes the subsequence strictly rising; `≤` would let equal values repeat. Read which one the task asks for.
- **Large inputs.** At tens of thousands of values, `n²` is too slow. The **patience** method keeps, for each length, the smallest value that can end it, and finds where each new value goes with a binary search: `O(n log n)`.

## When to use it

Use it when items must be chosen in order and each must beat the last: rising prices, nested boxes after sorting, the longest chain of anything. The linked problem, Rising Towers, asks for this length, but with up to 50 000 towers it needs the patience method.

## Related topics

- [Maximum subarray (Kadane)](learn:dynamic-programming/maximum-subarray): the best answer ending at each index, looking only one step back.
- [House robber](learn:dynamic-programming/house-robber): take or skip, one index at a time.
- [Longest common subsequence](learn:dynamic-programming/lcs): subsequences shared by two sequences.
- [Binary search](learn:searching/binary-search): the step that makes the patience method fast.
