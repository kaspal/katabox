## The idea

A row of houses each holds some loot, but two **neighbouring** houses can never both be robbed. What is the largest total, and which houses give it? Trying every allowed set grows exponentially; taking every other house is simple but often loses.

Let `best[i]` be the largest total from the **first `i` houses**. For house `i - 1`, the last of them, there are only two choices:

- **skip** it: the total is `best[i - 1]`;
- **take** it: its neighbour must be skipped, so the total is `best[i - 2] + value`.

`best[i]` is the larger of the two. The bases are `best[0] = 0` (no houses) and `best[1]` = the first house's value. The table holds totals only, so a **traceback** recovers the houses: walk back from `best[n]`. If `best[i]` equals `best[i - 1]`, house `i - 1` can be skipped, so step back one; otherwise it was taken, so record it and jump back two, over its neighbour. On a tie the sample prefers to skip. It outputs `best[n]`, then the chosen houses (0-based) from left to right.

### The default input

Houses `6, 3, 10, 8, 2, 9, 4`. `best[1] = 6`. House 1 (3): taking gives 0 + 3, skipping keeps 6, so `best[2] = 6`. House 2 (10): 6 + 10 = 16 beats 6. House 3 (8): 6 + 8 = 14 loses to 16. House 4 (2): 16 + 2 = 18. House 5 (9): 16 + 9 = 25 beats 18. House 6 (4): 18 + 4 = 22 loses to 25. The table is `[0, 6, 6, 16, 16, 18, 25, 25]`.

Traceback from 7: `best[7] = best[6]`, so house 6 is skipped. `best[6] ≠ best[5]`: take house 5 and jump to 4. `best[4] = best[3]`: skip house 3. `best[3] ≠ best[2]`: take house 2, jump to 1. `best[1] ≠ best[0]`: take house 0.

The output is `[25, 0, 2, 5]`: 6 + 10 + 9. Every other house would give only 22 (houses 0, 2, 4, 6) or 20 (houses 1, 3, 5).

## Steps

1. Set `best[0] = 0` and `best[1]` to the first value.
2. For `i` from 2 to `n`, set `best[i] = max(best[i - 1], best[i - 2] + values[i - 1])`.
3. Traceback: start at `i = n`. While `i > 0`: if `best[i] = best[i - 1]`, step back one; otherwise record house `i - 1` and step back two.
4. Return `best[n]` and the recorded houses, reversed.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n)`: one choice per house, plus a traceback of at most `n` steps |
| Extra space | `O(n)` for the table, which the traceback needs |

## Pitfalls

- **Stepping back one after a take.** The neighbour of a taken house was skipped by definition, so the traceback must jump two.
- **Mixing up indices.** `best` has `n + 1` cells, and `best[i]` is about house `i - 1`.
- **Expecting a single answer.** Ties can make two different sets equally good; preferring to skip picks one of them.
- **Dropping the table too early.** Two variables give the best total, but the houses need the whole table.

## When to use it

The skip-or-take choice with a "no two in a row" rule fits shift planning, picking non-adjacent seats, or any line of choices where using one blocks its neighbour. It is also the smallest example of a traceback.

## Related topics

- [Climbing stairs](learn:dynamic-programming/climbing-stairs): the same look-back of two cells, adding instead of choosing.
- [0/1 knapsack](learn:dynamic-programming/knapsack-01): take or leave each item, with a capacity instead of neighbours.
- [Maximum subarray (Kadane)](learn:dynamic-programming/maximum-subarray): a different rule over a row of values.
- [Longest increasing subsequence](learn:dynamic-programming/lis): choosing items in order with a traceback.
