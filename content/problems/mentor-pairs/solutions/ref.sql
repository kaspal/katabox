SELECT s.name, m.name AS mentor
FROM staff s
LEFT JOIN staff m ON m.id = s.mentor_id
ORDER BY s.name, s.id;
