SELECT sa.name AS student_a, sb.name AS student_b
FROM (
  SELECT DISTINCT e1.student_id AS a, e2.student_id AS b
  FROM enrollments e1
  JOIN enrollments e2 ON e2.course = e1.course AND e1.student_id < e2.student_id
) p
JOIN students sa ON sa.id = p.a
JOIN students sb ON sb.id = p.b
ORDER BY p.a, p.b;
