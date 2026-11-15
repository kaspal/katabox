SELECT name, price_cents
FROM products
WHERE in_stock = 1 AND price_cents BETWEEN 2000 AND 4999
ORDER BY price_cents DESC, name, id;
