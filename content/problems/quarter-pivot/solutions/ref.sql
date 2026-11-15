SELECT CAST(substr(sold_on, 1, 4) AS INTEGER) AS year,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 1 AND 3 THEN amount_cents ELSE 0 END) AS q1,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 4 AND 6 THEN amount_cents ELSE 0 END) AS q2,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 7 AND 9 THEN amount_cents ELSE 0 END) AS q3,
       SUM(CASE WHEN CAST(substr(sold_on, 6, 2) AS INTEGER) BETWEEN 10 AND 12 THEN amount_cents ELSE 0 END) AS q4
FROM sales
GROUP BY year
ORDER BY year;
