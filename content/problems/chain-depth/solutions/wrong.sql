-- Looks one level up only: everyone with a boss gets depth 1.
SELECT name, CASE WHEN boss_id IS NULL THEN 0 ELSE 1 END AS depth
FROM staff
ORDER BY depth, name, id;
