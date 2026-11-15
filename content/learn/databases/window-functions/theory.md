## The idea

`GROUP BY` collapses each group into one row. A **window function** computes over a group too, but **every row stays**: each row gets an extra value worked out from the rows around it, its **window**.

`SUM(v) OVER (PARTITION BY p ORDER BY id)` reads like this:

- `PARTITION BY p` splits the rows into partitions, one per value of `p`. Rows never see other partitions.
- `ORDER BY id` orders the rows inside each partition.
- `SUM(v)` adds up `v` over the window: by default, from the first row of the partition up to the current row. The result is a **running total**.

If the rows already arrive in `id` order, one pass is enough. Keep one running total per partition in a hash table. For each row, add its `v` to its partition's total (or start the total at `v` if the partition is new) and write that total next to the row. Rows of different partitions can interleave freely; each only touches its own total.

### The default input

The array `[1, 50, 2, 20, 1, 15, 1, 5, 2, 40]` holds pairs `(p, v)`, with ids 1 to 5 in order: `(1, 50)`, `(2, 20)`, `(1, 15)`, `(1, 5)`, `(2, 40)`. Think of two accounts, 1 and 2, and their payments.

Row 1 starts partition 1 at 50. Row 2 starts partition 2 at 20. Row 3 continues partition 1: 50 + 15 = 65. Row 4: 65 + 5 = 70. Row 5 continues partition 2: 20 + 40 = 60.

The output is `[50, 20, 65, 70, 60]`: all five rows kept, each with its partition's total so far, across 2 partitions.

## Steps

1. Read the input as pairs `(p, v)`, already in `id` order.
2. For each row, if `p` has no total yet, set it to `v`; otherwise add `v` to it.
3. Write the partition's current total as this row's value.
4. Return the totals in row order.

## Cost

With `n` rows and `p` partitions:

| Case | Cost |
|---|---|
| Best and average | `O(n)`: rows already in order, one pass |
| Worst | `O(n log n)`: the rows must be sorted by partition and order first |
| Extra space | `O(p)`: one running total per partition |

## Pitfalls

- **The default frame.** With `ORDER BY` inside `OVER`, the default window is `RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW`, which includes every row tied with the current one on the `ORDER BY` value. Here `id` is unique, so it is the same as `ROWS UNBOUNDED PRECEDING`, the frame the sample's check writes out. With duplicates, write `ROWS` to get a true row-by-row running total.
- **Leaving out `ORDER BY`.** Without it, the window is the whole partition, and every row gets the partition's grand total, not a running one.
- **Expecting fewer rows.** A window function never removes rows; to get one row per partition, use `GROUP BY`.
- **Filtering on the result in `WHERE`.** Window functions are computed after `WHERE`, so filter on them in an outer query.

## When to use it

Use a window function when every row needs context from its neighbours: running balances, rankings, the change since the previous row. Other window functions follow the same `OVER (...)` pattern: `ROW_NUMBER()` numbers the rows of each partition, `RANK()` does the same but gives ties the same rank, and `LAG(v)` reads `v` from the previous row. The linked problem, Mileage Log, answers range totals with running sums, the same prefix-sum idea.

## Related topics

- Prefix sums: a running total over one array, the heart of this topic.
- GROUP BY: one row per group instead of one per row.
- Hash table: separate chaining: the table that holds the totals per partition.
- Fenwick tree: running totals that stay correct when values change.
