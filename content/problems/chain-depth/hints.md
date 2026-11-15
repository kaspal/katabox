Each person is exactly one level deeper than their boss. You know where that rule starts (the director, at 0), but not how many levels there are. How can a query keep applying the same step until it runs out of people?
---
A `WITH RECURSIVE chain(id, depth) AS (...)` has two parts joined by `UNION ALL`: the starting rows, `SELECT id, 0 FROM staff WHERE boss_id IS NULL`, and a step that adds the people whose `boss_id` is an id already in `chain`, one level deeper. SQLite repeats the step until it adds no new rows.
---
The step is `SELECT s.id, chain.depth + 1 FROM staff s JOIN chain ON s.boss_id = chain.id`. After the `WITH`, join `chain` back to `staff` on `id` to get the names, and finish with `ORDER BY chain.depth, s.name, s.id`.
