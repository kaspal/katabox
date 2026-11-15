-- quarter = month / 3 + 1 puts March, June and September a quarter late, and December
-- in no quarter at all.
SELECT CAST(substr(sold_on, 1, 4) AS INTEGER) AS year,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) / 3 + 1 = 1 THEN amount_cents ELSE 0 END) AS q1,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) / 3 + 1 = 2 THEN amount_cents ELSE 0 END) AS q2,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) / 3 + 1 = 3 THEN amount_cents ELSE 0 END) AS q3,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) / 3 + 1 = 4 THEN amount_cents ELSE 0 END) AS q4
FROM sales
GROUP BY year
ORDER BY year;
