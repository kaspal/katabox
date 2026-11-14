## The idea

You get a **target** and a few **candidates**, and each candidate may be used as often as you like. List every combination that adds up to the target exactly. Order does not matter: `2 + 3 + 3` and `3 + 2 + 3` are the same combination and must appear once.

The sample first **sorts** the candidates. Then `search(start, left)` tracks how much is still missing. If `left` is 0, the current `chosen` is recorded. Otherwise it tries each candidate from position `start` on: choose it, call `search(k, left − candidate)`, and pop it afterwards. Two details do the real work:

- **The start index.** The call passes `k`, not `k + 1`, so the same candidate can be chosen again, but never an earlier one. Every combination is therefore built in non-decreasing order, and each one exactly once.
- **The `break`.** When a candidate is bigger than what is left, the loop stops. That is safe only because the list is sorted: every later candidate is bigger still. This **pruning** cuts whole branches without visiting them.

The output is the count, then each combination as its size followed by its values. The input is typed as the target followed by the candidates.

### The default input

Target 8, candidates `2, 3, 5`. Choosing 2 four times reaches 0: `[2, 2, 2, 2]`. Back at `2 + 2 + 2` (2 left), 3 is too big: break. At `2 + 2`, choosing 3 leaves 1, and everything breaks; 5 is bigger than 4. At `2`, choosing 3 twice reaches 0: `[2, 3, 3]`; `2 + 5` leaves 1 and breaks. Starting from 3, `3 + 3` leaves 2 and breaks, while `3 + 5` hits 0: `[3, 5]`. Finally `5` alone leaves 3, smaller than 5, so the search ends.

The output is `[3, 4, 2, 2, 2, 2, 3, 2, 3, 3, 2, 3, 5]`.

## Steps

1. Sort the candidates and call `search(0, target)` with an empty `chosen`.
2. If `left` is 0, record a copy of `chosen` and return.
3. For each position `k` from `start`: if `candidates[k] > left`, stop the loop; otherwise append it, call `search(k, left − candidates[k])` and pop it.
4. Return the count, then every combination as its size and values.

## Cost

| Case | Cost |
|---|---|
| Best | Output-sensitive: with sharp pruning, the work follows the number of combinations found |
| Average | Exponential in the target |
| Worst | Exponential in the target: small candidates make deep, wide trees |
| Extra space | `O(target)` depth: each level subtracts at least the smallest candidate |

## Pitfalls

- **Breaking without sorting.** On an unsorted list, a big candidate would stop the loop before a smaller one later on, and combinations go missing.
- **Restarting at 0.** Then `2 + 3 + 3`, `3 + 2 + 3` and `3 + 3 + 2` are all listed.
- **Passing `k + 1`.** Each candidate can then be used only once; that is a different problem.
- **Repeated candidates.** If a value appears twice in the input, its combinations appear twice too.

## When to use it

Use it when the actual combinations are needed, not just how many there are or the fewest pieces. [Coin change](learn:dynamic-programming/coin-change) counts or minimises with a table in `O(target · coins)` and never lists anything; listing can produce exponentially many answers. The linked problems are Coin Bag Combos and Postage Stamps.

## Related topics

- [Coin change](learn:dynamic-programming/coin-change): counting and minimising instead of listing.
- [Subsets](learn:backtracking/subsets): the same start index, each value used at most once.
- [Rod cutting (unbounded knapsack)](learn:dynamic-programming/rod-cutting): reusable pieces, best value with a table.
- [Permutations](learn:backtracking/permutations): when order does matter.
