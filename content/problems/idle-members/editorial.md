## Idea

This is an **anti-join**: keep each member for whom no matching row exists in another table. Here the matching row is "a visit by this member in November 2026", so the date condition belongs inside the test for existence, not outside it.

## The query

```sql
SELECT m.id, m.name
FROM members m
WHERE NOT EXISTS (
  SELECT 1 FROM visits v
  WHERE v.member_id = m.id AND v.day BETWEEN '2026-11-01' AND '2026-11-30'
)
ORDER BY m.name, m.id;
```

## Details that matter

- **Idle in November, not idle forever.** `WHERE m.id NOT IN (SELECT member_id FROM visits)` finds the members who never came at all. A member whose only visit was in October (Bo in Example 1) has a visit, so that query drops Bo, who is still idle in November. The month must be part of the inner query.
- **Both ends of the month count.** `BETWEEN` includes both bounds, so visits on November 1 and November 30 make a member active, while October 31 and December 1 do not. Writing `< '2026-11-30'` would miss the last day (Example 2).
- **Dates as text.** Every `day` has the form `YYYY-MM-DD`, so text order is date order and `BETWEEN` on strings is safe. `v.day LIKE '2026-11-%'` or `strftime('%Y-%m', v.day) = '2026-11'` work as well.
- **A `LEFT JOIN` works too, if the filter is in the right place.** `LEFT JOIN visits v ON v.member_id = m.id AND v.day BETWEEN ... WHERE v.id IS NULL` is equivalent. Moving the date test into `WHERE` returns no rows at all: a member with no match has a `NULL` day, which never passes `BETWEEN`.
- **`NOT IN` and `NULL`.** If the inner list of a `NOT IN` contains a `NULL`, the test is never true and no row comes back. `member_id` is `NOT NULL` here, but `NOT EXISTS` has no such trap.
- **Names repeat.** Two members can share a name, so `m.id` breaks the tie.

## Cost

For each member SQLite looks for one November visit and stops at the first. `visits.member_id` has no index of its own, so SQLite usually builds a temporary automatic index on it; with that, the query is about one lookup per member plus a sort, roughly (members + visits) log visits.
