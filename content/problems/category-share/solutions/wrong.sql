-- Integer division: 100 * part / total drops the decimals before rounding.
SELECT category,
       SUM(amount_cents) AS total_cents,
       ROUND(100 * SUM(amount_cents) / (SELECT SUM(amount_cents) FROM sales), 1) AS share_pct
FROM sales
GROUP BY category
ORDER BY share_pct DESC, category;
