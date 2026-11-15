One row per year means `GROUP BY` the year. The four quarters are not four groups but four columns of the same group: each column adds up only some of the group's rows.
---
`substr(sold_on, 1, 4)` is the year and `substr(sold_on, 6, 2)` the month, both as text; `CAST(... AS INTEGER)` makes them numbers. Inside an aggregate, `CASE WHEN <month is in the quarter> THEN amount_cents ELSE 0 END` counts a sale only in its own quarter.
---
Write four of those sums, one per range: `BETWEEN 1 AND 3`, `4 AND 6`, `7 AND 9`, `10 AND 12`, named `q1` to `q4`. Select `CAST(substr(sold_on, 1, 4) AS INTEGER) AS year`, then `GROUP BY year ORDER BY year`. The `ELSE 0` is what gives an empty quarter 0 instead of `NULL`.
