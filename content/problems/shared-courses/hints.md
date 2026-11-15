A pair needs two enrolments in the same course. What do you get if you join `enrollments` with itself on `course`?
---
`JOIN enrollments e2 ON e2.course = e1.course AND e1.student_id < e2.student_id` keeps each pair once per shared course, with the smaller id on the left, and drops a student matched with themselves. Two shared courses still give two rows, though.
---
Reduce to pairs first: `SELECT DISTINCT e1.student_id AS a, e2.student_id AS b` in a subquery. Then join `students` twice, once for `a` and once for `b`, name the columns `student_a` and `student_b`, and finish with `ORDER BY p.a, p.b`.
