## The idea

A staircase has `n` steps, and each move climbs either **1 step or 2**. In how many different orders can you reach the top? Listing every order works for tiny staircases, but the count grows by about 60% with every extra step.

Look at the **last move** instead. Any way to reach step `i` ends either with a 1-step move from step `i - 1` or with a 2-step move from step `i - 2`. No way ends in both, and every way ends in one of them, so the two groups can simply be added:

`ways[i] = ways[i - 1] + ways[i - 2]`

The question for `i` is answered from the answers for smaller staircases, which is optimal substructure; each of those answers is used twice, which is overlapping subproblems. A table filled from the bottom up handles both.

The base case is `ways[0] = 1`: there is exactly one way to stand at the bottom, by not moving at all. It makes `ways[2] = 2` come out right (1 + 1, or 2). The sample fills each cell in two moves: it copies `ways[i - 1]` (the last move was 1 step), then, when `i ≥ 2`, adds `ways[i - 2]` (the last move was 2 steps). It outputs the whole table `ways[0..n]`.

This is the Fibonacci rule again, shifted by one: `ways[i] = fib(i + 1)`.

### The default input

`n = 6`. Cell 1 copies `ways[0]`, so `ways[1] = 1`: a single step. Cell 2 copies 1 and adds 1, giving 2. Cell 3 gets 2 + 1 = 3: the orders 1 + 1 + 1, 1 + 2 and 2 + 1. Cell 4 gets 3 + 2 = 5, cell 5 gets 5 + 3 = 8, and cell 6 gets 8 + 5 = 13.

The output is `[1, 1, 2, 3, 5, 8, 13]`: 13 ways to climb six steps.

## Steps

1. Make a table of `n + 1` cells and set `ways[0] = 1`.
2. For `i` from 1 to `n`, set `ways[i] = ways[i - 1]`.
3. If `i ≥ 2`, add `ways[i - 2]` to it.
4. Return the table; `ways[n]` is the answer.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n)`: one cell per step, each from two earlier cells |
| Extra space | `O(n)` for the table, or `O(1)` with two variables for the last two cells |

## Pitfalls

- **`ways[0] = 0`.** It feels natural ("zero steps, zero moves"), but then every cell sums zeros and the whole table stays 0.
- **Off by one.** Steps 0 to `n` need `n + 1` cells.
- **Counting sets instead of orders.** Here 1 + 2 and 2 + 1 are different ways. Counting which move sizes are used, ignoring the order, needs a different loop.
- **Huge counts.** The numbers grow about 1.6 times per step; real problems usually ask for the answer modulo a large prime, applied after each addition.

## When to use it

The pattern "count the ways to build length `n` from pieces" fits tilings, codes made of short symbols and paths that move forward in fixed jumps. The linked problem, Path Tiles, adds a **third tile size**. The last-move argument still applies; working out what changes in the formula is the exercise.

## Related topics

- [Fibonacci: memo and table](learn:dynamic-programming/fibonacci-memo-table): the same rule, and both ways to compute it.
- [House robber](learn:dynamic-programming/house-robber): another table that looks back two cells.
- [Coin change](learn:dynamic-programming/coin-change): pieces of several sizes, counting the fewest instead of the ways.
- Grid paths (coming soon): counting routes in two dimensions.
