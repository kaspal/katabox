SELECT substr(placed, 1, 10) AS day,
       COUNT(*) AS orders,
       SUM(total_cents) AS revenue_cents
FROM orders
GROUP BY day
ORDER BY day;
