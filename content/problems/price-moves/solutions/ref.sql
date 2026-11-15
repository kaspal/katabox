SELECT ticker, day, close_cents, prev_cents,
       ROUND(100.0 * (close_cents - prev_cents) / prev_cents, 2) AS change_pct
FROM (
  SELECT ticker, day, close_cents,
         LAG(close_cents) OVER (PARTITION BY ticker ORDER BY day) AS prev_cents
  FROM prices
)
WHERE prev_cents IS NOT NULL
ORDER BY ticker, day;
