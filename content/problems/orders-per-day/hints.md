You need one row per day, but no column holds the day on its own: `placed` also carries the time. How do you cut the time off?
---
`substr(placed, 1, 10)` keeps just the `YYYY-MM-DD` part. Grouping by that value puts all the orders of one day in the same group, whatever their time.
---
`SELECT substr(placed, 1, 10) AS day, COUNT(*) AS orders, SUM(total_cents) AS revenue_cents FROM orders GROUP BY day ORDER BY day`. Group by the day, not by `placed`, or every distinct minute becomes its own row.
