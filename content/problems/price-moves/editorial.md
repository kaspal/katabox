## Idea

The previous close is simply the row before this one, once each company's rows are put in date order. A window function reads it directly:

1. **Split** the rows by `ticker` and **sort** each company's rows by `day`.
2. **Look back one row** with `LAG`: the previous trading day's close, or `NULL` on the first day.
3. **Drop** the first days, **compute** the change and **order** the result.

## The query

```sql
SELECT ticker, day, close_cents, prev_cents,
       ROUND(100.0 * (close_cents - prev_cents) / prev_cents, 2) AS change_pct
FROM (
  SELECT ticker, day, close_cents,
         LAG(close_cents) OVER (PARTITION BY ticker ORDER BY day) AS prev_cents
  FROM prices
)
WHERE prev_cents IS NOT NULL
ORDER BY ticker, day;
```

## Details that matter

- **`PARTITION BY ticker`.** Without it, `LAG(close_cents) OVER (ORDER BY ticker, day)` runs over one long list of all companies. Inside a company it gives the same answer, but each company's first day after the first company picks up the *previous company's* last close instead of `NULL`, so it is not filtered out. In Example 1, BOLT's 2026-03-02 would appear, compared with ACME's 1029, at −61.13%. The partition restarts the look-back for every ticker.
- **Previous trading day, not yesterday.** Joining each row to the row of `date(day, '-1 day')` misses every day after a gap, such as BOLT's 2026-03-04 in Example 1, and every Monday. `LAG` takes whatever row comes before in date order, however far back it is.
- **Filter after the window.** `WHERE` runs before window functions, so `prev_cents` cannot be tested in the same `SELECT` that computes it. Filtering first would also remove the very rows `LAG` needs to look at. The subquery computes the window over all rows, and the outer query filters.
- **Real division.** `close_cents` and `prev_cents` are integers, and SQLite divides integers as integers: `100 * (8 - 7) / 7` is 14, not 14.29. Writing `100.0` makes the whole expression real. Other arrangements, such as dividing first and multiplying by 100 afterwards, can differ in the last floating-point digits; the expected answers use exactly the expression above, rounded with `ROUND`.
- **No division by zero.** Every close is at least 1 cent, so `prev_cents` is never 0.
- **Dates as text.** `YYYY-MM-DD` sorts in date order, so `ORDER BY day` is correct, and the rows' `id` order does not matter.

## Cost

The window needs the rows sorted by `(ticker, day)`, and the final order is the same, so the query is one sort plus a pass: `O(n log n)`. A self-join that finds, for each row, the latest earlier day of the same ticker also works, but without an index it compares every row with every other row of its ticker.
