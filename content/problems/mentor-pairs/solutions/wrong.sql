-- An inner join drops everyone without a mentor.
SELECT s.name, m.name AS mentor
FROM staff s
JOIN staff m ON m.id = s.mentor_id
ORDER BY s.name, s.id;
