## The idea

A rod of length `n` can be sold whole or cut into pieces, and a piece of length `k` sells for `prices[k - 1]`. Cuts are free. Which cuts earn the most? The rod's length is the number of prices given.

Think about the **first piece** cut off the left end. If it has length `cut`, it earns its price, and the rest, of length `L - cut`, should then be cut as well as possible, which is a smaller version of the same question. So:

`best[L] = max over cut from 1 to L of prices[cut - 1] + best[L - cut]`

with `best[0] = 0`. Taking `cut = L` means selling the piece whole. The sample fills `best` from length 1 upwards and **improves** a cell only when a cut is strictly better, so on a tie the **shortest** first piece wins. It also remembers that piece in `first[L]`. The traceback then follows the rod: take `first[n]`, move on to the rest, take its first piece, and so on until nothing is left. The output is `best[n]`, then the piece lengths in that order.

This is the **unbounded knapsack**: piece lengths are items, the rod's length is the capacity, and an item may be used **any number of times**. In the 0/1 knapsack each item gets its own row and is used at most once; here `best[L - cut]` may already contain a piece of length `cut`, and that is allowed. One row of cells is enough.

### The default input

Prices `2, 5, 7, 8, 12, 13` for lengths 1 to 6. `best[1] = 2`. `best[2] = 5`: the whole piece beats 2 + 2. `best[3] = 7`: 2 + 5 comes first; 5 + 2 and the whole piece only tie. `best[4] = 10` from 5 + 5. `best[5] = 12`: 2 + 10 comes first, and 5 + 7 and the whole piece only tie. For length 6 the first pieces offer 14, **15**, 14, 13, 14 and 13, so `best[6] = 15` with a first piece of 2. The table is `[0, 2, 5, 7, 10, 12, 15]`.

Traceback: 6 starts with 2, the remaining 4 starts with 2, the remaining 2 is one piece of 2. The output is `[15, 2, 2, 2]`: three pieces of length 2, against 13 for the whole rod. The 0/1 knapsack could not use the same piece three times.

## Steps

1. Set `best[0] = 0`.
2. For each length `L` from 1 to `n`, and each first piece `cut` from 1 to `L`: if `prices[cut - 1] + best[L - cut] > best[L]`, store it and set `first[L] = cut`.
3. Traceback: from `L = n`, record `first[L]` and subtract it, until `L = 0`.
4. Return `best[n]` and the recorded pieces.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n²)`: each length tries every first piece |
| Extra space | `O(n)`: `best` and `first` |

## Pitfalls

- **Off by one in the prices.** A piece of length `cut` sells for `prices[cut - 1]`.
- **Forgetting the whole rod.** The loop must include `cut = L`, or a rod that sells best uncut is always chopped.
- **No record of the choice.** `best` alone gives the money, not the cuts; keep `first`.

## When to use it

Use the unbounded pattern whenever pieces can repeat: cutting stock, filling a budget with repeatable purchases, or maximising value with unlimited copies of each item.

## Related topics

- [0/1 knapsack](learn:dynamic-programming/knapsack-01): the version where each item is used at most once.
- [Coin change](learn:dynamic-programming/coin-change): reusable pieces again, minimising a count instead.
- [Climbing stairs](learn:dynamic-programming/climbing-stairs): another "first piece, then the rest" rule.
- [Matrix chain multiplication](learn:dynamic-programming/matrix-chain): choosing where to split, over intervals.
