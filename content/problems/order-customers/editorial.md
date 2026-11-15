## Idea

Three steps, each one clause of a `SELECT`:

1. **Join** each order to the customer who placed it, through `orders.customer_id`.
2. **Filter** to the big orders: `total_cents` of 10000 or more.
3. **Rename and order**: the order's `id` becomes `order_id`, and the rows go from the largest total down.

## The query

```sql
SELECT o.id AS order_id, c.name, c.city, o.total_cents
FROM orders o
JOIN customers c ON c.id = o.customer_id
WHERE o.total_cents >= 10000
ORDER BY o.total_cents DESC, o.id;
```

## Details that matter

- **Join on the foreign key.** `ON c.id = o.id` looks plausible, since both are called `id`, but it pairs order 1 with customer 1 just because the numbers match. In Example 1 that would give order 1 to Ana instead of Bruno, and an order whose id is not a customer id would vanish. The link between the tables is `o.customer_id`.
- **At least means `>=`.** An order of exactly 100.00 (`10000`) is big; `>` would drop it. Order 2 of Example 1, at `9999`, shows the other side of the boundary.
- **Cents, not units.** `total_cents >= 100` would keep almost every order. The threshold is 100.00 written in cents.
- **A full order.** Two orders can have the same total, even for the same customer, so `o.id` breaks the tie. Ordering by name would not be enough, because names repeat.
- **Aliases.** Both tables have an `id`, so each column must be qualified (`o.id`, `c.name`), and the result column must be named `order_id`.

## Cost

With the primary key on `customers.id`, SQLite looks up each big order's customer directly: about one step per order, then a sort of the big orders.
