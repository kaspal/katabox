## Idea

Three steps, each one clause of a `SELECT`:

1. **Join** each loan to its member, to get the name.
2. **Filter** to the late loans: returned strictly after the due date.
3. **Compute and order** the days late.

## The query

```sql
SELECT m.name, l.title,
       CAST(julianday(l.returned) - julianday(l.due) AS INTEGER) AS days_late
FROM loans l
JOIN members m ON m.id = l.member_id
WHERE l.returned > l.due
ORDER BY days_late DESC, l.title, l.id;
```

## Details that matter

- **On time is not late.** `>=` would list books returned on their due day with `days_late = 0`. The condition is strictly `>`.
- **`NULL` filters itself out.** A comparison with `NULL` is neither true nor false, so `WHERE l.returned > l.due` drops books still out without an extra `IS NOT NULL`.
- **Dates as text.** `YYYY-MM-DD` strings sort in date order, so comparing them as text is safe. To *subtract* dates you need numbers: `julianday` gives the day number, and the difference of two whole days is a whole number, which `CAST(... AS INTEGER)` turns into an integer.
- **A full order.** Two loans can be equally late with the same title (the same book borrowed twice), so the loan's `id` breaks the last tie. `ORDER BY` may use the alias `days_late` and columns that are not in the result.

## Cost

With an index on `members.id` (the primary key), SQLite looks up each loan's member directly: about one step per loan, then a sort of the late rows.
