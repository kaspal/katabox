## The idea

`SELECT g, COUNT(*), SUM(v) FROM t GROUP BY g ORDER BY g` collapses a table: all rows with the same `g` become a single output row, carrying how many rows the group had and the sum of their `v`. Many rows go in; one row per distinct `g` comes out.

The animation shows **hash aggregation**, the way most databases do it when the groups fit in memory. A hash table maps each group key to a small running state, here the pair `(count, sum)`. One pass over the rows is enough:

- If the row's `g` is **new**, open a group with count 1 and sum `v`.
- If the group **exists**, add 1 to its count and `v` to its sum.

Nothing is ever compared with other rows; each row touches only its own group. When the pass ends, the table holds the final count and sum for every group. A hash table keeps no useful order, so `ORDER BY g` is a separate step: sort the group keys and output the groups in that order.

The alternative is **sort-based grouping**: sort the rows by `g`, so equal keys sit next to each other, then sweep once and close a group whenever the key changes. It costs a full `O(n log n)` sort, but needs no hash table and gives the groups already in order. Databases pick it when the input is already sorted, for example read through an index on `g`.

### The default input

The array `[1, 10, 2, 5, 1, 30, 2, 7, 1, 2]` holds pairs `(g, v)`: five rows, `(1, 10)`, `(2, 5)`, `(1, 30)`, `(2, 7)`, `(1, 2)`.

Row 1 opens group 1: count 1, sum 10. Row 2 opens group 2: count 1, sum 5. Row 3 adds 30 to group 1: count 2, sum 40. Row 4 adds 7 to group 2: count 2, sum 12. Row 5 adds 2 to group 1: count 3, sum 42. Then the groups come out by `g`: group 1 first, then group 2.

The output is `[1, 3, 42, 2, 2, 12]`: group 1 has 3 rows summing to 42, group 2 has 2 rows summing to 12.

## Steps

1. Read the input as pairs `(g, v)`.
2. For each row, look `g` up in the hash table; start it at `(0, 0)` if missing, then add 1 to the count and `v` to the sum.
3. Sort the group keys.
4. For each key in order, output `g`, its count and its sum.

## Cost

With `n` rows and `g` groups:

| Case | Cost |
|---|---|
| Best | `O(n)`: one group, nothing to sort |
| Average and worst | `O(n + g log g)`: one pass, then sorting the group keys |
| Extra space | `O(g)`: one entry per group |

## Pitfalls

- **WHERE or HAVING?** `WHERE` filters rows **before** grouping, so it cannot see `COUNT(*)` or `SUM(v)`. `HAVING` filters groups **after** grouping: `HAVING COUNT(*) > 2` keeps only the large groups.
- **Expecting ordered groups.** Without `ORDER BY`, SQL promises no order; a hash table outputs groups in whatever order it likes.
- **Selecting a column that is not grouped.** In `SELECT g, v ... GROUP BY g`, a group has many `v` values; most databases reject it, and those that allow it return an arbitrary one.
- **`COUNT(*)` against `COUNT(v)`.** The first counts rows; the second skips rows where `v` is `NULL`.

## When to use it

Use hash aggregation whenever you must total, count or average records by a key: sales per customer, words per document, stock per product. In code it is just a dictionary of running totals. The linked problem, Crate Labels, adds up quantities per product in exactly this way.

## Related topics

- Hash table: separate chaining: the table that holds the groups.
- Join algorithms: the same hash table, used to pair rows instead of collapsing them.
- Window functions: totals per group that keep every row.
- Counting Sort: counting by key when the keys are small integers.
