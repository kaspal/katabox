## Idea

The reporting lines form a tree with the director at the root. Walk it from the top, one level at a time:

1. **Start** with the director, at depth 0.
2. **Step down**: everyone whose boss is already in the list joins it, one deeper than that boss. Repeat until nobody new is added.
3. **Name and order** the rows.

A recursive common table expression does exactly this. Its first `SELECT` gives the starting rows; its second `SELECT` reads the rows found so far and produces the next ones. SQLite keeps feeding each new row back into the second part until no more rows appear.

## The query

```sql
WITH RECURSIVE chain(id, depth) AS (
  SELECT id, 0 FROM staff WHERE boss_id IS NULL
  UNION ALL
  SELECT s.id, chain.depth + 1
  FROM staff s
  JOIN chain ON s.boss_id = chain.id
)
SELECT s.name, chain.depth
FROM chain
JOIN staff s ON s.id = chain.id
ORDER BY chain.depth, s.name, s.id;
```

## Details that matter

- **One level up is not enough.** A query that only checks whether `boss_id` is `NULL` gives 0 to the director and 1 to everyone else. In Example 1 that puts Dev and Ada at depth 1 instead of 2 and 3. Adding a fixed number of self-joins does not fix it either: a line can be 1000 people long, so the number of levels is only known by walking them.
- **Each person is reached once.** There are no loops and everyone has a single boss, so the walk reaches every person by exactly one path, and `UNION ALL` produces no duplicates. (With a loop, the depth would keep growing and the recursion would never end; the constraints rule that out.)
- **Long lines are fine.** SQLite runs a recursive CTE with a queue, not with nested calls, so a chain of 1000 levels does not run into a depth limit.
- **Repeated names.** Two people can share a name, so join and order by `id`, never by `name` alone. Ordering by `id` last makes the order complete.
- **No declared foreign key.** The schema does not declare `boss_id` as a reference, but the join only compares values, and every `boss_id` names a real person.
- **Down, not up.** You could also start from each person and climb to the director, counting the steps. That works, but it walks each line again for every person on it (up to about 500,000 rows for one long line); walking down from the director visits each person once.

## Cost

Every person enters `chain` once. For each of them SQLite looks up their direct reports in `staff`; with an index on `boss_id` (SQLite may build an automatic one) that is about `O(n log n)` in all, and even a plain scan per row, about `n²` = 10⁶ comparisons for 1000 people, is quick. The final join on the primary key and the sort add `O(n log n)`.
