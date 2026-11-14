## The idea

`C(n, k)`, read "n choose k", counts the ways to pick `k` items out of `n` when the order does not matter. The formula `n! / (k! · (n - k)!)` is correct, but the factorials explode long before the answer does: `20!` already needs 19 digits, while `C(20, 10)` is only 184 756.

**Pascal's triangle** builds the same numbers with additions only. Look at the last of the `n` items. Every pick of `k` items either **leaves it out**, and then chooses all `k` from the other `n - 1`, or **takes it**, and then chooses `k - 1` more from the other `n - 1`. No pick is in both groups, so:

`C(n, k) = C(n - 1, k - 1) + C(n - 1, k)`

The edges are 1: there is one way to pick nothing and one way to pick everything. So each row starts and ends with 1, and every inner entry is **the sum of the two entries above it**. Row `n` lists `C(n, 0)` to `C(n, n)`.

This is a **dynamic programming table**: each cell is a smaller question, answered once and reused by the row below. The sample builds rows 0 to `n` and returns row `n`. The input is a single `n` from 1 to 12.

### The default input

`n = 6`. The rows grow as:

`1` · `1 1` · `1 2 1` · `1 3 3 1` · `1 4 6 4 1` · `1 5 10 10 5 1` · `1 6 15 20 15 6 1`

For example, `C(6, 2) = C(5, 1) + C(5, 2) = 5 + 10 = 15`, and the middle `C(6, 3) = 10 + 10 = 20`. The output is `[1, 6, 15, 20, 15, 6, 1]`, after 15 additions (rows 2 to 6 have 1, 2, 3, 4 and 5 inner cells).

The row adds up to `64 = 2⁶`. That is no accident: every subset of 6 items has some size `k`, and there are `2⁶` subsets, since each item is in or out. Each row doubles the sum of the one above, because every entry above is added into two cells below.

## Steps

1. Start with row 0: `[1]`.
2. For `i` from 1 to `n`: make a row of `i + 1` ones; for `k` from 1 to `i - 1`, set `row[k] = above[k - 1] + above[k]`.
3. Return row `n`.

## Cost

| Case | Cost |
|---|---|
| Every case | `O(n²)`: about `n² / 2` additions |
| Extra space | `O(n²)` for the whole triangle, or `O(n)` keeping one row |

To keep one row, update it **from right to left**: `row[k] += row[k - 1]` then still reads the old `row[k - 1]`.

## Pitfalls

- **Factorials first.** `n!` overflows 64-bit integers at `n = 21`, even when `C(n, k)` is small; the table never builds a number bigger than the answer.
- **Left-to-right in one row.** It reads cells already updated in this row and produces wrong values.
- **Forgetting `k > n`.** There are 0 ways to pick more items than exist; check before reading the table.

## When to use it

Use the table when `n` is up to a few thousand, or when you need many `C(n, k)` values at once, possibly modulo a number, since it only adds. Paths in a grid are a classic case: a walk of `r` steps south and `c` east is a choice of which `r` of the `r + c` steps go south. The linked problems, Team Picks and Blocked Walks, go up to `n = 10⁵`, too big for a table; there, factorials modulo a prime divided with a [Modular inverse](learn:math-geometry/modular-inverse) take over.

## Related topics

- [Paths through a grid](learn:dynamic-programming/grid-paths): the same addition rule, cell by cell.
- [Climbing stairs](learn:dynamic-programming/climbing-stairs): another table where each value is the sum of earlier ones.
- [Modular inverse](learn:math-geometry/modular-inverse): the division-based route for large `n`.
- [Subsets](learn:backtracking/subsets): listing the `2ⁿ` picks one by one instead of counting them.
