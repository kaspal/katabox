## The idea

A **permutation** is one ordering of all the values; `n` values have `n!` of them. Backtracking builds each ordering by filling positions from left to right: for the next position, try every value that is not used yet, go on to the position after it, then take the value back and try the next one.

The sample keeps two pieces of shared state: `chosen`, the ordering built so far, and `used`, one flag per **input position**. Each call, `place()`, first checks whether `chosen` is full; if so, it records it and returns. Otherwise it walks `k` from 0 to `n − 1` and, for every position not yet used, marks it, appends `values[k]`, calls itself, then pops the value and clears the flag. Both pieces of state are restored, so the next choice starts from a clean slate.

Choices are made by position, not by value, so the count is always `n!`. With equal values, the same ordering comes out more than once: `[5, 5]` gives `[5, 5]` twice. When the input is sorted, the orderings come out in ascending (dictionary) order. The output is the count, then the permutations one after another, `n` values each. The app keeps `n ≤ 4`, at most 24 orderings.

### The default input

`[1, 2, 3]`. The first call has three choices. Under 1, the remaining positions give `[1, 2, 3]` and then `[1, 3, 2]`. Under 2 come `[2, 1, 3]` and `[2, 3, 1]`; under 3, `[3, 1, 2]` and `[3, 2, 1]`.

The outline has 1 + 3 + 6 + 6 = 16 calls: one at the top, three with one value, six with two and six full ones that record. The output is `[6, 1, 2, 3, 1, 3, 2, 2, 1, 3, 2, 3, 1, 3, 1, 2, 3, 2, 1]`.

## Steps

1. Start with an empty `chosen` and every `used` flag off, and call `place()`.
2. If `chosen` holds `n` values, record it and return.
3. Otherwise, for each position `k` that is not used: mark it, append `values[k]`, call `place()`, pop the value and clear the mark.
4. Return the count, then the recorded permutations.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n · n!)`: `n!` orderings of `n` values each, and every call scans `n` flags |
| Extra space | `O(n)` for the call depth, `chosen` and `used`, besides the output |

## Pitfalls

- **Clearing only half of the state.** Popping the value but leaving `used[k]` on hides that position from every later branch.
- **Duplicates.** To list each distinct ordering once, sort the values and skip a value equal to the one at the previous position while that previous one is unused.
- **Growth.** `10!` is already 3,628,800 orderings. If you only need the best one, look for pruning or a table instead.
- **Swapping in place.** Swapping values into position avoids `used`, but changes the order in which orderings appear.

## When to use it

Use it when order matters and `n` is small: seating plans, running order for a few tasks, trying every route through a handful of stops. For the cheapest route through many stops, dynamic programming over subsets is far faster.

## Related topics

- [Subsets](learn:backtracking/subsets): the same choose and undo, where order does not matter.
- [N-Queens](learn:backtracking/n-queens): a permutation of columns with extra checks.
- [Travelling salesperson (bitmask DP)](learn:dynamic-programming/bitmask-tsp): best ordering without listing them all.
- [Recursion and the call stack](learn:foundations/recursion-call-stack): how the calls stack up and unwind.
