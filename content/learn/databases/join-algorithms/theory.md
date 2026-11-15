## The idea

A **join** glues two tables together: `SELECT L.key, L.a, R.b FROM L JOIN R ON L.key = R.key` pairs every row of `L` with every row of `R` that has the same `key`, and outputs one row per pair. Rows without a partner on the other side simply disappear (this is an inner join).

The plainest way is the **nested-loop join**: for each row of `L`, scan all of `R` and keep the pairs whose keys are equal. It is always correct and needs no extra memory, but it compares every pair, `|L| · |R|` comparisons, even when almost nothing matches.

The **hash join**, the one the animation shows, works in two phases:

- **Build**: read `R` once and put each row into a hash index under its `key`. Rows with the same key share one entry, as a list of row positions.
- **Probe**: read `L` once. For each row, look its `key` up in the index: one hash lookup instead of a scan of `R`. Every R row listed there is a match, and each match becomes one output row.

The output keeps `L` order, and within one `L` row, `R` order.

### The default input

The array `[3, 1, 10, 2, 20, 3, 30, 2, 5, 3, 7, 3, 8, 4, 9]` starts with `3`, the number of `L` rows. The next three pairs are `L` as `(key, a)`: `(1, 10)`, `(2, 20)`, `(3, 30)`. The remaining pairs are `R` as `(key, b)`: `(2, 5)`, `(3, 7)`, `(3, 8)`, `(4, 9)`.

Build: the index gets key 2 with R row #1, key 3 with rows #2 and #3, and key 4 with row #4. Probe: key 1 finds nothing, so `(1, 10)` is dropped. Key 2 finds one row and outputs `(2, 20, 5)`. Key 3 finds two rows and outputs `(3, 30, 7)` and `(3, 30, 8)`. R row `(4, 9)` is never asked for.

The output is `[2, 20, 5, 3, 30, 7, 3, 30, 8]`: three joined rows from 4 inserts and 3 probes, where a nested-loop join would have compared all 12 pairs.

## Steps

1. Read the number of `L` rows, then split the pairs into `L` (that many) and `R` (the rest).
2. Build: for each `R` row, append its `b` to the index entry for its `key`.
3. Probe: for each `L` row, look up its `key`; for every value found, output `(key, a, b)`.
4. Return the output rows in that order.

## Cost

With `m` rows in `L` and `n` rows in `R`:

| Case | Cost |
|---|---|
| Best | `O(m + n)`: few or no matches |
| Average | `O(m + n + output)`: one insert per `R` row, one probe per `L` row, one write per joined row |
| Worst | `O(m · n)`: every row shares one key, so the output itself has `m · n` rows |
| Extra space | `O(n)` for the index |

A nested-loop join costs `O(m · n)` in every case.

Real databases also use a third method, the **sort-merge join**: sort both tables by `key`, then walk them side by side like the merge step of merge sort. It shines when both inputs already come sorted, for example from an index. The query planner chooses among the three from table sizes, available indexes and memory; a nested loop is often the best choice when one side is tiny or `R.key` has an index.

## Pitfalls

- **Building on the bigger table.** The index must fit in memory; databases build on the smaller side and probe with the larger one.
- **Duplicate keys multiply rows.** Two `L` rows and three `R` rows with the same key give six output rows. A join can be much larger than either table.
- **NULL keys.** In SQL, `NULL = NULL` is not true, so rows with a `NULL` key never join.
- **A missing `ON` condition.** Without it, every pair matches: a cross join of `|L| · |R|` rows.

## When to use it

Use a hash join whenever you must match records from two lists by an equal key and neither list is sorted: matching orders to customers, merging two exports by id. The linked problem, Two Sum, uses the same build-and-probe idea: remember values in a hash table, then look each partner up in constant time.

## Related topics

- Hash table: separate chaining: the index the build phase fills.
- Hash table: open addressing: another way to store that index.
- Merge Sort: the merge step behind the sort-merge join.
- GROUP BY: the same hash table, used to collapse rows instead of pairing them.
