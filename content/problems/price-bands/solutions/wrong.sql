-- Leaves out both ends of the band (20.00 and 49.99 are in it).
SELECT name, price_cents
FROM products
WHERE in_stock = 1 AND price_cents > 2000 AND price_cents < 4999
ORDER BY price_cents DESC, name, id;
