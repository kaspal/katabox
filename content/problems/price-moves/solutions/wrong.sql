-- LAG without PARTITION BY: a company's first day is compared with another company's
-- last close.
SELECT ticker, day, close_cents, prev_cents,
       ROUND(100.0 * (close_cents - prev_cents) / prev_cents, 2) AS change_pct
FROM (
  SELECT ticker, day, close_cents,
         LAG(close_cents) OVER (ORDER BY ticker, day) AS prev_cents
  FROM prices
)
WHERE prev_cents IS NOT NULL
ORDER BY ticker, day;
