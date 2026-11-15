SELECT o.id AS order_id, c.name, c.city, o.total_cents
FROM orders o
JOIN customers c ON c.id = o.customer_id
WHERE o.total_cents >= 10000
ORDER BY o.total_cents DESC, o.id;
