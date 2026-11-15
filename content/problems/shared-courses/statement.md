A language school keeps its students in `students` and every enrolment in `enrollments`: one row says that a student takes a course. A student can take several courses, and a course has many students.

To set up study groups, the school wants every **pair of students who share at least one course**. Each pair appears once, even when the two students share several courses, and a student is never paired with themselves.

This is an **SQL problem**: write one `SELECT` statement (SQLite 3.53, as bundled). The tables are in the schema panel.

## Tables

- `students(id, name)`: `id` is the primary key; no two students have the same name.
- `enrollments(id, student_id, course)`: `student_id` refers to `students(id)`; `course` is a course code such as `ENG101`. A student enrolls in a given course at most once.

## Result

Return one row per pair, with two columns:

| column | value |
| --- | --- |
| `student_a` | the name of the student with the **smaller** `id` |
| `student_b` | the name of the student with the larger `id` |

Order the rows by the `id` of `student_a`, then by the `id` of `student_b` (the ids are not in the result).

### Example 1

`students`:

| id | name |
| --- | --- |
| 1 | ana |
| 2 | bo |
| 3 | cy |

`enrollments`:

| id | student_id | course |
| --- | --- | --- |
| 1 | 1 | ENG101 |
| 2 | 2 | ENG101 |
| 3 | 1 | FRA101 |
| 4 | 2 | FRA101 |
| 5 | 3 | GER101 |

Result:

| student_a | student_b |
| --- | --- |
| ana | bo |

ana and bo share two courses, ENG101 and FRA101, but they are one pair, so they appear once. cy is alone in GER101.

### Example 2

`students`:

| id | name |
| --- | --- |
| 4 | di |
| 5 | ed |

`enrollments`:

| id | student_id | course |
| --- | --- | --- |
| 6 | 4 | ITA101 |
| 7 | 5 | JPN101 |

Result: empty (no rows). di and ed take different courses.

## Constraints

- 1 ≤ students ≤ 150, 0 ≤ enrollments ≤ 600.
- Student ids are distinct, from 1 to 10⁶; names are distinct, 1 to 10 lowercase letters.
- A course code is three capital letters and three digits.
- Every enrolment belongs to an existing student, and a student enrolls in a course at most once. A student may have no enrolments.
