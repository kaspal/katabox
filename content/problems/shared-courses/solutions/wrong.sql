-- No DISTINCT: two students who share two courses come out twice.
SELECT sa.name AS student_a, sb.name AS student_b
FROM enrollments e1
JOIN enrollments e2 ON e2.course = e1.course AND e1.student_id < e2.student_id
JOIN students sa ON sa.id = e1.student_id
JOIN students sb ON sb.id = e2.student_id
ORDER BY e1.student_id, e2.student_id;
