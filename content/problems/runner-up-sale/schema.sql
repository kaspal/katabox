-- A furniture chain's sales, by region. Amounts are in cents.
CREATE TABLE sales (
  id INTEGER PRIMARY KEY,
  region TEXT NOT NULL,
  amount_cents INTEGER NOT NULL
);
