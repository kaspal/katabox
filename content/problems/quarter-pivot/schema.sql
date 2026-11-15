-- An orchard's sales, one row per invoice. sold_on is 'YYYY-MM-DD'; amounts are in cents.
CREATE TABLE sales (
  id INTEGER PRIMARY KEY,
  sold_on TEXT NOT NULL,
  amount_cents INTEGER NOT NULL
);
