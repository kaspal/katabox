-- Orders the window by day only: with the default frame, transactions on the same day
-- are peers and all get the day's closing balance.
SELECT account, day, amount_cents,
       SUM(amount_cents) OVER (PARTITION BY account ORDER BY day) AS balance_cents
FROM transactions
ORDER BY account, day, id;
