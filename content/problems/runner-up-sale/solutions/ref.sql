SELECT r.region,
       (SELECT MAX(s.amount_cents)
        FROM sales s
        WHERE s.region = r.region
          AND s.amount_cents < (SELECT MAX(t.amount_cents) FROM sales t WHERE t.region = r.region)
       ) AS runner_up_cents
FROM (SELECT DISTINCT region FROM sales) r
ORDER BY r.region;
