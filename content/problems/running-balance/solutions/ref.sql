SELECT account, day, amount_cents,
       SUM(amount_cents) OVER (
         PARTITION BY account
         ORDER BY day, id
         ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
       ) AS balance_cents
FROM transactions
ORDER BY account, day, id;
