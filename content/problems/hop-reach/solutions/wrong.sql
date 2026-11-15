-- Stops at hops <= 3 instead of < 3, so piers four crossings away get in too.
WITH RECURSIVE reach(pier, hops) AS (
  SELECT 'Central', 0
  UNION
  SELECT r.to_pier, reach.hops + 1
  FROM reach
  JOIN routes r ON r.from_pier = reach.pier
  WHERE reach.hops <= 3
)
SELECT pier, MIN(hops) AS hops
FROM reach
WHERE pier <> 'Central'
GROUP BY pier
ORDER BY hops, pier;
