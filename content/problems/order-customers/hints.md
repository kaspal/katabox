The totals are in `orders`, the names and cities in `customers`. Which column of an order tells you who placed it?
---
`JOIN customers c ON c.id = o.customer_id` puts each order next to its customer. Do not join on `o.id`: that is the order's own number, and it can match an unrelated customer's id. Keep the rows with `o.total_cents >= 10000`; an order of exactly 100.00 counts.
---
Select `o.id AS order_id, c.name, c.city, o.total_cents` and finish with `ORDER BY o.total_cents DESC, o.id`, so equal totals come out by order id.
