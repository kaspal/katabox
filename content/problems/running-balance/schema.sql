-- A credit union's transactions. amount_cents is positive for deposits and negative
-- for withdrawals; day is 'YYYY-MM-DD'.
CREATE TABLE transactions (
  id INTEGER PRIMARY KEY,
  account TEXT NOT NULL,
  day TEXT NOT NULL,
  amount_cents INTEGER NOT NULL
);
