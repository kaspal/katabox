## Idea

Three steps, each one clause of a `SELECT`:

1. **Derive the day** of each order: the first 10 characters of `placed`.
2. **Group** the orders by that day, counting them and adding up their totals.
3. **Order** the groups by day.

## The query

```sql
SELECT substr(placed, 1, 10) AS day,
       COUNT(*) AS orders,
       SUM(total_cents) AS revenue_cents
FROM orders
GROUP BY day
ORDER BY day;
```

## Details that matter

- **Group by the day, not the timestamp.** `GROUP BY placed` makes one group per distinct minute: in Example 1 the two orders at 12:05 share a group, but the one at 19:40 gets its own, so May 1 comes out as two rows (2 orders, then 1). The `day` column looks right on each row, which makes the mistake easy to miss; the grouping key is what decides the rows.
- **The alias in `GROUP BY`.** SQLite lets `GROUP BY day` refer to the alias from the `SELECT` list. `GROUP BY substr(placed, 1, 10)` is the same and works in every database.
- **Text dates sort as dates.** `YYYY-MM-DD` strings sort in calendar order, so `ORDER BY day` needs no conversion. `date(placed)` would also give the day, since SQLite reads `YYYY-MM-DD HH:MM` as a time.
- **Only days with orders.** Grouping only produces groups that have rows, so empty days never appear and an empty table gives an empty result. Listing every day of a range would need a calendar (for example a recursive CTE), which this problem does not ask for.

## Cost

SQLite sorts the orders by day (or builds a temporary index on it) and then walks them once: about `O(n log n)` for `n` orders. The result has one row per day.
