-- A shop's bank statement and its own books, one row per payment reference.
-- Amounts are in cents.
CREATE TABLE bank (
  id INTEGER PRIMARY KEY,
  ref TEXT NOT NULL,
  amount_cents INTEGER NOT NULL
);
CREATE TABLE books (
  id INTEGER PRIMARY KEY,
  ref TEXT NOT NULL,
  amount_cents INTEGER NOT NULL
);
