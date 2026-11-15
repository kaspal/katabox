## Idea

Turning rows into columns ("pivoting") is grouping plus conditional sums:

1. **Group** the sales by year: one result row per year that has sales.
2. **Sum conditionally**: each quarter's column adds a sale's amount when its month falls in that quarter, and 0 otherwise.
3. **Order** by year.

## The query

```sql
SELECT CAST(substr(sold_on, 1, 4) AS INTEGER) AS year,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 1 AND 3 THEN amount_cents ELSE 0 END) AS q1,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 4 AND 6 THEN amount_cents ELSE 0 END) AS q2,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 7 AND 9 THEN amount_cents ELSE 0 END) AS q3,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 10 AND 12 THEN amount_cents ELSE 0 END) AS q4
FROM sales
GROUP BY year
ORDER BY year;
```

## Details that matter

- **The quarter formula.** Computing the quarter as `month / 3 + 1` looks tidy but is off by one at the end of each quarter: March gives 2, June 3, September 4, and December 5, which is no quarter at all. In Example 1, the March sale would move to Q2 and December's would vanish. The right formula is `(month - 1) / 3 + 1` (integer division); explicit ranges with `BETWEEN` avoid the arithmetic altogether.
- **0, not `NULL`.** Without `ELSE 0`, a `CASE` that matches no row of the group yields `NULL` for every row, and `SUM` of only `NULL`s is `NULL`. `ELSE 0` makes an empty quarter 0. (`TOTAL` would give a float `0.0`, which is not the integer 0.)
- **The year as an integer.** `substr` returns text, so without the cast `year` would come out as the text `'2025'` instead of the integer `2025`. The same goes for the month: `'03' BETWEEN 1 AND 3` compares text with numbers and is false in SQLite, so cast before comparing. (`strftime('%m', sold_on)` also returns text.)
- **Years without sales.** Grouping only creates groups for years that occur, so a year with no sales is absent, as the statement asks; an empty table gives no rows.

## Cost

One pass over the `n` sales, adding each amount to its year's running sums, then a sort of the few years: about `O(n)` work plus `O(y log y)` for `y` years.
