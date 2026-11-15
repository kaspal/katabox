-- A food truck's orders. placed is 'YYYY-MM-DD HH:MM'; totals are in cents.
CREATE TABLE orders (
  id INTEGER PRIMARY KEY,
  placed TEXT NOT NULL,
  total_cents INTEGER NOT NULL
);
