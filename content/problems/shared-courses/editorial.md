## Idea

A pair of students who share a course is two rows of `enrollments` with the same `course`, so the query is a **self-join**:

1. **Pair up** enrolments of the same course, keeping only `e1.student_id < e2.student_id`.
2. **Collapse** the result to distinct pairs of ids.
3. **Name** both students and order by their ids.

## The query

```sql
SELECT sa.name AS student_a, sb.name AS student_b
FROM (
  SELECT DISTINCT e1.student_id AS a, e2.student_id AS b
  FROM enrollments e1
  JOIN enrollments e2 ON e2.course = e1.course AND e1.student_id < e2.student_id
) p
JOIN students sa ON sa.id = p.a
JOIN students sb ON sb.id = p.b
ORDER BY p.a, p.b;
```

## Details that matter

- **One row per pair, not per course.** The self-join gives one row for every course two students share. Without `DISTINCT`, two students with two courses in common come out twice: that is the mistake of joining the names straight onto the self-join and ordering by `e1.student_id, e2.student_id`.
- **`<`, not `<>`.** `e1.student_id <> e2.student_id` drops self-pairs but keeps both `(ana, bo)` and `(bo, ana)`. Strict `<` keeps one orientation, the one the result asks for (smaller id first), and no student meets themselves.
- **Order by ids, not names.** The smaller id does not always belong to the alphabetically first name, so `ORDER BY student_a, student_b` can give the wrong order. Keeping the ids in the subquery lets `ORDER BY p.a, p.b` use them without showing them.
- **Join `students` twice.** Each side of the pair needs its own copy of the table (`sa` and `sb`); a single join can only name one of the two.
- **Students without courses** never appear in the self-join, so they need no special case.

## Cost

The self-join compares enrolments within each course: a course with `k` students gives about `k²/2` rows. An index on `enrollments(course)` finds the partners directly; then the distinct pairs are sorted and each name is a primary-key lookup.
