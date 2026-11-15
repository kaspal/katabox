-- Groups by the whole timestamp: one group per minute, not per day.
SELECT substr(placed, 1, 10) AS day,
       COUNT(*) AS orders,
       SUM(total_cents) AS revenue_cents
FROM orders
GROUP BY placed
ORDER BY day;
