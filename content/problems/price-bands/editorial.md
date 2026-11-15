## Idea

A filter with two parts and a sort:

1. **Keep** the products in stock.
2. **Keep** the prices inside the closed band 2000..4999.
3. **Order** from the most expensive down, with the name and then the id to settle ties.

## The query

```sql
SELECT name, price_cents
FROM products
WHERE in_stock = 1 AND price_cents BETWEEN 2000 AND 4999
ORDER BY price_cents DESC, name, id;
```

## Details that matter

- **The ends belong to the band.** Writing `price_cents > 2000 AND price_cents < 4999` looks natural but drops the products at exactly 20.00 and exactly 49.99, which the shop asked for. `BETWEEN a AND b` means `>= a AND <= b`, so both ends stay in.
- **Cents keep it exact.** The band in cents is 2000 to 4999, not 2000 to 5000: 49.99 is the last price in it, and 50.00 is out. Because the prices are whole numbers there is no rounding to worry about.
- **Filter on columns you do not return.** `in_stock` and `id` are not in the result, but `WHERE` and `ORDER BY` can still use them.
- **A full order.** Two products can have the same name and price, so the `id` breaks the last tie, even though it is not shown.

## Cost

One pass over the products to test the two conditions, then a sort of the rows that are kept. An index on `price_cents` would let SQLite read only the band, but at this size a scan is just as good.
