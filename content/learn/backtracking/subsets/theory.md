## The idea

A list of `n` values has `2ⁿ` subsets, because each value is either in or out. **Backtracking** lists them by building one answer at a time in a single shared list, `chosen`, with three moves repeated at every level: **choose** (append a value), **explore** (call itself to extend that choice) and **un-choose** (pop the value again). Since every call leaves `chosen` exactly as it found it, one list serves the whole search; only the answers that get recorded are copied.

This is ordinary [recursion](learn:foundations/recursion-call-stack) with one twist: the calls share their state and clean up after themselves. The app draws the search as an indented outline, one line per call; the indent is the number of values in `chosen`.

Here each call, `grow(start)`, first **records** a copy of `chosen`, because every partial choice is already a subset. Then it tries each position `k` from `start` on: choose `values[k]`, call `grow(k + 1)`, undo. Passing `k + 1` means values are added only in input order, so each subset is built once, in one order. The output is the count, then each subset as its size followed by its values. Values are picked by position, so equal values at different positions give repeated subsets.

Another way to list subsets is a **bitmask**: count from `0` to `2ⁿ − 1` and let bit `k` of the counter say whether value `k` is in. It needs no recursion but visits the subsets in a different order, and cannot skip a whole branch at once.

### The default input

`[1, 2, 3]`. `grow(0)` records `[]` and chooses 1; `grow(1)` records `[1]` and chooses 2; `grow(2)` records `[1, 2]` and chooses 3; `grow(3)` records `[1, 2, 3]` and has nothing left to try. Back in `grow(1)`, 2 is undone and 3 chosen: `[1, 3]`. Then 1 is undone and `grow(0)` moves on to `[2]`, `[2, 3]` and finally `[3]`.

Eight calls, eight subsets. The output is `[8, 0, 1, 1, 2, 1, 2, 3, 1, 2, 3, 2, 1, 3, 1, 2, 2, 2, 3, 1, 3]`.

## Steps

1. Start with an empty `chosen` and call `grow(0)`.
2. In `grow(start)`, record a copy of `chosen`.
3. For each position `k` from `start` to `n − 1`: append `values[k]`, call `grow(k + 1)`, then pop it.
4. Return the count, then every subset as its size and values.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n · 2ⁿ)`: `2ⁿ` subsets, each copied in up to `n` steps |
| Extra space | `O(n)` for the call depth and `chosen`, besides the output |

## Pitfalls

- **Recording `chosen` itself.** Every entry is then the same list, and it is empty when the search ends. Record a copy.
- **Forgetting to undo.** A value left in `chosen` leaks into every later subset.
- **Looping from 0 instead of `start`.** The start position is what keeps each subset in one order; without it, values repeat and the recursion never stops.
- **Growth.** Each extra value doubles the work: 20 values already give over a million subsets.

## When to use it

Use it when every set of yes-or-no choices must be listed or checked, especially when a partial choice can be dropped early, which skips its whole branch. The linked problems, Outfit Picker and Menu Masks, are built from choices like these.

## Related topics

- [Recursion and the call stack](learn:foundations/recursion-call-stack): the calls that backtracking stacks up and unwinds.
- [Bit manipulation basics](learn:foundations/bit-manipulation): subsets as the bits of a number.
- [Travelling salesperson (bitmask DP)](learn:dynamic-programming/bitmask-tsp): a table indexed by subsets.
- [Permutations](learn:backtracking/permutations): the same choose and undo, for every order.
- [Combination sum](learn:backtracking/combination-sum): subsets that must hit a total, with pruning.
