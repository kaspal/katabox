## The idea

A knapsack holds at most a given **capacity** of weight. Each item has a weight and a value and can be packed **once or not at all** (hence "0/1"). Which items give the largest total value? Trying every subset doubles the work with each item, and greedy rules such as "best value per unit of weight first" can paint themselves into a corner.

The table answers a two-part question: `best[i][c]` is the largest value using only the **first `i` items** with capacity **`c`**. Row 0 (no items) is all zeros. For item `i - 1` with weight `w` and value `v`, every cell first copies the cell **above it**, `best[i - 1][c]`: the item is left out. Then, if `w ≤ c`, it tries packing the item: `best[i - 1][c - w] + v`, the best of the earlier items in the room that is left, plus this item. It keeps the take only if it is strictly better. The answer sits in the bottom-right corner.

A **traceback** recovers the items. Start at the corner and go up one row at a time: if a cell differs from the one above, its item was packed, so record it and move left by its weight. The sample outputs the best value, then the packed items (0-based) in order. The input is typed as the capacity followed by "weight value" pairs.

### The default input

Capacity 10; items `(5, 10)`, `(4, 7)`, `(6, 12)`, `(3, 6)` as weight and value. Row 1 holds 10 from capacity 5 on. Row 2 reaches 17 at capacity 9: items 0 and 1. Row 3 (item 2, weight 6) at capacity 10 compares 17 above with `best[2][4] + 12 = 7 + 12 = 19`, and takes. Row 4 (item 3, weight 3) at capacity 10 compares 19 above with `best[3][7] + 6 = 12 + 6 = 18`, and leaves it.

Traceback: row 4 equals row 3 at capacity 10, so item 3 is out. Row 3 differs: item 2 is in, capacity drops to 4. Row 2 at 4 is 7 against row 1's 0: item 1 is in, capacity 0. Row 1 at 0 equals row 0: item 0 is out.

The output is `[19, 1, 2]`. Items 0, 2 and 3 all pay 2 per unit of weight, but any packing that starts with item 0 tops out at 17.

## Steps

1. Make a table of `n + 1` rows and `capacity + 1` columns, all zeros.
2. For each item `i` (weight `w`, value `v`) and each capacity `c`: copy `best[i - 1][c]`; if `w ≤ c` and `best[i - 1][c - w] + v` is larger, use that.
3. Traceback from row `n`, capacity `capacity`: whenever `best[i][c] ≠ best[i - 1][c]`, record item `i - 1` and subtract its weight.
4. Return the corner value and the recorded items, reversed.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n · capacity)`: one cell per item and capacity |
| Extra space | `O(n · capacity)` for the table, which the traceback needs |

## Pitfalls

- **Squeezing the table into one row, left to right.** With a single row, the cell at `c - w` to the left has already been updated for this item, so reading it can pack the same item twice. Going right to left reads only values from the previous item. (The one-row version also loses the traceback.)
- **Big capacities.** The cost grows with the capacity's value, not its number of digits; a capacity of a billion makes a billion columns.
- **Forgetting the weight in the traceback.** After a packed item, the rest of the answer lives at capacity `c - w` in the row above, not at `c`.

## When to use it

Use it for budgets with yes-or-no choices: packing, picking projects under a cost cap, choosing exam questions under a time limit. The linked problem, Backpack Weight, is this pattern.

## Related topics

- [Rod cutting (unbounded knapsack)](learn:dynamic-programming/rod-cutting): the same table when an item may repeat.
- [Coin change](learn:dynamic-programming/coin-change): reaching an exact total with reusable pieces.
- [House robber](learn:dynamic-programming/house-robber): take or skip, with a one-dimensional table.
- Bitmask dynamic programming (coming soon).
