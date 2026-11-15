SELECT m.id, m.name
FROM members m
WHERE NOT EXISTS (
  SELECT 1 FROM visits v
  WHERE v.member_id = m.id AND v.day BETWEEN '2026-11-01' AND '2026-11-30'
)
ORDER BY m.name, m.id;
