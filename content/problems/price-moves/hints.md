Each result row needs a value from another row: the same company's row just before it in time. Which kind of function can read a neighbouring row without joining the table to itself?
---
`LAG(close_cents) OVER (PARTITION BY ticker ORDER BY day)` gives the previous close of the same ticker, or `NULL` on its first day. A window function cannot be used in `WHERE`, so compute it in a subquery (or a `WITH`) and filter outside with `prev_cents IS NOT NULL`.
---
In the outer query, `ROUND(100.0 * (close_cents - prev_cents) / prev_cents, 2) AS change_pct`; the `100.0` makes the division real instead of integer. Select `ticker, day, close_cents, prev_cents` before it and finish with `ORDER BY ticker, day`.
