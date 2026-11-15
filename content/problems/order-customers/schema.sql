-- A bakery's customers and their orders. Totals are in cents.
CREATE TABLE customers (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  city TEXT NOT NULL
);
CREATE TABLE orders (
  id INTEGER PRIMARY KEY,
  customer_id INTEGER NOT NULL REFERENCES customers(id),
  placed TEXT NOT NULL,
  total_cents INTEGER NOT NULL
);
