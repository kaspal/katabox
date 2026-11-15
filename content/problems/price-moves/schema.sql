-- Closing prices of a few listed companies, one row per company and trading day.
CREATE TABLE prices (
  id INTEGER PRIMARY KEY,
  ticker TEXT NOT NULL,
  day TEXT NOT NULL,
  close_cents INTEGER NOT NULL
);
