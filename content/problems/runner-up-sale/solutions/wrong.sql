-- The second row by amount: a tie for the top amount makes the top amount its own runner-up.
SELECT r.region,
       (SELECT s.amount_cents
        FROM sales s
        WHERE s.region = r.region
        ORDER BY s.amount_cents DESC
        LIMIT 1 OFFSET 1
       ) AS runner_up_cents
FROM (SELECT DISTINCT region FROM sales) r
ORDER BY r.region;
