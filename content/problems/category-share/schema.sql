-- A stationery shop's sales lines. Amounts are in cents.
CREATE TABLE sales (
  id INTEGER PRIMARY KEY,
  category TEXT NOT NULL,
  amount_cents INTEGER NOT NULL
);
