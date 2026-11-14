## The idea

Given a row of numbers, some negative, find the **contiguous run** with the largest sum. Checking every start and end is `O(n²)` even with running sums. **Kadane's algorithm** does it in one pass.

It is dynamic programming with a well-chosen question: what is the best sum of a run that **ends exactly at index `i`**? Such a run either is the value alone or extends the best run ending at `i - 1`. Extending only helps if that earlier sum is positive, so:

`current(i) = value + current(i - 1)` if `current(i - 1) > 0`, else `value`

The answer is the largest `current(i)` over all `i`. Each step needs only the previous cell, so the "table" shrinks to one variable, `current`, plus the start of its run.

The sample **restarts** the run at `i` when `current ≤ 0`, and **extends** it otherwise. After each step it **records** the run if its sum is strictly better than the best so far. The best starts at the first value, not at 0, so a row of only negative numbers still works: the answer is the largest single value. The output is `[best, start, end]`, with 0-based indices.

### The default input

`-2, 3, -1, 4, -6, 2, 5, -3, 1`. Index 0 starts a run at -2, which ties the initial best and is not recorded again. The sum is negative, so index 1 restarts at 3: recorded, `[3, 1, 1]`. Index 2 extends to 2. Index 3 extends to 6: recorded, `[6, 1, 3]`. Index 4 extends to 0. Since 0 is not positive, index 5 restarts at 2 with start 5. Index 6 extends to 7: recorded, `[7, 5, 6]`. Indices 7 and 8 give 4 and 5, no better.

The output is `[7, 5, 6]`: `2 + 5`. The run from 1 to 6 also sums to 7, but restarting at a sum of 0 drops a prefix that adds nothing, and "strictly better" keeps the first run found.

On the all-negative preset `-4, -2, -7, -1, -5`, every index restarts, and the output is `[-1, 3, 3]`.

## Steps

1. Set `best` to the first value with start and end 0, and `current` to 0.
2. For each index `i`: if `current ≤ 0`, set `current` to the value and the run's start to `i`; otherwise add the value to `current`.
3. If `current > best`, record `current`, the run's start and `i`.
4. Return `[best, start, end]`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n)`: one pass, constant work per value |
| Extra space | `O(1)`: the current run and the best one |

## Pitfalls

- **Starting `best` at 0.** On an all-negative row it would report 0, the sum of an empty run, which is not allowed here.
- **Restarting only when `current < 0`.** The best sum is the same, but runs can carry a useless prefix that sums to 0, so the indices change.
- **Forgetting the start.** Record the start together with the sum, or the run cannot be reported.
- **Recording with `≥`.** That moves the answer to a later run on ties.

## When to use it

Use Kadane for the best stretch of daily gains, the strongest segment of a signal, or as a building block, for example the best rectangle in a grid by fixing two rows and running it on the column sums.

## Related topics

- [Prefix sums](learn:foundations/prefix-sums): a run's sum as a difference of two prefixes.
- [Sliding window](learn:searching/sliding-window): runs with a size or a condition instead of a best sum.
- [House robber](learn:dynamic-programming/house-robber): another one-pass choice over a row of values.
- [Longest increasing subsequence](learn:dynamic-programming/lis): best answer ending at each index, with a longer look back.
