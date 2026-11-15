WITH RECURSIVE chain(id, depth) AS (
  SELECT id, 0 FROM staff WHERE boss_id IS NULL
  UNION ALL
  SELECT s.id, chain.depth + 1
  FROM staff s
  JOIN chain ON s.boss_id = chain.id
)
SELECT s.name, chain.depth
FROM chain
JOIN staff s ON s.id = chain.id
ORDER BY chain.depth, s.name, s.id;
