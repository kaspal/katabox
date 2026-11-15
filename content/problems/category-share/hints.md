Each category's total is a plain `GROUP BY category` with `SUM(amount_cents)`. But the share also needs the total of the whole table, which no single group knows. How can a grouped query get at it?
---
A scalar subquery, `(SELECT SUM(amount_cents) FROM sales)`, gives the grand total as one value you can use in any row. Then watch the division: in SQLite an integer divided by an integer is an integer, with the decimals cut off.
---
Start the expression with a real number so the division keeps its decimals: `ROUND(100.0 * SUM(amount_cents) / (SELECT SUM(amount_cents) FROM sales), 1) AS share_pct`. Finish with `ORDER BY share_pct DESC, category`.
